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

#[cfg(feature = "bindings")]
use crate::{database::schema::feedback::*, routes::v1::{*, prompt::*, response::*}};
#[cfg(feature = "bindings")]
use ts_rs::TS;

pub mod database;
pub mod error;
pub mod routes;
pub mod state;
pub mod config;
pub mod prelude;

#[cfg(feature = "bindings")]
macro_rules! export {
    ($($path:path $(,)?)*) => {
        $(
            <$path>::export().unwrap();
        )*
    };
}

#[cfg(feature = "bindings")]
#[derive(serde::Serialize, TS)]
pub struct Page<T: TS> {
    records: Vec<T>,
    total: u16,
    page_no: u16,
}

pub fn main() {
    #[cfg(feature = "bindings")]
    export!(
        FeedbackTarget,
        PutFeedbackTargetRequest,
        FeedbackPrompt,
        PutFeedbackPromptRequest,
        FeedbackPromptField,
        PutFeedbackPromptFieldRequest,
        FeedbackPromptInputType,
        FeedbackPromptField,
        FeedbackPromptInputOptions,
        TextOptions,
        RatingOptions,
        FeedbackPromptResponse,
        FeedbackPromptFieldResponse,
        FeedbackPromptFieldData,
        CreateFeedbackTargetRequest,
        CreateFeedbackPromptRequest,
        CreateFeedbackPromptFieldRequest,
        GetFeedbackPromptResponsesResponseWrapper,
        SubmitFeedbackPromptResponseRequest,
        TextResponse,
        RatingResponse,
        Page<FeedbackPromptField>
    );
}
