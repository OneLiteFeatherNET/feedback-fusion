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

use fluvio::FluvioConfig;
use tonic::transport::Channel;
use tonic_health::pb::{HealthCheckRequest, health_client::HealthClient};

use crate::{config::BrokerConfiguration, prelude::*};

mod consumer;
mod producer;

#[async_trait::async_trait]
trait FeedbackFusionBrokerDriver {
    fn try_from_configuration(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized;

    async fn connect(&self) -> Result<()>;
}

pub struct FluvioBroker {
    config: FluvioConfig,
}

impl FeedbackFusionBrokerDriver for FluvioBroker {
    fn try_from_configuration(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(config) = config.fluvio() {
            Ok(Self {
                config: config.clone(),
            })
        } else {
            Err(FeedbackFusionError::ConfigurationError(
                "Required grpc broker configuraton missing".to_owned(),
            ))
        }
    }
}

pub struct GRPCBroker {
    config: GRPCBrokerDriverConfiguration,
}

#[async_trait::async_trait]
impl FeedbackFusionBrokerDriver for GRPCBroker {
    fn try_from_configuration(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(config) = config.grpc() {
            Ok(Self {
                config: config.clone(),
            })
        } else {
            Err(FeedbackFusionError::ConfigurationError(
                "Required grpc broker configuraton missing".to_owned(),
            ))
        }
    }

    async fn connect(&self) -> Result<()> {
        // try to connect to the health server
        let channel = Channel::from_shared(self.config.endpoint().clone())
            .map_err(|error| FeedbackFusionError::ConfigurationError(error.to_string()))?
            .connect()
            .await
            .map_err(|error| FeedbackFusionError::ConfigurationError(error.to_string()))?;
        let mut health_client = HealthClient::new(channel.clone());

        if health_client
            .check(HealthCheckRequest {
                service: "FeedbackFusionIndexerV1".to_owned(),
            })
            .await
            .is_ok()
        {
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
