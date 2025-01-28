//SPDX-FileCopyrightText: 2023 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2025 OneLiteFeatherNet

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

use crate::{prelude::*, save_as_json};
use rbatis::rbdc::DateTime;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ResourceKind {
    Target,
    Prompt,
    Field,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ResourceAuthorizationData {
    Scope(Vec<String>),
    Prompt(Vec<String>),
    User(Vec<String>),
}

save_as_json!(ResourceAuthorizationData, authorization_data);

#[derive(Deserialize, Serialize, Clone, Derivative, Debug, Getters, Setters, TypedBuilder)]
#[derivative(PartialEq)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct ResourceAuthorization {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    resource_kind: ResourceKind,
    resource_id: String,
    #[serde(
        serialize_with = "serialize_authorization_data",
        deserialize_with = "deserialize_authorization_data"
    )]
    authorization_data: ResourceAuthorizationData,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    created_at: DateTime,
}

crud!(ResourceAuthorization {});
