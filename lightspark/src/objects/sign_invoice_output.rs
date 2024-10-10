
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignInvoiceOutput {

    ///  The signed invoice object.
    #[serde(rename = "sign_invoice_output_invoice")]
    pub invoice: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment SignInvoiceOutputFragment on SignInvoiceOutput {
    __typename
    sign_invoice_output_invoice: invoice {
        id
    }
}
";



