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

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .type_attribute(
            "CreateTargetRequest",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "CreatePromptRequest",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "FieldOptions.options",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "FieldOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "FieldType",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "TextOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "RatingOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "CheckboxOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "SelectionOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "RangeOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "NumberOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "CheckboxStyle",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute("CreateTargetRequest", "#[derive(validator::Validate)]")
        .field_attribute(
            "CreateTargetRequest.name",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .field_attribute(
            "CreateTargetRequest.description",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .type_attribute("UpdateTargetRequest", "#[derive(validator::Validate)]")
        .field_attribute(
            "UpdateTargetRequest.name",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .field_attribute(
            "UpdateTargetRequest.description",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .type_attribute("CreatePromptRequest", "#[derive(validator::Validate)]")
        .field_attribute(
            "CreatePromptRequest.title",
            "#[validate(length(min = 1, max = 32), non_control_character)]",
        )
        .field_attribute(
            "UpdatePromptRequest.description",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .type_attribute("UpdatePromptRequest", "#[derive(validator::Validate)]")
        .field_attribute(
            "UpdatePromptRequest.title",
            "#[validate(length(min = 1, max = 32), non_control_character)]",
        )
        .field_attribute(
            "CreatePromptRequest.description",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .type_attribute("CreateFieldRequest", "#[derive(validator::Validate)]")
        .field_attribute(
            "CreateFieldRequest.title",
            "#[validate(length(min = 1, max = 32), non_control_character)]",
        )
        .field_attribute(
            "CreateFieldRequest.description",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .type_attribute("UpdateFieldRequest", "#[derive(validator::Validate)]")
        .field_attribute(
            "UpdateFieldRequest.title",
            "#[validate(length(min = 1, max = 32), non_control_character)]",
        )
        .field_attribute(
            "UpdateFieldRequest.description",
            "#[validate(length(min = 1, max = 255), non_control_character)]",
        )
        .type_attribute(
            "GetTargetsRequest",
            "#[derive(feedback_fusion_codegen::PageRequest)]",
        )
        .type_attribute(
            "GetPromptsRequest",
            "#[derive(feedback_fusion_codegen::PageRequest)]",
        )
        .type_attribute(
            "GetFieldsRequest",
            "#[derive(feedback_fusion_codegen::PageRequest)]",
        )
        .type_attribute(
            "GetResponsesRequest",
            "#[derive(feedback_fusion_codegen::PageRequest)]",
        )
        .file_descriptor_set_path(out_dir.join("feedback-fusion-v1-descriptor.bin"))
        .compile(&["../proto/feedback-fusion-v1.proto"], &["../proto"])
        .unwrap();
    Ok(())
}
