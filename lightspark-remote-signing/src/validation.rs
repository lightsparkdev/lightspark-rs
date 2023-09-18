// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

pub trait Validation {
    /// This function should return true if the webhook should be signed.
    ///
    /// Arguments:
    /// * `webhook` - The webhook event json serialized string to be validated.
    fn should_sign(&self, webhook: &str) -> bool;
}

pub struct PositiveValidator;

impl Validation for PositiveValidator {
    fn should_sign(&self, _: &str) -> bool {
        true
    }
}
