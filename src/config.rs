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

use crate::{
    database::schema::feedback::{FieldOptions, FieldType},
    prelude::*,
};

lazy_static! {
    pub static ref CONFIG: Config = envy::from_env::<Config>().unwrap();
    pub static ref DATABASE_CONFIG: DatabaseConfiguration =
        DatabaseConfiguration::extract().unwrap();
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct Config {
    #[serde(default = "default_global_rate_limit")]
    global_rate_limit: u64,
    oidc_discovery_url: String,
    #[serde(default = "default_oidc_audience")]
    oidc_audience: String,
    config_path: Option<String>,
}

#[inline]
fn default_global_rate_limit() -> u64 {
    10
}

#[inline]
fn default_oidc_audience() -> String {
    "feedback-fusion".to_owned()
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
    #[serde(default)]
    active: bool,
    fields: Option<Vec<FieldConfig>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FieldConfig {
    id: String,
    description: Option<String>,
    field_type: FieldType,
    options: FieldOptions,
}

pub async fn sync_config() -> Result<()> {
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
    let config = toml::from_str(content.as_str()).map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Error while reading config: {}", error))
    })?;
    info!("Sucessfully parsed config");

    Ok(())
}
