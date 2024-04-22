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
use rbatis::rbdc::DateTime;

#[derive(
    Deserialize, Serialize, Clone, Derivative, Debug, Getters, Setters, TypedBuilder, Validate,
)]
#[derivative(PartialEq)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct Target {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    #[validate(length(max = 255))]
    name: String,
    #[builder(default)]
    description: Option<String>,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    created_at: DateTime,
}

impl Into<feedback_fusion_common::proto::Target> for Target {
    fn into(self) -> feedback_fusion_common::proto::Target {
        feedback_fusion_common::proto::Target {
            id: self.id,
            name: self.name,
            description: self.description,
            updated_at: self.updated_at.into(),
            created_at: self.created_at.into(),
        }
    }
}

impl Into<Target> for feedback_fusion_common::proto::Target {
    fn into(self) -> Target {
        Target {
            id: self.id,
            name: self.name,
            description: self.description,
            updated_at: self.updated_at.into(),
            created_at: self.created_at.into(),
        }
    }
}

crud!(Target {});
impl_select!(Target {select_by_id(id: &str) -> Option => "`WHERE id = #{id} LIMIT 1`"});
impl_select_page_wrapper!(Target {select_page(query: &str) => "`WHERE name ILIKE COALESCE('%' || NULLIF(#{query}, '') || '%', '%%')`"});
