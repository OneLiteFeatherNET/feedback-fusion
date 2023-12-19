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

use crate::{database::schema::feedback::FeedbackPrompt, prelude::*};

pub async fn router(state: FeedbackFusionState) -> Router<FeedbackFusionState> {
    Router::new()
        .route(
            "/",
            post(post_prompt)
                .get(get_prompts)
                .put(put_prompt)
                .layer(oidc_layer!()),
        )
        .route("/:prompt", delete(delete_prompt).layer(oidc_layer!()))
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
))]
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
    FeedbackPrompt::insert(state.connection(), &prompt).await?;

    Ok((StatusCode::CREATED, Json(prompt)))
}

/// GET /feedback/target/:target/prompt
#[utoipa::path(get, path = "/feedback/target/:target/prompt", params(Pagination), responses(
    (status = 200, body = Page<FeedbackPrompt>)
))]
pub async fn get_prompts(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Path(target): Path<String>,
) -> Result<Json<Page<FeedbackPrompt>>> {
    let prompts = FeedbackPrompt::select_page_by_target(
        state.connection(),
        &pagination.request(),
        target.as_str(),
    )
    .await?;

    Ok(Json(prompts))
}

/// PUT /feedback/target/:target/prompt
#[utoipa::path(put, path = "/feedback/target/:target/prompt", request_body = FeedbackPrompt, responses(
    (status = 200, body = FeedbackPrompt)
))]
pub async fn put_prompt(
    State(state): State<FeedbackFusionState>,
    Json(prompt): Json<FeedbackPrompt>,
) -> Result<Json<FeedbackPrompt>> {
    prompt.validate()?;

    FeedbackPrompt::update_by_column(state.connection(), &prompt, "id").await?;
    Ok(Json(prompt))
}

/// DELETE /feedback/target/:target/prompt/:prompt
#[utoipa::path(delete, path = "/feedback/target/:target/prompt/:prompt", responses(
    (status = 200, description = "Deleted")
))]
pub async fn delete_prompt(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
) -> Result<StatusCode> {
    FeedbackPrompt::delete_by_column(state.connection(), "id", prompt.as_str()).await?;
    Ok(StatusCode::OK)
}
