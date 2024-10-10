// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::error::Error;
use crate::objects::channel_fees::ChannelFees;
use crate::objects::channel_status::ChannelStatus;
use crate::objects::channel_to_transactions_connection::ChannelToTransactionsConnection;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::transaction_type::TransactionType;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::entity_wrapper::EntityWrapper;
use crate::types::get_entity::GetEntity;
use crate::types::graphql_requester::GraphQLRequester;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::vec::Vec;

/// This is an object representing a channel on the Lightning Network. You can retrieve this object to get detailed information on a specific Lightning Network channel.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "channel_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "channel_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "channel_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The transaction that funded the channel upon channel opening.
    #[serde(rename = "channel_funding_transaction")]
    pub funding_transaction: Option<EntityWrapper>,

    /// The total amount of funds in this channel, including the channel balance on the local node, the channel balance on the remote node and the on-chain fees to close the channel.
    #[serde(rename = "channel_capacity")]
    pub capacity: Option<CurrencyAmount>,

    /// The channel balance on the local node.
    #[serde(rename = "channel_local_balance")]
    pub local_balance: Option<CurrencyAmount>,

    /// The channel balance on the local node that is currently allocated to in-progress payments.
    #[serde(rename = "channel_local_unsettled_balance")]
    pub local_unsettled_balance: Option<CurrencyAmount>,

    /// The channel balance on the remote node.
    #[serde(rename = "channel_remote_balance")]
    pub remote_balance: Option<CurrencyAmount>,

    /// The channel balance on the remote node that is currently allocated to in-progress payments.
    #[serde(rename = "channel_remote_unsettled_balance")]
    pub remote_unsettled_balance: Option<CurrencyAmount>,

    /// The channel balance that is currently allocated to in-progress payments.
    #[serde(rename = "channel_unsettled_balance")]
    pub unsettled_balance: Option<CurrencyAmount>,

    /// The total balance in this channel, including the channel balance on both local and remote nodes.
    #[serde(rename = "channel_total_balance")]
    pub total_balance: Option<CurrencyAmount>,

    /// The current status of this channel.
    #[serde(rename = "channel_status")]
    pub status: Option<ChannelStatus>,

    /// The estimated time to wait for the channel's hash timelock contract to expire when force closing the channel. It is in unit of minutes.
    #[serde(rename = "channel_estimated_force_closure_wait_minutes")]
    pub estimated_force_closure_wait_minutes: Option<i64>,

    /// The amount to be paid in fees for the current set of commitment transactions.
    #[serde(rename = "channel_commit_fee")]
    pub commit_fee: Option<CurrencyAmount>,

    /// The fees charged for routing payments through this channel.
    #[serde(rename = "channel_fees")]
    pub fees: Option<ChannelFees>,

    /// If known, the remote node of the channel.
    #[serde(rename = "channel_remote_node")]
    pub remote_node: Option<EntityWrapper>,

    /// The local Lightspark node of the channel.
    #[serde(rename = "channel_local_node")]
    pub local_node: EntityWrapper,

    /// The unique identifier of the channel on Lightning Network, which is the location in the chain that the channel was confirmed. The format is <block-height>:<tx-index>:<tx-output>.
    #[serde(rename = "channel_short_channel_id")]
    pub short_channel_id: Option<String>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,
}

impl Entity for Channel {
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
        "Channel"
    }
}

impl GetEntity for Channel {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Channel {{
                    ... ChannelFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment ChannelFragment on Channel {
    __typename
    channel_id: id
    channel_created_at: created_at
    channel_updated_at: updated_at
    channel_funding_transaction: funding_transaction {
        id
    }
    channel_capacity: capacity {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_local_balance: local_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_local_unsettled_balance: local_unsettled_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_remote_balance: remote_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_remote_unsettled_balance: remote_unsettled_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_unsettled_balance: unsettled_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_total_balance: total_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_status: status
    channel_estimated_force_closure_wait_minutes: estimated_force_closure_wait_minutes
    channel_commit_fee: commit_fee {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_fees: fees {
        __typename
        channel_fees_base_fee: base_fee {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        channel_fees_fee_rate_per_mil: fee_rate_per_mil
    }
    channel_remote_node: remote_node {
        id
    }
    channel_local_node: local_node {
        id
    }
    channel_short_channel_id: short_channel_id
}
";

impl Channel {
    pub async fn get_uptime_percentage(
        &self,
        requester: &impl GraphQLRequester,
        after_date: Option<DateTime<Utc>>,
        before_date: Option<DateTime<Utc>>,
    ) -> Result<Option<i64>, Error> {
        let query = "query FetchChannelUptimePercentage($entity_id: ID!, $after_date: DateTime, $before_date: DateTime) {
    entity(id: $entity_id) {
        ... on Channel {
            uptime_percentage(, after_date: $after_date, before_date: $before_date)
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert("after_date", after_date.map(|dt| dt.to_rfc3339()).into());
        variables.insert("before_date", before_date.map(|dt| dt.to_rfc3339()).into());

        let value = serde_json::to_value(variables).map_err(|err| Error::ConversionError(err))?;
        let result = requester.execute_graphql(&query, Some(value)).await?;
        let json = result["entity"]["uptime_percentage"].clone();
        let result = json.as_i64();
        Ok(result)
    }

    pub async fn get_transactions(
        &self,
        requester: &impl GraphQLRequester,
        types: Option<Vec<TransactionType>>,
        after_date: Option<DateTime<Utc>>,
        before_date: Option<DateTime<Utc>>,
    ) -> Result<ChannelToTransactionsConnection, Error> {
        let query = "query FetchChannelToTransactionsConnection($entity_id: ID!, $types: [TransactionType!], $after_date: DateTime, $before_date: DateTime) {
    entity(id: $entity_id) {
        ... on Channel {
            transactions(, types: $types, after_date: $after_date, before_date: $before_date) {
                __typename
                channel_to_transactions_connection_count: count
                channel_to_transactions_connection_average_fee: average_fee {
                    __typename
                    currency_amount_original_value: original_value
                    currency_amount_original_unit: original_unit
                    currency_amount_preferred_currency_unit: preferred_currency_unit
                    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                }
                channel_to_transactions_connection_total_amount_transacted: total_amount_transacted {
                    __typename
                    currency_amount_original_value: original_value
                    currency_amount_original_unit: original_unit
                    currency_amount_preferred_currency_unit: preferred_currency_unit
                    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                }
                channel_to_transactions_connection_total_fees: total_fees {
                    __typename
                    currency_amount_original_value: original_value
                    currency_amount_original_unit: original_unit
                    currency_amount_preferred_currency_unit: preferred_currency_unit
                    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                }
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert("types", types.into());
        variables.insert("after_date", after_date.map(|dt| dt.to_rfc3339()).into());
        variables.insert("before_date", before_date.map(|dt| dt.to_rfc3339()).into());

        let value = serde_json::to_value(variables).map_err(|err| Error::ConversionError(err))?;
        let result = requester.execute_graphql(&query, Some(value)).await?;
        let json = result["entity"]["transactions"].clone();
        let result = serde_json::from_value(json).map_err(|err| Error::JsonError(err))?;
        Ok(result)
    }
}
