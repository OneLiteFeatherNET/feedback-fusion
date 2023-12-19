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

use rbatis::rbdc::DateTime;

use super::input::FeedbackPromptInputOptions;

#[derive(Deserialize, Serialize, Clone, Derivative, Debug, Getters, MutGetters, TypedBuilder, ToSchema, Validate)]
#[derivative(PartialEq)]
#[get = "pub"]
#[get_mut = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct FeedbackPrompt {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    #[validate(length(max = 255))]
    title: String,
    target: String,
    #[builder(default = true)]
    active: bool,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    created_at: DateTime,
}

crud!(FeedbackPrompt {});
impl_select_page!(FeedbackPrompt {select_page_by_target(target: &str) => "`WHERE target = #{target}`"});

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum FeedbackPromptInputType {
    Text,
    Rating,
}

#[derive(Deserialize, Serialize, Clone, Derivative, Debug, Getters, MutGetters, TypedBuilder, ToSchema, Validate)]
#[derivative(PartialEq)]
#[get = "pub"]
#[get_mut = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct FeedbackPromptField {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    #[validate(length(max = 255))]
    title: String,
    prompt: String,
    r#type: FeedbackPromptInputType,
    options: FeedbackPromptInputOptions,
    #[builder(default)]
    #[derivative(PartialEq = "ignore")]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    created_at: DateTime,
}

crud!(FeedbackPromptField {});
impl_select_page!(FeedbackPromptField {select_page_by_prompt(prompt: &str) => "`WHERE prompt = #{prompt}`"});

