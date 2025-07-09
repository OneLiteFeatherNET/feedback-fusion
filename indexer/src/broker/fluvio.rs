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
    broker::FeedbackFusionIndexerBrokerDriver,
    config::{BrokerConfiguration, FluvioBrokerConfiguration},
    prelude::*,
};
use feedback_fusion_common::event::EventBatch;
use fluvio::{Fluvio, Offset, consumer::ConsumerConfigExtBuilder};
use futures::StreamExt;
use kanal::{AsyncReceiver, AsyncSender};
use prost::Message;

pub struct FluvioBroker {
    config: FluvioBrokerConfiguration,
    fluvio: Option<Fluvio>,
}

#[async_trait]
impl FeedbackFusionIndexerBrokerDriver for FluvioBroker {
    fn try_from_config(config: BrokerConfiguration) -> Result<Self>
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
                "Required fluvio broker configuraton missing".to_owned(),
            ))
        }
    }

    async fn start_listener(
        &mut self,
        sender: AsyncSender<EventBatch>,
        _: AsyncSender<()>,
        shutdown_receiver: AsyncReceiver<()>,
    ) -> Result<()> {
        info!("Starting fluvio broker");

        let fluvio = Fluvio::connect_with_config(self.config.fluvio())
            .await
            .map_err(|error| FeedbackFusionError::ConfigurationError(error.to_string()))?;

        let mut consumer = fluvio
            .consumer_with_config(
                ConsumerConfigExtBuilder::default()
                    .topic(self.config.topic())
                    .partition(0)
                    .offset_start(Offset::end())
                    .build()
                    .unwrap(),
            )
            .await?;

        tokio::spawn(async move {
            loop {
                // listen to the shutdown signal
                if let Ok(Some(())) = shutdown_receiver.try_recv() {
                    info!("Got shotdown signal, gracefully stopping fluvio consumer");
                    break;
                }

                if let Some(event) = consumer.next().await {
                    match event {
                        Ok(record) => match EventBatch::decode(record.value()) {
                            Ok(batch) => {
                                sender.send(batch).await.ok();
                            }
                            Err(error) => error!("Error while decoding batch: {error}"),
                        },
                        Err(error) => error!("Error while consuming fluvio topic: {error}"),
                    }
                }
            }

            #[allow(unreachable_code)]
            Ok::<(), FeedbackFusionError>(())
        });

        Ok(())
    }
}
