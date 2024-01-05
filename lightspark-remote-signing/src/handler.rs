use lightspark::{
    objects::{
        id_and_signature::IdAndSignature, remote_signing_sub_event_type::RemoteSigningSubEventType,
        webhook_event_type::WebhookEventType,
    },
    webhooks::WebhookEvent,
};
use serde_json::from_value;
use tracing::info;

use crate::{
    response::Response, signer::LightsparkSigner, signing_requests::SigningJob,
    validation::Validation, Error,
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
        if !matches!(event.event_type, WebhookEventType::RemoteSigning) {
            return Err(Error::WebhookEventNotRemoteSigning);
        }

        let data = &event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
        let sub_type: RemoteSigningSubEventType = from_value(data["sub_event_type"].clone())
            .map_err(|_| Error::WebhookEventDataMissing)?;
        info!("handler for sub_type: {:?}", sub_type.to_string());
        let event_json =
            serde_json::to_string(&event).expect("Serialize event to json should not fail");
        if !self.validator.should_sign(event_json) {
            self.handle_decline_to_sign_messages(event).map(Some)
        } else {
            match sub_type {
                RemoteSigningSubEventType::Ecdh => self.handle_ecdh(event).map(Some),
                RemoteSigningSubEventType::SignInvoice => self.handle_sign_invoice(event).map(Some),
                RemoteSigningSubEventType::ReleasePaymentPreimage => {
                    self.handle_release_payment_preimage(event).map(Some)
                }
                RemoteSigningSubEventType::GetPerCommitmentPoint => {
                    self.handle_get_per_commitment_point(event).map(Some)
                }
                RemoteSigningSubEventType::ReleasePerCommitmentSecret => {
                    self.handle_release_per_commitment_secret(event).map(Some)
                }
                RemoteSigningSubEventType::DeriveKeyAndSign => {
                    self.handle_derive_key_and_sign(event).map(Some)
                }
                RemoteSigningSubEventType::RequestInvoicePaymentHash => {
                    self.handle_request_invoice_payment_hash(event).map(Some)
                }
                RemoteSigningSubEventType::RevealCounterpartyPerCommitmentSecret => Ok(None),
            }
        }
    }

    pub fn handle_request_invoice_payment_hash(
        &self,
        event: &WebhookEvent,
    ) -> Result<Response, Error> {
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
        let invoice_id = data["invoice_id"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;
        let nonce = self.signer.generate_preimage_nonce();
        let nonce_str = hex::encode(&nonce);

        let payment_hash = self
            .signer
            .generate_preimage_hash(nonce)
            .map_err(Error::SignerError)?;
        let payment_hash_str = hex::encode(payment_hash);
        Ok(Response::set_invoice_payment_hash_response(
            invoice_id,
            &payment_hash_str,
            &nonce_str,
        ))
    }

    pub fn handle_decline_to_sign_messages(&self, event: &WebhookEvent) -> Result<Response, Error> {
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;

        let signing_jobs: Vec<SigningJob> = serde_json::from_value(data["signing_jobs"].clone())
            .map_err(|_| Error::WebhookEventDataMissing)?;

        let payload_ids: Vec<String> = signing_jobs.iter().map(|job| job.id.clone()).collect();
        Ok(Response::decline_to_sign_message_response(&payload_ids))
    }

    pub fn handle_ecdh(&self, event: &WebhookEvent) -> Result<Response, Error> {
        info!("Handling ECDH webhook event");
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
        let node_id = &event.entity_id;
        let public_key = data["peer_public_key"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;
        let public_key_bytes = hex::decode(public_key).map_err(Error::PublicKeyDecodeError)?;
        let ss = self
            .signer
            .ecdh(public_key_bytes.to_vec())
            .map_err(Error::SignerError)?;
        let ss_str = hex::encode(ss);
        Ok(Response::ecdh_response(node_id, &ss_str))
    }

    pub fn handle_sign_invoice(&self, event: &WebhookEvent) -> Result<Response, Error> {
        info!("Handling sign invoice webhook event");
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
        let invoice_id = data["invoice_id"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;
        let invoice_hash = data["payreq_hash"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;
        let invoice_hash_bytes = hex::decode(invoice_hash).map_err(|_| Error::HexEncodingError)?;
        let signature = self
            .signer
            .sign_invoice_hash(invoice_hash_bytes)
            .map_err(Error::SignerError)?;
        Ok(Response::sign_invoice_response(
            invoice_id,
            hex::encode(signature.get_signature()).as_str(),
            signature.get_recovery_id(),
        ))
    }

    pub fn handle_release_payment_preimage(&self, event: &WebhookEvent) -> Result<Response, Error> {
        info!("Handling release payment preimage webhook event");
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
        let nonce = data["preimage_nonce"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;
        let invoice_id = data["invoice_id"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;

        let nonce_bytes = hex::decode(nonce).map_err(|_| Error::HexEncodingError)?;
        let preimage = self
            .signer
            .generate_preimage(nonce_bytes)
            .map_err(Error::SignerError)?;

        Ok(Response::release_payment_preimage_response(
            invoice_id,
            hex::encode(preimage).as_str(),
        ))
    }

    pub fn handle_get_per_commitment_point(&self, event: &WebhookEvent) -> Result<Response, Error> {
        info!("Handling get per commitment point webhook event");
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
        let per_commitment_point_idx = data["per_commitment_point_idx"]
            .as_u64()
            .ok_or(Error::WebhookEventDataMissing)?;

        let derivation_path = data["derivation_path"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;

        let channel_id = &event.entity_id;

        let per_commitment_point = self
            .signer
            .get_per_commitment_point(derivation_path.to_string(), per_commitment_point_idx)
            .map_err(Error::SignerError)?;

        let commitment_point_str = hex::encode(per_commitment_point);
        Ok(Response::get_channel_per_commitment_response(
            channel_id,
            commitment_point_str.as_str(),
            per_commitment_point_idx,
        ))
    }

    pub fn handle_release_per_commitment_secret(
        &self,
        event: &WebhookEvent,
    ) -> Result<Response, Error> {
        info!("Handling release per commitment secret webhook event");
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;
        let per_commitment_point_idx = data["per_commitment_point_idx"]
            .as_u64()
            .ok_or(Error::WebhookEventDataMissing)?;

        let derivation_path = data["derivation_path"]
            .as_str()
            .ok_or(Error::WebhookEventDataMissing)?;

        let channel_id = &event.entity_id;
        let commitment_secret = self
            .signer
            .release_per_commitment_secret(derivation_path.to_string(), per_commitment_point_idx)
            .map_err(Error::SignerError)?;

        let commitment_secret_str = hex::encode(commitment_secret);

        Ok(Response::release_channel_per_commitment_secret_response(
            channel_id,
            &commitment_secret_str,
            per_commitment_point_idx as i64,
        ))
    }

    pub fn handle_derive_key_and_sign(&self, event: &WebhookEvent) -> Result<Response, Error> {
        info!("Handling derive key and sign webhook event");
        let data = event.data.as_ref().ok_or(Error::WebhookEventDataMissing)?;

        let signing_jobs: Vec<SigningJob> = serde_json::from_value(data["signing_jobs"].clone())
            .map_err(|_| Error::WebhookEventDataMissing)?;

        let mut signatures: Vec<IdAndSignature> = vec![];
        for signing_job in signing_jobs {
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
        Ok(Response::sign_messages_response(signatures))
    }
}
