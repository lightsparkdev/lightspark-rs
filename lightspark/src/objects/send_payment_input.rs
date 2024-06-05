// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendPaymentInput {
    /// The node from where you want to send the payment.
    pub node_id: String,

    /// The public key of the destination node.
    pub destination_public_key: String,

    /// The timeout in seconds that we will try to make the payment.
    pub timeout_secs: i64,

    /// The amount you will send to the destination node, expressed in msats.
    pub amount_msats: i64,

    /// The maximum amount of fees that you want to pay for this payment to be sent, expressed in msats.
    pub maximum_fees_msats: i64,

    /// The idempotency key of the request. The same result will be returned for the same idempotency key.
    pub idempotency_key: Option<String>,
}
