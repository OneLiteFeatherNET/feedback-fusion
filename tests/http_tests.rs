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
use reqwest::StatusCode;
use serde::Deserialize;
use test_log::test;

mod common;

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct TargetResponse {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct PromptResponse {
    id: String,
    target: String,
    active: bool,
    title: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct FieldResponse {
    id: String,
    prompt: String,
    title: String,
}

#[test(tokio::test)]
async fn test_target_endpoints() {
    let _server = run_server();
    let client = client().await;

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
    let target = {
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

        let data = response.json::<TargetResponse>().await;
        assert!(data.is_ok());

        data.unwrap()
    };

    // test get by id endpoint
    {
        let response = client
            .get(format!("{}/v1/target/{}", HTTP_ENDPOINT, &target.id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<TargetResponse>().await.unwrap();
        assert_eq!(&target, &data);
    }

    // test get page endpoint
    {
        let response = client
            .get(format!("{}/v1/target", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<Page<TargetResponse>>().await.unwrap();
        assert_eq!(1, data.records.len());
        let first = data.records.first().unwrap();
        assert_eq!(&target, first);
    }

    // test put endpoint
    {
        let response = client
            .put(format!("{}/v1/target/{}", HTTP_ENDPOINT, &target.id))
            .json(&serde_json::json!({
                "name": "updated"
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!("{}/v1/target/{}", HTTP_ENDPOINT, &target.id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<TargetResponse>().await.unwrap();
        assert_eq!("updated", data.name.as_str());
    }

    // test delete endpoint
    {
        let response = client
            .delete(format!("{}/v1/target/{}", HTTP_ENDPOINT, &target.id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!("{}/v1/target/{}", HTTP_ENDPOINT, &target.id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }
}

#[test(tokio::test)]
async fn test_prompt_endpoints() {
    let _server = run_server();
    let client = client().await;
    // prepare dependencies
    let response = client
        .post(format!("{}/v1/target", HTTP_ENDPOINT))
        .json(&serde_json::json!({
            "name": "Name"
        }))
        .send()
        .await
        .unwrap();
    let target = response.json::<TargetResponse>().await.unwrap();

    // test auth
    {
        let response = reqwest::Client::default()
            .post(format!("{}/v1/target/test/prompt", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .get(format!("{}/v1/target/test/prompt", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .put(format!("{}/v1/target/test/prompt/test", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .delete(format!("{}/v1/target/test/prompt/test", HTTP_ENDPOINT))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    // test post
    let prompt = {
        let response = client
            .post(format!("{}/v1/target/{}/prompt", HTTP_ENDPOINT, &target.id))
            .json(&serde_json::json!({
                "title": "title"
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::CREATED, response.status());

        let data = response.json::<PromptResponse>().await;
        assert!(data.is_ok());

        data.unwrap()
    };

    // test get
    {
        let response = client
            .get(format!("{}/v1/target/{}/prompt", HTTP_ENDPOINT, &target.id))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<Page<PromptResponse>>().await.unwrap();
        assert_eq!(1, data.records.len());
        assert_eq!(&prompt, data.records.first().unwrap());
    }

    // test put
    {
        let response = client
            .put(format!(
                "{}/v1/target/{}/prompt/{}",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .json(&serde_json::json!({
                "active": false
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!("{}/v1/target/{}/prompt", HTTP_ENDPOINT, &target.id))
            .send()
            .await
            .unwrap();
        assert_eq!(
            false,
            response
                .json::<Page<PromptResponse>>()
                .await
                .unwrap()
                .records
                .first()
                .unwrap()
                .active
        );
    }

    // test delete
    {
        let response = client
            .delete(format!(
                "{}/v1/target/{}/prompt/{}",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!("{}/v1/target/{}/prompt", HTTP_ENDPOINT, &target.id))
            .send()
            .await
            .unwrap();
        assert_eq!(
            0,
            response
                .json::<Page<PromptResponse>>()
                .await
                .unwrap()
                .records
                .len()
        );
    }
}

#[test(tokio::test)]
async fn test_prompt_field_endpoints() {
    let _server = run_server();
    let client = client().await;

    // prepare dependencies
    let (target, prompt) = {
        let target = {
            let response = client
                .post(format!("{}/v1/target", HTTP_ENDPOINT))
                .json(&serde_json::json!({
                    "name": "Name"
                }))
                .send()
                .await
                .unwrap();
            response.json::<TargetResponse>().await.unwrap()
        };

        let prompt = {
            let response = client
                .post(format!("{}/v1/target/{}/prompt", HTTP_ENDPOINT, &target.id))
                .json(&serde_json::json!({
                    "title": "title"
                }))
                .send()
                .await
                .unwrap();
            response.json::<PromptResponse>().await.unwrap()
        };

        (target, prompt)
    };

    // test auth
    {
        let response = reqwest::Client::default()
            .post(format!(
                "{}/v1/target/test/prompt/test/field",
                HTTP_ENDPOINT
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .get(format!(
                "{}/v1/target/test/prompt/test/field",
                HTTP_ENDPOINT
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .put(format!(
                "{}/v1/target/test/prompt/test/field/test",
                HTTP_ENDPOINT
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let response = reqwest::Client::default()
            .delete(format!(
                "{}/v1/target/test/prompt/test/field/test",
                HTTP_ENDPOINT
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    // test post
    let field = {
        // test wrong type
        let response = client
            .post(format!(
                "{}/v1/target/{}/prompt/{}/field",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .json(&serde_json::json!({
                "title": "test",
                "type": "text",
                "options": {"max": 5, "description": "hell yea"}
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::BAD_REQUEST, response.status());

        // test insert
        let response = client
            .post(format!(
                "{}/v1/target/{}/prompt/{}/field",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .json(&serde_json::json!({
                "title": "Test",
                "type": "text",
                "options": {
                    "placeholder": "placeholder",
                    "description": "description",
                }
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::CREATED, response.status());

        let field = response.json::<FieldResponse>().await.unwrap();
        assert_eq!(&prompt.id, &field.prompt);
        field
    };

    // test get
    {
        let response = client
            .get(format!(
                "{}/v1/target/{}/prompt/{}/field",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let data = response.json::<Page<FieldResponse>>().await.unwrap();
        assert_eq!(1, data.total);
        assert_eq!(&&field, data.records.first().as_ref().unwrap());
    }

    // test put
    {
        // test put invalid options
        let response = client
            .put(format!(
                "{}/v1/target/{}/prompt/{}/field/{}",
                HTTP_ENDPOINT, &target.id, &prompt.id, &field.id
            ))
            .json(&serde_json::json!({
                "options": {"max": 5, "description": "test"}
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::BAD_REQUEST, response.status());

        // test put title
        let response = client
            .put(format!(
                "{}/v1/target/{}/prompt/{}/field/{}",
                HTTP_ENDPOINT, &target.id, &prompt.id, &field.id
            ))
            .json(&serde_json::json!({ "title": "Updated" }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!(
                "{}/v1/target/{}/prompt/{}/field",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .send()
            .await
            .unwrap();
        let data = response.json::<Page<FieldResponse>>().await.unwrap();
        assert_eq!("Updated", data.records.first().unwrap().title.as_str());
    }

    // test delete
    {
        let response = client
            .delete(format!(
                "{}/v1/target/{}/prompt/{}/field/{}",
                HTTP_ENDPOINT, &target.id, &prompt.id, &field.id
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());

        let response = client
            .get(format!(
                "{}/v1/target/{}/prompt/{}/field",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .send()
            .await
            .unwrap();
        let page = response.json::<Page<FieldResponse>>().await.unwrap();
        assert_eq!(0, page.total);
    }
}

#[test(tokio::test)]
async fn test_response_endpoints() {
    let _server = run_server();
    let client = client().await;

    let (target, prompt) = {
        let target = {
            let response = client
                .post(format!("{}/v1/target", HTTP_ENDPOINT))
                .json(&serde_json::json!({
                    "name": "Name"
                }))
                .send()
                .await
                .unwrap();
            response.json::<TargetResponse>().await.unwrap()
        };

        let prompt = {
            let response = client
                .post(format!("{}/v1/target/{}/prompt", HTTP_ENDPOINT, &target.id))
                .json(&serde_json::json!({
                    "title": "title"
                }))
                .send()
                .await
                .unwrap();
            response.json::<PromptResponse>().await.unwrap()
        };

        (target, prompt)
    };

    // test auth
    {
        let response = reqwest::Client::default()
            .post(format!(
                "{}/v1/target/{}/prompt/{}/response",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNSUPPORTED_MEDIA_TYPE, response.status());

        let response = reqwest::Client::default()
            .get(format!(
                "{}/v1/target/{}/prompt/{}/response",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    // test post response on empty prompt and non existing prompt
    {
        let response = client
            .post(format!(
                "{}/v1/target/{}/prompt/test/response",
                HTTP_ENDPOINT, &target.id
            ))
            .json(&serde_json::json!({ "responses": {} }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::BAD_REQUEST, response.status());

        let response = client
            .post(format!(
                "{}/v1/target/{}/prompt/{}/response",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .json(&serde_json::json!({ "responses": {} }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    // create testing fields
    let (text_field, rating_field) = {
        let response = client
            .post(format!(
                "{}/v1/target/{}/prompt/{}/field",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .json(&serde_json::json!({
                "title": "Test",
                "type": "text",
                "options": {
                    "placeholder": "placeholder",
                    "description": "description",
                }
            }))
            .send()
            .await
            .unwrap();
        let text_field = response.json::<FieldResponse>().await.unwrap();

        let response = client
            .post(format!(
                "{}/v1/target/{}/prompt/{}/field",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .json(&serde_json::json!({
                "title": "Test",
                "type": "rating",
                "options": {
                    "max": 10,
                    "description": "description",
                }
            }))
            .send()
            .await
            .unwrap();
        let rating_field = response.json::<FieldResponse>().await.unwrap();

        (text_field, rating_field)
    };

    // test post response
    {
        let response = client
            .post(format!(
                "{}/v1/target/{}/prompt/{}/response",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .json(&serde_json::json!({
                "responses": {
                    &text_field.id: {"data": "Yea"},
                    &rating_field.id: {"data": 5}
                }
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::CREATED, response.status());
    }

    // test get
    {
        let response = client
            .get(format!(
                "{}/v1/target/{}/prompt/{}/response",
                HTTP_ENDPOINT, &target.id, &prompt.id
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status());
    }
}
