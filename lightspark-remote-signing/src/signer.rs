// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::fmt;
use std::str::FromStr;

use bitcoin::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use bitcoin::hashes::{sha512, Hash, HashEngine, Hmac, HmacEngine};
use bitcoin::secp256k1::ecdh::SharedSecret;
use bitcoin::secp256k1::hashes::sha256;
use bitcoin::secp256k1::{Message, PublicKey, Scalar, Secp256k1, SecretKey};
use rand_core::{OsRng, RngCore};
use tracing::debug;

const NODE_KEY_PATH: &str = "m/0";

#[derive(Copy, Clone, Debug)]
pub enum Error {
    Bip39Error(bip39::Error),
    Secp256k1Error(bitcoin::secp256k1::Error),
    KeyDerivationError,
    EntropyLengthError,
    KeyTweakError,
}

#[derive(Copy, Clone, Debug)]
pub enum Network {
    Bitcoin,
    Testnet,
    Regtest,
    Signet,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bip39Error(err) => write!(f, "Bip39 error {}", err),
            Self::Secp256k1Error(err) => write!(f, "Secp256k1 error {}", err),
            Self::KeyDerivationError => write!(f, "Key derivation error"),
            Self::EntropyLengthError => write!(f, "Entropy must be 32 bytes"),
            Self::KeyTweakError => write!(f, "Key tweak error"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone)]
pub struct Mnemonic {
    internal: bip39::Mnemonic,
}

impl Mnemonic {
    pub fn random() -> Result<Mnemonic, Error> {
        let internal = bip39::Mnemonic::generate(24).map_err(Error::Bip39Error)?;
        Ok(Self { internal })
    }

    pub fn from_entropy(entropy: Vec<u8>) -> Result<Mnemonic, Error> {
        let slice = entropy.as_slice();
        let array: [u8; 32] = slice.try_into().map_err(|_| Error::EntropyLengthError)?;
        let internal = bip39::Mnemonic::from_entropy(&array).map_err(Error::Bip39Error)?;
        Ok(Self { internal })
    }

    pub fn from_phrase(phrase: String) -> Result<Mnemonic, Error> {
        let internal =
            bip39::Mnemonic::parse_normalized(phrase.as_str()).map_err(Error::Bip39Error)?;
        Ok(Self { internal })
    }

    pub fn as_string(&self) -> String {
        self.internal.to_string()
    }
}

#[derive(Clone)]
pub struct Seed {
    seed: Vec<u8>,
}

impl Seed {
    pub fn from_mnemonic(mnemonic: &Mnemonic) -> Self {
        let seed = mnemonic.internal.to_seed("").to_vec();
        Self { seed }
    }

    pub fn new(seed: Vec<u8>) -> Self {
        Self { seed }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.seed.clone()
    }
}

#[derive(Clone)]
pub struct InvoiceSignature {
    signature: Vec<u8>,
    recovery_id: i32,
}

impl InvoiceSignature {
    pub fn get_signature(&self) -> Vec<u8> {
        self.signature.clone()
    }

    pub fn get_recovery_id(&self) -> i32 {
        self.recovery_id
    }
}

pub struct LightsparkSigner {
    master_private_key: ExtendedPrivKey,
    node_private_key: ExtendedPrivKey,
}

impl LightsparkSigner {
    pub fn new(seed: &Seed, network: Network) -> Result<LightsparkSigner, Error> {
        let network: bitcoin::Network = match network {
            Network::Bitcoin => bitcoin::Network::Bitcoin,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Regtest => bitcoin::Network::Regtest,
            Network::Signet => bitcoin::Network::Signet,
        };
        let master_private_key = ExtendedPrivKey::new_master(network, seed.as_bytes().as_slice())
            .map_err(|_| Error::KeyDerivationError)?;
        let secp = Secp256k1::new();
        let node_key_path =
            DerivationPath::from_str(NODE_KEY_PATH).map_err(|_| Error::KeyDerivationError)?;
        let node_private_key = master_private_key
            .derive_priv(&secp, &node_key_path)
            .map_err(|_| Error::KeyDerivationError)?;
        Ok(Self {
            master_private_key,
            node_private_key,
        })
    }

    pub fn from_bytes(seed: Vec<u8>, network: Network) -> Result<LightsparkSigner, Error> {
        let seed = Seed::new(seed);
        Self::new(&seed, network)
    }

    pub fn get_master_public_key(&self) -> Result<String, Error> {
        let secp = Secp256k1::new();
        let pubkey = ExtendedPubKey::from_priv(&secp, &self.master_private_key);
        Ok(pubkey.to_string())
    }

    pub fn derive_public_key(&self, derivation_path: String) -> Result<String, Error> {
        let secp = Secp256k1::new();
        let path =
            DerivationPath::from_str(&derivation_path).map_err(|_| Error::KeyDerivationError)?;
        let private_key = self
            .master_private_key
            .derive_priv(&secp, &path)
            .map_err(|_| Error::KeyDerivationError)?;
        let pubkey = ExtendedPubKey::from_priv(&secp, &private_key);
        Ok(pubkey.to_string())
    }

    pub fn derive_key_and_sign(
        &self,
        message: Vec<u8>,
        derivation_path: String,
        add_tweak: Option<Vec<u8>>,
        mul_tweak: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, Error> {
        let secp = Secp256k1::new();
        let signing_key =
            self.derive_and_tweak_key(derivation_path.clone(), add_tweak, mul_tweak)?;
        let msg = Message::from_slice(message.as_slice()).map_err(Error::Secp256k1Error)?;
        let signature = secp.sign_ecdsa(&msg, &signing_key);

        debug!("Derivation: {}", derivation_path);
        debug!("Signing Key: {}", hex::encode(signing_key.as_ref()));
        debug!(
            "Verification Key: {}",
            signing_key.public_key(&secp).to_string()
        );
        debug!("Message: {}", hex::encode(message.as_slice()));

        Ok(signature.serialize_compact().to_vec())
    }

    pub fn ecdh(&self, public_key: Vec<u8>) -> Result<Vec<u8>, Error> {
        let pubkey = PublicKey::from_slice(public_key.as_slice()).map_err(Error::Secp256k1Error)?;
        let our_key = self.node_private_key.private_key;
        let ss = SharedSecret::new(&pubkey, &our_key);
        Ok(ss.as_ref().to_vec())
    }

    pub fn get_per_commitment_point(
        &self,
        derivation_path: String,
        per_commitment_point_idx: u64,
    ) -> Result<Vec<u8>, Error> {
        let per_commitment_secret =
            self.release_per_commitment_secret(derivation_path, per_commitment_point_idx)?;
        let secret_key = SecretKey::from_slice(per_commitment_secret.as_slice())
            .map_err(Error::Secp256k1Error)?;
        let public_key = secret_key.public_key(&Secp256k1::new());
        Ok(public_key.serialize().to_vec())
    }

    pub fn release_per_commitment_secret(
        &self,
        derivation_path: String,
        per_commitment_point_idx: u64,
    ) -> Result<Vec<u8>, Error> {
        let key = self
            .derive_key(derivation_path)
            .map_err(|_| Error::KeyDerivationError)?;
        let channel_seed = sha256::Hash::hash(&key.private_key[..])
            .as_byte_array()
            .to_vec();
        let commitment_seed = self.build_commitment_seed(channel_seed);
        Ok(self.build_commitment_secret(commitment_seed, per_commitment_point_idx))
    }

    pub fn generate_preimage_nonce(&self) -> Vec<u8> {
        let mut rng = OsRng;
        let mut nonce = [0u8; 32];
        rng.fill_bytes(&mut nonce);
        nonce.to_vec()
    }

    pub fn generate_preimage(&self, nonce: Vec<u8>) -> Result<Vec<u8>, Error> {
        let key = self.derive_key("m/4h".to_owned())?;
        let mut hmac_engine: HmacEngine<sha512::Hash> =
            HmacEngine::new(&key.private_key.secret_bytes());
        hmac_engine.input(b"invoice preimage");
        hmac_engine.input(nonce.as_slice());
        let hmac_result: Hmac<sha512::Hash> = Hmac::from_engine(hmac_engine);
        Ok(hmac_result[..32].into())
    }

    pub fn generate_preimage_hash(&self, nonce: Vec<u8>) -> Result<Vec<u8>, Error> {
        let preimage = self.generate_preimage(nonce)?;
        Ok(sha256::Hash::hash(preimage.as_slice())
            .as_byte_array()
            .to_vec())
    }

    fn derive_and_tweak_key(
        &self,
        derivation_path: String,
        add_tweak: Option<Vec<u8>>,
        mul_tweak: Option<Vec<u8>>,
    ) -> Result<SecretKey, Error> {
        let derived_key = self
            .derive_key(derivation_path)
            .map_err(|_| Error::KeyDerivationError)?;
        let add_tweak: Option<[u8; 32]> = add_tweak
            .filter(|tweak| !tweak.is_empty())
            .map(|tweak| tweak.try_into().map_err(|_| Error::KeyTweakError))
            .transpose()?;
        let mul_tweak: Option<[u8; 32]> = mul_tweak
            .filter(|tweak| !tweak.is_empty())
            .map(|tweak| tweak.try_into().map_err(|_| Error::KeyTweakError))
            .transpose()?;
        self.tweak_key(derived_key.private_key, add_tweak, mul_tweak)
    }

    fn derive_key(&self, derivation_path: String) -> Result<ExtendedPrivKey, Error> {
        let secp = Secp256k1::new();
        let path =
            DerivationPath::from_str(&derivation_path).map_err(|_| Error::KeyDerivationError)?;
        let private_key = self
            .master_private_key
            .derive_priv(&secp, &path)
            .map_err(|_| Error::KeyDerivationError)?;
        Ok(private_key)
    }

    fn build_commitment_seed(&self, seed: Vec<u8>) -> Vec<u8> {
        let mut hasher = sha256::Hash::engine();
        hasher.input(seed.as_slice());
        hasher.input(&b"commitment seed"[..]);
        sha256::Hash::from_engine(hasher).to_byte_array().to_vec()
    }

    fn build_commitment_secret(&self, seed: Vec<u8>, idx: u64) -> Vec<u8> {
        let mut res = seed;
        for i in 0..48 {
            let bitpos = 47 - i;
            if idx & (1 << bitpos) == (1 << bitpos) {
                res[bitpos / 8] ^= 1 << (bitpos & 7);
                res = sha256::Hash::hash(&res).to_byte_array().to_vec();
            }
        }
        res
    }

    fn tweak_key(
        &self,
        secret_key: SecretKey,
        add_tweak: Option<[u8; 32]>,
        mul_tweak: Option<[u8; 32]>,
    ) -> Result<SecretKey, Error> {
        let mut res: SecretKey = secret_key;
        if let Some(mul_tweak) = mul_tweak {
            let scalar = Scalar::from_be_bytes(mul_tweak).map_err(|_| Error::KeyTweakError)?;
            res = res.mul_tweak(&scalar).map_err(Error::Secp256k1Error)?;
        }

        if let Some(add_tweak) = add_tweak {
            let scalar = Scalar::from_be_bytes(add_tweak).map_err(|_| Error::KeyTweakError)?;
            res = res.add_tweak(&scalar).map_err(Error::Secp256k1Error)?;
        }

        Ok(res)
    }

    pub fn sign_invoice(&self, unsigned_invoice: String) -> Result<InvoiceSignature, Error> {
        let signing_key = self.node_private_key.private_key;
        let msg = Message::from_hashed_data::<sha256::Hash>(unsigned_invoice.as_bytes());
        let secp = Secp256k1::new();
        let sig = secp
            .sign_ecdsa_recoverable(&msg, &signing_key)
            .serialize_compact();
        let res = InvoiceSignature {
            signature: sig.1.to_vec(),
            recovery_id: sig.0.to_i32(),
        };
        Ok(res)
    }

    pub fn sign_invoice_hash(&self, invoice_hash: Vec<u8>) -> Result<InvoiceSignature, Error> {
        let signing_key = self.node_private_key.private_key;
        let msg = Message::from_slice(invoice_hash.as_slice()).map_err(Error::Secp256k1Error)?;
        let secp = Secp256k1::new();
        let sig = secp
            .sign_ecdsa_recoverable(&msg, &signing_key)
            .serialize_compact();
        let res = InvoiceSignature {
            signature: sig.1.to_vec(),
            recovery_id: sig.0.to_i32(),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::ecdsa::Signature;
    use hex;

    #[test]
    fn test_key_derivation() {
        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let xprv = signer.derive_key("m".to_owned()).unwrap();
        let xprv_string = xprv.to_string();
        let expected_string = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";
        assert_eq!(xprv_string.as_str(), expected_string);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let xprv = signer.derive_key("m/0'".to_owned()).unwrap();
        let xprv_string = xprv.to_string();
        let expected_string = "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7";
        assert_eq!(xprv_string.as_str(), expected_string);
    }

    #[test]
    fn test_public_key() {
        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let public_key_string = signer.get_master_public_key().unwrap();
        let expected_string = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
        assert_eq!(public_key_string, expected_string);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let public_key_string = signer.derive_public_key("m/0'".to_owned()).unwrap();
        let expected_string = "xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw";
        assert_eq!(public_key_string, expected_string);
    }

    #[test]
    fn test_sign() {
        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);

        let public_key_string = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let xpub = signer.derive_public_key("m".to_owned()).unwrap();
        assert_eq!(xpub, public_key_string);

        let verification_key = ExtendedPubKey::from_str(public_key_string)
            .unwrap()
            .public_key;

        let message = b"Hello, world!";
        let hash = sha256::Hash::hash(message);
        let signature_bytes = signer
            .derive_key_and_sign(hash.to_byte_array().to_vec(), "m".to_owned(), None, None)
            .unwrap();
        let signature = Signature::from_compact(signature_bytes.as_slice()).unwrap();
        let msg = Message::from_hashed_data::<sha256::Hash>(message);
        let secp = Secp256k1::new();
        assert!(secp
            .verify_ecdsa(&msg, &signature, &verification_key)
            .is_ok());
    }

    #[test]
    fn test_ecdh() {
        let seed1_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed1_bytes = hex::decode(seed1_hex_string).unwrap();
        let seed1 = Seed::new(seed1_bytes);

        let seed2_hex_string = "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542";
        let seed2_bytes = hex::decode(seed2_hex_string).unwrap();
        let seed2 = Seed::new(seed2_bytes);

        let signer1 = LightsparkSigner::new(&seed1, Network::Bitcoin).unwrap();
        let pub1 = signer1.derive_public_key("m/0".to_owned()).unwrap();
        let xpub1 = ExtendedPubKey::from_str(&pub1).unwrap();
        let pub1_bytes = xpub1.public_key.serialize();

        let signer2 = LightsparkSigner::new(&seed2, Network::Bitcoin).unwrap();
        let pub2 = signer2.derive_public_key("m/0".to_owned()).unwrap();
        let xpub2 = ExtendedPubKey::from_str(&pub2).unwrap();
        let pub2_bytes = xpub2.public_key.serialize();

        let secret_1 = signer1.ecdh(pub2_bytes.to_vec()).unwrap();
        let secret_2 = signer2.ecdh(pub1_bytes.to_vec()).unwrap();
        assert_eq!(secret_1, secret_2);
    }

    #[test]
    fn test_tweak() {
        let base_hex_string = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        let base_bytes = hex::decode(base_hex_string).unwrap();
        let secrect_key = SecretKey::from_slice(base_bytes.as_slice()).unwrap();

        let mul_tweak = "efbf7ba5a074276701798376950a64a90f698997cce0dff4d24a6d2785d20963";
        let mul_tweak_bytes = hex::decode(mul_tweak).unwrap();

        let add_tweak = "8be02a96a97b9a3c1c9f59ebb718401128b72ec009d85ee1656319b52319b8ce";
        let add_tweak_bytes = hex::decode(add_tweak).unwrap();

        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let key = signer
            .tweak_key(
                secrect_key,
                Some(add_tweak_bytes.try_into().unwrap()),
                Some(mul_tweak_bytes.try_into().unwrap()),
            )
            .unwrap();

        let result_hex = "d09ffff62ddb2297ab000cc85bcb4283fdeb6aa052affbc9dddcf33b61078110";
        assert_eq!(format!("{}", key.display_secret()), result_hex);
    }

    #[test]
    fn test_preimage() {
        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let nonce = signer.generate_preimage_nonce();
        let preimage = signer.generate_preimage(nonce.clone());
        let preimage_hash = sha256::Hash::hash(preimage.unwrap().as_slice())
            .as_byte_array()
            .to_vec();
        let preimage_hash2 = signer.generate_preimage_hash(nonce).unwrap();
        assert_eq!(preimage_hash, preimage_hash2);
    }

    #[test]
    fn test_preimage_with_vectors() {
        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        assert_eq!(
            hex::encode(signer.generate_preimage([0u8; 32].to_vec()).unwrap()),
            "ceb7494bb4dc84e5963a151f26faa2e759379aeb7b8cc9b02cf9753202d39381"
        );

        assert_eq!(
            hex::encode(signer.generate_preimage([1u8; 32].to_vec()).unwrap()),
            "d9a850ee1be830b3af70e88ce8085b5d23a24ca8b1dcb9164a4716f6a8771a85"
        );
    }

    #[test]
    fn test_commitment() {
        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);

        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();
        let commitment_point = signer
            .get_per_commitment_point("m/3/2104864975".to_owned(), 281474976710654)
            .unwrap();
        let commitment_secret = signer
            .release_per_commitment_secret("m/3/2104864975".to_owned(), 281474976710654)
            .unwrap();

        let secret_key = SecretKey::from_slice(commitment_secret.as_slice()).unwrap();
        let public_key = secret_key.public_key(&Secp256k1::new()).serialize();
        assert_eq!(commitment_point, public_key);
    }

    #[test]
    fn test_derive_and_sign() {
        let msg = hex::decode("476bdd1db5d91897d00d75300eef50c0da7e0b2dada06dde93cbb5903b7e16b2")
            .unwrap();
        let seed_hex_string = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed_hex_string).unwrap();
        let seed = Seed::new(seed_bytes);
        let signer = LightsparkSigner::new(&seed, Network::Bitcoin).unwrap();

        let signature_bytes = signer
            .derive_key_and_sign(msg.clone(), "m/3/2106220917/0".to_owned(), None, None)
            .unwrap();
        let pubkey = signer
            .derive_public_key("m/3/2106220917/0".to_owned())
            .unwrap();
        let verification_key = ExtendedPubKey::from_str(&pubkey).unwrap().public_key;

        let msg = Message::from_slice(&msg).unwrap();
        let signature = Signature::from_compact(signature_bytes.as_slice()).unwrap();
        let secp = Secp256k1::new();
        assert!(secp
            .verify_ecdsa(&msg, &signature, &verification_key)
            .is_ok());
    }
}
