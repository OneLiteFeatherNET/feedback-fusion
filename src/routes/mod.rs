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
use rbatis::sql::PageRequest;

pub mod feedback;
mod oidc;

pub async fn router(state: FeedbackFusionState) -> Router {
    Router::new().nest("/feedback", feedback::router(state).await)
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct SearchQuery {
    #[serde(default)]
    query: String,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct Pagination {
    #[serde(default = "page")]
    page: usize,
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
}
