use lightspark::objects::id_and_signature::IdAndSignature;

use crate::response::Response;

#[derive(Clone)]
pub enum SigningResponse {
    GetPerCommitmentPointResponse(GetPerCommitmentPointResponse),
    ReleasePerCommitmentSecretResponse(ReleasePerCommitmentSecretResponse),
    DeriveKeyAndSignResponse(DeriveKeyAndSignResponse),
    InvoicePaymentHashResponse(InvoicePaymentHashResponse),
    ReleasePaymentPreimageResponse(ReleasePaymentPreimageResponse),
}

impl SigningResponse {
    pub fn graphql_response(&self) -> Response {
        match self {
            Self::GetPerCommitmentPointResponse(r) => {
                Response::get_channel_per_commitment_response(
                    &r.channel_id,
                    &r.per_commitment_point_hex,
                    r.per_commitment_point_idx,
                )
            }
            Self::ReleasePerCommitmentSecretResponse(r) => {
                Response::release_channel_per_commitment_secret_response(
                    &r.channel_id,
                    &r.per_commitment_secret,
                    r.per_commitment_point_idx,
                )
            }
            Self::DeriveKeyAndSignResponse(r) => {
                Response::sign_messages_response(r.signatures.clone())
            }
            Self::InvoicePaymentHashResponse(r) => Response::set_invoice_payment_hash_response(
                &r.invoice_id,
                &r.payment_hash,
                r.nonce.as_deref(),
            ),
            Self::ReleasePaymentPreimageResponse(r) => {
                Response::release_payment_preimage_response(&r.invoice_id, &r.payment_preimage)
            }
        }
    }
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
    pub signatures: Vec<IdAndSignature>,
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
