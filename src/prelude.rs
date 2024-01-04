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

pub use crate::{
    config::*,
    database::{DatabaseConfiguration, DatabaseConnection},
    database_request,
    error::*,
    impl_select_page_wrapper,
    routes::{oidc::*, *},
    state::FeedbackFusionState,
};
pub use axum::{
    extract::{Json, Query, State},
    routing::*,
    Router,
};
pub use derivative::Derivative;
pub use getset::{Getters, MutGetters, Setters};
pub use lazy_static::lazy_static;
pub use paste::paste;
pub use rbatis::{
    crud, impl_insert, impl_select, impl_select_page, impled, plugin::page::Page, py_sql,
    rbdc::JsonV, IPageRequest,
};
pub use serde::{Deserialize, Serialize};
pub use tracing::{debug, error, info, info_span, warn};
pub use typed_builder::TypedBuilder;
pub use utoipa::{IntoParams, ToSchema};
pub use validator::Validate;

#[cfg(feature = "bindings")]
pub use ts_rs::TS;
