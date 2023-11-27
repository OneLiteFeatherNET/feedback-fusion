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

use rbatis::rbdc::DateTime;

use crate::{database::DatabaseConnection, prelude::*};

pub mod session;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TOTPChallengeState {
    Pending,
    Passed,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOTPChallenge {
    id: String,
    pub account: String,
    pub state: TOTPChallengeState,
    created_at: DateTime,
}

crud!(TOTPChallenge {});

impl TOTPChallenge {
    pub async fn start(id: String, connection: &DatabaseConnection) -> Result<Self> {
        let challenge = TOTPChallenge {
            id: nanoid!(),
            account: id,
            state: TOTPChallengeState::Pending,
            created_at: DateTime::now(),
        };

        // insert the challenge
        TOTPChallenge::insert(&connection, &challenge).await?;
        Ok(challenge)
    }
}
