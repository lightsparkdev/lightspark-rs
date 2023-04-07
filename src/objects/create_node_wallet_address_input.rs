// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateNodeWalletAddressInput {
    pub node_id: String,
}
