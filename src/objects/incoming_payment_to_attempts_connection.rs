// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::incoming_payment_attempt::IncomingPaymentAttempt;
use serde::Deserialize;
use std::vec::Vec;

/// The connection from incoming payment to all attempts.
#[derive(Deserialize)]
pub struct IncomingPaymentToAttemptsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "incoming_payment_to_attempts_connection_count")]
    pub count: i64,

    /// The incoming payment attempts for the current page of this connection.
    #[serde(rename = "incoming_payment_to_attempts_connection_entities")]
    pub entities: Vec<IncomingPaymentAttempt>,
}

pub const FRAGMENT: &str = "
fragment IncomingPaymentToAttemptsConnectionFragment on IncomingPaymentToAttemptsConnection {
    __typename
    incoming_payment_to_attempts_connection_count: count
    incoming_payment_to_attempts_connection_entities: entities {
        id
    }
}
";
