// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateOfferOutput {
    #[serde(rename = "create_offer_output_offer")]
    pub offer: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment CreateOfferOutputFragment on CreateOfferOutput {
    __typename
    create_offer_output_offer: offer {
        id
    }
}
";
