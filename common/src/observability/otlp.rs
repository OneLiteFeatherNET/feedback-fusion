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

use crate::{observability::OTLPConfiguration, prelude::*};
use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_http::{HeaderExtractor, Request};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    trace::{BatchSpanProcessor, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tower_http::{
    classify::{GrpcErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, TraceLayer},
};
use tracing::Span;
use tracing_opentelemetry::{OpenTelemetryLayer, OpenTelemetrySpanExt};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

lazy_static::lazy_static! {
    static ref HEADERS_TO_KEEP: Vec<&'static str> =
        vec!["traceparent", "x-request-id", "user-agent"];
}


pub fn init_tracing(config: &Option<OTLPConfiguration>) -> Option<SdkTracerProvider> {
    if let Some(config) = config {
        let endpoint = config.endpoint();

        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

        let exporter = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()
            .unwrap();

        let resource = Resource::builder()
            .with_attribute(KeyValue::new(SERVICE_NAME, config.service_name().clone()))
            .build();

        let tracer_provider = SdkTracerProvider::builder()
            .with_span_processor(BatchSpanProcessor::builder(exporter).build())
            .with_resource(resource)
            .build();

        let tracer = tracer_provider.tracer("feedback-fusion");

        global::set_tracer_provider(tracer_provider.clone());

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer())
            .with(OpenTelemetryLayer::new(tracer))
            .init();

        info!("Initiating tracing with collector {}", endpoint);

        Some(tracer_provider)
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer())
            .init();

        None
    }
}


#[derive(Clone)]
pub struct MakeFeedbackFusionSpan;

impl<B> MakeSpan<B> for MakeFeedbackFusionSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        if request.uri().path().contains("grpc.health.v1.Health/Check") {
            tracing::debug_span!("HealthCheck")
        } else {
            let headers = request
                .headers()
                .iter()
                .filter(|(key, _)| HEADERS_TO_KEEP.contains(&key.as_str()))
                .collect_vec();

            let span = tracing::info_span!(
                "gRPC Request",
                host = %request.uri().host().unwrap_or_default(),
                path = %request.uri().path(),
                headers = ?headers,
                version = ?request.version(),
                request_id = nanoid::nanoid!()
            );

            // try to extract the context
            let context = global::get_text_map_propagator(|propagator| {
                propagator.extract(&HeaderExtractor(request.headers()))
            });
            span.set_parent(context).ok();

            span
        }
    }
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<GrpcErrorsAsFailures>, MakeFeedbackFusionSpan> {
    TraceLayer::new_for_grpc().make_span_with(MakeFeedbackFusionSpan)
}
