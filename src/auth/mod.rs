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

use crate::database::schema::account::InternalAccount;
use crate::prelude::*;
use argon2::password_hash::SaltString;
use argon2::Argon2;

#[async_trait]
pub trait Authenticate {
    async fn login(&self, password: &str, token: Option<&str>) -> Result<()>;
    async fn derive_key(&self, password: &str) -> Result<[u8; 32]>;
    async fn secret(&self, password: &str) -> Result<String>;
    async fn regenerate_secret(&self, password: &str) -> Result<()>;
}

#[instrument(skip_all)]
pub fn derive_key(password: &str, salt: &SaltString) -> Result<[u8; 32]> {
    let mut buffer = [0u8; 32];

    Argon2::default()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut buffer)
        .map_err(|_| FeedbackFusionError::Unauthorized)?;

    Ok(buffer)
}

#[async_trait]
impl Authenticate for InternalAccount {
    #[instrument(skip_all)]
    async fn login(&self, password: &str, token: Option<&str>) -> Result<()> {
        todo!()
    }

    #[instrument(skip_all)]
    async fn derive_key(&self, password: &str) -> Result<[u8; 32]> {
        let salt = SaltString::from_b64(self.salt().as_str())
            .map_err(|_| FeedbackFusionError::Unauthorized)?;

        derive_key(password, &salt)
    }

    #[instrument(skip_all)]
    async fn secret(&self, password: &str) -> Result<String> {
        todo!()
    }

    #[instrument(skip_all)]
    async fn regenerate_secret(&self, password: &str) -> Result<()> {
        todo!()
    }
}
