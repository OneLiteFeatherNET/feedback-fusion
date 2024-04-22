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

use std::str::FromStr;

use crate::prelude::*;

use aliri::{
    jwa,
    jwt::{self, CoreValidator},
};
use aliri_clock::UnixTime;
use aliri_oauth2::{Authority, HasScope, Scope};
use openidconnect::{
    core::{CoreJwsSigningAlgorithm, CoreProviderMetadata},
    IssuerUrl,
};

pub async fn authority() -> Result<Authority> {
    // sadly aliri does not support oidc yet, so we have to do the config stuff manually :(((((
    // discover the oidc endpoints
    let issuer = IssuerUrl::new(CONFIG.oidc_discovery_url().clone())
        .map_err(|_| FeedbackFusionError::ConfigurationError("invalid discovery url".to_owned()))?;
    let metadata = CoreProviderMetadata::discover_async(
        issuer.clone(),
        openidconnect::reqwest::async_http_client,
    )
    .await
    .map_err(|_| FeedbackFusionError::ConfigurationError("invalid oidc endpoint".to_owned()))?;
    // extract the jwks
    let jwks_url = metadata.jwks_uri().url();
    // extract the algorithms
    let algorithms = metadata
        .id_token_signing_alg_values_supported()
        .iter()
        .filter_map(|key| match key {
            CoreJwsSigningAlgorithm::HmacSha256 => Some(jwa::Algorithm::HS256),
            CoreJwsSigningAlgorithm::HmacSha384 => Some(jwa::Algorithm::HS384),
            CoreJwsSigningAlgorithm::HmacSha512 => Some(jwa::Algorithm::HS512),
            CoreJwsSigningAlgorithm::EcdsaP256Sha256 => Some(jwa::Algorithm::PS256),
            CoreJwsSigningAlgorithm::EcdsaP384Sha384 => Some(jwa::Algorithm::PS384),
            CoreJwsSigningAlgorithm::EcdsaP521Sha512 => Some(jwa::Algorithm::PS512),
            CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256 => Some(jwa::Algorithm::RS256),
            CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha384 => Some(jwa::Algorithm::RS384),
            CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha512 => Some(jwa::Algorithm::RS512),
            _ => None,
        })
        .collect::<Vec<jwa::Algorithm>>();

    // build the validator
    let mut validator = CoreValidator::default()
        .add_allowed_audience(
            jwt::Audience::from_str(CONFIG.oidc_audience().as_str())
                .expect("Invalid oidc audience"),
        )
        .require_issuer(jwt::Issuer::from_str(issuer.as_str()).unwrap());
    for algorithm in algorithms {
        validator = validator.add_approved_algorithm(algorithm);
    };

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
    nbf: UnixTime,
    exp: UnixTime,
    scope: Scope,
}

impl jwt::CoreClaims for OIDCClaims {
    fn nbf(&self) -> Option<UnixTime> {
        Some(self.nbf)
    }
    fn exp(&self) -> Option<UnixTime> {
        Some(self.exp)
    }
    fn aud(&self) -> &jwt::Audiences {
        &self.aud
    }
    fn iss(&self) -> Option<&jwt::IssuerRef> {
        Some(&self.iss)
    }
    fn sub(&self) -> Option<&jwt::SubjectRef> {
        None
    }
}

impl HasScope for OIDCClaims {
    fn scope(&self) -> &Scope {
        &self.scope
    }
}

