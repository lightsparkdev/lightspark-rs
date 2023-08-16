// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use serde::Deserialize;

/// This is an object representing a detailed breakdown of the balance for a Lightspark Node.
#[derive(Clone, Deserialize)]
pub struct BlockchainBalance {
    /// The total wallet balance, including unconfirmed UTXOs.
    #[serde(rename = "blockchain_balance_total_balance")]
    pub total_balance: Option<CurrencyAmount>,

    /// The balance of confirmed UTXOs in the wallet.
    #[serde(rename = "blockchain_balance_confirmed_balance")]
    pub confirmed_balance: Option<CurrencyAmount>,

    /// The balance of unconfirmed UTXOs in the wallet.
    #[serde(rename = "blockchain_balance_unconfirmed_balance")]
    pub unconfirmed_balance: Option<CurrencyAmount>,

    /// The balance that's locked by an on-chain transaction.
    #[serde(rename = "blockchain_balance_locked_balance")]
    pub locked_balance: Option<CurrencyAmount>,

    /// Funds required to be held in reserve for channel bumping.
    #[serde(rename = "blockchain_balance_required_reserve")]
    pub required_reserve: Option<CurrencyAmount>,

    /// Funds available for creating channels or withdrawing.
    #[serde(rename = "blockchain_balance_available_balance")]
    pub available_balance: Option<CurrencyAmount>,
}

pub const FRAGMENT: &str = "
fragment BlockchainBalanceFragment on BlockchainBalance {
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
";
