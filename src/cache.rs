//SPDX-FileCopyrightText: 2024 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2024 OneLiteFeatherNet

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

use crate::{
    database::schema::feedback::{Field, Prompt},
    prelude::*,
};
#[cfg(feature = "caching-skytable")]
use bb8::ManageConnection;
#[cfg(feature = "caching-skytable")]
use cached::IOCachedAsync;
#[cfg(feature = "caching-skytable")]
use chrono::Utc;
use feedback_fusion_codegen::dynamic_cache;

#[cfg(feature = "caching-skytable")]
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "caching-skytable")]
use skytable::{
    aio::TcpConnection,
    pool::{ConnectionMgrTcp, ConnectionMgrTls},
    query, ClientResult, Pipeline, Query, Response,
};

#[cfg(feature = "caching-skytable")]
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::DerefMut,
    time::Duration,
};
#[cfg(feature = "caching-skytable")]
use thiserror::Error;
#[cfg(feature = "caching-skytable")]
use tokio::io::{AsyncRead, AsyncWrite};
#[cfg(feature = "caching-skytable")]
use tracing::{instrument, Instrument};

// may publish this as crate or submit as pr for cached
#[cfg(feature = "caching-skytable")]
pub struct SkytableCacheBuilder<'a, K, V> {
    username: &'a str,
    password: &'a str,
    host: &'a str,
    port: u16,
    space: &'a str,
    model: &'a str,
    refresh: bool,
    seconds: u64,
    _phantom: PhantomData<(K, V)>,
}

#[cfg(feature = "caching-skytable")]
impl<'a, K, V> SkytableCacheBuilder<'a, K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned,
{
    pub fn new(host: &'a str, port: u16, username: &'a str, password: &'a str) -> Self {
        Self {
            username,
            password,
            host,
            port,
            space: "cached",
            model: "cached",
            refresh: false,
            seconds: 0,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn set_space(mut self, space: &'a str) -> Self {
        self.space = space;
        self
    }

    #[must_use]
    pub fn set_model(mut self, model: &'a str) -> Self {
        self.model = model;
        self
    }

    #[must_use]
    pub fn set_refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    #[must_use]
    pub fn set_lifetime(mut self, duration: Duration) -> Self {
        self.seconds = duration.as_secs();
        self
    }

    pub async fn build(self) -> ClientResult<SkytableCache<'a, ConnectionMgrTcp, K, V>> {
        let pool = skytable::pool::get_async(
            32,
            skytable::Config::new(self.host, self.port, self.username, self.password),
        )
        .await
        .unwrap();

        let cache = SkytableCache {
            space: self.space,
            model: self.model,
            seconds: self.seconds,
            pool,
            refresh: self.refresh,
            _phantom: PhantomData,
        };
        cache.init().await;

        Ok(cache)
    }
}

#[cfg(feature = "caching-skytable")]
pub struct SkytableTlsCacheBuilder<'a, K, V> {
    username: &'a str,
    password: &'a str,
    host: &'a str,
    port: u16,
    space: &'a str,
    model: &'a str,
    certificate: &'a str,
    refresh: bool,
    seconds: u64,
    _phantom: PhantomData<(K, V)>,
}

#[cfg(feature = "caching-skytable")]
impl<'a, K, V> SkytableTlsCacheBuilder<'a, K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned,
{
    pub fn new(host: &'a str, port: u16, username: &'a str, password: &'a str) -> Self {
        Self {
            username,
            password,
            host,
            port,
            space: "cached",
            model: "cached",
            certificate: "",
            refresh: false,
            seconds: 0,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn set_space(mut self, space: &'a str) -> Self {
        self.space = space;
        self
    }

    #[must_use]
    pub fn set_model(mut self, model: &'a str) -> Self {
        self.model = model;
        self
    }

    #[must_use]
    pub fn set_certificate(mut self, certificate: &'a str) -> Self {
        self.certificate = certificate;
        self
    }

    #[must_use]
    pub fn set_refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    #[must_use]
    pub fn set_lifetime(mut self, duration: Duration) -> Self {
        self.seconds = duration.as_secs();
        self
    }

    pub async fn build(self) -> ClientResult<SkytableCache<'a, ConnectionMgrTls, K, V>> {
        let pool = skytable::pool::get_tls_async(
            32,
            skytable::Config::new(self.host, self.port, self.username, self.password),
            self.certificate,
        )
        .await
        .unwrap();

        let cache = SkytableCache {
            space: self.space,
            model: self.model,
            seconds: self.seconds,
            pool,
            refresh: self.refresh,
            _phantom: PhantomData,
        };
        cache.init().await;

        Ok(cache)
    }
}

#[cfg(feature = "caching-skytable")]
#[derive(Error, Debug)]
pub enum SkytableCacheError {
    #[error(transparent)]
    SkytableError(#[from] skytable::error::Error),
    #[error(transparent)]
    BincodeError(#[from] bincode::Error),
    #[error("{0}")]
    PoolError(String),
}

#[cfg(feature = "caching-skytable")]
impl<E> From<bb8::RunError<E>> for SkytableCacheError
where
    E: Debug,
{
    fn from(value: bb8::RunError<E>) -> Self {
        Self::PoolError(format!("{:?}", value))
    }
}

#[cfg(feature = "caching-skytable")]
#[derive(Clone, Query, Response)]
struct CachedSkytableValue {
    pub ckey: String,
    pub cvalue: Vec<u8>,
    pub ttl: i64,
}

#[cfg(feature = "caching-skytable")]
impl CachedSkytableValue {
    fn new(ckey: String, cvalue: Vec<u8>, ttl: i64) -> Self {
        Self { ckey, cvalue, ttl }
    }
}

#[cfg(feature = "caching-skytable")]
pub struct SkytableCache<'a, C: ManageConnection, K, V> {
    space: &'a str,
    model: &'a str,
    seconds: u64,
    pool: bb8::Pool<C>,
    refresh: bool,
    _phantom: PhantomData<(K, V)>,
}

#[cfg(feature = "caching-skytable")]
impl<'a, C, I, S, K, V> SkytableCache<'a, C, K, V>
where
    S: AsyncRead + AsyncWrite + Send + Sync + Unpin,
    I: DerefMut<Target = TcpConnection<S>> + Send + Sync,
    C: ManageConnection<Connection = I> + Send + Sync,
{
    fn build_model(&self) -> String {
        format!("{}.{}", self.space, self.model)
    }

    pub async fn init(&self) {
        let mut connection = self.pool.get().await.unwrap();

        let pipeline = Pipeline::new()
            .add(&query!(format!(
                "CREATE SPACE IF NOT EXISTS {}",
                self.space
            )))
            .add(&query!(format!(
                "CREATE MODEL IF NOT EXISTS {}.{}(ckey: string, cvalue: binary, ttl: sint64)",
                self.space, self.model
            )));
        connection.execute_pipeline(&pipeline).await.unwrap();
    }
}

#[cfg(feature = "caching-skytable")]
#[async_trait::async_trait]
impl<'a, C, I, S, K, V> IOCachedAsync<K, V> for SkytableCache<'a, C, K, V>
where
    S: AsyncRead + AsyncWrite + Send + Sync + Unpin,
    I: DerefMut<Target = TcpConnection<S>> + Send + Sync,
    C: ManageConnection<Connection = I> + Send + Sync,
    K: Display + Send + Sync,
    V: Serialize + DeserializeOwned + Send + Sync,
{
    type Error = SkytableCacheError;

    #[instrument(skip_all)]
    async fn cache_get(&self, k: &K) -> std::result::Result<Option<V>, Self::Error> {
        let mut connection = self.pool.get().await?;
        let response: ClientResult<CachedSkytableValue> = connection
            .query_parse(&query!(
                format!("select * from {} where ckey = ?", self.build_model()).as_str(),
                k.to_string()
            ))
            .await;

        match response {
            Ok(response) => {
                if response.ttl < Utc::now().timestamp() {
                    Ok(None)
                } else {
                    if self.refresh {
                        let new_ttl = Utc::now().timestamp() + self.seconds as i64;
                        connection
                            .query(&query!(
                                format!("update {} set ttl = ? where ckey = ?", self.build_model())
                                    .as_str(),
                                new_ttl,
                                k.to_string()
                            ))
                            .await?;
                    }

                    Ok(bincode::deserialize(response.cvalue.as_slice())?)
                }
            }
            Err(error) => match error {
                skytable::error::Error::ServerError(111) => Ok(None),
                error => Err(SkytableCacheError::from(error)),
            },
        }
    }

    #[instrument(skip_all)]
    async fn cache_set(&self, k: K, v: V) -> std::result::Result<Option<V>, Self::Error> {
        let encoded = bincode::serialize(&v)?;
        let ttl = Utc::now().timestamp() + self.seconds as i64;
        let value = CachedSkytableValue::new(k.to_string(), encoded, ttl);

        self.pool
            .get()
            .await?
            .query(&query!(
                format!("insert into {}(?, ?, ?)", self.build_model()).as_str(),
                value
            ))
            .await?;

        Ok(Some(v))
    }

    async fn cache_remove(&self, k: &K) -> std::result::Result<Option<V>, Self::Error> {
        let mut connection = self.pool.get().await?;

        let response: ClientResult<CachedSkytableValue> = connection
            .query_parse(&query!(
                format!("select * from {} where key = ?", self.build_model()).as_str(),
                k.to_string()
            ))
            .await;

        connection
            .query(&query!(
                format!("delete from {} where key = ?", self.build_model()).as_str(),
                k.to_string()
            ))
            .await?;

        match response {
            Ok(response) => Ok(bincode::deserialize(response.cvalue.as_slice())?),
            Err(error) => match error {
                // 111 = NotFound https://docs.skytable.io/protocol/errors
                skytable::error::Error::ServerError(111) => Ok(None),
                error => Err(SkytableCacheError::from(error)),
            },
        }
    }

    fn cache_set_refresh(&mut self, refresh: bool) -> bool {
        let old = self.refresh;
        self.refresh = refresh;
        old
    }
}

#[macro_export]
macro_rules! invalidate {
    ($cache: ident, $key: expr) => {
        paste! {{
            let key = async { $key }.await;    

            tokio::spawn(async move {
                [<invalidate_ $cache>](key).await.unwrap();
            });
        }
    }};
}

#[dynamic_cache(ttl = "300", key = r#"format!('prompt-{}', id)"#)]
pub async fn fetch_prompt(connection: &DatabaseConnection, id: &str) -> Result<Option<Prompt>> {
    let result = database_request!(
        Prompt::select_by_id(connection, id).await,
        "Select prompt by id"
    )?;

    Ok(result)
}

#[dynamic_cache(ttl = "300", key = r#"format!('fields-{}', prompt)"#)]
pub async fn fields_by_prompt(connection: &DatabaseConnection, prompt: &str) -> Result<Vec<Field>> {
    let result = database_request!(
        Field::select_by_column(connection, "prompt", prompt).await,
        "Select fields by prompt"
    )?;

    Ok(result)
}
