use lightspark::{objects::id_and_signature::IdAndSignature, webhooks::WebhookEvent};
use tracing::info;

use crate::{
    response::Response,
    signer::LightsparkSigner,
    signing_requests::{
        DeriveKeyAndSignRequest, GetPerCommitmentPointRequest, InvoicePaymentHashRequest,
        ReleasePaymentPreimageRequest, ReleasePerCommitmentSecretRequest, SigningJob,
        SigningRequest,
    },
    signing_responses::{
        DeriveKeyAndSignResponse, GetPerCommitmentPointResponse, InvoicePaymentHashResponse,
        ReleasePaymentPreimageResponse, ReleasePerCommitmentSecretResponse, SigningResponse,
    },
    validation::Validation,
    Error,
};

/// A handler for lightspark remote signing webhook events.
pub struct Handler {
    signer: LightsparkSigner,
    validator: Box<dyn Validation>,
}

impl Handler {
    /// Create a new handler.
    /// # Arguments
    ///
    /// * `signer` - A LightsparkSigner instance, which will be used to sign messages.
    /// * `validator` - A Validation instance, which will be used to determine whether to sign messages.
    pub fn new(signer: LightsparkSigner, validator: Box<dyn Validation>) -> Self {
        Self { signer, validator }
    }

    pub fn handle_remote_signing_webhook_msg(
        &self,
        event: &WebhookEvent,
    ) -> Result<Option<Response>, Error> {
        let request = SigningRequest::parse_from_webhook_event(event)?;
        let event_json =
            serde_json::to_string(&event).expect("Serialize event to json should not fail");
        if !self.validator.should_sign(event_json) {
            self.handle_decline_to_sign_messages(event).map(Some)
        } else {
            let response: Option<SigningResponse> = match request {
                SigningRequest::GetPerCommitmentPointRequest(r) => {
                    Some(self.handle_get_per_commitment_point(&r)?)
                }
                SigningRequest::ReleasePerCommitmentSecretRequest(r) => {
                    Some(self.handle_release_per_commitment_secret(&r)?)
                }
                SigningRequest::DeriveKeyAndSignRequest(r) => {
                    Some(self.handle_derive_key_and_sign(&r)?)
                }
                SigningRequest::InvoicePaymentHashRequest(r) => {
                    Some(self.handle_request_invoice_payment_hash(&r)?)
                }
                SigningRequest::ReleasePaymentPreimageRequest(r) => {
                    Some(self.handle_release_payment_preimage(&r)?)
                }
                SigningRequest::ReleaseCounterpartyPerCommitmentSecretRequest(_) => None,
            };

            Ok(response.map(|r| Some(r.graphql_response())).flatten())
        }
    }

    pub fn handle_request_invoice_payment_hash(
        &self,
        request: &InvoicePaymentHashRequest,
    ) -> Result<SigningResponse, Error> {
        let nonce = self.signer.generate_preimage_nonce();
        let nonce_str = hex::encode(&nonce);

        let payment_hash = self
            .signer
            .generate_preimage_hash(nonce)
            .map_err(Error::SignerError)?;
        let payment_hash_str = hex::encode(payment_hash);
        Ok(SigningResponse::InvoicePaymentHashResponse(
            InvoicePaymentHashResponse {
                invoice_id: request.invoice_id.clone(),
                payment_hash: payment_hash_str,
                nonce: Some(nonce_str),
            },
        ))
    }

    pub fn handle_decline_to_sign_messages(&self, event: &WebhookEvent) -> Result<Response, Error> {
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;

        let signing_jobs: Vec<SigningJob> = serde_json::from_value(data["signing_jobs"].clone())
            .map_err(|_| Error::WebhookEventDataMissing)?;

        let payload_ids: Vec<String> = signing_jobs.iter().map(|job| job.id.clone()).collect();
        Ok(Response::decline_to_sign_message_response(&payload_ids))
    }

    pub fn handle_release_payment_preimage(
        &self,
        request: &ReleasePaymentPreimageRequest,
    ) -> Result<SigningResponse, Error> {
        info!("Handling release payment preimage webhook event");
        let nonce = request
            .nonce
            .clone()
            .expect("Nonce should not be empty if you use SDK to manage preimage");
        let nonce_bytes = hex::decode(nonce).map_err(|_| Error::HexEncodingError)?;
        let preimage = self
            .signer
            .generate_preimage(nonce_bytes)
            .map_err(Error::SignerError)?;

        Ok(SigningResponse::ReleasePaymentPreimageResponse(
            ReleasePaymentPreimageResponse {
                invoice_id: request.invoice_id.clone(),
                payment_preimage: hex::encode(preimage),
            },
        ))
    }

    pub fn handle_get_per_commitment_point(
        &self,
        request: &GetPerCommitmentPointRequest,
    ) -> Result<SigningResponse, Error> {
        info!("Handling get per commitment point webhook event");
        let per_commitment_point = self
            .signer
            .get_per_commitment_point(
                request.derivation_path.clone(),
                request.per_commitmnet_point_idx,
            )
            .map_err(Error::SignerError)?;

        let commitment_point_str = hex::encode(per_commitment_point);
        Ok(SigningResponse::GetPerCommitmentPointResponse(
            GetPerCommitmentPointResponse {
                channel_id: request.channel_id.clone(),
                per_commitment_point_idx: request.per_commitmnet_point_idx,
                per_commitment_point_hex: commitment_point_str,
            },
        ))
    }

    pub fn handle_release_per_commitment_secret(
        &self,
        request: &ReleasePerCommitmentSecretRequest,
    ) -> Result<SigningResponse, Error> {
        info!("Handling release per commitment secret webhook event");
        let commitment_secret = self
            .signer
            .release_per_commitment_secret(
                request.derivation_path.clone(),
                request.per_commitment_point_idx,
            )
            .map_err(Error::SignerError)?;

        let commitment_secret_str = hex::encode(commitment_secret);

        Ok(SigningResponse::ReleasePerCommitmentSecretResponse(
            ReleasePerCommitmentSecretResponse {
                channel_id: request.channel_id.clone(),
                per_commitment_secret: commitment_secret_str,
                per_commitment_point_idx: request.per_commitment_point_idx,
            },
        ))
    }

    pub fn handle_derive_key_and_sign(
        &self,
        requeset: &DeriveKeyAndSignRequest,
    ) -> Result<SigningResponse, Error> {
        info!("Handling derive key and sign webhook event");

        let mut signatures: Vec<IdAndSignature> = vec![];
        for signing_job in requeset.signing_jobs.clone() {
            let signature = self
                .signer
                .derive_key_and_sign(
                    hex::decode(signing_job.message).map_err(|_| Error::HexEncodingError)?,
                    signing_job.derivation_path,
                    signing_job
                        .add_tweak
                        .map(|tweak| hex::decode(tweak).map_err(|_| Error::HexEncodingError))
                        .transpose()?,
                    signing_job
                        .mul_tweak
                        .map(|tweak| hex::decode(tweak).map_err(|_| Error::HexEncodingError))
                        .transpose()?,
                )
                .map_err(Error::SignerError)?;

            signatures.push(IdAndSignature {
                id: signing_job.id,
                signature: hex::encode(signature),
            });
        }

        Ok(SigningResponse::DeriveKeyAndSignResponse(
            DeriveKeyAndSignResponse { signatures },
        ))
    }
}
