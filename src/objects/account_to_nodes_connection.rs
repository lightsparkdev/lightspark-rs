// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::lightspark_node::LightsparkNode;
use crate::objects::lightspark_node_purpose::LightsparkNodePurpose;
use crate::objects::page_info::PageInfo;
use serde::Deserialize;
use std::vec::Vec;

/// A connection between an account and the nodes it manages.
#[derive(Deserialize)]
pub struct AccountToNodesConnection {
    /// An object that holds pagination information about the objects in this connection.
    #[serde(rename = "account_to_nodes_connection_page_info")]
    pub page_info: PageInfo,

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "account_to_nodes_connection_count")]
    pub count: i64,

    /// The main purpose for the selected set of nodes. It is automatically determined from the nodes that are selected in this connection and is used for optimization purposes, as well as to determine the variation of the UI that should be presented to the user.
    #[serde(rename = "account_to_nodes_connection_purpose")]
    pub purpose: Option<LightsparkNodePurpose>,

    /// The nodes for the current page of this connection.
    #[serde(rename = "account_to_nodes_connection_entities")]
    pub entities: Vec<LightsparkNode>,
}

pub const FRAGMENT: &str = "
fragment AccountToNodesConnectionFragment on AccountToNodesConnection {
    __typename
    account_to_nodes_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
    account_to_nodes_connection_count: count
    account_to_nodes_connection_purpose: purpose
    account_to_nodes_connection_entities: entities {
        id
    }
}
";
