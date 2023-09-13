// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct LightningFeeEstimateForNodeInput {
    /// The node from where you want to send the payment.
    pub node_id: String,

    /// The public key of the node that you want to pay.
    pub destination_node_public_key: String,

    /// The payment amount expressed in msats.
    pub amount_msats: i64,
}
