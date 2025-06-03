//SPDX-FileCopyrightText: 2025 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2025 OneLiteFeatherNet

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

use crate::{
    database::schema::{authorization::ResourceAuthorization, user::UserContext},
    prelude::*,
};
use feedback_fusion_common::proto::{
    AuthorizationGrant as ProtoAuthorizationGrant, AuthorizationType as ProtoAuthorizationType,
    DeleteResourceAuthorizationRequest, GetResourceAuthorizationRequest,
    GetResourceAuthorizationsRequest, ResourceAuthorization as ProtoResourceAuthorization,
    ResourceAuthorizationList, ResourceAuthorizationPage, ResourceKind as ProtoResourceKind,
    UpdateResourceAuthorizationRequest,
};

use feedback_fusion_common::proto::CreateResourceAuthorizationRequest;
use v1::FeedbackFusionV1Context;

pub async fn create_resource_authorization(
    context: &FeedbackFusionV1Context,
    request: Request<CreateResourceAuthorizationRequest>,
    _user_context: UserContext,
) -> Result<Response<ResourceAuthorizationList>> {
    let connection = context.connection();
    let data = request.into_inner();
    data.validate()?;

    // a creation request can contain multiple possible values therefore we have to create multiple
    // authorizations
    let authorization_data = data
        .authorization_data
        .ok_or(FeedbackFusionError::BadRequest(
            "missing authorization_data".to_owned(),
        ))?;
    let grants = authorization_data
        .grant
        .iter()
        .map(|repr| ProtoAuthorizationGrant::try_from(*repr).unwrap())
        .collect::<Vec<_>>();
    let authorization_type = ProtoAuthorizationType::try_from(authorization_data.r#type).unwrap();
    let kind = ProtoResourceKind::try_from(data.resource_kind).unwrap();

    let authorizations = authorization_data
        .values
        .into_iter()
        .flat_map(|value| {
            // for each grant
            grants
                .iter()
                .flat_map(|grant| {
                    // for each resource_id
                    data.resource_id
                        .iter()
                        .map(|id| {
                            ResourceAuthorization::builder()
                                .resource_kind(&kind)
                                .resource_id(id.clone())
                                .authorization_grant(grant)
                                .authorization_type(&authorization_type)
                                .authorization_value(value.clone())
                                .build()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // insert the authorizations
    database_request!(
        ResourceAuthorization::insert_batch(
            connection,
            authorizations.as_slice(),
            authorizations.len() as u64,
        )
        .await,
        "Create ResourceAuthorizations"
    )?;

    Ok(Response::new(ResourceAuthorizationList {
        authorizations: authorizations
            .into_iter()
            .map(From::from)
            .collect::<Vec<_>>(),
    }))
}

pub async fn get_resoruce_authorization(
    context: &FeedbackFusionV1Context,
    request: Request<GetResourceAuthorizationRequest>,
    _user_context: UserContext,
) -> Result<Response<ProtoResourceAuthorization>> {
    let connection = context.connection();
    let data = request.into_inner();
    data.validate()?;

    let authorization = database_request!(
        ResourceAuthorization::select_by_id(connection, data.id.as_str()).await,
        "Select authorization by id"
    )?
    .ok_or(FeedbackFusionError::BadRequest(
        "ResourceAuthorization not found".to_owned(),
    ))?;

    Ok(Response::new(authorization.into()))
}

pub async fn get_resource_authorizations(
    context: &FeedbackFusionV1Context,
    request: Request<GetResourceAuthorizationsRequest>,
    _user_context: UserContext,
) -> Result<Response<ResourceAuthorizationPage>> {
    let _connection = context.connection();
    let _data = request.into_inner();

    todo!();
}

pub async fn update_resource_authorization(
    context: &FeedbackFusionV1Context,
    request: Request<UpdateResourceAuthorizationRequest>,
    _user_context: UserContext,
) -> Result<Response<ProtoResourceAuthorization>> {
    let connection = context.connection();
    let data = request.into_inner();
    data.validate()?;

    let mut authorization = database_request!(
        ResourceAuthorization::select_by_id(connection, data.id.as_str()).await,
        "Select ResourceAuthorization by id"
    )?
    .ok_or(FeedbackFusionError::BadRequest(
        "ResourceAuthorization not found".to_owned(),
    ))?;

    authorization.set_resource_id(data.resource_id);
    if let Some(grant) = data.authorization_grant {
        authorization
            .set_authorization_grant((&ProtoAuthorizationGrant::try_from(grant).unwrap()).into());
    }

    if let Some(authorization_type) = data.authorization_type {
        authorization.set_authorization_type(
            (&ProtoAuthorizationType::try_from(authorization_type).unwrap()).into(),
        );
    }

    database_request!(
        ResourceAuthorization::update_by_column(connection, &authorization, "id").await,
        "Update Resourceauthorization"
    )?;

    Ok(Response::new(authorization.into()))
}

pub async fn delete_resoruce_authorization(
    context: &FeedbackFusionV1Context,
    request: Request<DeleteResourceAuthorizationRequest>,
    _user_context: UserContext,
) -> Result<Response<()>> {
    let connection = context.connection();
    let data = request.into_inner();

    database_request!(
        ResourceAuthorization::delete_by_column(connection, "id", data.id.as_str()).await,
        "Delete ResourceAuthorization by id"
    )?;

    Ok(Response::new(()))
}
