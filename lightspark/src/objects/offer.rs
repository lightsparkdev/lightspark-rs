// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::entity_wrapper::EntityWrapper;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object represents a BOLT #12 offer (https://github.com/lightning/bolts/blob/master/12-offer-encoding.md) created by a Lightspark Node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Offer {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "offer_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "offer_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "offer_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The details of the offer.
    #[serde(rename = "offer_data")]
    pub data: EntityWrapper,

    /// The BOLT12 encoded offer. Starts with 'lno'.
    #[serde(rename = "offer_encoded_offer")]
    pub encoded_offer: String,

    /// The amount of the offer. If null, the payer chooses the amount.
    #[serde(rename = "offer_amount")]
    pub amount: Option<CurrencyAmount>,

    /// The description of the offer.
    #[serde(rename = "offer_description")]
    pub description: Option<String>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,
}

impl Entity for Offer {
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
        "Offer"
    }
}

impl GetEntity for Offer {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Offer {{
                    ... OfferFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment OfferFragment on Offer {
    __typename
    offer_id: id
    offer_created_at: created_at
    offer_updated_at: updated_at
    offer_data: data {
        id
    }
    offer_encoded_offer: encoded_offer
    offer_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    offer_description: description
}
";
