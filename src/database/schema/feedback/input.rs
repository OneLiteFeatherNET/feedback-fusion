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

use crate::prelude::*;
use rbatis::rbdc::{DateTime, JsonV};

use super::FeedbackPromptInputType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, ToSchema)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(TS))]
pub enum FeedbackPromptInputOptions {
    Text(TextOptions),
    Rating(RatingOptions),
}

// TODO: gen with macro
impl PartialEq<FeedbackPromptInputOptions> for FeedbackPromptInputType {
    fn eq(&self, other: &FeedbackPromptInputOptions) -> bool {
        match self {
            Self::Text => matches!(other, FeedbackPromptInputOptions::Text(_)),
            Self::Rating => matches!(other, FeedbackPromptInputOptions::Rating(_)),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct TextOptions {
    #[validate(length(max = 255))]
    description: String,
    #[validate(length(max = 255))]
    placeholder: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct RatingOptions {
    #[validate(length(max = 255))]
    description: String,
    max: u8,
}

#[derive(
    Deserialize, Serialize, Clone, Derivative, Debug, Getters, MutGetters, TypedBuilder, ToSchema,
)]
#[derivative(PartialEq)]
#[get = "pub"]
#[get_mut = "pub"]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct FeedbackPromptResponse {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    prompt: String,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    #[cfg_attr(feature = "bindings", ts(type = "Date"))]
    created_at: DateTime,
}

crud!(FeedbackPromptResponse {});
impl_select_page_wrapper!(FeedbackPromptResponse {select_page_by_prompt(prompt: &str) => "WHERE prompt = #{prompt}"});

#[derive(
    Deserialize, Serialize, Clone, PartialEq, Debug, Getters, MutGetters, TypedBuilder, ToSchema,
)]
#[get = "pub"]
#[get_mut = "pub"]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct FeedbackPromptFieldResponse {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    response: String,
    field: String,
    #[cfg(not(feature = "bindings"))]
    #[schema(value_type = FeedbackPromptFieldData)]
    data: JsonV<FeedbackPromptFieldData>,
    #[cfg(feature = "bindings")]
    data: FeedbackPromptFieldData
}

crud!(FeedbackPromptFieldResponse {});

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub enum FeedbackPromptFieldData {
    Text(TextResponse),
    Rating(RatingResponse),
}

// TODO: use macro
impl PartialEq<FeedbackPromptFieldData> for FeedbackPromptInputType {
    fn eq(&self, other: &FeedbackPromptFieldData) -> bool {
        match self {
            Self::Text => matches!(other, FeedbackPromptFieldData::Text(_)),
            Self::Rating => matches!(other, FeedbackPromptFieldData::Rating(_)),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct TextResponse {
    data: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct RatingResponse {
    data: u8,
}
