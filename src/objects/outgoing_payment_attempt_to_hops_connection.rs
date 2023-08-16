// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::hop::Hop;
use serde::Deserialize;
use std::vec::Vec;

/// The connection from an outgoing payment attempt to the list of sequential hops that define the path from sender node to recipient node.
#[derive(Clone, Deserialize)]
pub struct OutgoingPaymentAttemptToHopsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "outgoing_payment_attempt_to_hops_connection_count")]
    pub count: i64,

    /// The hops for the current page of this connection.
    #[serde(rename = "outgoing_payment_attempt_to_hops_connection_entities")]
    pub entities: Vec<Hop>,
}

pub const FRAGMENT: &str = "
fragment OutgoingPaymentAttemptToHopsConnectionFragment on OutgoingPaymentAttemptToHopsConnection {
    __typename
    outgoing_payment_attempt_to_hops_connection_count: count
    outgoing_payment_attempt_to_hops_connection_entities: entities {
        id
    }
}
";
