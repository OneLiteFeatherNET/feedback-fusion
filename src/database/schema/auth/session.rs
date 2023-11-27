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

use crate::{prelude::*, database::DatabaseConnection};

#[derive(Deserialize, Serialize, Debug, PartialEq, ToSchema, Clone)]
pub struct Session {
    id: String,
    account: String,
    iat: i64,
    exp: i64,
    refresh_token: String,
    refresh_exp: i64
}

impl_insert!(Session {});
impl_delete!(Session {});
impl_select!(Session {select_by_id(id: &str) -> Option => "`where id = #{id} limit 1`"});

impl Session {
    #[instrument(skip(connection))]
    pub async fn start(account: &str, connection: &DatabaseConnection) -> Result<Self> {
        let now = chrono::Utc::now().timestamp_millis();

        let session = Self {
            id: nanoid!(32),
            account: account.to_string(),
            iat: now,
            exp: now + *CONFIG.session_length() as i64,
            refresh_token: nanoid!(48),
            refresh_exp: now + *CONFIG.session_length() as i64
        };

        Session::insert(connection, &session).await?;
        Ok(session)
    }

    #[instrument(skip_all)]
    async fn end(&self, connection: &DatabaseConnection) -> Result<()> {
        // delete the session 
        Session::delete_by_column(&connection, "id", self.id.as_str()).await?;

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn is_valid(&self, connection: &DatabaseConnection) -> Result<()> {
        if chrono::Utc::now().timestamp_millis() >= self.exp {
            Err(FeedbackFusionError::Unauthorized)
        } else {
            Ok(())
        }
    }

    #[instrument(skip(connection))]
    pub async fn is_session_valid(id: &str, connection: &DatabaseConnection) -> Result<Session> {
        // try to fetch the session 
        let session: Option<Session> = Session::select_by_id(connection, id).await?;
        return if let Some(session) = session {
            session.is_valid(connection).await?;

            Ok(session)
        } else {
            Err(FeedbackFusionError::Unauthorized)
        }
    }

    #[instrument(skip_all)]
    pub async fn refresh(&self, refresh_token: &str, connection: &DatabaseConnection) -> Result<Session> {
        if refresh_token == self.refresh_token {
            // end the current session 
            self.end(connection).await?;

            Session::start(self.account.as_str(), connection).await
        } else {
            Err(FeedbackFusionError::Unauthorized)
        }
    }
}

