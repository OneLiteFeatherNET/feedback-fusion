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

use feedback_fusion_common::{
    database::{DatabaseConfiguration, DatabaseConfigurationScheme},
    observability::OTLPConfiguration,
};
use fluvio::FluvioClusterConfig;

use crate::prelude::*;

lazy_static! {
    pub static ref CONFIG: Config = read_config().unwrap();
    pub static ref DATABASE_CONFIG: DatabaseConfiguration =
        DatabaseConfiguration::extract(CONFIG.database()).unwrap();
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct Config {
    otlp: Option<OTLPConfiguration>,
    database: DatabaseConfigurationScheme,
    broker: BrokerConfiguration,
}

#[derive(Deserialize, Debug, Clone)]
pub enum BrokerMode {
    GRPC,
    Fluvio,
}

#[derive(Deserialize, Debug, Clone, Getters)]
#[get = "pub"]
pub struct BrokerConfiguration {
    mode: BrokerMode,
    fluvio: Option<FluvioClusterConfig>,
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
        FeedbackFusionError::ConfigurationError(format!("Error while reading config: {error}"))
    })?;

    // parse the config
    let config: Config = hcl::from_str(content.as_str()).map_err(|error| {
        FeedbackFusionError::ConfigurationError(format!("Error while reading config: {error}"))
    })?;
    info!("Sucessfully parsed config");

    Ok(config)
}
