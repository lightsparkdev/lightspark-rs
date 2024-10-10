
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::currency_amount::CurrencyAmount;
use crate::types::entity_wrapper::EntityWrapper;
use crate::objects::entity::Entity;
use crate::types::get_entity::GetEntity;
use crate::types::custom_date_formats::custom_date_format;
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelSnapshot {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "channel_snapshot_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "channel_snapshot_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "channel_snapshot_updated_at")]
    pub updated_at: DateTime<Utc>,

    
    #[serde (rename = "channel_snapshot_local_balance")]
    pub local_balance: Option<CurrencyAmount>,

    
    #[serde (rename = "channel_snapshot_local_unsettled_balance")]
    pub local_unsettled_balance: Option<CurrencyAmount>,

    
    #[serde (rename = "channel_snapshot_remote_balance")]
    pub remote_balance: Option<CurrencyAmount>,

    
    #[serde (rename = "channel_snapshot_remote_unsettled_balance")]
    pub remote_unsettled_balance: Option<CurrencyAmount>,

    
    #[serde (rename = "channel_snapshot_status")]
    pub status: Option<String>,

    
    #[serde(rename = "channel_snapshot_channel")]
    pub channel: EntityWrapper,

    
    #[serde (rename = "channel_snapshot_local_channel_reserve")]
    pub local_channel_reserve: Option<CurrencyAmount>,

    /// The timestamp that was used to query the snapshot of the channel
    #[serde(with = "custom_date_format", rename = "channel_snapshot_timestamp")]
    pub timestamp: DateTime<Utc>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Entity for ChannelSnapshot {

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
        "ChannelSnapshot"
    }
}


impl GetEntity for ChannelSnapshot {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on ChannelSnapshot {{
                    ... ChannelSnapshotFragment
                }}
            }}
        }}

        {}", FRAGMENT)
    }    
}



pub const FRAGMENT: &str = "
fragment ChannelSnapshotFragment on ChannelSnapshot {
    __typename
    channel_snapshot_id: id
    channel_snapshot_created_at: created_at
    channel_snapshot_updated_at: updated_at
    channel_snapshot_local_balance: local_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_snapshot_local_unsettled_balance: local_unsettled_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_snapshot_remote_balance: remote_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_snapshot_remote_unsettled_balance: remote_unsettled_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_snapshot_status: status
    channel_snapshot_channel: channel {
        id
    }
    channel_snapshot_local_channel_reserve: local_channel_reserve {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_snapshot_timestamp: timestamp
}
";



