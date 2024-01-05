use lightspark::objects::bitcoin_network::BitcoinNetwork;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum SigningRequest {
    GetPerCommitmentPointRequest(GetPerCommitmentPointRequest),
    ReleasePerCommitmentSecretRequest(ReleasePerCommitmentSecretRequest),
    DeriveKeyAndSignRequest(DeriveKeyAndSignRequest),
    InvoicePaymentHashRequest(InvoicePaymentHashRequest),
    ReleasePaymentPreimageRequest(ReleasePaymentPreimageRequest),
    ReleaseCounterpartyPerCommitmentSecretRequest(ReleaseCounterpartyPerCommitmentSecretRequest),
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

/// A signing request asking for a per-commitment secret to be released for a particular channel.
/// The per-commitment secret is the secret described in bolt 3.
///
/// [Bolt 3]: https://github.com/lightning/bolts/blob/master/03-transactions.md#per-commitment-secret-requirements
#[derive(Clone, Deserialize)]
pub struct ReleasePerCommitmentSecretRequest {
    pub channel_id: String,
    pub per_commitment_secret: String,
    pub bitcoin_network: BitcoinNetwork,
}

/// A signing request asking for a key to be derived and used to sign a message.
#[derive(Clone, Deserialize)]
pub struct DeriveKeyAndSignRequest {
    pub signing_jobs: Vec<SigningJob>,
    pub bitcoin_network: BitcoinNetwork,
}

/// A signing request asking for a payment hash.
/// A payment hash is the hash of a payment preimage. The payment preimage is the secret that is
/// revealed when a payment is made on the Lightning Network.
#[derive(Clone, Deserialize)]
pub struct InvoicePaymentHashRequest {
    pub invoice_id: String,
    pub bitcoin_network: BitcoinNetwork,
}

/// A signing request asking for a payment preimage to be released.
#[derive(Clone, Deserialize)]
pub struct ReleasePaymentPreimageRequest {
    pub invoice_id: String,
    pub nonce: Option<String>,
    pub bitcoin_network: BitcoinNetwork,
}

/// An informational request that reveals counterparty per-commitment secret.
#[derive(Clone, Deserialize)]
pub struct ReleaseCounterpartyPerCommitmentSecretRequest {
    pub channel_id: String,
    pub per_commitment_secret_idx: u64,
    pub per_commitment_secret: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SigningJob {
    pub id: String,
    pub derivation_path: String,
    pub message: String,
    pub add_tweak: Option<String>,
    pub mul_tweak: Option<String>,
}
