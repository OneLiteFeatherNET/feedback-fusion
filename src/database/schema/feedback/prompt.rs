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

use crate::{database::schema::date_time_to_timestamp, prelude::*, to_date_time};
use rbatis::rbdc::DateTime;

use super::FieldOptions;

#[derive(
    Deserialize, Serialize, Clone, Derivative, Debug, Getters, Setters, TypedBuilder, Validate,
)]
#[derivative(PartialEq)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct Prompt {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    #[validate(length(max = 255))]
    title: String,
    #[validate(length(max = 255))]
    description: String,
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

impl Into<feedback_fusion_common::proto::Prompt> for Prompt {
    fn into(self) -> feedback_fusion_common::proto::Prompt {
        feedback_fusion_common::proto::Prompt {
            id: self.id,
            title: self.title,
            description: self.description,
            target: self.target,
            active: self.active,
            updated_at: Some(date_time_to_timestamp(self.updated_at)),
            created_at: Some(date_time_to_timestamp(self.created_at)),
        }
    }
}

impl Into<Prompt> for feedback_fusion_common::proto::Prompt {
    fn into(self) -> Prompt {
        Prompt {
            id: self.id,
            title: self.title,
            description: self.description,
            target: self.target,
            active: self.active,
            updated_at: to_date_time!(self.updated_at),
            created_at: to_date_time!(self.created_at),
        }
    }
}

crud!(Prompt {});
impl_select!(Prompt {select_by_id(id: &str) -> Option => "`WHERE id = #{id} LIMIT 1`"});
impl_select_page_wrapper!(Prompt {select_page_by_target(target: &str) => "`WHERE target = #{target}`"});

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Rating,
    Checkbox,
    Selection,
    Range,
    Number,
}

impl Into<feedback_fusion_common::proto::FieldType> for FieldType {
    fn into(self) -> feedback_fusion_common::proto::FieldType {
        match self {
            Self::Text => feedback_fusion_common::proto::FieldType::Text,
            Self::Rating => feedback_fusion_common::proto::FieldType::Rating,
            Self::Checkbox => feedback_fusion_common::proto::FieldType::Checkbox,
            Self::Selection => feedback_fusion_common::proto::FieldType::Selection,
            Self::Range => feedback_fusion_common::proto::FieldType::Range,
            Self::Number => feedback_fusion_common::proto::FieldType::Number,
        }
    }
}

impl Into<FieldType> for feedback_fusion_common::proto::FieldType {
    fn into(self) -> FieldType {
        match self {
            Self::Text => FieldType::Text,
            Self::Rating => FieldType::Rating,
            Self::Checkbox => FieldType::Checkbox,
            Self::Selection => FieldType::Selection,
            Self::Range => FieldType::Range,
            Self::Number => FieldType::Number,
        }
    }
}

#[derive(
    Deserialize, Serialize, Clone, Derivative, Debug, Getters, Setters, TypedBuilder, Validate,
)]
#[derivative(PartialEq)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct Field {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    #[validate(length(max = 32))]
    title: String,
    #[validate(length(max = 255))]
    description: Option<String>,
    prompt: String,
    r#type: FieldType,
    options: JsonV<FieldOptions>,
    #[builder(default)]
    #[derivative(PartialEq = "ignore")]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    created_at: DateTime,
}

impl Into<feedback_fusion_common::proto::Field> for Field {
    fn into(self) -> feedback_fusion_common::proto::Field {
        feedback_fusion_common::proto::Field {
            id: self.id,
            title: self.title,
            description: self.description,
            prompt: self.prompt,
            field_type: self.r#type.into(),
            options: self.options.0.into(),
            updated_at: Some(date_time_to_timestamp(self.updated_at)),
            created_at: Some(date_time_to_timestamp(self.created_at)),
        }
    }
}

impl Into<Field> for feedback_fusion_common::proto::Field {
    fn into(self) -> Field {
        Field {
            id: self.id,
            title: self.title,
            description: self.description,
            prompt: self.prompt,
            r#type: self.field_type.into(),
            options: JsonV(self.options.into()),
            updated_at: to_date_time!(self.updated_at),
            created_at: to_date_time!(self.created_at),
        }
    }
}

crud!(Field {});
impl_select!(Field {select_by_id(id: &str) -> Option => "`WHERE id = #{id} LIMIT 1`"});
impl_select_page_wrapper!(Field {select_page_by_prompt(prompt: &str) => "`WHERE prompt = #{prompt}`"});
