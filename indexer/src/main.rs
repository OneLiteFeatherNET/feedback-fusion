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

#![allow(clippy::too_many_arguments)]

use crate::{
    broker::FeedbackFusionIndexerBroker,
    config::{CONFIG, DATABASE_CONFIG},
    prelude::*,
    processor::FeedbackFusionIndexerProcessor,
};
use feedback_fusion_common::database::DatabaseConnection;

mod broker;
mod config;
mod error;
mod processor;

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
    lazy_static::initialize(&CONFIG);

    feedback_fusion_common::observability::otlp::init_tracing(CONFIG.otlp());

    // initialize the shutdown channel
    let (shutdown_sender, shutdown_receiver) = kanal::unbounded_async::<()>();

    // start the broker driver
    let mut broker = FeedbackFusionIndexerBroker::initialize().await.unwrap();
    let broker_event_batch_receiver = broker
        .start_loop(shutdown_sender.clone(), shutdown_receiver.clone())
        .await
        .unwrap();

    // connect to the database
    debug!("Connecting to the Database");
    let connection = DATABASE_CONFIG.connect().await.unwrap();
    let connection = DatabaseConnection::from(connection);
    info!("Connection to the Database established");

    // start the processor worker
    let processor =
        FeedbackFusionIndexerProcessor::initialize(connection, broker_event_batch_receiver);
    processor
        .start_worker(shutdown_sender.clone(), shutdown_receiver)
        .await;

    debug!("Trying to listen for graceful shutdown");
    match tokio::signal::ctrl_c().await {
        Ok(()) => {}
        Err(error) => {
            error!("Unable to listen for the shutdown signal: {}", error);
        }
    }

    info!("Received shutdown signal... shutting down...");
    shutdown_sender.send(()).await.unwrap();

    feedback_fusion_common::observability::otlp::shutdown_tracing();
}

pub mod prelude {
    pub use crate::error::*;
    pub use async_trait::async_trait;
    pub use feedback_fusion_common::prelude::*;
}
