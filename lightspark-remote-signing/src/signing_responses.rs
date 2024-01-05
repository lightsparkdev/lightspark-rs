use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum SigningResponse {
    GetPerCommitmentPointResponse(GetPerCommitmentPointResponse),
    ReleasePerCommitmentSecretResponse(ReleasePerCommitmentSecretResponse),
    DeriveKeyAndSignResponse(DeriveKeyAndSignResponse),
    InvoicePaymentHashResponse(InvoicePaymentHashResponse),
    ReleasePaymentPreimageResponse(ReleasePaymentPreimageResponse),
}

#[derive(Clone)]
pub struct GetPerCommitmentPointResponse {
    pub channel_id: String,
    pub per_commitment_point_idx: u64,
    pub per_commitment_point_hex: String,
}

#[derive(Clone)]
pub struct ReleasePerCommitmentSecretResponse {
    pub channel_id: String,
    pub per_commitment_secret: String,
    pub per_commitment_point_idx: i64,
}

#[derive(Clone)]
pub struct DeriveKeyAndSignResponse {
    pub signatures: Vec<SignatureResponse>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignatureResponse {
    pub id: String,
    pub signature: String,
}

#[derive(Clone)]
pub struct InvoicePaymentHashResponse {
    pub invoice_id: String,
    pub payment_hash: String,
    pub nonce: Option<String>,
}

#[derive(Clone)]
pub struct ReleasePaymentPreimageResponse {
    pub invoice_id: String,
    pub payment_preimage: String,
}
