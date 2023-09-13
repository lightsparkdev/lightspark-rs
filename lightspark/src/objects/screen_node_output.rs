// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::risk_rating::RiskRating;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ScreenNodeOutput {
    #[serde(rename = "screen_node_output_rating")]
    pub rating: RiskRating,
}

pub const FRAGMENT: &str = "
fragment ScreenNodeOutputFragment on ScreenNodeOutput {
    __typename
    screen_node_output_rating: rating
}
";
