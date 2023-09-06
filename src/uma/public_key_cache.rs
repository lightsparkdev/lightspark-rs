use super::protocol::PubKeyResponse;
use std::collections::HashMap;

pub trait PublicKeyCache {
    fn fetch_public_key_for_vasp(&self, vasp_domain: &str) -> Option<&PubKeyResponse>;
    fn add_public_key_for_vasp(&mut self, vasp_domain: &str, public_key: &PubKeyResponse);
    fn remove_public_key_for_vasp(&mut self, vasp_domain: &str);
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
