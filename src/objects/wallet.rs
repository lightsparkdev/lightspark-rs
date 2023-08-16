// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::error::Error;
use crate::objects::balances::Balances;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::lightspark_node_owner::LightsparkNodeOwner;
use crate::objects::wallet_status::WalletStatus;
use crate::request::requester::Requester;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// This object represents a Lightspark Wallet, tied to your Lightspark account. Wallets can be used to send or receive funds over the Lightning Network. You can retrieve this object to receive information about a specific wallet tied to your Lightspark account.
#[derive(Clone, Deserialize)]
pub struct Wallet {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "wallet_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "wallet_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "wallet_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The date and time when the wallet user last logged in.
    #[serde(with = "custom_date_format_option", rename = "wallet_last_login_at")]
    pub last_login_at: Option<DateTime<Utc>>,

    /// The balances that describe the funds in this wallet.
    #[serde(rename = "wallet_balances")]
    pub balances: Option<Balances>,

    /// The unique identifier of this wallet, as provided by the Lightspark Customer during login.
    #[serde(rename = "wallet_third_party_identifier")]
    pub third_party_identifier: String,

    /// The status of this wallet.
    #[serde(rename = "wallet_status")]
    pub status: WalletStatus,
}

impl LightsparkNodeOwner for Wallet {
    fn type_name(&self) -> &'static str {
        "Wallet"
    }
}

impl Entity for Wallet {
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
        "Wallet"
    }
}

impl GetEntity for Wallet {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Wallet {{
                    ... WalletFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment WalletFragment on Wallet {
    __typename
    wallet_id: id
    wallet_created_at: created_at
    wallet_updated_at: updated_at
    wallet_last_login_at: last_login_at
    wallet_balances: balances {
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
    wallet_third_party_identifier: third_party_identifier
    wallet_status: status
}
";

impl Wallet {
    pub async fn get_total_amount_received(
        &self,
        requester: &Requester,
        created_after_date: Option<DateTime<Utc>>,
        created_before_date: Option<DateTime<Utc>>,
    ) -> Result<CurrencyAmount, Error> {
        let query = "query FetchWalletTotalAmountReceived($entity_id: ID!, $created_after_date: DateTime, $created_before_date: DateTime) {
    entity(id: $entity_id) {
        ... on Wallet {
            total_amount_received(, created_after_date: $created_after_date, created_before_date: $created_before_date) {
                __typename
                currency_amount_original_value: original_value
                currency_amount_original_unit: original_unit
                currency_amount_preferred_currency_unit: preferred_currency_unit
                currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert(
            "created_after_date",
            created_after_date.map(|dt| dt.to_rfc3339()).into(),
        );
        variables.insert(
            "created_before_date",
            created_before_date.map(|dt| dt.to_rfc3339()).into(),
        );

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let result = requester
            .execute_graphql(query, Some(value))
            .await
            .map_err(Error::ClientError)?;
        let json = result["entity"]["total_amount_received"].clone();
        let result = serde_json::from_value(json).map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn get_total_amount_sent(
        &self,
        requester: &Requester,
        created_after_date: Option<DateTime<Utc>>,
        created_before_date: Option<DateTime<Utc>>,
    ) -> Result<CurrencyAmount, Error> {
        let query = "query FetchWalletTotalAmountSent($entity_id: ID!, $created_after_date: DateTime, $created_before_date: DateTime) {
    entity(id: $entity_id) {
        ... on Wallet {
            total_amount_sent(, created_after_date: $created_after_date, created_before_date: $created_before_date) {
                __typename
                currency_amount_original_value: original_value
                currency_amount_original_unit: original_unit
                currency_amount_preferred_currency_unit: preferred_currency_unit
                currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert(
            "created_after_date",
            created_after_date.map(|dt| dt.to_rfc3339()).into(),
        );
        variables.insert(
            "created_before_date",
            created_before_date.map(|dt| dt.to_rfc3339()).into(),
        );

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let result = requester
            .execute_graphql(query, Some(value))
            .await
            .map_err(Error::ClientError)?;
        let json = result["entity"]["total_amount_sent"].clone();
        let result = serde_json::from_value(json).map_err(Error::JsonError)?;
        Ok(result)
    }
}
