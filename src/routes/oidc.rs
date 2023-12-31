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

use aliri::jwt::{self, CoreValidator};
use aliri_clock::UnixTime;
use aliri_oauth2::{Authority, HasScope, Scope};
use openidconnect::{core::CoreProviderMetadata, IssuerUrl};

pub async fn authority() -> Result<Authority> {
    // discover the oidc endpoints
    let issuer = IssuerUrl::new(CONFIG.oidc_discovery_url().clone())
        .map_err(|_| FeedbackFusionError::ConfigurationError("invalid discovery url".to_owned()))?;
    let metadata =
        CoreProviderMetadata::discover_async(issuer, openidconnect::reqwest::async_http_client)
            .await
            .map_err(|_| {
                FeedbackFusionError::ConfigurationError("invalid oidc endpoint".to_owned())
            })?;
    // extract the jwks
    let jwks_url = metadata.jwks_uri().url();

    // build the validator
    let validator = CoreValidator::default();
    // build the authority
    let authority = Authority::new_from_url(jwks_url.to_string(), validator)
        .await
        .unwrap();

    Ok(authority)
}

#[derive(Debug, Clone, Deserialize)]
pub struct OIDCClaims {
    iss: jwt::Issuer,
    aud: jwt::Audiences,
    sub: jwt::Subject,
    scope: Scope,
}

impl jwt::CoreClaims for OIDCClaims {
    fn nbf(&self) -> Option<UnixTime> {
        None
    }
    fn exp(&self) -> Option<UnixTime> {
        None
    }
    fn aud(&self) -> &jwt::Audiences {
        &self.aud
    }
    fn iss(&self) -> Option<&jwt::IssuerRef> {
        Some(&self.iss)
    }
    fn sub(&self) -> Option<&jwt::SubjectRef> {
        Some(&self.sub)
    }
}

impl HasScope for OIDCClaims {
    fn scope(&self) -> &Scope {
        &self.scope
    }
}

pub mod scope {
    aliri_axum::scope_guards! {
        type Claims = super::OIDCClaims;

        pub scope API = "api:feedback-fusion";
        pub scope Read = ["api:feedback-fusion" || "feedback-fusion:read"];
        pub scope Write = ["api:feedback-fusion" || "feedback-fusion:write"];
    }
}
