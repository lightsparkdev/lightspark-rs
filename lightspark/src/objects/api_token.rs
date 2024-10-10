
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::entity::Entity;
use crate::types::get_entity::GetEntity;
use std::vec::Vec;
use crate::objects::permission::Permission;
use crate::types::custom_date_formats::custom_date_format;
use crate::objects::audit_log_actor::AuditLogActor;
use chrono::{DateTime, Utc};

/// This is an object representing a Lightspark API token, that can be used to authenticate this account when making API calls or using our SDKs. See the “Authentication” section of our API docs for more details on its usage.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiToken {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "api_token_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "api_token_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "api_token_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// An opaque identifier that should be used as a client_id (or username) in the HTTP Basic Authentication scheme when issuing requests against the Lightspark API.
    #[serde (rename = "api_token_client_id")]
    pub client_id: String,

    /// An arbitrary name chosen by the creator of the token to help identify the token in the list of tokens that have been created for the account.
    #[serde (rename = "api_token_name")]
    pub name: String,

    /// A list of permissions granted to the token.
    #[serde (rename = "api_token_permissions")]
    pub permissions: Vec<Permission>,

    /// Whether the api token has been deleted.
    #[serde (rename = "api_token_is_deleted")]
    pub is_deleted: bool,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl AuditLogActor for ApiToken {


    fn type_name(&self) -> &'static str {
        "ApiToken"
    }
}



impl Entity for ApiToken {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String {
        self.id.clone()
    }

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }


    fn type_name(&self) -> &'static str {
        "ApiToken"
    }
}


impl GetEntity for ApiToken {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on ApiToken {{
                    ... ApiTokenFragment
                }}
            }}
        }}

        {}", FRAGMENT)
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
    api_token_is_deleted: is_deleted
}
";



