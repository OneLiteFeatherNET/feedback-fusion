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

use std::borrow::Cow;

use crate::{
    database::schema::{
        authorization::{ResourceAuthorization, ResourceAuthorizationType, ResourceKind},
        user::UserContext,
    },
    prelude::*,
};
use feedback_fusion_common::proto::{
    AuthorizationGrant as ProtoAuthorizationGrant, AuthorizationType as ProtoAuthorizationType,
    DeleteResourceAuthorizationRequest, ExportResourceAuthorizationsRequest,
    GetResourceAuthorizationRequest, GetResourceAuthorizationsRequest,
    ResourceAuthorization as ProtoResourceAuthorization, ResourceAuthorizationExportResponse,
    ResourceAuthorizationList, ResourceAuthorizationPage, ResourceKind as ProtoResourceKind,
    UpdateResourceAuthorizationRequest,
};
use gxhash::{HashMap as GxHashMap, HashSet as GxHashSet};

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
        ResourceAuthorization::select_page_wrapper(&DATABASE_CONFIG, connection, &page_request).await,
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

pub fn authorizations_to_endpoints(
    resource_authorizations: &[ResourceAuthorization],
) -> Export<'_> {
    // Step 1: Group all authorizations by (AuthorizationType, AuthorizationValue).
    // This enables us to later create one mapping per scope/group/subject.
    let mut authorizations_grouped_by_type_and_value: GxHashMap<
        (&ResourceAuthorizationType, &str),
        Vec<&ResourceAuthorization>,
    > = GxHashMap::default();

    for resource_authorization in resource_authorizations {
        // Currently, Subject authorization type is not supported.
        if let ResourceAuthorizationType::Subject = resource_authorization.authorization_type() {
            unimplemented!("Not supported yet");
        }
        let group_key = (
            resource_authorization.authorization_type(),
            resource_authorization.authorization_value().as_str(),
        );
        authorizations_grouped_by_type_and_value
            .entry(group_key)
            .or_default()
            .push(resource_authorization);
    }

    // Step 2: For each group (scope/group), process in parallel for performance.
    let (scope_mappings, group_mappings): (Vec<_>, Vec<_>) =
        authorizations_grouped_by_type_and_value
            .into_par_iter()
            .map(
                |((authorization_type, mapping_name), resource_authorizations_in_group)| {
                    // For each ResourceKind in this group, collect all (Permission, resource_id) pairs.
                    // This allows us to later generate endpoints with the correct permissions and selectors.
                    let mut kind_to_permission_and_id_patterns: GxHashMap<
                        &ResourceKind,
                        Vec<(&Permission, Option<&str>)>,
                    > = GxHashMap::default();

                    for resource_authorization in &resource_authorizations_in_group {
                        kind_to_permission_and_id_patterns
                            .entry(resource_authorization.resource_kind())
                            .or_default()
                            .push((
                                resource_authorization.authorization_grant(),
                                resource_authorization.resource_id().as_deref(),
                            ));
                    }

                    // For each ResourceKind, we will generate endpoints and collect the permissions for each.
                    let mut endpoint_to_permissions: GxHashMap<
                        Endpoint<'_>,
                        GxHashSet<Permission>,
                    > = GxHashMap::default();

                    for (resource_kind, permissions_and_id_patterns) in
                        kind_to_permission_and_id_patterns
                    {
                        // For each permission, collect:
                        // - specific IDs (exact matches)
                        // - wildcard patterns (e.g., glob patterns)
                        // - a flag indicating if the permission applies to all resources (None)
                        let mut permission_to_specific_and_wildcard_ids: GxHashMap<
                            Permission,
                            (GxHashSet<&str>, Vec<&str>, bool),
                        > = GxHashMap::default();

                        for (permission, resource_id_option) in permissions_and_id_patterns {
                            let entry = permission_to_specific_and_wildcard_ids
                                .entry(*permission)
                                .or_insert_with(|| (GxHashSet::default(), Vec::new(), false));
                            match resource_id_option {
                                None => entry.2 = true, // This permission applies to all resources of this kind
                                Some(resource_id)
                                    if resource_id.contains('*') || resource_id.contains('?') =>
                                {
                                    entry.1.push(resource_id); // This is a wildcard pattern
                                }
                                Some(resource_id) => {
                                    entry.0.insert(resource_id); // This is a specific resource ID
                                }
                            }
                        }

                        for (permission, (mut specific_ids, wildcard_patterns, is_all)) in
                            permission_to_specific_and_wildcard_ids
                        {
                            // At this point, we have all the information for a (ResourceKind, Permission) pair.
                            // Now we try to minimize the grants:
                            // If both wildcards and specific IDs are present, remove all specific IDs that are covered by any wildcard.
                            if !wildcard_patterns.is_empty() && !specific_ids.is_empty() {
                                let mut specific_ids_covered_by_wildcard = GxHashSet::default();
                                for wildcard_pattern in &wildcard_patterns {
                                    let wildcard_matcher =
                                        Wildcard::new(wildcard_pattern.as_bytes()).unwrap();
                                    for &specific_id in &specific_ids {
                                        if wildcard_matcher.is_match(specific_id.as_bytes()) {
                                            specific_ids_covered_by_wildcard.insert(specific_id);
                                        }
                                    }
                                }
                                // Remove all specific IDs that are covered by a wildcard pattern
                                specific_ids.retain(|specific_id| {
                                    !specific_ids_covered_by_wildcard.contains(specific_id)
                                });
                            }

                            // If the 'all' flag is set, we only need the All selector, and can ignore specifics and wildcards.
                            let endpoint_scope_selector = if is_all {
                                EndpointScopeSelector::All
                            } else if !wildcard_patterns.is_empty() || !specific_ids.is_empty() {
                                // If we have only one ID or pattern, use the Specific selector.
                                // Otherwise, use Multiple to group all IDs and patterns together.
                                let mut all_ids: Vec<Cow<'_, str>> = Vec::new();
                                for wildcard_pattern in &wildcard_patterns {
                                    all_ids.push(Cow::Borrowed(*wildcard_pattern));
                                }
                                for specific_id in &specific_ids {
                                    all_ids.push(Cow::Borrowed(*specific_id));
                                }
                                if all_ids.len() == 1 {
                                    EndpointScopeSelector::Specific(all_ids.pop().unwrap())
                                } else {
                                    EndpointScopeSelector::Multiple(all_ids)
                                }
                            } else {
                                // Fallback: if there are no IDs or patterns, default to All
                                EndpointScopeSelector::All
                            };

                            // Map the ResourceKind to the corresponding Endpoint variant.
                            // This ensures the correct endpoint type is used for each resource kind.
                            let endpoint = match resource_kind {
                                ResourceKind::Target => Endpoint::Target(endpoint_scope_selector),
                                ResourceKind::Prompt => Endpoint::Prompt(endpoint_scope_selector),
                                ResourceKind::Field => Endpoint::Field(endpoint_scope_selector),
                                ResourceKind::Response => {
                                    Endpoint::Response(endpoint_scope_selector)
                                }
                                ResourceKind::Export => Endpoint::Export(endpoint_scope_selector),
                                ResourceKind::Authorize => Endpoint::Authorize(None),
                            };

                            // Insert permission into the set for this endpoint.
                            // As soon as all three basic permissions (Read, Write, List) are present,
                            // immediately collapse to Permission::All for efficiency.
                            let perms = endpoint_to_permissions.entry(endpoint).or_default();
                            perms.insert(permission);
                            if perms.contains(&Permission::Read)
                                && perms.contains(&Permission::Write)
                                && perms.contains(&Permission::List)
                            {
                                perms.clear();
                                perms.insert(Permission::All);
                            }
                        }
                    }

                    // Now we have a mapping of endpoints to their permission sets.
                    // No need for a separate collapsing step; the set is already minimal.
                    let grants = endpoint_to_permissions
                        .into_iter()
                        .map(|(endpoint, permissions_set)| AuthorizationGrants {
                            endpoint,
                            permissions: permissions_set.into_iter().collect(),
                        })
                        .collect();

                    let authorization_mapping = AuthorizationMapping {
                        name: mapping_name.to_string(),
                        grants,
                    };

                    // Sort the mapping into scopes or groups based on the authorization type.
                    match authorization_type {
                        ResourceAuthorizationType::Scope => (Some(authorization_mapping), None),
                        ResourceAuthorizationType::Group => (None, Some(authorization_mapping)),
                        ResourceAuthorizationType::Subject => unreachable!(),
                    }
                },
            )
            .unzip();

    // Step 3: Build the final Export struct containing all scopes and groups.
    Export {
        scopes: scope_mappings.into_iter().flatten().collect(),
        groups: group_mappings.into_iter().flatten().collect(),
    }
}

#[derive(Serialize)]
pub struct Export<'a> {
    scopes: Vec<AuthorizationMapping<'a>>,
    groups: Vec<AuthorizationMapping<'a>>,
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
    let export = authorizations_to_endpoints(authorizations.as_slice());

    Ok(Response::new(ResourceAuthorizationExportResponse {
        export: hcl::to_string(&export)?,
    }))
}

#[cfg(test)]
mod tests {
    use crate::{
        database::schema::authorization::{
            ResourceAuthorization, ResourceAuthorizationType, ResourceKind,
        },
        Permission,
    };

    use super::{authorizations_to_endpoints, Endpoint, EndpointScopeSelector};

    #[test]
    fn export_authorizations() {
        let authorizations = &[
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::All)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::All)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Group)
                .authorization_value("group")
                .authorization_grant(Permission::All)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Group)
                .authorization_value("groups")
                .authorization_grant(Permission::All)
                .build(),
        ];

        let export = authorizations_to_endpoints(authorizations);

        let scopes = export.scopes;
        let groups = export.groups;

        assert_eq!(scopes.len(), 1);
        assert_eq!(groups.len(), 2);

        let mut scopes = scopes.iter();
        assert!(scopes
            .next()
            .is_some_and(|scope| scope.name().as_str().eq("scope")));

        let mut groups = groups.iter();
        assert!(groups.all(|group| ["group", "groups"].contains(&group.name().as_str())));
    }

    #[test]
    fn export_endpoints() {
        let authorizations = &[
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::All)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Field)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::All)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Prompt)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::All)
                .build(),
        ];

        let export = authorizations_to_endpoints(authorizations);
        let scope = export.scopes.first().unwrap();

        assert_eq!(scope.grants().len(), 3);
    }

    #[test]
    fn export_multiple_specifics_are_grouped() {
        let authorizations = &[
            ResourceAuthorization::builder()
                .resource_id(Some("id1".to_owned()))
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Read)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(Some("id2".to_owned()))
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Read)
                .build(),
        ];

        let export = authorizations_to_endpoints(authorizations);
        let scope = export.scopes.first().unwrap();
        let grant = scope.grants().first().unwrap();

        match grant.endpoint() {
            Endpoint::Target(EndpointScopeSelector::Multiple(ids)) => {
                assert!(ids.contains(&"id1".into()));
                assert!(ids.contains(&"id2".into()));
            }
            _ => panic!("Expected Multiple selector"),
        }
    }

    #[test]
    fn export_specific_and_wildcard_merges() {
        let authorizations = &[
            ResourceAuthorization::builder()
                .resource_id(Some("foo*".to_owned()))
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Read)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(Some("foo123".to_owned()))
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Read)
                .build(),
        ];

        let export = authorizations_to_endpoints(authorizations);
        let scope = export.scopes.first().unwrap();
        let grant = scope.grants().first().unwrap();

        match grant.endpoint() {
            Endpoint::Target(EndpointScopeSelector::Specific(wild)) => {
                assert_eq!(wild, "foo*");
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn export_different_permissions_same_id() {
        let authorizations = &[
            ResourceAuthorization::builder()
                .resource_id(Some("id1".to_owned()))
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Read)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(Some("id1".to_owned()))
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Write)
                .build(),
        ];

        let export = authorizations_to_endpoints(authorizations);
        let scope = export.scopes.first().unwrap();
        let grant = scope.grants().first().unwrap();

        assert!(grant.permissions().contains(&Permission::Read));
        assert!(grant.permissions().contains(&Permission::Write));
    }

    #[test]
    fn export_all_and_specific() {
        let authorizations = &[
            ResourceAuthorization::builder()
                .resource_id(None)
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Read)
                .build(),
            ResourceAuthorization::builder()
                .resource_id(Some("id1".to_owned()))
                .resource_kind(ResourceKind::Target)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_value("scope")
                .authorization_grant(Permission::Read)
                .build(),
        ];

        let export = authorizations_to_endpoints(authorizations);
        let scope = export.scopes.first().unwrap();
        let grant = scope.grants().first().unwrap();

        match grant.endpoint() {
            Endpoint::Target(EndpointScopeSelector::All) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn test_permission_collapse_to_all_and_negative_case() {
        let positive_auths = vec![
            ResourceAuthorization::builder()
                .resource_kind(ResourceKind::Target)
                .resource_id(None)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_grant(Permission::Read)
                .authorization_value("test-scope")
                .build(),
            ResourceAuthorization::builder()
                .resource_kind(ResourceKind::Target)
                .resource_id(None)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_grant(Permission::Write)
                .authorization_value("test-scope")
                .build(),
            ResourceAuthorization::builder()
                .resource_kind(ResourceKind::Target)
                .resource_id(None)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_grant(Permission::List)
                .authorization_value("test-scope")
                .build(),
        ];

        let export = authorizations_to_endpoints(&positive_auths);

        assert_eq!(export.scopes.len(), 1);
        let grants = &export.scopes[0].grants;
        assert_eq!(grants.len(), 1);
        let permissions = &grants[0].permissions;
        assert_eq!(permissions.len(), 1);
        assert!(permissions.contains(&Permission::All));

        let negative_auths = vec![
            ResourceAuthorization::builder()
                .resource_kind(ResourceKind::Target)
                .resource_id(None)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_grant(Permission::Read)
                .authorization_value("test-scope")
                .build(),
            ResourceAuthorization::builder()
                .resource_kind(ResourceKind::Target)
                .resource_id(None)
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_grant(Permission::Write)
                .authorization_value("test-scope")
                .build(),
        ];

        let export = authorizations_to_endpoints(&negative_auths);

        assert_eq!(export.scopes.len(), 1);
        let grants = &export.scopes[0].grants;
        assert_eq!(grants.len(), 1);
        let permissions = &grants[0].permissions;
        assert_eq!(permissions.len(), 2);
        assert!(permissions.contains(&Permission::Read));
        assert!(permissions.contains(&Permission::Write));
        assert!(!permissions.contains(&Permission::All));
    }
}
