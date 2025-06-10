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
    database::schema::{
        authorization::{
            ResourceAuthorization, ResourceAuthorizationGrant, ResourceAuthorizationType,
            ResourceKind,
        },
        user::UserContext,
    },
    prelude::*,
};
use dashmap::{DashMap, DashSet};
use feedback_fusion_common::proto::{
    AuthorizationGrant as ProtoAuthorizationGrant, AuthorizationType as ProtoAuthorizationType,
    DeleteResourceAuthorizationRequest, ExportResourceAuthorizationsRequest,
    GetResourceAuthorizationRequest, GetResourceAuthorizationsRequest,
    ResourceAuthorization as ProtoResourceAuthorization, ResourceAuthorizationExportResponse,
    ResourceAuthorizationList, ResourceAuthorizationPage, ResourceKind as ProtoResourceKind,
    UpdateResourceAuthorizationRequest,
};

use feedback_fusion_common::proto::CreateResourceAuthorizationRequest;
use v1::FeedbackFusionV1Context;
use wildcard::Wildcard;

pub async fn create_resource_authorization(
    context: &FeedbackFusionV1Context<'_>,
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

pub async fn get_resource_authorization(
    context: &FeedbackFusionV1Context<'_>,
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
    context: &FeedbackFusionV1Context<'_>,
    request: Request<GetResourceAuthorizationsRequest>,
    _user_context: UserContext,
) -> Result<Response<ResourceAuthorizationPage>> {
    let connection = context.connection();
    let data = request.into_inner();
    let page_request = data.page_request();

    let authorizations = database_request!(
        ResourceAuthorization::select_page_wrapper(connection, &page_request).await,
        "Select authorizations page"
    )?;

    Ok(Response::new(ResourceAuthorizationPage {
        page_token: page_request.page_no().try_into()?,
        next_page_token: TryInto::<i32>::try_into(page_request.page_no())? + 1i32,
        page_size: page_request.page_size().try_into()?,
        total: authorizations.total.try_into()?,
        authorizations: authorizations.records.into_iter().map(Into::into).collect(),
    }))
}

pub async fn update_resource_authorization(
    context: &FeedbackFusionV1Context<'_>,
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

pub async fn delete_resource_authorization(
    context: &FeedbackFusionV1Context<'_>,
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

#[instrument]
fn try_minify_grants<'a>(
    x: &'a DashSet<&'a ResourceAuthorizationGrant>,
    y: &DashSet<&ResourceAuthorizationGrant>,
) -> Result<DashSet<&'a ResourceAuthorizationGrant>> {
    // try to match all grants
    if x.iter().all(|x| {
        y.iter()
            .any(|y| x.key().eq(y.key()) || y.key().eq(&&ResourceAuthorizationGrant::All))
    }) && y.iter().all(|y| {
        x.iter()
            .any(|x| y.key().eq(x.key()) || x.key().eq(&&ResourceAuthorizationGrant::All))
    }) {
        // now we have a confirmed match and can minify the grants / at least try to do it
        // If x already contains `All` we delete everything besides `All`, otherwise we check
        // wether we have all 3 `ResourceAuthorizationGrant` and convert them to all. Otherwise we
        // do nothing
        if x.contains(&ResourceAuthorizationGrant::All) || x.len() == PERMISSIONS.len() - 1 {
            let result = DashSet::new();
            result.insert(&ResourceAuthorizationGrant::All);

            Ok(result)
        } else {
            // TODO: get rid of the clone
            Ok(x.clone())
        }
    } else {
        Err(FeedbackFusionError::Unauthorized)
    }
}

#[instrument(skip_all)]
fn authorizations_to_endpoints<'a>(
    authorizations: &[ResourceAuthorization],
) -> (Vec<AuthorizationMapping<'a>>, Vec<AuthorizationMapping<'a>>) {
    // We do not want to just output garbage little endpoints we can easily generate from the input
    // slice instead we want to minify and therefore optimize the output combinations as much as we
    // can.
    //
    // In order to do that we have to "sort" our input in different recursive categories (here
    // maps):
    // - Get map which matches the combination of the input authentication credentials
    // - Get the submap which does match the referenced `ResourceKind`
    // - Get the set which contains all grants for the given `resource_id`

    let map: DashMap<
        (&ResourceAuthorizationType, &str),
        DashMap<&ResourceKind, DashMap<Option<Vec<&String>>, DashSet<&ResourceAuthorizationGrant>>>,
    > = DashMap::new();

    authorizations.into_par_iter().for_each(|authorization| {
        map.entry((
            authorization.authorization_type(),
            authorization.authorization_value().as_str(),
        ))
        .or_default()
        .entry(authorization.resource_kind())
        .or_default()
        .entry(
            authorization
                .resource_id()
                .as_ref()
                .and_then(|id| Some(vec![id])),
        )
        .or_default()
        .insert(authorization.authorization_grant());
    });

    // Now we've done the first step of the optimization and can now continue with the minification
    // of the output structure.
    //
    // Therefore we can now eliminate all `resource_id` if we can construct them by another one
    // (via Wildcard) and the permissions match. In this case we cant just compare the grants
    // directly as we have to take care of `All` as an grant value.
    //
    // After we've done this we can eliminate

    map.par_iter().for_each(|authorization_entry| {
        let resource_kind_map = authorization_entry.value();

        resource_kind_map
            .par_iter()
            .for_each(|resource_kind_entry| {
                let resource_id_map = resource_kind_entry.value();

                resource_id_map.par_iter().for_each(|resource_id_entry| {
                    // check wether this entry can be constructed by one of the other keys
                    let can_be_constructed = !resource_id_entry.key().is_none()
                        && resource_id_map.par_iter_mut().any(|mut entry| {
                            let key = entry.key();

                            if key.eq(resource_id_entry.key())
                                || key
                                    .as_ref()
                                    .is_some_and(|inner| inner.iter().any(|key| key.contains("*")))
                            {
                                false
                            } else {
                                key.is_none()
                                    || key.as_ref().is_some_and(|key| {
                                        key.iter().any(|key| {
                                            let could_be_match = {
                                                // TODO: we could theoretically also calculate overlapping
                                                // wildcards which can not be constructed out of each other
                                                let wildcard =
                                                    Wildcard::new(key.as_bytes()).unwrap();

                                                resource_id_entry
                                                    .key()
                                                    .as_ref()
                                                    .unwrap()
                                                    .iter()
                                                    .any(|k| wildcard.is_match(k.as_bytes()))
                                            };

                                            // now as this is a pattern match we have to verify that the
                                            // permissions also do match
                                            //
                                            // This does automatically minify the entry permissions
                                            if could_be_match {
                                                if let Ok(result) = try_minify_grants(
                                                    entry.value(),
                                                    resource_id_entry.value(),
                                                ) {
                                                    let mut grants = entry.value_mut();
                                                    *grants = result;
                                                    true
                                                } else {
                                                    false
                                                }
                                            } else {
                                                false
                                            }
                                        })
                                    })
                            }
                        });

                    if can_be_constructed {
                        // as this can be constructed we now can safely remove it
                        resource_id_map.remove(resource_id_entry.key());
                    }
                });
            });
    });

    let mut scopes = Vec::new();
    let mut groups = Vec::new();

    // we can now iterate another time after the changes were done to get the final output
    map.into_par_iter().for_each(|authorization_entry| {
        let authorization = authorization_entry.0;
        let resource_kind_map = authorization_entry.1;

        let mut authorization_mapping = AuthorizationMapping {
            name: authorization.1.to_string(),
            grants: Vec::new(),
        };

        resource_kind_map
            .into_par_iter()
            .for_each(|resource_kind_entry| {
                let resource_id_map = resource_kind_entry.1;

                resource_id_map
                    .into_par_iter()
                    .for_each(|resource_id_entry| {
                        // authorization_mapping.grants.push(AuthorizationGrants {
                        //     endpoint:
                        // });
                    });
            });
    });

    (scopes, groups)
}

pub async fn export_resource_authorizations(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<ExportResourceAuthorizationsRequest>,
    _user_context: UserContext,
) -> Result<Response<ResourceAuthorizationExportResponse>> {
    let connection = context.connection();
    let data = request.into_inner();

    let authorizations = database_request!(
        ResourceAuthorization::select_by_ids(connection, data.ids.as_slice()).await,
        "Select authorizations by ids"
    )?;

    // convert the authorizations to endpoints with permissions
    let converted = authorizations_to_endpoints(authorizations.as_slice());

    todo!()
}
