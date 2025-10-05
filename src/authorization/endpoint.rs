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

use std::borrow::Cow;

use async_recursion::async_recursion;
use strum_macros::{Display, IntoStaticStr};

use crate::{
    authorization::{get_target_ids_by_field_ids, get_target_ids_by_prompt_ids},
    database::schema::authorization::ResourceKind,
    prelude::*,
};

/// This limits the access to the specified endpoint to the given scope.
///
/// We can use this to only allow access to the endpoint for specific element ids
/// and similar.
#[derive(
    Hash,
    PartialEq,
    Eq,
    Deserialize,
    Debug,
    Clone,
    Display,
    IntoStaticStr,
    Serialize,
    Ord,
    PartialOrd,
    Default
)]
pub enum EndpointScopeSelector<'a> {
    /// unscoped access
    #[default]
    All,
    /// access only for a specific id
    Specific(Cow<'a, str>),
    /// access for multiple ids
    Multiple(Vec<Cow<'a, str>>),
    /// custom wildcard format
    Wildcard(Cow<'a, str>),
}

impl<'a> EndpointScopeSelector<'a> {
    pub fn with_resource_kind(self, resource_kind: &ResourceKind) -> Endpoint<'a> {
        match resource_kind {
            ResourceKind::Target => Endpoint::Target(self),
            ResourceKind::Prompt => Endpoint::Prompt(self),
            ResourceKind::Field => Endpoint::Field(self),
            ResourceKind::Response => Endpoint::Response(self),
            ResourceKind::Export => Endpoint::Export(self),
            ResourceKind::Authorize => Endpoint::Authorize(None),
        }
    }
}

/// Does define a scope for the client authorization with in the given endpoint group.
///
/// You can limit the access to the specified endpoint by defining a `EndpointScopeSelector`.
/// If you wish to use wildcards you should use the `Custom` variant.
#[derive(Hash, PartialEq, Eq, Deserialize, Debug, Clone, Display, IntoStaticStr, Serialize)]
pub enum Endpoint<'a> {
    Target(EndpointScopeSelector<'a>),
    Prompt(EndpointScopeSelector<'a>),
    Field(EndpointScopeSelector<'a>),
    Response(EndpointScopeSelector<'a>),
    Export(EndpointScopeSelector<'a>),
    /// this is always target based
    Authorize(Option<Box<Endpoint<'a>>>),
    // this can contain a wildcard definition
    Custom(Cow<'a, str>, EndpointScopeSelector<'a>),
}

impl Endpoint<'_> {
    pub fn none(&self) -> Endpoint<'_> {
        match self {
            Endpoint::Target(_) => Endpoint::Target(EndpointScopeSelector::default()),
            Endpoint::Prompt(_) => Endpoint::Prompt(EndpointScopeSelector::default()),
            Endpoint::Field(_) => Endpoint::Field(EndpointScopeSelector::default()),
            Endpoint::Response(_) => Endpoint::Response(EndpointScopeSelector::default()),
            Endpoint::Export(_) => Endpoint::Export(EndpointScopeSelector::default()),
            Endpoint::Authorize(_) => Endpoint::Authorize(None),
            Endpoint::Custom(wildcard, _) => {
                Endpoint::Custom(wildcard.clone(), EndpointScopeSelector::default())
            }
        }
    }
}

impl Endpoint<'_> {
    #[instrument(skip_all)]
    #[async_recursion]
    pub async fn get_target_ids(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<Option<Vec<String>>> {
        match self {
            Endpoint::Target(EndpointScopeSelector::Specific(id))
            | Endpoint::Export(EndpointScopeSelector::Specific(id)) => {
                Ok(Some(vec![id.to_string()]))
            }
            Endpoint::Prompt(EndpointScopeSelector::Specific(id))
            | Endpoint::Response(EndpointScopeSelector::Specific(id)) => Ok(Some(
                get_target_ids_by_prompt_ids(connection, &[id])
                    .await?
                    .into_iter()
                    .map(|result| result.result)
                    .collect(),
            )),
            Endpoint::Field(EndpointScopeSelector::Specific(id)) => Ok(Some(
                get_target_ids_by_field_ids(connection, &[id])
                    .await?
                    .into_iter()
                    .map(|result| result.result)
                    .collect(),
            )),
            Endpoint::Target(EndpointScopeSelector::Multiple(ids))
            | Endpoint::Export(EndpointScopeSelector::Multiple(ids)) => {
                Ok(Some(ids.iter().map(|id| id.to_string()).collect()))
            }
            Endpoint::Prompt(EndpointScopeSelector::Multiple(ids))
            | Endpoint::Response(EndpointScopeSelector::Multiple(ids)) => {
                let id_refs: Vec<&str> = ids.iter().map(|id| id.as_ref()).collect();
                Ok(Some(
                    get_target_ids_by_prompt_ids(connection, &id_refs)
                        .await?
                        .into_iter()
                        .map(|result| result.result)
                        .collect(),
                ))
            }
            Endpoint::Field(EndpointScopeSelector::Multiple(ids)) => {
                let id_refs: Vec<&str> = ids.iter().map(|id| id.as_ref()).collect();
                Ok(Some(
                    get_target_ids_by_field_ids(connection, &id_refs)
                        .await?
                        .into_iter()
                        .map(|result| result.result)
                        .collect(),
                ))
            }
            Endpoint::Authorize(Some(boxed_endpoint)) => {
                boxed_endpoint.get_target_ids(connection).await
            }
            _ => Ok(None),
        }
    }
}
