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

use super::FieldType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum FieldOptions {
    Text(TextOptions),
    Rating(RatingOptions),
    Checkbox(CheckboxOptions),
    Selection(SelectionOptions),
    Range(RangeOptions),
    Number(NumberOptions),
}

impl Into<feedback_fusion_common::proto::create_field_request::Options> for FieldOptions {
    fn into(self) -> feedback_fusion_common::proto::create_field_request::Options {
        match self {
            Text(options) => {
                feedback_fusion_common::proto::create_field_request::Options::Text(options.into())
            }
            Rating(options) => {
                feedback_fusion_common::proto::create_field_request::Options::Rating(options.into())
            }
            Checkbox(options) => {
                feedback_fusion_common::proto::create_field_request::Options::Checkbox(
                    options.into(),
                )
            }
            Selection(options) => {
                feedback_fusion_common::proto::create_field_request::Options::Selection(
                    options.into(),
                )
            }
            Range(options) => {
                feedback_fusion_common::proto::create_field_request::Options::Range(options.into())
            }
            Number(options) => {
                feedback_fusion_common::proto::create_field_request::Options::Number(options.into())
            }
        }
    }
}

impl Into<FieldOptions> for feedback_fusion_common::proto::create_field_request::Options {
    fn into(self) -> FieldOptions {
        match self {
            Text(options) => FieldOptions::Text(options.into()),
            Rating(options) => FieldOptions::Rating(options.into()),
            Checkbox(options) => FieldOptions::Checkbox(options.into()),
            Selection(options) => FieldOptions::Selection(options.into()),
            Range(options) => FieldOptions::Range(options.into()),
            Number(options) => FieldOptions::Number(options.into()),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, Validate)]
#[builder(field_defaults(setter(into)))]
pub struct TextOptions {
    /// the input placeholder
    #[validate(length(max = 255))]
    placeholder: String,
    /// support for textareas
    #[serde(default = "default_lines")]
    lines: u8,
}

impl Into<feedback_fusion_common::proto::TextOptions> for TextOptions {
    fn into(self) -> feedback_fusion_common::proto::TextOptions {
        feedback_fusion_common::proto::TextOptions {
            placeholder: self.placeholder,
            lines: self.lines.into(),
        }
    }
}

impl Into<TextOptions> for feedback_fusion_common::proto::TextOptions {
    fn into(self) -> TextOptions {
        TextOptions {
            placeholder: self.placeholder,
            lines: self.lines.into(),
        }
    }
}

fn default_lines() -> u8 {
    1
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, Validate)]
#[builder(field_defaults(setter(into)))]
pub struct RatingOptions {
    /// the best rating (determines how many stars / points are shown in the frontend)
    max: u8,
}

impl Into<feedback_fusion_common::proto::RatingOptions> for RatingOptions {
    fn into(self) -> feedback_fusion_common::proto::RatingOptions {
        feedback_fusion_common::proto::RatingOptions {
            max: self.max.into(),
        }
    }
}

impl Into<RatingOptions> for feedback_fusion_common::proto::RatingOptions {
    fn into(self) -> RatingOptions {
        RatingOptions {
            max: self.max.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CheckboxStyle {
    Switch,
    Checkbox,
}

impl Into<feedback_fusion_common::proto::CheckboxStyle> for CheckboxStyle {
    fn into(self) -> feedback_fusion_common::proto::CheckboxStyle {
        match self {
            Self::Switch => feedback_fusion_common::proto::CheckboxStyle::Switch,
            Self::Checkbox => feedback_fusion_common::proto::CheckboxStyle::Normal,
        }
    }
}

impl Into<CheckboxStyle> for feedback_fusion_common::proto::CheckboxStyle {
    fn into(self) -> CheckboxStyle {
        match self {
            Self::Switch => CheckboxStyle::Switch,
            Self::Checkbox => CheckboxStyle::Normal,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, Validate)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CheckboxOptions {
    /// the default state of the checkbox
    default_state: bool,
    style: CheckboxStyle,
}

impl Into<feedback_fusion_common::proto::CheckboxOptions> for CheckboxOptions {
    fn into(self) -> feedback_fusion_common::proto::CheckboxOptions {
        feedback_fusion_common::proto::CheckboxOptions {
            default_state: self.default_state,
            style: self.style.into(),
        }
    }
}

impl Into<CheckboxOptions> for feedback_fusion_common::proto::CheckboxOptions {
    fn into(self) -> CheckboxOptions {
        CheckboxOptions {
            default_state: self.default_state,
            style: self.style.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, Validate)]
#[builder(field_defaults(setter(into)))]
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

impl Into<feedback_fusion_common::proto::SelectionOptions> for SelectionOptions {
    fn into(self) -> feedback_fusion_common::proto::SelectionOptions {
        feedback_fusion_common::proto::SelectionOptions {
            values: self.values,
            multiple: self.multiple,
            combobox: self.combobox,
        }
    }
}

impl Into<SelectionOptions> for feedback_fusion_common::proto::SelectionOptions {
    fn into(self) -> SelectionOptions {
        SelectionOptions {
            values: self.values,
            multiple: self.multiple,
            combobox: self.combobox,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, Validate)]
#[builder(field_defaults(setter(into)))]
pub struct RangeOptions {
    /// the min value
    #[serde(default)]
    min: u8,
    /// the max value
    max: u8,
}

impl Into<feedback_fusion_common::proto::RangeOptions> for RangeOptions {
    fn into(self) -> feedback_fusion_common::proto::RangeOptions {
        feedback_fusion_common::proto::RangeOptions {
            min: self.min.into(),
            max: self.max.into(),
        }
    }
}

impl Into<RangeOptions> for feedback_fusion_common::proto::RangeOptions {
    fn into(self) -> RangeOptions {
        RangeOptions {
            min: self.min.into(),
            max: self.max.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, TypedBuilder, Validate)]
#[builder(field_defaults(setter(into)))]
pub struct NumberOptions {
    /// the min value
    #[serde(default)]
    min: u8,
    /// the max value
    max: u8,
    /// input placeholder
    placeholder: String,
}

impl Into<feedback_fusion_common::proto::NumberOptions> for NumberOptions {
    fn into(self) -> feedback_fusion_common::proto::NumberOptions {
        feedback_fusion_common::proto::NumberOptions {
            min: self.min.into(),
            max: self.max.into(),
            placeholder: self.placeholder,
        }
    }
}

impl Into<NumberOptions> for feedback_fusion_common::proto::NumberOptions {
    fn into(self) -> NumberOptions {
        NumberOptions {
            min: self.min.into(),
            max: self.max.into(),
            placeholder: self.placeholder,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Derivative, Debug, Getters, MutGetters, TypedBuilder)]
#[derivative(PartialEq)]
#[get = "pub"]
#[get_mut = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct PromptResponse {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    prompt: String,
    #[derivative(PartialEq = "ignore")]
    #[builder(default)]
    created_at: DateTime,
}

impl Into<feedback_fusion_common::proto::PromptResponse> for PromptResponse {
    fn into(self) -> feedback_fusion_common::proto::PromptResponse {
        feedback_fusion_common::proto::PromptResponse {
            id: self.id,
            prompt: self.prompt,
            created_at: self.created_at.into(),
        }
    }
}

impl Into<PromptResponse> for feedback_fusion_common::proto::PromptResponse {
    fn into(self) -> PromptResponse {
        PromptResponse {
            id: self.id,
            prompt: self.prompt,
            created_at: self.created_at.into(),
        }
    }
}

crud!(PromptResponse {});
impl_select_page_wrapper!(PromptResponse {select_page_by_prompt(prompt: &str) => "WHERE prompt = #{prompt}"});

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Getters, MutGetters, TypedBuilder)]
#[get = "pub"]
#[get_mut = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct FieldResponse {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    response: String,
    field: String,
    data: JsonV<FieldData>,
}

impl Into<feedback_fusion_common::proto::FieldResponse> for FieldResponse {
    fn into(self) -> feedback_fusion_common::proto::FieldResponse {
        feedback_fusion_common::proto::FieldResponse {
            id: self.id,
            response: self.response,
            field: self.field,
            data: self.data.0.into(),
        }
    }
}

impl Into<FieldResponse> for feedback_fusion_common::proto::FieldResponse {
    fn into(self) -> FieldResponse {
        FieldResponse {
            id: self.id,
            response: self.response,
            field: self.field,
            data: JsonV(self.data.into()),
        }
    }
}

crud!(FieldResponse {});

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum FieldData {
    Text(TextResponse),
    Rating(RatingResponse),
    Checkbox(CheckboxResponse),
    Selection(SelectionResponse),
    Range(RangeResponse),
    Number(NumberResponse),
}

impl Into<feedback_fusion_common::proto::field_response::Data> for FieldData {
    fn into(self) -> feedback_fusion_common::proto::field_response::Data {
        match self {
            Text(data) => feedback_fusion_common::proto::field_response::Data::Text(data.into()),
            Rating(data) => {
                feedback_fusion_common::proto::field_response::Data::Rating(data.into())
            }
            Checkbox(data) => {
                feedback_fusion_common::proto::field_response::Data::Checkbox(data.into())
            }
            Selection(data) => {
                feedback_fusion_common::proto::field_response::Data::Selection(data.into())
            }
            Range(data) => feedback_fusion_common::proto::field_response::Data::Range(data.into()),
            Number(data) => {
                feedback_fusion_common::proto::field_response::Data::Number(data.into())
            }
        }
    }
}

impl Into<FieldData> for feedback_fusion_common::proto::field_response::Data {
    fn into(self) -> FieldData {
        match self {
            Text(data) => FieldData::Text(data.into()),
            Rating(data) => FieldData::Rating(data.into()),
            Checkbox(data) => FieldData::Checkbox(data.into()),
            Selection(data) => FieldData::Selection(data.into()),
            Range(data) => FieldData::Range(data.into()),
            Number(data) => FieldData::Number(data.into()),
        }
    }
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
impl FieldData {
    #[allow(unused_variables)]
    pub fn validate(&self, options: &FieldOptions) -> Result<()> {
        validate_data!(
            self,
            options,
            response,
            options,
            Self::Text = FieldOptions::Text => { Ok(()) },
            Self::Rating = FieldOptions::Rating => {
                if response.rating > options.max {
                    Err(FeedbackFusionError::BadRequest(format!(
                        "data '{}' is greater than '{}'", response.rating, options.max
                    )))
                } else {
                    Ok(())
                }
            },
            Self::Checkbox = FieldOptions::Checkbox => { Ok(()) },
            Self::Selection = FieldOptions::Selection => {
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
            Self::Range = FieldOptions::Range => {
                if response.start < options.min || response.end > options.max {
                    Err(FeedbackFusionError::BadRequest(format!(
                        "data does is not within '{}' and '{}'", options.min, options.max
                    )))
                } else {
                    Ok(())
                }
            },
            Self::Number = FieldOptions::Number => {
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TextResponse {
    text: String,
}

impl Into<feedback_fusion_common::proto::TextResponse> for TextResponse {
    fn into(self) -> feedback_fusion_common::proto::TextResponse {
        feedback_fusion_common::proto::TextResponse { text: self.text }
    }
}

impl Into<TextResponse> for feedback_fusion_common::proto::TextResponse {
    fn into(self) -> TextResponse {
        TextResponse { text: self.text }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RatingResponse {
    rating: u8,
}

impl Into<feedback_fusion_common::proto::RatingResponse> for RatingResponse {
    fn into(self) -> feedback_fusion_common::proto::RatingResponse {
        feedback_fusion_common::proto::RatingResponse {
            rating: self.rating.into(),
        }
    }
}

impl Into<RatingResponse> for feedback_fusion_common::proto::RatingResponse {
    fn into(self) -> RatingResponse {
        RatingResponse {
            rating: self.rating.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckboxResponse {
    checked: bool,
}

impl Into<feedback_fusion_common::proto::CheckboxResponse> for CheckboxResponse {
    fn into(self) -> feedback_fusion_common::proto::CheckboxResponse {
        feedback_fusion_common::proto::CheckboxResponse {
            checked: self.checked,
        }
    }
}

impl Into<CheckboxResponse> for feedback_fusion_common::proto::CheckboxResponse {
    fn into(self) -> CheckboxResponse {
        CheckboxResponse {
            checked: self.checked,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SelectionResponse {
    values: Vec<String>,
}

impl Into<feedback_fusion_common::proto::SelectionResponse> for SelectionResponse {
    fn into(self) -> feedback_fusion_common::proto::SelectionResponse {
        feedback_fusion_common::proto::SelectionResponse {
            values: self.values,
        }
    }
}

impl Into<SelectionResponse> for feedback_fusion_common::proto::SelectionResponse {
    fn into(self) -> SelectionResponse {
        SelectionResponse {
            values: self.values,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RangeResponse {
    start: u8,
    end: u8,
}

impl Into<feedback_fusion_common::proto::RangeResponse> for RangeResponse {
    fn into(self) -> feedback_fusion_common::proto::RangeResponse {
        feedback_fusion_common::proto::RangeResponse {
            start: self.start.into(),
            end: self.end.into(),
        }
    }
}

impl Into<RangeResponse> for feedback_fusion_common::proto::RangeResponse {
    fn into(self) -> RangeResponse {
        RangeResponse {
            start: self.start.into(),
            end: self.end.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct NumberResponse {
    number: u8,
}

impl Into<feedback_fusion_common::proto::NumberResponse> for NumberResponse {
    fn into(self) -> feedback_fusion_common::proto::NumberResponse {
        feedback_fusion_common::proto::NumberResponse {
            number: Self.number.into(),
        }
    }
}

impl Into<NumberResponse> for feedback_fusion_common::proto::NumberResponse {
    fn into(self) -> NumberResponse {
        NumberResponse {
            number: Self.number.into(),
        }
    }
}
