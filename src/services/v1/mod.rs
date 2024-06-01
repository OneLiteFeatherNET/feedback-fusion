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

use crate::{database::schema::feedback::Prompt, prelude::*};
use aliri_oauth2::{policy, scope, HasScope};
use aliri_traits::Policy;
use feedback_fusion_common::proto::{
    feedback_fusion_v1_server::FeedbackFusionV1,
    public_feedback_fusion_v1_server::PublicFeedbackFusionV1, CreateFieldRequest,
    CreatePromptRequest, CreateResponsesRequest, CreateTargetRequest, DeleteFieldRequest,
    DeletePromptRequest, DeleteTargetRequest, Field as ProtoField, FieldPage, GetFieldsRequest,
    GetPromptRequest, GetPromptsRequest, GetResponsesRequest, GetTargetRequest, GetTargetsRequest,
    Prompt as ProtoPrompt, PromptPage, PromptResponse, ResponsePage, Target as ProtoTarget,
    TargetPage, UpdateFieldRequest, UpdatePromptRequest, UpdateTargetRequest,
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

#[derive(Clone, Getters)]
#[get = "pub"]
pub struct PublicFeedbackFusionV1Context {
    pub connection: DatabaseConnection,
}

// https://github.com/neoeinstein/aliri/blob/main/aliri_tower/examples/.tonic.rs#L35
macro_rules! handler {
    ($handler:path, $self:ident, $request:ident, $policy:ident, $($scope:literal $(,)?)*) => {{
         $policy
            .evaluate(
                $request
                    .extensions()
                    .get::<OIDCClaims>()
                    .ok_or_else(|| Status::permission_denied("missing claims"))?
                    .scope(),
            )
            .map_err(|_| {
                let message = format!(
                    "insufficient scopes, requires one of: [\"{}\"]",
                    (&$policy)
                        .into_iter()
                        .map(|s| s.iter().map(|t| t.as_str()).collect::<Vec<_>>().join(" "))
                        .collect::<Vec<_>>()
                        .join("\" or \"")
                );
                Status::permission_denied(message)
            })?;

        handler!($handler, $self, $request)
    }};
    ($handler:path, $self:ident, $request:ident, $($scope:literal $(,)?)*, $target:block) => {{
        paste! {
            match async $target.await {
                Ok(target) => {
                    if let Some(target) = target {
                        let policy = policy![
                            scope!["api:feedback-fusion"]
                            $(,
                                scope![$scope],
                                aliri_oauth2::Scope::empty().and(aliri_oauth2::scope::ScopeToken::from_string(format!("{}:target:{}", $scope, target)).unwrap())
                            )*
                        ];

                        handler!($handler, $self, $request, policy, $($scope,)*)
                    } else {
                        Err(Status::invalid_argument("Resource target could not be found"))
                    }
                }
                Err(error) =>  {
                    error!("Error occurred during target fetch: {}", error);
                    Err(error.into())
                }
            }
        }
    }};
    ($handler:path, $self:ident, $request:ident, $($scope:literal $(,)?)*) => {{
        let policy = policy![
            scope!["api:feedback-fusion"]
            $(,
                scope![$scope]
            )*
        ];

        handler!($handler, $self, $request, policy, $($scope,)*)
    }};
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
        handler!(
            target::create_target,
            self,
            request,
            "feedback-fusion:write"
        )
    }

    #[instrument(skip_all)]
    async fn get_target(
        &self,
        request: Request<GetTargetRequest>,
    ) -> std::result::Result<Response<ProtoTarget>, Status> {
        handler!(
            target::get_target,
            self,
            request,
            "feedback-fusion:read",
            "feedback-fusion:getTarget",
            { Ok::<_, FeedbackFusionError>(Some(request.get_ref().id.clone())) }
        )
    }

    #[instrument(skip_all)]
    async fn get_targets(
        &self,
        request: Request<GetTargetsRequest>,
    ) -> std::result::Result<Response<TargetPage>, Status> {
        handler!(
            target::get_targets,
            self,
            request,
            "feedback-fusion:read",
            "feedback-fusion:listTargets"
        )
    }

    #[instrument(skip_all)]
    async fn update_target(
        &self,
        request: Request<UpdateTargetRequest>,
    ) -> std::result::Result<Response<ProtoTarget>, Status> {
        handler!(
            target::update_target,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:putTarget",
            { Ok::<_, FeedbackFusionError>(Some(request.get_ref().id.clone())) }
        )
    }

    #[instrument(skip_all)]
    async fn delete_target(
        &self,
        request: Request<DeleteTargetRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(
            target::delete_target,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:deleteTarget",
            { Ok::<_, FeedbackFusionError>(Some(request.get_ref().id.clone())) }
        )
    }

    #[instrument(skip_all)]
    async fn create_prompt(
        &self,
        request: Request<CreatePromptRequest>,
    ) -> std::result::Result<Response<ProtoPrompt>, Status> {
        handler!(
            prompt::create_prompt,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:writePrompt",
            { Ok::<_, FeedbackFusionError>(Some(request.get_ref().target.clone())) }
        )
    }

    #[instrument(skip_all)]
    async fn get_prompts(
        &self,
        request: Request<GetPromptsRequest>,
    ) -> std::result::Result<Response<PromptPage>, Status> {
        handler!(
            prompt::get_prompts,
            self,
            request,
            "feedback-fusion:read",
            "feedback-fusion:listPrompts",
            { Ok::<_, FeedbackFusionError>(Some(request.get_ref().target.clone())) }
        )
    }

    #[instrument(skip_all)]
    async fn update_prompt(
        &self,
        request: Request<UpdatePromptRequest>,
    ) -> std::result::Result<Response<ProtoPrompt>, Status> {
        handler!(
            prompt::update_prompt,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:putPrompt",
            {
                Ok::<_, FeedbackFusionError>(database_request!(
                    Prompt::select_by_id(self.connection(), request.get_ref().id.as_str())
                        .await?
                        .and_then(|prompt| Some(prompt.target().clone())),
                    "Authorization"
                ))
            }
        )
    }

    #[instrument(skip_all)]
    async fn delete_prompt(
        &self,
        request: Request<DeletePromptRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(
            prompt::delete_prompt,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:deleteTarget",
            {
                Ok::<_, FeedbackFusionError>(database_request!(
                    Prompt::select_by_id(self.connection(), request.get_ref().id.as_str())
                        .await?
                        .and_then(|prompt| Some(prompt.target().clone())),
                    "Authorization"
                ))
            }
        )
    }

    #[instrument(skip_all)]
    async fn create_field(
        &self,
        request: Request<CreateFieldRequest>,
    ) -> std::result::Result<Response<ProtoField>, Status> {
        handler!(
            field::create_field,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:writeField",
            {
                Ok::<_, FeedbackFusionError>(database_request!(
                    Prompt::select_by_id(self.connection(), request.get_ref().prompt.as_str())
                        .await?
                        .and_then(|prompt| Some(prompt.target().clone())),
                    "Authorization"
                ))
            }
        )
    }

    #[instrument(skip_all)]
    async fn get_fields(
        &self,
        request: Request<GetFieldsRequest>,
    ) -> std::result::Result<Response<FieldPage>, Status> {
        handler!(
            field::get_fields,
            self,
            request,
            "feedback-fusion:read",
            "feedback-fusion:listFields",
            {
                Ok::<_, FeedbackFusionError>(database_request!(
                    Prompt::select_by_id(self.connection(), request.get_ref().prompt.as_str())
                        .await?
                        .and_then(|prompt| Some(prompt.target().clone())),
                    "Authorization"
                ))
            }
        )
    }

    #[instrument(skip_all)]
    async fn update_field(
        &self,
        request: Request<UpdateFieldRequest>,
    ) -> std::result::Result<Response<ProtoField>, Status> {
        handler!(
            field::update_field,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:putField",
            {
                let prompt: Option<Prompt> = database_request!(
                    self.connection()
                        .query_decode(
                            "SELECT prompt.* FROM prompt INNER JOIN field ON field.prompt = prompt.id WHERE field.id = ?",
                            vec![rbs::to_value!(request.get_ref().id.as_str())]
                        )
                        .await?,
                    "Authorization"
                );
                Ok::<_, FeedbackFusionError>(
                    prompt.and_then(|prompt| Some(prompt.target().clone())),
                )
            }
        )
    }

    #[instrument(skip_all)]
    async fn delete_field(
        &self,
        request: Request<DeleteFieldRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(
            field::delete_field,
            self,
            request,
            "feedback-fusion:write",
            "feedback-fusion:deleteField",
            {
                let prompt: Option<Prompt> = database_request!(
                    self.connection()
                        .query_decode(
                            "SELECT prompt.* FROM prompt INNER JOIN field ON field.prompt = prompt.id WHERE field.id = ?",
                            vec![rbs::to_value!(request.get_ref().id.as_str())]
                        )
                        .await?,
                    "Authorization"
                );
                Ok::<_, FeedbackFusionError>(
                    prompt.and_then(|prompt| Some(prompt.target().clone())),
                )
            }
        )
    }

    #[instrument(skip_all)]
    async fn get_responses(
        &self,
        request: Request<GetResponsesRequest>,
    ) -> std::result::Result<Response<ResponsePage>, Status> {
        handler!(
            response::get_responses,
            self,
            request,
            "feedback-fusion:read",
            "feedback-fusion:listResponses",
            {
                Ok::<_, FeedbackFusionError>(database_request!(
                    Prompt::select_by_id(self.connection(), request.get_ref().prompt.as_str())
                        .await?
                        .and_then(|prompt| Some(prompt.target().clone())),
                    "Authorization"
                ))
            }
        )
    }
}

#[async_trait::async_trait]
impl PublicFeedbackFusionV1 for PublicFeedbackFusionV1Context {
    #[instrument(skip_all)]
    async fn get_active_fields(
        &self,
        request: Request<GetFieldsRequest>,
    ) -> std::result::Result<Response<FieldPage>, Status> {
        handler!(field::get_active_fields, self, request)
    }

    #[instrument(skip_all)]
    async fn get_prompt(
        &self,
        request: Request<GetPromptRequest>,
    ) -> std::result::Result<Response<ProtoPrompt>, Status> {
        handler!(prompt::get_prompt, self, request)
    }

    #[instrument(skip_all)]
    async fn create_responses(
        &self,
        request: Request<CreateResponsesRequest>,
    ) -> std::result::Result<Response<PromptResponse>, Status> {
        handler!(response::create_responses, self, request)
    }
}
