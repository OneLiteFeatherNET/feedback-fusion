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

use crate::prelude::*;
use feedback_fusion_common::{database::DatabaseConnection, event::EventBatch};
use kanal::{AsyncReceiver, AsyncSender};

mod audit;

pub struct FeedbackFusionIndexerProcessor {
    connection: DatabaseConnection,
    receiver: AsyncReceiver<EventBatch>,
}

impl FeedbackFusionIndexerProcessor {
    pub fn initialize(connection: DatabaseConnection, receiver: AsyncReceiver<EventBatch>) -> Self {
        Self {
            connection,
            receiver,
        }
    }

    pub async fn start_worker(
        self,
        shutdown_sender: AsyncSender<()>,
        shutdown_receiver: AsyncReceiver<()>,
    ) {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    event = shutdown_receiver.recv() => {
                       match event {
                            Ok(()) => {
                                info!("Got shutdown signal, stopping processor worker");

                                break;
                            },
                            Err(error) => {
                                error!("Got an error while listening for shutdown signal on processor worker: {error}");
                                error!("Sending shutdown signal to other receivers");

                                shutdown_sender.send(()).await.ok();
                                break;
                            }
                        }
                    }

                    event = self.receiver.recv() => {
                        match event {
                            Ok(batch) => {
                                self.process(batch).await?;
                            }
                            Err(error) => {
                                error!("Got an error while listening for event batches on processor worker: {error}");
                                error!("Sending shutdown signal to other receivers");

                                shutdown_sender.send(()).await.ok();
                                break;
                            }
                        }
                    }
                };
            }

            Ok::<(), FeedbackFusionError>(())
        });
    }

    #[instrument(skip_all)]
    async fn process(&self, batch: EventBatch) -> Result<()> {
        debug!("Processing {} events", batch.events.len());

        let grouped_by_type = batch
            .events
            .into_iter()
            .sorted_by(|x, y| x.event_type.cmp(&y.event_type))
            .chunk_by(|event| event.event_type);

        // collect the chunks in order to make them sendable to another thread
        let grouped_by_type = grouped_by_type
            .into_iter()
            .map(|(key, chunk)| (key, chunk.collect_vec()))
            .collect_vec();

        for (event_type, events) in grouped_by_type {
            #[allow(clippy::single_match)]
            match event_type {
                // RESOURCE_MODIFIED
                1 => {
                    debug!(
                        "Got {} RESOURCE_MODIFIED events in current batch",
                        events.len()
                    );
                    // create the audit version
                    audit::create_audit_versions(events.as_slice(), &self.connection).await?;
                }
                // UNKNOWN
                _ => {}
            }
        }

        Ok(())
    }
}
