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
    cache::fields_by_prompt,
    database::schema::{
        feedback::{FieldData, FieldResponse, PromptResponse},
        user::UserContext,
    },
    prelude::*,
};
use feedback_fusion_common::proto::{
    CreateResponsesRequest, FieldResponseList, GetResponsesRequest, ProtoFieldResponse,
    ProtoPromptResponse, ResponsePage,
};
use rbatis::rbatis_codegen::IntoSql;
use std::collections::HashMap;

#[instrument(skip_all)]
pub async fn create_responses(
    context: &PublicFeedbackFusionV1Context,
    request: Request<CreateResponsesRequest>,
) -> Result<Response<ProtoPromptResponse>> {
    let data = request.into_inner();
    // start transaction
    let transaction = context.connection().acquire_begin().await?;
    let transaction = transaction.defer_async(|tx| async move {
        if !tx.done() {
            let _ = tx.rollback().await;
        }
    });

    // fetch the fields of the prompt
    let fields = fields_by_prompt(context.connection(), data.prompt.as_str()).await?;
    // as we can assume a prompt has to have at least 1 field we can throw the 400 here
    if fields.is_empty() {
        return Err(FeedbackFusionError::BadRequest("invalid prompt".to_owned()));
    }

    // insert the response dataprompt
    let response = PromptResponse::builder().prompt(data.prompt).build();
    database_request!(
        PromptResponse::insert(&transaction, &response).await,
        "Insert response id"
    )?;

    // transform the hashmap into a field data vec
    let data = data
        .data
        .into_iter()
        .map(|(key, value)| {
            // try to get the field
            let field = fields.iter().find(|f| key.eq(f.id()));

            if let Some(field) = field {
                // validate the data
                let field_data: FieldData = value.try_into()?;
                field_data.validate(field.options())?;

                Ok(FieldResponse::builder()
                    .response(response.id().as_str())
                    .field(field.id())
                    .data(field_data)
                    .build())
            } else {
                Err(FeedbackFusionError::BadRequest(format!(
                    "Invalid field '{key}'"
                )))
            }
        })
        .collect::<Result<Vec<FieldResponse>>>()?;
    // insert them as batch
    database_request!(
        FieldResponse::insert_batch(&transaction, data.as_slice(), data.len() as u64).await,
        "Insert response data"
    )?;

    // commit the transaction
    transaction.commit().await?;

    Ok(Response::new(response.into()))
}

#[py_sql(
    "`SELECT * FROM field_response`
        ` WHERE response IN `
            ${responses.sql()}"
)]
async fn field_responses(
    rb: &dyn rbatis::executor::Executor,
    responses: &[String],
) -> rbatis::Result<Vec<FieldResponse>> {
    impled!()
}

#[instrument(skip_all)]
pub async fn get_responses(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<GetResponsesRequest>,
    _user_context: UserContext,
) -> Result<Response<ResponsePage>> {
    let data = request.into_inner();
    let page_request = data.page_request();
    let connection = context.connection();

    // select a page of responses
    let responses = database_request!(
        PromptResponse::select_page_by_prompt(
            connection,
            &page_request,
            data.prompt.as_str()
        )
        .await,
        "Select responses by prompt"
    )?;

    warn!("{:?}", responses);

    let records = if responses.total > 0 {
        database_request!(
            field_responses(
                connection,
                responses
                    .records
                    .iter()
                    .map(|response| response.id().clone())
                    .collect::<Vec<String>>()
                    .as_slice(),
            )
            .await,
            "Select responses by id"
        )?
        .into_iter()
        .chunk_by(|value| value.response().clone())
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

    warn!("{records:?}");

    Ok(Response::new(ResponsePage {
        page_token: page_request.page_no().try_into()?,
        next_page_token: TryInto::<i32>::try_into(page_request.page_no())? + 1i32,
        page_size: page_request.page_size().try_into()?,
        total: responses.total.try_into()?,
        data: records,
    }))
}
