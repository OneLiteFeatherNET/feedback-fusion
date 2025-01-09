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
    CreateTargetRequest, DeleteTargetRequest, GetTargetsRequest, UpdateTargetRequest,
};
use test_log::test;

use feedback_fusion_common::connect;

fn create_request() -> CreateTargetRequest {
    CreateTargetRequest {
        name: "Target".to_owned(),
        description: Some("Description".to_owned()),
    }
}

#[test(tokio::test)]
async fn test_create() {
    let (mut client, _) = connect!();

    let request = create_request();
    let response = client.create_target(request).await;
    assert!(response.is_ok())
}

#[test(tokio::test)]
async fn test_get() {
    let (mut client, _) = connect!();

    let target = client
        .create_target(create_request())
        .await
        .unwrap()
        .into_inner();

    let request = GetTargetsRequest::default();
    let response = client.get_targets(request).await;
    assert!(response.is_ok_and(|response| response
        .into_inner()
        .targets
        .iter()
        .find(|t| t.eq(&&target))
        .is_some()));
}

#[test(tokio::test)]
async fn test_update() {
    let (mut client, _) = connect!();

    let target = client
        .create_target(create_request())
        .await
        .unwrap()
        .into_inner();

    let request = UpdateTargetRequest {
        id: target.id.clone(),
        name: Some("Well".to_owned()),
        ..Default::default()
    };
    let response = client.update_target(request).await;
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert_eq!(target.id.as_str(), response.id.as_str());
    assert_eq!("Well", response.name.as_str());
    assert_eq!(Some("Description".to_owned()), response.description);

    let request = UpdateTargetRequest {
        id: target.id.clone(),
        description: Some("Changed".to_owned()),
        ..Default::default()
    };
    let response = client.update_target(request).await;
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert_eq!(target.id.as_str(), response.id.as_str());
    assert_eq!("Well", response.name.as_str());
    assert_eq!(Some("Changed".to_owned()), response.description);
}

#[test(tokio::test)]
async fn test_delete() {
    let (mut client, _) = connect!();

    let target = client
        .create_target(create_request())
        .await
        .unwrap()
        .into_inner();

    let request = DeleteTargetRequest {
        id: target.id.clone(),
    };
    let response = client.delete_target(request).await;
    assert!(response.is_ok());

    let request = GetTargetsRequest::default();
    let response = client.get_targets(request).await;
    assert!(response.is_ok_and(|response| response
        .into_inner()
        .targets
        .iter()
        .find(|t| t.eq(&&target))
        .is_none()));

    let target = client
        .create_target(create_request())
        .await
        .unwrap()
        .into_inner();

    let target2 = client
        .create_target(create_request())
        .await
        .unwrap()
        .into_inner();

    let request = DeleteTargetRequest {
        id: target.id.clone(),
    };
    let response = client.delete_target(request).await;
    assert!(response.is_ok());

    let request = GetTargetsRequest::default();
    let response = client.get_targets(request).await;
    assert!(response.is_ok_and(|response| {
        let inner = response.into_inner();

        inner.targets.iter().find(|t| t.eq(&&target2)).is_some()
            && inner.targets.iter().find(|t| t.eq(&&target)).is_none()
    }));
}
