// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
//!
//! Lightspark Rust SDK is a thin wrapper around the GraphQL API that provides an easy way to
//! integrate with the Lightspark systems in a Rust environment.
//!
//! # Getting Started
//!
//! After obtain an API token, a client can be initiate.
//! ```
//! use lightspark::request::auth_provider::AccountAuthProvider;
//! use lightspark::client::LightsparkClient;
//!
//! let api_id = "<your api token id>";
//! let api_token = "<your api token secret>";
//! let auth_provider = AccountAuthProvider::new(api_id.to_string(), api_token.to_string());
//! let client = match LightsparkClient::new(auth_provider) {
//!    Ok(value) => value,
//!    Err(err) => {
//!       println!("{}", err);
//!       return;
//!    }
//! };
//! ```
//!
//! You are now ready to use the Lightspark SDK!
//!
//! See more examples in examples/example.rs
//!
/// The version of this library.
pub const VERSION: &str = "0.5.0";

pub mod client;
pub mod crypto;
pub mod error;
pub mod objects;
pub mod request;
pub mod types;
pub mod uma;
pub mod webhooks;
