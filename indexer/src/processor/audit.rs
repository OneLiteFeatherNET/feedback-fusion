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

use crate::prelude::*;
use feedback_fusion_common::{
    database::{
        DatabaseConnection,
        schema::audit::{AuditAction, AuditResource, AuditVersion},
    },
    database_request,
    event::{Event, event::EventContent},
};
use rbatis::rbdc::DateTime;

#[derive(Deserialize, Getters, Debug)]
#[get = "pub"]
struct MaxVersionQuery {
    resource_id: String,
    resource_type: AuditResource,
    max_version: String,
}

/// Create the audit versions for the given events.
///
/// We assume the events are already filtered and are all `ResourceModifiedEvent` variants
#[instrument(skip_all)]
pub async fn create_audit_versions(
    events: &[Event],
    connection: &DatabaseConnection,
) -> Result<()> {
    // filter the versions and map them to a correct auditversion
    let audit_versions = events.iter().filter_map(|event| {
        let inner = event.event_content.as_ref()?;

        #[allow(irrefutable_let_patterns)]
        if let EventContent::ResourceModifiedEvent(inner_event) = inner {
            let action = match inner_event.operation() {
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

            let resource_type = match inner_event.resource_kind() {
                feedback_fusion_common::event::ResourceKind::Target => AuditResource::Target,
                feedback_fusion_common::event::ResourceKind::Prompt => AuditResource::Prompt,
                feedback_fusion_common::event::ResourceKind::Field => AuditResource::Field,
            };

            let created_at = DateTime::from_timestamp(
                event
                    .created_at
                    .map(|timestamp| timestamp.seconds)
                    .unwrap_or(DateTime::now().unix_timestamp()),
            );

            let audit_version = AuditVersion::builder()
                .version(1u32)
                .action(action)
                .resource_type(resource_type)
                .resource_id(inner_event.id.clone())
                .data(inner_event.data.clone())
                .created_at(created_at.clone())
                .build();

            Some(audit_version)
        } else {
            None
        }
    });

    // sort them by the key (resource_kind, resource_id) annd chunk them afterwards by the very
    // same key
    let mut grouped_by_key = audit_versions
        .sorted_by(|x, y| {
            (x.resource_type(), x.resource_id(), x.created_at()).cmp(&(
                y.resource_type(),
                y.resource_id(),
                x.created_at(),
            ))
        })
        .chunk_by(|audit_version| {
            (
                audit_version.resource_type().clone(),
                audit_version.resource_id().clone(),
            )
        })
        .into_iter()
        .map(|(k, v)| (k, v.collect_vec()))
        .collect_vec();

    // create the selector
    let selector = (0..grouped_by_key.len())
        .map(|_| "(resource_type = ? AND resource_id = ?)")
        .collect_vec()
        .join(" OR ");

    // parse the arguments, so all (resource_type, resource_id)
    let arguments = grouped_by_key
        .iter()
        .map(|(k, _)| vec![value!(k.0.clone()), value!(k.1.clone())])
        .flatten()
        .collect_vec();

    let query = format!(
        "SELECT resource_id, resource_type, MAX(version) AS max_version FROM audit_version WHERE {selector} GROUP BY resource_id, resource_type"
    );

    let mut result = database_request!(
        connection
            .query_decode::<Vec<MaxVersionQuery>>(&query, arguments)
            .await,
        "Select prior versions"
    )?;

    if result.is_empty() {
        result = vec![MaxVersionQuery {
            resource_type: AuditResource::Target,
            resource_id: "".to_owned(),
            max_version: "0".to_owned(),
        }]
    }

    // sort these keys the same way
    let latest_version_by_key = result.into_iter().sorted_by(|x, y| {
        (x.resource_type(), x.resource_id()).cmp(&(y.resource_type(), y.resource_id()))
    });

    // start a mutable iterator of the audit versions
    let mut audit_versions_by_key = grouped_by_key.iter_mut();
    // iterate over the latest versions
    latest_version_by_key
        .into_iter()
        .for_each(|latest_version| {
            loop {
                // check wether the next version group is a match.
                // If it is a match then a previous audit record seems to have existed otherwise due to
                // the sorted keys we now fure sure that it will be the first audit log version.
                //
                // We do this until we can finally find a match
                let next = audit_versions_by_key.next();
                if let Some(next) = next {
                    let key = &next.0;
                    let audit_versions = &mut next.1;
                    let mut is_match = false;

                    let latest_version = {
                        if latest_version.resource_type().eq(&key.0)
                            && latest_version.resource_id().eq(&key.1)
                        {
                            is_match = true;

                            latest_version.max_version.parse().unwrap()
                        } else {
                            0
                        }
                    } + 1;

                    audit_versions
                        .iter_mut()
                        .enumerate()
                        .for_each(|(index, audit_version)| {
                            audit_version.set_version(latest_version + index as u32);
                        });

                    if is_match {
                        break;
                    }

                    // now we can mut iter the versions and set the version to latest_version + 1 +
                    // index
                } else {
                    break;
                }
            }
        });

    let audit_versions = grouped_by_key
        .into_iter()
        .map(|(_, v)| v)
        .flatten()
        .collect_vec();

    // persist result
    database_request!(
        AuditVersion::insert_batch(
            connection,
            audit_versions.as_slice(),
            audit_versions.len() as u64
        )
        .await,
        "Insert audits"
    )?;

    Ok(())
}
