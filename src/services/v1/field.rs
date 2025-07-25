//SPDX-FileCopyrightText: 2024 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2024 OneLiteFeatherNet

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
    cache::fetch_prompt,
    database::schema::{
        feedback::{Field, FieldOptions, FieldType},
        user::UserContext,
    },
    prelude::*,
};
use feedback_fusion_common::proto::{
    CreateFieldRequest, DeleteFieldRequest, Field as ProtoField, FieldPage, GetFieldsRequest,
    UpdateFieldRequest,
};

#[instrument(skip_all)]
pub async fn create_field(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<CreateFieldRequest>,
    _user_context: UserContext,
) -> Result<Response<ProtoField>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.connection();

    // build the field
    let field = Field::builder()
        .field_type(Into::<FieldType>::into(data.field_type()))
        .title(data.title)
        .description(data.description)
        .options(TryInto::<FieldOptions>::try_into(data.options.ok_or(
            FeedbackFusionError::BadRequest("missing fieldOptions".to_owned()),
        )?)?)
        .prompt(data.prompt)
        .build();
    database_request!(Field::insert(connection, &field).await, "Insert field")?;

    invalidate!(fields_by_prompt, format!("prompt-{}", field.prompt()));

    Ok(Response::new(field.into()))
}

#[instrument(skip_all)]
pub async fn get_active_fields(
    context: &PublicFeedbackFusionV1Context,
    request: Request<GetFieldsRequest>,
) -> Result<Response<FieldPage>> {
    let data = request.into_inner();
    let page_request = data.page_request();
    let connection = context.connection();

    // fetch the prompt
    let prompt = fetch_prompt(connection, data.prompt.as_str()).await?;

    let prompt = prompt.ok_or(FeedbackFusionError::BadRequest(
        "invalid prompt".to_string(),
    ))?;
    // only allow active prompts
    if !prompt.active() {
        return Err(FeedbackFusionError::Forbidden("inactive prompt".to_owned()));
    }

    // may consider caching this as well
    let page = database_request!(
        Field::select_page_by_prompt_wrapper(connection, &page_request, prompt.id().as_str()).await,
        "Select fields by prompt"
    )?;

    Ok(Response::new(FieldPage {
        page_token: page_request.page_no().try_into()?,
        next_page_token: TryInto::<i32>::try_into(page_request.page_no())? + 1i32,
        page_size: page_request.page_size().try_into()?,
        total: page.total.try_into()?,
        fields: page
            .records
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ProtoField>>(),
    }))
}

#[instrument(skip_all)]
pub async fn get_fields(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<GetFieldsRequest>,
    _user_context: UserContext,
) -> Result<Response<FieldPage>> {
    let data = request.into_inner();
    let page_request = data.page_request();

    let page = database_request!(
        Field::select_page_by_prompt_wrapper(
            context.connection(),
            &page_request,
            data.prompt.as_str()
        )
        .await,
        "Select fields by prompt"
    )?;

    Ok(Response::new(FieldPage {
        page_token: page_request.page_no().try_into()?,
        next_page_token: TryInto::<i32>::try_into(page_request.page_no())? + 1i32,
        page_size: page_request.page_size().try_into()?,
        total: page.total.try_into()?,
        fields: page
            .records
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ProtoField>>(),
    }))
}

#[instrument(skip_all)]
pub async fn update_field(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<UpdateFieldRequest>,
    _user_context: UserContext,
) -> Result<Response<ProtoField>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.connection();

    let mut field = database_request!(
        Field::select_by_id(connection, data.id.as_str()).await,
        "Select field by id"
    )?
    .ok_or(FeedbackFusionError::BadRequest("not found".to_owned()))?;

    if let Some(title) = data.title {
        field.set_title(title);
    };
    if let Some(description) = data.description {
        field.set_description(Some(description));
    };
    if let Some(options) = data.options {
        field.set_options(options.try_into()?);
    };

    database_request!(
        Field::update_by_column(connection, &field, "id").await,
        "Update field by id"
    )?;

    invalidate!(fields_by_prompt, format!("prompt-{}", field.prompt()));

    Ok(Response::new(field.into()))
}

#[instrument(skip_all)]
pub async fn delete_field(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<DeleteFieldRequest>,
    _user_context: UserContext,
) -> Result<Response<()>> {
    database_request!(
        Field::delete_by_column(context.connection(), "id", request.into_inner().id.as_str()).await,
        "Delete field by id"
    )?;

    Ok(Response::new(()))
}
