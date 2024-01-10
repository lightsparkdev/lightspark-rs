
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::htlc_attempt_failure_code::HtlcAttemptFailureCode;
use crate::types::custom_date_formats::custom_date_format;
use crate::objects::outgoing_payment_attempt_to_hops_connection::OutgoingPaymentAttemptToHopsConnection;
use crate::types::get_entity::GetEntity;
use serde_json::Value;
use std::collections::HashMap;
use crate::error::Error;
use chrono::{DateTime, Utc};
use crate::types::entity_wrapper::EntityWrapper;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::types::graphql_requester::GraphQLRequester;
use crate::objects::outgoing_payment_attempt_status::OutgoingPaymentAttemptStatus;
use crate::objects::entity::Entity;
use crate::objects::currency_amount::CurrencyAmount;

/// This object represents an attempted Lightning Network payment sent from a Lightspark Node. You can retrieve this object to receive payment related information about any payment attempt sent from your Lightspark Node on the Lightning Network, including any potential reasons the payment may have failed.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutgoingPaymentAttempt {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "outgoing_payment_attempt_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "outgoing_payment_attempt_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "outgoing_payment_attempt_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The status of an outgoing payment attempt.
    #[serde (rename = "outgoing_payment_attempt_status")]
    pub status: OutgoingPaymentAttemptStatus,

    /// If the payment attempt failed, then this contains the Bolt #4 failure code.
    #[serde (rename = "outgoing_payment_attempt_failure_code")]
    pub failure_code: Option<HtlcAttemptFailureCode>,

    /// If the payment attempt failed, then this contains the index of the hop at which the problem occurred.
    #[serde (rename = "outgoing_payment_attempt_failure_source_index")]
    pub failure_source_index: Option<i64>,

    /// The date and time when the attempt was initiated.
    #[serde(with = "custom_date_format", rename = "outgoing_payment_attempt_attempted_at")]
    pub attempted_at: DateTime<Utc>,

    /// The time the outgoing payment attempt failed or succeeded.
    #[serde(with = "custom_date_format_option", rename = "outgoing_payment_attempt_resolved_at")]
    pub resolved_at: Option<DateTime<Utc>>,

    /// The total amount of funds required to complete a payment over this route. This value includes the cumulative fees for each hop. As a result, the attempt extended to the first-hop in the route will need to have at least this much value, otherwise the route will fail at an intermediate node due to an insufficient amount.
    #[serde (rename = "outgoing_payment_attempt_amount")]
    pub amount: Option<CurrencyAmount>,

    /// The sum of the fees paid at each hop within the route of this attempt. In the case of a one-hop payment, this value will be zero as we don't need to pay a fee to ourselves.
    #[serde (rename = "outgoing_payment_attempt_fees")]
    pub fees: Option<CurrencyAmount>,

    /// The outgoing payment for this attempt.
    #[serde(rename = "outgoing_payment_attempt_outgoing_payment")]
    pub outgoing_payment: EntityWrapper,

    /// The channel snapshot at the time the outgoing payment attempt was made.
    #[serde(rename = "outgoing_payment_attempt_channel_snapshot")]
    pub channel_snapshot: Option<EntityWrapper>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Entity for OutgoingPaymentAttempt {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String {
        self.id.clone()
    }

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }


    fn type_name(&self) -> &'static str {
        "OutgoingPaymentAttempt"
    }
}


impl GetEntity for OutgoingPaymentAttempt {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on OutgoingPaymentAttempt {{
                    ... OutgoingPaymentAttemptFragment
                }}
            }}
        }}

        {}", FRAGMENT)
    }    
}



pub const FRAGMENT: &str = "
fragment OutgoingPaymentAttemptFragment on OutgoingPaymentAttempt {
    __typename
    outgoing_payment_attempt_id: id
    outgoing_payment_attempt_created_at: created_at
    outgoing_payment_attempt_updated_at: updated_at
    outgoing_payment_attempt_status: status
    outgoing_payment_attempt_failure_code: failure_code
    outgoing_payment_attempt_failure_source_index: failure_source_index
    outgoing_payment_attempt_attempted_at: attempted_at
    outgoing_payment_attempt_resolved_at: resolved_at
    outgoing_payment_attempt_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    outgoing_payment_attempt_fees: fees {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    outgoing_payment_attempt_outgoing_payment: outgoing_payment {
        id
    }
    outgoing_payment_attempt_channel_snapshot: channel_snapshot {
        id
    }
}
";


impl OutgoingPaymentAttempt {

    
    pub async fn get_hops(&self, requester:&impl GraphQLRequester, first: Option<i64>, after: Option<String>) -> Result<OutgoingPaymentAttemptToHopsConnection, Error> {
        let query = "query FetchOutgoingPaymentAttemptToHopsConnection($entity_id: ID!, $first: Int, $after: String) {
    entity(id: $entity_id) {
        ... on OutgoingPaymentAttempt {
            hops(, first: $first, after: $after) {
                __typename
                outgoing_payment_attempt_to_hops_connection_count: count
                outgoing_payment_attempt_to_hops_connection_page_info: page_info {
                    __typename
                    page_info_has_next_page: has_next_page
                    page_info_has_previous_page: has_previous_page
                    page_info_start_cursor: start_cursor
                    page_info_end_cursor: end_cursor
                }
                outgoing_payment_attempt_to_hops_connection_entities: entities {
                    __typename
                    hop_id: id
                    hop_created_at: created_at
                    hop_updated_at: updated_at
                    hop_destination: destination {
                        id
                    }
                    hop_index: index
                    hop_public_key: public_key
                    hop_amount_to_forward: amount_to_forward {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    hop_fee: fee {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    hop_expiry_block_height: expiry_block_height
                }
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert("first", first.into());
        variables.insert("after", after.into());

                
        let value = serde_json::to_value(variables).map_err(|err| Error::ConversionError(err))?;
        let result = requester.execute_graphql(&query, Some(value)).await?;
        let json = result["entity"]["hops"].clone();
        let result = serde_json::from_value(json).map_err(|err| Error::JsonError(err))?;
        Ok(result)
    }

}
