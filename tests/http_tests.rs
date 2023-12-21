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

use common::*;
use rbatis::sql::Page;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};
use serde::Deserialize;
use test_log::test;

mod common;

#[derive(Clone, Deserialize)]
struct IdResponse {
    id: String,
    name: Option<String>,
}

#[test(tokio::test)]
async fn test_target_endpoints() {
    let _server = run_server();
    let access_token = authenticate().await;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(format!("Bearer {}", access_token).as_str()).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();

    // test auth
    {
        let response = reqwest::Client::default()
            .post(format!("{}/v1/target", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .get(format!("{}/v1/target", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .put(format!("{}/v1/target", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .delete(format!("{}/v1/target/awdawd", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    // test creation
    let id = {
        let response = client
            .post(format!("{}/v1/target", HTTP_ENDPOINT))
            .json(&serde_json::json!({
                "name": "Name",
                "description": "Description"
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::CREATED, response.status());

        let data = response.json::<IdResponse>().await;
        assert!(data.is_ok());

        data.unwrap().id
    };

    // test get by id endpoint
    {
        let response = client
            .get(format!("{}/v1/target/{}", HTTP_ENDPOINT, &id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<IdResponse>().await.unwrap();
        assert_eq!(&id, &data.id);
    }

    // test get page endpoint
    {
        let response = client
            .get(format!("{}/v1/target", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<Page<IdResponse>>().await.unwrap();
        assert_eq!(1, data.records.len());
        let first = data.records.first().unwrap();
        assert_eq!(&id, &first.id);
    }

    // test put endpoint
    {
        let response = client
            .put(format!("{}/v1/target", HTTP_ENDPOINT))
            .json(&serde_json::json!({
                "id": &id,
                "name": "updated"
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!("{}/v1/target/{}", HTTP_ENDPOINT, &id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<IdResponse>().await.unwrap();
        assert_eq!("updated", data.name.unwrap().as_str());
    }

    // test delete endpoint
    {
        let response = client
            .delete(format!("{}/v1/target/{}", HTTP_ENDPOINT, &id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!("{}/v1/target/{}", HTTP_ENDPOINT, &id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }
}
