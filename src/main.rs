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
 *
 */

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate utoipa;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{BoxError, Router, Server};
use std::net::SocketAddr;
use std::time::Duration;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_global_rate_limit")]
    global_rate_limit: u64,
}

fn default_global_rate_limit() -> u64 {
    10
}

lazy_static! {
    pub static ref CONFIG: Config = envy::from_env::<Config>().unwrap();
}

#[tokio::main]
async fn main() {
    // init config
    lazy_static::initialize(&CONFIG);

    // init the tracing subscriber with the `RUST_LOG` env filter
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (sender, receiver) = kanal::oneshot_async::<()>();

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    tokio::spawn(async move {
        Server::bind(&address)
            .serve(router().into_make_service())
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

fn router() -> Router {
    Router::new().layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error!("Unhandled error occurred: {}", err),
                )
            }))
            .layer(BufferLayer::new(1024))
            // set the max requests per sec for all incoming calls
            .layer(RateLimitLayer::new(
                CONFIG.global_rate_limit.clone(),
                Duration::from_secs(1),
            ))
            .layer(TraceLayer::new_for_http()),
    )
}
