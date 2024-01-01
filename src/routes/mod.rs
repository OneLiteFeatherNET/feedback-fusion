//SPDX-FileCopyrightText: 2023 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2023 OneLiteFeatherNet

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

use aliri_tower::Oauth2Authorizer;
use rbatis::plugin::page::PageRequest;

pub mod oidc;
pub mod v1;

pub async fn router(state: FeedbackFusionState) -> Router {
    let (authorized, unauthorized) = v1::router(state).await;

    // build the authority
    let authority = oidc::authority().await.unwrap();
    let authorizer = Oauth2Authorizer::new()
        .with_claims::<OIDCClaims>()
        .with_terse_error_handler();

    Router::new()
        .nest(
            "/v1",
            authorized
                .layer(authorizer.jwt_layer(authority))
        )
        .nest("/v1", unauthorized)
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchQuery {
    #[serde(default)]
    #[param(nullable)]
    query: String,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    #[param(default = 1)]
    #[serde(default = "page")]
    page: usize,
    #[param(default = 20)]
    #[serde(default = "page_size")]
    page_size: usize,
}

fn page() -> usize {
    1
}

fn page_size() -> usize {
    20
}

impl Pagination {
    pub fn request(self) -> PageRequest {
        PageRequest::new(self.page as u64, self.page_size as u64)
    }

    pub fn eval(&self) -> (u64, u64) {
        let request = self.clone().request();

        (request.page_size(), request.offset())
    }
}
