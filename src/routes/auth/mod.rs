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

use crate::{
    auth::Authenticate,
    database::schema::{account::Account, auth::session::Session},
    prelude::*,
};

pub fn router(state: FeedbackFusionState) -> Router {
    Router::new().route("/login", post(login)).with_state(state)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InternalLoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TOTPChallengePrompt {
    pub challenge: String,
}

#[utoipa::path(post, path = "/auth/login", request_body = InternalLoginRequest, responses(
    (status = 200, description = "Login successful", body = Session),
    (status = 413, description = "TOTP challenge required", body = TOTPChallengePrompt)
), tag = "Authentication")]
pub async fn login(
    State(state): State<FeedbackFusionState>,
    Json(data): Json<InternalLoginRequest>,
) -> Result<Json<Session>> {
    let connection = state.connection();

    // fetch the account by the username
    if let Account::Internal(account) =
        Account::select_by_username(connection, data.username.as_str())
            .await?
            .ok_or(FeedbackFusionError::Unauthorized)?
    {
        // try to login
        match account.login(data.password.as_str(), connection).await {
            Ok(()) => {
                // start a new Session
                let session = Session::start(account.id().as_str(), connection).await?;

                Ok(Json(session))
            }
            Err(error) => Err(error),
        }
    } else {
        Err(FeedbackFusionError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::tests::TestSuite;

    #[tokio::test]
    async fn test_login() {
        let suite = TestSuite::new().await;

        let response = suite
            .connector()
            .post("/auth/login")
            .json(&serde_json::json! ({
                "username": "wfaf",
                "password": TestSuite::PASSWORD
            }))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response = suite
            .connector()
            .post("/auth/login")
            .json(&serde_json::json!({
                "username": TestSuite::USERNAME,
                "password": "dwadwd"
            }))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response = suite
            .connector()
            .post("/auth/login")
            .json(&serde_json::json!({
                "username": TestSuite::USERNAME,
                "password": TestSuite::PASSWORD
            }))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}

