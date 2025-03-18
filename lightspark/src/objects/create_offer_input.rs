// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateOfferInput {
    /// The node from which to create the offer.
    pub node_id: String,

    /// The amount for which the offer should be created, in millisatoshis. Setting the amount to 0 will allow the payer to specify an amount.
    pub amount_msats: Option<i64>,

    /// A short description of the offer.
    pub description: Option<String>,
}
