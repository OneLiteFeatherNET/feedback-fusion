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

use std::collections::HashMap;

use crate::prelude::*;
use aliri::jwt::CoreClaims;
use openidconnect::{AccessToken, core::CoreUserInfoClaims};

#[derive(
    Deserialize, Serialize, Clone, Derivative, Debug, Getters, Setters, TypedBuilder, Encode, Decode,
)]
#[derivative(PartialEq)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct User {
    id: String,
    username: String,
}

crud!(User {}, "oidc_user");
impl_select!(User {select_by_id(id: &str) -> Option => "`WHERE id = #{id}`"}, "oidc_user");

impl From<CoreUserInfoClaims> for User {
    fn from(claims: CoreUserInfoClaims) -> Self {
        Self::builder()
            .id(claims.subject().to_string())
            .username(
                claims
                    .preferred_username()
                    .map(|username| username.to_string())
                    .unwrap_or_default(),
            )
            .build()
    }
}

impl User {
    #[instrument(skip_all)]
    pub async fn fetch(
        access_token: AccessToken,
        client: &openidconnect::core::CoreClient,
        claims: &OIDCClaims,
    ) -> Result<Self> {
        if claims.is_application() {
            let sub = claims
                .sub()
                .ok_or(FeedbackFusionError::Unauthorized)?
                .to_string();
            let preferred_username = claims.preferred_username();

            Ok(Self::builder()
                .id(&sub)
                .username(preferred_username)
                .build())
        } else {
            let user_info: CoreUserInfoClaims = client
                .user_info(access_token, None)
                .map_err(|error| FeedbackFusionError::OIDCError(error.to_string()))?
                .request_async(openidconnect::reqwest::async_http_client)
                .await
                .map_err(|error| FeedbackFusionError::OIDCError(error.to_string()))?;

            Ok(Self::from(user_info))
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Getters, PartialEq, Encode, Decode)]
#[get = "pub"]
pub struct UserContext {
    pub user: User,
    pub authorizations: HashMap<String, bool>,
}
