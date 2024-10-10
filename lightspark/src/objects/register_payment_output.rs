
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterPaymentOutput {

    
    #[serde(rename = "register_payment_output_payment")]
    pub payment: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment RegisterPaymentOutputFragment on RegisterPaymentOutput {
    __typename
    register_payment_output_payment: payment {
        id
    }
}
";



