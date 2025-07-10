//SPDX-FileCopyrightText: 2025 OneLiteFeatherNet
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

use std::collections::{BTreeMap, BTreeSet};

use crate::prelude::*;
use feedback_fusion_common::{
    database::{
        DatabaseConnection,
        schema::audit::{AuditAction, AuditResource, AuditVersion},
    },
    database_request,
    event::{Event, event::Event as EventType},
};
use rbs::{Value, to_value};

#[derive(Deserialize)]
struct MaxVersionQuery {
    resource_id: String,
    resource_type: AuditResource,
    max_version: u32,
}

/// Create the audit versions for the given events.
///
/// We assume the events are already filtered and are all `ResourceModifiedEvent` variants
#[instrument(skip_all)]
pub async fn create_audit_versions(
    events: &[Event],
    connection: &DatabaseConnection,
) -> Result<()> {
    // transform the events into audits
    let audit_versions: BTreeSet<((String, AuditResource), AuditVersion)> = events
        .iter()
        .filter_map(|event| {
            let inner = event.event.as_ref().unwrap();

            #[allow(irrefutable_let_patterns)]
            if let EventType::ResourceModifiedEvent(event) = inner {
                // map the operation type
                let action = match event.operation() {
                    feedback_fusion_common::event::ResourceModificationOperation::Create => {
                        AuditAction::Create
                    }
                    feedback_fusion_common::event::ResourceModificationOperation::Update => {
                        AuditAction::Update
                    }
                    feedback_fusion_common::event::ResourceModificationOperation::Delete => {
                        AuditAction::Delete
                    }
                };

                // map the resource kind
                let resource_type = match event.resource_kind() {
                    feedback_fusion_common::event::ResourceKind::Target => AuditResource::Target,
                    feedback_fusion_common::event::ResourceKind::Prompt => AuditResource::Prompt,
                    feedback_fusion_common::event::ResourceKind::Field => AuditResource::Field,
                };

                let key = (event.id.clone(), resource_type.clone());

                // we do not need to process any of the remaining data in order to build the version
                // we will evaluate the version number later
                Some((
                    key,
                    AuditVersion::builder()
                        .version(0u32)
                        .action(action)
                        .resource_type(resource_type)
                        .resource_id(event.id.clone())
                        .data(event.data.clone())
                        .build(),
                ))
            } else {
                None
            }
        })
        .collect();

    let selector = (0..audit_versions.len())
        .map(|_| "(?, ?)")
        .collect_vec()
        .join(", ");
    let mut arguments: Vec<Value> = Vec::new();
    audit_versions.iter().for_each(|version| {
        arguments.push(to_value!(version.1.resource_id()));
        arguments.push(to_value!(version.1.resource_type()));
    });

    // TODO: this should not be compatible with mssql?
    let query = format!(
        "SELECT resource_id, resource_type, MAX(version) AS max_version FROM audit_versions WHERE (resource_id, resource_type) IN ({selector}) GROUP BY resource_id, resource_type"
    );
    let result = database_request!(
        connection
            .query_decode::<Vec<MaxVersionQuery>>(&query, arguments)
            .await,
        "Select prior versions"
    )?;

    let version_map: BTreeMap<(String, AuditResource), u32> = result
        .into_iter()
        .map(|row| ((row.resource_id, row.resource_type), row.max_version))
        .collect();

    let finalized_versions: Vec<AuditVersion> = audit_versions
        .into_iter()
        .map(|(key, mut version)| {
            let next = version_map.get(&key).copied().unwrap_or(0) + 1;
            version.set_version(next);

            version
        })
        .collect();

    // save the final result
    database_request!(
        AuditVersion::insert_batch(
            connection,
            finalized_versions.as_slice(),
            finalized_versions.len() as u64
        )
        .await,
        "Insert audits"
    )?;

    Ok(())
}
