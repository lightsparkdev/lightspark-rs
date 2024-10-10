
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::on_chain_fee_target::OnChainFeeTarget;
use crate::objects::withdrawal_mode::WithdrawalMode;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestWithdrawalInput {

    /// The node from which you'd like to make the withdrawal.
    
    pub node_id: String,

    /// The bitcoin address where the withdrawal should be sent.
    
    pub bitcoin_address: String,

    /// The amount you want to withdraw from this node in Satoshis. Use the special value -1 to withdrawal all funds from this node.
    
    pub amount_sats: i64,

    /// The strategy that should be used to withdraw the funds from this node.
    
    pub withdrawal_mode: WithdrawalMode,

    /// The idempotency key of the request. The same result will be returned for the same idempotency key.
    
    pub idempotency_key: Option<String>,

    /// The target of the fee that should be used when crafting the L1 transaction. You should only set `fee_target` or `sats_per_vbyte`. If neither of them is set, default value of MEDIUM will be used as `fee_target`.
    
    pub fee_target: Option<OnChainFeeTarget>,

    /// A manual fee rate set in sat/vbyte that should be used when crafting the L1 transaction. You should only set `fee_target` or `sats_per_vbyte`
    
    pub sats_per_vbyte: Option<i64>,

}





