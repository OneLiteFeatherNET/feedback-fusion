//SPDX-FileCopyrightText: 2025 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2024 OneLiteFeatherNet

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

use arbitrary::Arbitrary;
use lazy_static::lazy_static;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse, Scope,
};

use crate::proto::CreateFieldRequest;

lazy_static! {
    pub static ref GRPC_ENDPOINT: String = std::env::var("GRPC_ENDPOINT").unwrap();
}

#[allow(unused)]
pub async fn authenticate(scope: &str, client_id: &str, client_secret: &str) -> String {
    let issuer = IssuerUrl::new(std::env::var("OIDC_PROVIDER").unwrap()).unwrap();
    let metadata = CoreProviderMetadata::discover_async(issuer, async_http_client)
        .await
        .unwrap();
    let client = CoreClient::from_provider_metadata(
        metadata,
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
    );

    let token_response = client
        .exchange_client_credentials()
        .add_scope(Scope::new(scope.to_string()))
        .request_async(async_http_client)
        .await
        .unwrap();

    token_response.access_token().secret().clone()
}

#[macro_export]
macro_rules! connect {
    () => {{
        let channel = tonic::transport::Channel::from_static(&$crate::tests::GRPC_ENDPOINT)
            .connect()
            .await
            .unwrap();
        let token: tonic::metadata::MetadataValue<_> = format!(
            "Bearer {}",
            $crate::tests::authenticate("api:feedback-fusion", "client", "secret").await
        )
        .parse()
        .unwrap();

        let client =
            $crate::proto::feedback_fusion_v1_client::FeedbackFusionV1Client::with_interceptor(
                channel,
                move |mut request: tonic::Request<()>| {
                    request
                        .metadata_mut()
                        .insert("authorization", token.clone());

                    Ok(request)
                },
            );

        let public_client =
            $crate::proto::public_feedback_fusion_v1_client::PublicFeedbackFusionV1Client::connect(
                $crate::tests::GRPC_ENDPOINT.as_str(),
            )
            .await
            .unwrap();

        (client, public_client)
    }};
    ($scope:ident) => {{
        let (client, public_client) = connect!();

        let channel = tonic::transport::Channel::from_static(&$crate::tests::GRPC_ENDPOINT)
            .connect()
            .await
            .unwrap();
        let token: tonic::metadata::MetadataValue<_> = format!(
            "Bearer {}",
            $crate::tests::authenticate(stringify!($scope), "dynamic", "secret").await
        )
        .parse()
        .unwrap();

        println!("{:?}", token);

        let scope_client =
            $crate::proto::feedback_fusion_v1_client::FeedbackFusionV1Client::with_interceptor(
                channel,
                move |mut request: tonic::Request<()>| {
                    request
                        .metadata_mut()
                        .insert("authorization", token.clone());

                    Ok(request)
                },
            );

        (client, public_client, scope_client)
    }};
}

impl Arbitrary<'_> for CreateFieldRequest {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(CreateFieldRequest {
            prompt: "".to_owned(),
            title: u
                .arbitrary::<String>()?
                .chars()
                .take(u.int_in_range(0..=34)?)
                .collect(),
            description: Some(
                u.arbitrary::<String>()?
                    .chars()
                    .take(u.int_in_range(0..=34)?)
                    .collect(),
            ),
            field_type: u.int_in_range(0..=6)?,
            options: Some(crate::proto::FieldOptions::arbitrary(u)?),
        })
    }
}
