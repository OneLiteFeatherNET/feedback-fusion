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
    pub static ref CONFIG: Config = envy::from_env::<Config>().unwrap();
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
pub struct NConfig {
    cache: Option<CacheConfiguration>,
    oidc: OIDCConfiguration,
    #[cfg(feature = "otlp")]
    otlp: OTLPConfiguration,
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

config!(
    OIDCConfiguration,
    (
        issser: Option<String>,
        provider: String
    ),

    (
        audience: String = "feedback-fusion"
    )
);

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct OTLPConfiguration {
    endpoint: String,
}

config!(
    Config,
    (
        oidc_provider: String,
        oidc_issuer: Option<String>,
        config_path: Option<String>,
        otlp_endpoint: Option<String>,

        skytable_host: Option<String>,
        skytable_port: Option<u16>,
        skytable_certificate: Option<String>,
        skytable_username: Option<String>,
        skytable_password: Option<String>
    ),

    (
        service_name: String = "feedback-fusion"
        oidc_audience: String = "feedback-fusion",

        skytable_space: String = "cache",
        skytable_model: String = "feedbackfusion",

        oidc_scope_api: String = "api:feedback-fusion",
        oidc_scope_write: String = "feedback-fusion:write",
        oidc_scope_read: String = "feedback-fusion:read",

        oidc_scope_write_target: String = "feedback-fusion:writeTarget",
        oidc_scope_read_target: String = "feedback-fusion:readTarget"

        oidc_scope_write_prompt: String = "feedback-fusion:writePrompt",
        oidc_scope_read_prompt: String = "feedback-fusion:readPrompt"

        oidc_scope_write_field: String = "feedback-fusion:writeField",
        oidc_scope_read_field: String = "feedback-fusion:readField",

        oidc_scope_read_response: String = "feedback-fusion:readResponse",

        oidc_groups_claim: String = "groups",
    )
);

#[macro_export]
macro_rules! skytable_configuration {
    () => {{
        $crate::cache::SkytableCacheBuilder::new(
            CONFIG.skytable_host().as_ref().unwrap().as_str(),
            *CONFIG.skytable_port().as_ref().unwrap(),
            CONFIG.skytable_username().as_ref().unwrap().as_str(),
            CONFIG.skytable_password().as_ref().unwrap().as_str(),
        )
        .set_space(CONFIG.skytable_space())
        .set_model(CONFIG.skytable_model())
        .set_refresh(false)
        .set_lifetime(std::time::Duration::from_secs(300))
        .build()
        .await
        .unwrap()
    }};
}

#[macro_export]
macro_rules! skytable_configuration_tls {
    () => {{
        $crate::cache::SkytableTlsCacheBuilder::new(
            CONFIG.skytable_host().as_ref().unwrap().as_str(),
            *CONFIG.skytable_port().as_ref().unwrap(),
            CONFIG.skytable_username().as_ref().unwrap().as_str(),
            CONFIG.skytable_password().as_ref().unwrap().as_str(),
        )
        .set_certificate(CONFIG.skytable_certificate().as_ref().unwrap().as_str())
        .set_space(CONFIG.skytable_space())
        .set_model(CONFIG.skytable_model())
        .set_refresh(false)
        .set_lifetime(std::time::Duration::from_secs(300))
        .build()
        .await
        .unwrap()
    }};
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InstanceConfig {
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

#[instrument(skip_all)]
pub async fn sync_config(connection: &DatabaseConnection) -> Result<()> {
    // as this function can only be called when the notify watch agent got created we can use
    // unwrap here
    let config_path = CONFIG.config_path.as_ref().unwrap();
    let content = tokio::fs::read_to_string(config_path)
        .await
        .map_err(|error| {
            FeedbackFusionError::ConfigurationError(format!(
                "Error while reading config: {}",
                error
            ))
        })?;

    // parse the config
    let config: InstanceConfig = serde_yaml::from_str(content.as_str()).map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Error while reading config: {}", error))
    })?;
    info!("Sucessfully parsed config");

    // start a database transaction
    let transaction = connection.acquire_begin().await?;
    let transaction = transaction.defer_async(|mut tx| async move {
        if !tx.done {
            let _ = tx.rollback().await;
        }
    });

    for target in config.targets.into_iter() {
        sync_target(target, &transaction).await?;
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
