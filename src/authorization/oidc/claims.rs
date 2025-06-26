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

use aliri::jwt::{self};
use aliri_clock::UnixTime;
use aliri_oauth2::{HasScope, Scope};

use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};

// we do not know the group claim names during the compile time, therefore we do have to use this
// huge custom deserializer
#[derive(Debug, Clone)]
pub struct OIDCClaims {
    sub: jwt::Subject,
    iss: jwt::Issuer,
    iat: UnixTime,
    aud: jwt::Audiences,
    nbf: Option<UnixTime>,
    exp: UnixTime,
    scope: Scope,
    groups: Vec<String>,
    is_application: bool,
}

impl OIDCClaims {
    pub fn is_application(&self) -> bool {
        self.is_application
    }
}

impl<'de> Deserialize<'de> for OIDCClaims {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(OIDCClaimsVisitor)
    }
}

struct OIDCClaimsVisitor;

impl<'de> Visitor<'de> for OIDCClaimsVisitor {
    type Value = OIDCClaims;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<V>(self, mut map: V) -> std::result::Result<OIDCClaims, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut sub = None;
        let mut iss = None;
        let mut iat = None;
        let mut aud = None;
        let mut nbf = None;
        let mut exp = None;
        let mut scope = None;
        let mut groups = None;
        let mut is_application = false;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "sub" => {
                    sub = Some(map.next_value()?);
                }
                "client_id" => {
                    is_application = true;
                    sub = Some(map.next_value()?);
                }
                "iss" => {
                    iss = Some(map.next_value()?);
                }
                "iat" => {
                    iat = Some(map.next_value()?);
                }
                "aud" | "audience" => {
                    aud = Some(map.next_value()?);
                }
                "nbf" => {
                    nbf = Some(map.next_value()?);
                }
                "exp" => {
                    exp = Some(map.next_value()?);
                }
                "scope" => {
                    scope = Some(map.next_value()?);
                }
                _ if key.as_str() == CONFIG.oidc().group_claim().as_str() => {
                    groups = Some(map.next_value()?);
                }
                _ => {
                    let _: serde_json::Value = map.next_value()?;
                }
            }
        }

        let sub = sub.ok_or_else(|| serde::de::Error::missing_field("sub"))?;
        let iss = iss.ok_or_else(|| serde::de::Error::missing_field("iss"))?;
        let iat = iat.ok_or_else(|| serde::de::Error::missing_field("iat"))?;
        let aud = aud.unwrap_or(jwt::Audiences::default());
        let exp = exp.ok_or_else(|| serde::de::Error::missing_field("exp"))?;
        let scope = scope.unwrap_or(Scope::empty());
        let groups = groups
            .ok_or_else(|| serde::de::Error::missing_field(CONFIG.oidc().group_claim().as_str()))?;

        Ok(OIDCClaims {
            is_application,
            sub,
            iss,
            iat,
            aud,
            nbf,
            exp,
            scope,
            groups,
        })
    }
}

impl jwt::CoreClaims for OIDCClaims {
    fn nbf(&self) -> Option<UnixTime> {
        Some(self.nbf.unwrap_or(self.iat))
    }
    fn exp(&self) -> Option<UnixTime> {
        Some(self.exp)
    }
    fn aud(&self) -> &jwt::Audiences {
        &self.aud
    }
    fn iss(&self) -> Option<&jwt::IssuerRef> {
        Some(&self.iss)
    }
    fn sub(&self) -> Option<&jwt::SubjectRef> {
        Some(&self.sub)
    }
}

impl HasScope for OIDCClaims {
    fn scope(&self) -> &Scope {
        &self.scope
    }
}

impl OIDCClaims {
    pub fn groups(&self) -> &Vec<String> {
        &self.groups
    }
}
