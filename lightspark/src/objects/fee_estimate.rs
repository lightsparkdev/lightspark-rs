// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// This object represents the estimated L1 transaction fees for the Bitcoin network. Fee estimates are separated by potential confirmation speeds for settlement.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeEstimate {
    #[serde(rename = "fee_estimate_fee_fast")]
    pub fee_fast: CurrencyAmount,

    #[serde(rename = "fee_estimate_fee_min")]
    pub fee_min: CurrencyAmount,
}

pub const FRAGMENT: &str = "
fragment FeeEstimateFragment on FeeEstimate {
    __typename
    fee_estimate_fee_fast: fee_fast {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    fee_estimate_fee_min: fee_min {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
}
";
