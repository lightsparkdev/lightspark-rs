use lightspark::{
    objects::{
        bitcoin_network::BitcoinNetwork, remote_signing_sub_event_type::RemoteSigningSubEventType,
        webhook_event_type::WebhookEventType,
    },
    webhooks::WebhookEvent,
};
use serde::Deserialize;
use serde_json::from_value;

use crate::Error;

#[derive(Clone, Deserialize)]
pub enum SigningRequest {
    GetPerCommitmentPointRequest(GetPerCommitmentPointRequest),
    ReleasePerCommitmentSecretRequest(ReleasePerCommitmentSecretRequest),
    DeriveKeyAndSignRequest(DeriveKeyAndSignRequest),
    InvoicePaymentHashRequest(InvoicePaymentHashRequest),
    ReleasePaymentPreimageRequest(ReleasePaymentPreimageRequest),
    ReleaseCounterpartyPerCommitmentSecretRequest(ReleaseCounterpartyPerCommitmentSecretRequest),
}

impl SigningRequest {
    pub fn parse_from_webhook_event(webhook_event: &WebhookEvent) -> Result<Self, Error> {
        if !matches!(webhook_event.event_type, WebhookEventType::RemoteSigning) {
            return Err(Error::WebhookEventNotRemoteSigning);
        }

        let data = &webhook_event
            .data
            .as_ref()
            .ok_or(Error::WebhookEventDataMissing)?;
        let sub_type: RemoteSigningSubEventType = from_value(data["sub_event_type"].clone())
            .map_err(|_| Error::WebhookEventDataMissing)?;
        match sub_type {
            RemoteSigningSubEventType::GetPerCommitmentPoint => {
                Ok(Self::GetPerCommitmentPointRequest(
                    GetPerCommitmentPointRequest::parse_from_webhook_event(webhook_event)?,
                ))
            }
            RemoteSigningSubEventType::ReleasePerCommitmentSecret => {
                Ok(Self::ReleasePerCommitmentSecretRequest(
                    ReleasePerCommitmentSecretRequest::parse_from_webhook_event(webhook_event)?,
                ))
            }
            RemoteSigningSubEventType::DeriveKeyAndSign => Ok(Self::DeriveKeyAndSignRequest(
                DeriveKeyAndSignRequest::parse_from_webhook_event(webhook_event)?,
            )),
            RemoteSigningSubEventType::RequestInvoicePaymentHash => {
                Ok(Self::InvoicePaymentHashRequest(
                    InvoicePaymentHashRequest::parse_from_webhook_event(webhook_event)?,
                ))
            }
            RemoteSigningSubEventType::ReleasePaymentPreimage => {
                Ok(Self::ReleasePaymentPreimageRequest(
                    ReleasePaymentPreimageRequest::parse_from_webhook_event(webhook_event)?,
                ))
            }
            RemoteSigningSubEventType::RevealCounterpartyPerCommitmentSecret => {
                Ok(Self::ReleaseCounterpartyPerCommitmentSecretRequest(
                    ReleaseCounterpartyPerCommitmentSecretRequest::parse_from_webhook_event(
                        webhook_event,
                    )?,
                ))
            }
            _ => Err(Error::WebhookEventTypeNotSupported),
        }
    }
}

/// A signing request asking for a per-commitment point for a particular channel.
/// The per-commitment point is the point on the secp256k1 curve for the commitment secret described
/// in bolt 3.
///
/// [Bolt 3]: https://github.com/lightning/bolts/blob/master/03-transactions.md#per-commitment-secret-requirements
#[derive(Clone, Deserialize)]
pub struct GetPerCommitmentPointRequest {
    pub channel_id: String,
    pub derivation_path: String,
    pub per_commitmnet_point_idx: u64,
    pub bitcoin_network: BitcoinNetwork,
}

impl GetPerCommitmentPointRequest {
    pub fn parse_from_webhook_event(webhook_event: &WebhookEvent) -> Result<Self, Error> {
        let data = webhook_event
            .data
            .as_ref()
            .ok_or(Error::WebhookEventDataMissing)?;
        let data = data
            .as_object()
            .ok_or(Error::WebhookEventDataMissing)?
            .clone();
        let channel_id = webhook_event.entity_id.clone();
        let derivation_path = data
            .get("derivation_path")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?
            .to_string();
        let per_commitmnet_point_idx = data
            .get("per_commitment_point_idx")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_u64()
            .ok_or(Error::WebhookEventDataMissing)?;
        let bitcoin_network = bitcoin_network_from_webhook_event(webhook_event)?;
        Ok(Self {
            channel_id,
            derivation_path,
            per_commitmnet_point_idx,
            bitcoin_network,
        })
    }
}

/// A signing request asking for a per-commitment secret to be released for a particular channel.
/// The per-commitment secret is the secret described in bolt 3.
///
/// [Bolt 3]: https://github.com/lightning/bolts/blob/master/03-transactions.md#per-commitment-secret-requirements
#[derive(Clone, Deserialize)]
pub struct ReleasePerCommitmentSecretRequest {
    pub channel_id: String,
    pub derivation_path: String,
    pub per_commitment_point_idx: u64,
    pub bitcoin_network: BitcoinNetwork,
}

impl ReleasePerCommitmentSecretRequest {
    pub fn parse_from_webhook_event(webhook_event: &WebhookEvent) -> Result<Self, Error> {
        let data = webhook_event
            .data
            .as_ref()
            .ok_or(Error::WebhookEventDataMissing)?;
        let data = data
            .as_object()
            .ok_or(Error::WebhookEventDataMissing)?
            .clone();
        let channel_id = webhook_event.entity_id.clone();
        let derivation_path = data
            .get("derivation_path")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?
            .to_string();
        let per_commitment_point_idx = data
            .get("per_commitment_point_idx")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_u64()
            .ok_or(Error::WebhookEventDataMissing)?;
        let bitcoin_network = bitcoin_network_from_webhook_event(webhook_event)?;
        Ok(Self {
            channel_id,
            derivation_path,
            per_commitment_point_idx,
            bitcoin_network,
        })
    }
}

/// A signing request asking for a key to be derived and used to sign a message.
#[derive(Clone, Deserialize)]
pub struct DeriveKeyAndSignRequest {
    pub signing_jobs: Vec<SigningJob>,
    pub bitcoin_network: BitcoinNetwork,
}

impl DeriveKeyAndSignRequest {
    pub fn parse_from_webhook_event(webhook_event: &WebhookEvent) -> Result<Self, Error> {
        let data = webhook_event
            .data
            .as_ref()
            .ok_or(Error::WebhookEventDataMissing)?;
        let data = data
            .as_object()
            .ok_or(Error::WebhookEventDataMissing)?
            .clone();
        let signing_jobs = data
            .get("signing_jobs")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_array()
            .ok_or(Error::WebhookEventDataMissing)?
            .iter()
            .map(|v| serde_json::from_value(v.clone()).map_err(|_| Error::WebhookEventDataMissing))
            .collect::<Result<Vec<SigningJob>, Error>>()?;
        let bitcoin_network = bitcoin_network_from_webhook_event(webhook_event)?;
        Ok(Self {
            signing_jobs,
            bitcoin_network,
        })
    }
}

/// A signing request asking for a payment hash.
/// A payment hash is the hash of a payment preimage. The payment preimage is the secret that is
/// revealed when a payment is made on the Lightning Network.
#[derive(Clone, Deserialize)]
pub struct InvoicePaymentHashRequest {
    pub invoice_id: String,
    pub bitcoin_network: BitcoinNetwork,
}

impl InvoicePaymentHashRequest {
    pub fn parse_from_webhook_event(webhook_event: &WebhookEvent) -> Result<Self, Error> {
        let data = webhook_event
            .data
            .as_ref()
            .ok_or(Error::WebhookEventDataMissing)?;
        let data = data
            .as_object()
            .ok_or(Error::WebhookEventDataMissing)?
            .clone();
        let invoice_id = data
            .get("invoice_id")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?
            .to_string();
        let bitcoin_network = bitcoin_network_from_webhook_event(webhook_event)?;
        Ok(Self {
            invoice_id,
            bitcoin_network,
        })
    }
}

/// A signing request asking for a payment preimage to be released.
#[derive(Clone, Deserialize)]
pub struct ReleasePaymentPreimageRequest {
    pub invoice_id: String,
    pub nonce: Option<String>,
    pub bitcoin_network: BitcoinNetwork,
}

impl ReleasePaymentPreimageRequest {
    pub fn parse_from_webhook_event(webhook_event: &WebhookEvent) -> Result<Self, Error> {
        let data = webhook_event
            .data
            .as_ref()
            .ok_or(Error::WebhookEventDataMissing)?;
        let data = data
            .as_object()
            .ok_or(Error::WebhookEventDataMissing)?
            .clone();
        let invoice_id = data
            .get("invoice_id")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?
            .to_string();
        let nonce = data
            .get("preimage_nonce")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_str()
            .map(|s| s.to_string());
        let bitcoin_network = bitcoin_network_from_webhook_event(webhook_event)?;
        Ok(Self {
            invoice_id,
            nonce,
            bitcoin_network,
        })
    }
}

/// An informational request that reveals counterparty per-commitment secret.
#[derive(Clone, Deserialize)]
pub struct ReleaseCounterpartyPerCommitmentSecretRequest {
    pub channel_id: String,
    pub per_commitment_secret_idx: u64,
    pub per_commitment_secret: String,
}

impl ReleaseCounterpartyPerCommitmentSecretRequest {
    pub fn parse_from_webhook_event(webhook_event: &WebhookEvent) -> Result<Self, Error> {
        let data = webhook_event
            .data
            .as_ref()
            .ok_or(Error::WebhookEventDataMissing)?;
        let data = data
            .as_object()
            .ok_or(Error::WebhookEventDataMissing)?
            .clone();
        let channel_id = webhook_event.entity_id.clone();
        let per_commitment_secret_idx = data
            .get("per_commitment_secret_idx")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_u64()
            .ok_or(Error::WebhookEventDataMissing)?;
        let per_commitment_secret = data
            .get("per_commitment_secret")
            .ok_or(Error::WebhookEventDataMissing)?
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?
            .to_string();
        Ok(Self {
            channel_id,
            per_commitment_secret_idx,
            per_commitment_secret,
        })
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct SigningJob {
    pub id: String,
    pub derivation_path: String,
    pub message: String,
    pub add_tweak: Option<String>,
    pub mul_tweak: Option<String>,
}

fn bitcoin_network_from_webhook_event(event: &WebhookEvent) -> Result<BitcoinNetwork, Error> {
    let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
    let data = data
        .as_object()
        .ok_or(Error::WebhookEventDataMissing)?
        .clone();
    let bitcoin_network_str = data
        .get("bitcoin_network")
        .ok_or(Error::WebhookEventDataMissing)?
        .as_str()
        .ok_or(Error::WebhookEventDataMissing)?;
    match bitcoin_network_str {
        "MAINNET" => Ok(BitcoinNetwork::Mainnet),
        "TESTNET" => Ok(BitcoinNetwork::Testnet),
        "REGTEST" => Ok(BitcoinNetwork::Regtest),
        _ => Err(Error::WebhookEventDataMissing),
    }
}
