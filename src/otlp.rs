//SPDX-FileCopyrightText: 2024 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2024 OneLiteFeatherNet

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
use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_http::{HeaderExtractor, Request};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, Resource};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tower_http::{
    classify::{GrpcErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, TraceLayer},
};
use tracing::Span;
use tracing_opentelemetry::{OpenTelemetryLayer, OpenTelemetrySpanExt};
use tracing_subscriber::layer::SubscriberExt;

lazy_static! {
    static ref HEADERS_TO_KEEP: Vec<&'static str> = vec!["traceparent", "x-request-id", "user-agent"];
}

pub fn init_tracing() {
    if let Some(config) = CONFIG.otlp() {
        let endpoint = config.endpoint();

        let subscriber = tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer());

        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

        let tracer_provider = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(endpoint),
            )
            .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
                Resource::new(vec![KeyValue::new(
                    SERVICE_NAME,
                    config.service_name().clone(),
                )]),
            ))
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap();

        let subscriber = subscriber.with(OpenTelemetryLayer::new(
            tracer_provider.tracer("feedback-fusion"),
        ));
        tracing::subscriber::set_global_default(subscriber).ok();

        info!("Initiating tracing with collector {}", endpoint);
    }
}

#[derive(Clone)]
pub struct MakeFeedbackFusionSpan;

impl<B> MakeSpan<B> for MakeFeedbackFusionSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let span = if request.uri().path().contains("grpc.health.v1.Health/Check") {
            tracing::debug_span!("HealthCheck")
        } else {
            let headers = request
                .headers()
                .iter()
                .filter(|(key, _)| {
                    HEADERS_TO_KEEP.contains(&key.as_str()) 
                })
                .collect_vec();

            let span = tracing::info_span!(
                "gRPC Request",
                host = %request.uri().host().unwrap_or_default(),
                path = %request.uri().path(),
                headers = ?headers,
                version = ?request.version()
            );

            // try to extract the context
            let context = global::get_text_map_propagator(|propagator| {
                propagator.extract(&HeaderExtractor(request.headers()))
            });
            span.set_parent(context);

            span
        };

        span
    }
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<GrpcErrorsAsFailures>, MakeFeedbackFusionSpan> {
    TraceLayer::new_for_grpc().make_span_with(MakeFeedbackFusionSpan)
}
