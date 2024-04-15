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

use super::FeedbackPromptInputType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, ToSchema)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(TS))]
pub enum FeedbackPromptInputOptions {
    Text(TextOptions),
    Rating(RatingOptions),
    Checkbox(CheckboxOptions),
    Selection(SelectionOptions),
    Range(RangeOptions),
    Number(NumberOptions),
}

// TODO: gen with macro
impl PartialEq<FeedbackPromptInputOptions> for FeedbackPromptInputType {
    fn eq(&self, other: &FeedbackPromptInputOptions) -> bool {
        match self {
            Self::Text => matches!(other, FeedbackPromptInputOptions::Text(_)),
            Self::Rating => matches!(other, FeedbackPromptInputOptions::Rating(_)),
            Self::Checkbox => matches!(other, FeedbackPromptInputOptions::Checkbox(_)),
            Self::Selection => matches!(other, FeedbackPromptInputOptions::Selection(_)),
            Self::Range => matches!(other, FeedbackPromptInputOptions::Range(_)),
            Self::Number => matches!(other, FeedbackPromptInputOptions::Number(_)),
        }
    }
}

macro_rules! impl_parse {
    ($t:ident, $data:ident, $target:path, $($ty:path $(,)?)*) => {
        match $t {
            $(
                $ty => {
                    let object = $data.as_object_mut().unwrap();
                    object.insert("type".to_owned(), serde_json::to_value(&$ty)?);

                    Ok(serde_json::from_value::<$target>($data)?)
                },
            )*
        }
    };
}

impl FeedbackPromptInputOptions {
    pub fn parse(ty: &FeedbackPromptInputType, mut data: serde_json::Value) -> Result<Self> {
        impl_parse!(
            ty,
            data,
            Self,
            FeedbackPromptInputType::Text,
            FeedbackPromptInputType::Rating,
            FeedbackPromptInputType::Checkbox,
            FeedbackPromptInputType::Selection,
            FeedbackPromptInputType::Range,
            FeedbackPromptInputType::Number
        )
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct TextOptions {
    /// the input placeholder
    #[validate(length(max = 255))]
    placeholder: String,
    /// support for textareas
    #[serde(default = "default_lines")]
    lines: u8,
}

fn default_lines() -> u8 {
    1
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct RatingOptions {
    /// the best rating (determines how many stars / points are shown in the frontend)
    max: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(TS))]
pub enum CheckboxStyle {
    Switch,
    Checkbox,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct CheckboxOptions {
    /// the default state of the checkbox
    default_state: bool,
    style: CheckboxStyle,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct SelectionOptions {
    /// all possible selections
    values: Vec<String>,
    /// allows the client to select multiple values
    #[serde(default)]
    multiple: bool,
    /// allows the client to add it's own values
    #[serde(default)]
    combobox: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct RangeOptions {
    /// the min value
    #[serde(default)]
    min: u8,
    /// the max value
    max: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, ToSchema, Validate)]
#[builder(field_defaults(setter(into)))]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct NumberOptions {
    /// the min value
    #[serde(default)]
    min: u8,
    /// the max value
    max: u8,
    /// input placeholder
    placeholder: String,
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
    data: FeedbackPromptFieldData,
}

crud!(FeedbackPromptFieldResponse {});

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, ToSchema)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(TS))]
pub enum FeedbackPromptFieldData {
    Text(TextResponse),
    Rating(RatingResponse),
    Checkbox(CheckboxResponse),
    Selection(SelectionResponse),
    Range(RangeResponse),
    Number(NumberResponse),
}

macro_rules! validate_data {
    ($self: expr, $voptions: expr, $rident:ident, $ident:ident, $($type:path = $options:path => $if:block $(,)?)*) => {
        $(
            if let &$type($rident) = &$self {
                if let &$options($ident) = &$voptions {
                    $if
                } else {
                    Err(FeedbackFusionError::BadRequest(concat!("invalid data type: expected ", stringify!($options)).to_owned()))
                }
            } else)* {
                Ok(())
            }
    };
}

// TODO: please do this with a macro
impl FeedbackPromptFieldData {
    pub fn parse(ty: &FeedbackPromptInputType, mut data: serde_json::Value) -> Result<Self> {
        impl_parse!(
            ty,
            data,
            Self,
            FeedbackPromptInputType::Text,
            FeedbackPromptInputType::Rating,
            FeedbackPromptInputType::Checkbox,
            FeedbackPromptInputType::Selection,
            FeedbackPromptInputType::Range,
            FeedbackPromptInputType::Number
        )
    }

    #[allow(unused_variables)]
    pub fn validate(&self, options: &FeedbackPromptInputOptions) -> Result<()> {
        validate_data!(
            self,
            options,
            response,
            options,
            Self::Text = FeedbackPromptInputOptions::Text => { Ok(()) },
            Self::Rating = FeedbackPromptInputOptions::Rating => {
                if response.rating > options.max {
                    Err(FeedbackFusionError::BadRequest(format!(
                        "data '{}' is greater than '{}'", response.rating, options.max
                    )))
                } else {
                    Ok(())
                }
            },
            Self::Checkbox = FeedbackPromptInputOptions::Checkbox => { Ok(()) },
            Self::Selection = FeedbackPromptInputOptions::Selection => {
                if !options.combobox {
                    let invalid = response.values.iter().find(|value| !options.values.contains(value));

                    if let Some(invalid) = invalid {
                        Err(FeedbackFusionError::BadRequest(format!(
                            "found invalid value '{}'", invalid
                        )))
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            },
            Self::Range = FeedbackPromptInputOptions::Range => {
                if response.start < options.min || response.end > options.max {
                    Err(FeedbackFusionError::BadRequest(format!(
                        "data does is not within '{}' and '{}'", options.min, options.max
                    )))
                } else {
                    Ok(())
                }
            },
            Self::Number = FeedbackPromptInputOptions::Number => {
                if response.number > options.max || response.number < options.min {
                    Err(FeedbackFusionError::BadRequest(format!(
                        "data '{}' does is not within '{}' and '{}'", response.number, options.min, options.max
                    )))
                } else {
                    Ok(())
                }
            }
        )
    }
}

// TODO: use macro
impl PartialEq<FeedbackPromptFieldData> for FeedbackPromptInputType {
    fn eq(&self, other: &FeedbackPromptFieldData) -> bool {
        match self {
            Self::Text => matches!(other, FeedbackPromptFieldData::Text(_)),
            Self::Rating => matches!(other, FeedbackPromptFieldData::Rating(_)),
            Self::Checkbox => matches!(other, FeedbackPromptFieldData::Checkbox(_)),
            Self::Selection => matches!(other, FeedbackPromptFieldData::Selection(_)),
            Self::Range => matches!(other, FeedbackPromptFieldData::Range(_)),
            Self::Number => matches!(other, FeedbackPromptFieldData::Number(_)),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct TextResponse {
    text: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct RatingResponse {
    rating: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct CheckboxResponse {
    checked: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct SelectionResponse {
    values: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct RangeResponse {
    start: u8,
    end: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, PartialEq)]
#[cfg_attr(feature = "bindings", derive(TS))]
pub struct NumberResponse {
    number: u8,
}
