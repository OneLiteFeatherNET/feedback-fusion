//SPDX-FileCopyrightText: 2023 OneLiteFeatherNet
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
use http::header::AUTHORIZATION;
use openidconnect::{core::CoreClient, AccessToken};
use rbs::to_value;
use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap},
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

impl UserContext {
    #[instrument(skip_all)]
    pub async fn get_otherwise_fetch<T>(
        request: &Request<T>,
        client: &CoreClient,
        connection: &DatabaseConnection,
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

        match get_user_context(connection, subject.as_str(), &scopes, &groups).await {
            Ok(context) => Ok(context),
            Err(FeedbackFusionError::OIDCError(_)) => {
                let user = User::fetch(access_token, client, claims).await?;

                // due to sync problems this could occur twice at the same time, therefore we just
                // dont use the result error. As the cache already got invalidated by the other
                // process we dont need to do it in that case :)
                if database_request!(User::insert(connection, &user).await, "Save user").is_ok() {
                    invalidate!(
                        get_user_context,
                        format!(
                            "get-user-context-{}-{:?}-{:?}",
                            subject.as_str(),
                            &scopes,
                            &groups
                        )
                    );
                }

                get_user_context(connection, subject.as_str(), &scopes, &groups).await
            }
            error => error,
        }
    }

    #[instrument(skip_all)]
    pub fn has_grant<'a>(
        scopes: &BTreeSet<&ScopeTokenRef>,
        groups: &BTreeSet<&String>,
        authorizations: &[ResourceAuthorization],
        endpoint: Endpoint<'a>,
        permission: Permission,
    ) -> Result<Vec<String>> {
        let authorization_string = Authorization(&endpoint, &permission).to_string();
        let entry = PERMISSION_MATRIX
            .get(&(endpoint.to_static(), permission.clone()))
            .ok_or(FeedbackFusionError::Unauthorized)?;

        // verify the scopes
        let scope = scopes.iter().find(|scope| entry.0.contains(scope.as_str()));

        // verify the groups
        let group = groups.iter().find(|group| entry.1.contains(group.as_str()));

        if scope.is_none() && group.is_none() {
            // find possibly matching authorizations
            let authorizations = authorizations.iter().filter(|authorization| {
                let matches_permission = authorization.authorization_grant().eq(&permission);
                let matches_endpoint = authorization.resource_kind().eq(&endpoint);
                let matches_id = authorization.resource_id().as_ref().is_none_or(|pattern| {
                    Wildcard::new(pattern.as_bytes())
                        .unwrap()
                        .is_match(authorization_string.as_bytes())
                });

                matches_permission && matches_endpoint && matches_id
            });

            Ok(authorizations
                .into_iter()
                .map(ResourceAuthorization::to_string)
                .collect())
        } else {
            Ok(vec![authorization_string])
        }
    }

    #[instrument(skip(self, connection))]
    pub async fn authorize(
        &self,
        connection: &DatabaseConnection,
        endpoint: &Endpoint<'_>,
        permission: &Permission,
    ) -> Result<()> {
        // we firrst need to verify that the user has the access to the target, therefore we get
        // the target id. We do not have to find prompts for fields as prompts and fields are
        // public available and therefore just have to verify the target
        let target_id = match endpoint {
            Endpoint::Target(id) => id.as_ref().map(|id| id.to_string()),
            Endpoint::Prompt(Some(id)) => Some(get_target_id_by_prompt_id(connection, id).await?),
            Endpoint::Field(Some(id)) => Some(get_target_id_by_field_id(connection, id).await?),
            _ => None,
        };

        // check wether we can access this target if its `Some`
        if let Some(target_id) = target_id {
            debug!("Detected {:?} belongs to target {}", endpoint, target_id);

            let authorization =
                Authorization(&Endpoint::Target(None), &Permission::Write).to_string();
            let endpoint = Endpoint::Target(Some(Cow::Borrowed(target_id.as_str())));

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
            debug!("Client does not have the permission to perform {:?} on {:?}", permission, endpoint);
            Err(FeedbackFusionError::Unauthorized)
        }
    }
}

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

    let wildcard = Wildcard::new(second.as_bytes()).unwrap();

    match endpoint {
        Endpoint::Target(Some(id)) | Endpoint::Prompt(Some(id)) | Endpoint::Field(Some(id)) => {
            if wildcard.is_match(id.as_bytes()) {
                debug!(
                    "ResourceAuthorization {} does not match {:?} with {:?}",
                    key, endpoint, permission
                );

                true
            } else {
                false
            }
        }
        _ => false,
    }
}

#[dynamic_cache(
    ttl = "300",
    key = r#"format!('get-user-matrix-{:?}-{:?}-{:?}', scopes, groups, subject)"#
)]
async fn get_user_matrix(
    connection: &DatabaseConnection,
    scopes: &BTreeSet<&ScopeTokenRef>,
    groups: &BTreeSet<&String>,
    subject: &str,
) -> Result<HashMap<String, bool>> {
    // fetch all matching resource authorizations
    let authorizations: Vec<ResourceAuthorization> = database_request!(
        ResourceAuthorization::select_matching(connection, scopes, groups, subject).await,
        "Select resource authorizations"
    )?;

    Ok(PERMISSION_MATRIX
        .clone()
        .into_iter()
        .flat_map(|entry| {
            let (endpoint, permission) = entry.0;
            let fallback = Authorization(&endpoint, &permission).to_string();

            if let Ok(keys) =
                UserContext::has_grant(scopes, groups, &authorizations, endpoint, permission)
            {
                keys.into_iter()
                    .map(|identifier| (identifier, true))
                    .collect()
            } else {
                vec![(fallback, false)]
            }
        })
        .collect::<HashMap<String, bool>>())
}

#[dynamic_cache(
    ttl = "300",
    key = r#"format!('get-user-context-{}-{:?}-{:?}', subject, scopes, groups)"#
)]
async fn get_user_context(
    connection: &DatabaseConnection,
    subject: &str,
    scopes: &BTreeSet<&ScopeTokenRef>,
    groups: &BTreeSet<&String>,
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
        authorizations: get_user_matrix(connection, scopes, groups, subject).await?,
    })
}

#[dynamic_cache(ttl = "3600", key = r#"format!('get-target-id-by-prompt-id-{}', id)"#)]
async fn get_target_id_by_prompt_id(connection: &DatabaseConnection, id: &str) -> Result<String> {
    let id: Option<String> = database_request!(
        connection
            .query_decode(
                "SELECT target FROM prompt WHERE id = ?",
                vec![to_value!(id)]
            )
            .await,
        "Select target id by prompt id"
    )?;

    id.ok_or(FeedbackFusionError::Unauthorized)
}

#[dynamic_cache(ttl = "3600", key = r#"format!('get-target-id-by-field-id-{}', id)"#)]
async fn get_target_id_by_field_id(connection: &DatabaseConnection, id: &str) -> Result<String> {
    let id: Option<String> = database_request!(
        connection
            .query_decode(
                "SELECT prompt.target FROM field field JOIN prompt prompt ON field.prompt = prompt.id WHERE field.id = ?",
                vec![to_value!(id)]
            )
            .await,
        "Select target id by field id"
    )?;

    id.ok_or(FeedbackFusionError::Unauthorized)
}
