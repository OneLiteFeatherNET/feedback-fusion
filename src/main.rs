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

use std::time::Duration;

use crate::{
    authorization::oidc::layer::{AuthorizedService, OIDCErrorHandler},
    broker::FeedbackFusionBroker,
    prelude::*,
    services::v1::{FeedbackFusionV1Context, PublicFeedbackFusionV1Context},
};
use aliri_tower::Oauth2Authorizer;
use feedback_fusion_common::proto::{
    feedback_fusion_v1_server::FeedbackFusionV1Server,
    public_feedback_fusion_v1_server::PublicFeedbackFusionV1Server,
};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod authorization;
pub mod broker;
pub mod cache;
pub mod config;
pub mod database;
pub mod error;
pub mod services;

const ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() {
    // init config
    lazy_static::initialize(&CONFIG);

    // init the tracing subscriber with the `RUST_LOG` env filter if otlp is disabled
    feedback_fusion_common::observability::otlp::init_tracing(CONFIG.otlp());

    debug!("Reading DatabaseConfig");
    lazy_static::initialize(&DATABASE_CONFIG);

    // connect to the database
    debug!("Connecting to the Database");
    let connection = DATABASE_CONFIG.connect().await.unwrap();
    let connection = DatabaseConnection::from(connection);
    info!("Connection to the Database established");

    // start the broker driver
    let broker = FeedbackFusionBroker::initialize().await.unwrap();
    let broker_event_sender = broker.start_loop(connection.clone()).await.unwrap();

    // sync the presets
    config::sync_preset(&connection).await.unwrap();
    let (sender, receiver) = kanal::unbounded_async::<()>();
    tokio::spawn(async move {
        debug!("Constructing health reporter");
        let (health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<PublicFeedbackFusionV1Server<PublicFeedbackFusionV1Context>>()
            .await;

        debug!("Constructing reflection service");
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(
                feedback_fusion_common::proto::FILE_DESCRIPTOR_SET,
            )
            .build_v1()
            .unwrap();

        // build the authority
        info!("Tryng to contact the OIDC Provider");
        let (authority, client) = crate::authorization::oidc::authority().await.unwrap();
        authority.spawn_refresh(Duration::from_secs(60 * 60 * 6));
        let authorizer = Oauth2Authorizer::new()
            .with_claims::<OIDCClaims>()
            .with_error_handler(OIDCErrorHandler::from(authority.clone()));

        let service = FeedbackFusionV1Context {
            connection: connection.clone(),
            client,
            permission_matrix: config::read_permission_matrix(
                CONFIG.oidc().scopes(),
                CONFIG.oidc().groups(),
            ),
            broker_event_sender,
        };
        let service = tower::ServiceBuilder::new()
            .layer(authorizer.jwt_layer(authority))
            .service(FeedbackFusionV1Server::new(service));

        let public_service = PublicFeedbackFusionV1Context { connection };
        let public_service = PublicFeedbackFusionV1Server::new(public_service);

        info!("Listening for incoming requests on {ADDRESS}");

        let trace_layer = observability::otlp::trace_layer();

        Server::builder()
            .layer(trace_layer)
            .accept_http1(true)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
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

    feedback_fusion_common::observability::otlp::shutdown_tracing();
}

pub mod prelude {
    pub use crate::{
        authorization::{endpoint::*, oidc::claims::*},
        cache::*,
        config::*,
        error::*,
        invalidate,
        services::*,
    };
    pub use bincode::{Decode, Encode};
    pub use cached::{IOCachedAsync, proc_macro::*};
    pub use derivative::Derivative;
    pub use feedback_fusion_codegen::dynamic_cache;
    pub use feedback_fusion_common::{database::*, prelude::*, *};
    pub use getset::{Getters, MutGetters, Setters};
    pub use lazy_static::lazy_static;
    pub use rayon::prelude::*;
    pub use rbatis::{
        IPageRequest, crud, impl_insert, impl_select, impl_select_page, impled, plugin::page::Page,
        py_sql, rbdc::JsonV,
    };
    pub use tonic::{Request, Response};
    pub use typed_builder::TypedBuilder;
    pub use validator::Validate;
}
