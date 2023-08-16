// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::{env, fmt};

use crate::{crypto::sign_payload, error::Error, request::auth_provider::AuthProvider, VERSION};
use chrono::{Duration, Utc};
use os_version::detect;
use rand::RngCore;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{json, to_string, Value};

const DEFAULT_BASE_URL: &str = "https://api.lightspark.com/graphql/server/2023-04-04";

#[derive(Debug)]
pub enum RequesterError {
    ReqwestError(reqwest::Error),
    GraphqlError(String),
}

impl fmt::Display for RequesterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReqwestError(err) => write!(f, "Network error {}", err),
            Self::GraphqlError(err) => write!(f, "Graphql error {}", err),
        }
    }
}

impl std::error::Error for RequesterError {}

impl From<reqwest::Error> for RequesterError {
    fn from(error: reqwest::Error) -> Self {
        RequesterError::ReqwestError(error)
    }
}

/// A Requester struct for graphql operations.
pub struct Requester {
    client: reqwest::Client,
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
            Ok(client) => Ok(Requester { client }),
            Err(err) => Err(Error::ClientCreationError(format!(
                "reqwest client creation error: {}",
                err
            ))),
        }
    }

    /// This executes a graphql operaion without signing.
    ///
    /// Returns the json result for the operation.
    ///
    /// # Arguments
    ///
    /// * `operation` - graphql query or mutation to be executed.
    /// * `variables` - variable for the graphql.
    pub async fn execute_graphql(
        &self,
        operation: &str,
        variables: Option<Value>,
    ) -> Result<Value, RequesterError> {
        self.execute_graphql_signing(operation, variables, None)
            .await
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
    pub async fn execute_graphql_signing(
        &self,
        operation: &str,
        variables: Option<Value>,
        signing_key: Option<Vec<u8>>,
    ) -> Result<Value, RequesterError> {
        let re = regex::Regex::new(r"\s*(?:query|mutation)\s+(\w+)").map_err(|_| {
            RequesterError::GraphqlError("The operation is not a query or a mutation".to_owned())
        })?;
        let operation_name = re
            .captures(operation)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .map(|s| s.to_owned());

        let mut body = json!({
            "operationName": operation_name,
            "query": operation,
            "nonce": if signing_key.is_some() { Some(rand::thread_rng().next_u32()) } else { None },
            "expires_at": if signing_key.is_some() {
                Some((Utc::now() + Duration::hours(1)).to_rfc3339())
            } else {
                None
            },
        });

        if let Some(vars) = variables {
            body["variables"] = vars
        } else {
            body["variables"] = serde_json::json!({});
        }

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        if let Some(op) = operation_name {
            if let Ok(value) = HeaderValue::from_str(op.as_str()) {
                headers.insert("X-GraphQL-Operation", value);
            }
        }

        if let Some(key) = signing_key {
            let json_string = to_string(&body)
                .map_err(|_| RequesterError::GraphqlError("Body malformat.".to_owned()))?;
            let payload: Vec<u8> = json_string.into_bytes();
            let signing = sign_payload(payload.as_slice(), key.as_slice());
            if let Ok(signing) = signing {
                if let Ok(value) = HeaderValue::from_str(signing.as_str()) {
                    headers.insert("X-Lightspark-Signing", value);
                }
            }
        }

        let response = self
            .client
            .post(DEFAULT_BASE_URL)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let response_json: Value = response.json().await?;

        if let Some(_errors) = response_json.get("errors") {
            // Check if there are any errors in the response
            Err(RequesterError::GraphqlError(_errors.to_string()))
        } else if let Some(data) = response_json.get("data") {
            // Return the data field of the response as json
            Ok(data.clone())
        } else {
            Err(RequesterError::GraphqlError("missing data".to_owned()))
        }
    }
}
