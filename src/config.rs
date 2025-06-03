/*
 * Copyright (c) 2023 OneLiteFeatherNET
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

use std::{borrow::Cow, ops::Deref};

use dashmap::{DashMap, DashSet};
use rbatis::executor::Executor;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, IntoStaticStr};
use wildcard::Wildcard;

use crate::{
    database::{
        schema::feedback::{Field, FieldOptions, FieldType, Prompt, Target},
        DatabseConfigurationScheme,
    },
    prelude::*,
};

pub type PermissionMatrix = DashMap<
    (Endpoint<'static>, Permission),
    (DashSet<Cow<'static, str>>, DashSet<Cow<'static, str>>),
>;

lazy_static! {
    pub static ref CONFIG: Config<'static> = read_config().unwrap();
    pub static ref PERMISSION_MATRIX: PermissionMatrix =
        read_permission_matrix(CONFIG.oidc().scopes(), CONFIG.oidc().groups());
    pub static ref DATABASE_CONFIG: DatabaseConfiguration =
        DatabaseConfiguration::extract().unwrap();
    pub static ref ENDPOINTS: &'static [Endpoint<'static>] = &[
        Endpoint::Target(EndpointScopeSelector::default()),
        Endpoint::Prompt(EndpointScopeSelector::default()),
        Endpoint::Field(EndpointScopeSelector::default()),
        Endpoint::Export(EndpointScopeSelector::default()),
        Endpoint::Response(EndpointScopeSelector::default()),
        Endpoint::Authorize(None)
    ];
}

macro_rules! config {
    ($config:ident, ($($ident:ident: $type:ty $(,)? )*),  ($($dident:ident: $dtype:ty = $default:expr $(,)?)*)) => {
        paste! {
            #[derive(Deserialize, Debug, Clone, Getters)]
            #[get = "pub"]
            pub struct $config {
                $(
                    $ident: $type,
                )*

                $(
                    #[serde(default = "default_" $config:lower "_" $dident)]
                    $dident: $dtype,
                )*
            }

            $(
                #[inline]
                fn [<default_ $config:lower _ $dident>]() -> $dtype {
                    $default.to_owned()
                }
            )*

        }
    };
    ($config:ident, $lifetime:lifetime, ($($ident:ident: $type:ty $(,)? )*),  ($($dident:ident: $dtype:ty = $default:expr $(,)?)*)) => {
        paste! {
            #[derive(Deserialize, Debug, Clone, Getters)]
            #[get = "pub"]
            pub struct $config<$lifetime> {
                $(
                    $ident: $type,
                )*

                $(
                    #[serde(default = "default_" $config:lower "_" $dident)]
                    $dident: $dtype,
                )*
            }

            $(
                #[inline]
                fn [<default_ $config:lower _ $dident>]() -> $dtype {
                    $default.to_owned()
                }
            )*
        }
    };
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct Config<'a> {
    cache: Option<CacheConfiguration>,
    oidc: OIDCConfiguration<'a>,
    #[cfg(feature = "otlp")]
    otlp: Option<OTLPConfiguration>,
    preset: Option<PresetConfig>,
    database: DatabseConfigurationScheme,
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct CacheConfiguration {
    skytable: Option<SkytableConfiguration>,
}

config!(
    SkytableConfiguration,
    (
        host: String,
        port: u16,
        certificate: Option<String>,
        username: String,
        password: String
    ),

    (
        space: String = "cache",
        model: String = "feedbackfusion",
    )
);

/// This limits the access to the specified endpoint to the given scope.
///
/// We can use this to only allow access to the endpoint for specific element ids
/// and similar.
#[derive(Hash, PartialEq, Eq, Deserialize, Debug, Clone, Display, IntoStaticStr)]
pub enum EndpointScopeSelector<'a> {
    /// unscoped access
    All,
    /// access only for a specific id
    Specific(Cow<'a, str>),
    /// access for multiple ids
    Multiple(Vec<Cow<'a, str>>),
    /// custom wildcard format
    Wildcard(Cow<'a, str>),
}

impl Default for EndpointScopeSelector<'_> {
    fn default() -> Self {
        Self::All
    }
}

/// Does define a scope for the client authorization with in the given endpoint group.
///
/// You can limit the access to the specified endpoint by defining a `EndpointScopeSelector`.
/// If you wish to use wildcards you should use the `Custom` variant.
#[derive(Hash, PartialEq, Eq, Deserialize, Debug, Clone, Display, IntoStaticStr)]
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
    pub fn to_static(self) -> Endpoint<'static> {
        match self {
            Endpoint::Target(opt) => Endpoint::Target(opt.to_owned().clone()),
            // Endpoint::Prompt(opt) => Endpoint::Prompt(opt.to_owned()),
            // Endpoint::Field(opt) => Endpoint::Field(opt.to_owned()),
            // Endpoint::Response(opt) => Endpoint::Response(opt.to_owned()),
            // Endpoint::Export(opt) => Endpoint::Export(opt.to_owned()),
            Endpoint::Authorize(Some(inner)) => {
                Endpoint::Authorize(Some(Box::new(inner.to_static())))
            }
            Endpoint::Authorize(None) => Endpoint::Authorize(None),
            // Endpoint::Custom(wildcard, opt) => {
            //     let wildcard = Cow::Owned(wildcard.clone().into_owned());
            //     let opt = opt.to_owned();
            //     Endpoint::Custom(wildcard, opt)
            // }
            _ => Endpoint::Authorize(None),
        }
    }

    pub fn none(&self) -> Endpoint {
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

#[derive(Hash, PartialEq, Eq, Deserialize, Debug, Clone, EnumIter, Display, IntoStaticStr)]
pub enum Permission {
    Read,
    Write,
    List,
    All,
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct AuthorizationGrants<'a> {
    endpoint: Endpoint<'a>,
    permissions: Vec<Permission>,
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct AuthorizationMapping<'a> {
    name: String,
    grants: Vec<AuthorizationGrants<'a>>,
}

config!(
    OIDCConfiguration,
    'a,
    (
        issuer: Option<String>,
        provider: String,
        scopes: Vec<AuthorizationMapping<'a>>,
        groups: Vec<AuthorizationMapping<'a>>,
    ),

    (
        audience: String = "feedback-fusion",
        group_claim: String = "groups"
    )
);

config!(
    OTLPConfiguration,
    (
        endpoint: String,
    ),

    (
        service_name: String = "feedback-fusion"
    )
);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PresetConfig {
    targets: Vec<TargetConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TargetConfig {
    id: String,
    name: String,
    description: Option<String>,
    prompts: Option<Vec<PromptConfig>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PromptConfig {
    id: String,
    title: String,
    description: String,
    #[serde(default)]
    active: bool,
    fields: Option<Vec<FieldConfig>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FieldConfig {
    id: String,
    title: String,
    description: Option<String>,
    field_type: FieldType,
    options: FieldOptions,
}

pub fn read_config() -> Result<Config<'static>> {
    // as this function can only be called when the notify watch agent got created we can use
    // unwrap here
    let config_path = std::env::var("FEEDBACK_FUSION_CONFIG").map_err(|_| {
        FeedbackFusionError::ConfigurationError(
            "Missing FEEDBACK_FUSION_CONFIG environment variable".to_owned(),
        )
    })?;
    let content = std::fs::read_to_string(config_path).map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Error while reading config: {}", error))
    })?;

    // parse the config
    let config: Config = serde_yaml::from_str(content.as_str()).map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Error while reading config: {}", error))
    })?;
    info!("Sucessfully parsed config");

    Ok(config)
}

/// This function does require CONFIG to be already initialized and performs the reverse /
/// backwards mapping from endpoint and permission to a list of scopes and grouops instead of the
/// other way.
pub fn read_permission_matrix(
    scopes: &'static [AuthorizationMapping],
    groups: &'static [AuthorizationMapping],
) -> PermissionMatrix {
    let map: PermissionMatrix = DashMap::new();

    for (index, list) in [scopes, groups].iter().enumerate() {
        for mapping in list.iter() {
            let name = Cow::Borrowed(mapping.name().as_str());
            for grant in mapping.grants.iter() {
                // the grant does support wildcards for the `endpoint` field.
                // therefore we here try to match the endpoint variatns
                let endpoints = {
                    let endpoint = grant.endpoint.to_string();
                    let pattern = Wildcard::new(endpoint.as_bytes()).unwrap();
                    ENDPOINTS
                        .iter()
                        .filter(|endpoint| pattern.is_match(endpoint.to_string().as_bytes()))
                        .collect::<Vec<&Endpoint>>()
                };

                for permission in grant.permissions.iter() {
                    // the permissions field does support wildcards for the `permission` field.
                    // therefore we here try to match the permission variatns
                    let permissions = {
                        let stringified = permission.to_string();
                        let pattern = Wildcard::new(stringified.as_bytes()).unwrap();
                        Permission::iter()
                            .filter(|permission| {
                                pattern.is_match(permission.to_string().as_bytes())
                            })
                            .collect::<Vec<Permission>>()
                    };

                    for endpoint in endpoints.clone().into_iter() {
                        for permission in permissions.clone().into_iter() {
                            map.entry((endpoint.clone(), permission.clone()))
                                .or_insert_with(|| (DashSet::new(), DashSet::new()));

                            let entry = map.get_mut(&(endpoint.clone(), permission)).unwrap();
                            if index == 0 {
                                entry.0.insert(name.clone());
                            } else {
                                entry.1.insert(name.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    map
}

pub async fn sync_preset(connection: &DatabaseConnection) -> Result<()> {
    if let Some(preset) = CONFIG.preset() {
        let transaction = connection.acquire_begin().await?;
        let mut transaction = transaction.defer_async(|mut tx| async move {
            if !tx.done {
                let _ = tx.rollback().await;
            }
        });

        for target in preset.clone().targets.into_iter() {
            sync_target(target, &transaction).await?;
        }

        transaction.commit().await?;
    }

    Ok(())
}

macro_rules! update_otherwise_create {
    ($transaction:ident, $path:path, $data:ident, $($field:ident $(,)?)* => $target:ident = $source:ident) => {
        paste!{
            let result = $path::select_by_id($transaction, $data.id.as_str()).await?;

            if let Some(mut data) = result {
                $(
                    data.[<set_ $field>]($data.$field);
                )*

                $path::update_by_column($transaction, &data, "id").await?;
            } else {
                $path::insert(
                    $transaction,
                    &$path::builder()
                    .id($data.id)
                    $(
                        .$field($data.$field)
                    )*
                    .$target($source)
                    .build()
                ).await?;
            }
        }
    };
    ($transaction:ident, $path:path, $data:ident, $($field:ident $(,)?)*) => {
        paste!{
            let result = $path::select_by_id($transaction, $data.id.as_str()).await?;

            if let Some(mut data) = result {
                $(
                    data.[<set_ $field>]($data.$field);
                )*

                $path::update_by_column($transaction, &data, "id").await?;
            } else {
                $path::insert(
                    $transaction,
                    &$path::builder()
                    .id($data.id)
                    $(
                        .$field($data.$field)
                    )*
                    .build()
                ).await?;
            }
        }
    };
}

pub async fn sync_target(target: TargetConfig, transaction: &dyn Executor) -> Result<()> {
    let id = target.id.clone();
    update_otherwise_create!(transaction, Target, target, name, description);

    if let Some(prompts) = target.prompts {
        for prompt in prompts.into_iter() {
            sync_prompt(prompt, transaction, id.clone()).await?;
        }
    }

    Ok(())
}

pub async fn sync_prompt(
    prompt: PromptConfig,
    transaction: &dyn Executor,
    target: String,
) -> Result<()> {
    let id = prompt.id.clone();
    update_otherwise_create!(transaction, Prompt, prompt, title, description, active => target = target);

    if let Some(fields) = prompt.fields {
        for field in fields.into_iter() {
            sync_field(field, transaction, id.clone()).await?;
        }
    }

    Ok(())
}

pub async fn sync_field(
    field: FieldConfig,
    transaction: &dyn Executor,
    prompt: String,
) -> Result<()> {
    update_otherwise_create!(transaction, Field, field, title, description, field_type, options => prompt = prompt);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref SCOPES: Vec<AuthorizationMapping<'static>> = vec![AuthorizationMapping {
            name: "scope1".to_string(),
            grants: vec![AuthorizationGrants {
                endpoint: Endpoint::Target(None),
                permissions: vec![Permission::Read],
            },],
        },];
        static ref GROUPS: Vec<AuthorizationMapping<'static>> = vec![AuthorizationMapping {
            name: "group1".to_string(),
            grants: vec![AuthorizationGrants {
                endpoint: Endpoint::Prompt(None),
                permissions: vec![Permission::Write],
            },],
        },];
        static ref SCOPES_WILDCARD: Vec<AuthorizationMapping<'static>> =
            vec![AuthorizationMapping {
                name: "scope2".to_string(),
                grants: vec![AuthorizationGrants {
                    endpoint: Endpoint::Custom(Cow::Borrowed("*"), None),
                    permissions: vec![Permission::All],
                },],
            },];
        static ref SCOPES_NO_MATCH: Vec<AuthorizationMapping<'static>> =
            vec![AuthorizationMapping {
                name: "scope3".to_string(),
                grants: vec![AuthorizationGrants {
                    endpoint: Endpoint::Custom(Cow::Borrowed("NoneExistent"), None),
                    permissions: vec![Permission::All],
                },],
            },];
    }

    #[test]
    fn test_read_permission_matrix() {
        let matrix = read_permission_matrix(&SCOPES, &GROUPS);

        assert!(matrix.contains_key(&(Endpoint::Target(None), Permission::Read)));
        assert!(matrix.contains_key(&(Endpoint::Prompt(None), Permission::Write)));

        let entry = matrix
            .get(&(Endpoint::Target(None), Permission::Read))
            .unwrap();
        assert!(entry.0.contains(&Cow::Borrowed("scope1")));
        assert!(entry.1.is_empty());

        let entry = matrix
            .get(&(Endpoint::Prompt(None), Permission::Write))
            .unwrap();
        assert!(entry.0.is_empty());
        assert!(entry.1.contains(&Cow::Borrowed("group1")));
    }

    #[test]
    fn test_read_permission_matrix_with_wildcards() {
        let matrix = read_permission_matrix(&SCOPES_WILDCARD, &[]);

        for endpoint in ENDPOINTS.iter() {
            for permission in Permission::iter() {
                let entry = matrix.get(&(endpoint.clone(), permission.clone())).unwrap();
                assert!(entry.0.contains(&Cow::Borrowed("scope2")));
            }
        }
    }

    #[test]
    fn test_read_permission_matrix_no_matches() {
        let matrix = read_permission_matrix(&SCOPES_NO_MATCH, &[]);

        assert!(matrix.is_empty());
    }
}
