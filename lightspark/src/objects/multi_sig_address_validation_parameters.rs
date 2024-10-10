
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MultiSigAddressValidationParameters {

    /// The counterparty funding public key used to create the 2-of-2 multisig for the address.
    #[serde (rename = "multi_sig_address_validation_parameters_counterparty_funding_pubkey")]
    pub counterparty_funding_pubkey: String,

    /// The derivation path used to derive the funding public key for the 2-of-2 multisig address.
    #[serde (rename = "multi_sig_address_validation_parameters_funding_pubkey_derivation_path")]
    pub funding_pubkey_derivation_path: String,

}



pub const FRAGMENT: &str = "
fragment MultiSigAddressValidationParametersFragment on MultiSigAddressValidationParameters {
    __typename
    multi_sig_address_validation_parameters_counterparty_funding_pubkey: counterparty_funding_pubkey
    multi_sig_address_validation_parameters_funding_pubkey_derivation_path: funding_pubkey_derivation_path
}
";



