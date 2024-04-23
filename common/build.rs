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
        .type_attribute("CreateTargetRequest", "#[derive(Validate)]")
        .field_attribute("CreateTargetRequest.name", "#[validate(length(max = 255))]")
        .type_attribute("UpdateTargetRequest", "#[derive(Validate)]")
        .field_attribute("UpdateTargetRequest.name", "#[validate(length(max = 255))]")
        .field_attribute(
            "UpdateTargetRequest.description",
            "#[validate(length(max = 255))]",
        )
        .type_attribute("CreatePromptRequest", "#[derive(Validate)]")
        .field_attribute("CreatePromptRequest.title", "#[validate(length(max = 32))]")
        .field_attribute(
            "UpdatePromptRequest.description",
            "#[validate(length(max = 255))]",
        )
        .type_attribute("UpdatePromptRequest", "#[derive(Validate)]")
        .field_attribute("UpdatePromptRequest.title", "#[validate(length(max = 32))]")
        .field_attribute(
            "CreatePromptRequest.description",
            "#[validate(length(max = 255))]",
        )
        .type_attribute("CreateFieldRequest", "#[derive(Validate)]")
        .field_attribute("CreateFieldRequest.title", "#[validate(length(max = 32))]")
        .field_attribute(
            "CreateFieldRequest.description",
            "#[validate(length(max = 255))]",
        )
        .type_attribute("UpdateFieldRequest", "#[derive(Validate)]")
        .field_attribute("UpdateFieldRequest.title", "#[validate(length(max = 32))]")
        .field_attribute(
            "UpdateFieldRequest.description",
            "#[validate(length(max = 255))]",
        )
        .file_descriptor_set_path(out_dir.join("feedback-fusion-v1-descriptor.bin"))
        .compile(&["../proto/feedback-fusion-v1.proto"], &["../proto"])
        .unwrap();
    Ok(())
}
