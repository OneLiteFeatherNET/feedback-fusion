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

use axum::{extract::Path, http::StatusCode};
use rbatis::sql::Page;
use validator::Validate;

use crate::{
    database::schema::feedback::{
        FeedbackPrompt, FeedbackPromptField, FeedbackPromptInputOptions, FeedbackPromptInputType,
    },
    prelude::*,
};

pub async fn router(state: FeedbackFusionState) -> Router<FeedbackFusionState> {
    Router::new()
        .route("/", post(post_prompt).get(get_prompts).put(put_prompt))
        .route("/:prompt", delete(delete_prompt))
        .route(
            "/:prompt/field",
            post(post_field).put(put_field).get(get_fields),
        )
        .route("/:prompt/field/:field", delete(delete_field))
        .layer(oidc_layer!())
        .with_state(state)
}

#[derive(ToSchema, Deserialize, Debug, Clone, Validate)]
pub struct CreateFeedbackPromptRequest {
    #[validate(length(max = 255))]
    title: String,
    #[serde(default)]
    active: bool,
}

/// POST /feedback/target/:target/prompt
#[utoipa::path(post, path = "/feedback/target/:target/prompt", request_body = CreateFeedbackPromptRequest, responses(
    (status = 201, body = FeedbackPrompt)
), tag = "FeedbackTargetPrompt")]
pub async fn post_prompt(
    State(state): State<FeedbackFusionState>,
    Path(target): Path<String>,
    Json(data): Json<CreateFeedbackPromptRequest>,
) -> Result<(StatusCode, Json<FeedbackPrompt>)> {
    data.validate()?;

    // build the prompt
    let prompt = FeedbackPrompt::builder()
        .title(data.title)
        .active(data.active)
        .target(target)
        .build();
    database_request!(FeedbackPrompt::insert(state.connection(), &prompt).await?);

    Ok((StatusCode::CREATED, Json(prompt)))
}

/// GET /feedback/target/:target/prompt
#[utoipa::path(get, path = "/feedback/target/:target/prompt", params(Pagination), responses(
    (status = 200, body = FeedbackPromptPage)
), tag = "FeedbackTargetPrompt")]
pub async fn get_prompts(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Path(target): Path<String>,
) -> Result<Json<Page<FeedbackPrompt>>> {
    let prompts = database_request!(
        FeedbackPrompt::select_page_by_target(
            state.connection(),
            &pagination.request(),
            target.as_str(),
        )
        .await?
    );

    Ok(Json(prompts))
}

/// PUT /feedback/target/:target/prompt
#[utoipa::path(put, path = "/feedback/target/:target/prompt", request_body = FeedbackPrompt, responses(
    (status = 200, body = FeedbackPrompt)
), tag = "FeedbackTargetPrompt")]
pub async fn put_prompt(
    State(state): State<FeedbackFusionState>,
    Json(prompt): Json<FeedbackPrompt>,
) -> Result<Json<FeedbackPrompt>> {
    prompt.validate()?;

    database_request!(FeedbackPrompt::update_by_column(state.connection(), &prompt, "id").await?);
    Ok(Json(prompt))
}

/// DELETE /feedback/target/:target/prompt/:prompt
#[utoipa::path(delete, path = "/feedback/target/:target/prompt/:prompt", responses(
    (status = 200, description = "Deleted")
), tag = "FeedbackTargetPrompt")]
pub async fn delete_prompt(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
) -> Result<StatusCode> {
    database_request!(
        FeedbackPrompt::delete_by_column(state.connection(), "id", prompt.as_str()).await?
    );
    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, ToSchema, Deserialize, Validate)]
pub struct CreateFeedbackPromptFieldRequest {
    title: String,
    r#type: FeedbackPromptInputType,
    options: FeedbackPromptInputOptions,
}

/// POST /feedback/target/:target/prompt/:prompt/field
#[utoipa::path(post, path = "/feedback/target/:target/prompt/:prompt/field", request_body = CreateFeedbackPromptFieldRequest, responses(
    (status = 201, description = "Created", body = FeedbackPromptField)
), tag = "FeedbackTargetPromptField")]
pub async fn post_field(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
    Json(data): Json<CreateFeedbackPromptFieldRequest>,
) -> Result<(StatusCode, Json<FeedbackPromptField>)> {
    data.validate()?;

    // build the field
    let field = FeedbackPromptField::builder()
        .title(data.title)
        .r#type(data.r#type)
        .options(data.options)
        .prompt(prompt)
        .build();
    database_request!(FeedbackPromptField::insert(state.connection(), &field).await?);

    Ok((StatusCode::CREATED, Json(field)))
}

/// GET /feedback/target/:target/prompt/:prompt/field
#[utoipa::path(get, path = "/feedback/target/:target/prompt/:prompt/field", params(Pagination), responses(
    (status = 200, body = FeedbackPromptFieldPage)
), tag = "FeedbackTargetPromptField")]
pub async fn get_fields(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Path((_, prompt)): Path<(String, String)>,
) -> Result<Json<Page<FeedbackPromptField>>> {
    let page = database_request!(
        FeedbackPromptField::select_page_by_prompt(
            state.connection(),
            &pagination.request(),
            prompt.as_str()
        )
        .await?
    );

    Ok(Json(page))
}

/// PUT /feedback/target/:target/prompt/:prompt/field
#[utoipa::path(put, path = "/feedback/target/:target/prompt/:prompt/field", request_body = FeedbackPromptField, responses(
    (status = 200, body = FeedbackPromptField, description = "updated")
), tag = "FeedbackTargetPromptField")]
pub async fn put_field(
    State(state): State<FeedbackFusionState>,
    Json(data): Json<FeedbackPromptField>,
) -> Result<Json<FeedbackPromptField>> {
    data.validate()?;

    database_request!(
        FeedbackPromptField::update_by_column(state.connection(), &data, "id").await?
    );
    Ok(Json(data))
}

/// DELETE /feedback/target/:target/prompt/:prompt/field/:field
#[utoipa::path(delete, path = "/feedback/target/:target/prompt/:prompt/field/:field", responses(
    (status = 200, description = "Deleted")
), tag = "FeedbackTargetPromptField")]
pub async fn delete_field(
    State(state): State<FeedbackFusionState>,
    Path((_, _, field)): Path<(String, String, String)>,
) -> Result<StatusCode> {
    database_request!(
        FeedbackPromptField::delete_by_column(state.connection(), "id", field.as_str()).await?
    );

    Ok(StatusCode::OK)
}
