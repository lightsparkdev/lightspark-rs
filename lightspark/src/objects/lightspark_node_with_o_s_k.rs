// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::error::Error;
use crate::objects::balances::Balances;
use crate::objects::bitcoin_network::BitcoinNetwork;
use crate::objects::blockchain_balance::BlockchainBalance;
use crate::objects::channel_status::ChannelStatus;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::lightspark_node::LightsparkNode;
use crate::objects::lightspark_node_status::LightsparkNodeStatus;
use crate::objects::lightspark_node_to_channels_connection::LightsparkNodeToChannelsConnection;
use crate::objects::node::Node;
use crate::objects::node_address_type::NodeAddressType;
use crate::objects::node_to_addresses_connection::NodeToAddressesConnection;
use crate::objects::secret::Secret;
use crate::types::get_entity::GetEntity;
use crate::types::graphql_requester::GraphQLRequester;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::vec::Vec;

use crate::types::custom_date_formats::custom_date_format;
use crate::types::entity_wrapper::EntityWrapper;

/// This is a Lightspark node with OSK.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LightsparkNodeWithOSK {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "lightspark_node_with_o_s_k_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(
        with = "custom_date_format",
        rename = "lightspark_node_with_o_s_k_created_at"
    )]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(
        with = "custom_date_format",
        rename = "lightspark_node_with_o_s_k_updated_at"
    )]
    pub updated_at: DateTime<Utc>,

    /// A name that identifies the node. It has no importance in terms of operating the node, it is just a way to identify and search for commercial services or popular nodes. This alias can be changed at any time by the node operator.
    #[serde(rename = "lightspark_node_with_o_s_k_alias")]
    pub alias: Option<String>,

    /// The Bitcoin Network this node is deployed in.
    #[serde(rename = "lightspark_node_with_o_s_k_bitcoin_network")]
    pub bitcoin_network: BitcoinNetwork,

    /// A hexadecimal string that describes a color. For example "#000000" is black, "#FFFFFF" is white. It has no importance in terms of operating the node, it is just a way to visually differentiate nodes. That color can be changed at any time by the node operator.
    #[serde(rename = "lightspark_node_with_o_s_k_color")]
    pub color: Option<String>,

    /// A summary metric used to capture how well positioned a node is to send, receive, or route transactions efficiently. Maximizing a node's conductivity helps a node’s transactions to be capital efficient. The value is an integer ranging between 0 and 10 (bounds included).
    #[serde(rename = "lightspark_node_with_o_s_k_conductivity")]
    pub conductivity: Option<i64>,

    /// The name of this node in the network. It will be the most human-readable option possible, depending on the data available for this node.
    #[serde(rename = "lightspark_node_with_o_s_k_display_name")]
    pub display_name: String,

    /// The public key of this node. It acts as a unique identifier of this node in the Lightning Network.
    #[serde(rename = "lightspark_node_with_o_s_k_public_key")]
    pub public_key: Option<String>,

    /// The owner of this LightsparkNode.
    #[serde(rename = "lightspark_node_with_o_s_k_owner")]
    pub owner: EntityWrapper,

    /// The current status of this node.
    #[serde(rename = "lightspark_node_with_o_s_k_status")]
    pub status: Option<LightsparkNodeStatus>,

    /// The sum of the balance on the Bitcoin Network, channel balances, and commit fees on this node.
    #[serde(rename = "lightspark_node_with_o_s_k_total_balance")]
    pub total_balance: Option<CurrencyAmount>,

    /// The total sum of the channel balances (online and offline) on this node.
    #[serde(rename = "lightspark_node_with_o_s_k_total_local_balance")]
    pub total_local_balance: Option<CurrencyAmount>,

    /// The sum of the channel balances (online only) that are available to send on this node.
    #[serde(rename = "lightspark_node_with_o_s_k_local_balance")]
    pub local_balance: Option<CurrencyAmount>,

    /// The sum of the channel balances that are available to receive on this node.
    #[serde(rename = "lightspark_node_with_o_s_k_remote_balance")]
    pub remote_balance: Option<CurrencyAmount>,

    /// The details of the balance of this node on the Bitcoin Network.
    #[serde(rename = "lightspark_node_with_o_s_k_blockchain_balance")]
    pub blockchain_balance: Option<BlockchainBalance>,

    /// The utxos of the channels that are connected to this node. This is used in uma flow for pre-screening.
    #[serde(rename = "lightspark_node_with_o_s_k_uma_prescreening_utxos")]
    pub uma_prescreening_utxos: Vec<String>,

    /// The balances that describe the funds in this node.
    #[serde(rename = "lightspark_node_with_o_s_k_balances")]
    pub balances: Option<Balances>,

    /// The private key client is using to sign a GraphQL request which will be verified at server side.
    #[serde(rename = "lightspark_node_with_o_s_k_encrypted_signing_private_key")]
    pub encrypted_signing_private_key: Option<Secret>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,
}

impl LightsparkNode for LightsparkNodeWithOSK {
    /// The owner of this LightsparkNode.
    fn get_owner_id(&self) -> EntityWrapper {
        self.owner.clone()
    }

    /// The current status of this node.
    fn get_status(&self) -> Option<LightsparkNodeStatus> {
        self.status.clone()
    }

    /// The sum of the balance on the Bitcoin Network, channel balances, and commit fees on this node.
    fn get_total_balance(&self) -> Option<CurrencyAmount> {
        self.total_balance.clone()
    }

    /// The total sum of the channel balances (online and offline) on this node.
    fn get_total_local_balance(&self) -> Option<CurrencyAmount> {
        self.total_local_balance.clone()
    }

    /// The sum of the channel balances (online only) that are available to send on this node.
    fn get_local_balance(&self) -> Option<CurrencyAmount> {
        self.local_balance.clone()
    }

    /// The sum of the channel balances that are available to receive on this node.
    fn get_remote_balance(&self) -> Option<CurrencyAmount> {
        self.remote_balance.clone()
    }

    /// The details of the balance of this node on the Bitcoin Network.
    fn get_blockchain_balance(&self) -> Option<BlockchainBalance> {
        self.blockchain_balance.clone()
    }

    /// The utxos of the channels that are connected to this node. This is used in uma flow for pre-screening.
    fn get_uma_prescreening_utxos(&self) -> Vec<String> {
        self.uma_prescreening_utxos.clone()
    }

    /// The balances that describe the funds in this node.
    fn get_balances(&self) -> Option<Balances> {
        self.balances.clone()
    }

    fn type_name(&self) -> &'static str {
        "LightsparkNodeWithOSK"
    }
}

impl Node for LightsparkNodeWithOSK {
    /// A name that identifies the node. It has no importance in terms of operating the node, it is just a way to identify and search for commercial services or popular nodes. This alias can be changed at any time by the node operator.
    fn get_alias(&self) -> Option<String> {
        self.alias.clone()
    }

    /// The Bitcoin Network this node is deployed in.
    fn get_bitcoin_network(&self) -> BitcoinNetwork {
        self.bitcoin_network.clone()
    }

    /// A hexadecimal string that describes a color. For example "#000000" is black, "#FFFFFF" is white. It has no importance in terms of operating the node, it is just a way to visually differentiate nodes. That color can be changed at any time by the node operator.
    fn get_color(&self) -> Option<String> {
        self.color.clone()
    }

    /// A summary metric used to capture how well positioned a node is to send, receive, or route transactions efficiently. Maximizing a node's conductivity helps a node’s transactions to be capital efficient. The value is an integer ranging between 0 and 10 (bounds included).
    fn get_conductivity(&self) -> Option<i64> {
        self.conductivity
    }

    /// The name of this node in the network. It will be the most human-readable option possible, depending on the data available for this node.
    fn get_display_name(&self) -> String {
        self.display_name.clone()
    }

    /// The public key of this node. It acts as a unique identifier of this node in the Lightning Network.
    fn get_public_key(&self) -> Option<String> {
        self.public_key.clone()
    }

    fn type_name(&self) -> &'static str {
        "LightsparkNodeWithOSK"
    }
}

impl Entity for LightsparkNodeWithOSK {
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
        "LightsparkNodeWithOSK"
    }
}

impl GetEntity for LightsparkNodeWithOSK {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on LightsparkNodeWithOSK {{
                    ... LightsparkNodeWithOSKFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment LightsparkNodeWithOSKFragment on LightsparkNodeWithOSK {
    __typename
    lightspark_node_with_o_s_k_id: id
    lightspark_node_with_o_s_k_created_at: created_at
    lightspark_node_with_o_s_k_updated_at: updated_at
    lightspark_node_with_o_s_k_alias: alias
    lightspark_node_with_o_s_k_bitcoin_network: bitcoin_network
    lightspark_node_with_o_s_k_color: color
    lightspark_node_with_o_s_k_conductivity: conductivity
    lightspark_node_with_o_s_k_display_name: display_name
    lightspark_node_with_o_s_k_public_key: public_key
    lightspark_node_with_o_s_k_owner: owner {
        id
    }
    lightspark_node_with_o_s_k_status: status
    lightspark_node_with_o_s_k_total_balance: total_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    lightspark_node_with_o_s_k_total_local_balance: total_local_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    lightspark_node_with_o_s_k_local_balance: local_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    lightspark_node_with_o_s_k_remote_balance: remote_balance {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    lightspark_node_with_o_s_k_blockchain_balance: blockchain_balance {
        __typename
        blockchain_balance_total_balance: total_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        blockchain_balance_confirmed_balance: confirmed_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        blockchain_balance_unconfirmed_balance: unconfirmed_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        blockchain_balance_locked_balance: locked_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        blockchain_balance_required_reserve: required_reserve {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        blockchain_balance_available_balance: available_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
    }
    lightspark_node_with_o_s_k_uma_prescreening_utxos: uma_prescreening_utxos
    lightspark_node_with_o_s_k_balances: balances {
        __typename
        balances_owned_balance: owned_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        balances_available_to_send_balance: available_to_send_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        balances_available_to_withdraw_balance: available_to_withdraw_balance {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
    }
    lightspark_node_with_o_s_k_encrypted_signing_private_key: encrypted_signing_private_key {
        __typename
        secret_encrypted_value: encrypted_value
        secret_cipher: cipher
    }
}
";

impl LightsparkNodeWithOSK {
    pub async fn get_addresses(
        &self,
        requester: &impl GraphQLRequester,
        first: Option<i64>,
        types: Option<Vec<NodeAddressType>>,
    ) -> Result<NodeToAddressesConnection, Error> {
        let query = "query FetchNodeToAddressesConnection($entity_id: ID!, $first: Int, $types: [NodeAddressType!]) {
    entity(id: $entity_id) {
        ... on LightsparkNodeWithOSK {
            addresses(, first: $first, types: $types) {
                __typename
                node_to_addresses_connection_count: count
                node_to_addresses_connection_entities: entities {
                    __typename
                    node_address_address: address
                    node_address_type: type
                }
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert("first", first.into());
        variables.insert("types", types.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let result = requester.execute_graphql(query, Some(value)).await?;
        let json = result["entity"]["addresses"].clone();
        let result = serde_json::from_value(json).map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn get_channels(
        &self,
        requester: &impl GraphQLRequester,
        first: Option<i64>,
        statuses: Option<Vec<ChannelStatus>>,
        after: Option<String>,
    ) -> Result<LightsparkNodeToChannelsConnection, Error> {
        let query = "query FetchLightsparkNodeToChannelsConnection($entity_id: ID!, $first: Int, $statuses: [ChannelStatus!], $after: String) {
    entity(id: $entity_id) {
        ... on LightsparkNodeWithOSK {
            channels(, first: $first, statuses: $statuses, after: $after) {
                __typename
                lightspark_node_to_channels_connection_count: count
                lightspark_node_to_channels_connection_page_info: page_info {
                    __typename
                    page_info_has_next_page: has_next_page
                    page_info_has_previous_page: has_previous_page
                    page_info_start_cursor: start_cursor
                    page_info_end_cursor: end_cursor
                }
                lightspark_node_to_channels_connection_entities: entities {
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
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert("first", first.into());
        variables.insert("statuses", statuses.into());
        variables.insert("after", after.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let result = requester.execute_graphql(query, Some(value)).await?;
        let json = result["entity"]["channels"].clone();
        let result = serde_json::from_value(json).map_err(Error::JsonError)?;
        Ok(result)
    }
}
