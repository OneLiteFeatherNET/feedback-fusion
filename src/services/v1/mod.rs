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

use crate::{database::schema::user::UserContext, prelude::*};
use feedback_fusion_common::proto::{
    AuditVersionPage, CreateFieldRequest, CreatePromptRequest, CreateResourceAuthorizationRequest,
    CreateResponsesRequest, CreateTargetRequest, DataExportRequest, DataExportResponse,
    DeleteFieldRequest, DeletePromptRequest, DeleteResourceAuthorizationRequest,
    DeleteTargetRequest, ExportResourceAuthorizationsRequest, FieldPage, GetAuditVersionsRequest,
    GetFieldsRequest, GetPromptRequest, GetPromptsRequest, GetResourceAuthorizationRequest,
    GetResourceAuthorizationsRequest, GetResponsesRequest, GetTargetRequest, GetTargetsRequest,
    PromptPage, ProtoEvent, ProtoField, ProtoPrompt, ProtoPromptResponse,
    ProtoResourceAuthorization, ProtoTarget, ResourceAuthorizationExportResponse,
    ResourceAuthorizationList, ResourceAuthorizationPage, ResponsePage, RollbackResourceRequest,
    TargetPage, UpdateFieldRequest, UpdatePromptRequest, UpdateResourceAuthorizationRequest,
    UpdateTargetRequest, UserInfoResponse, feedback_fusion_v1_server::FeedbackFusionV1,
    public_feedback_fusion_v1_server::PublicFeedbackFusionV1,
};
use kanal::AsyncSender;
use openidconnect::core::CoreClient;
use std::borrow::Cow;
use tonic::{Response, Status};

pub mod audit;
pub mod authorization;
pub mod export;
pub mod field;
pub mod prompt;
pub mod response;
pub mod target;
pub mod user;

#[derive(Getters)]
#[get = "pub"]
pub struct FeedbackFusionV1Context<'a> {
    pub connection: DatabaseConnection,
    pub client: CoreClient,
    pub permission_matrix: PermissionMatrix<'a>,
    pub broker_event_sender: AsyncSender<ProtoEvent>,
}

#[derive(Getters)]
#[get = "pub"]
pub struct PublicFeedbackFusionV1Context {
    pub connection: DatabaseConnection,
}

// https://github.com/neoeinstein/aliri/blob/main/aliri_tower/examples/.tonic.rs#L35
macro_rules! handler {
    ($handler:path, $self:ident, $request:ident, $endpoint:path, $permission:path) => {
        handler!($handler, $self, $request, $endpoint { EndpointScopeSelector::All }, $permission)
    };
    ($handler:path, $self:ident, $request:ident, off) => {{
        match UserContext::get_otherwise_fetch(&$request, &$self.client, &$self.connection, &$self.permission_matrix).await {
            Ok(context) => {
                handler!($handler, $self, $request, context)
            }
            Err(error) => Err(error.into()),
        }
    }};
    ($handler:path, $self:ident, $request:ident, $endpoint:path $inner:block, $permission:path) => {{
        match UserContext::get_otherwise_fetch(&$request, &$self.client, &$self.connection, &$self.permission_matrix).await {
            Ok(context) => {
                if let Err(error) = context
                    .authorize(&$self.connection, &$endpoint(async $inner.await), &$permission)
                    .await
                {
                    Err(error.into())
                } else {
                    handler!($handler, $self, $request, context)
                }
            }
            Err(error) => Err(error.into()),
        }
    }};
    ($handler:path, $self:ident, $request:ident, $context:ident) => {{
        match $handler($self, $request, $context).await {
            Ok(response) => Ok(response),
            Err(error) => Err(error.into()),
        }
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
impl FeedbackFusionV1 for FeedbackFusionV1Context<'static> {
    #[instrument(skip_all)]
    async fn create_target(
        &self,
        request: Request<CreateTargetRequest>,
    ) -> std::result::Result<Response<ProtoTarget>, Status> {
        handler!(
            target::create_target,
            self,
            request,
            Endpoint::Target,
            Permission::Write
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
            Endpoint::Target { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().id.as_str()) )},
            Permission::Read
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
            Endpoint::Target,
            Permission::List
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
            Endpoint::Target { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().id.as_str())) },
            Permission::Write
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
            Endpoint::Target { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().id.as_str())) },
            Permission::Write
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
            Endpoint::Target { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().target.as_str())) },
            Permission::Write
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
            Endpoint::Prompt,
            Permission::List
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
            Endpoint::Prompt { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().id.as_str())) },
            Permission::Write
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
            Endpoint::Prompt { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().id.as_str())) },
            Permission::Write
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
            Endpoint::Prompt { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().prompt.as_str())) },
            Permission::Write
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
            Endpoint::Field,
            Permission::List
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
            Endpoint::Field { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().id.as_str())) },
            Permission::Write
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
            Endpoint::Field { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().id.as_str())) },
            Permission::Write
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
            Endpoint::Response { EndpointScopeSelector::Specific(Cow::Borrowed(request.get_ref().prompt.as_str())) },
            Permission::List
        )
    }

    #[instrument(skip_all)]
    async fn get_user_info(
        &self,
        request: Request<()>,
    ) -> std::result::Result<Response<UserInfoResponse>, Status> {
        match UserContext::get_otherwise_fetch(
            &request,
            &self.client,
            &self.connection,
            self.permission_matrix(),
        )
        .await
        {
            Ok(context) => match user::get_user_info(self, request, context).await {
                Ok(response) => Ok(response),
                Err(error) => Err(error.into()),
            },
            Err(error) => Err(error.into()),
        }
    }

    #[instrument(skip_all)]
    async fn export_data(
        &self,
        request: Request<DataExportRequest>,
    ) -> std::result::Result<Response<DataExportResponse>, Status> {
        handler!(
            export::export_data,
            self,
            request,
            Endpoint::Export { EndpointScopeSelector::Multiple(request.get_ref().targets.iter().map(|target| Cow::Borrowed(target.as_str())).collect()) },
            Permission::Read
        )
    }

    #[instrument(skip_all)]
    async fn create_resource_authorization(
        &self,
        request: Request<CreateResourceAuthorizationRequest>,
    ) -> std::result::Result<Response<ResourceAuthorizationList>, Status> {
        handler!(
            authorization::create_resource_authorization,
            self,
            request,
            Endpoint::Authorize { None },
            Permission::Write
        )
    }

    #[instrument(skip_all)]
    async fn get_resource_authorization(
        &self,
        request: Request<GetResourceAuthorizationRequest>,
    ) -> std::result::Result<Response<ProtoResourceAuthorization>, Status> {
        handler!(
            authorization::get_resource_authorization,
            self,
            request,
            Endpoint::Authorize { None },
            Permission::Read
        )
    }

    #[instrument(skip_all)]
    async fn get_resource_authorizations(
        &self,
        request: Request<GetResourceAuthorizationsRequest>,
    ) -> std::result::Result<Response<ResourceAuthorizationPage>, Status> {
        handler!(
            authorization::get_resource_authorizations,
            self,
            request,
            Endpoint::Authorize { None },
            Permission::List
        )
    }

    #[instrument(skip_all)]
    async fn update_resource_authorization(
        &self,
        request: Request<UpdateResourceAuthorizationRequest>,
    ) -> std::result::Result<Response<ProtoResourceAuthorization>, Status> {
        handler!(
            authorization::update_resource_authorization,
            self,
            request,
            Endpoint::Authorize { None },
            Permission::Write
        )
    }

    #[instrument(skip_all)]
    async fn delete_resource_authorization(
        &self,
        request: Request<DeleteResourceAuthorizationRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(
            authorization::delete_resource_authorization,
            self,
            request,
            Endpoint::Authorize { None },
            Permission::Write
        )
    }

    #[instrument(skip_all)]
    async fn export_resource_authorizations(
        &self,
        request: Request<ExportResourceAuthorizationsRequest>,
    ) -> std::result::Result<Response<ResourceAuthorizationExportResponse>, Status> {
        handler!(
            authorization::export_resource_authorizations,
            self,
            request,
            Endpoint::Authorize { None },
            Permission::List
        )
    }

    #[instrument(skip_all)]
    async fn get_audit_versions(
        &self,
        request: Request<GetAuditVersionsRequest>,
    ) -> std::result::Result<Response<AuditVersionPage>, Status> {
        handler!(audit::get_audit_versions, self, request, off)
    }

    #[instrument(skip_all)]
    async fn rollback_resource(
        &self,
        request: Request<RollbackResourceRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        handler!(audit::rollback_resource, self, request, off)
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
    ) -> std::result::Result<Response<ProtoPromptResponse>, Status> {
        handler!(response::create_responses, self, request)
    }
}
