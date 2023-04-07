// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_unit::CurrencyUnit;
use serde::Deserialize;

/// Represents the value and unit for an amount of currency.
#[derive(Clone, Deserialize)]
pub struct CurrencyAmount {
    /// The original numeric value for this CurrencyAmount.
    #[serde(rename = "currency_amount_original_value")]
    pub original_value: i64,

    /// The original unit of currency for this CurrencyAmount.
    #[serde(rename = "currency_amount_original_unit")]
    pub original_unit: CurrencyUnit,

    /// The unit of user's preferred currency.
    #[serde(rename = "currency_amount_preferred_currency_unit")]
    pub preferred_currency_unit: CurrencyUnit,

    /// The rounded numeric value for this CurrencyAmount in the very base level of user's preferred currency. For example, for USD, the value will be in cents.
    #[serde(rename = "currency_amount_preferred_currency_value_rounded")]
    pub preferred_currency_value_rounded: i64,

    /// The approximate float value for this CurrencyAmount in the very base level of user's preferred currency. For example, for USD, the value will be in cents.
    #[serde(rename = "currency_amount_preferred_currency_value_approx")]
    pub preferred_currency_value_approx: f64,
}

pub const FRAGMENT: &str = "
fragment CurrencyAmountFragment on CurrencyAmount {
    __typename
    currency_amount_original_value: original_value
    currency_amount_original_unit: original_unit
    currency_amount_preferred_currency_unit: preferred_currency_unit
    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
}
";
