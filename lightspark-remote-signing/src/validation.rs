// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use lightspark::webhooks::WebhookEvent;

pub trait Validation {
    fn should_sign(&self, webhook: &WebhookEvent) -> bool;
}

pub struct PositiveValidator;

impl Validation for PositiveValidator {
    fn should_sign(&self, _: &WebhookEvent) -> bool {
        true
    }
}
