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

use feedback_fusion_common::{
    proto::{
        CreatePromptRequest, CreateTargetRequest, DeletePromptRequest, GetPromptRequest,
        GetPromptsRequest, UpdatePromptRequest,
    },
    tests::VerifyAudit,
    verify_audit_exists,
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

#[test(tokio::test)]
async fn test_create() {
    let (mut client, _) = connect!();

    let target = client
        .create_target(create_target())
        .await
        .unwrap()
        .into_inner();

    let request = create_prompt(target.id.clone());
    let response = client.create_prompt(request).await;
    assert!(response.is_ok());
    let inner = response.unwrap().into_inner();

    verify_audit_exists!(client, inner, Prompt, Create);
}

#[test(tokio::test)]
async fn test_get() {
    let (mut client, mut public_client) = connect!();

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

    let request = GetPromptRequest {
        id: prompt.id.clone(),
    };
    let response = public_client.get_prompt(request).await;
    assert!(response.is_ok_and(|response| response.into_inner().eq(&prompt)));

    let request = GetPromptsRequest {
        target: target.id,
        ..Default::default()
    };
    let response = client.get_prompts(request).await;
    assert!(response.is_ok_and(|response| {
        response
            .into_inner()
            .prompts
            .iter()
            .find(|p| p.eq(&&prompt))
            .is_some()
    }));
}

#[test(tokio::test)]
async fn test_update() {
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

    let request = UpdatePromptRequest {
        id: prompt.id.clone(),
        active: Some(false),
        description: None,
        title: None,
    };
    let response = client.update_prompt(request).await;
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert_eq!(&prompt.id, &response.id);
    assert_eq!(&false, &response.active);
    assert_eq!("Title", response.title.as_str());
    assert_eq!("Description", response.description.as_str());

    let request = UpdatePromptRequest {
        id: prompt.id.clone(),
        active: None,
        description: Some("Well".to_owned()),
        title: None,
    };
    let response = client.update_prompt(request).await;
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert_eq!(&prompt.id, &response.id);
    assert_eq!(&false, &response.active);
    assert_eq!("Title", response.title.as_str());
    assert_eq!("Well", response.description.as_str());

    let request = UpdatePromptRequest {
        id: prompt.id.clone(),
        active: None,
        description: None,
        title: Some("Done".to_owned()),
    };
    let response = client.update_prompt(request).await;
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert_eq!(&prompt.id, &response.id);
    assert_eq!(&false, &response.active);
    assert_eq!("Done", response.title.as_str());
    assert_eq!("Well", response.description.as_str());

    verify_audit_exists!(client, response, Prompt, Update);
}

#[test(tokio::test)]
async fn test_delete() {
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

    let request = DeletePromptRequest {
        id: prompt.id.clone(),
    };
    let response = client.delete_prompt(request).await;
    assert!(response.is_ok());

    let request = GetPromptsRequest {
        target: target.id.clone(),
        ..Default::default()
    };
    let response = client.get_prompts(request).await;
    assert!(response.is_ok_and(|response| {
        response
            .into_inner()
            .prompts
            .iter()
            .find(|p| p.eq(&&prompt))
            .is_none()
    }));

    let prompt = client
        .create_prompt(create_prompt(target.id.clone()))
        .await
        .unwrap()
        .into_inner();

    let prompt2 = client
        .create_prompt(create_prompt(target.id.clone()))
        .await
        .unwrap()
        .into_inner();

    let request = DeletePromptRequest {
        id: prompt.id.clone(),
    };
    let response = client.delete_prompt(request).await;
    assert!(response.is_ok());

    let request = GetPromptsRequest {
        target: target.id,
        ..Default::default()
    };
    let response = client.get_prompts(request).await;
    assert!(response.is_ok_and(|response| {
        let inner = response.into_inner();

        inner.prompts.iter().find(|t| t.eq(&&prompt)).is_none()
            && inner.prompts.iter().find(|t| t.eq(&&prompt2)).is_some()
    }));

    verify_audit_exists!(client, prompt, Prompt, Delete);
}
