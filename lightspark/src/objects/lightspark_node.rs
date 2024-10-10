// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use super::lightspark_node_with_o_s_k::LightsparkNodeWithOSK;
use super::lightspark_node_with_remote_signing::LightsparkNodeWithRemoteSigning;
use crate::objects::balances::Balances;
use crate::objects::blockchain_balance::BlockchainBalance;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::lightspark_node_status::LightsparkNodeStatus;
use crate::objects::node::Node;
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::vec::Vec;

pub trait LightsparkNode: Node + Entity {
    /// The owner of this LightsparkNode.
    fn get_owner_id(&self) -> EntityWrapper;

    /// The current status of this node.
    fn get_status(&self) -> Option<LightsparkNodeStatus>;

    /// The sum of the balance on the Bitcoin Network, channel balances, and commit fees on this node.
    fn get_total_balance(&self) -> Option<CurrencyAmount>;

    /// The total sum of the channel balances (online and offline) on this node.
    fn get_total_local_balance(&self) -> Option<CurrencyAmount>;

    /// The sum of the channel balances (online only) that are available to send on this node.
    fn get_local_balance(&self) -> Option<CurrencyAmount>;

    /// The sum of the channel balances that are available to receive on this node.
    fn get_remote_balance(&self) -> Option<CurrencyAmount>;

    /// The details of the balance of this node on the Bitcoin Network.
    fn get_blockchain_balance(&self) -> Option<BlockchainBalance>;

    /// The utxos of the channels that are connected to this node. This is used in uma flow for pre-screening.
    fn get_uma_prescreening_utxos(&self) -> Vec<String>;

    /// The balances that describe the funds in this node.
    fn get_balances(&self) -> Option<Balances>;

    fn type_name(&self) -> &'static str;
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum LightsparkNodeEnum {
    LightsparkNodeWithOSK(LightsparkNodeWithOSK),
    LightsparkNodeWithRemoteSigning(LightsparkNodeWithRemoteSigning),
}

impl<'de> Deserialize<'de> for LightsparkNodeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "LightsparkNodeWithOSK" => {
                    let obj = LightsparkNodeWithOSK::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(LightsparkNodeEnum::LightsparkNodeWithOSK(obj))
                }
                "LightsparkNodeWithRemoteSigning" => {
                    let obj =
                        LightsparkNodeWithRemoteSigning::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(LightsparkNodeEnum::LightsparkNodeWithRemoteSigning(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on LightsparkNode",
            ))
        }
    }
}
