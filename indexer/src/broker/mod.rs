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

use crate::{
    broker::{fluvio::FluvioBroker, grpc::GRPCBroker},
    config::{BrokerConfiguration, CONFIG},
    prelude::*,
};
use feedback_fusion_common::proto::ProtoEventBatch;
use kanal::{AsyncReceiver, AsyncSender};

pub mod fluvio;
pub mod grpc;

#[async_trait]
pub trait FeedbackFusionIndexerBrokerDriver: Send + Sync {
    fn try_from_config(config: BrokerConfiguration) -> Result<Self>
    where
        Self: Sized;

    async fn start_listener(
        &mut self,
        sender: AsyncSender<ProtoEventBatch>,
        shutdown_sender: AsyncSender<()>,
        shutdown_receiver: AsyncReceiver<()>,
    ) -> Result<()>;
}

pub struct FeedbackFusionIndexerBroker {
    driver: Box<dyn FeedbackFusionIndexerBrokerDriver>,
}

impl FeedbackFusionIndexerBroker {
    pub async fn initialize() -> Result<Self> {
        // fetch the configuration
        let config = CONFIG.broker().clone();
        let broker_driver: Box<dyn FeedbackFusionIndexerBrokerDriver>;

        if let Ok(driver) = FluvioBroker::try_from_config(config.clone()) {
            broker_driver = Box::new(driver);
        } else if let Ok(driver) = GRPCBroker::try_from_config(config.clone()) {
            broker_driver = Box::new(driver);
        } else {
            let error = "Either fluvio or grpc broker driver has to be configured";

            error!("{error}");
            return Err(FeedbackFusionError::ConfigurationError(error.to_owned()));
        }

        Ok(Self {
            driver: broker_driver,
        })
    }

    pub async fn start_loop(
        &mut self,
        shutdown_sender: AsyncSender<()>,
        shutdown_receiver: AsyncReceiver<()>,
    ) -> Result<AsyncReceiver<ProtoEventBatch>> {
        // create a new channel so we can send events to the producer
        let (sender, receiver) = kanal::unbounded_async();

        self.driver
            .start_listener(sender, shutdown_sender, shutdown_receiver)
            .await?;

        Ok(receiver)
    }
}
