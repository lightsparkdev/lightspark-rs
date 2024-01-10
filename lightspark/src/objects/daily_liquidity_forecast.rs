
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::custom_date_formats::custom_date_only_format;
use crate::objects::lightning_payment_direction::LightningPaymentDirection;
use chrono::NaiveDate;
use crate::objects::currency_amount::CurrencyAmount;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DailyLiquidityForecast {

    /// The date for which this forecast was generated.
    #[serde(with = "custom_date_only_format", rename = "daily_liquidity_forecast_date")]
    pub date: NaiveDate,

    /// The direction for which this forecast was generated.
    #[serde (rename = "daily_liquidity_forecast_direction")]
    pub direction: LightningPaymentDirection,

    /// The value of the forecast. It represents the amount of msats that we think will be moved for that specified direction, for that node, on that date.
    #[serde (rename = "daily_liquidity_forecast_amount")]
    pub amount: CurrencyAmount,

}



pub const FRAGMENT: &str = "
fragment DailyLiquidityForecastFragment on DailyLiquidityForecast {
    __typename
    daily_liquidity_forecast_date: date
    daily_liquidity_forecast_direction: direction
    daily_liquidity_forecast_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
}
";



