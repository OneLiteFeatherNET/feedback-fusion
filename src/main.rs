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

use std::{path::PathBuf, str::FromStr};

use crate::{
    prelude::*,
    services::v1::{FeedbackFusionV1Context, PublicFeedbackFusionV1Context},
};
use aliri_tower::Oauth2Authorizer;
use feedback_fusion_common::proto::{
    feedback_fusion_v1_server::FeedbackFusionV1Server,
    public_feedback_fusion_v1_server::PublicFeedbackFusionV1Server,
};
use futures::stream::StreamExt;
use notify::{RecommendedWatcher, Watcher};
#[cfg(feature = "otlp")]
use opentelemetry::global::shutdown_tracer_provider;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
#[cfg(not(feature = "otlp"))]
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "caching-skytable")]
pub mod cache;
pub mod config;
pub mod database;
pub mod error;
#[cfg(feature = "otlp")]
pub mod otlp;
pub mod services;

const ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() {
    // init config
    lazy_static::initialize(&CONFIG);
    // init the tracing subscriber with the `RUST_LOG` env filter
    if CONFIG.otlp_endpoint().is_none() {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    #[cfg(feature = "otlp")]
    otlp::init_tracing();

    debug!("Reading Databaseconfig");
    lazy_static::initialize(&DATABASE_CONFIG);

    // connect to the database
    debug!("Connecting to the Database");
    let connection = DATABASE_CONFIG.connect().await.unwrap();
    let connection = DatabaseConnection::from(connection);
    info!("Connection to the Database established");

    // start config file watcher
    if let Some(config_path) = CONFIG.config_path().as_ref() {
        let connection = connection.clone();
        info!("CONFIG_PATH present, starting watcher");
        // initial load
        match config::sync_config(&connection).await {
            Ok(_) => info!("Config reloaded"),
            Err(error) => error!("Error occurred while syncinc config: {error}"),
        };

        tokio::spawn(async move {
            let (sender, receiver) = kanal::bounded_async(1);

            let mut watcher = RecommendedWatcher::new(
                move |response| {
                    let sender = sender.clone();
                    tokio::spawn(async move { sender.send(response).await.unwrap() });
                },
                notify::Config::default(),
            )
            .unwrap();

            watcher
                .watch(
                    &PathBuf::from_str(config_path.as_str()).unwrap(),
                    notify::RecursiveMode::NonRecursive,
                )
                .unwrap();
            info!("Watching for changes at {config_path}");

            let mut stream = receiver.stream();
            while let Some(response) = stream.next().await {
                match response {
                    Ok(_) => {
                        info!("Detected config chage");

                        match config::sync_config(&connection).await {
                            Ok(_) => info!("Config reloaded"),
                            Err(error) => error!("Error occurred while syncinc config: {error}"),
                        }
                    }
                    Err(error) => error!("Error occurred during watch: {error}"),
                }
            }

            Ok::<(), FeedbackFusionError>(())
        });
    } else {
        warn!("CONFIG_PATH not set, wont start watcher");
    }

    let (sender, receiver) = kanal::oneshot_async::<()>();

    tokio::spawn(async move {
        debug!("Constructing health reporter");
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<PublicFeedbackFusionV1Server<PublicFeedbackFusionV1Context>>()
            .await;

        debug!("Constructing reflection service");
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(
                feedback_fusion_common::proto::FILE_DESCRIPTOR_SET,
            )
            .build()
            .unwrap();

        // build the authority
        info!("Tryng to contact the OIDC Provider");
        let authority = oidc::authority().await.unwrap();
        let authorizer = Oauth2Authorizer::new()
            .with_claims::<OIDCClaims>()
            .with_verbose_error_handler();

        let service = FeedbackFusionV1Context {
            connection: connection.clone(),
        };
        let service = tower::ServiceBuilder::new()
            .layer(authorizer.jwt_layer(authority))
            .service(FeedbackFusionV1Server::new(service));

        let public_service = PublicFeedbackFusionV1Context { connection };
        let public_service = PublicFeedbackFusionV1Server::new(public_service);

        info!("Listening for incoming requests on {ADDRESS}");

        #[cfg(not(feature = "otlp"))]
        let trace_layer = TraceLayer::new_for_grpc();
        #[cfg(feature = "otlp")]
        let trace_layer = otlp::trace_layer();

        Server::builder()
            .layer(trace_layer)
            .accept_http1(true)
            .layer(GrpcWebLayer::new())
            .add_service(health_service)
            .add_service(reflection_service)
            .add_service(AuthorizedService(service))
            .add_service(public_service)
            .serve_with_shutdown(ADDRESS.parse().unwrap(), async move {
                receiver.recv().await.ok();
            })
            .await
            .unwrap();
    });

    debug!("Trying to listen for graceful shutdown");
    match tokio::signal::ctrl_c().await {
        Ok(()) => {}
        Err(error) => {
            error!("Unable to listen for the shutdown signal: {}", error);
        }
    }

    info!("Received shutdown signal... shutting down...");
    sender.send(()).await.unwrap();
    #[cfg(feature = "otlp")]
    shutdown_tracer_provider()
}

pub mod prelude {
    pub use crate::{
        config::*,
        database::{DatabaseConfiguration, DatabaseConnection},
        database_request,
        error::*,
        impl_select_page_wrapper,
        services::{oidc::*, *},
    };
    pub use derivative::Derivative;
    pub use feedback_fusion_common::IntoPageRequest;
    pub use getset::{Getters, MutGetters, Setters};
    pub use itertools::Itertools;
    pub use lazy_static::lazy_static;
    pub use paste::paste;
    pub use rbatis::{
        crud, impl_insert, impl_select, impl_select_page, impled, plugin::page::Page, py_sql,
        rbdc::JsonV, IPageRequest,
    };
    pub use serde::{Deserialize, Serialize};
    pub use tonic::{Request, Response};
    pub use tracing::{debug, error, info, info_span, instrument, warn, Instrument};
    pub use typed_builder::TypedBuilder;
    pub use validator::Validate;
}
