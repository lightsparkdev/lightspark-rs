
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;
use crate::objects::entity::Entity;
use chrono::{DateTime, Utc};
use crate::types::get_entity::GetEntity;
use crate::types::custom_date_formats::custom_date_format;
use crate::objects::currency_amount::CurrencyAmount;

/// This object represents a specific node that existed on a particular payment route. You can retrieve this object to get information about a node on a particular payment path and all payment-relevant information for that node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hop {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "hop_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "hop_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "hop_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The destination node of the hop.
    #[serde(rename = "hop_destination")]
    pub destination: Option<EntityWrapper>,

    /// The zero-based index position of this hop in the path
    #[serde (rename = "hop_index")]
    pub index: i64,

    /// The public key of the node to which the hop is bound.
    #[serde (rename = "hop_public_key")]
    pub public_key: Option<String>,

    /// The amount that is to be forwarded to the destination node.
    #[serde (rename = "hop_amount_to_forward")]
    pub amount_to_forward: Option<CurrencyAmount>,

    /// The fees to be collected by the source node for forwarding the payment over the hop.
    #[serde (rename = "hop_fee")]
    pub fee: Option<CurrencyAmount>,

    /// The block height at which an unsettled HTLC is considered expired.
    #[serde (rename = "hop_expiry_block_height")]
    pub expiry_block_height: Option<i64>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Entity for Hop {

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
        "Hop"
    }
}


impl GetEntity for Hop {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Hop {{
                    ... HopFragment
                }}
            }}
        }}

        {}", FRAGMENT)
    }    
}



pub const FRAGMENT: &str = "
fragment HopFragment on Hop {
    __typename
    hop_id: id
    hop_created_at: created_at
    hop_updated_at: updated_at
    hop_destination: destination {
        id
    }
    hop_index: index
    hop_public_key: public_key
    hop_amount_to_forward: amount_to_forward {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    hop_fee: fee {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    hop_expiry_block_height: expiry_block_height
}
";



