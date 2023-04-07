// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::outgoing_payment_attempt::OutgoingPaymentAttempt;
use serde::Deserialize;
use std::vec::Vec;

/// The connection from outgoing payment to all attempts.
#[derive(Deserialize)]
pub struct OutgoingPaymentToAttemptsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "outgoing_payment_to_attempts_connection_count")]
    pub count: i64,

    /// The attempts for the current page of this connection.
    #[serde(rename = "outgoing_payment_to_attempts_connection_entities")]
    pub entities: Vec<OutgoingPaymentAttempt>,
}

pub const FRAGMENT: &str = "
fragment OutgoingPaymentToAttemptsConnectionFragment on OutgoingPaymentToAttemptsConnection {
    __typename
    outgoing_payment_to_attempts_connection_count: count
    outgoing_payment_to_attempts_connection_entities: entities {
        id
    }
}
";
