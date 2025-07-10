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

use std::path::Path;

use feedback_fusion_common::event::{
    EventBatch,
    feedback_fusion_indexer_v1_server::{FeedbackFusionIndexerV1, FeedbackFusionIndexerV1Server},
};
use kanal::{AsyncReceiver, AsyncSender};
use tonic::{
    Request, Response, Status,
    transport::{Certificate, Identity, Server, ServerTlsConfig},
};

use crate::{
    broker::FeedbackFusionIndexerBrokerDriver,
    config::{BrokerConfiguration, GRPCBrokerConfiguration},
    prelude::*,
};

const ADDRESS: &str = "0.0.0.0:7000";

#[derive(Clone)]
pub struct GRPCBroker {
    sender: Option<AsyncSender<EventBatch>>,
    config: GRPCBrokerConfiguration,
}

#[async_trait]
impl FeedbackFusionIndexerV1 for GRPCBroker {
    #[instrument(skip_all)]
    async fn send_batch(
        &self,
        request: Request<EventBatch>,
    ) -> std::result::Result<Response<()>, Status> {
        let batch = request.into_inner();

        self.sender.as_ref().unwrap().send(batch).await.ok();

        Ok(Response::new(()))
    }
}

#[async_trait]
impl FeedbackFusionIndexerBrokerDriver for GRPCBroker {
    fn try_from_config(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(config) = config.grpc() {
            // parse the given paths to the certificate files and verify they exist
            if Path::new(config.certificate()).is_file()
                && Path::new(config.key()).is_file()
                && Path::new(config.certificate_authority()).is_file()
            {
                Ok(Self {
                    config: config.clone(),
                    sender: None,
                })
            } else {
                Err(FeedbackFusionError::ConfigurationError(
                    "Client certificate does not exist".to_owned(),
                ))
            }
        } else {
            Err(FeedbackFusionError::ConfigurationError(
                "Required grpc broker configuraton missing".to_owned(),
            ))
        }
    }

    async fn start_listener(
        &mut self,
        sender: AsyncSender<EventBatch>,
        shutdown_sender: AsyncSender<()>,
        shutdown_receiver: AsyncReceiver<()>,
    ) -> Result<()> {
        info!("Starting gRPC broker");
        self.sender = Some(sender);

        // parse the given paths to the certificate files and read them
        let certificate = tokio::fs::read_to_string(self.config.certificate()).await?;
        let key = tokio::fs::read_to_string(self.config.key()).await?;
        let certificate_authority =
            tokio::fs::read_to_string(self.config.certificate_authority()).await?;

        // build the client identity
        let identity = Identity::from_pem(certificate, key);
        let certificate = Certificate::from_pem(certificate_authority);
        let tls_config = ServerTlsConfig::new()
            .identity(identity)
            .client_ca_root(certificate);

        let (health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<FeedbackFusionIndexerV1Server<GRPCBroker>>()
            .await;

        debug!("Constructing reflection service");
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(
                feedback_fusion_common::event::FILE_DESCRIPTOR_SET,
            )
            .build_v1()
            .unwrap();

        info!("Listening for incoming gRPC requests on {ADDRESS}");
        let trace_layer = feedback_fusion_common::observability::otlp::trace_layer();

        let clone = self.clone();
        tokio::spawn(async move {
            if let Err(error) = async {
                Server::builder()
                    .tls_config(tls_config)?
                    .layer(trace_layer)
                    .accept_http1(true)
                    .add_service(health_service)
                    .add_service(reflection_service)
                    .add_service(FeedbackFusionIndexerV1Server::new(clone))
                    .serve_with_shutdown(ADDRESS.parse().unwrap(), async move {
                        shutdown_receiver.recv().await.ok();

                        info!("Got shotdown signal, gracefully stopping gRPC server");
                    })
                    .await?;

                Ok::<(), FeedbackFusionError>(())
            }
            .await
            {
                error!("Error while starting gRPC server: {error}");

                shutdown_sender.send(()).await.ok();
            }
        });

        Ok(())
    }
}
