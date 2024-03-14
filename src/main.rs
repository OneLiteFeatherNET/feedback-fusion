/*
 * Copyright (c) 2023 OneLiteFeatherNET
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#![allow(clippy::too_many_arguments)]

use crate::prelude::*;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Server};
use std::{net::SocketAddr, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod config;
pub mod database;
pub mod error;
pub mod prelude;
pub mod routes;
pub mod state;

#[tokio::main]
async fn main() {
    // init the tracing subscriber with the `RUST_LOG` env filter
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // init config
    lazy_static::initialize(&CONFIG);
    lazy_static::initialize(&DATABASE_CONFIG);

    let (sender, receiver) = kanal::oneshot_async::<()>();
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));

    // connect to the database
    let connection = DATABASE_CONFIG.connect().await.unwrap();
    let connection = DatabaseConnection::from(connection);

    tokio::spawn(async move {
        Server::bind(&address)
            .serve(router(connection).await.into_make_service())
            .with_graceful_shutdown(async move {
                receiver.recv().await.ok();
            })
            .await
            .unwrap();
    });
    info!("Listening for incoming requests");

    match tokio::signal::ctrl_c().await {
        Ok(()) => {}
        Err(error) => {
            error!("Unable to listen for the shutdown signal: {}", error);
        }
    }

    info!("Received shutdown signal... shutting down...");
    sender.send(()).await.unwrap();
}

async fn router(connection: DatabaseConnection) -> Router {
    let state = FeedbackFusionState::new(connection);

    routes::router(state).await.layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error!("Unhandled error occurred: {}", error),
                )
            }))
            .layer(BufferLayer::new(1024))
            // set the max requests per sec for all incoming calls
            .layer(RateLimitLayer::new(
                *CONFIG.global_rate_limit(),
                Duration::from_secs(1),
            ))
            .layer(TraceLayer::new_for_http()),
    )
}
