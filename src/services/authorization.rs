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

use aliri::jwt::CoreClaims;
use aliri_oauth2::{scope::ScopeTokenRef, HasScope};
use async_recursion::async_recursion;
use http::header::AUTHORIZATION;
use openidconnect::{core::CoreClient, AccessToken};
use rbs::to_value;
use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap},
    ops::Deref,
};
use tonic::Request;
use wildcard::Wildcard;

use crate::{
    database::schema::{
        authorization::{Authorization, ResourceAuthorization},
        user::{User, UserContext},
    },
    prelude::*,
};

impl Endpoint<'_> {
    #[instrument(skip_all)]
    #[async_recursion]
    pub async fn get_target_ids(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<Option<Vec<String>>> {
        match self {
            Endpoint::Target(EndpointScopeSelector::Specific(id))
            | Endpoint::Export(EndpointScopeSelector::Specific(id)) => {
                Ok(Some(vec![id.to_string()]))
            }
            Endpoint::Prompt(EndpointScopeSelector::Specific(id))
            | Endpoint::Response(EndpointScopeSelector::Specific(id)) => {
                Ok(Some(get_target_ids_by_prompt_ids(connection, &[id]).await?))
            }
            Endpoint::Field(EndpointScopeSelector::Specific(id)) => {
                Ok(Some(get_target_ids_by_field_ids(connection, &[id]).await?))
            }
            Endpoint::Target(EndpointScopeSelector::Multiple(ids))
            | Endpoint::Export(EndpointScopeSelector::Multiple(ids)) => {
                Ok(Some(ids.iter().map(|id| id.to_string()).collect()))
            }
            Endpoint::Prompt(EndpointScopeSelector::Multiple(ids))
            | Endpoint::Response(EndpointScopeSelector::Multiple(ids)) => {
                let id_refs: Vec<&str> = ids.iter().map(|id| id.as_ref()).collect();
                Ok(Some(
                    get_target_ids_by_prompt_ids(connection, &id_refs).await?,
                ))
            }
            Endpoint::Field(EndpointScopeSelector::Multiple(ids)) => {
                let id_refs: Vec<&str> = ids.iter().map(|id| id.as_ref()).collect();
                Ok(Some(
                    get_target_ids_by_field_ids(connection, &id_refs).await?,
                ))
            }
            Endpoint::Authorize(Some(boxed_endpoint)) => {
                boxed_endpoint.get_target_ids(connection).await
            }
            _ => Ok(None),
        }
    }
}

impl UserContext {
    #[instrument(skip_all)]
    pub async fn get_otherwise_fetch<'a, T>(
        request: &Request<T>,
        client: &CoreClient,
        connection: &DatabaseConnection,
        matrix: &PermissionMatrix<'a>,
    ) -> Result<Self> {
        let claims = request
            .extensions()
            .get::<OIDCClaims>()
            .ok_or(FeedbackFusionError::Unauthorized)?;

        let subject = claims
            .sub()
            .ok_or(FeedbackFusionError::OIDCError(
                "Client is missing sub claim".to_owned(),
            ))?
            .to_string();

        let bearer = request
            .metadata()
            .get(AUTHORIZATION.as_str())
            .unwrap()
            .to_str()
            .unwrap();
        let access_token = AccessToken::new(bearer.split(" ").last().unwrap().to_string());

        let scopes = claims.scope().iter().collect();
        let groups = claims.groups().iter().collect();

        match get_user_context(connection, subject.as_str(), &scopes, &groups, matrix).await {
            Ok(context) => Ok(context),
            Err(FeedbackFusionError::OIDCError(_)) => {
                // let user = User::fetch(access_token, client, claims).await?;

                // TODO: cleanup with new cache implementation for auithentication

                // // due to sync problems this could occur twice at the same time, therefore we just
                // // dont use the result error. As the cache already got invalidated by the other
                // // process we dont need to do it in that case :)
                // if database_request!(User::insert(connection, &user).await, "Save user").is_ok() {
                //     invalidate!(
                //         get_user_context,
                //         format!(
                //             "get-user-context-{}-{:?}-{:?}",
                //             subject.as_str(),
                //             &scopes,
                //             &groups
                //         )
                //     );
                // }

                get_user_context(connection, subject.as_str(), &scopes, &groups, matrix).await
            }
            error => error,
        }
    }

    /// Only for internal user matrix creation, use `authorize` for UserContext based permission
    /// checks
    #[instrument(skip_all)]
    pub fn has_grant<'a>(
        scopes: &BTreeSet<&ScopeTokenRef>,
        groups: &BTreeSet<&String>,
        authorizations: &[ResourceAuthorization],
        endpoint: &'a Endpoint<'a>,
        permission: &'a Permission,
        matrix: &PermissionMatrix<'a>,
    ) -> Result<Vec<String>> {
        let authorization_string = Authorization(endpoint, permission).to_string();
        let entry = matrix
            .get(&PermissionMatrixKey::from((endpoint, permission)))
            .ok_or(FeedbackFusionError::Unauthorized)?;

        // verify the scopes
        let scope = scopes
            .iter()
            .any(|scope| entry.scopes().contains(scope.as_str()));

        // verify the groups
        let group = groups
            .iter()
            .any(|group| entry.groups().contains(group.as_str()));

        let mut authorizations = {
            // find possibly matching authorizations
            let authorizations = authorizations.iter().filter(|authorization| {
                let matches_permission = authorization.authorization_grant().eq(permission);
                let matches_endpoint = authorization.resource_kind().eq(endpoint);
                // we do not have to verify the endpoint here as this function is only called for
                // user matrix creation

                matches_permission && matches_endpoint
            });

            authorizations
                .into_iter()
                .map(ResourceAuthorization::to_string)
                .collect::<Vec<_>>()
        };

        if scope || group {
            authorizations.push(authorization_string);
        }

        Ok(authorizations)
    }

    #[instrument(skip(self, connection))]
    pub async fn authorize(
        &self,
        connection: &DatabaseConnection,
        endpoint: &Endpoint<'_>,
        permission: &Permission,
    ) -> Result<()> {
        // we first need to verify that the user has the access to the target, therefore we get
        // the target id. We do not have to find prompts for fields as prompts and fields are
        // public available and therefore just have to verify the target
        let target_ids = endpoint.get_target_ids(connection).await?;

        // check wether we can access this target if its `Some`
        if let Some(target_ids) = target_ids {
            if !target_ids.into_iter().all(|target_id| {
                debug!("Detected {:?} belongs to target {}", endpoint, target_id);

                let authorization = Authorization(
                    &Endpoint::Target(EndpointScopeSelector::default()),
                    &Permission::Write,
                )
                .to_string();
                let endpoint = Endpoint::Target(EndpointScopeSelector::Specific(Cow::Borrowed(
                    target_id.as_str(),
                )));

                if self
                    .authorizations
                    .get(&authorization)
                    .is_some_and(|value| !*value)
                    && !self
                        .authorizations
                        .keys()
                        // custom authorizations have 2 times :: in it therefore we can divide the keys
                        // into custom authorizations and permission matrix authorizations. Custom resource
                        // authorizations are always with value true therefore we just have to search for
                        // any matching custom auth key
                        .any(|key| is_match(key, &endpoint, &Permission::Read))
                {
                    debug!("Client is not authorized to access target {}", target_id);
                    false
                } else {
                    true
                }
            }) {
                return Err(FeedbackFusionError::Unauthorized);
            }
        }

        // now we made sure the user can access the target and we now verify if he can access the
        // endpoint he called
        let authorization = Authorization(&endpoint.none(), permission).to_string();
        if self
            .authorizations
            .get(&authorization)
            .is_some_and(|value| *value)
            || self
                .authorizations
                .keys()
                .any(|key| is_match(key, endpoint, permission))
        {
            Ok(())
        } else {
            debug!(
                "Client does not have the permission to perform {:?} on {:?}",
                permission, endpoint
            );
            Err(FeedbackFusionError::Unauthorized)
        }
    }
}

/// Verify wether a auth key does match the given endpoint and permission
#[instrument]
fn is_match(key: &String, endpoint: &Endpoint, permission: &Permission) -> bool {
    let mut split = key.split("::");

    let (Some(first), Some(second), Some(third)) = (split.next(), split.next(), split.next())
    else {
        return false;
    };

    // verify the given endpoint is a match
    if !first.eq(Into::<&str>::into(endpoint)) {
        return false;
    }

    // verify it is the same permission
    if !third.eq(Into::<&str>::into(permission)) {
        return false;
    }

    // TODO: generate via macro
    let ids = match endpoint {
        Endpoint::Target(EndpointScopeSelector::Specific(id))
        | Endpoint::Prompt(EndpointScopeSelector::Specific(id))
        | Endpoint::Field(EndpointScopeSelector::Specific(id))
        | Endpoint::Export(EndpointScopeSelector::Specific(id))
        | Endpoint::Response(EndpointScopeSelector::Specific(id)) => vec![id],
        Endpoint::Target(EndpointScopeSelector::Multiple(list))
        | Endpoint::Prompt(EndpointScopeSelector::Multiple(list))
        | Endpoint::Field(EndpointScopeSelector::Multiple(list))
        | Endpoint::Export(EndpointScopeSelector::Multiple(list)) => list.iter().collect(),
        Endpoint::Authorize(Some(boxed_endpoint)) => match boxed_endpoint.deref() {
            Endpoint::Target(EndpointScopeSelector::Specific(id))
            | Endpoint::Prompt(EndpointScopeSelector::Specific(id))
            | Endpoint::Field(EndpointScopeSelector::Specific(id))
            | Endpoint::Export(EndpointScopeSelector::Specific(id))
            | Endpoint::Response(EndpointScopeSelector::Specific(id)) => vec![id],
            Endpoint::Target(EndpointScopeSelector::Multiple(list))
            | Endpoint::Prompt(EndpointScopeSelector::Multiple(list))
            | Endpoint::Field(EndpointScopeSelector::Multiple(list))
            | Endpoint::Export(EndpointScopeSelector::Multiple(list)) => list.iter().collect(),
            // we do not allow further recursion, therefore we ignore this
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };

    // if the list is emptry it means we have a None inside the target endpoint, therefore we have
    // to check wether the wildcard matches everything
    if ids.is_empty() {
        second.eq("*")
    } else {
        let wildcard = Wildcard::new(second.as_bytes()).unwrap();

        ids.iter().all(|id| wildcard.is_match(id.as_bytes()))
    }
}

// #[dynamic_cache(
//     ttl = "300",
//     key = r#"format!('get-user-matrix-{:?}-{:?}-{:?}', scopes, groups, subject)"#
// )]
#[instrument(skip_all)]
async fn get_user_matrix<'a>(
    connection: &DatabaseConnection,
    scopes: &BTreeSet<&ScopeTokenRef>,
    groups: &BTreeSet<&String>,
    subject: &str,
    matrix: &PermissionMatrix<'a>,
) -> Result<HashMap<String, bool>> {
    // fetch all matching resource authorizations
    let authorizations: Vec<ResourceAuthorization> = database_request!(
        ResourceAuthorization::select_matching(connection, scopes, groups, subject).await,
        "Select resource authorizations"
    )?;

    Ok(matrix
        .iter()
        .flat_map(|entry| {
            let endpoint = entry.key().endpoint();
            let permission = entry.key().permission();

            let fallback = Authorization(endpoint, permission).to_string();

            if let Ok(keys) = UserContext::has_grant(
                scopes,
                groups,
                &authorizations,
                endpoint,
                permission,
                matrix,
            ) {
                keys.into_iter()
                    .map(|identifier| (identifier, true))
                    .collect()
            } else {
                vec![(fallback, false)]
            }
        })
        .collect::<HashMap<String, bool>>())
}

// #[dynamic_cache(
//     ttl = "300",
//     key = r#"format!('get-user-context-{}-{:?}-{:?}', subject, scopes, groups)"#
// )]
#[instrument(skip_all)]
async fn get_user_context<'a>(
    connection: &DatabaseConnection,
    subject: &str,
    scopes: &BTreeSet<&ScopeTokenRef>,
    groups: &BTreeSet<&String>,
    matrix: &PermissionMatrix<'a>,
) -> Result<UserContext> {
    let user = database_request!(
        User::select_by_id(connection, subject).await,
        "Fetch user by subject"
    )?
    .ok_or(FeedbackFusionError::OIDCError(format!(
        "User {} does not exist yet",
        subject
    )))?;

    Ok(UserContext {
        user,
        authorizations: get_user_matrix(connection, scopes, groups, subject, matrix).await?,
    })
}

#[dynamic_cache(
    ttl = "3600",
    key = r#"format!('get-target-id-by-prompt-id-{:?}', id)"#
)]
#[instrument(skip_all)]
async fn get_target_ids_by_prompt_ids(
    connection: &DatabaseConnection,
    id: &[&str],
) -> Result<Vec<String>> {
    Ok(database_request!(
        connection
            .query_decode::<Vec<String>>(
                "SELECT target FROM prompt WHERE id IN ?",
                vec![to_value!(id)]
            )
            .await,
        "Select target id by prompt id"
    )?)
}

#[dynamic_cache(ttl = "3600", key = r#"format!('get-target-id-by-field-id-{:?}', id)"#)]
#[instrument(skip_all)]
async fn get_target_ids_by_field_ids(
    connection: &DatabaseConnection,
    id: &[&str],
) -> Result<Vec<String>> {
    Ok(database_request!(
        connection
            .query_decode::<Vec<String>>(
                "SELECT prompt.target FROM field field JOIN prompt prompt ON field.prompt = prompt.id WHERE field.id IN ?",
                vec![to_value!(id)]
            )
            .await,
        "Select target id by field id"
    )?)
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::BTreeSet};

    use aliri_oauth2::scope::ScopeTokenRef;

    use crate::{
        authorization::is_match,
        database::schema::{
            authorization::{
                ResourceAuthorization, ResourceAuthorizationGrant, ResourceAuthorizationType,
                ResourceKind,
            },
            user::UserContext,
        },
        Endpoint, Permission,
    };

    #[test]
    fn test_is_match_empty() {
        assert!(!is_match(
            &"".to_owned(),
            &Endpoint::Target(Some(Cow::Borrowed("Foo"))),
            &Permission::Write
        ));
    }

    #[test]
    fn test_is_match_invalid_format() {
        let endpoint = &Endpoint::Target(Some(Cow::Borrowed("Foo")));
        let permission = &Permission::Write;

        assert!(!is_match(&"Foo".to_owned(), endpoint, permission));
        assert!(!is_match(&"Foo::Bar".to_owned(), endpoint, permission));
        assert!(!is_match(
            &"Foo::Bar::FooBar".to_owned(),
            endpoint,
            permission
        ));
    }

    #[test]
    fn test_is_match_endpoint_matches() {
        let endpoint = &Endpoint::Target(Some(Cow::Borrowed("Foo")));
        let permission = &Permission::Write;

        assert!(is_match(
            &"Target::Foo::Write".to_owned(),
            endpoint,
            permission
        ));
        assert!(!is_match(
            &"Targett::Foo::Write".to_owned(),
            endpoint,
            permission
        ));
        assert!(!is_match(
            &"Target::Foo::Write".to_owned(),
            &Endpoint::Prompt(None),
            permission
        ));
    }

    #[test]
    fn test_is_match_permission_matches() {
        let endpoint = &Endpoint::Target(Some(Cow::Borrowed("Foo")));
        let permission = &Permission::Write;

        assert!(is_match(
            &"Target::Foo::Write".to_owned(),
            endpoint,
            permission
        ));
        assert!(!is_match(
            &"Target::Foo::Writer".to_owned(),
            endpoint,
            permission
        ));
        assert!(!is_match(
            &"Target::Foo::Write".to_owned(),
            endpoint,
            &Permission::Read
        ));
    }

    #[test]
    fn test_is_match_wildcard() {
        let endpoint = &Endpoint::Target(Some(Cow::Borrowed("FooBar")));
        let permission = &Permission::Write;

        assert!(is_match(
            &"Target::FooBar::Write".to_owned(),
            endpoint,
            permission
        ));

        assert!(!is_match(
            &"Target::FooBa::Write".to_owned(),
            endpoint,
            permission
        ));

        assert!(is_match(
            &"Target::*::Write".to_owned(),
            endpoint,
            permission
        ));

        assert!(is_match(
            &"Target::F*r::Write".to_owned(),
            endpoint,
            permission
        ));

        assert!(!is_match(
            &"Target::f*r::Write".to_owned(),
            endpoint,
            permission
        ));
    }

    #[test]
    fn test_is_match_full_wildcard() {
        let endpoint = &Endpoint::Target(None);
        let permission = &Permission::Write;

        assert!(!is_match(
            &"Target::FooBar::Write".to_owned(),
            endpoint,
            permission
        ));

        assert!(is_match(
            &"Target::*::Write".to_owned(),
            endpoint,
            permission
        ));

        assert!(!is_match(
            &"Target::F*::Write".to_owned(),
            endpoint,
            permission
        ));
    }

    #[test]
    fn test_has_grant_empty() {
        assert!(UserContext::has_grant(
            &BTreeSet::new(),
            &BTreeSet::new(),
            &[],
            Endpoint::Target(None),
            Permission::Write
        )
        .is_ok_and(|list| list.is_empty()));
    }

    fn has_grant_setup<'a>() -> (
        BTreeSet<&'a ScopeTokenRef>,
        BTreeSet<String>,
        BTreeSet<&'a ScopeTokenRef>,
        BTreeSet<String>,
        Vec<ResourceAuthorization>,
    ) {
        let valid_scopes: BTreeSet<&ScopeTokenRef> =
            BTreeSet::from([ScopeTokenRef::from_static("api:feedback-fusion")]);
        let valid_groups: BTreeSet<String> = BTreeSet::from(["admin".to_owned()]);

        let invalid_scopes: BTreeSet<&ScopeTokenRef> =
            BTreeSet::from([ScopeTokenRef::from_static("foo")]);
        let invalid_groups: BTreeSet<String> = BTreeSet::from(["bar".to_owned()]);

        let resource_authorizations = vec![
            ResourceAuthorization::builder()
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_grant(ResourceAuthorizationGrant::Write)
                .authorization_value("".to_owned())
                .resource_kind(ResourceKind::Target)
                .resource_id("".to_owned())
                .build(),
            ResourceAuthorization::builder()
                .authorization_type(ResourceAuthorizationType::Scope)
                .authorization_grant(ResourceAuthorizationGrant::Read)
                .authorization_value("".to_owned())
                .resource_kind(ResourceKind::Prompt)
                .resource_id("".to_owned())
                .build(),
        ];

        (
            valid_scopes,
            valid_groups,
            invalid_scopes,
            invalid_groups,
            resource_authorizations,
        )
    }

    #[test]
    fn test_has_grant_scopes_and_groups_only() {
        let endpoint = Endpoint::Target(None);
        let permission = Permission::Write;
        let (valid_scopes, valid_groups, invalid_scopes, invalid_groups, _) = has_grant_setup();
        let valid_groups = valid_groups.iter().collect::<BTreeSet<&String>>();
        let invalid_groups = invalid_groups.iter().collect::<BTreeSet<&String>>();

        assert!(UserContext::has_grant(
            &valid_scopes,
            &valid_groups,
            &[],
            endpoint.clone(),
            permission.clone()
        )
        .is_ok_and(|list| !list.is_empty()));

        assert!(UserContext::has_grant(
            &valid_scopes,
            &invalid_groups,
            &[],
            endpoint.clone(),
            permission.clone()
        )
        .is_ok_and(|list| !list.is_empty()));

        assert!(UserContext::has_grant(
            &invalid_scopes,
            &valid_groups,
            &[],
            endpoint.clone(),
            permission.clone()
        )
        .is_ok_and(|list| !list.is_empty()));

        assert!(UserContext::has_grant(
            &invalid_scopes,
            &invalid_groups,
            &[],
            endpoint.clone(),
            permission.clone()
        )
        .is_ok_and(|list| list.is_empty()));
    }

    #[test]
    fn test_has_grant_resource_authorizations_only() {
        let (_, _, _, _, resource_authorizations) = has_grant_setup();

        assert!(UserContext::has_grant(
            &BTreeSet::new(),
            &BTreeSet::new(),
            resource_authorizations.as_slice(),
            Endpoint::Target(None),
            Permission::Write
        )
        .is_ok_and(|list| !list.is_empty()));

        assert!(UserContext::has_grant(
            &BTreeSet::new(),
            &BTreeSet::new(),
            resource_authorizations.as_slice(),
            Endpoint::Target(None),
            Permission::Read
        )
        .is_ok_and(|list| list.is_empty()));

        assert!(UserContext::has_grant(
            &BTreeSet::new(),
            &BTreeSet::new(),
            resource_authorizations.as_slice(),
            Endpoint::Field(None),
            Permission::Read
        )
        .is_ok_and(|list| list.is_empty()));

        assert!(UserContext::has_grant(
            &BTreeSet::new(),
            &BTreeSet::new(),
            resource_authorizations.as_slice(),
            Endpoint::Prompt(None),
            Permission::Read
        )
        .is_ok_and(|list| !list.is_empty()));

        assert!(UserContext::has_grant(
            &BTreeSet::new(),
            &BTreeSet::new(),
            resource_authorizations.as_slice(),
            Endpoint::Prompt(None),
            Permission::Write
        )
        .is_ok_and(|list| list.is_empty()));
    }
}
