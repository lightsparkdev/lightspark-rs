
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::entity::Entity;
use crate::objects::incentives_ineligibility_reason::IncentivesIneligibilityReason;
use chrono::{DateTime, Utc};
use crate::types::get_entity::GetEntity;
use crate::types::custom_date_formats::custom_date_format;
use crate::objects::incentives_status::IncentivesStatus;

/// This is an object representing an UMA.ME invitation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UmaInvitation {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "uma_invitation_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "uma_invitation_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "uma_invitation_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The code that uniquely identifies this invitation.
    #[serde (rename = "uma_invitation_code")]
    pub code: String,

    /// The URL where this invitation can be claimed.
    #[serde (rename = "uma_invitation_url")]
    pub url: String,

    /// The UMA of the user who created the invitation.
    #[serde (rename = "uma_invitation_inviter_uma")]
    pub inviter_uma: String,

    /// The UMA of the user who claimed the invitation.
    #[serde (rename = "uma_invitation_invitee_uma")]
    pub invitee_uma: Option<String>,

    /// The current status of the incentives that may be tied to this invitation.
    #[serde (rename = "uma_invitation_incentives_status")]
    pub incentives_status: IncentivesStatus,

    /// The reason why the invitation is not eligible for incentives, if applicable.
    #[serde (rename = "uma_invitation_incentives_ineligibility_reason")]
    pub incentives_ineligibility_reason: Option<IncentivesIneligibilityReason>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Entity for UmaInvitation {

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
        "UmaInvitation"
    }
}


impl GetEntity for UmaInvitation {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on UmaInvitation {{
                    ... UmaInvitationFragment
                }}
            }}
        }}

        {}", FRAGMENT)
    }    
}



pub const FRAGMENT: &str = "
fragment UmaInvitationFragment on UmaInvitation {
    __typename
    uma_invitation_id: id
    uma_invitation_created_at: created_at
    uma_invitation_updated_at: updated_at
    uma_invitation_code: code
    uma_invitation_url: url
    uma_invitation_inviter_uma: inviter_uma
    uma_invitation_invitee_uma: invitee_uma
    uma_invitation_incentives_status: incentives_status
    uma_invitation_incentives_ineligibility_reason: incentives_ineligibility_reason
}
";



