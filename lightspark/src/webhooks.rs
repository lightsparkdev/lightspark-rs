use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;

use crate::types::custom_date_formats::custom_date_format;
use crate::{error::Error, objects::webhook_event_type::WebhookEventType};
use chrono::{DateTime, Utc};

pub const SIGNATURE_HEADER: &str = "lightspark-signature";

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        hex_digest: &str,
        webhook_secret: &str,
    ) -> Result<WebhookEvent, Error> {
        if let Ok(digest_bytes) = hex::decode(hex_digest) {
            let hmac: Hmac<Sha256> = Hmac::new_from_slice(webhook_secret.as_bytes())
                .expect("HMAC can take key of any size")
                .chain_update(data);

            if hmac.verify_slice(digest_bytes.as_slice()).is_ok() {
                return serde_json::from_slice(data).map_err(Error::JsonError);
            }
        }
        Err(Error::WebhookSignatureError)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_verify_and_parse() {
        let data = "{\"event_type\": \"NODE_STATUS\", \"event_id\": \"1615c8be5aa44e429eba700db2ed8ca5\", \"timestamp\": \"2023-05-17T23:56:47.874449+00:00\", \"entity_id\": \"lightning_node:01882c25-157a-f96b-0000-362d42b64397\"}";
        let hex_digest = "62a8829aeb48b4142533520b1f7f86cdb1ee7d718bf3ea15bc1c662d4c453b74";
        let webhook_secret = "3gZ5oQQUASYmqQNuEk0KambNMVkOADDItIJjzUlAWjX";

        let result =
            super::WebhookEvent::verify_and_parse(data.as_bytes(), hex_digest, webhook_secret)
                .expect("Success case");

        assert_eq!(result.event_type, super::WebhookEventType::NodeStatus);
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

    #[test]
    fn test_invalid_signature() {
        let data = "{\"event_type\": \"NODE_STATUS\", \"event_id\": \"1615c8be5aa44e429eba700db2ed8ca5\", \"timestamp\": \"2023-05-17T23:56:47.874449+00:00\", \"entity_id\": \"lightning_node:01882c25-157a-f96b-0000-362d42b64397\"}";
        let hex_digest = "deadbeef";
        let webhook_secret = "3gZ5oQQUASYmqQNuEk0KambNMVkOADDItIJjzUlAWjX";

        let result =
            super::WebhookEvent::verify_and_parse(data.as_bytes(), hex_digest, webhook_secret);

        assert!(matches!(result, Err(super::Error::WebhookSignatureError)));
    }

    #[test]
    fn test_invalid_digest_bytes() {
        let data = "{\"event_type\": \"NODE_STATUS\", \"event_id\": \"1615c8be5aa44e429eba700db2ed8ca5\", \"timestamp\": \"2023-05-17T23:56:47.874449+00:00\", \"entity_id\": \"lightning_node:01882c25-157a-f96b-0000-362d42b64397\"}";
        let hex_digest = "NotAHexDigest";
        let webhook_secret = "3gZ5oQQUASYmqQNuEk0KambNMVkOADDItIJjzUlAWjX";

        let result =
            super::WebhookEvent::verify_and_parse(data.as_bytes(), hex_digest, webhook_secret);

        assert!(matches!(result, Err(super::Error::WebhookSignatureError)));
    }

    #[test]
    fn test_remote_signing_webhook() {
        let data = "{\"event_type\": \"REMOTE_SIGNING\", \"event_id\": \"1615c8be5aa44e429eba700db2ed8ca5\", \"timestamp\": \"2023-05-17T23:56:47.874449+00:00\", \"entity_id\": \"lightning_node:01882c25-157a-f96b-0000-362d42b64397\", \"data\": {\"sub_event_type\": \"ECDH\", \"public_key\": \"027c4b09ffb985c298afe7e5813266cbfcb7780b480ac294b0b43dc21f2be3d13c\"}}";
        let hex_digest = "17db38526ce47682f4052e3182766fe2f23810ac538e32d5f20bbe1deb2e3519";
        let webhook_secret = "3gZ5oQQUASYmqQNuEk0KambNMVkOADDItIJjzUlAWjX";

        let result =
            super::WebhookEvent::verify_and_parse(data.as_bytes(), hex_digest, webhook_secret)
                .expect("Success case");

        assert_eq!(result.event_type, super::WebhookEventType::RemoteSigning);

        let data = result.data.expect("Data is missing");
        let sub_event_type = data["sub_event_type"]
            .as_str()
            .expect("sub_event_type is missing");
        assert_eq!(sub_event_type, "ECDH");

        let public_key = data["public_key"].as_str().expect("public_key is missing");
        assert_eq!(
            public_key,
            "027c4b09ffb985c298afe7e5813266cbfcb7780b480ac294b0b43dc21f2be3d13c"
        );
    }
}
