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

use std::time::Duration;

use crate::{broker::FeedbackFusionBrokerDriver, prelude::*};
use feedback_fusion_common::event::{Event, EventBatch};
use kanal::AsyncReceiver;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

use crate::broker::FeedbackFusionBroker;

const BATCH_SIZE: usize = 10;
const BATCH_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn start_loop(
    mut broker: FeedbackFusionBroker,
    receiver: AsyncReceiver<Event>,
) -> Result<()> {
    let mut batch = Vec::with_capacity(BATCH_SIZE);

    loop {
        let first_event = match receiver.recv().await {
            Ok(event) => event,
            Err(_) => {
                info!("Channel closed. Sending final batch and shutting down.");
                break;
            }
        };
        batch.push(first_event);

        let timeout = tokio::time::sleep(BATCH_TIMEOUT);
        tokio::pin!(timeout);
        while batch.len() < BATCH_SIZE {
            tokio::select! {
                biased;

                result = receiver.recv() => {
                    match result {
                        Ok(event) => batch.push(event),
                        Err(_) => {
                            break;
                        }
                    }
                },

                _ = &mut timeout => {
                    break;
                }
            }
        }

        if !batch.is_empty() {
            debug!("Sending batch of {} events.", batch.len());
            if let Err(error) = send_batch(&mut broker.driver, &mut batch).await {
                error!("Failed to send batch: {error}");
            }
        }
    }

    if !batch.is_empty() {
        info!(
            "Sending final batch of {} events before shutdown.",
            batch.len()
        );
        if let Err(error) = send_batch(&mut broker.driver, &mut batch).await {
            error!("Failed to send final batch: {error}");
        }
    }

    Ok(())
}

async fn send_batch(
    driver: &mut Box<dyn FeedbackFusionBrokerDriver>,
    events: &mut Vec<Event>,
) -> Result<()> {
    if events.is_empty() {
        return Ok(());
    }

    let batch = EventBatch {
        events: events.to_vec(),
    };

    let retry_strategy = ExponentialBackoff::from_millis(100)
        .max_delay(Duration::from_secs(5))
        .map(jitter)
        .take(5);

    for delay in retry_strategy {
        match driver.send_batch(batch.clone()).await {
            Ok(_) => {
                events.clear();
                return Ok(());
            }
            Err(error) => {
                error!("Error occurred while sending batch: {error}");
                tokio::time::sleep(delay).await;
            }
        }
    }

    events.clear();

    Ok(())
}
