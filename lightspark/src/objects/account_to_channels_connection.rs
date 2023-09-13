// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::channel::Channel;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Clone, Deserialize)]
pub struct AccountToChannelsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "account_to_channels_connection_count")]
    pub count: i64,

    /// The channels for the current page of this connection.
    #[serde(rename = "account_to_channels_connection_entities")]
    pub entities: Vec<Channel>,
}

pub const FRAGMENT: &str = "
fragment AccountToChannelsConnectionFragment on AccountToChannelsConnection {
    __typename
    account_to_channels_connection_count: count
    account_to_channels_connection_entities: entities {
        id
    }
}
";
