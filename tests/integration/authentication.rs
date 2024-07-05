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
    feedback_fusion_v1_client::FeedbackFusionV1Client, CreateFieldRequest, CreatePromptRequest,
    CreateTargetRequest, DeleteFieldRequest, DeletePromptRequest, DeleteTargetRequest,
    GetFieldsRequest, GetPromptsRequest, GetResponsesRequest, GetTargetRequest, GetTargetsRequest,
    UpdateFieldRequest, UpdatePromptRequest, UpdateTargetRequest,
};
use test_log::test;
use tonic::{Code, Request};

macro_rules! test_authentication {
    ($request:path, $method:ident) => {
        paste::paste! {
            #[test(tokio::test)]
            async fn [<test_$method>]() {
                let mut client = FeedbackFusionV1Client::connect(crate::common::GRPC_ENDPOINT)
                    .await
                    .unwrap();

                let request = Request::new($request::default());
                let response = client.$method(request).await;
                assert!(response.is_err_and(|status| status.code().eq(&Code::Unauthenticated)));
            }
        }
    };
}

test_authentication!(CreateTargetRequest, create_target);
test_authentication!(GetTargetRequest, get_target);
test_authentication!(GetTargetsRequest, get_targets);
test_authentication!(UpdateTargetRequest, update_target);
test_authentication!(DeleteTargetRequest, delete_target);

test_authentication!(CreatePromptRequest, create_prompt);
test_authentication!(GetPromptsRequest, get_prompts);
test_authentication!(UpdatePromptRequest, update_prompt);
test_authentication!(DeletePromptRequest, delete_prompt);

test_authentication!(CreateFieldRequest, create_field);
test_authentication!(GetFieldsRequest, get_fields);
test_authentication!(UpdateFieldRequest, update_field);
test_authentication!(DeleteFieldRequest, delete_field);

test_authentication!(GetResponsesRequest, get_responses);
