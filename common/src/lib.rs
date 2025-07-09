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

pub mod database;
pub mod observability;
#[cfg(feature = "arbitrary")]
pub mod tests;

pub mod proto {
    use validator::Validate;
    tonic::include_proto!("feedback_fusion_v1");
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("feedback-fusion-v1-descriptor");
}

#[allow(clippy::module_inception)]
pub mod event {
    tonic::include_proto!("feedback_fusion_event_v1");
}

pub trait PageRequest {
    fn page_request(&self) -> rbatis::plugin::page::PageRequest;
}

pub mod prelude {
    pub use anyhow::anyhow;
    pub use getset::{Getters, Setters};
    pub use itertools::Itertools;
    pub use lazy_static::lazy_static;
    pub use paste::paste;
    pub use serde::{Deserialize, Serialize};
    pub use serde_inline_default::serde_inline_default;
    pub use tracing::{debug, error, info, instrument, trace, warn};
}
