
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::currency_amount::CurrencyAmount;
use crate::types::entity_wrapper::EntityWrapper;
use crate::objects::entity::Entity;
use crate::types::get_entity::GetEntity;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::objects::incoming_payment_attempt_status::IncomingPaymentAttemptStatus;
use crate::types::custom_date_formats::custom_date_format;
use chrono::{DateTime, Utc};

/// This object represents any attempted payment sent to a Lightspark node on the Lightning Network. You can retrieve this object to receive payment related information about a specific incoming payment attempt.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingPaymentAttempt {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "incoming_payment_attempt_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "incoming_payment_attempt_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "incoming_payment_attempt_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The status of the incoming payment attempt.
    #[serde (rename = "incoming_payment_attempt_status")]
    pub status: IncomingPaymentAttemptStatus,

    /// The time the incoming payment attempt failed or succeeded.
    #[serde(with = "custom_date_format_option", rename = "incoming_payment_attempt_resolved_at")]
    pub resolved_at: Option<DateTime<Utc>>,

    /// The total amount of that was attempted to send.
    #[serde (rename = "incoming_payment_attempt_amount")]
    pub amount: CurrencyAmount,

    /// The channel this attempt was made on.
    #[serde(rename = "incoming_payment_attempt_channel")]
    pub channel: EntityWrapper,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Entity for IncomingPaymentAttempt {

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
        "IncomingPaymentAttempt"
    }
}


impl GetEntity for IncomingPaymentAttempt {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on IncomingPaymentAttempt {{
                    ... IncomingPaymentAttemptFragment
                }}
            }}
        }}

        {}", FRAGMENT)
    }    
}



pub const FRAGMENT: &str = "
fragment IncomingPaymentAttemptFragment on IncomingPaymentAttempt {
    __typename
    incoming_payment_attempt_id: id
    incoming_payment_attempt_created_at: created_at
    incoming_payment_attempt_updated_at: updated_at
    incoming_payment_attempt_status: status
    incoming_payment_attempt_resolved_at: resolved_at
    incoming_payment_attempt_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    incoming_payment_attempt_channel: channel {
        id
    }
}
";



