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

use super::FeedbackFusionV1Context;
use crate::{
    database::schema::feedback::{FieldResponse, PromptResponse},
    prelude::*,
};
use feedback_fusion_common::proto::{
    FieldResponse as ProtoFieldResponse, FieldResponseList, GetResponsesRequest, ResponsePage,
};
use rbatis::rbatis_codegen::IntoSql;
use std::collections::HashMap;

// #[derive(Deserialize, Clone, Debug, ToSchema)]
// pub struct SubmitFeedbackPromptResponseRequest {
//     #[cfg(not(feature = "bindings"))]
//     responses: HashMap<String, serde_json::Value>,
//     #[cfg(feature = "bindings")]
//     responses: HashMap<String, FieldData>,
// }
//
// pub async fn post_response(
//    (state): State<FeedbackFusionState>,
//     Path((_, prompt)): Path<(String, String)>,
//     Json(data): Json<SubmitFeedbackPromptResponseRequest>,
// ) -> Result<(StatusCode, Json<PromptResponse>)> {
//     // start transaction
//     let transaction =.connection().acquire_begin().await?;
//     let mut transaction = transaction.defer_async(|mut tx| async move {
//         if !tx.done {
//             let _ = tx.rollback().await;
//         }
//     });
//
//     // fetch the fields of the prompt
//     let fields = database_request!(
//         FeedbackPromptField::select_by_column(&transaction, "prompt", prompt.as_str()).await?
//     );
//     // as we can assume a prompt has to have at least 1 field we can throw the 400 here
//     if fields.is_empty() {
//         return Err(FeedbackFusionError::BadRequest("invalid prompt".to_owned()));
//     }
//
//     // insert the response dataprompt
//     let response = PromptResponse::builder().prompt(prompt).build();
//     database_request!(PromptResponse::insert(&transaction, &response).await?);
//
//     // transform the hashmap into a field data vec
//     let data = data
//         .responses
//         .into_iter()
//         .map(|(key, value)| {
//             // try to get the field
//             let field = fields.iter().find(|f| key.eq(f.id()));
//
//             if let Some(field) = field {
//                 let value = FieldData::parse(field.r#type(), value)?;
//                 // validate the data
//                 value.validate(field.options())?;
//
//                 #[cfg(not(feature = "bindings"))]
//                 let value = JsonV(value);
//
//                 Ok(FieldResponse::builder()
//                     .response(response.id().as_str())
//                     .field(field.id())
//                     .data(value)
//                     .build())
//             } else {
//                 Err(FeedbackFusionError::BadRequest(format!(
//                     "Invalid field '{}'",
//                     key
//                 )))
//             }
//         })
//         .collect::<Result<Vec<FieldResponse>>>()?;
//     // insert them as batch
//     database_request!(
//         FieldResponse::insert_batch(&transaction, data.as_slice(), data.len() as u64).await?
//     );
//
//     // commit the transaction
//     transaction.commit().await?;
//
//     Ok((StatusCode::CREATED, Json(response)))
// }

pub type GetFeedbackPromptResponsesResponse = HashMap<String, Vec<FieldResponse>>;

#[derive(Deserialize, Debug, Clone)]
struct DatabaseResult {
    result: JsonV<GetFeedbackPromptResponsesResponse>,
}

// impl Into<Responses> for DatabaseResult {
//     fn into(self) -> Responses {
//         let data = self.result.0;
//
//         for value in data.values_mut() {
//             *value = FieldResponseList { data: value.into() };
//         }
//
//         Responses { data }
//     }
// }

#[py_sql(
    "`SELECT jsonb_object_agg(response, rows) AS RESULT FROM (`
        `SELECT response, `
        `jsonb_agg(jsonb_build_object('id', id, 'response', response, 'field', field, 'data', data)) AS ROWS `
            `FROM feedback_prompt_field_response `
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
