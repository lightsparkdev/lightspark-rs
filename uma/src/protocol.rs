// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::fmt;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::kyc_status::KycStatus;

use super::{
    currency::Currency,
    payer_data::{PayerData, PayerDataOptions},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidReceiverAddress,
    InvalidUrl,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidReceiverAddress => write!(f, "Invalid receiver address"),
            Self::InvalidUrl => write!(f, "Invalid URL"),
        }
    }
}

/// LnurlpRequest is the first request in the UMA protocol.
/// It is sent by the VASP that is sending the payment to find out information about the receiver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LnurlpRequest {
    /// receiver_address is the address of the user at VASP2 that is receiving the payment.
    pub receiver_address: String,

    /// nonce is a random string that is used to prevent replay attacks.
    pub nonce: String,

    /// signature is the hex-encoded signature of sha256(receiver_address|nonce|timestamp).
    pub signature: String,

    /// is_subject_to_travel_rule indicates VASP1 is a financial institution that requires travel
    /// rule information.
    pub is_subject_to_travel_rule: bool,

    /// vasp_domain is the domain of the VASP that is sending the payment. It will be used by VASP2
    /// to fetch the public keys of VASP1.
    pub vasp_domain: String,

    /// timestamp is the unix timestamp of when the request was sent. Used in the signature.
    pub timestamp: i64,

    /// uma_version is the version of the UMA protocol that VASP1 prefers to use for this
    /// transaction. For the version negotiation flow,
    /// see https://static.swimlanes.io/87f5d188e080cb8e0494e46f80f2ae74.png
    pub uma_version: String,
}

impl LnurlpRequest {
    pub fn encode_to_url(&self) -> Result<url::Url, Error> {
        let receiver_address_parts: Vec<&str> = self.receiver_address.split('@').collect();
        if receiver_address_parts.len() != 2 {
            return Err(Error::InvalidReceiverAddress);
        }
        let scheme = if receiver_address_parts[1].starts_with("localhost:") {
            "http"
        } else {
            "https"
        };
        let mut lnurlp_url = Url::parse(&format!(
            "{}://{}/.well-known/lnurlp/{}",
            scheme, receiver_address_parts[1], receiver_address_parts[0]
        ))
        .map_err(|_| Error::InvalidUrl)?;

        lnurlp_url
            .query_pairs_mut()
            .append_pair("signature", &self.signature)
            .append_pair("vaspDomain", &self.vasp_domain)
            .append_pair("nonce", &self.nonce)
            .append_pair(
                "isSubjectToTravelRule",
                &self.is_subject_to_travel_rule.to_string(),
            )
            .append_pair("timestamp", &self.timestamp.to_string())
            .append_pair("umaVersion", &self.uma_version);

        Ok(lnurlp_url)
    }

    pub fn signable_payload(&self) -> Vec<u8> {
        let payload_string = format!(
            "{}|{}|{}",
            self.receiver_address, self.nonce, self.timestamp
        );
        payload_string.into_bytes()
    }
}

/// LnurlpResponse is the response to the LnurlpRequest.
/// It is sent by the VASP that is receiving the payment to provide information to the sender about the receiver.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LnurlpResponse {
    pub tag: String,
    pub callback: String,

    #[serde(rename = "minSendable")]
    pub min_sendable: i64,

    #[serde(rename = "maxSendable")]
    pub max_sendable: i64,

    #[serde(rename = "metadata")]
    pub encoded_metadata: String,

    pub currencies: Vec<Currency>,

    #[serde(rename = "payerData")]
    pub required_payer_data: PayerDataOptions,

    pub compliance: LnurlComplianceResponse,

    /// UmaVersion is the version of the UMA protocol that VASP2 has chosen for this transaction
    /// based on its own support and VASP1's specified preference in the LnurlpRequest. For the
    /// version negotiation flow, see
    /// https://static.swimlanes.io/87f5d188e080cb8e0494e46f80f2ae74.png
    #[serde(rename = "umaVersion")]
    pub uma_version: String,
}

/// LnurlComplianceResponse is the `compliance` field  of the LnurlpResponse.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LnurlComplianceResponse {
    /// kyc_status indicates whether VASP2 has KYC information about the receiver.
    #[serde(rename = "kycStatus")]
    pub kyc_status: KycStatus,

    /// signature is the hex-encoded signature of sha256(ReceiverAddress|Nonce|Timestamp).
    pub signature: String,

    /// nonce is a random string that is used to prevent replay attacks.
    #[serde(rename = "signatureNonce")]
    pub nonce: String,

    /// timestamp is the unix timestamp of when the request was sent. Used in the signature.
    #[serde(rename = "signatureTimestamp")]
    pub timestamp: i64,

    /// is_subject_to_travel_rule indicates whether VASP2 is a financial institution that requires travel rule information.
    #[serde(rename = "isSubjectToTravelRule")]
    pub is_subject_to_travel_rule: bool,

    /// receiver_identifier is the identifier of the receiver at VASP2.
    #[serde(rename = "receiverIdentifier")]
    pub receiver_identifier: String,
}

impl LnurlpResponse {
    pub fn signable_payload(&self) -> Vec<u8> {
        let payload_string = format!(
            "{}|{}|{}",
            self.compliance.receiver_identifier, self.compliance.nonce, self.compliance.timestamp
        );
        payload_string.into_bytes()
    }
}

/// PayRequest is the request sent by the sender to the receiver to retrieve an invoice.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PayRequest {
    /// currency_code is the ISO 3-digit currency code that the receiver will receive for this
    /// payment.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,

    /// amount is the amount that the receiver will receive for this payment in the smallest unit of
    /// the specified currency (i.e. cents for USD).
    pub amount: i64,

    /// PayerData is the data that the sender will send to the receiver to identify themselves.
    #[serde(rename = "payerData")]
    pub payer_data: PayerData,
}

impl PayRequest {
    pub fn signable_payload(&self) -> Vec<u8> {
        let payload_string = format!(
            "{}|{}|{}",
            self.payer_data.identifier,
            self.payer_data.compliance.signature_nonce,
            self.payer_data.compliance.signature_timestamp,
        );
        payload_string.into_bytes()
    }
}

/// PayReqResponse is the response sent by the receiver to the sender to provide an invoice.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PayReqResponse {
    /// encoded_invoice is the BOLT11 invoice that the sender will pay.
    #[serde(rename = "pr")]
    pub encoded_invoice: String,

    /// routes is usually just an empty list from legacy LNURL, which was replaced by route hints in
    /// the BOLT11 invoice.
    pub routes: Vec<Route>,

    pub compliance: PayReqResponseCompliance,

    #[serde(rename = "paymentInfo")]
    pub payment_info: PayReqResponsePaymentInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Route {
    pub pubkey: String,
    pub path: Vec<Path>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub pubkey: String,
    pub fee: i64,
    pub msatoshi: i64,
    pub channel: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PayReqResponseCompliance {
    /// node_pub_key is the public key of the receiver's node if known.
    #[serde(rename = "nodePubKey")]
    pub node_pub_key: Option<String>,

    /// utxos is a list of UTXOs of channels over which the receiver will likely receive the
    /// payment.
    pub utxos: Vec<String>,

    /// utxo_callback is the URL that the sender VASP will call to send UTXOs of the channel that
    /// the sender used to send the payment once it completes.
    #[serde(rename = "utxoCallback")]
    pub utxo_callback: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PayReqResponsePaymentInfo {
    /// CurrencyCode is the ISO 3-digit currency code that the receiver will receive for this
    /// payment.
    #[serde(rename = "currencyCode")]
    pub currency_code: String,

    /// Multiplier is the conversion rate. It is the number of millisatoshis that the receiver will
    /// receive for 1 unit of the specified currency.
    #[serde(rename = "multiplier")]
    pub multiplier: i64,

    /// ExchangeFeesMillisatoshi is the fees charged (in millisats) by the receiving VASP for this
    /// transaction. This is separate from the Multiplier.
    #[serde(rename = "exchangeFeesMillisatoshi")]
    pub exchange_fees_millisatoshi: i64,
}

/// PubKeyResponse is sent from a VASP to another VASP to provide its public keys.
/// It is the response to GET requests at `/.well-known/lnurlpubkey`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PubKeyResponse {
    /// signing_pub_key is used to verify signatures from a VASP.
    #[serde(rename = "signingPubKey")]
    pub signing_pub_key: Vec<u8>,

    // encryption_pub_key is used to encrypt TR info sent to a VASP.
    #[serde(rename = "encryptionPubKey")]
    pub encryption_pub_key: Vec<u8>,

    // expiration_timestamp [Optional] Seconds since epoch at which these pub keys must be refreshed.
    // They can be safely cached until this expiration (or forever if null).
    #[serde(rename = "expirationTimestamp")]
    pub expiration_timestamp: Option<i64>,
}

/// UtxoWithAmount is a pair of utxo and amount transferred over that corresponding channel.
/// It can be used to register payment for KYT.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UtxoWithAmount {
    /// utxo The utxo of the channel over which the payment went through in the format of
    /// <transaction_hash>:<output_index>.
    pub utxo: String,

    /// Amount The amount of funds transferred in the payment in mSats.
    #[serde(rename = "amountMsats")]
    pub amount: i64,
}
