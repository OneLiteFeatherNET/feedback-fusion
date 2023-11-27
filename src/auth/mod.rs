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
    database::schema::{account::{InternalAccount, MachineAccount}, auth::TOTPChallenge},
    prelude::*,
};
use argon2::{Argon2, PasswordVerifier};
use totp_rs::{Algorithm, TOTP};

pub trait Authenticate {
    fn login(&self, password: &str, token: Option<&str>) -> Result<()>;
}

impl Authenticate for InternalAccount {
    #[instrument(skip_all)]
    fn login(&self, password: &str, token: Option<&str>) -> Result<()> {
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
            if let Some(token) = token {
                // verify the token
                TOTP::new(
                    Algorithm::SHA1,
                    6,
                    1,
                    30,
                    self.secret().as_bytes().to_vec(),
                    None,
                    "".to_owned(),
                )
                .unwrap()
                .check_current(token)
                .map_err(|_| FeedbackFusionError::Unauthorized)?;
            } else {
                let challenge = TOTPChallenge::from(self.id().clone());
                return Err(FeedbackFusionError::TOTPChallenge(challenge));
            };
        }

        Ok(())
    }
}

impl Authenticate for MachineAccount {
    #[instrument(skip_all)]
    fn login(&self, password: &str, _token: Option<&str>) -> Result<()>  {
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
