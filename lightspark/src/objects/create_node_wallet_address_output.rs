
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::multi_sig_address_validation_parameters::MultiSigAddressValidationParameters;
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateNodeWalletAddressOutput {

    
    #[serde(rename = "create_node_wallet_address_output_node")]
    pub node: EntityWrapper,

    
    #[serde (rename = "create_node_wallet_address_output_wallet_address")]
    pub wallet_address: String,

    /// Vaildation parameters for the 2-of-2 multisig address. None if the address is not a 2-of-2 multisig address.
    #[serde (rename = "create_node_wallet_address_output_multisig_wallet_address_validation_parameters")]
    pub multisig_wallet_address_validation_parameters: Option<MultiSigAddressValidationParameters>,

}



pub const FRAGMENT: &str = "
fragment CreateNodeWalletAddressOutputFragment on CreateNodeWalletAddressOutput {
    __typename
    create_node_wallet_address_output_node: node {
        id
    }
    create_node_wallet_address_output_wallet_address: wallet_address
    create_node_wallet_address_output_multisig_wallet_address_validation_parameters: multisig_wallet_address_validation_parameters {
        __typename
        multi_sig_address_validation_parameters_counterparty_funding_pubkey: counterparty_funding_pubkey
        multi_sig_address_validation_parameters_funding_pubkey_derivation_path: funding_pubkey_derivation_path
    }
}
";



