//SPDX-FileCopyrightText: 2023 OneLiteFeatherNet
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

use rbatis::rbatis_codegen::IntoSql;
use std::collections::BTreeSet;

use crate::prelude::*;
use aliri_oauth2::scope::ScopeTokenRef;
use rbatis::rbdc::DateTime;

use super::date_time_to_timestamp;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, strum_macros::Display, Hash, Eq)]
pub enum ResourceKind {
    Target,
    Prompt,
    Field,
    // references a target
    Export,
    // references a target
    Authorize,
    // references a target
    Response,
}

impl PartialEq<Endpoint<'_>> for ResourceKind {
    fn eq(&self, other: &Endpoint<'_>) -> bool {
        match *self {
            Self::Target => matches!(other, Endpoint::Target(_)),
            Self::Prompt => matches!(other, Endpoint::Prompt(_)),
            Self::Field => matches!(other, Endpoint::Field(_)),
            Self::Export => matches!(other, Endpoint::Export(_)),
            Self::Authorize => matches!(other, Endpoint::Authorize(_)),
            Self::Response => matches!(other, Endpoint::Response(_)),
        }
    }
}

impl From<&feedback_fusion_common::proto::ResourceKind> for ResourceKind {
    fn from(value: &feedback_fusion_common::proto::ResourceKind) -> Self {
        match value {
            feedback_fusion_common::proto::ResourceKind::ResourceTarget => Self::Target,
            feedback_fusion_common::proto::ResourceKind::ResourcePrompt => Self::Prompt,
            feedback_fusion_common::proto::ResourceKind::ResourceField => Self::Field,
            feedback_fusion_common::proto::ResourceKind::ResourceExport => Self::Export,
            feedback_fusion_common::proto::ResourceKind::ResourceAuthorize => Self::Authorize,
            feedback_fusion_common::proto::ResourceKind::ResourceResponse => Self::Response,
        }
    }
}

impl From<ResourceKind> for i32 {
    fn from(value: ResourceKind) -> Self {
        match value {
            ResourceKind::Target => {
                feedback_fusion_common::proto::ResourceKind::ResourceTarget.into()
            }
            ResourceKind::Prompt => {
                feedback_fusion_common::proto::ResourceKind::ResourcePrompt.into()
            }
            ResourceKind::Field => {
                feedback_fusion_common::proto::ResourceKind::ResourceField.into()
            }
            ResourceKind::Export => {
                feedback_fusion_common::proto::ResourceKind::ResourceExport.into()
            }
            ResourceKind::Authorize => {
                feedback_fusion_common::proto::ResourceKind::ResourceAuthorize.into()
            }
            ResourceKind::Response => {
                feedback_fusion_common::proto::ResourceKind::ResourceResponse.into()
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum ResourceAuthorizationType {
    Scope,
    Group,
    Subject,
}

impl From<&feedback_fusion_common::proto::AuthorizationType> for ResourceAuthorizationType {
    fn from(value: &feedback_fusion_common::proto::AuthorizationType) -> Self {
        match value {
            feedback_fusion_common::proto::AuthorizationType::TypeSubject => Self::Subject,
            feedback_fusion_common::proto::AuthorizationType::TypeScope => Self::Scope,
            feedback_fusion_common::proto::AuthorizationType::TypeGroup => Self::Group,
        }
    }
}

impl From<ResourceAuthorizationType> for i32 {
    fn from(value: ResourceAuthorizationType) -> Self {
        match value {
            ResourceAuthorizationType::Scope => {
                feedback_fusion_common::proto::AuthorizationType::TypeScope.into()
            }
            ResourceAuthorizationType::Group => {
                feedback_fusion_common::proto::AuthorizationType::TypeGroup.into()
            }
            ResourceAuthorizationType::Subject => {
                feedback_fusion_common::proto::AuthorizationType::TypeSubject.into()
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, strum_macros::Display, Hash, Eq)]
pub enum ResourceAuthorizationGrant {
    Read,
    Write,
    List,
    All,
}

impl From<&feedback_fusion_common::proto::AuthorizationGrant> for ResourceAuthorizationGrant {
    fn from(value: &feedback_fusion_common::proto::AuthorizationGrant) -> Self {
        match value {
            feedback_fusion_common::proto::AuthorizationGrant::Read => Self::Read,
            feedback_fusion_common::proto::AuthorizationGrant::Write => Self::Write,
            feedback_fusion_common::proto::AuthorizationGrant::List => Self::List,
            feedback_fusion_common::proto::AuthorizationGrant::All => Self::All,
        }
    }
}

impl PartialEq<Permission> for ResourceAuthorizationGrant {
    fn eq(&self, other: &Permission) -> bool {
        match *self {
            Self::Write => matches!(other, Permission::Write),
            Self::Read => matches!(other, Permission::Read),
            Self::List => matches!(other, Permission::List),
            Self::All => matches!(other, Permission::All),
        }
    }
}

impl From<ResourceAuthorizationGrant> for i32 {
    fn from(value: ResourceAuthorizationGrant) -> Self {
        match value {
            ResourceAuthorizationGrant::Read => {
                feedback_fusion_common::proto::AuthorizationGrant::Read.into()
            }
            ResourceAuthorizationGrant::Write => {
                feedback_fusion_common::proto::AuthorizationGrant::Write.into()
            }
            ResourceAuthorizationGrant::List => {
                feedback_fusion_common::proto::AuthorizationGrant::List.into()
            }
            ResourceAuthorizationGrant::All => {
                feedback_fusion_common::proto::AuthorizationGrant::All.into()
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Derivative, Debug, Getters, Setters, TypedBuilder)]
#[derivative(PartialEq)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct ResourceAuthorization {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    resource_kind: ResourceKind,
    resource_id: Option<String>,
    authorization_type: ResourceAuthorizationType,
    authorization_grant: ResourceAuthorizationGrant,
    authorization_value: String,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    updated_at: DateTime,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    created_at: DateTime,
}

impl From<ResourceAuthorization> for feedback_fusion_common::proto::ResourceAuthorization {
    fn from(value: ResourceAuthorization) -> Self {
        feedback_fusion_common::proto::ResourceAuthorization {
            id: value.id,
            resource_kind: value.resource_kind.into(),
            resource_id: value.resource_id,
            authorization_type: value.authorization_type.into(),
            authorization_grant: value.authorization_grant.into(),
            value: value.authorization_value,
            updated_at: Some(date_time_to_timestamp(value.updated_at)),
            created_at: Some(date_time_to_timestamp(value.created_at)),
        }
    }
}

impl std::fmt::Display for ResourceAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(inner) = &self.resource_id {
            write!(
                f,
                "{}::{}::{}",
                self.resource_kind(),
                inner,
                self.authorization_grant()
            )
        } else {
            write!(
                f,
                "{}::{}",
                self.resource_kind(),
                self.authorization_grant()
            )
        }
    }
}

crud!(ResourceAuthorization {});
impl_select!(ResourceAuthorization {select_matching(scopes: &BTreeSet<&ScopeTokenRef>, groups: &BTreeSet<&String>, subject: &str) => "`WHERE (authorization_type = 'Scope' AND authorization_value IN ${scopes.sql()}) OR (authorization_type = 'Group' AND authorization_value IN ${groups.sql()}) OR (authorization_type = 'Subject' AND authorization_value = #{subject})`"});
impl_select!(ResourceAuthorization {select_by_id(id: &str) -> Option => "`WHERE id = #{id}`"});
impl_select!(ResourceAuthorization {select_by_ids(ids: &[String]) => "`WHERE id IN ${id.sql()}`"});
impl_select_page_wrapper!(ResourceAuthorization {select_page() => "``"});

pub struct Authorization<'a>(pub &'a Endpoint<'a>, pub &'a Permission);

impl std::fmt::Display for Authorization<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Endpoint::Target(EndpointScopeSelector::Specific(inner))
        | Endpoint::Prompt(EndpointScopeSelector::Specific(inner))
        | Endpoint::Field(EndpointScopeSelector::Specific(inner)) = &self.0
        {
            write!(f, "{}::{}::{}", self.0, inner, self.1)
        } else {
            write!(f, "{}::{}", self.0, self.1)
        }
    }
}
