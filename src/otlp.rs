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
use opentelemetry::{global, KeyValue};
use opentelemetry_http::{HeaderExtractor, Request};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, Resource};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tower_http::{
    classify::{GrpcErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, TraceLayer},
};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::layer::SubscriberExt;

pub fn init_tracing() {
    if let Some(endpoint) = CONFIG.otlp_endpoint() {
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
                    CONFIG.service_name().clone(),
                )]),
            ))
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap();

        let layer = tracing_opentelemetry::layer().with_tracer(tracer_provider);
        let subscriber = subscriber.with(layer);
        tracing::subscriber::set_global_default(subscriber).ok();
    }
}

#[derive(Clone)]
pub struct MakeFeedbackFusionSpan;

impl<B> MakeSpan<B> for MakeFeedbackFusionSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        // make the span
        let span = tracing::info_span!(
            "gRPC Request",
            host = ?request.uri().host(),
            path = %request.uri().path(),
            version = ?request.version()
        );

        // try to extract the context
        let context = global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor(request.headers()))
        });
        span.set_parent(context);

        span
    }
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<GrpcErrorsAsFailures>, MakeFeedbackFusionSpan> {
    TraceLayer::new_for_grpc().make_span_with(MakeFeedbackFusionSpan)
}
