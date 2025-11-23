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

use std::time::Duration;

use feedback_fusion_common::{
    common::ProtoResourceKind,
    proto::{
        CreateTargetRequest, GetAuditVersionsRequest, GetTargetRequest, RollbackResourceRequest,
        UpdateTargetRequest, proto_resource::Inner,
    },
    tests::VerifyAudit,
    verify_audit_exists,
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
async fn test_save() {
    let (mut client, _) = connect!();

    let target = client
        .create_target(create_request())
        .await
        .unwrap()
        .into_inner();

    // as we can't gurantee the audit log to exist after a given time period we just retry and hope
    // we can get a response after idk 10 cycles?
    let mut i = 0;
    while i < 50 {
        let audit_versions = client
            .get_audit_versions(GetAuditVersionsRequest {
                page_size: 10,
                page_token: 1,
                resource_id: target.id.clone(),
                resource_type: ProtoResourceKind::Target as i32,
            })
            .await;
        if let Ok(response) = audit_versions {
            let data = response.into_inner();

            if data.audit_versions.is_empty() {
                i += 1;
                tokio::time::sleep(Duration::from_millis(300)).await;

                continue;
            }

            assert_eq!(data.total, 1i32);
            assert_eq!(data.page_token, 1i32);
            assert_eq!(data.audit_versions.len(), 1);

            let first = data.audit_versions.first().unwrap();
            assert_eq!(first.resource_id.as_str(), target.id.as_str());
            assert_eq!(first.resource_type, ProtoResourceKind::Target as i32);
            assert!(first.data.is_some());

            let data = first.data.as_ref().unwrap();
            assert!(data.inner.is_some());

            let inner = data.inner.as_ref().unwrap();
            assert!(matches!(inner, Inner::Target(_)));

            if let Inner::Target(version) = inner {
                assert_eq!(target.id.as_str(), version.id.as_str());
                assert_eq!(target.name.as_str(), version.name.as_str());
                assert_eq!(target.description, version.description);

                return;
            } else {
                assert!(false);
            }
        };

        i += 1;
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    assert!(false);
}

#[test(tokio::test)]
async fn test_rollback() {
    let (mut client, _) = connect!();

    let target = client
        .create_target(create_request())
        .await
        .unwrap()
        .into_inner();

    // update the target
    client
        .update_target(UpdateTargetRequest {
            id: target.id.clone(),
            name: Some("FooBar".to_owned()),
            description: Some("FooBarFoo".to_owned()),
        })
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(5000)).await;

    // rollback to version 1
    let response = client
        .rollback_resource(RollbackResourceRequest {
            resource_id: target.id.clone(),
            resource_type: ProtoResourceKind::Target as i32,
            version: 1,
        })
        .await;
    assert!(response.is_ok());

    // fetch the target
    let target = client
        .get_target(GetTargetRequest { id: target.id })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(target.name.as_str(), "Target");
    assert_eq!(target.description.as_ref().unwrap().as_str(), "Description");

    verify_audit_exists!(client, target, Target, Rollback);
}
