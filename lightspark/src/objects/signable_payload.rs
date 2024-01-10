
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;
use crate::objects::entity::Entity;
use crate::objects::signable_payload_status::SignablePayloadStatus;
use chrono::{DateTime, Utc};
use crate::types::get_entity::GetEntity;
use crate::types::custom_date_formats::custom_date_format;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignablePayload {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "signable_payload_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "signable_payload_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "signable_payload_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The payload that needs to be signed.
    #[serde (rename = "signable_payload_payload")]
    pub payload: String,

    /// The consistent method for generating the same set of accounts and wallets for a given private key
    #[serde (rename = "signable_payload_derivation_path")]
    pub derivation_path: String,

    /// The status of the payload.
    #[serde (rename = "signable_payload_status")]
    pub status: SignablePayloadStatus,

    /// The tweak value to add.
    #[serde (rename = "signable_payload_add_tweak")]
    pub add_tweak: Option<String>,

    /// The tweak value to multiply.
    #[serde (rename = "signable_payload_mul_tweak")]
    pub mul_tweak: Option<String>,

    /// The signable this payload belongs to.
    #[serde(rename = "signable_payload_signable")]
    pub signable: EntityWrapper,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Entity for SignablePayload {

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
        "SignablePayload"
    }
}


impl GetEntity for SignablePayload {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on SignablePayload {{
                    ... SignablePayloadFragment
                }}
            }}
        }}

        {}", FRAGMENT)
    }    
}



pub const FRAGMENT: &str = "
fragment SignablePayloadFragment on SignablePayload {
    __typename
    signable_payload_id: id
    signable_payload_created_at: created_at
    signable_payload_updated_at: updated_at
    signable_payload_payload: payload
    signable_payload_derivation_path: derivation_path
    signable_payload_status: status
    signable_payload_add_tweak: add_tweak
    signable_payload_mul_tweak: mul_tweak
    signable_payload_signable: signable {
        id
    }
}
";



