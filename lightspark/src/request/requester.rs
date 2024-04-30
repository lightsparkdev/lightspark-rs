// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::env;

use crate::{
    error::Error,
    key::{OperationSigningKey, RSASigningKey},
    request::auth_provider::AuthProvider,
    types::graphql_requester::GraphQLRequester,
    VERSION,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use os_version::detect;
use rand::RngCore;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{json, to_string, Value};

const DEFAULT_BASE_URL: &str = "https://api.lightspark.com/graphql/server/2023-09-13";

/// A Requester struct for graphql operations.
pub struct Requester {
    client: reqwest::Client,
    base_url: Option<String>,
}

fn user_agent() -> String {
    let rustc_version = match version_check::Version::read() {
        Some(version) => format!("/{}", version),
        None => String::new(),
    };
    let os_version = match detect() {
        Ok(version) => format!("/{}", version.to_string()),
        Err(_) => String::new(),
    };
    format!(
        "lightspark-rs/{} rustc{} {}{}",
        VERSION,
        rustc_version,
        env::consts::OS,
        os_version
    )
}

#[async_trait]
impl GraphQLRequester for Requester {
    /// This executes a graphql operaion without signing.
    ///
    /// Returns the json result for the operation.
    ///
    /// # Arguments
    ///
    /// * `operation` - graphql query or mutation to be executed.
    /// * `variables` - variable for the graphql.
    async fn execute_graphql(
        &self,
        operation: &str,
        variables: Option<Value>,
    ) -> Result<Value, Error> {
        self.execute_graphql_signing::<RSASigningKey>(operation, variables, None)
            .await
    }
}

pub fn build_graphql_request_body(
    operation: &str,
    variables: Option<Value>,
    has_signing_key: bool,
) -> Result<Value, Error> {
    let re = regex::Regex::new(r"\s*(?:query|mutation)\s+(\w+)").map_err(|_| {
        Error::GraphqlError("The operation is not a query or a mutation".to_owned())
    })?;
    let operation_name = re
        .captures(operation)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .map(|s| s.to_owned());

    let mut body = json!({
        "operationName": operation_name,
        "query": operation,
        "nonce": if has_signing_key { Some(rand::thread_rng().next_u64()) } else { None },
        "expires_at": if has_signing_key {
            Some((Utc::now() + Duration::try_hours(1).expect("1 hour should not go out of bounds")).to_rfc3339())
        } else {
            None
        },
    });

    if let Some(vars) = variables {
        body["variables"] = vars
    } else {
        body["variables"] = serde_json::json!({});
    }

    Ok(body)
}

impl Requester {
    pub fn new<T: AuthProvider>(auth_provider: T) -> Result<Self, Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        let auth_header_value = reqwest::header::HeaderValue::from_str(&auth_provider.auth_token())
            .map_err(|err| {
                Error::ClientCreationError(format!(
                    "Auth token cannot convert to HeaderValue: {}",
                    err
                ))
            })?;

        headers.insert(reqwest::header::AUTHORIZATION, auth_header_value);
        let user_agent = user_agent();
        let user_agent_header_value =
            reqwest::header::HeaderValue::from_str(&user_agent).map_err(|err| {
                Error::ClientCreationError(format!(
                    "Auth token cannot convert to HeaderValue: {}",
                    err
                ))
            })?;
        headers.insert("User-Agent", user_agent_header_value.clone());

        headers.insert("X-Lightspark-SDK", user_agent_header_value);

        match reqwest::Client::builder().default_headers(headers).build() {
            Ok(client) => Ok(Requester {
                client,
                base_url: None,
            }),
            Err(err) => Err(Error::ClientCreationError(format!(
                "reqwest client creation error: {}",
                err
            ))),
        }
    }

    pub fn set_base_url(&mut self, base_url: Option<String>) {
        self.base_url = base_url;
    }

    /// This executes a graphql operaion. If the signing_key is provided, the operation will be
    /// signed.
    ///
    /// Returns the json result for the operation.
    ///
    /// # Arguments
    ///
    /// * `operation` - graphql query or mutation to be executed.
    /// * `variables` - variable for the graphql.
    /// * `signing_key` - the node's private signing key.
    pub async fn execute_graphql_signing<T: OperationSigningKey>(
        &self,
        operation: &str,
        variables: Option<Value>,
        signing_key: Option<T>,
    ) -> Result<Value, Error> {
        let payload = build_graphql_request_body(operation, variables, signing_key.is_some())?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        if let Some(op) = payload["operationName"].as_str() {
            if let Ok(value) = HeaderValue::from_str(op) {
                headers.insert("X-GraphQL-Operation", value);
            }
        }

        let json_string = to_string(&payload)
            .map_err(|_| Error::GraphqlError("Failed to serialise payload".to_owned()))?;

        if let Some(key) = signing_key {
            let signing = key
                .sign_payload(&json_string.as_bytes())
                .map_err(Error::CryptoError)?;
            headers.insert(
                "X-Lightspark-Signing",
                HeaderValue::from_str(signing.as_str()).map_err(|_| Error::InvalidHeaderValue)?,
            );
        }

        let body = if json_string.len() > 1024 {
            headers.insert("Content-Encoding", HeaderValue::from_static("zstd"));
            zstd::bulk::compress(json_string.as_bytes(), 0)
                .map_err(|_| Error::GraphqlError("Failed to compress payload".to_owned()))?
        } else {
            json_string.into_bytes()
        };

        let url = match &self.base_url {
            Some(base_url) => base_url.clone(),
            None => DEFAULT_BASE_URL.to_owned(),
        };

        let response = self
            .client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e.to_string()))?;

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| Error::ReqwestError(e.to_string()))?;

        if let Some(_errors) = response_json.get("errors") {
            // Check if there are any errors in the response
            Err(Error::GraphqlError(_errors.to_string()))
        } else if let Some(data) = response_json.get("data") {
            // Return the data field of the response as json
            Ok(data.clone())
        } else {
            Err(Error::GraphqlError("missing data".to_owned()))
        }
    }
}
