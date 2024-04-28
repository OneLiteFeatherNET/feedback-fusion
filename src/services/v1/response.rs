//SPDX-FileCopyrightText: 2023 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2023 OneLiteFeatherNet

//Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
//associated documentation files (the "Software"), to deal in the Software without restriction,
//including without limitation the rights to use, copy, modify, merge, publish, distribute,
//sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice (including the next paragraph) shall be
//included in all copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
//NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
//NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
//DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use super::{FeedbackFusionV1Context, PublicFeedbackFusionV1Context};
use crate::{
    database::schema::feedback::{Field, FieldData, FieldResponse, PromptResponse},
    prelude::*,
};
use feedback_fusion_common::proto::{
    CreateResponsesRequest, FieldResponse as ProtoFieldResponse, FieldResponseList,
    GetResponsesRequest, PromptResponse as ProtoPromptResponse, ResponsePage,
};
use rbatis::rbatis_codegen::IntoSql;
use std::collections::HashMap;

pub async fn create_responses(
    context: &PublicFeedbackFusionV1Context,
    request: Request<CreateResponsesRequest>,
) -> Result<Response<ProtoPromptResponse>> {
    let data = request.into_inner();
    // start transaction
    let transaction = context.connection().acquire_begin().await?;
    let mut transaction = transaction.defer_async(|mut tx| async move {
        if !tx.done {
            let _ = tx.rollback().await;
        }
    });

    // fetch the fields of the prompt
    let fields = database_request!(
        Field::select_by_column(&transaction, "prompt", data.prompt.as_str()).await?
    );
    // as we can assume a prompt has to have at least 1 field we can throw the 400 here
    if fields.is_empty() {
        return Err(FeedbackFusionError::BadRequest("invalid prompt".to_owned()));
    }

    // insert the response dataprompt
    let response = PromptResponse::builder().prompt(data.prompt).build();
    database_request!(PromptResponse::insert(&transaction, &response).await?);

    // transform the hashmap into a field data vec
    let data = data
        .data
        .into_iter()
        .map(|(key, value)| {
            // try to get the field
            let field = fields.iter().find(|f| key.eq(f.id()));

            if let Some(field) = field {
                // validate the data
                let field_data: FieldData = value.data.unwrap().try_into()?;
                field_data.validate(field.options())?;

                Ok(FieldResponse::builder()
                    .response(response.id().as_str())
                    .field(field.id())
                    .data(JsonV(field_data))
                    .build())
            } else {
                Err(FeedbackFusionError::BadRequest(format!(
                    "Invalid field '{}'",
                    key
                )))
            }
        })
        .collect::<Result<Vec<FieldResponse>>>()?;
    // insert them as batch
    database_request!(
        FieldResponse::insert_batch(&transaction, data.as_slice(), data.len() as u64).await?
    );

    // commit the transaction
    transaction.commit().await?;

    Ok(Response::new(response.into()))
}

pub type GetFeedbackPromptResponsesResponse = HashMap<String, Vec<FieldResponse>>;

#[derive(Deserialize, Debug, Clone)]
struct DatabaseResult {
    result: JsonV<GetFeedbackPromptResponsesResponse>,
}

#[py_sql(
    "`SELECT jsonb_object_agg(response, rows) AS RESULT FROM (`
        `SELECT response, `
        `jsonb_agg(jsonb_build_object('id', id, 'response', response, 'field', field, 'data', data)) AS ROWS `
            `FROM field_response `
                ` WHERE response IN `
                    ${responses.sql()} 
    ` GROUP BY response) subquery`"
)]
async fn group_field_responses(
    rb: &dyn rbatis::executor::Executor,
    responses: &[String],
) -> rbatis::Result<DatabaseResult> {
    impled!()
}

pub async fn get_responses(
    context: &FeedbackFusionV1Context,
    request: Request<GetResponsesRequest>,
) -> Result<Response<ResponsePage>> {
    let data = request.into_inner();
    let page_request = data.into_page_request();
    let connection = context.connection();

    // select a page of responses
    let responses = database_request!(
        PromptResponse::select_page_by_prompt_wrapper(
            connection,
            &page_request,
            data.prompt.as_str()
        )
        .await?
    );

    let records = if responses.total > 0 {
        database_request!(
            group_field_responses(
                connection,
                responses
                    .records
                    .iter()
                    .map(|response| response.id().clone())
                    .collect::<Vec<String>>()
                    .as_slice(),
            )
            .await?
            .result
            .0
        )
        .into_iter()
        .map(|(key, value)| {
            let value = FieldResponseList {
                data: value
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<ProtoFieldResponse>>(),
            };
            (key, value)
        })
        .collect::<HashMap<String, FieldResponseList>>()
    } else {
        HashMap::new()
    };

    Ok(Response::new(ResponsePage {
        page_token: page_request.page_no().try_into()?,
        next_page_token: TryInto::<i32>::try_into(page_request.page_no())? + 1i32,
        page_size: page_request.page_size().try_into()?,
        data: records,
    }))
}
