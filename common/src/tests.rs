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

use crate::{
    common::ProtoResourceKind,
    proto::{
        proto_resource::Inner, AuditVersionPage, CreateFieldRequest, ProtoAuditAction, ProtoField,
        ProtoPrompt, ProtoTarget,
    },
};

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
            options: Some(crate::proto::ProtoFieldOptions::arbitrary(u)?),
        })
    }
}

pub trait VerifyAudit {
    #[allow(clippy::result_unit_err)]
    fn verify_audit_matches(
        &self,
        versions: AuditVersionPage,
        operation: ProtoAuditAction,
    ) -> Result<(), ()>
    where
        Self: Sized;
}

macro_rules! implement_verify_audit {
    ($proto_resource:path, $resource:ident, $($validator:ident $(,)?)+) => {
        impl VerifyAudit for $proto_resource {
            fn verify_audit_matches(&self, versions: AuditVersionPage, operation: ProtoAuditAction) -> Result<(), ()>
            where
                Self: Sized,
            {
                if versions
                    .audit_versions
                    .iter()
                    // verify the metadata matches for all entries
                    .map(|version| {
                        assert_eq!(version.resource_id.as_str(), self.id.as_str());
                        assert_eq!(version.resource_type, ProtoResourceKind::$resource as i32);
                        assert!(version.data.is_some());

                        version
                    })
                    // filter for the operation
                    .filter(|version| version.action().eq(&operation))
                    // now we try to find a audit version matching the data
                    .find(|version| {
                        let data = version.data.as_ref().unwrap();

                        if data.inner.is_some() {
                            let inner = data.inner.as_ref().unwrap();

                            if operation.eq(&ProtoAuditAction::Delete) {
                                return true;
                            }

                            // verify the inner value
                            if matches!(inner, Inner::$resource(_)) {
                                if let Inner::$resource(version) = inner {
                                   $(
                                        self.$validator.eq(&version.$validator) &&
                                    )*
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }).is_some() {
                        Ok(())
                    } else {
                        Err(())
                    }

            }
        }
    };
}

implement_verify_audit!(ProtoTarget, Target, id, name, description, updated_at);
implement_verify_audit!(
    ProtoPrompt,
    Prompt,
    id,
    title,
    description,
    target,
    active,
    updated_at
);
implement_verify_audit!(
    ProtoField,
    Field,
    id,
    prompt,
    title,
    description,
    options,
    field_type,
    updated_at
);

#[macro_export]
macro_rules! verify_audit_exists {
    ($client:ident, $inner:ident, $resource:ident, $action:ident) => {{
        let retry_strategy = tokio_retry::strategy::FibonacciBackoff::from_millis(1000)
            .map(tokio_retry::strategy::jitter)
            .take(5);
        let result = tokio_retry::Retry::spawn(retry_strategy, move || {
            let mut client = $client.clone();
            let inner = $inner.clone();

            async move {
                let audit_versions = client
                    .get_audit_versions(feedback_fusion_common::proto::GetAuditVersionsRequest {
                        page_size: 100,
                        page_token: 1,
                        resource_id: inner.id.clone(),
                        resource_type: feedback_fusion_common::common::ProtoResourceKind::$resource
                            as i32,
                    })
                    .await
                    .map_err(|_| ())?;

                inner.verify_audit_matches(
                    audit_versions.into_inner(),
                    feedback_fusion_common::proto::ProtoAuditAction::$action,
                )
            }
        })
        .await;
        assert!(result.is_ok());
    }};
}
