// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTestModeInvoiceOutput {
    #[serde(rename = "create_test_mode_invoice_output_encoded_payment_request")]
    pub encoded_payment_request: String,
}

pub const FRAGMENT: &str = "
fragment CreateTestModeInvoiceOutputFragment on CreateTestModeInvoiceOutput {
    __typename
    create_test_mode_invoice_output_encoded_payment_request: encoded_payment_request
}
";
