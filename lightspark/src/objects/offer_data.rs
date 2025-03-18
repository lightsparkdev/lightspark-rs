// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::bitcoin_network::BitcoinNetwork;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

/// This object represents the data associated with a BOLT #12 offer. You can retrieve this object to receive the relevant data associated with a specific offer.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OfferData {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "offer_data_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "offer_data_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "offer_data_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The requested amount in this invoice. If it is equal to 0, the sender should choose the amount to send.
    #[serde(rename = "offer_data_amount")]
    pub amount: Option<CurrencyAmount>,

    /// The Bech32 encoded offer.
    #[serde(rename = "offer_data_encoded_offer")]
    pub encoded_offer: String,

    /// The bitcoin networks supported by the offer.
    #[serde(rename = "offer_data_bitcoin_networks")]
    pub bitcoin_networks: Vec<BitcoinNetwork>,

    /// The date and time when this invoice will expire.
    #[serde(with = "custom_date_format_option", rename = "offer_data_expires_at")]
    pub expires_at: Option<DateTime<Utc>>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,
}

impl Entity for OfferData {
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
        "OfferData"
    }
}

impl GetEntity for OfferData {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on OfferData {{
                    ... OfferDataFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment OfferDataFragment on OfferData {
    __typename
    offer_data_id: id
    offer_data_created_at: created_at
    offer_data_updated_at: updated_at
    offer_data_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    offer_data_encoded_offer: encoded_offer
    offer_data_bitcoin_networks: bitcoin_networks
    offer_data_expires_at: expires_at
}
";
