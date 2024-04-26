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
use crate::{database::schema::feedback::Prompt, prelude::*};
use feedback_fusion_common::proto::{
    CreatePromptRequest, DeletePromptRequest, GetPromptRequest, GetPromptsRequest,
    Prompt as ProtoPrompt, PromptPage, UpdatePromptRequest,
};
use validator::Validate;

pub async fn create_prompt(
    context: &FeedbackFusionV1Context,
    request: Request<CreatePromptRequest>,
) -> Result<Response<ProtoPrompt>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.connection();

    // build the prompt
    let prompt = Prompt::builder()
        .title(data.title)
        .description(data.description)
        .active(data.active)
        .target(data.target)
        .build();
    database_request!(Prompt::insert(connection, &prompt).await?);

    Ok(Response::new(prompt.into()))
}

pub async fn get_prompt(
    context: &PublicFeedbackFusionV1Context,
    request: Request<GetPromptRequest>,
) -> Result<Response<ProtoPrompt>> {
    let data = request.into_inner();
    let connection = context.connection();

    let prompt: Option<Prompt> =
        database_request!(Prompt::select_by_id(connection, data.id.as_str()).await?);

    match prompt {
        Some(prompt) => Ok(Response::new(prompt.into())),
        None => Err(FeedbackFusionError::BadRequest(
            "invalid prompt".to_string(),
        )),
    }
}

pub async fn get_prompts(
    context: &FeedbackFusionV1Context,
    request: Request<GetPromptsRequest>,
) -> Result<Response<PromptPage>> {
    let data = request.into_inner();
    let page_request = data.into_page_request();
    let connection = context.connection();

    let prompts = database_request!(
        Prompt::select_page_by_target_wrapper(connection, &page_request, data.target.as_str(),)
            .await?
    );

    Ok(Response::new(PromptPage {
        page_token: page_request.page_no().try_into()?,
        next_page_token: TryInto::<i32>::try_into(page_request.page_no())? + 1i32,
        page_size: page_request.page_size().try_into()?,
        prompts: prompts
            .records
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ProtoPrompt>>(),
    }))
}

pub async fn update_prompt(
    context: &FeedbackFusionV1Context,
    request: Request<UpdatePromptRequest>,
) -> Result<Response<ProtoPrompt>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.connection();

    let mut prompt =
        database_request!(Prompt::select_by_id(connection, data.id.as_str())
            .await?
            .ok_or(FeedbackFusionError::BadRequest("not found".to_owned()))?);

    prompt.set_title(data.title.unwrap_or(prompt.title().clone()));
    prompt.set_description(data.description.unwrap_or(prompt.description().clone()));
    prompt.set_active(data.active.unwrap_or(*prompt.active()));

    database_request!(Prompt::update_by_column(connection, &prompt, "id").await?);
    Ok(Response::new(prompt.into()))
}

pub async fn delete_prompt(
    context: &FeedbackFusionV1Context,
    request: Request<DeletePromptRequest>,
) -> Result<Response<()>> {
    database_request!(
        Prompt::delete_by_column(context.connection(), "id", request.into_inner().id.as_str())
            .await?
    );

    Ok(Response::new(()))
}
