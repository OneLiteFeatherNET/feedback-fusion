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
        .compile_protos(&["../proto/common/resource.proto"], &["../proto"])
        .unwrap();

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
            "ProtoFieldOptions.options",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoFieldOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoFieldType",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoTextOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoRatingOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoCheckboxOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoSelectionOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoRangeOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "ProtoNumberOptions",
            r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#,
        )
        .type_attribute(
            "protoCheckboxStyle",
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
        .type_attribute(
            "ProtoResourceAuthorizationData",
            "#[derive(validator::Validate)]",
        )
        .field_attribute(
            "ProtoResourceAuthorizationData.values",
            "#[validate(length(min = 1))]",
        )
        .type_attribute(
            "CreateResourceAuthorizationRequest",
            "#[derive(validator::Validate)]",
        )
        .field_attribute(
            "CreateResourceAuthorizationRequest.authorization_data",
            "#[validate(nested)]",
        )
        .type_attribute(
            "GetResourceAuthorizationsRequest",
            "#[derive(feedback_fusion_codegen::PageRequest)]",
        )
        .type_attribute(
            "GetResourceAuthorizationRequest",
            "#[derive(validator::Validate)]",
        )
        .field_attribute(
            "GetResourceAuthorizationRequest.id",
            "#[validate(length(min = 1, max = 32), non_control_character)]",
        )
        .type_attribute(
            "UpdateResourceAuthorizationRequest",
            "#[derive(validator::Validate)]",
        )
        .field_attribute(
            "UpdateResourceAuthorizationRequest.id",
            "#[validate(length(min = 1, max = 32), non_control_character)]",
        )
        .type_attribute(
            "GetAuditVersionsRequest",
            "#[derive(feedback_fusion_codegen::PageRequest)]",
        )
        .file_descriptor_set_path(out_dir.join("feedback-fusion-v1-descriptor.bin"))
        .compile_protos(
            &["../proto/feedback-fusion-v1/service.proto"],
            &["../proto"],
        )
        .unwrap();

    tonic_build::configure()
        .type_attribute("ProtoEvent", "#[derive(Eq, Hash, typed_builder::TypedBuilder)]")
        .type_attribute("ProtoEvent", "#[builder(field_defaults(setter(into)))]")
        .field_attribute("ProtoEvent.created_at", "#[builder(default_code = r#\"Some(prost_types::Timestamp::from(std::time::SystemTime::now()))\"#)]")
        .field_attribute("ProtoEvent.event_type", "#[builder(setter(transform = |event_type: ProtoEventType| event_type as i32))]")
        .type_attribute("ProtoEvent.event_content", "#[derive(Eq, Hash)]")
        .type_attribute("ProtoResourceModifiedEvent", "#[derive(Eq, Hash, typed_builder::TypedBuilder)]")
        .field_attribute("ProtoResourceModifiedEvent.operation", "#[builder(setter(transform = |operation: ProtoResourceModificationOperation| operation as i32))]")
        .field_attribute("ProtoResourceModifiedEvent.resource_kind", "#[builder(setter(transform = |kind: crate::common::ProtoResourceKind| kind as i32))]")
        .field_attribute("ProtoResourceModifiedEvent.data", "#[builder(setter(transform = |message: &impl prost::Message| {let mut buffer = Vec::new();message.encode(&mut buffer).unwrap();buffer}))]")
        .file_descriptor_set_path(out_dir.join("feedback-fusion-event-v1-descriptor.bin"))
        .compile_protos(&["../proto/feedback-fusion-event-v1/service.proto"], &["../proto"])
        .unwrap();

    Ok(())
}
