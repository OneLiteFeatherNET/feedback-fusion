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

use crate::{
    database::{
        schema::{
            account::{InternalAccount, MachineAccount},
            auth::TOTPChallenge,
        },
        DatabaseConnection,
    },
    prelude::*,
};
use argon2::{Argon2, PasswordVerifier};

pub mod totp;

#[async_trait]
pub trait Authenticate {
    async fn login(&self, password: &str, connection: &DatabaseConnection) -> Result<()>;
}

#[async_trait]
impl Authenticate for InternalAccount {
    #[instrument(skip_all)]
    async fn login(&self, password: &str, connection: &DatabaseConnection) -> Result<()> {
        // compare the password with the stored hash
        Argon2::default()
            .verify_password(
                password.as_bytes(),
                &argon2::PasswordHash::new(self.password_hash().as_str())
                    .map_err(|_| FeedbackFusionError::Unauthorized)?,
            )
            .map_err(|_| FeedbackFusionError::Unauthorized)?;

        // process 2fa
        if *self.totp() {
            // start new totp challenge
            let challenge = TOTPChallenge::start(self.id().clone(), connection).await?;

            Err(FeedbackFusionError::TOTPChallenge(challenge))
        } else {
            Ok(())
        }
    }
}

#[async_trait]
impl Authenticate for MachineAccount {
    #[instrument(skip_all)]
    async fn login(&self, password: &str, _connection: &DatabaseConnection) -> Result<()> {
        // compare the password with the stored hash
        Argon2::default()
            .verify_password(
                password.as_bytes(),
                &argon2::PasswordHash::new(self.password_hash().as_str())
                    .map_err(|_| FeedbackFusionError::Unauthorized)?,
            )
            .map_err(|_| FeedbackFusionError::Unauthorized)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{database::schema::account::InternalAccount, auth::Authenticate};

    #[tokio::test]
    async fn login() {
        let connection = crate::tests::connect().await;

        let account = InternalAccount::builder()
            .username("username")
            .password_hash("password")
            .build();

        assert!(account.login("password", &connection).await.is_ok());
        assert!(account.login("passwor", &connection).await.is_err());
    }
}
