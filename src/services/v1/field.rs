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

use super::FeedbackFusionV1Context;
use crate::{database::schema::feedback::Field, prelude::*};
use feedback_fusion_common::proto::{
    CreateFieldRequest, DeleteFieldRequest, Field as ProtoField, FieldPage, GetFieldsRequest,
    UpdateFieldRequest,
};

pub async fn create_field(
    context: &FeedbackFusionV1Context,
    request: Request<CreateFieldRequest>,
) -> Result<Response<Field>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.state.connection();

    // build the field
    let field = Field::builder()
        .title(data.title)
        .description(data.description)
        .r#type(data.r#type.clone())
        .options(JsonV(data.options))
        .prompt(prompt)
        .build();
    database_request!(Field::insert(connection, &field).await?);

    Ok(Response::new(field.into()))
}

// pub async fn fetch(
//     State(state): State<FeedbackFusionState>,
//     Query(pagination): Query<Pagination>,
//     Path((_, prompt)): Path<(String, String)>,
// ) -> Result<Json<Page<FeedbackPromptField>>> {
//     // fetch the prompt
//     let prompt = database_request!(FeedbackPrompt::select_by_id(
//         state.connection(),
//         prompt.as_str()
//     )
//     .await?
//     .ok_or(FeedbackFusionError::BadRequest(
//         "invalid prompt".to_string()
//     ))?);
//     // only allow active prompts
//     if !prompt.active() {
//         return Err(FeedbackFusionError::Forbidden("inactive prompt".to_owned()));
//     }
//
//     let page = database_request!(
//         FeedbackPromptField::select_page_by_prompt_wrapper(
//             state.connection(),
//             &pagination.request(),
//             prompt.id().as_str()
//         )
//         .await?
//     );
//
//     Ok(Json(page))
// }

pub async fn get_fields(
    context: &FeedbackFusionV1Context,
    request: Request<GetFieldsRequest>,
) -> Result<Response<FieldPage>> {
    let page = database_request!(
        Field::select_page_by_prompt_wrapper(
            context.state.connection(),
            &pagination.request(),
            request.into_inner().prompt.as_str()
        )
        .await?
    );

    Ok(Response::new(page.into()))
}

pub async fn update_field(
    context: &FeedbackFusionV1Context,
    request: Request<UpdateFieldRequest>,
) -> Result<Response<ProtoField>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.state.connection();

    let mut field = database_request!(Field::select_by_id(connection, data.id.as_str())
        .await?
        .ok_or(FeedbackFusionError::BadRequest("not found".to_owned()))?);

    field.set_title(data.title.unwrap_or(field.title().to_string()));
    if let Some(Ok(options)) = options {
        field.set_options(JsonV(options));
    }

    database_request!(Field::update_by_column(connection, &field, "id").await?);

    Ok(Response::new(field.into()))
}

pub async fn delete_field(
    context: &FeedbackFusionV1Context,
    request: Request<DeleteFieldRequest>,
) -> Result<Response<()>> {
    database_request!(
        Field::delete_by_column(
            context.state.connection(),
            "id",
            request.into_inner().id.as_str()
        )
        .await?
    );

    Ok(Response::new(()))
}
