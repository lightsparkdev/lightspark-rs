// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use lightspark_remote_signing::signer::{LightsparkSigner, Mnemonic, Seed};

/// Utility to generate a new mnemonic, seed, and master public key.
fn main() {
    let mnemonic = Mnemonic::random().unwrap();
    println!("{}", mnemonic.as_string());
    println!();

    let seed = Seed::from_mnemonic(&mnemonic);
    println!("{}", hex::encode(seed.as_bytes()));
    println!();

    let signer =
        LightsparkSigner::new(&seed, lightspark_remote_signing::signer::Network::Bitcoin).unwrap();
    let pubkey = signer.get_master_public_key().unwrap();
    println!("{}", pubkey);
}
