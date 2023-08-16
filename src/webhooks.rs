use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;

use crate::types::custom_date_formats::custom_date_format;
use crate::{error::Error, objects::webhook_event_type::WebhookEventType};
use chrono::{DateTime, Utc};

#[derive(Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_type: WebhookEventType,
    pub event_id: String,

    #[serde(with = "custom_date_format")]
    pub timestamp: DateTime<Utc>,
    pub entity_id: String,
    pub wallet_id: Option<String>,
    pub data: Option<Value>,
}

impl WebhookEvent {
    pub fn verify_and_parse(
        data: &[u8],
        hexdigest: String,
        webhood_secret: String,
    ) -> Result<WebhookEvent, Error> {
        let mut hmac: Hmac<Sha256> =
            Hmac::new_from_slice(webhood_secret.as_bytes()).expect("HMAC can take key of any size");
        hmac.update(data);
        let result = hmac.finalize();
        let code_bytes = result.into_bytes();
        let hex_string = hex::encode(code_bytes);
        if !hex_string
            .to_ascii_lowercase()
            .eq(&hexdigest.to_ascii_lowercase())
        {
            return Err(Error::WebhookSignatureError);
        }
        Self::parse(data)
    }

    fn parse(data: &[u8]) -> Result<WebhookEvent, Error> {
        let event: WebhookEvent = serde_json::from_slice(data).map_err(Error::JsonError)?;
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_verify_and_parse() {
        let data = "{\"event_type\": \"NODE_STATUS\", \"event_id\": \"1615c8be5aa44e429eba700db2ed8ca5\", \"timestamp\": \"2023-05-17T23:56:47.874449+00:00\", \"entity_id\": \"lightning_node:01882c25-157a-f96b-0000-362d42b64397\"}";
        let hexdigest = "62a8829aeb48b4142533520b1f7f86cdb1ee7d718bf3ea15bc1c662d4c453b74";
        let webhook_secret = "3gZ5oQQUASYmqQNuEk0KambNMVkOADDItIJjzUlAWjX";

        let result = super::WebhookEvent::verify_and_parse(
            data.as_bytes(),
            hexdigest.to_string(),
            webhook_secret.to_string(),
        )
        .expect("Success case");

        assert_eq!(
            result.event_type.to_string(),
            super::WebhookEventType::NodeStatus.to_string()
        );
        assert_eq!(result.event_id, "1615c8be5aa44e429eba700db2ed8ca5");
        assert_eq!(
            result.entity_id,
            "lightning_node:01882c25-157a-f96b-0000-362d42b64397"
        );
        assert_eq!(
            result.timestamp.to_string(),
            "2023-05-17 23:56:47.874449 UTC"
        );
    }
}
