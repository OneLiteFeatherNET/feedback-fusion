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

use crate::{
    database::schema::feedback::{
        FeedbackPrompt, FeedbackPromptField, FeedbackPromptInputOptions, FeedbackPromptInputType,
    },
    prelude::*,
};

pub async fn router(state: FeedbackFusionState) -> Router<FeedbackFusionState> {
    Router::new()
        .route("/", post(post_prompt).get(get_prompts))
        .route("/:prompt", delete(delete_prompt).put(put_prompt))
        .route("/:prompt/field", post(post_field).get(get_fields))
        .route("/:prompt/field/:field", delete(delete_field).put(put_field))
        .with_state(state)
}

#[derive(ToSchema, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct CreateFeedbackPromptRequest {
    #[validate(length(max = 255))]
    title: String,
    #[serde(default)]
    active: bool,
}

/// POST /v1/target/:target/prompt
#[utoipa::path(post, path = "/v1/target/:target/prompt", request_body = CreateFeedbackPromptRequest, responses(
    (status = 201, body = FeedbackPrompt)
), tag = "FeedbackTargetPrompt", security(("oidc" = ["feedback-fusion:write"])))]
pub async fn post_prompt(
    State(state): State<FeedbackFusionState>,
    Path(target): Path<String>,
    _guard: scope::Write,
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

/// GET /v1/target/:target/prompt/:prompt
#[utoipa::path(get, path = "/v1/target/:target/prompt/:prompt", responses(
    (status = 200, body = FeedbackPrompt)
), tag = "FeedbackTargetPrompt", security(()))]
pub async fn get_prompt(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
) -> Result<Json<FeedbackPrompt>> {
    let prompt: Option<FeedbackPrompt> =
        database_request!(FeedbackPrompt::select_by_id(state.connection(), prompt.as_str()).await?);

    match prompt {
        Some(prompt) => Ok(Json(prompt)),
        None => Err(FeedbackFusionError::BadRequest(
            "invalid prompt".to_string(),
        )),
    }
}

/// GET /v1/target/:target/prompt
#[utoipa::path(get, path = "/v1/target/:target/prompt", params(Pagination), responses(
    (status = 200, body = FeedbackPromptPage)
), tag = "FeedbackTargetPrompt", security(("oidc" = ["feedback-fusion:read"])))]
pub async fn get_prompts(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Path(target): Path<String>,
    _guard: scope::Read,
) -> Result<Json<Page<FeedbackPrompt>>> {
    let prompts = database_request!(
        FeedbackPrompt::select_page_by_target_wrapper(
            state.connection(),
            &pagination.request(),
            target.as_str(),
        )
        .await?
    );

    Ok(Json(prompts))
}

#[derive(Deserialize, Debug, Clone, ToSchema, Validate)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct PutFeedbackPromptRequest {
    #[validate(length(max = 255))]
    title: Option<String>,
    active: Option<bool>,
}

/// PUT /v1/target/:target/prompt/:prompt
#[utoipa::path(put, path = "/v1/target/:target/prompt/:prompt", request_body = PutFeedbackPromptRequest, responses(
    (status = 200, body = FeedbackPrompt)
), tag = "FeedbackTargetPrompt", security(("oidc" = ["feedback-fusion:write"])))]
pub async fn put_prompt(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
    _guard: scope::Write,
    Json(data): Json<PutFeedbackPromptRequest>,
) -> Result<Json<FeedbackPrompt>> {
    data.validate()?;
    let mut prompt = database_request!(FeedbackPrompt::select_by_id(
        state.connection(),
        prompt.as_str()
    )
    .await?
    .ok_or(FeedbackFusionError::BadRequest("not found".to_owned()))?);

    prompt.set_title(data.title.unwrap_or(prompt.title().clone()));
    prompt.set_active(data.active.unwrap_or(*prompt.active()));

    database_request!(FeedbackPrompt::update_by_column(state.connection(), &prompt, "id").await?);
    Ok(Json(prompt))
}

/// DELETE /v1/target/:target/prompt/:prompt
#[utoipa::path(delete, path = "/v1/target/:target/prompt/:prompt", responses(
    (status = 200, description = "Deleted")
), tag = "FeedbackTargetPrompt", security(("oidc" = ["feedback-fusion:write"])))]
pub async fn delete_prompt(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
    _guard: scope::Write,
) -> Result<StatusCode> {
    database_request!(
        FeedbackPrompt::delete_by_column(state.connection(), "id", prompt.as_str()).await?
    );
    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, ToSchema, Deserialize, Validate)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct CreateFeedbackPromptFieldRequest {
    #[validate(length(max = 255))]
    title: String,
    r#type: FeedbackPromptInputType,
    options: FeedbackPromptInputOptions,
}

/// POST /v1/target/:target/prompt/:prompt/field
#[utoipa::path(post, path = "/v1/target/:target/prompt/:prompt/field", request_body = CreateFeedbackPromptFieldRequest, responses(
    (status = 201, description = "Created", body = FeedbackPromptField)
), tag = "FeedbackTargetPromptField", security(("oidc" = ["feedback-fusion:write"])))]
pub async fn post_field(
    State(state): State<FeedbackFusionState>,
    Path((_, prompt)): Path<(String, String)>,
    _guard: scope::Write,
    Json(data): Json<CreateFeedbackPromptFieldRequest>,
) -> Result<(StatusCode, Json<FeedbackPromptField>)> {
    data.validate()?;
    // validate type and enum
    if !data.r#type.eq(&data.options) {
        return Err(FeedbackFusionError::BadRequest(
            "type does not match".to_owned(),
        ));
    };

    // build the field
    #[cfg(not(feature = "bindings"))]
    let options = JsonV(data.options);
    #[cfg(feature = "bindings")]
    let options = data.options;
    let field = FeedbackPromptField::builder()
        .title(data.title)
        .r#type(data.r#type)
        .options(options)
        .prompt(prompt)
        .build();
    database_request!(FeedbackPromptField::insert(state.connection(), &field).await?);

    Ok((StatusCode::CREATED, Json(field)))
}

/// GET /v1/target/:target/prompt/:prompt/fetch
#[utoipa::path(get, path = "/v1/target/:target/prompt/:prompt/fetch", params(Pagination), responses(
    (status = 200, body = FeedbackPromptFieldPage)
), security(()), tag = "FeedbackTargetPromptField")]
pub async fn fetch(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Path((_, prompt)): Path<(String, String)>,
) -> Result<Json<Page<FeedbackPromptField>>> {
    // fetch the prompt
    let prompt = database_request!(FeedbackPrompt::select_by_id(
        state.connection(),
        prompt.as_str()
    )
    .await?
    .ok_or(FeedbackFusionError::BadRequest(
        "invalid prompt".to_string()
    ))?);
    // only allow active prompts
    if !prompt.active() {
        return Err(FeedbackFusionError::Forbidden("inactive prompt".to_owned()));
    }

    let page = database_request!(
        FeedbackPromptField::select_page_by_prompt_wrapper(
            state.connection(),
            &pagination.request(),
            prompt.id().as_str()
        )
        .await?
    );

    Ok(Json(page))
}

/// GET /v1/target/:target/prompt/:prompt/field
#[utoipa::path(get, path = "/v1/target/:target/prompt/:prompt/field", params(Pagination), responses(
    (status = 200, body = FeedbackPromptFieldPage)
), tag = "FeedbackTargetPromptField", security(("oidc" = ["feedback-fusion:read"])), security(()))]
pub async fn get_fields(
    State(state): State<FeedbackFusionState>,
    Query(pagination): Query<Pagination>,
    Path((_, prompt)): Path<(String, String)>,
    _guard: scope::Read,
) -> Result<Json<Page<FeedbackPromptField>>> {
    let page = database_request!(
        FeedbackPromptField::select_page_by_prompt_wrapper(
            state.connection(),
            &pagination.request(),
            prompt.as_str()
        )
        .await?
    );

    Ok(Json(page))
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct PutFeedbackPromptFieldRequest {
    #[validate(length(max = 255))]
    title: Option<String>,
    options: Option<FeedbackPromptInputOptions>,
}

/// PUT /v1/target/:target/prompt/:prompt/field/:field
#[utoipa::path(put, path = "/v1/target/:target/prompt/:prompt/field/:field", request_body = PutFeedbackPromptFieldRequest, responses(
    (status = 200, body = FeedbackPromptField, description = "updated")
), tag = "FeedbackTargetPromptField", security(("oidc" = ["feedback-fusion:write"])))]
pub async fn put_field(
    State(state): State<FeedbackFusionState>,
    Path((_, _, field)): Path<(String, String, String)>,
    _guard: scope::Write,
    Json(data): Json<PutFeedbackPromptFieldRequest>,
) -> Result<Json<FeedbackPromptField>> {
    data.validate()?;

    let mut field = database_request!(FeedbackPromptField::select_by_id(
        state.connection(),
        field.as_str()
    )
    .await?
    .ok_or(FeedbackFusionError::BadRequest("not found".to_owned()))?);
    // validate type and enum
    if data
        .options
        .as_ref()
        .is_some_and(|options| !field.r#type().eq(options))
    {
        return Err(FeedbackFusionError::BadRequest(
            "type does not match".to_owned(),
        ));
    };

    field.set_title(data.title.unwrap_or(field.title().to_string()));
    if let Some(options) = data.options {
        if field.r#type().eq(&options) {
            #[cfg(not(feature = "bindings"))]
            field.set_options(JsonV(options));
        }
    }

    database_request!(
        FeedbackPromptField::update_by_column(state.connection(), &field, "id").await?
    );
    Ok(Json(field))
}

/// DELETE /v1/target/:target/prompt/:prompt/field/:field
#[utoipa::path(delete, path = "/v1/target/:target/prompt/:prompt/field/:field", responses(
    (status = 200, description = "Deleted")
), tag = "FeedbackTargetPromptField", security(("oidc" = ["feedback-fusion:write"])))]
pub async fn delete_field(
    State(state): State<FeedbackFusionState>,
    Path((_, _, field)): Path<(String, String, String)>,
    _guard: scope::Write,
) -> Result<StatusCode> {
    database_request!(
        FeedbackPromptField::delete_by_column(state.connection(), "id", field.as_str()).await?
    );

    Ok(StatusCode::OK)
}
