//SPDX-FileCopyrightText: 2024 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2025 OneLiteFeatherNet

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

use feedback_fusion_common::proto::{DataExportRequest, DataExportResponse};
use v1::FeedbackFusionV1Context;

use crate::{
    database::schema::feedback::{Field, Prompt, Target},
    prelude::*,
};

pub async fn export_data(
    context: &FeedbackFusionV1Context,
    request: Request<DataExportRequest>,
) -> Result<Response<DataExportResponse>> {
    let data = request.into_inner();
    let connection = context.connection();

    let targets: Vec<Target> = database_request!(
        Target::select_in_column(connection, "id", data.targets.as_slice()).await,
        "Fetch Targets"
    )?;

    let prompts: Vec<Prompt> = database_request!(
        Prompt::select_in_column(connection, "target", data.targets.as_slice()).await,
        "Fetch Prompts"
    )?;

    let fields: Vec<Field> = database_request!(
        Field::select_in_column(
            connection,
            "prompt",
            prompts
                .iter()
                .map(|prompt| prompt.id())
                .collect::<Vec<_>>()
                .as_slice()
        )
        .await,
        "Fetch Fields"
    )?;
    Ok(Response::new(DataExportResponse {
        export: serde_yaml::to_string(&serde_json::json!({
            "preset": serde_json::json!({ "targets": targets
            .into_iter()
            .map(|target| serde_json::json!({
                "id": target.id(),
                "name": target.name(),
                "description": target.description(),
                "prompts": prompts.iter().filter(|prompt| prompt.target().eq(target.id())).map(
                    |prompt| serde_json::json!({
                        "id": prompt.id(),
                        "title": prompt.title(),
                        "description": prompt.description(),
                        "active": prompt.active(),
                        "fields": fields.iter().filter(|field| field.prompt().eq(prompt.id()))
                            .map(
                                |field| serde_json::json!({
                                    "id": field.id(),
                                    "title": field.title(),
                                    "description": field.description(),
                                    "field_type": field.field_type(),
                                    "options": field.options()
                                })
                            ).collect::<Vec<_>>()
                    })
                ).collect::<Vec<_>>()
            }))
            .collect::<Vec<_>>()})
        }))
        .unwrap(),
    }))
}
