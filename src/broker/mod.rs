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

use feedback_fusion_common::event::feedback_fusion_indexer_v1_client::FeedbackFusionIndexerV1Client;
use fluvio::{Fluvio, FluvioClusterConfig};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tonic_health::pb::{HealthCheckRequest, health_client::HealthClient};

use crate::{config::BrokerConfiguration, prelude::*};

mod consumer;
mod producer;

#[async_trait::async_trait]
trait FeedbackFusionBrokerDriver {
    fn try_from_configuration(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized;

    async fn connect(&mut self) -> Result<()>;
}

pub struct FluvioBroker {
    config: FluvioClusterConfig,
    fluvio: Option<Fluvio>,
}

#[async_trait::async_trait]
impl FeedbackFusionBrokerDriver for FluvioBroker {
    fn try_from_configuration(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(config) = config.fluvio() {
            Ok(Self {
                config: config.clone(),
                fluvio: None,
            })
        } else {
            Err(FeedbackFusionError::ConfigurationError(
                "Required grpc broker configuraton missing".to_owned(),
            ))
        }
    }

    async fn connect(&mut self) -> Result<()> {
        let fluvio = Fluvio::connect_with_config(&self.config)
            .await
            .map_err(|error| FeedbackFusionError::ConfigurationError(error.to_string()))?;

        self.fluvio = Some(fluvio);

        Ok(())
    }
}

pub struct GRPCBroker {
    config: GRPCBrokerDriverConfiguration,
    client: Option<FeedbackFusionIndexerV1Client<Channel>>,
}

#[async_trait::async_trait]
impl FeedbackFusionBrokerDriver for GRPCBroker {
    fn try_from_configuration(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(config) = config.grpc() {
            // parse the given paths to the certificate files and verify they exist
            if Path::new(config.tls().certificate()).is_file()
                && Path::new(config.tls().key()).is_file()
                && Path::new(config.tls().certificate_authority()).is_file()
            {
                Ok(Self {
                    config: config.clone(),
                    client: None,
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

    async fn connect(&mut self) -> Result<()> {
        // parse the given paths to the certificate files and read them
        let certificate = tokio::fs::read_to_string(self.config.tls().certificate()).await?;
        let key = tokio::fs::read_to_string(self.config.tls().key()).await?;
        let certificate_authority =
            tokio::fs::read_to_string(self.config.tls().certificate_authority()).await?;

        // build the client identity
        let identity = Identity::from_pem(certificate, key);
        let certificate = Certificate::from_pem(certificate_authority);
        let tls_config = ClientTlsConfig::new()
            .identity(identity)
            .ca_certificate(certificate);

        // try to connect to the health server
        let channel = Channel::from_shared(self.config.endpoint().clone())
            .map_err(|error| FeedbackFusionError::ConfigurationError(error.to_string()))?
            .tls_config(tls_config)
            .map_err(|error| FeedbackFusionError::ConfigurationError(error.to_string()))?
            .connect()
            .await
            .map_err(|error| FeedbackFusionError::ConfigurationError(error.to_string()))?;
        let mut health_client = HealthClient::new(channel.clone());

        // check wether the service is reachable
        info!(
            "Sending HealthCheckRequest to the gRPC indexer on {}",
            self.config.endpoint()
        );
        if health_client
            .check(HealthCheckRequest {
                service: "FeedbackFusionIndexerV1".to_owned(),
            })
            .await
            .is_ok()
        {
            let client = FeedbackFusionIndexerV1Client::new(channel);
            self.client = Some(client);
            info!("Connected to gRPC indexer");

            Ok(())
        } else {
            Err(FeedbackFusionError::ConfigurationError(
                "Indexer grpc service is unhealthy".to_owned(),
            ))
        }
    }
}

pub struct FeedbackFusionBroker {
    driver: Box<dyn FeedbackFusionBrokerDriver>,
}

impl FeedbackFusionBroker {
    pub async fn initialize() -> Result<Self> {
        // fetch the configuration
        let config = CONFIG.broker().clone();
        let broker_driver: Box<dyn FeedbackFusionBrokerDriver>;

        if let Ok(driver) = FluvioBroker::try_from_configuration(config.clone()) {
            broker_driver = Box::new(driver);
        } else {
            if let Ok(driver) = GRPCBroker::try_from_configuration(config.clone()) {
                broker_driver = Box::new(driver);
            } else {
                let error = "Either fluvio or grpc broker driver has to be configured";

                error!("{error}");
                return Err(FeedbackFusionError::ConfigurationError(error.to_owned()));
            }
        }

        Ok(Self {
            driver: broker_driver,
        })
    }
}
