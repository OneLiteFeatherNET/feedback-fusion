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

use std::str::FromStr;

use aliri::{
    jwa,
    jwt::{self, CoreValidator},
};
use aliri_oauth2::Authority;
use openidconnect::{
    ClientId, IssuerUrl,
    core::{CoreClient, CoreJwsSigningAlgorithm, CoreProviderMetadata},
};

use crate::prelude::*;

pub mod claims;
pub mod layer;

pub async fn authority() -> Result<(Authority, CoreClient)> {
    // sadly aliri does not support oidc yet, so we have to do the config stuff manually :(((((
    // discover the oidc endpoints
    let issuer = IssuerUrl::new(CONFIG.oidc().provider().clone()).map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Invalid discovery url: {error}"))
    })?;
    let metadata = CoreProviderMetadata::discover_async(
        issuer.clone(),
        openidconnect::reqwest::async_http_client,
    )
    .await
    .map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Invalid oidc endpoint: {error}"))
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

    // we do not use any routes requireing client authentication therefore we can just use dummys
    // here
    let client =
        CoreClient::from_provider_metadata(metadata, ClientId::new("dummy".to_owned()), None);

    Ok((authority, client))
}
