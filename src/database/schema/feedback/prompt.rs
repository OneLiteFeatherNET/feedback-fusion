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

use crate::{prelude::*, save_as_json, to_date_time};
use feedback_fusion_common::proto::{ProtoField, ProtoFieldType, ProtoPrompt};
use rbatis::rbdc::DateTime;

use super::FieldOptions;

#[derive(
    Deserialize,
    Serialize,
    Clone,
    Derivative,
    Debug,
    Getters,
    Setters,
    TypedBuilder,
    Validate,
    Encode,
    Decode,
)]
#[derivative(PartialEq)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct Prompt {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    #[validate(length(max = 255))]
    title: String,
    #[validate(length(max = 255))]
    description: String,
    target: String,
    #[builder(default = true)]
    #[serde(deserialize_with = "serde_this_or_that::as_bool")]
    active: bool,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    #[bincode(with_serde)]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    #[bincode(with_serde)]
    created_at: DateTime,
}

impl From<Prompt> for ProtoPrompt {
    fn from(val: Prompt) -> Self {
        ProtoPrompt {
            id: val.id,
            title: val.title,
            description: val.description,
            target: val.target,
            active: val.active,
            updated_at: Some(date_time_to_timestamp(&val.updated_at)),
            created_at: Some(date_time_to_timestamp(&val.created_at)),
        }
    }
}

impl From<ProtoPrompt> for Prompt {
    fn from(val: ProtoPrompt) -> Self {
        Prompt {
            id: val.id,
            title: val.title,
            description: val.description,
            target: val.target,
            active: val.active,
            updated_at: to_date_time!(val.updated_at),
            created_at: to_date_time!(val.created_at),
        }
    }
}

crud!(Prompt {});
impl_select!(Prompt {select_by_id(id: &str) -> Option => "`WHERE id = #{id}`"});
impl_select_page!(Prompt {select_page_by_target(target: &str) => "`WHERE target = #{target}`"});

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Encode, Decode)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Rating,
    Checkbox,
    Selection,
    Range,
    Number,
}

impl TryFrom<i32> for FieldType {
    type Error = FeedbackFusionError;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::Text),
            1 => Ok(Self::Rating),
            2 => Ok(Self::Checkbox),
            3 => Ok(Self::Selection),
            4 => Ok(Self::Range),
            5 => Ok(Self::Number),
            _ => Err(FeedbackFusionError::BadRequest("invalid type".to_owned())),
        }
    }
}

impl From<FieldType> for i32 {
    fn from(val: FieldType) -> Self {
        match val {
            FieldType::Text => 0,
            FieldType::Rating => 1,
            FieldType::Checkbox => 2,
            FieldType::Selection => 3,
            FieldType::Range => 4,
            FieldType::Number => 5,
        }
    }
}

impl From<FieldType> for ProtoFieldType {
    fn from(val: FieldType) -> Self {
        match val {
            FieldType::Text => ProtoFieldType::Text,
            FieldType::Rating => ProtoFieldType::Rating,
            FieldType::Checkbox => ProtoFieldType::Checkbox,
            FieldType::Selection => ProtoFieldType::Selection,
            FieldType::Range => ProtoFieldType::Range,
            FieldType::Number => ProtoFieldType::Number,
        }
    }
}

impl From<ProtoFieldType> for FieldType {
    fn from(val: ProtoFieldType) -> Self {
        match val {
            ProtoFieldType::Text => FieldType::Text,
            ProtoFieldType::Rating => FieldType::Rating,
            ProtoFieldType::Checkbox => FieldType::Checkbox,
            ProtoFieldType::Selection => FieldType::Selection,
            ProtoFieldType::Range => FieldType::Range,
            ProtoFieldType::Number => FieldType::Number,
        }
    }
}

save_as_json!(FieldOptions, options);

#[derive(
    Deserialize,
    Serialize,
    Clone,
    Derivative,
    Debug,
    Getters,
    Setters,
    TypedBuilder,
    Validate,
    Encode,
    Decode,
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
    field_type: FieldType,
    #[serde(
        serialize_with = "serialize_options",
        deserialize_with = "deserialize_options"
    )]
    options: FieldOptions,
    #[builder(default_code = r#"DateTime::utc()"#)]
    #[derivative(PartialEq = "ignore")]
    #[bincode(with_serde)]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    #[bincode(with_serde)]
    created_at: DateTime,
}

impl From<Field> for ProtoField {
    fn from(val: Field) -> Self {
        ProtoField {
            id: val.id,
            title: val.title,
            description: val.description,
            prompt: val.prompt,
            field_type: val.field_type.into(),
            options: Some(val.options.into()),
            updated_at: Some(date_time_to_timestamp(&val.updated_at)),
            created_at: Some(date_time_to_timestamp(&val.created_at)),
        }
    }
}

impl TryInto<Field> for ProtoField {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<Field> {
        Ok(Field {
            id: self.id,
            title: self.title,
            description: self.description,
            prompt: self.prompt,
            field_type: self.field_type.try_into()?,
            options: self.options.unwrap().try_into()?,
            updated_at: to_date_time!(self.updated_at),
            created_at: to_date_time!(self.created_at),
        })
    }
}

crud!(Field {});
impl_select!(Field {select_by_id(id: &str) -> Option => "`WHERE id = #{id}`"});
impl_select_page!(Field {select_page_by_prompt(prompt: &str) => "`WHERE prompt = #{prompt}`"});
