// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::entity::Entity;
use crate::objects::permission::Permission;
use crate::types::custom_date_format::custom_date_format;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::vec::Vec;

#[derive(Deserialize)]
pub struct ApiToken {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "api_token_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "api_token_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "api_token_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// An opaque identifier that should be used as a client_id (or username) in the HTTP Basic Authentication scheme when issuing requests against the Lightspark API.
    #[serde(rename = "api_token_client_id")]
    pub client_id: String,

    /// An arbitrary name chosen by the creator of the token to help identify the token in the list of tokens that have been created for the account.
    #[serde(rename = "api_token_name")]
    pub name: String,

    /// A list of permissions granted to the token.
    #[serde(rename = "api_token_permissions")]
    pub permissions: Vec<Permission>,
}

impl Entity for ApiToken {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String {
        return self.id.clone();
    }

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc> {
        return self.created_at;
    }

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc> {
        return self.updated_at;
    }

    fn type_name(&self) -> &'static str {
        "ApiToken"
    }
}

impl GetEntity for ApiToken {
    fn get_entity_query() -> String {
        return format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on ApiToken {{
                    ... ApiTokenFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        );
    }
}

pub const FRAGMENT: &str = "
fragment ApiTokenFragment on ApiToken {
    __typename
    api_token_id: id
    api_token_created_at: created_at
    api_token_updated_at: updated_at
    api_token_client_id: client_id
    api_token_name: name
    api_token_permissions: permissions
}
";
