//SPDX-FileCopyrightText: 2024 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2024 OneLiteFeatherNet

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

use feedback_fusion_common::proto::{
    CheckboxOptions, CreateFieldRequest, CreatePromptRequest, CreateTargetRequest,
    DeleteFieldRequest, FieldOptions, FieldType, GetFieldsRequest, NumberOptions, RangeOptions,
    RatingOptions, SelectionOptions, TextOptions, UpdateFieldRequest,
};
use test_log::test;

use feedback_fusion_common::connect;

fn create_target() -> CreateTargetRequest {
    CreateTargetRequest {
        name: "Target".to_owned(),
        description: Some("Description".to_owned()),
    }
}

fn create_prompt(target: String) -> CreatePromptRequest {
    CreatePromptRequest {
        target,
        title: "Title".to_owned(),
        description: "Description".to_owned(),
        active: true,
    }
}

fn create_field(prompt: String) -> CreateFieldRequest {
    CreateFieldRequest {
        prompt,
        title: "Field".to_owned(),
        description: Some("Description".to_owned()),
        field_type: FieldType::Text.into(),
        options: Some(FieldOptions {
            options: Some(feedback_fusion_common::proto::field_options::Options::Text(
                TextOptions {
                    lines: 1,
                    placeholder: "Placeholder".to_owned(),
                },
            )),
        }),
    }
}

macro_rules! setup {
    ($client:ident, $type:ident) => {
        paste::paste! {
            {
                let target = $client
                    .create_target(create_target())
                    .await
                    .unwrap()
                    .into_inner();

                let prompt = $client
                    .create_prompt(create_prompt(target.id.clone()))
                    .await
                    .unwrap()
                    .into_inner();

                let field = $client
                    .create_field([<create_ $type:lower _field>](prompt.id.clone()))
                    .await
                    .unwrap()
                    .into_inner();

                (target, prompt, field)
            }
        }
    };
}

macro_rules! type_tests {
    ($(($type:ident, $options:expr) $(,)? )*) => {
        paste::paste! {
            $(
                fn [<create_ $type:lower _field>](prompt: String) -> CreateFieldRequest {
                    CreateFieldRequest {
                        prompt,
                        title: "Field $name".to_owned(),
                        description: Some("Description".to_owned()),
                        field_type: FieldType::$type.into(),
                        options: Some(FieldOptions {
                            options: Some($options),
                        }),
                    }
                }

                #[test(tokio::test)]
                async fn [<test_create_ $type:lower>]() {
                    let (mut client, _) = connect!();

                    let target = client
                        .create_target(create_target())
                        .await
                        .unwrap()
                        .into_inner();

                    let prompt = client
                        .create_prompt(create_prompt(target.id.clone()))
                        .await
                        .unwrap()
                        .into_inner();

                    let response = client.create_field([<create_ $type:lower _field>](prompt.id.clone())).await;
                    assert!(response.is_ok());
                }

                #[test(tokio::test)]
                async fn [<test_get_ $type:lower>]() {
                    let (mut client, _) = connect!();

                    let (_, prompt, field) = setup!(client, $type);

                    let response = client
                        .get_fields(GetFieldsRequest {
                            prompt: prompt.id.clone(),
                            ..Default::default()
                        })
                        .await;
                    assert!(response.is_ok_and(|response| response
                        .into_inner()
                        .fields
                        .iter()
                        .find(|f| f.eq(&&field))
                        .is_some()));
                }

                #[test(tokio::test)]
                async fn [<test_update_ $type:lower>]() {
                    let (mut client, _) = connect!();

                    let (_, _, field) = setup!(client, $type);

                    let request = UpdateFieldRequest {
                        id: field.id.clone(),
                        title: Some("Well".to_owned()),
                        ..Default::default()
                    };
                    let response = client.update_field(request).await;
                    assert!(response.is_ok());
                    let response = response.unwrap().into_inner();
                    assert_eq!(&field.id, &response.id);
                    assert_eq!("Well", response.title.as_str());
                    assert_eq!("Description", response.description.unwrap().as_str());

                    let request = UpdateFieldRequest {
                        id: field.id.clone(),
                        description: Some("Done".to_owned()),
                        ..Default::default()
                    };
                    let response = client.update_field(request).await;
                    assert!(response.is_ok());
                    let response = response.unwrap().into_inner();
                    assert_eq!(&field.id, &response.id);
                    assert_eq!("Well", response.title.as_str());
                    assert_eq!("Done", response.description.unwrap().as_str());
                }
            )*
        }
    };
}

type_tests!(
    (
        Text,
        feedback_fusion_common::proto::field_options::Options::Text(TextOptions {
            lines: 1,
            placeholder: "Placeholder".to_owned(),
        })
    ),
    (
        Rating,
        feedback_fusion_common::proto::field_options::Options::Rating(RatingOptions { max: 5 })
    ),
    (
        Checkbox,
        feedback_fusion_common::proto::field_options::Options::Checkbox(CheckboxOptions {
            default_state: false,
            ..Default::default()
        })
    ),
    (
        Selection,
        feedback_fusion_common::proto::field_options::Options::Selection(SelectionOptions {
            multiple: false,
            combobox: false,
            values: vec!["Foo".to_owned()]
        })
    ),
    (
        Range,
        feedback_fusion_common::proto::field_options::Options::Range(RangeOptions {
            min: 5,
            max: 10
        })
    ),
    (
        Number,
        feedback_fusion_common::proto::field_options::Options::Number(NumberOptions {
            min: 5,
            max: 10,
            placeholder: "Placeholder".to_owned()
        })
    )
);

#[test(tokio::test)]
async fn test_delete() {
    let (mut client, _) = connect!();

    let (_, prompt, field) = setup!(client, Text);

    let request = DeleteFieldRequest {
        id: field.id.clone(),
    };
    let response = client.delete_field(request).await;
    assert!(response.is_ok());

    let request = GetFieldsRequest {
        prompt: prompt.id.clone(),
        ..Default::default()
    };
    let response = client.get_fields(request).await;
    assert!(response.is_ok_and(|response| response
        .into_inner()
        .fields
        .iter()
        .find(|f| f.eq(&&field))
        .is_none()));

    let field = client
        .create_field(create_field(prompt.id.clone()))
        .await
        .unwrap()
        .into_inner();

    let field2 = client
        .create_field(create_field(prompt.id.clone()))
        .await
        .unwrap()
        .into_inner();

    let request = DeleteFieldRequest {
        id: field.id.clone(),
    };
    let response = client.delete_field(request).await;
    assert!(response.is_ok());

    let request = GetFieldsRequest {
        prompt: prompt.id,
        ..Default::default()
    };
    let response = client.get_fields(request).await;
    assert!(response.is_ok_and(|response| {
        let inner = response.into_inner();

        inner.fields.iter().find(|t| t.eq(&&field)).is_none()
            && inner.fields.iter().find(|t| t.eq(&&field2)).is_some()
    }));
}
