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
//! use lightspark::key::RSASigningKey;
//!
//! let api_id = "<your api token id>";
//! let api_token = "<your api token secret>";
//! let auth_provider = AccountAuthProvider::new(api_id.to_string(), api_token.to_string());
//! let client = match LightsparkClient::<RSASigningKey>::new(auth_provider) {
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
pub const VERSION: &str = "0.10.2";

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "base")]
pub mod crypto;
#[cfg(feature = "base")]
pub mod error;
#[cfg(feature = "base")]
pub mod key;
#[cfg(feature = "objects")]
pub mod objects;
#[cfg(feature = "client")]
pub mod request;
#[cfg(feature = "base")]
pub mod types;
#[cfg(feature = "objects")]
pub mod utils;
#[cfg(feature = "webhooks")]
pub mod webhooks;
