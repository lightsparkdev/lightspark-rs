// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::compliance_provider::ComplianceProvider;
use crate::objects::payment_direction::PaymentDirection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct RegisterPaymentInput {
    pub provider: ComplianceProvider,

    pub payment_id: String,

    pub node_pubkey: String,

    pub direction: PaymentDirection,
}
