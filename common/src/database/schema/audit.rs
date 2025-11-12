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

use crate::{
    common::ProtoResourceKind,
    database::schema::user::WithUser,
    prelude::*,
    proto::{ProtoAuditAction, ProtoAuditVersion, ProtoResource},
};
use prost::Message;
use rbatis::{
    pysql_select_page,
    rbdc::{Bytes, DateTime},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AuditResource {
    Target,
    Prompt,
    Field,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
    Rollback,
}

#[derive(
    Deserialize,
    Serialize,
    Clone,
    Derivative,
    Debug,
    Getters,
    Setters,
    TypedBuilder,
    Eq,
    PartialOrd,
    Hash,
)]
#[derivative(PartialEq, Ord)]
#[get = "pub"]
#[set = "pub"]
#[builder(field_defaults(setter(into)))]
pub struct AuditVersion {
    #[builder(default_code = r#"nanoid::nanoid!()"#)]
    id: String,
    resource_id: String,
    resource_type: AuditResource,
    action: AuditAction,
    #[derivative(Ord = "ignore")]
    pub data: Bytes,
    version: u32,
    made_by: String,
    #[derivative(PartialEq = "ignore")]
    #[builder(default_code = r#"DateTime::utc()"#)]
    created_at: DateTime,
}

crud!(AuditVersion {});
// yea we actually have to use lowercase select here as rbatis for some reason doesnt accept
// uppercase in the page interceptor...
pysql_select_page!(select_audit_version_page_by_resource_type_and_resource_id(resource_type: &impl Serialize, resource_id: &str) -> WithUser<AuditVersion> => "
    `select `
    if do_count == true:
        `COUNT(1) from audit_version `
    if do_count == false:
        `audit_version.*, oidc_user.id AS oidc_user_id, oidc_user.username AS oidc_user_username from audit_version JOIN oidc_user ON oidc_user.id = made_by `
    `WHERE resource_type = #{resource_type} AND resource_id = #{resource_id} `
    if do_count == false:
        `order by version DESC`");

impl From<&AuditResource> for ProtoResourceKind {
    fn from(value: &AuditResource) -> Self {
        match value {
            AuditResource::Target => ProtoResourceKind::Target,
            AuditResource::Prompt => ProtoResourceKind::Prompt,
            AuditResource::Field => ProtoResourceKind::Field,
        }
    }
}

// unused
// impl TryInto<AuditResource> for ProtoResourceKind {
//     type Error = anyhow::Error;
//
//     fn try_into(self) -> Result<AuditResource, Self::Error> {
//         match self {
//             ProtoResourceKind::Target => Ok(AuditResource::Target),
//             ProtoResourceKind::Prompt => Ok(AuditResource::Prompt),
//             ProtoResourceKind::Field => Ok(AuditResource::Field),
//             other => Err(anyhow!(
//                 "Got currently unsupported resource kind {other:?} while parsing AuditResource"
//             )),
//         }
//     }
// }

impl From<&AuditAction> for ProtoAuditAction {
    fn from(value: &AuditAction) -> Self {
        match value {
            AuditAction::Create => Self::Create,
            AuditAction::Update => Self::Update,
            AuditAction::Delete => Self::Delete,
            AuditAction::Rollback => Self::Rollback,
        }
    }
}

// unused
// impl TryInto<AuditAction> for ProtoAuditAction {
//     type Error = anyhow::Error;
//
//     fn try_into(self) -> Result<AuditAction, Self::Error> {
//         match self {
//             Self::Create => Ok(AuditAction::Create),
//             Self::Update => Ok(AuditAction::Update),
//             Self::Delete => Ok(AuditAction::Delete),
//             Self::Unknown => Err(anyhow!("Got UNKNOWN AuditAction")),
//         }
//     }
// }

// currently not required
// impl TryInto<AuditVersion> for ProtoAuditVersion {
//     type Error = anyhow::Error;
//
//     fn try_into(self) -> Result<AuditVersion, Self::Error> {
//         Ok(AuditVersion {
//             id: self.id,
//             resource_id: self.resource_id,
//             data: self
//                 .data
//                 .ok_or(anyhow!("Missing data on ProtoAuditVersion"))?
//                 .encode_to_vec()
//                 .into(),
//             resource_type: ProtoResourceKind::try_from(self.resource_type)
//                 .unwrap_or(ProtoResourceKind::Unknown)
//                 .try_into()?,
//             action: ProtoAuditAction::try_from(self.action)
//                 .unwrap_or(ProtoAuditAction::Unknown)
//                 .try_into()?,
//             version: self.version as u32,
//             made_by: self.made_by,
//             created_at: to_date_time!(self.created_at),
//         })
//     }
// }

impl TryInto<ProtoAuditVersion> for WithUser<AuditVersion> {
    type Error = prost::DecodeError;

    fn try_into(self) -> Result<ProtoAuditVersion, Self::Error> {
        Ok(ProtoAuditVersion {
            made_by: self.proto_user(),
            id: self.inner.id,
            resource_id: self.inner.resource_id,
            data: Some(ProtoResource::decode(self.inner.data.as_ref())?),
            resource_type: Into::<ProtoResourceKind>::into(&self.inner.resource_type).into(),
            action: Into::<ProtoAuditAction>::into(&self.inner.action).into(),
            version: self.inner.version as i32,
            created_at: Some(date_time_to_timestamp(&self.inner.created_at)),
        })
    }
}
