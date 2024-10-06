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

use rbatis::executor::Executor;

use crate::{
    database::schema::feedback::{Field, FieldOptions, FieldType, Prompt, Target},
    prelude::*,
};

lazy_static! {
    pub static ref CONFIG: Config = read_config().unwrap();
    pub static ref DATABASE_CONFIG: DatabaseConfiguration =
        DatabaseConfiguration::extract().unwrap();
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
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct Config {
    cache: Option<CacheConfiguration>,
    oidc: OIDCConfiguration,
    #[cfg(feature = "otlp")]
    otlp: Option<OTLPConfiguration>,
    preset: Option<PresetConfig>,
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

#[derive(Deserialize, Debug, Clone)]
pub enum Endpoint {}

#[derive(Deserialize, Debug, Clone)]
pub enum Permission {
    Read,
    Write,
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct PermissionMapping {
    endpoint: Endpoint,
    permissions: Vec<Permission>,
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct AuthorizationMapping {
    name: String,
    permissions: Vec<PermissionMapping>,
}

config!(
    OIDCConfiguration,
    (
        issuer: Option<String>,
        provider: String,
        scopes: Vec<AuthorizationMapping>,
        groups: Vec<AuthorizationMapping>,
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

pub fn read_config() -> Result<Config> {
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

pub async fn sync_preset(connection: &DatabaseConnection) -> Result<()> {
    if let Some(preset) = CONFIG.preset() {
        let transaction = connection.acquire_begin().await?;
        let transaction = transaction.defer_async(|mut tx| async move {
            if !tx.done {
                let _ = tx.rollback().await;
            }
        });

        for target in preset.clone().targets.into_iter() {
            sync_target(target, &transaction).await?;
        }
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

#[instrument(skip_all)]
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

#[instrument(skip_all)]
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

#[instrument(skip_all)]
pub async fn sync_field(
    field: FieldConfig,
    transaction: &dyn Executor,
    prompt: String,
) -> Result<()> {
    update_otherwise_create!(transaction, Field, field, title, description, field_type, options => prompt = prompt);

    Ok(())
}
