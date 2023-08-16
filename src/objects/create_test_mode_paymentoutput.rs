// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

/// This is an object identifying the output of a test mode payment. This object can be used to retrieve the associated payment made from a Test Mode Payment call.
#[derive(Clone, Deserialize)]
pub struct CreateTestModePaymentoutput {
    /// The payment that has been sent.
    #[serde(rename = "create_test_mode_paymentoutput_payment")]
    pub payment: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment CreateTestModePaymentoutputFragment on CreateTestModePaymentoutput {
    __typename
    create_test_mode_paymentoutput_payment: payment {
        id
    }
}
";
