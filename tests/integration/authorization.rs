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

use test_log::test;

use feedback_fusion_common::{
    connect,
    proto::{
        AuthorizationGrant, AuthorizationType, CreateResourceAuthorizationRequest,
        CreateTargetRequest, DeleteResourceAuthorizationRequest,
        ExportResourceAuthorizationsRequest, GetResourceAuthorizationRequest,
        GetResourceAuthorizationsRequest, GetTargetRequest, ResourceAuthorizationData,
        ResourceKind,
    },
};

fn create_authorization(id: &str) -> CreateResourceAuthorizationRequest {
    CreateResourceAuthorizationRequest {
        resource_id: vec![id.to_string()],
        resource_kind: ResourceKind::ResourceTarget.into(),
        authorization_data: Some(ResourceAuthorizationData {
            r#type: AuthorizationType::TypeScope.into(),
            grant: vec![AuthorizationGrant::All.into()],
            values: vec!["dynamic".to_owned()],
        }),
    }
}

#[test(tokio::test)]
async fn test_create() {
    let (mut client, _) = connect!();

    let request = create_authorization("foo");
    let response = client.create_resource_authorization(request).await;

    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert_eq!(response.authorizations.len(), 1);

    let authorization = response.authorizations.first().unwrap();
    assert_eq!(authorization.resource_kind, 0);
    assert_eq!(authorization.resource_id(), "foo");
    assert_eq!(
        authorization.authorization_type(),
        AuthorizationType::TypeScope
    );
    assert_eq!(
        authorization.authorization_grant(),
        AuthorizationGrant::All.into()
    );
    assert_eq!(authorization.value, "dynamic");
}

#[test(tokio::test)]
async fn test_get() {
    let (mut client, _) = connect!();

    let request = create_authorization("foo");
    let response = client.create_resource_authorization(request).await;

    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    let authorization = response.authorizations.first().unwrap();

    let response = client
        .get_resource_authorization(GetResourceAuthorizationRequest {
            id: authorization.id.clone(),
        })
        .await;
    assert!(response.is_ok());
    assert_eq!(&response.unwrap().into_inner(), authorization);
}

#[test(tokio::test)]
async fn test_get_list() {
    let (mut client, _) = connect!();

    let request = create_authorization("foo");
    let response = client.create_resource_authorization(request).await;
    assert!(response.is_ok());

    let response = client
        .get_resource_authorizations(GetResourceAuthorizationsRequest::default())
        .await;
    assert!(response.is_ok());
    assert!(!response.unwrap().into_inner().authorizations.is_empty());
}

#[test(tokio::test)]
async fn test_export() {
    let (mut client, _) = connect!();

    let request = create_authorization("foo");
    let response = client.create_resource_authorization(request).await;
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    let authorization = response.authorizations.first().unwrap();

    let response = client
        .export_resource_authorizations(ExportResourceAuthorizationsRequest {
            ids: vec![authorization.id.clone()],
        })
        .await;
    assert!(response.is_ok());
    let export = response.unwrap().into_inner().export;
    assert_eq!(export, include_str!("export.hcl"));
}

#[test(tokio::test)]
async fn test_delete() {
    let (mut client, _) = connect!();

    let request = create_authorization("foo");
    let response = client.create_resource_authorization(request).await;

    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    let authorization = response.authorizations.first().unwrap();

    let response = client
        .delete_resource_authorization(DeleteResourceAuthorizationRequest {
            id: authorization.id.clone(),
        })
        .await;
    assert!(response.is_ok());
}

#[test(tokio::test)]
async fn test_resource_authorization_actually_works() {
    let (mut client, _, mut scope_client) = connect!(dynamic);

    let response = client
        .create_target(CreateTargetRequest {
            name: "Target".to_owned(),
            description: Some("Description".to_owned()),
        })
        .await;
    assert!(response.is_ok());
    let target = response.unwrap().into_inner();

    let response = scope_client
        .get_target(GetTargetRequest {
            id: target.id.clone(),
        })
        .await;
    assert!(response.is_err());

    let request = create_authorization(target.id.as_str());
    let response = client.create_resource_authorization(request).await;
    assert!(response.is_ok());
    let authorization = response.unwrap().into_inner().authorizations;
    let authorization = authorization.first();

    let response = scope_client
        .get_target(GetTargetRequest {
            id: target.id.clone(),
        })
        .await;
    assert!(response.is_ok());

    let response = client
        .delete_resource_authorization(DeleteResourceAuthorizationRequest {
            id: authorization.unwrap().id.clone(),
        })
        .await;
    assert!(response.is_ok());

    let response = scope_client
        .get_target(GetTargetRequest {
            id: target.id.clone(),
        })
        .await;
    assert!(response.is_err());
}
