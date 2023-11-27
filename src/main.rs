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

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate delegate;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nanoid;
#[macro_use]
extern crate paste;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate typed_builder;
#[macro_use]
extern crate utoipa;

use crate::{config::Config, database::{DatabaseConfiguration, DatabaseConnection}, state::FeedbackFusionState};
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router, Server};
use std::{net::SocketAddr, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

lazy_static! {
    pub static ref CONFIG: Config = envy::from_env::<Config>().unwrap();
    pub static ref DATABASE_CONFIG: DatabaseConfiguration =
        DatabaseConfiguration::extract().unwrap();
}

pub mod auth;
pub mod config;
pub mod database;
pub mod error;
pub mod state;

#[cfg(test)]
pub mod tests;

#[tokio::main]
async fn main() {
    // init config
    lazy_static::initialize(&CONFIG);
    lazy_static::initialize(&DATABASE_CONFIG);

    // init the tracing subscriber with the `RUST_LOG` env filter
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (sender, receiver) = kanal::oneshot_async::<()>();
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));

    // connect to the database
    let connection = DATABASE_CONFIG.connect().await.unwrap();

    tokio::spawn(async move {
        Server::bind(&address)
            .serve(router(connection).into_make_service())
            .with_graceful_shutdown(async move {
                receiver.recv().await.ok();
            })
            .await
            .unwrap();
    });

    match tokio::signal::ctrl_c().await {
        Ok(()) => {}
        Err(error) => {
            error!("Unable to listen for the shutdown signal: {}", error);
        }
    }

    info!("Received shutdown signal... shutting down...");
    sender.send(()).await.unwrap();
}

fn router(connection: DatabaseConnection) -> Router {
    Router::new()
        .layer(
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
                    CONFIG.global_rate_limit().clone(),
                    Duration::from_secs(1),
                ))
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(FeedbackFusionState::new(connection))
}

pub mod prelude {
    pub use crate::{config::*, error::*, CONFIG, DATABASE_CONFIG};
    pub use axum::extract::{Json, Query};
}

