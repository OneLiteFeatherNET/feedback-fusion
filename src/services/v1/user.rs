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

use std::collections::HashMap;

use feedback_fusion_common::proto::UserInfoResponse;
use v1::FeedbackFusionV1Context;

use crate::prelude::*;

pub async fn get_user_info(
    _context: &FeedbackFusionV1Context,
    request: Request<()>,
) -> Result<Response<UserInfoResponse>> {
    // we have the permission matrix in format endpoint, permission -> scopes, groups and therefore
    // we would just have to iterate over the entryset and perform the context authorization for
    // each entry.
    let permissions = PERMISSION_MATRIX
        .clone()
        .into_iter()
        .map(|entry| {
            let (endpoint, permission) = entry.0;
            let identifier = format!("{:?}::{:?}", endpoint, permission);

            if FeedbackFusionV1Context::authorize(&request, endpoint.clone(), permission.clone())
                .is_ok()
            {
                (identifier, true)
            } else {
                (identifier, false)
            }
        })
        .collect::<HashMap<String, bool>>();

    Ok(Response::new(UserInfoResponse { permissions }))
}
