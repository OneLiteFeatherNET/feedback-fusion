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
use std::collections::{BTreeSet, HashMap};
use tonic::Request;

use crate::{
    database::schema::user::{User, UserContext},
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

        let access_token = AccessToken::new(
            request
                .metadata()
                .get(AUTHORIZATION.as_str())
                .unwrap()
                .to_str()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .to_string(),
        );

        let scopes = claims.scope().iter().collect();
        let groups = claims.groups().iter().collect();

        if let Ok(context) = get_user_context(connection, subject.as_str(), &scopes, &groups).await
        {
            Ok(context)
        } else {
            let user = User::fetch(access_token, client).await?;

            database_request!(User::insert(connection, &user).await, "Save user")?;
            invalidate!(
                get_user_context,
                format!(
                    "get-user-context-{}-{:?}-{:?}",
                    subject.as_str(),
                    &scopes,
                    &groups
                )
            );

            get_user_context(connection, subject.as_str(), &scopes, &groups).await
        }
    }
}

#[dynamic_cache(
    ttl = "300",
    key = r#"format!('get-user-matrix-{:?}-{:?}', _scopes, _groups)"#
)]
async fn get_user_matrix(
    _connection: &DatabaseConnection,
    _scopes: &BTreeSet<&ScopeTokenRef>,
    _groups: &BTreeSet<&String>,
) -> Result<HashMap<String, bool>> {
    todo!()
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
        authorizations: get_user_matrix(connection, scopes, groups).await?,
    })
}
