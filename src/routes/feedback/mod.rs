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

use crate::{database::schema::feedback::FeedbackTarget, prelude::*};

pub mod prompt;

pub async fn router(state: FeedbackFusionState) -> Router {
    Router::new()
        .route(
            "/target",
            post(post_target)
                .put(put_target)
                .get(get_targets)
                .get(get_target)
                .delete(delete_target)
                .layer(oidc_layer!()),
        )
        .nest(
            "/target/:target/prompt",
            prompt::router(state.clone()).await,
        )
        .with_state(state)
}

#[derive(ToSchema, Deserialize, Debug, Clone, Validate)]
pub struct CreateFeedbackTargetRequest {
    #[validate(length(max = 255))]
    name: String,
    description: Option<String>,
}

/// POST /feedback/target
#[utoipa::path(post, path = "/feedback/target", request_body = CreateFeedbackTargetRequest, tag = "FeedbackTarget", responses(
    (status = 201, description = "Target created", body = FeedbackTarget)
))]
pub async fn post_target(
    State(state): State<FeedbackFusionState>,
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

/// GET /feedback/target
#[utoipa::path(get, path = "/feedback/target", params(SearchQuery, Pagination ), tag = "FeedbackTarget", responses(
    (status = 200, description = "Page of Targets", body = FeedbackTargetPage)
))]
pub async fn get_targets(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Query(search): Query<SearchQuery>,
) -> Result<Json<Page<FeedbackTarget>>> {
    let connection = state.connection();

    // fetch the Page
    let page = database_request!(
        FeedbackTarget::select_page(connection, &pagination.request(), search.query.as_str())
            .await?
    );
    Ok(Json(page))
}

/// GET /feedback/target/:id
#[utoipa::path(get, path = "/feedback/target/:id", tag = "FeedbackTarget", responses(
    (status = 200, description = "Target", body = FeedbackTarget),
    (status = 400, description = "Target not found")
))]
pub async fn get_target(
    State(state): State<FeedbackFusionState>,
    Path(id): Path<String>,
) -> Result<Json<FeedbackTarget>> {
    let connection = state.connection();

    let target = database_request!(FeedbackTarget::select_by_id(connection, id.as_str()).await?);
    match target {
        Some(target) => Ok(Json(target)),
        None => Err(FeedbackFusionError::BadRequest(
            "Target not found".to_owned(),
        )),
    }
}

/// PUT /feedback/target
#[utoipa::path(put, path = "/feedback/target", request_body = FeedbackTarget, tag = "FeedbackTarget", responses(
    (status = 200, description = "Updated", body = FeedbackTarget)
))]
pub async fn put_target(
    State(state): State<FeedbackFusionState>,
    Json(target): Json<FeedbackTarget>,
) -> Result<Json<FeedbackTarget>> {
    let connection = state.connection();

    target.validate()?;

    database_request!(FeedbackTarget::update_by_column(connection, &target, "id").await?);
    Ok(Json(target))
}

/// DELETE /feedback/target/:id
#[utoipa::path(delete, path = "/feedback/target/:id", tag = "FeedbackTarget", responses(
    (status = 200, description = "Deleted")
))]
pub async fn delete_target(
    State(state): State<FeedbackFusionState>,
    Path(id): Path<String>,
) -> Result<StatusCode> {
    let connection = state.connection();

    database_request!(FeedbackTarget::delete_by_column(connection, "id", id.as_str()).await?);
    Ok(StatusCode::OK)
}
