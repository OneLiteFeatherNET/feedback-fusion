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

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use feedback_fusion_common::proto::{
    feedback_fusion_v1_client::FeedbackFusionV1Client, field_options::Options,
    public_feedback_fusion_v1_client::PublicFeedbackFusionV1Client, response_data::Data,
    CheckboxOptions, CheckboxResponse, CheckboxStyle, CreateFieldRequest, CreatePromptRequest,
    CreateResponsesRequest, CreateTargetRequest, Field, FieldOptions, FieldType, GetPromptRequest,
    NumberOptions, NumberResponse, Prompt, RangeOptions, RangeResponse, RatingOptions,
    RatingResponse, ResponseData, SelectionOptions, SelectionResponse, Target, TextOptions,
    TextResponse,
};
use futures::StreamExt;
use nanoid::nanoid;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse, Scope,
};
use rand::{seq::IteratorRandom, thread_rng, Rng};
use tokio::runtime::Runtime;
use tonic::{metadata::MetadataValue, transport::Channel};

pub async fn authenticate() -> String {
    let issuer = IssuerUrl::new(std::env::var("OIDC_PROVIDER").unwrap()).unwrap();
    let metadata = CoreProviderMetadata::discover_async(issuer, async_http_client)
        .await
        .unwrap();
    let client = CoreClient::from_provider_metadata(
        metadata,
        ClientId::new(std::env::var("OIDC_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(
            std::env::var("OIDC_CLIENT_SECRET").unwrap(),
        )),
    );

    let token_response = client
        .exchange_client_credentials()
        .add_scope(Scope::new("api:feedback-fusion".to_owned()))
        .request_async(async_http_client)
        .await
        .unwrap();

    token_response.access_token().secret().clone()
}

fn create_random_field_request(prompt: String) -> CreateFieldRequest {
    let mut rng = rand::thread_rng();
    let (field_type, options) = match rng.gen_range(0..=5) {
        0 => (
            FieldType::Text,
            FieldOptions {
                options: Some(Options::Text(TextOptions {
                    placeholder: nanoid!(),
                    lines: rng.gen_range(1..=10),
                })),
            },
        ),
        1 => (
            FieldType::Rating,
            FieldOptions {
                options: Some(Options::Rating(RatingOptions {
                    max: rng.gen_range(1..=10),
                })),
            },
        ),
        2 => (
            FieldType::Checkbox,
            FieldOptions {
                options: Some(Options::Checkbox(CheckboxOptions {
                    style: if rng.gen_bool(0.5) {
                        CheckboxStyle::Normal
                    } else {
                        CheckboxStyle::Switch
                    }
                    .into(),
                    default_state: rng.gen_bool(0.5),
                })),
            },
        ),
        3 => (
            FieldType::Selection,
            FieldOptions {
                options: Some(Options::Selection(SelectionOptions {
                    values: vec![nanoid!(), nanoid!()],
                    multiple: rng.gen_bool(0.5),
                    combobox: rng.gen_bool(0.5),
                })),
            },
        ),
        4 => (
            FieldType::Range,
            FieldOptions {
                options: Some(Options::Range(RangeOptions {
                    min: rng.gen_range(0..=10),
                    max: rng.gen_range(10..=20),
                })),
            },
        ),
        _ => (
            FieldType::Number,
            FieldOptions {
                options: Some(Options::Number(NumberOptions {
                    min: rng.gen_range(0..=10),
                    max: rng.gen_range(10..=20),
                    placeholder: nanoid!(),
                })),
            },
        ),
    };

    CreateFieldRequest {
        prompt,
        title: nanoid!(),
        description: Some(nanoid!()),
        field_type: field_type.into(),
        options: Some(options),
    }
}

fn create_random_response_data(field_options: &FieldOptions) -> ResponseData {
    let mut rng = rand::thread_rng();
    match field_options.options.as_ref().unwrap() {
        Options::Text(_) => ResponseData {
            data: Some(Data::Text(TextResponse { text: nanoid!() })),
        },
        Options::Rating(options) => ResponseData {
            data: Some(Data::Rating(RatingResponse {
                rating: rng.gen_range(1..=options.max),
            })),
        },
        Options::Checkbox(_options) => ResponseData {
            data: Some(Data::Checkbox(CheckboxResponse {
                checked: rng.gen_bool(0.5),
            })),
        },
        Options::Selection(options) => ResponseData {
            data: Some(Data::Selection(SelectionResponse {
                values: if options.combobox {
                    if options.multiple {
                        vec![nanoid!(), nanoid!()]
                    } else {
                        vec![nanoid!()]
                    }
                } else {
                    if options.multiple {
                        options.values.clone()
                    } else {
                        vec![options.values[0].clone()]
                    }
                },
            })),
        },
        Options::Range(options) => ResponseData {
            data: Some(Data::Range(RangeResponse {
                start: rng.gen_range(options.min..=options.max),
                end: rng.gen_range(options.min..=options.max),
            })),
        },
        Options::Number(options) => ResponseData {
            data: Some(Data::Number(NumberResponse {
                number: rng.gen_range(options.min..=options.max),
            })),
        },
    }
}
async fn setup_benchmark() -> (Target, Vec<Prompt>, Vec<(Prompt, Vec<Field>)>) {
    let channel = Channel::from_shared(std::env::var("GRPC_ENDPOINT").unwrap())
        .unwrap()
        .connect()
        .await
        .unwrap();
    let token: MetadataValue<_> = format!("Bearer {}", authenticate().await).parse().unwrap();
    let mut client =
        FeedbackFusionV1Client::with_interceptor(channel, move |mut request: tonic::Request<()>| {
            request
                .metadata_mut()
                .insert("authorization", token.clone());

            Ok(request)
        });

    let target = client
        .create_target(CreateTargetRequest {
            name: "Benchmark".to_owned(),
            description: None,
        })
        .await
        .unwrap()
        .into_inner();
    let target_id = &target.id;

    let inactive_prompts = futures::stream::iter(1..=5)
        .then(|_| {
            let mut client = client.clone();

            async move {
                client
                    .create_prompt(CreatePromptRequest {
                        target: target_id.to_string(),
                        active: false,
                        title: nanoid!(),
                        description: nanoid!(),
                    })
                    .await
                    .unwrap()
                    .into_inner()
            }
        })
        .collect()
        .await;

    let prompts = futures::stream::iter(1..=5)
        .then(|_| {
            let mut client = client.clone();

            async move {
                let prompt = client
                    .create_prompt(CreatePromptRequest {
                        target: target_id.to_string(),
                        active: false,
                        title: nanoid!(),
                        description: nanoid!(),
                    })
                    .await
                    .unwrap()
                    .into_inner();

                let fields = futures::stream::iter(1..=10)
                    .then(|_| {
                        let mut client = client.clone();
                        let id = prompt.id.clone();

                        async move {
                            client
                                .create_field(create_random_field_request(id))
                                .await
                                .unwrap()
                                .into_inner()
                        }
                    })
                    .collect()
                    .await;

                (prompt, fields)
            }
        })
        .collect()
        .await;

    (target, inactive_prompts, prompts)
}

fn benchmark_endpoints(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();

    let (_target, inactive_prompts, prompts) = runtime.block_on(setup_benchmark());
    let mut public_client = runtime
        .block_on(PublicFeedbackFusionV1Client::connect(
            std::env::var("GRPC_ENDPOINT").unwrap(),
        ))
        .unwrap();

    c.bench_function("Get Prompt with invalid id", |b| {
        b.iter(|| {
            runtime
                .block_on(public_client.get_prompt(GetPromptRequest::default()))
                .ok()
        })
    });

    c.bench_function("Get Prompt", |b| {
        b.iter_batched(
            || {
                let active_prompts = prompts
                    .clone()
                    .into_iter()
                    .map(|(k, _)| k)
                    .collect::<Vec<Prompt>>();
                let mut prompts = Vec::with_capacity(active_prompts.len() + inactive_prompts.len());
                prompts.extend(active_prompts);
                prompts.extend(inactive_prompts.clone());

                let mut rng = thread_rng();
                prompts.into_iter().choose(&mut rng).unwrap()
            },
            |prompt| {
                runtime
                    .block_on(public_client.get_prompt(GetPromptRequest { id: prompt.id }))
                    .unwrap()
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("Create Responses", |b| {
        b.iter_batched(
            || {
                let mut rng = thread_rng();
                let (prompt, fields) = prompts.clone().into_iter().choose(&mut rng).unwrap();

                CreateResponsesRequest {
                    prompt: prompt.id,
                    data: fields
                        .into_iter()
                        .map(|field| {
                            (
                                field.id,
                                create_random_response_data(&field.options.unwrap()),
                            )
                        })
                        .collect(),
                }
            },
            |request| {
                runtime
                    .block_on(public_client.create_responses(request))
                    .unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_endpoints);
criterion_main!(benches);
