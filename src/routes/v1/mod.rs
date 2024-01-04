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
use validator::Validate;

use crate::{database::schema::feedback::FeedbackTarget, prelude::*};

pub mod prompt;
pub mod response;

pub async fn router(state: FeedbackFusionState) -> (Router, Router) {
    (
        Router::new()
            .route("/target", post(post_target).get(get_targets))
            .route(
                "/target/:target",
                get(get_target).delete(delete_target).put(put_target),
            )
            .nest(
                "/target/:target/prompt",
                prompt::router(state.clone()).await,
            )
            .nest(
                "/target/:target/prompt/:prompt/response",
                response::router(state.clone()).await,
            )
            .with_state(state.clone()),
        Router::new()
            .route("/target/:target/prompt/:prompt/fetch", get(prompt::fetch))
            .route(
                "/target/:target/prompt/:prompt/response",
                post(response::post_response),
            )
            .with_state(state),
    )
}

#[derive(ToSchema, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct CreateFeedbackTargetRequest {
    #[validate(length(max = 255))]
    name: String,
    description: Option<String>,
}

/// POST /v1/target
#[utoipa::path(post, path = "/v1/target", request_body = CreateFeedbackTargetRequest, tag = "FeedbackTarget", responses(
    (status = 201, description = "Target created", body = FeedbackTarget)
), security(("oidc" = ["feedback-fusion:write"])))]
pub async fn post_target(
    State(state): State<FeedbackFusionState>,
    _guard: scope::Write,
    Json(data): Json<CreateFeedbackTargetRequest>,
) -> Result<(StatusCode, Json<FeedbackTarget>)> {
    let connection = state.connection();

    // validate
    data.validate()?;
    // build
    let target = FeedbackTarget::builder()
        .name(data.name)
        .description(data.description)
        .build();

    // create the target
    database_request!(FeedbackTarget::insert(connection, &target).await?);

    Ok((StatusCode::CREATED, Json(target)))
}

/// GET /v1/target
#[utoipa::path(get, path = "/v1/target", params(SearchQuery, Pagination ), tag = "FeedbackTarget", responses(
    (status = 200, description = "Page of Targets", body = FeedbackTargetPage)
), security(("oidc" = ["feedback-fusion:read"])))]
pub async fn get_targets(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Query(search): Query<SearchQuery>,
    _guard: scope::Read,
) -> Result<Json<Page<FeedbackTarget>>> {
    let connection = state.connection();

    // fetch the Page
    let page = database_request!(
        FeedbackTarget::select_page_wrapper(
            connection,
            &pagination.request(),
            search.query.as_str()
        )
        .await?
    );
    Ok(Json(page))
}

/// GET /v1/target/:target
#[utoipa::path(get, path = "/v1/target/:id", tag = "FeedbackTarget", responses(
    (status = 200, description = "Target", body = FeedbackTarget),
    (status = 400, description = "Target not found")
), security(("oidc" = ["feedback-fusion:read"])))]
pub async fn get_target(
    State(state): State<FeedbackFusionState>,
    Path(target): Path<String>,
    _guard: scope::Read,
) -> Result<Json<FeedbackTarget>> {
    let connection = state.connection();

    let target =
        database_request!(FeedbackTarget::select_by_id(connection, target.as_str()).await?);
    match target {
        Some(target) => Ok(Json(target)),
        None => Err(FeedbackFusionError::BadRequest(
            "Target not found".to_owned(),
        )),
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct PutFeedbackTargetRequest {
    #[validate(length(max = 255))]
    name: Option<String>,
    #[validate(length(max = 255))]
    description: Option<String>,
}

/// PUT /v1/target/:target
#[utoipa::path(put, path = "/v1/target/:target", request_body = PutFeedbackTargetRequest, tag = "FeedbackTarget", responses(
    (status = 200, description = "Updated", body = FeedbackTarget)
), security(("oidc" = ["feedback-fusion:write"])))]
pub async fn put_target(
    State(state): State<FeedbackFusionState>,
    Path(target): Path<String>,
    _guard: scope::Write,
    Json(data): Json<PutFeedbackTargetRequest>,
) -> Result<Json<FeedbackTarget>> {
    data.validate()?;

    let mut target = database_request!(FeedbackTarget::select_by_id(
        state.connection(),
        target.as_str()
    )
    .await?
    .ok_or(FeedbackFusionError::BadRequest("not found".to_owned()))?);
    target.set_name(data.name.unwrap_or(target.name().clone()));
    target.set_description(data.description.or(target.description().clone()));

    database_request!(FeedbackTarget::update_by_column(state.connection(), &target, "id").await?);
    Ok(Json(target))
}

/// DELETE /v1/target/:target
#[utoipa::path(delete, path = "/v1/target/:target", tag = "FeedbackTarget", responses(
    (status = 200, description = "Deleted")
), security(("oidc" = ["feedback-fusion:write"])))]
pub async fn delete_target(
    State(state): State<FeedbackFusionState>,
    Path(target): Path<String>,
    _guard: scope::Write,
) -> Result<StatusCode> {
    let connection = state.connection();

    database_request!(FeedbackTarget::delete_by_column(connection, "id", target.as_str()).await?);
    Ok(StatusCode::OK)
}
