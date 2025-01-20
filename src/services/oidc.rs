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
use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};

pub async fn authority() -> Result<Authority> {
    // sadly aliri does not support oidc yet, so we have to do the config stuff manually :(((((
    // discover the oidc endpoints
    let issuer = IssuerUrl::new(CONFIG.oidc().provider().clone()).map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Invalid discovery url: {}", error))
    })?;
    let metadata = CoreProviderMetadata::discover_async(
        issuer.clone(),
        openidconnect::reqwest::async_http_client,
    )
    .await
    .map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Invalid oidc endpoint: {}", error))
    })?;
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
            jwt::Audience::from_str(CONFIG.oidc().audience().as_str())
                .expect("Invalid oidc audience"),
        )
        .require_issuer(
            jwt::Issuer::from_str(
                CONFIG
                    .oidc()
                    .issuer()
                    .clone()
                    .unwrap_or(issuer.clone().to_string())
                    .as_str(),
            )
            .unwrap(),
        );
    for algorithm in algorithms {
        validator = validator.add_approved_algorithm(algorithm);
    }

    // build the authority
    let authority = Authority::new_from_url(jwks_url.to_string(), validator)
        .await
        .unwrap();

    Ok(authority)
}

// we do not know the group claim names during the compile time, therefore we do have to use this
// huge custom deserializer
#[derive(Debug, Clone)]
pub struct OIDCClaims {
    iss: jwt::Issuer,
    iat: UnixTime,
    aud: jwt::Audiences,
    nbf: Option<UnixTime>,
    exp: UnixTime,
    scope: Scope,
    groups: Vec<String>,
}

impl<'de> Deserialize<'de> for OIDCClaims {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(OIDCClaimsVisitor)
    }
}

struct OIDCClaimsVisitor;

impl<'de> Visitor<'de> for OIDCClaimsVisitor {
    type Value = OIDCClaims;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<V>(self, mut map: V) -> std::result::Result<OIDCClaims, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut iss = None;
        let mut iat = None;
        let mut aud = None;
        let mut nbf = None;
        let mut exp = None;
        let mut scope = None;
        let mut groups = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "iss" => {
                    iss = Some(map.next_value()?);
                }
                "iat" => {
                    iat = Some(map.next_value()?);
                }
                "aud" | "audience" => {
                    aud = Some(map.next_value()?);
                }
                "nbf" => {
                    nbf = Some(map.next_value()?);
                }
                "exp" => {
                    exp = Some(map.next_value()?);
                }
                "scope" => {
                    scope = Some(map.next_value()?);
                }
                _ if key.as_str() == CONFIG.oidc().group_claim().as_str() => {
                    groups = Some(map.next_value()?);
                }
                _ => {
                    let _: serde_json::Value = map.next_value()?;
                }
            }
        }

        let iss = iss.ok_or_else(|| serde::de::Error::missing_field("iss"))?;
        let iat = iat.ok_or_else(|| serde::de::Error::missing_field("iat"))?;
        let aud = aud.or(Some(jwt::Audiences::default())).unwrap();
        let exp = exp.ok_or_else(|| serde::de::Error::missing_field("exp"))?;
        let scope = scope.or(Some(Scope::empty())).unwrap();
        let groups = groups
            .ok_or_else(|| serde::de::Error::missing_field(CONFIG.oidc().group_claim().as_str()))?;

        Ok(OIDCClaims {
            iss,
            iat,
            aud,
            nbf,
            exp,
            scope,
            groups,
        })
    }
}

impl jwt::CoreClaims for OIDCClaims {
    fn nbf(&self) -> Option<UnixTime> {
        Some(self.nbf.unwrap_or(self.iat))
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

impl OIDCClaims {
    pub fn groups(&self) -> &Vec<String> {
        &self.groups
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthorizedService<S>(pub S);

impl<S, T> tonic::server::NamedService
    for AuthorizedService<tower_http::validate_request::ValidateRequestHeader<S, T>>
where
    S: tonic::server::NamedService,
{
    const NAME: &'static str = S::NAME;
}

impl<S, R> tower::Service<R> for AuthorizedService<S>
where
    S: tower::Service<R>,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    #[inline]
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: R) -> Self::Future {
        self.0.call(req)
    }
}
