
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::withdrawal_mode::WithdrawalMode;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WithdrawalFeeEstimateInput {

    /// The node from which you'd like to make the withdrawal.
    
    pub node_id: String,

    /// The amount you want to withdraw from this node in Satoshis. Use the special value -1 to withdrawal all funds from this node.
    
    pub amount_sats: i64,

    /// The strategy that should be used to withdraw the funds from this node.
    
    pub withdrawal_mode: WithdrawalMode,

}





