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

use crate::prelude::*;
use feedback_fusion_common::proto::{
    feedback_fusion_v1_server::FeedbackFusionV1, CreateFieldRequest, CreatePromptRequest,
    CreateTargetRequest, DeleteFieldRequest, DeletePromptRequest, DeleteTargetRequest,
    Field as ProtoField, FieldPage, GetFieldsRequest, GetPromptsRequest, GetResponsesRequest,
    GetTargetRequest, GetTargetsRequest, Prompt as ProtoPrompt, PromptPage, ResponsePage,
    Target as ProtoTarget, TargetPage, UpdateFieldRequest, UpdatePromptRequest,
    UpdateTargetRequest,
};
use tonic::{Response, Status};

pub mod field;
pub mod prompt;
pub mod response;
pub mod target;

#[derive(Clone, Getters)]
#[get = "pub"]
pub struct FeedbackFusionV1Context {
    pub connection: DatabaseConnection,
}

macro_rules! handler {
    ($handler:path, $self:ident, $request:ident) => {{
        match $handler($self, $request).await {
            Ok(response) => Ok(response),
            Err(error) => Err(error.into()),
        }
    }};
}

// may consider to divide the service into its parts, but as of now this wouldn't be a real
// enhacement
#[async_trait::async_trait]
impl FeedbackFusionV1 for FeedbackFusionV1Context {
    #[instrument(skip_all)]
    async fn create_target(
        &self,
        request: Request<CreateTargetRequest>,
    ) -> std::result::Result<Response<ProtoTarget>, Status> {
        handler!(target::create_target, self, request)
    }

    #[instrument(skip_all)]
    async fn get_target(
        &self,
        request: Request<GetTargetRequest>,
    ) -> std::result::Result<Response<ProtoTarget>, Status> {
        handler!(target::get_target, self, request)
    }

    #[instrument(skip_all)]
    async fn get_targets(
        &self,
        request: Request<GetTargetsRequest>,
    ) -> std::result::Result<Response<TargetPage>, Status> {
        handler!(target::get_targets, self, request)
    }

    #[instrument(skip_all)]
    async fn update_target(
        &self,
        request: Request<UpdateTargetRequest>,
    ) -> std::result::Result<Response<ProtoTarget>, Status> {
        handler!(target::update_target, self, request)
    }

    #[instrument(skip_all)]
    async fn delete_target(
        &self,
        request: Request<DeleteTargetRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(target::delete_target, self, request)
    }

    #[instrument(skip_all)]
    async fn create_prompt(
        &self,
        request: Request<CreatePromptRequest>,
    ) -> std::result::Result<Response<ProtoPrompt>, Status> {
        handler!(prompt::create_prompt, self, request)
    }

    #[instrument(skip_all)]
    async fn get_prompts(
        &self,
        request: Request<GetPromptsRequest>,
    ) -> std::result::Result<Response<PromptPage>, Status> {
        handler!(prompt::get_prompts, self, request)
    }

    #[instrument(skip_all)]
    async fn update_prompt(
        &self,
        request: Request<UpdatePromptRequest>,
    ) -> std::result::Result<Response<ProtoPrompt>, Status> {
        handler!(prompt::update_prompt, self, request)
    }

    #[instrument(skip_all)]
    async fn delete_prompt(
        &self,
        request: Request<DeletePromptRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(prompt::delete_prompt, self, request)
    }

    #[instrument(skip_all)]
    async fn create_field(
        &self,
        request: Request<CreateFieldRequest>,
    ) -> std::result::Result<Response<ProtoField>, Status> {
        handler!(field::create_field, self, request)
    }

    #[instrument(skip_all)]
    async fn get_fields(
        &self,
        request: Request<GetFieldsRequest>,
    ) -> std::result::Result<Response<FieldPage>, Status> {
        handler!(field::get_fields, self, request)
    }

    #[instrument(skip_all)]
    async fn update_field(
        &self,
        request: Request<UpdateFieldRequest>,
    ) -> std::result::Result<Response<ProtoField>, Status> {
        handler!(field::update_field, self, request)
    }

    #[instrument(skip_all)]
    async fn delete_field(
        &self,
        request: Request<DeleteFieldRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(field::delete_field, self, request)
    }

    #[instrument(skip_all)]
    async fn get_responses(
        &self,
        request: Request<GetResponsesRequest>,
    ) -> std::result::Result<Response<ResponsePage>, Status> {
        handler!(response::get_responses, self, request)
    }
}
