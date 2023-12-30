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

use std::collections::HashMap;

use axum::extract::Path;
use rbatis::{rbdc::JsonV, sql::Page};

use crate::{
    database::schema::feedback::{
        FeedbackPromptField, FeedbackPromptFieldData, FeedbackPromptFieldResponse,
        FeedbackPromptResponse,
    },
    prelude::*,
};

pub async fn router(state: FeedbackFusionState) -> Router<FeedbackFusionState> {
    Router::new()
        .route("/", post(post_response))
        .with_state(state)
}

#[derive(Deserialize, Clone, Debug, ToSchema)]
pub struct SubmitFeedbackPromptResponseRequest {
    responses: HashMap<String, FeedbackPromptFieldData>,
}

/// POST /target/:target/prompt/:prompt/response
pub async fn post_response(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
    Json(data): Json<SubmitFeedbackPromptResponseRequest>,
) -> Result<Json<FeedbackPromptResponse>> {
    // start transaction
    let mut transaction = state.connection().acquire_begin().await?;
    // fetch the fields of the prompt
    let fields = database_request!(
        FeedbackPromptField::select_by_column(&transaction, "prompt", prompt.as_str()).await?
    );
    // as we can assume a prompt has to have at least 1 field we can throw the 400 here
    if fields.len() == 0 {
        return Err(FeedbackFusionError::BadRequest("invalid prompt".to_owned()));
    }

    // insert the response dataprompt
    let response = FeedbackPromptResponse::builder().prompt(prompt).build();
    database_request!(FeedbackPromptResponse::insert(&transaction, &response).await?);

    // transform the hashmap into a field data vec
    let data = data
        .responses
        .into_iter()
        .filter_map(|(field, value)| {
            // validate the type of field and response
            if fields
                .iter()
                .find(|f| field.eq(f.id()) && f.r#type().eq(&value))
                .is_some()
            {
                Some(
                    FeedbackPromptFieldResponse::builder()
                        .response(response.id().as_str())
                        .field(field)
                        .data(JsonV(value))
                        .build(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<FeedbackPromptFieldResponse>>();
    // insert them as batch
    database_request!(
        FeedbackPromptFieldResponse::insert_batch(&transaction, data.as_slice(), data.len() as u64)
            .await?
    );

    // commit the transaction
    transaction.commit().await?;

    Ok(Json(response))
}

