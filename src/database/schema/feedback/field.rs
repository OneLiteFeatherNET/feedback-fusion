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

macro_rules! map_options {
    ($($path:path $(,)?)*) => {
       $(
        impl Into<$path> for FieldOptions {
            fn into(self) -> $path {
                match self {
                    Self::Text(options) => <$path>::Text(options.into()),
                    Self::Rating(options) => <$path>::Rating(options.into()),
                    Self::Checkbox(options) => <$path>::Checkbox(options.into()),
                    Self::Selection(options) => <$path>::Selection(options.into()),
                    Self::Range(options) => <$path>::Range(options.into()),
                    Self::Number(options) => <$path>::Number(options.into()),
                }
            }
        }

        impl TryInto<FieldOptions> for $path {
            type Error = FeedbackFusionError;

            fn try_into(self) -> Result<FieldOptions> {
                Ok(match self {
                    Self::Text(options) => FieldOptions::Text(options.try_into()?),
                    Self::Rating(options) => FieldOptions::Rating(options.try_into()?),
                    Self::Checkbox(options) => FieldOptions::Checkbox(options.try_into()?),
                    Self::Selection(options) => FieldOptions::Selection(options.into()),
                    Self::Range(options) => FieldOptions::Range(options.try_into()?),
                    Self::Number(options) => FieldOptions::Number(options.try_into()?),
                })
            }
        }
        )*
    };
}

map_options!(
    feedback_fusion_common::proto::create_field_request::Options,
    feedback_fusion_common::proto::update_field_request::Options,
    feedback_fusion_common::proto::field::Options,
);

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

impl TryInto<TextOptions> for feedback_fusion_common::proto::TextOptions {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<TextOptions> {
        Ok(TextOptions {
            placeholder: self.placeholder,
            lines: self.lines.try_into()?,
        })
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

impl TryInto<RatingOptions> for feedback_fusion_common::proto::RatingOptions {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<RatingOptions> {
        Ok(RatingOptions {
            max: self.max.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CheckboxStyle {
    Switch,
    Normal,
}

impl Into<i32> for CheckboxStyle {
    fn into(self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::Switch => 1,
        }
    }
}

impl TryFrom<i32> for CheckboxStyle {
    type Error = FeedbackFusionError;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Switch),
            _ => Err(FeedbackFusionError::BadRequest(
                "invalid CheckboxStyle".to_owned(),
            )),
        }
    }
}

impl Into<feedback_fusion_common::proto::CheckboxStyle> for CheckboxStyle {
    fn into(self) -> feedback_fusion_common::proto::CheckboxStyle {
        match self {
            Self::Switch => feedback_fusion_common::proto::CheckboxStyle::Switch,
            Self::Normal => feedback_fusion_common::proto::CheckboxStyle::Normal,
        }
    }
}

impl Into<CheckboxStyle> for feedback_fusion_common::proto::CheckboxStyle {
    fn into(self) -> CheckboxStyle {
        match self {
            Self::Switch => CheckboxStyle::Switch,
            Self::Normal => CheckboxStyle::Normal,
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

impl TryInto<CheckboxOptions> for feedback_fusion_common::proto::CheckboxOptions {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<CheckboxOptions> {
        Ok(CheckboxOptions {
            default_state: self.default_state,
            style: self.style.try_into()?,
        })
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

impl TryInto<RangeOptions> for feedback_fusion_common::proto::RangeOptions {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<RangeOptions> {
        Ok(RangeOptions {
            min: self.min.try_into()?,
            max: self.max.try_into()?,
        })
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

impl TryInto<NumberOptions> for feedback_fusion_common::proto::NumberOptions {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<NumberOptions> {
        Ok(NumberOptions {
            min: self.min.try_into()?,
            max: self.max.try_into()?,
            placeholder: self.placeholder,
        })
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
            created_at: Some(date_time_to_timestamp(self.created_at)),
        }
    }
}

impl Into<PromptResponse> for feedback_fusion_common::proto::PromptResponse {
    fn into(self) -> PromptResponse {
        PromptResponse {
            id: self.id,
            prompt: self.prompt,
            created_at: to_date_time!(self.created_at),
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
            data: Some(self.data.0.into()),
        }
    }
}

impl TryInto<FieldResponse> for feedback_fusion_common::proto::FieldResponse {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<FieldResponse> {
        Ok(FieldResponse {
            id: self.id,
            response: self.response,
            field: self.field,
            data: JsonV(self.data.unwrap().try_into()?),
        })
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

macro_rules! map_response {
    ($($path:path $(,)?)*) => {
        $(
        impl Into<$path> for FieldData {
            fn into(self) -> $path {
                match self {
                    Self::Text(data) => <$path>::Text(data.into()),
                    Self::Rating(data) => <$path>::Rating(data.into()),
                    Self::Checkbox(data) => <$path>::Checkbox(data.into()),
                    Self::Selection(data) => <$path>::Selection(data.into()),
                    Self::Range(data) => <$path>::Range(data.into()),
                    Self::Number(data) => <$path>::Number(data.into()),
                }
            }
        }

        impl TryInto<FieldData> for $path {
            type Error = FeedbackFusionError;

            fn try_into(self) -> Result<FieldData> {
                Ok(match self {
                    Self::Text(data) => FieldData::Text(data.into()),
                    Self::Rating(data) => FieldData::Rating(data.try_into()?),
                    Self::Checkbox(data) => FieldData::Checkbox(data.into()),
                    Self::Selection(data) => FieldData::Selection(data.into()),
                    Self::Range(data) => FieldData::Range(data.try_into()?),
                    Self::Number(data) => FieldData::Number(data.try_into()?),
                })
            }
        }
        )*
    };
}

map_response!(
    feedback_fusion_common::proto::field_response::Data,
    feedback_fusion_common::proto::response_data::Data
);

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
                if !options.multiple && response.values.len() > 1 {
                    Err(FeedbackFusionError::BadRequest("selecting multiple is not allowed".to_owned()))
                } else {
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

impl TryInto<RatingResponse> for feedback_fusion_common::proto::RatingResponse {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<RatingResponse> {
        Ok(RatingResponse {
            rating: self.rating.try_into()?,
        })
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

impl TryInto<RangeResponse> for feedback_fusion_common::proto::RangeResponse {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<RangeResponse> {
        Ok(RangeResponse {
            start: self.start.try_into()?,
            end: self.end.try_into()?,
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct NumberResponse {
    number: u8,
}

impl Into<feedback_fusion_common::proto::NumberResponse> for NumberResponse {
    fn into(self) -> feedback_fusion_common::proto::NumberResponse {
        feedback_fusion_common::proto::NumberResponse {
            number: self.number.into(),
        }
    }
}

impl TryInto<NumberResponse> for feedback_fusion_common::proto::NumberResponse {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<NumberResponse> {
        Ok(NumberResponse {
            number: self.number.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CheckboxOptions, CheckboxResponse, FieldData, FieldOptions, NumberOptions, NumberResponse,
        RangeOptions, RangeResponse, RatingOptions, RatingResponse, SelectionOptions,
        SelectionResponse, TextOptions, TextResponse,
    };

    #[test]
    pub fn test_text_validation() {
        let data = FieldData::Text(TextResponse {
            text: "Hello, world!".to_owned(),
        });

        let options = FieldOptions::Text(TextOptions {
            placeholder: "".to_owned(),
            lines: 1,
        });
        assert!(data.validate(&options).is_ok());
    }

    #[test]
    pub fn test_rating_validation() {
        let data = FieldData::Rating(RatingResponse { rating: 5 });

        let options = FieldOptions::Rating(RatingOptions { max: 5 });
        assert!(data.validate(&options).is_ok());

        let options = FieldOptions::Rating(RatingOptions { max: 4 });
        assert!(data.validate(&options).is_err());
    }

    #[test]
    pub fn test_checkbox_validation() {
        let options = FieldOptions::Checkbox(CheckboxOptions {
            default_state: true,
            style: super::CheckboxStyle::Switch,
        });

        let data = FieldData::Checkbox(CheckboxResponse { checked: true });
        assert!(data.validate(&options).is_ok());

        let data = FieldData::Checkbox(CheckboxResponse { checked: false });
        assert!(data.validate(&options).is_ok());
    }

    #[test]
    pub fn test_selection_validation() {
        let options = FieldOptions::Selection(SelectionOptions {
            multiple: false,
            combobox: false,
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned()],
        });
        assert!(data.validate(&options).is_ok());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["FooBar".to_owned()],
        });
        assert!(data.validate(&options).is_err());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });
        assert!(data.validate(&options).is_err());

        let options = FieldOptions::Selection(SelectionOptions {
            multiple: true,
            combobox: false,
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned()],
        });
        assert!(data.validate(&options).is_ok());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["FooBar".to_owned()],
        });
        assert!(data.validate(&options).is_err());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });
        assert!(data.validate(&options).is_ok());

        let options = FieldOptions::Selection(SelectionOptions {
            multiple: false,
            combobox: true,
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned()],
        });
        assert!(data.validate(&options).is_ok());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["FooBar".to_owned()],
        });
        assert!(data.validate(&options).is_ok());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });
        assert!(data.validate(&options).is_err());

        let options = FieldOptions::Selection(SelectionOptions {
            multiple: true,
            combobox: true,
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned()],
        });
        assert!(data.validate(&options).is_ok());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["FooBar".to_owned()],
        });
        assert!(data.validate(&options).is_ok());

        let data = FieldData::Selection(SelectionResponse {
            values: vec!["Foo".to_owned(), "Bar".to_owned()],
        });
        assert!(data.validate(&options).is_ok());
    }

    #[test]
    pub fn test_range_validation() {
        let options = FieldOptions::Range(RangeOptions { min: 2, max: 5 });

        let data = FieldData::Range(RangeResponse { start: 1, end: 5 });
        assert!(data.validate(&options).is_err());

        let data = FieldData::Range(RangeResponse { start: 2, end: 6 });
        assert!(data.validate(&options).is_err());

        let data = FieldData::Range(RangeResponse { start: 2, end: 4 });
        assert!(data.validate(&options).is_ok());
    }

    #[test]
    pub fn test_number_validation() {
        let options = FieldOptions::Number(NumberOptions {
            min: 2,
            max: 4,
            placeholder: "".to_owned(),
        });

        let data = FieldData::Number(NumberResponse { number: 1 });
        assert!(data.validate(&options).is_err());

        let data = FieldData::Number(NumberResponse { number: 5 });
        assert!(data.validate(&options).is_err());

        let data = FieldData::Number(NumberResponse { number: 2 });
        assert!(data.validate(&options).is_ok());
    }
}
