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

use axum_test_helper::TestClient;

use crate::database::{
    schema::account::{Account, InternalAccount},
    BaseConfiguration, DatabaseConfiguration, DatabaseConnection,
};

pub async fn connect() -> DatabaseConnection {
    fast_log::init(fast_log::Config::new().console()).ok();
    let config = envy::from_env::<BaseConfiguration>().unwrap();
    let connection = DatabaseConfiguration::Postgres(config)
        .connect()
        .await
        .unwrap();

    connection
}

#[derive(Getters)]
#[get = "pub"]
pub struct TestSuite {
    connector: TestClient,
    connection: DatabaseConnection,
    account: Account,
}

impl TestSuite {
    pub const USERNAME: &'static str = "username";
    pub const PASSWORD: &'static str = "password";

    pub async fn new() -> Self {
        let connection = connect().await;

        // create a new account
        let account = Account::Internal(InternalAccount::builder()
            .username(Self::USERNAME)
            .password_hash(Self::PASSWORD)
            .build());
        Account::insert(&connection, &account).await.unwrap();
    

        // init the client connector
        let connector = TestClient::new(crate::router(connection.clone()));

        Self {
            connector,
            connection,
            account
        }
    }
}

