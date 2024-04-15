//SPDX-FileCopyrightText: 2023 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2023 OneLiteFeatherNet

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

use crate::{
    database::schema::feedback::*,
    routes::v1::{prompt::*, response::*, *},
};
use std::{fs, path::Path};
use utoipa::{
    openapi::security::{OpenIdConnect, SecurityScheme},
    Modify, OpenApi, ToSchema,
};

#[derive(ToSchema)]
#[aliases(
    FeedbackTargetPage = Page<FeedbackTarget>,
    FeedbackPromptPage = Page<FeedbackPrompt>,
    FeedbackPromptFieldPage = Page<FeedbackPromptField>

)]
#[allow(unused)]
pub struct Page<T: for<'a> ToSchema<'a>> {
    records: Vec<T>,
    total: u16,
    page_no: u16,
}

pub mod config;
pub mod database;
pub mod error;
pub mod prelude;
pub mod routes;
pub mod state;

pub fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            post_target,
            get_targets,
            put_target,
            delete_target,
            post_prompt,
            get_prompt,
            get_prompts,
            put_prompt,
            delete_prompt,
            post_field,
            put_field,
            get_fields,
            delete_field,
            post_response,
            get_responses,
            fetch
        ),
        components(
            schemas(
                FeedbackTarget,
                PutFeedbackTargetRequest,
                FeedbackPrompt,
                PutFeedbackPromptRequest,
                FeedbackPromptField,
                PutFeedbackPromptFieldRequest,
                FeedbackPromptInputType,
                FeedbackPromptField,
                FeedbackPromptInputOptions,
                TextOptions,
                RatingOptions,
                CheckboxOptions,
                CheckboxStyle,
                SelectionOptions,
                RangeOptions,
                NumberOptions,
                FeedbackPromptResponse,
                FeedbackPromptFieldResponse,
                FeedbackPromptFieldData,
                CreateFeedbackTargetRequest,
                CreateFeedbackPromptRequest,
                CreateFeedbackPromptFieldRequest,
                FeedbackTargetPage,
                FeedbackPromptPage,
                FeedbackPromptFieldPage,
                GetFeedbackPromptResponsesResponseWrapper,
                SubmitFeedbackPromptResponseRequest,
                TextResponse,
                RatingResponse,
                CheckboxResponse,
                SelectionResponse,
                RangeResponse,
                NumberResponse
            )
        ),
        tags(
            (name = "FeedbackTarget", description = "A Target contains multiple prompts and is therefore used in order to manage multiple projects with the same instance."),
            (name = "FeedbackTargetPrompt", description = "A Prompt contains multiple fields and collects the feedback for your project."),
            (name = "FeedbackTargetPromptField", description = "A Field is a input prompt for the clients visiting ur website and rating ur project or whatever."),
            (name = "FeedbackPromptResponse", description = "Collect responses from clients and manage / view them"), 
        ),
        modifiers(&Security)
    )]
    struct OpenApiSpecification;
    struct Security;

    impl Modify for Security {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "oidc",
                    SecurityScheme::OpenIdConnect(OpenIdConnect::new(
                        "https://your-oidc-provider.tld",
                    )),
                )
            }
        }
    }

    let destination = Path::new("./target").join("openapi.yaml");
    // write the spec file
    fs::write(
        destination,
        OpenApiSpecification::openapi().to_yaml().unwrap(),
    )
    .unwrap();
}
