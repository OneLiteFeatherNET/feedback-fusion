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

use aliri_oauth2::Authority;
use aliri_tower::OnJwtError;

use http::Response;
use tokio::runtime::Handle;
use tonic::{Status, body::Body};

#[derive(Clone, Copy, Debug)]
pub struct AuthorizedService<S>(pub S);

impl<S, T> tonic::server::NamedService
    for AuthorizedService<tower_http::validate_request::ValidateRequestHeader<S, T>>
where
    S: tonic::server::NamedService,
{
    const NAME: &'static str = S::NAME;
}

impl<S, R> tower::Service<R> for AuthorizedService<S>
where
    S: tower::Service<R>,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    #[inline]
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: R) -> Self::Future {
        self.0.call(req)
    }
}

#[derive(Clone)]
pub struct OIDCErrorHandler {
    authority: Authority,
}

impl From<Authority> for OIDCErrorHandler {
    fn from(authority: Authority) -> Self {
        Self { authority }
    }
}

impl OnJwtError for OIDCErrorHandler {
    type Body = Body;

    fn on_jwt_invalid(&self, _error: aliri::error::JwtVerifyError) -> Response<Self::Body> {
        Status::unauthenticated("unauthenticated").into_http()
    }

    fn on_no_matching_jwk(&self) -> Response<Self::Body> {
        warn!("No matching jwk for request found, refreshing jwks...");

        let handle = Handle::current();
        let authority = self.authority.clone();

        // theoretically aliri does spawn a thread itself but as it requires a async runtime
        // context we do have to spawn a new runtim thread ourselves
        handle.spawn(async move { authority.refresh().await.ok() });

        Status::unauthenticated("unauthenticated").into_http()
    }

    fn on_missing_or_malformed(&self) -> Response<Self::Body> {
        Status::unauthenticated("unauthenticated").into_http()
    }
}
