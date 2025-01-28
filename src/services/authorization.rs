//SPDX-FileCopyrightText: 2023 OneLiteFeatherNet
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

use aliri::jwt::CoreClaims;
use openidconnect::{core::CoreClient, AccessToken};

use crate::{database::schema::user::User, prelude::*};

impl User {
    #[instrument(skip_all)]
    pub async fn get_otherwise_fetch(
        access_token: AccessToken,
        claims: OIDCClaims,
        client: CoreClient,
        connection: &DatabaseConnection,
    ) -> Result<Self> {
        let subject = claims
            .sub()
            .ok_or(FeedbackFusionError::OIDCError(
                "Client is missing sub claim".to_owned(),
            ))?
            .to_string();

        if let Some(user) = get_user(connection, subject.as_str()).await? {
            Ok(user)
        } else {
            let user = User::fetch(access_token, client).await?;

            database_request!(User::insert(connection, &user).await, "Save user")?;
            invalidate!(get_user, format!("get-user-{}", subject.as_str()));

            Ok(user)
        }
    }
}

#[dynamic_cache(ttl = "300", key = r#"format!('get-user-{}', subject)"#)]
async fn get_user(connection: &DatabaseConnection, subject: &str) -> Result<Option<User>> {
    let result = database_request!(
        User::select_by_id(connection, subject).await,
        "Fetch user by subject"
    )?;

    Ok(result)
}
