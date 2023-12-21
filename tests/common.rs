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

use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse, Scope,
};
use std::{
    fs::File,
    path::Path,
    process::{Child, Command, Stdio},
};
use tracing::debug;

pub const HTTP_ENDPOINT: &'static str = "http://localhost:8000";

pub struct BackendServer(Child);

impl Drop for BackendServer {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

pub fn run_server() -> BackendServer {
    // construct the executable path
    let mut path = std::env::current_exe().unwrap();
    assert!(path.pop());
    assert!(path.pop());
    path = path.join(env!("CARGO_PKG_NAME"));

    // prepare the command
    let mut command = Command::new(path);
    let stdout = Stdio::from(File::create(Path::new(env!("OUT_DIR")).join("stdout")).unwrap());
    let stderr = Stdio::from(File::create(Path::new(env!("OUT_DIR")).join("stderr")).unwrap());
    debug!("OUT={}", env!("OUT_DIR"));

    command.stdin(Stdio::piped());
    command.stdout(stdout);
    command.stderr(stderr);

    command.env_clear();
    let database = env!("DATABASE");
    let mut env = vec!["_USERNAME", "_PASSWORD", "_ENDPOINT", "_DATABASE"]
        .into_iter()
        .map(|s| format!("{}{}", database, s))
        .collect::<Vec<String>>();
    env.push("OIDC_DISCOVERY_URL".to_owned());

    for key in env.iter() {
        if let Ok(value) = std::env::var(key) {
            debug!("{:?}: {:?}", key, value);
            command.env(key, value);
        }
    }
    command.env("RUST_LOG", "DEBUG");

    BackendServer(command.spawn().unwrap())
}

pub async fn authenticate() -> String {
    let issuer = IssuerUrl::new(env!("OIDC_DISCOVERY_URL").to_owned()).unwrap();
    let metadata = CoreProviderMetadata::discover_async(issuer, async_http_client)
        .await
        .unwrap();
    let client = CoreClient::from_provider_metadata(
        metadata,
        ClientId::new(env!("OIDC_CLIENT_ID").to_owned()),
        Some(ClientSecret::new(env!("OIDC_CLIENT_SECRET").to_owned())),
    );

    let token_response = client
        .exchange_client_credentials()
        .add_scope(Scope::new("api".to_owned()))
        .request_async(async_http_client)
        .await
        .unwrap();

    token_response.access_token().secret().clone()
}
