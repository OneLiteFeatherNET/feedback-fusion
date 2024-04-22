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
use feedback_fusion_common::proto::{
    feedback_fusion_v1_server::FeedbackFusionV1,
    public_feedback_fusion_v1_server::{PublicFeedbackFusionV1, PublicFeedbackFusionV1Server},
};
use std::time::Duration;
use tonic::transport::Server;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod config;
pub mod database;
pub mod error;
pub mod services;
pub mod state;

const ADDRESS: &'static str = "[::1]:8000";

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

    // connect to the database
    let connection = DATABASE_CONFIG.connect().await.unwrap();
    let connection = DatabaseConnection::from(connection);

    tokio::spawn(async move {
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<PublicFeedbackFusionV1Server<PublicFeedbackFusionV1>>()
            .await;

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(
                feedback_fusion_common::proto::FILE_DESCRIPTOR_SET,
            )
            .build()
            .unwrap();

        let service = FeedbackFusionV1::default();
        let service = tower::ServiceBuilder::new()
            .layer(tower_http::trace::TraceLayer::new_for_grpc())
            .service(service)
            .into_inner();

        let public_service = PublicFeedbackFusionV1::default();
        let public_service = tower::ServiceBuilder::new()
            .layer(tower_http::trace::TraceLayer::new_for_grpc())
            .service(public_service)
            .into_inner();

        Server::builder()
            .add_server(health_service)
            .add_service(reflection_service)
            .add_service(service)
            .add_service(public_service)
            .serve_with_shutdown(ADDRESS.parse().unwrap(), async move {
                receiver.recv().await.ok();
            })
            .await
            .unwrap();
    });
    info!("Listening for incoming requests on {ADDRESS}");

    match tokio::signal::ctrl_c().await {
        Ok(()) => {}
        Err(error) => {
            error!("Unable to listen for the shutdown signal: {}", error);
        }
    }

    info!("Received shutdown signal... shutting down...");
    sender.send(()).await.unwrap();
}

pub mod prelude {
    pub use crate::{
        config::*,
        database::{DatabaseConfiguration, DatabaseConnection},
        database_request,
        error::*,
        impl_select_page_wrapper,
        routes::{oidc::*, *},
        state::FeedbackFusionState,
    };
    pub use derivative::Derivative;
    pub use getset::{Getters, MutGetters, Setters};
    pub use lazy_static::lazy_static;
    pub use paste::paste;
    pub use rbatis::{
        crud, impl_insert, impl_select, impl_select_page, impled, plugin::page::Page, py_sql,
        rbdc::JsonV, IPageRequest,
    };
    pub use serde::{Deserialize, Serialize};
    pub use tonic::{Request, Response};
    pub use tracing::{debug, error, info, info_span, warn};
    pub use typed_builder::TypedBuilder;
    pub use validator::Validate;
}
