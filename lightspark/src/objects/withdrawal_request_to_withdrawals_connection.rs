// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::withdrawal::Withdrawal;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WithdrawalRequestToWithdrawalsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "withdrawal_request_to_withdrawals_connection_count")]
    pub count: i64,

    /// The withdrawals for the current page of this connection.
    #[serde(rename = "withdrawal_request_to_withdrawals_connection_entities")]
    pub entities: Vec<Withdrawal>,
}

pub const FRAGMENT: &str = "
fragment WithdrawalRequestToWithdrawalsConnectionFragment on WithdrawalRequestToWithdrawalsConnection {
    __typename
    withdrawal_request_to_withdrawals_connection_count: count
    withdrawal_request_to_withdrawals_connection_entities: entities {
        id
    }
}
";
