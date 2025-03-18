// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayOfferInput {
    /// The ID of the node that will be sending the payment.
    pub node_id: String,

    /// The Bech32 offer you want to pay (as defined by the BOLT12 standard).
    pub encoded_offer: String,

    /// The timeout in seconds that we will try to make the payment.
    pub timeout_secs: i64,

    /// The maximum amount of fees that you want to pay for this payment to be sent, expressed in msats.
    pub maximum_fees_msats: i64,

    /// The amount you will pay for this offer, expressed in msats. It should ONLY be set when the offer amount is zero.
    pub amount_msats: Option<i64>,

    /// An idempotency key for this payment. If provided, it will be used to create a payment with the same idempotency key. If not provided, a new idempotency key will be generated.
    pub idempotency_key: Option<String>,
}
