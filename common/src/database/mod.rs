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
use rbatis::RBatis;
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

                                connection.intercepts.clear();
                                connection.intercepts.push(std::sync::Arc::new(rbatis::plugin::intercept_log::LogInterceptor::new(log::LevelFilter::Debug)));

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

/// rbatis doesnt convert the LIMIT statements for postgres and mssql therefore we need a wrapper
/// REF: https://rbatis.github.io/rbatis.io/#/v4/?id=macros-select-page
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! impl_select_page_wrapper {
    ($table:path {}) => {
        impl_select_page_wrapper!($table{select_page() => ""});
    };
    ($table:path {$ident:ident ($($arg:ident: $ty:ty $(,)?)*) => $expr:expr}) => {
        paste!{
            impl_select_page!($table {$ident($($arg: $ty,)* limit_sql: &str) => $expr});

            impl $table {
                pub async fn [<$ident _wrapper>](executor: &dyn rbatis::executor::Executor, page_request: &dyn rbatis::IPageRequest, $($arg: $ty,)*) -> std::result::Result<rbatis::plugin::page::Page<$table>, rbatis::rbdc::Error> {

                  use std::ops::Deref;
                  let limit = page_request.page_size();
                  let offset = page_request.offset();

                  match crate::config::DATABASE_CONFIG.deref() {
                     $crate::database::DatabaseConfiguration::Postgres(_) => Self::$ident(executor, page_request, $($arg,)* format!(" LIMIT {} OFFSET {} ", limit, offset).as_str()).await,
                     $crate::database::DatabaseConfiguration::MSSQL(_) => Self::$ident(executor, page_request, $($arg,)* format!(" ORDER BY id OFFSET {} ROWS FETCH NEXT {} ROWS ONLY", offset, limit).as_str()).await,
                    #[allow(unreachable_patterns)]
                    _ => Self::$ident(executor, page_request, $($arg,)* format!(" LIMIT {},{} ", offset, limit).as_str()).await
                 }
               }
            }
        }

    }
}
