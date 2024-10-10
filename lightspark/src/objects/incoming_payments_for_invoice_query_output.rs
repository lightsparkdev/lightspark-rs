
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::objects::incoming_payment::IncomingPayment;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingPaymentsForInvoiceQueryOutput {

    
    #[serde (rename = "incoming_payments_for_invoice_query_output_payments")]
    pub payments: Vec<IncomingPayment>,

}



pub const FRAGMENT: &str = "
fragment IncomingPaymentsForInvoiceQueryOutputFragment on IncomingPaymentsForInvoiceQueryOutput {
    __typename
    incoming_payments_for_invoice_query_output_payments: payments {
        __typename
        incoming_payment_id: id
        incoming_payment_created_at: created_at
        incoming_payment_updated_at: updated_at
        incoming_payment_status: status
        incoming_payment_resolved_at: resolved_at
        incoming_payment_amount: amount {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        incoming_payment_transaction_hash: transaction_hash
        incoming_payment_is_uma: is_uma
        incoming_payment_destination: destination {
            id
        }
        incoming_payment_payment_request: payment_request {
            id
        }
        incoming_payment_uma_post_transaction_data: uma_post_transaction_data {
            __typename
            post_transaction_data_utxo: utxo
            post_transaction_data_amount: amount {
                __typename
                currency_amount_original_value: original_value
                currency_amount_original_unit: original_unit
                currency_amount_preferred_currency_unit: preferred_currency_unit
                currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
            }
        }
        incoming_payment_is_internal_payment: is_internal_payment
    }
}
";



