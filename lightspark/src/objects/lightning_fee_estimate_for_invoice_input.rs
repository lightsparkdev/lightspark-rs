// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct LightningFeeEstimateForInvoiceInput {
    /// The node from where you want to send the payment.
    pub node_id: String,

    /// The invoice you want to pay (as defined by the BOLT11 standard).
    pub encoded_payment_request: String,

    /// If the invoice does not specify a payment amount, then the amount that you wish to pay, expressed in msats.
    pub amount_msats: Option<i64>,
}
