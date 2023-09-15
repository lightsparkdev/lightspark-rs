// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use lightspark::objects::{
    decline_to_sign_messages_output, id_and_signature::IdAndSignature,
    release_channel_per_commitment_secret_output, release_payment_preimage_output,
    set_invoice_payment_hash_output, sign_invoice_output, sign_messages_output,
    update_channel_per_commitment_point_output, update_node_shared_secret_output,
};
use serde_json::{json, Value};

/// This struct is used as a response for lightspark signing webhook requests.
/// The response for a remote signing webhook needs to be issued as a graphql request through
/// the lightspark client.
///
/// The response struct contains the graphql query and variables for the request.
#[derive(Debug, Clone)]
pub struct Response {
    pub query: String,
    pub variables: Value,
}

const UPDATE_NODE_SHARED_SECRET_MUTATION: &str = "mutation UpdateNodeSharedSecret(
    $node_id : ID!
    $shared_secret : Hash32!
  ) {
      update_node_shared_secret(input: {
          node_id: $node_id
          shared_secret: $shared_secret
      }) {
        ...UpdateNodeSharedSecretOutputFragment
      }
  }
";

const UPDATE_CHANNEL_PER_COMMITMENT_POINT_MUTATION: &str =
    "mutation UpdateChannelPerCommitmentPoint(
  $channel_id : ID!
  $per_commitment_point : PublicKey!
  $per_commitment_point_index : Long!
) {
    update_channel_per_commitment_point(input: {
		channel_id: $channel_id
    per_commitment_point_index: $per_commitment_point_index
		per_commitment_point: $per_commitment_point
	}) {
        ...UpdateChannelPerCommitmentPointOutputFragment
    }
}";

const RELEASE_CHANNEL_PER_COMMITMENT_SECRET_MUTATION: &str =
    "mutation ReleaseChannelPerCommitmentSecret(
  $channel_id: ID!
  $per_commitment_secret: Hash32!
  $per_commitment_index: Long!
) {
    release_channel_per_commitment_secret(input: {
		channel_id: $channel_id
		per_commitment_secret: $per_commitment_secret
        per_commitment_index: $per_commitment_index
	}) {
        ...ReleaseChannelPerCommitmentSecretOutputFragment
    }
}";

const SIGN_INVOICE_MUTATION: &str = "mutation SignInvoice(
  $invoice_id : ID!
  $signature : Signature!
  $recovery_id : Int!
) {
    sign_invoice(input: {
		invoice_id: $invoice_id
		signature: $signature
		recovery_id: $recovery_id
	}) {
        ...SignInvoiceOutputFragment
    }
}";

const RELEASE_PAYMENT_PREIMAGE_MUTATION: &str = "
mutation ReleasePaymentPreimage(
  $invoice_id: ID!
  $payment_preimage: Hash32!
) {
    release_payment_preimage(input: {
		invoice_id: $invoice_id
		payment_preimage: $payment_preimage
	}) {
        ...ReleasePaymentPreimageOutputFragment
    }
}";

const SIGN_MESSAGES_MUTATION: &str = "
mutation SignMessages(
  $signatures: [IdAndSignature!]!
) {
    sign_messages(input: {
		signatures: $signatures
	}) {
        ...SignMessagesOutputFragment
    }
}";

const DECLINE_TO_SIGN_MESSAGES_MUTATION: &str = "
mutation DeclineToSignMessages($payload_ids: [ID!]!) {
	decline_to_sign_messages(input: {
		payload_ids: $payload_ids
	}) {
		...DeclineToSignMessagesOutputFragment
	}
}";

const SET_INVOICE_PAYMENT_HASH: &str = "
mutation SetInvoicePaymentHash(
  $invoice_id: ID!
  $payment_hash: Hash32!
  $preimage_nonce: Hash32!
) {
    set_invoice_payment_hash(input: {
		invoice_id: $invoice_id
		payment_hash: $payment_hash
		preimage_nonce: $preimage_nonce
	}) {
        ...SetInvoicePaymentHashOutputFragment
    }
}";

impl Response {
    pub fn ecdh_response(node_id: &str, shared_secret: &str) -> Response {
        let variables = json!({
            "node_id": node_id,
            "shared_secret": shared_secret,
        });
        let query = format!(
            "{}\n{}",
            UPDATE_NODE_SHARED_SECRET_MUTATION,
            update_node_shared_secret_output::FRAGMENT
        );
        Response { query, variables }
    }

    pub fn get_channel_per_commitment_response(
        channel_id: &str,
        per_commitment_point: &str,
        per_commitment_point_index: u64,
    ) -> Response {
        let variables = json!({
            "channel_id": channel_id,
            "per_commitment_point": per_commitment_point,
            "per_commitment_point_index": per_commitment_point_index,
        });
        let query = format!(
            "{}\n{}",
            UPDATE_CHANNEL_PER_COMMITMENT_POINT_MUTATION,
            update_channel_per_commitment_point_output::FRAGMENT
        );
        Response { query, variables }
    }

    pub fn release_channel_per_commitment_secret_response(
        channel_id: &str,
        per_commitment_secret: &str,
        per_commitment_index: i64,
    ) -> Response {
        let variables = json!({
            "channel_id": channel_id,
            "per_commitment_secret": per_commitment_secret,
            "per_commitment_index": per_commitment_index,
        });
        let query = format!(
            "{}\n{}",
            RELEASE_CHANNEL_PER_COMMITMENT_SECRET_MUTATION,
            release_channel_per_commitment_secret_output::FRAGMENT
        );
        Response { query, variables }
    }

    pub fn sign_invoice_response(invoice_id: &str, signature: &str, recovery_id: i32) -> Response {
        let variables = json!({
            "invoice_id": invoice_id,
            "signature": signature,
            "recovery_id": recovery_id,
        });
        let query = format!(
            "{}\n{}",
            SIGN_INVOICE_MUTATION,
            sign_invoice_output::FRAGMENT
        );
        Response { query, variables }
    }

    pub fn release_payment_preimage_response(invoice_id: &str, payment_preimage: &str) -> Response {
        let variables = json!({
            "invoice_id": invoice_id,
            "payment_preimage": payment_preimage,
        });
        let query = format!(
            "{}\n{}",
            RELEASE_PAYMENT_PREIMAGE_MUTATION,
            release_payment_preimage_output::FRAGMENT
        );
        Response { query, variables }
    }

    pub fn sign_messages_response(signatures: Vec<IdAndSignature>) -> Response {
        let variables = json!({
            "signatures": signatures,
        });
        let query = format!(
            "{}\n{}",
            SIGN_MESSAGES_MUTATION,
            sign_messages_output::FRAGMENT
        );
        Response { query, variables }
    }

    pub fn decline_to_sign_message_response(payload_ids: &[String]) -> Response {
        let variables = json!({
            "payload_ids": payload_ids,
        });
        let query = format!(
            "{}\n{}",
            DECLINE_TO_SIGN_MESSAGES_MUTATION,
            decline_to_sign_messages_output::FRAGMENT
        );
        Response { query, variables }
    }

    pub fn set_invoice_payment_hash_response(
        invoice_id: &str,
        payment_hash: &str,
        pre_image_nonce: &str,
    ) -> Response {
        let variables = json!({
            "invoice_id": invoice_id,
            "payment_hash": payment_hash,
            "preimage_nonce": pre_image_nonce,
        });
        let query = format!(
            "{}\n{}",
            SET_INVOICE_PAYMENT_HASH,
            set_invoice_payment_hash_output::FRAGMENT
        );
        Response { query, variables }
    }
}
