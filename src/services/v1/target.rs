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
    CreateTargetRequest, DeleteTargetRequest, GetTargetRequest, GetTargetsRequest,
    Target as ProtoTarget, TargetPage, UpdateTargetRequest,
};

use crate::{database::schema::feedback::Target, prelude::*};

use super::FeedbackFusionV1Context;

pub async fn create_target(
    context: &FeedbackFusionV1Context,
    request: Request<CreateTargetRequest>,
) -> Result<Response<ProtoTarget>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.state.connection();

    let target = Target::builder()
        .name(data.name)
        .description(data.description)
        .build();

    // create the target
    database_request!(Target::insert(connection, &target).await?);

    Ok(Response::new(target.into()))
}

pub async fn get_target(
    context: &FeedbackFusionV1Context,
    request: Request<GetTargetRequest>,
) -> Result<Response<ProtoTarget>> {
    let data = request.into_inner();
    let connection = context.state.connection();

    let target = database_request!(Target::select_by_id(connection, data.id.as_str()).await?);
    match target {
        Some(target) => Ok(Response::new(target.into())),
        None => Err(FeedbackFusionError::BadRequest(
            "Target not found".to_owned(),
        )),
    }
}

pub async fn get_targets(
    context: &FeedbackFusionV1Context,
    request: Request<GetTargetsRequest>,
) -> Result<Response<TargetPage>> {
    let data = request.into_inner();
    let connection = context.state.connection();

    // TODO: write translation macro
    let page = database_request!(
        Target::select_page_wrapper(connection, &pagination.request(), data.query.as_str()).await?
    );
    Ok(Request::new(page.into()))
}

pub async fn update_target(
    context: &FeedbackFusionV1Context,
    request: Request<UpdateTargetRequest>,
) -> Result<Response<Target>> {
    let data = request.into_inner();
    data.validate()?;
    let connection = context.state.connection();

    let mut target =
        database_request!(Target::select_by_id(connection, data.id.as_str())
            .await?
            .ok_or(FeedbackFusionError::BadRequest("not found".to_owned()))?);
    target.set_name(data.name.unwrap_or(target.name().clone()));
    target.set_description(data.description.or(target.description().clone()));

    database_request!(Target::update_by_column(connection, &target, "id").await?);
    Ok(Request::new(target.into()))
}

pub async fn delete_target(
    context: &FeedbackFusionV1Context,
    request: Request<DeleteTargetRequest>,
) -> Result<Response<()>> {
    let data = request.into_inner();
    let connection = self.state.connection();

    database_request!(Target::delete_by_column(connection, "id", data.id.as_str()).await?);
    Ok(Response::new(()))
}
