//SPDX-FileCopyrightText: 2025 OneLiteFeatherNet
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

use crate::{impl_select_page_wrapper, prelude::*};
use rbatis::rbdc::{Bytes, DateTime};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AuditResource {
    Target,
    Prompt,
    Field,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
}

#[derive(
    Deserialize, Serialize, Clone, Derivative, Debug, Getters, Setters, TypedBuilder, Eq, PartialOrd, Hash
)]
#[derivative(PartialEq, Ord)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct AuditVersion {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    resource_id: String,
    resource_type: AuditResource,
    action: AuditAction,
    #[derivative(Ord = "ignore")]
    data: Bytes,
    version: u32,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    created_at: DateTime,
}

crud!(AuditVersion {});
impl_select_page_wrapper!(AuditVersion { select_page_by_resource_id(id: &str) => "`WHERE resource_id = #{id}`" });
