// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::crypto_sanctions_screening_provider::CryptoSanctionsScreeningProvider;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Clone, Deserialize, Serialize)]
pub struct ScreenBitcoinAddressesInput {
    pub provider: CryptoSanctionsScreeningProvider,

    pub addresses: Vec<String>,
}
