// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use base64::{engine::general_purpose, Engine as _};
use std::string::String;
pub trait AuthProvider {
    fn auth_token(&self) -> String;
}

pub struct AccountAuthProvider {
    account_id: String,
    account_token: String,
}

impl AuthProvider for AccountAuthProvider {
    fn auth_token(&self) -> String {
        let string = format!("{}:{}", self.account_id, self.account_token);
        format!("Basic {}", general_purpose::STANDARD.encode(string))
    }
}

impl AccountAuthProvider {
    pub fn new(account_id: String, account_token: String) -> Self {
        AccountAuthProvider {
            account_id,
            account_token,
        }
    }
}
