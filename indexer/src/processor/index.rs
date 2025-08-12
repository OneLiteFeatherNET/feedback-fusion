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

use feedback_fusion_common::{
    common::ProtoResourceKind,
    database::{
        DatabaseConnection,
        schema::index::{IndexComponent, IndexEntry},
    },
    database_request,
    proto::{
        ProtoEvent, ProtoResource, ProtoResourceModificationOperation, ProtoResourceModifiedEvent,
        proto_event::EventContent, proto_resource::Inner,
    },
};
use gxhash::{HashMap, HashMapExt};
use prost::Message;
use rbatis::{executor::Executor, rbatis_codegen::IntoSql, rbdc::DateTime};
use rbs::{Value, value};

use crate::prelude::*;

pub trait MakeIndexEntries {
    fn make_entries(&self, event: &ProtoResourceModifiedEvent) -> Vec<IndexEntry>;
}

impl MakeIndexEntries for ProtoResource {
    fn make_entries(&self, event: &ProtoResourceModifiedEvent) -> Vec<IndexEntry> {
        match self.inner.as_ref().unwrap() {
            Inner::Field(field) => {
                // we now need to parse the protobuf
                let target_entry = IndexEntry::builder()
                    .key_value(event.id.as_str())
                    .key_type(IndexComponent::Field)
                    .value_type(IndexComponent::Target)
                    .value(String::new())
                    .build();

                let prompt_entry = IndexEntry::builder()
                    .key_value(event.id.as_str())
                    .key_type(IndexComponent::Field)
                    .value_type(IndexComponent::Prompt)
                    .value(field.prompt.clone())
                    .build();

                vec![target_entry, prompt_entry]
            }
            Inner::Prompt(prompt) => {
                let target_entry = IndexEntry::builder()
                    .key_value(event.id.as_str())
                    .key_type(IndexComponent::Prompt)
                    .value_type(IndexComponent::Target)
                    .value(prompt.target.clone())
                    .build();

                vec![target_entry]
            }
            _ => Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct PromptToTarget {
    id: String,
    target: String,
}

#[py_sql(
    "`SELECT target, id FROM prompt`
        ` WHERE id IN `
            ${ids.sql()}"
)]
async fn prompt_to_target(
    rb: &dyn rbatis::executor::Executor,
    ids: &[String],
) -> rbatis::Result<Vec<PromptToTarget>> {
    impled!()
}

/// This does map resource relationships in a 1:1 manner in order to reduce join or select overhead
/// on authorization queries.
///
/// We assume all of the given events are already of the `ResourceModifiedEvent` type
#[instrument(skip_all)]
pub async fn maintain_index(events: &[ProtoEvent], connection: &DatabaseConnection) -> Result<()> {
    // sort them by created at
    let index_entries = events
        .iter()
        .filter_map(|event| {
            let inner = event.event_content.as_ref()?;

            #[allow(irrefutable_let_patterns)]
            if let EventContent::ResourceModifiedEvent(inner_event) = inner {
                let key_type = match inner_event.resource_kind() {
                    ProtoResourceKind::Prompt => IndexComponent::Prompt,
                    ProtoResourceKind::Field => IndexComponent::Field,
                    _ => return None,
                };

                let created_at = DateTime::from_timestamp(
                    event
                        .created_at
                        .map(|timestamp| timestamp.seconds)
                        .unwrap_or(DateTime::now().unix_timestamp()),
                );

                match ProtoResource::decode(inner_event.data.as_slice()) {
                    Ok(resource) => {
                        let entries = resource.make_entries(inner_event);

                        Some((
                            created_at,
                            inner_event.operation(),
                            // TODO: get rid of this clone
                            (key_type, inner_event.id.clone()),
                            entries,
                        ))
                    }
                    Err(error) => {
                        error!("Error while decoding resource bytes: {error:?}");
                        None
                    }
                }
            } else {
                None
            }
        })
        // now sort them in ascending order
        .sorted_by_key(|(timestamp, _, _, _)| timestamp.clone());

    // we will now initialize a new GxHashMap having the combination of key_type and id as key
    let mut entries_by_resource: HashMap<(IndexComponent, String), Vec<IndexEntry>> =
        HashMap::new();

    // this will store all key cominations that we need to delete from the index
    let mut pending_deletion: Vec<Value> = Vec::new();

    // we will now iterate through the entries with ascending timestamp
    index_entries
        .into_iter()
        .for_each(|(_, operation, key, entries)| match operation {
            ProtoResourceModificationOperation::Delete => {
                // on a Delete we will just delete the key if it does exist
                entries_by_resource.remove(&key);

                // and ad it to the pending deletion list
                pending_deletion.push(value!(key.0));
                pending_deletion.push(value!(key.1));
            }
            _ => {
                entries_by_resource.insert(key, entries);
            }
        });

    // TODO: PLEASE FIND A MORE PERFORMANT WAY OF DOING THIS
    // now we will fetch the target references for the fields
    let prompts = entries_by_resource
        .iter()
        .filter(|entry| entry.0.0.eq(&IndexComponent::Field))
        .map(|entry| {
            entry
                .1
                .iter()
                .find(|index_entry| index_entry.value_type().eq(&IndexComponent::Prompt))
                .unwrap()
                .value()
                .clone()
        })
        .collect_vec();

    // start the transaction
    let transaction = feedback_fusion_common::database::transaction(connection).await?;

    if !prompts.is_empty() {
        let prompts_to_target = database_request!(
            prompt_to_target(&transaction, prompts.as_slice()).await,
            "Select target ids"
        )?;
        prompts_to_target.into_iter().for_each(|result| {
            entries_by_resource
                .iter_mut()
                .filter(|entry| {
                    entry.0.0.eq(&IndexComponent::Field)
                        && entry.1.iter().any(|entry| {
                            entry.value_type().eq(&IndexComponent::Prompt)
                                && entry.value().as_str().eq(result.id.as_str())
                        })
                })
                .for_each(|entry| {
                    let mut entry = entry
                        .1
                        .iter_mut()
                        .find(|entry| entry.value_type().eq(&IndexComponent::Target));

                    if let Some(entry) = entry.as_mut() {
                        entry.set_value(result.target.clone());
                    }
                });
        });
    }

    // delete the items marked for deletion
    if !pending_deletion.is_empty() {
        let selector = (0..pending_deletion.len())
            .map(|_| "(key_type = ? AND key_value = ?)")
            .collect_vec()
            .join(" OR ");

        let query = format!("DELETE FROM index_entry WHERE {selector}");
        database_request!(
            transaction.query(&query, pending_deletion).await,
            "Remove deleted resources from index"
        )?;
    }

    // now we can save all the entries
    let entries = entries_by_resource.into_values().flatten().collect_vec();
    if !entries.is_empty() {
        database_request!(
            IndexEntry::insert_batch(&transaction, entries.as_slice(), entries.len() as u64).await,
            "Insert entries"
        )?;
    }

    transaction.commit().await?;

    Ok(())
}
