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

use crate::{database::schema::feedback::Target, prelude::*};
use feedback_fusion_common::proto::{
    feedback_fusion_v1_server::FeedbackFusionV1, CreateFieldRequest, CreatePromptRequest,
    CreateTargetRequest, DeleteFieldRequest, DeletePromptRequest, DeleteTargetRequest,
    Field as ProtoField, FieldPage, FieldResponsePage, GetFieldsRequest, GetPromptsRequest,
    GetResponsesRequest, GetTargetRequest, GetTargetsRequest, Prompt as ProtoPrompt, PromptPage,
    Target as ProtoTarget, TargetPage, UpdateFieldRequest, UpdatePromptRequest,
    UpdateTargetRequest,
};
use tonic::Response;
use validator::Validate;

pub mod field;
pub mod prompt;
pub mod response;
pub mod target;

pub struct FeedbackFusionV1Context {
    state: FeedbackFusionState,
}

macro_rules! handler {
    ($handler:path, $self:ident, $request:ident) => {
        let response = $handler($self, $request).await?;

        Ok(response)
    };
}

// may consider to divide the service into its parts, but as of now this wouldn't be a real
// enhacement
#[async_trait::async_trait]
impl FeedbackFusionV1 for FeedbackFusionV1Context {
    #[instrument(skip_all)]
    async fn create_target(
        &self,
        request: Request<CreateTargetRequest>,
    ) -> Result<Response<ProtoTarget>> {
        handler!(target::create_target, self, request)
    }

    #[instrument(skip_all)]
    async fn get_target(
        &self,
        request: Request<GetTargetRequest>,
    ) -> Result<Response<ProtoTarget>> {
        handler!(target::get_target, self, request)
    }

    #[instrument(skip_all)]
    async fn get_targets(
        &self,
        request: Request<GetTargetsRequest>,
    ) -> Result<Response<TargetPage>> {
        handler!(target::get_targets, self, request)
    }

    #[instrument(skip_all)]
    async fn update_target(
        &self,
        request: Request<UpdateTargetRequest>,
    ) -> Result<Response<Target>> {
        handler!(target::update_target, self, request)
    }

    #[instrument(skip_all)]
    async fn delete_target(&self, request: Request<DeleteTargetRequest>) -> Result<Response<()>> {
        handler!(target::delete_target, self, request)
    }

    #[instrument(skip_all)]
    async fn create_prompt(
        &self,
        request: Request<CreatePromptRequest>,
    ) -> Result<Response<ProtoPrompt>> {
        handler!(prompt::create_prompt, self, request)
    }

    #[instrument(skip_all)]
    async fn get_prompts(
        &self,
        request: Request<GetPromptsRequest>,
    ) -> Result<Response<PromptPage>> {
        handler!(prompt::get_prompts, self, request)
    }

    #[instrument(skip_all)]
    async fn update_prompt(&self, request: Request<UpdatePromptRequest>) -> Result<ProtoPrompt> {
        handler!(prompt::update_prompt, self, request)
    }

    #[instrument(skip_all)]
    async fn delete_prompt(&self, request: Request<DeletePromptRequest>) -> Result<Response<()>> {
        handler!(prompt::delete_prompt, self, request)
    }

    #[instrument(skip_all)]
    async fn create_field(
        &self,
        request: Request<CreateFieldRequest>,
    ) -> Result<Response<ProtoField>> {
        handler!(field::create_field, self, request)
    }

    #[instrument(skip_all)]
    async fn get_fields(&self, request: Request<GetFieldsRequest>) -> Result<Response<FieldPage>> {
        handler!(field::get_fields, self, request)
    }

    #[instrument(skip_all)]
    async fn update_field(
        &self,
        request: Request<UpdateFieldRequest>,
    ) -> Result<Response<ProtoField>> {
        handler!(field::update_field, self, request)
    }

    #[instrument(skip_all)]
    async fn delete_field(&self, request: Request<DeleteFieldRequest>) -> Result<Response<()>> {
        handler!(field::delete_field, self, request)
    }

    #[instrument(skip_all)]
    async fn get_responses(
        &self,
        request: Request<GetResponsesRequest>,
    ) -> Result<Response<FieldResponsePage>> {
        handler!(responses::get_responses, self, request)
    }
}
