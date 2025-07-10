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

use std::{fmt::Debug, num::TryFromIntError};

use crate::prelude::*;
use thiserror::Error;
use tonic::Status;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum FeedbackFusionError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    ConfigurationError(String),
    #[error(transparent)]
    DatabaseError(#[from] rbatis::Error),
    #[error("unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Forbidden(String),
    #[error(transparent)]
    CacheError(#[from] crate::cache::SkytableCacheError),
    #[error("{0}")]
    OIDCError(String),
    #[error(transparent)]
    HCLError(#[from] hcl::Error),
    #[error(transparent)]
    IoError(#[from] tokio::io::Error),
    #[error(transparent)]
    FluvioErrorCode(#[from] fluvio::dataplane::link::ErrorCode),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    TransportError(#[from] tonic::transport::Error),
    #[error(transparent)]
    UriError(#[from] http::uri::InvalidUri),
    #[error(transparent)]
    BincodeEncodeError(#[from] bincode::error::EncodeError),
    #[error(transparent)]
    KanalSendError(#[from] kanal::SendError)
}

impl From<ValidationErrors> for FeedbackFusionError {
    fn from(value: ValidationErrors) -> Self {
        Self::BadRequest(value.to_string())
    }
}

impl From<TryFromIntError> for FeedbackFusionError {
    fn from(value: TryFromIntError) -> Self {
        Self::BadRequest(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, FeedbackFusionError>;

impl From<FeedbackFusionError> for Status {
    fn from(val: FeedbackFusionError) -> Self {
        match val {
            FeedbackFusionError::Unauthorized => Status::unauthenticated("unauthorized"),
            FeedbackFusionError::Forbidden(error) => Status::permission_denied(error.to_string()),
            FeedbackFusionError::BadRequest(error) => Status::invalid_argument(error.to_string()),
            _ => {
                error!("Error occurred while processing the request: {:?}", val);

                Status::internal("Internal server error")
            }
        }
    }
}
