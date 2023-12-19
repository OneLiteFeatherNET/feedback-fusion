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

use crate::{database::migration::Migration, prelude::*};
use rbatis::RBatis;

pub mod migration;
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

macro_rules! database_configuration {
    ($(($ident:ident, $config:path, $driver:path, $scheme:literal) $(,)?)*) => {
        paste! {
            #[derive(Debug, Clone)]
            pub enum DatabaseConfiguration {
                $(
                    #[cfg(feature = "" $ident:lower)]
                    $ident($config),
                )*
            }

            impl DatabaseConfiguration {
                #[inline(always)]
                pub fn extract() -> crate::error::Result<Self> {
                    $(
                       #[cfg(feature = "" $ident:lower)] 
                       if let Ok(config) = envy::prefixed(stringify!([<$ident:lower _>]).trim()).from_env::<$config>() {
                           return Ok(Self::$ident(config));
                      }
                 )*

                   Err(crate::error::FeedbackFusionError::ConfigurationError("invalid database configuration".to_owned()))
                }

                #[inline(always)]
                pub async fn connect(&self) -> Result<RBatis> {
                    let connection = RBatis::new();
                    let version = env!("CARGO_PKG_VERSION");

                    match self {
                        $(
                            #[cfg(feature = "" $ident:lower)]
                            Self::$ident(config) => {
                                let url = config.to_url($scheme);
                                connection.init($driver {}, url.as_str())?;

                                #[cfg(test)]
                                connection.exec(include_str!("drop.sql"), vec![]).await?;

                                // perform migrations
                                let last: Option<Migration> = Migration::select_latest(&connection).await.unwrap_or_default();
                                if let Some(last) = last {
                                    if !version.eq(last.version.as_str()) {
                                        // perform all pending migrations
                                        let migrations: Vec<(&str, &str)> = vec![];

                                        for(v, sql) in migrations {
                                            if version_compare::compare_to(v, version, version_compare::Cmp::Gt).unwrap() {
                                                connection.exec(sql, vec![]).await?;
                                                // insert the migration
                                               Migration::insert(&connection, &Migration::from(v.to_string())).await?;
                                            }
                                        }
                                    }
                                }

                                // perform init queries
                                let sql = include_str!(stringify!([<$ident:lower>] .sql));
                                connection.exec(sql, vec![]).await?;
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
    )
);

#[macro_export]
macro_rules! database_request {
    ($expr: expr) => {{
        let span = info_span!("Database Request");
        let _ = span.enter();
        $expr
    }};
    ($expr: expr, $title: expr) => {{
        let span = info_span!(concat!("Database Request: ", $title));
        let _ = span.enter();
        $expr
    }};
}

