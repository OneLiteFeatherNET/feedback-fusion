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

use cached::IOCachedAsync;
use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use skytable::{
    aio::TcpConnection, query, response::Rows, ClientResult, ConnectionAsync, ConnectionTlsAsync,
    Pipeline, Query, Response,
};
use std::{fmt::Display, marker::PhantomData, ops::DerefMut, time::Duration};
use thiserror::Error;
use tokio::{net::TcpStream, sync::Mutex};

// may publish this as crate or submit as pr for cached
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

    pub async fn build(self) -> ClientResult<SkytableCache<'a, ConnectionAsync, K, V>> {
        Ok(SkytableCache {
            space: self.space,
            model: self.model,
            seconds: self.seconds,
            connection: Mutex::new(
                skytable::Config::new(self.host, self.port, self.username, self.password)
                    .connect_async()
                    .await?,
            ),
            refresh: self.refresh,
            _phantom: PhantomData,
        })
    }
}

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

    pub async fn build(self) -> ClientResult<SkytableCache<'a, ConnectionTlsAsync, K, V>> {
        Ok(SkytableCache {
            space: self.space,
            model: self.model,
            seconds: self.seconds,
            connection: Mutex::new(
                skytable::Config::new(self.host, self.port, self.username, self.password)
                    .connect_tls_async(self.certificate)
                    .await?,
            ),
            refresh: self.refresh,
            _phantom: PhantomData,
        })
    }
}

#[derive(Error, Debug)]
pub enum SkytableCacheError {
    #[error(transparent)]
    SkytableError(#[from] skytable::error::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Clone, Query, Response)]
struct CachedSkytableValue {
    pub key: String,
    pub value: String,
    pub version: Option<u64>,
    pub ttl: i64,
}
impl CachedSkytableValue {
    fn new(key: String, value: String, ttl: i64) -> Self {
        Self {
            key,
            value,
            version: Some(1),
            ttl,
        }
    }
}

pub struct SkytableCache<'a, C, K, V> {
    space: &'a str,
    model: &'a str,
    seconds: u64,
    connection: Mutex<C>,
    refresh: bool,
    _phantom: PhantomData<(C, K, V)>,
}

impl<'a, C, K, V> SkytableCache<'a, C, K, V> {
    fn build_model(&self) -> String {
        format!("{}.{}", self.space, self.model)
    }
}

#[async_trait::async_trait]
impl<'a, C, K, V> IOCachedAsync<K, V> for SkytableCache<'a, C, K, V>
where
    C: DerefMut<Target = TcpConnection<TcpStream>> + Send + Sync,
    K: Display + Send + Sync,
    V: Serialize + DeserializeOwned + Send + Sync,
{
    type Error = SkytableCacheError;

    async fn cache_get(&self, k: &K) -> Result<Option<V>, Self::Error> {
        let mut pipeline = Pipeline::new();

        if self.refresh {
            pipeline.push(&query!(
                format!("update {} set ttl = ?", self.build_model()).as_str(),
                Utc::now().timestamp() + self.seconds as i64
            ));
        }

        pipeline.push(&query!(
            format!("select all * from {} where key = ?", self.build_model()).as_str(),
            k.to_string()
        ));
        let response: Rows<CachedSkytableValue> = self
            .connection
            .lock()
            .await
            .execute_pipeline(&pipeline)
            .await?
            .into_iter()
            .last()
            .unwrap()
            .parse()?;

        if response.len() != 1 || response.to_vec().first().unwrap().ttl <= Utc::now().timestamp() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_str(response[0].value.as_str())?))
        }
    }

    async fn cache_set(&self, k: K, v: V) -> Result<Option<V>, Self::Error> {
        let value = CachedSkytableValue::new(
            k.to_string(),
            serde_json::to_string(&v)?,
            Utc::now().timestamp() + self.seconds as i64,
        );
        self.connection
            .lock()
            .await
            .query(&query!(
                format!("insert into {}(?, ?, ?)", self.build_model()).as_str(),
                value
            ))
            .await?;

        Ok(Some(v))
    }

    async fn cache_remove(&self, k: &K) -> Result<Option<V>, Self::Error> {
        let pipeline = Pipeline::new()
            .add(&query!(
                format!("select * all from {} where key = ?", self.build_model()).as_str(),
                k.to_string()
            ))
            .add(&query!(
                format!("delete from {} where key = ?", self.build_model()).as_str(),
                k.to_string()
            ));

        let response: Rows<CachedSkytableValue> = self
            .connection
            .lock()
            .await
            .execute_pipeline(&pipeline)
            .await?
            .into_iter()
            .next()
            .unwrap()
            .parse()?;

        if response.len() != 1 || response.to_vec().first().unwrap().ttl <= Utc::now().timestamp() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_str(response[0].value.as_str())?))
        }
    }

    fn cache_set_refresh(&mut self, refresh: bool) -> bool {
        let old = self.refresh;
        self.refresh = refresh;
        old
    }
}
