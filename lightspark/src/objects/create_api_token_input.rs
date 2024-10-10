// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::permission::Permission;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateApiTokenInput {
    /// An arbitrary name that the user can choose to identify the API token in a list.
    pub name: String,

    /// List of permissions to grant to the API token
    pub permissions: Vec<Permission>,
}
