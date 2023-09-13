// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateTestModePaymentInput {
    /// The node to where you want to send the payment.
    pub local_node_id: String,

    /// The invoice you want to be paid (as defined by the BOLT11 standard).
    pub encoded_invoice: String,

    /// The amount you will be paid for this invoice, expressed in msats. It should ONLY be set when the invoice amount is zero.
    pub amount_msats: Option<i64>,
}
