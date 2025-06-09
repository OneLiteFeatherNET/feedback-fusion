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

use std::borrow::Cow;

use dashmap::{DashMap, DashSet};
use rbatis::executor::Executor;
use strum_macros::{Display, EnumIter, IntoStaticStr};
use wildcard::Wildcard;

use crate::{
    database::{
        schema::feedback::{Field, FieldOptions, FieldType, Prompt, Target},
        DatabseConfigurationScheme,
    },
    prelude::*,
};

#[derive(Getters, PartialEq, Eq, Hash, Debug)]
#[get = "pub"]
pub struct PermissionMatrixKey<'a> {
    endpoint: &'a Endpoint<'a>,
    permission: &'a Permission,
}

impl<'a> From<(&'a Endpoint<'a>, &'a Permission)> for PermissionMatrixKey<'a> {
    fn from(value: (&'a Endpoint<'a>, &'a Permission)) -> Self {
        Self {
            endpoint: value.0,
            permission: value.1,
        }
    }
}

#[derive(Getters, MutGetters, Debug)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct PermissionMatrixValue<'a> {
    scopes: DashSet<&'a str>,
    groups: DashSet<&'a str>,
}

impl<'a> From<(DashSet<&'a str>, DashSet<&'a str>)> for PermissionMatrixValue<'a> {
    fn from(value: (DashSet<&'a str>, DashSet<&'a str>)) -> Self {
        Self {
            scopes: value.0,
            groups: value.1,
        }
    }
}

pub type PermissionMatrix<'a> = DashMap<PermissionMatrixKey<'a>, PermissionMatrixValue<'a>>;

lazy_static! {
    pub static ref CONFIG: Config<'static> = read_config().unwrap();
    pub static ref DATABASE_CONFIG: DatabaseConfiguration =
        DatabaseConfiguration::extract().unwrap();
    pub static ref ENDPOINTS: [Endpoint<'static>; 6] = [
        Endpoint::Target(EndpointScopeSelector::default()),
        Endpoint::Prompt(EndpointScopeSelector::default()),
        Endpoint::Field(EndpointScopeSelector::default()),
        Endpoint::Export(EndpointScopeSelector::default()),
        Endpoint::Response(EndpointScopeSelector::default()),
        Endpoint::Authorize(None)
    ];
    pub static ref PERMISSIONS: [Permission; 4] = [
        Permission::Write,
        Permission::Read,
        Permission::List,
        Permission::All
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
#[derive(Hash, PartialEq, Eq, Deserialize, Debug, Clone, Display, IntoStaticStr, Serialize)]
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
/// backwards mapping from endpoint and permission to a list of scopes and groups instead of the
/// other way.
pub fn read_permission_matrix<'a>(
    scopes: &'static [AuthorizationMapping],
    groups: &'static [AuthorizationMapping],
) -> PermissionMatrix<'a> {
    let map: PermissionMatrix = DashMap::new();

    for (index, list) in [scopes, groups].iter().enumerate() {
        for mapping in list.iter() {
            let name = mapping.name().as_str();

            for grant in mapping.grants.iter() {
                let endpoint_string = match &grant.endpoint {
                    Endpoint::Custom(custom, _) => custom.to_string(),
                    _ => grant.endpoint.to_string(),
                };
                let endpoint_pattern = Wildcard::new(endpoint_string.as_bytes()).unwrap();

                for endpoint in ENDPOINTS
                    .iter()
                    .filter(|endpoint| endpoint_pattern.is_match(endpoint.to_string().as_bytes()))
                {
                    for permission in grant.permissions.iter() {
                        let permission_string = permission.to_string();
                        let permission_pattern =
                            Wildcard::new(permission_string.as_bytes()).unwrap();

                        for permission in PERMISSIONS.iter().filter(|ppermission| {
                            permission_pattern.is_match(ppermission.to_string().as_bytes())
                                || permission.eq(&Permission::All)
                        }) {
                            let key = PermissionMatrixKey::from((endpoint, permission));

                            let mut entry = map.entry(key).or_insert_with(|| {
                                PermissionMatrixValue::from((DashSet::new(), DashSet::new()))
                            });

                            if index == 0 {
                                entry.scopes_mut().insert(name);
                            } else {
                                entry.groups_mut().insert(name);
                            }
                        }
                    }
                }
            }
        }
    }

    debug!("Read PermissionMatrix: {:?}", map);

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
                endpoint: Endpoint::Target(EndpointScopeSelector::default()),
                permissions: vec![Permission::Read],
            },],
        },];
        static ref GROUPS: Vec<AuthorizationMapping<'static>> = vec![AuthorizationMapping {
            name: "group1".to_string(),
            grants: vec![AuthorizationGrants {
                endpoint: Endpoint::Prompt(EndpointScopeSelector::default()),
                permissions: vec![Permission::Write],
            },],
        },];
        static ref SCOPES_WILDCARD: Vec<AuthorizationMapping<'static>> =
            vec![AuthorizationMapping {
                name: "scope2".to_string(),
                grants: vec![AuthorizationGrants {
                    endpoint: Endpoint::Custom(
                        Cow::Borrowed("*"),
                        EndpointScopeSelector::default()
                    ),
                    permissions: vec![Permission::All],
                },],
            },];
        static ref SCOPES_NO_MATCH: Vec<AuthorizationMapping<'static>> =
            vec![AuthorizationMapping {
                name: "scope3".to_string(),
                grants: vec![AuthorizationGrants {
                    endpoint: Endpoint::Custom(
                        Cow::Borrowed("NoneExistent"),
                        EndpointScopeSelector::default()
                    ),
                    permissions: vec![Permission::All],
                },],
            },];
        static ref TARGET_ENDPOINT: Endpoint<'static> =
            Endpoint::Target(EndpointScopeSelector::default());
        static ref TARGET: PermissionMatrixKey<'static> =
            PermissionMatrixKey::from((&*TARGET_ENDPOINT, &Permission::Read));
        static ref PROMPT_ENDPOINT: Endpoint<'static> =
            Endpoint::Prompt(EndpointScopeSelector::default());
        static ref PROMPT: PermissionMatrixKey<'static> =
            PermissionMatrixKey::from((&*PROMPT_ENDPOINT, &Permission::Write));
    }

    #[test]
    fn test_read_permission_matrix() {
        let matrix = read_permission_matrix(&SCOPES, &GROUPS);

        assert!(matrix.contains_key(&TARGET));
        assert!(matrix.contains_key(&PROMPT));

        let entry = matrix.get(&TARGET).unwrap();
        assert!(entry.scopes().contains("scope1"));
        assert!(entry.groups().is_empty());

        let entry = matrix.get(&PROMPT).unwrap();
        assert!(entry.scopes().is_empty());
        assert!(entry.groups().contains("group1"));
    }

    #[test]
    fn test_read_permission_matrix_with_wildcards() {
        let matrix = read_permission_matrix(&SCOPES_WILDCARD, &[]);

        for endpoint in ENDPOINTS.iter() {
            for permission in PERMISSIONS.iter() {
                let entry = matrix
                    .get(&PermissionMatrixKey::from((endpoint, permission)))
                    .unwrap();
                assert!(entry.scopes().contains("scope2"));
            }
        }
    }

    #[test]
    fn test_read_permission_matrix_no_matches() {
        let matrix = read_permission_matrix(&SCOPES_NO_MATCH, &[]);

        assert!(matrix.is_empty());
    }
}
