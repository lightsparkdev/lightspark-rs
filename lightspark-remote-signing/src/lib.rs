use std::fmt;

pub mod handler;
pub mod response;
pub mod signer;
pub mod validation;

#[derive(Debug)]
pub enum Error {
    WebhookEventNotRemoteSigning,
    WebhookEventDataMissing,
    UnknownSubEventType,
    PublicKeyDecodeError(hex::FromHexError),
    HexEncodingError,
    SignerError(signer::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::WebhookEventNotRemoteSigning => {
                "Webhook event is not a remote signing event".to_string()
            }
            Error::WebhookEventDataMissing => "Webhook event data is missing".to_string(),
            Error::UnknownSubEventType => "Unknown sub event type".to_string(),
            Error::PublicKeyDecodeError(e) => format!("Error decoding public key: {}", e),
            Error::HexEncodingError => "Error encoding hex".to_string(),
            Error::SignerError(e) => format!("Error signing: {}", e),
        };
        write!(f, "{}", msg)
    }
}
