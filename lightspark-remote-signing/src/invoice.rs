use crate::Error;

pub trait PaymentPreimageProvider: Send + Sync {
    fn release_preimage(&self, invoice_id: &str) -> Result<String, Error>;
    fn get_payment_hash(&self, invoice_id: &str) -> Result<String, Error>;
}
