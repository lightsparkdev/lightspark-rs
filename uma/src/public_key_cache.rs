// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use super::protocol::PubKeyResponse;
use std::collections::HashMap;

/// PublicKeyCache is an interface for a cache of public keys for other VASPs.
///
/// Implementations of this interface should be thread-safe.
pub trait PublicKeyCache {
    /// fetch_public_key_for_vasp fetches the public key entry for a VASP if in the cache, otherwise
    /// returns nil.
    fn fetch_public_key_for_vasp(&self, vasp_domain: &str) -> Option<&PubKeyResponse>;

    /// add_public_key_for_vasp adds a public key entry for a VASP to the cache.
    fn add_public_key_for_vasp(&mut self, vasp_domain: &str, public_key: &PubKeyResponse);

    /// remove_public_key_for_vasp removes a public key for a VASP from the cache.
    fn remove_public_key_for_vasp(&mut self, vasp_domain: &str);

    /// clear clears the cache.
    fn clear(&mut self);
}

pub struct InMemoryPublicKeyCache {
    cache: HashMap<String, PubKeyResponse>,
}

impl Default for InMemoryPublicKeyCache {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryPublicKeyCache {
    pub fn new() -> InMemoryPublicKeyCache {
        InMemoryPublicKeyCache {
            cache: HashMap::new(),
        }
    }
}

impl PublicKeyCache for InMemoryPublicKeyCache {
    fn fetch_public_key_for_vasp(&self, vasp_domain: &str) -> Option<&PubKeyResponse> {
        self.cache.get(vasp_domain)
    }

    fn add_public_key_for_vasp(&mut self, vasp_domain: &str, public_key: &PubKeyResponse) {
        self.cache
            .insert(vasp_domain.to_string(), public_key.clone());
    }

    fn remove_public_key_for_vasp(&mut self, vasp_domain: &str) {
        self.cache.remove(vasp_domain);
    }

    fn clear(&mut self) {
        self.cache.clear();
    }
}
