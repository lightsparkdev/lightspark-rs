
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::vec::Vec;
use crate::objects::lightning_payment_direction::LightningPaymentDirection;
use crate::types::custom_date_formats::custom_date_only_format;
use crate::objects::daily_liquidity_forecast::DailyLiquidityForecast;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LightsparkNodeToDailyLiquidityForecastsConnection {

    
    #[serde(with = "custom_date_only_format", rename = "lightspark_node_to_daily_liquidity_forecasts_connection_from_date")]
    pub from_date: NaiveDate,

    
    #[serde(with = "custom_date_only_format", rename = "lightspark_node_to_daily_liquidity_forecasts_connection_to_date")]
    pub to_date: NaiveDate,

    
    #[serde (rename = "lightspark_node_to_daily_liquidity_forecasts_connection_direction")]
    pub direction: LightningPaymentDirection,

    /// The daily liquidity forecasts for the current page of this connection.
    #[serde (rename = "lightspark_node_to_daily_liquidity_forecasts_connection_entities")]
    pub entities: Vec<DailyLiquidityForecast>,

}



pub const FRAGMENT: &str = "
fragment LightsparkNodeToDailyLiquidityForecastsConnectionFragment on LightsparkNodeToDailyLiquidityForecastsConnection {
    __typename
    lightspark_node_to_daily_liquidity_forecasts_connection_from_date: from_date
    lightspark_node_to_daily_liquidity_forecasts_connection_to_date: to_date
    lightspark_node_to_daily_liquidity_forecasts_connection_direction: direction
    lightspark_node_to_daily_liquidity_forecasts_connection_entities: entities {
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
}
";



