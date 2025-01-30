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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, strum_macros::Display)]
pub enum ResourceKind {
    Target,
    Prompt,
    Field,
}

impl PartialEq<Endpoint<'_>> for ResourceKind {
    fn eq(&self, other: &Endpoint<'_>) -> bool {
        match *self {
            Self::Target => matches!(other, Endpoint::Target(_)),
            Self::Prompt => matches!(other, Endpoint::Prompt(_)),
            Self::Field => matches!(other, Endpoint::Field(_)),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ResourceAuthorizationType {
    Scope,
    Group,
    Subject,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, strum_macros::Display)]
pub enum ResourceAuthorizationGrant {
    Read,
    Write,
}

impl PartialEq<Permission> for ResourceAuthorizationGrant {
    fn eq(&self, other: &Permission) -> bool {
        match *self {
            Self::Write => matches!(other, Permission::Write),
            Self::Read => matches!(other, Permission::Read),
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

pub struct Authorization<'a>(pub &'a Endpoint<'a>, pub &'a Permission);

impl std::fmt::Display for Authorization<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Endpoint::Target(Some(inner))
        | Endpoint::Prompt(Some(inner))
        | Endpoint::Field(Some(inner)) = &self.0
        {
            write!(f, "{}::{}::{}", self.0, inner, self.1)
        } else {
            write!(f, "{}::{}", self.0, self.1)
        }
    }
}
