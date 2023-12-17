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

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;


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
}

#[derive(Serialize, Debug, Clone)]
pub struct FeedbackFusionErrorResponse {
    error: String,
}

impl From<String> for FeedbackFusionErrorResponse {
    fn from(value: String) -> Self {
        Self { error: value }
    }
}

pub type Result<T> = std::result::Result<T, FeedbackFusionError>;

impl IntoResponse for FeedbackFusionError {
    fn into_response(self) -> Response {
        match self {
            FeedbackFusionError::BadRequest(error) => (
                StatusCode::BAD_REQUEST,
                Json(FeedbackFusionErrorResponse::from(error)),
            ),
            FeedbackFusionError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(FeedbackFusionErrorResponse::from("Unauthorized".to_owned())),
            ),
            _ => {
                error!("Error occurred while processing request: {:?}", self);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(FeedbackFusionErrorResponse::from(
                        "Internal server error".to_owned(),
                    )),
                )
            }
        }
        .into_response()
    }
}
