// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::risk_rating::RiskRating;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Clone, Deserialize)]
pub struct ScreenBitcoinAddressesOutput {
    #[serde(rename = "screen_bitcoin_addresses_output_ratings")]
    pub ratings: Vec<RiskRating>,
}

pub const FRAGMENT: &str = "
fragment ScreenBitcoinAddressesOutputFragment on ScreenBitcoinAddressesOutput {
    __typename
    screen_bitcoin_addresses_output_ratings: ratings
}
";
