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

use crate::{database::schema::date_time_to_timestamp, prelude::*, to_date_time, save_as_json};
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
        impl From<FieldOptions> for $path {
            fn from(value: FieldOptions) -> Self {
                match value {
                    FieldOptions::Text(options) => Self::Text(options.into()),
                    FieldOptions::Rating(options) => Self::Rating(options.into()),
                    FieldOptions::Checkbox(options) => Self::Checkbox(options.into()),
                    FieldOptions::Selection(options) => Self::Selection(options.into()),
                    FieldOptions::Range(options) => Self::Range(options.into()),
                    FieldOptions::Number(options) => Self::Number(options.into()),
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

impl From<TextOptions> for feedback_fusion_common::proto::TextOptions {
    fn from(value: TextOptions) -> feedback_fusion_common::proto::TextOptions {
        feedback_fusion_common::proto::TextOptions {
            placeholder: value.placeholder,
            lines: value.lines.into(),
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

impl From<RatingOptions> for feedback_fusion_common::proto::RatingOptions {
    fn from(value: RatingOptions) -> feedback_fusion_common::proto::RatingOptions {
        feedback_fusion_common::proto::RatingOptions {
            max: value.max.into(),
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

impl From<CheckboxStyle> for i32 {
    fn from(value: CheckboxStyle) -> Self {
        match value {
            CheckboxStyle::Normal => 0,
            CheckboxStyle::Switch => 1,
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

impl From<CheckboxStyle> for feedback_fusion_common::proto::CheckboxStyle {
    fn from(value: CheckboxStyle) -> feedback_fusion_common::proto::CheckboxStyle {
        match value {
            CheckboxStyle::Switch => feedback_fusion_common::proto::CheckboxStyle::Switch,
            CheckboxStyle::Normal => feedback_fusion_common::proto::CheckboxStyle::Normal,
        }
    }
}

impl From<feedback_fusion_common::proto::CheckboxStyle> for CheckboxStyle {
    fn from(value: feedback_fusion_common::proto::CheckboxStyle) -> CheckboxStyle {
        match value {
            feedback_fusion_common::proto::CheckboxStyle::Switch => CheckboxStyle::Switch,
            feedback_fusion_common::proto::CheckboxStyle::Normal => CheckboxStyle::Normal,
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

impl From<CheckboxOptions> for feedback_fusion_common::proto::CheckboxOptions {
    fn from(value: CheckboxOptions) -> feedback_fusion_common::proto::CheckboxOptions {
        feedback_fusion_common::proto::CheckboxOptions {
            default_state: value.default_state,
            style: value.style.into(),
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

impl From<SelectionOptions> for feedback_fusion_common::proto::SelectionOptions {
    fn from(value: SelectionOptions) -> Self {
        feedback_fusion_common::proto::SelectionOptions {
            values: value.values,
            multiple: value.multiple,
            combobox: value.combobox,
        }
    }
}

impl From<feedback_fusion_common::proto::SelectionOptions> for SelectionOptions {
    fn from(value: feedback_fusion_common::proto::SelectionOptions) -> Self {
        SelectionOptions {
            values: value.values,
            multiple: value.multiple,
            combobox: value.combobox,
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

impl From<RangeOptions> for feedback_fusion_common::proto::RangeOptions {
    fn from(value: RangeOptions) -> Self {
        feedback_fusion_common::proto::RangeOptions {
            min: value.min.into(),
            max: value.max.into(),
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

impl From<NumberOptions> for feedback_fusion_common::proto::NumberOptions {
    fn from(value: NumberOptions) -> Self {
        feedback_fusion_common::proto::NumberOptions {
            min: value.min.into(),
            max: value.max.into(),
            placeholder: value.placeholder,
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
    #[builder(default_code = r#"DateTime::utc()"#)]
    created_at: DateTime,
}

impl From<PromptResponse> for feedback_fusion_common::proto::PromptResponse {
    fn from(value: PromptResponse) -> Self {
        feedback_fusion_common::proto::PromptResponse {
            id: value.id,
            prompt: value.prompt,
            created_at: Some(date_time_to_timestamp(value.created_at)),
        }
    }
}

impl From<feedback_fusion_common::proto::PromptResponse> for PromptResponse {
    fn from(value: feedback_fusion_common::proto::PromptResponse) -> Self {
        PromptResponse {
            id: value.id,
            prompt: value.prompt,
            created_at: to_date_time!(value.created_at),
        }
    }
}

crud!(PromptResponse {});
impl_select_page_wrapper!(PromptResponse {select_page_by_prompt(prompt: &str) => "WHERE prompt = #{prompt}"});

save_as_json!(FieldData, data);

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Getters, MutGetters, TypedBuilder)]
#[get = "pub"]
#[get_mut = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct FieldResponse {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    response: String,
    field: String,
    #[serde(
        serialize_with = "serialize_data",
        deserialize_with = "deserialize_data"
    )]
    data: FieldData,
}

impl From<FieldResponse> for feedback_fusion_common::proto::FieldResponse {
    fn from(value: FieldResponse) -> Self {
        feedback_fusion_common::proto::FieldResponse {
            id: value.id,
            response: value.response,
            field: value.field,
            data: Some(value.data.into()),
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
            data: self.data.unwrap().try_into()?,
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
        impl From<FieldData> for $path {
            fn from(value: FieldData) -> $path {
                match value {
                    FieldData::Text(data) => Self::Text(data.into()),
                    FieldData::Rating(data) => Self::Rating(data.into()),
                    FieldData::Checkbox(data) => Self::Checkbox(data.into()),
                    FieldData::Selection(data) => Self::Selection(data.into()),
                    FieldData::Range(data) => Self::Range(data.into()),
                    FieldData::Number(data) => Self::Number(data.into()),
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
                } else if !options.combobox {
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

impl From<TextResponse> for feedback_fusion_common::proto::TextResponse {
    fn from(value: TextResponse) -> Self {
        feedback_fusion_common::proto::TextResponse { text: value.text }
    }
}

impl From<feedback_fusion_common::proto::TextResponse> for TextResponse {
    fn from(value: feedback_fusion_common::proto::TextResponse) -> Self {
        TextResponse { text: value.text }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RatingResponse {
    rating: u8,
}

impl From<RatingResponse> for feedback_fusion_common::proto::RatingResponse {
    fn from(value: RatingResponse) -> Self {
        feedback_fusion_common::proto::RatingResponse {
            rating: value.rating.into(),
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

impl From<CheckboxResponse> for feedback_fusion_common::proto::CheckboxResponse {
    fn from(value: CheckboxResponse) -> Self {
        feedback_fusion_common::proto::CheckboxResponse {
            checked: value.checked,
        }
    }
}

impl From<feedback_fusion_common::proto::CheckboxResponse> for CheckboxResponse {
    fn from(value: feedback_fusion_common::proto::CheckboxResponse) -> Self {
        CheckboxResponse {
            checked: value.checked,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SelectionResponse {
    values: Vec<String>,
}

impl From<SelectionResponse> for feedback_fusion_common::proto::SelectionResponse {
    fn from(value: SelectionResponse) -> Self {
        feedback_fusion_common::proto::SelectionResponse {
            values: value.values,
        }
    }
}

impl From<feedback_fusion_common::proto::SelectionResponse> for SelectionResponse {
    fn from(value: feedback_fusion_common::proto::SelectionResponse) -> Self {
        SelectionResponse {
            values: value.values,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RangeResponse {
    start: u8,
    end: u8,
}

impl From<RangeResponse> for feedback_fusion_common::proto::RangeResponse {
    fn from(value: RangeResponse) -> Self {
        feedback_fusion_common::proto::RangeResponse {
            start: value.start.into(),
            end: value.end.into(),
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

impl From<NumberResponse> for feedback_fusion_common::proto::NumberResponse {
    fn from(value: NumberResponse) -> Self {
        feedback_fusion_common::proto::NumberResponse {
            number: value.number.into(),
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
