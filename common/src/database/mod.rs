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
use prost_types::Timestamp;
use rbatis::{executor::RBatisTxExecutorGuard, rbdc::DateTime, RBatis};
use tokio_retry::{
    strategy::{jitter, FibonacciBackoff},
    Retry,
};

pub mod schema;

pub type DatabaseConnection = RBatis;

#[derive(Deserialize, Debug, Clone)]
pub struct BaseConfiguration {
    pub endpoint: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl BaseConfiguration {
    pub fn to_url(&self, scheme: &str) -> String {
        format!(
            "{scheme}://{}:{}@{}/{}",
            self.username, self.password, self.endpoint, self.database
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MSSQLConfiguration {
    pub endpoint: String,
    pub username: String,
    pub password: String,
    pub database: String,
    #[serde(default = "bool_true")]
    pub encrypt: bool,
    #[serde(default = "bool_true")]
    pub trust_server_certificate: bool,
}

fn bool_true() -> bool {
    true
}

impl MSSQLConfiguration {
    pub fn to_url(&self, _scheme: &str) -> String {
        format!(
            "jdbc:sqlserver://{};username={};password={};database={};encrypt=false;TrustServerCertificate=true",
            self.endpoint, self.username, self.password, self.database
        )
    }
}

macro_rules! database_configuration {
    ($(($ident:ident, $config:path, $driver:path, $scheme:literal) $(,)?)*) => {
        paste! {
            #[derive(Debug, Clone)]
            pub enum DatabaseConfiguration {
                $(
                    $ident($config),
                )*
            }

            #[derive(Debug, Clone, Getters, Deserialize)]
            #[get = "pub"]
            pub struct DatabaseConfigurationScheme {
                $(
                        [<$ident:lower>]: Option<$config>,
                )*
            }

            impl DatabaseConfiguration {
                #[inline(always)]
                pub fn extract(config: &DatabaseConfigurationScheme) -> anyhow::Result<Self> {
                    $(
                       if let Some(config) = config.[<$ident:lower>]() {
                           return Ok(Self::$ident(config.clone()));
                      }
                 )*

                   Err(anyhow!("invalid database configuration"))
                }

                #[inline(always)]
                pub async fn connect(&self) -> rbatis::error::Result<RBatis> {
                    let connection = RBatis::new();

                    match self {
                        $(
                            Self::$ident(config) => {
                                let url = config.to_url($scheme);

                                let retry_strategy = FibonacciBackoff::from_millis(5000)
                                    .map(jitter)
                                    .take(5);

                                Retry::spawn(retry_strategy, || async {
                                    match connection.init($driver {}, url.as_str()) {
                                        Ok(_) => {
                                            // check wether the connection actually works
                                            connection.query("SELECT 1", vec![]).await
                                        },
                                        Err(error) => {
                                            error!("Failed to establish DatabaseConnection");
                                            error!("{}", error);

                                            Err(error)
                                        }
                                    }
                                }).await?;

                                // perform init queries
                                connection.exec(include_str!(stringify!([<$ident:lower>] .sql)), vec![]).await?;
                            }
                        )*
                    };

                    Ok(connection)
                }
            }
        }
    };
}

database_configuration!(
    (
        Postgres,
        BaseConfiguration,
        rbdc_pg::driver::PgDriver,
        "postgres"
    ),
    (
        MySQL,
        BaseConfiguration,
        rbdc_mysql::driver::MysqlDriver,
        "mysql"
    ),
    (
        MSSQL,
        MSSQLConfiguration,
        rbdc_mssql::driver::MssqlDriver,
        "mssql"
    )
);

#[macro_export]
macro_rules! database_request {
    ($expr: expr, $title: expr) => {{
        let span = info_span!(concat!("Database request: ", $title));

        async { $expr }.instrument(span).await
    }};
}

pub fn date_time_to_timestamp(date_time: DateTime) -> Timestamp {
    Timestamp::date_time(
        date_time.year().into(),
        date_time.mon(),
        date_time.day(),
        date_time.hour(),
        date_time.minute(),
        date_time.sec(),
    )
    .unwrap()
}

#[macro_export]
macro_rules! to_date_time {
    ($ident:expr) => {{
        DateTime::from_timestamp($ident.unwrap().seconds)
    }};
}

#[macro_export()]
macro_rules! save_as_json {
    ($struct:path, $ident:ident) => {
        paste! {
            fn [<serialize_ $ident>]<S>(sub: &$struct, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let json_string = serde_json::to_string(sub).map_err(|error| serde::ser::Error::custom(format!("failed to serialize {} as json: {}", stringify!($struct), error)))?;
                serializer.serialize_str(&json_string)
            }

            fn [<deserialize_ $ident>]<'de, D>(deserializer: D) -> std::result::Result<$struct, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let json_string = String::deserialize(deserializer)?;
                serde_json::from_str(&json_string).map_err(|error| serde::de::Error::custom(format!("failed to deserialize {} from json: {}", stringify!($struct), error)))
            }
        }
    };
}

pub async fn transaction(connection: &DatabaseConnection) -> anyhow::Result<RBatisTxExecutorGuard> {
    let transaction = connection.acquire_begin().await?;
    let transaction = transaction.defer_async(|tx| async move {
        if !tx.done() {
            let _ = tx.rollback().await;
        }
    });

    Ok(transaction)
}
