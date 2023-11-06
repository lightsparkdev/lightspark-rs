// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// This represents the fee policies set for a channel on the Lightning Network.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelFees {
    #[serde(rename = "channel_fees_base_fee")]
    pub base_fee: Option<CurrencyAmount>,

    #[serde(rename = "channel_fees_fee_rate_per_mil")]
    pub fee_rate_per_mil: Option<i64>,
}

pub const FRAGMENT: &str = "
fragment ChannelFeesFragment on ChannelFees {
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
";
