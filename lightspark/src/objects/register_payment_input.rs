// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::compliance_provider::ComplianceProvider;
use crate::objects::payment_direction::PaymentDirection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterPaymentInput {
    /// The compliance provider that is going to screen the node. You need to be a customer of the selected provider and store the API key on the Lightspark account setting page.
    pub provider: ComplianceProvider,

    /// The Lightspark ID of the lightning payment you want to register. It can be the id of either an OutgoingPayment or an IncomingPayment.
    pub payment_id: String,

    /// The public key of the counterparty lightning node, which would be the public key of the recipient node if it is to register an outgoing payment, or the public key of the sender node if it is to register an incoming payment.
    pub node_pubkey: String,

    /// Indicates whether this payment is an OutgoingPayment or an IncomingPayment.
    pub direction: PaymentDirection,
}
