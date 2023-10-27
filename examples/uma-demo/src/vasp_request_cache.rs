use std::collections::HashMap;

use lightspark::objects::invoice_data::InvoiceData;
use uma::protocol::LnurlpResponse;

#[derive(Clone)]
pub struct Vasp1InitialRequestData {
    pub lnurl_response: LnurlpResponse,
    pub receiver_id: String,
    pub vasp2_domain: String,
}

#[derive(Clone)]
pub struct Vasp1PayReqData {
    pub encoded_invoice: String,
    pub utxo_callback: String,
    pub invoice_data: InvoiceData,
}

#[derive(Clone)]
pub struct Vasp1PayReqCache {
    pub uma_request_cache: HashMap<String, Vasp1InitialRequestData>,
    pub pay_req_cache: HashMap<String, Vasp1PayReqData>,
}

impl Default for Vasp1PayReqCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Vasp1PayReqCache {
    pub fn new() -> Self {
        Vasp1PayReqCache {
            uma_request_cache: HashMap::new(),
            pay_req_cache: HashMap::new(),
        }
    }

    pub fn get_lnurlp_response_data(&self, key: &str) -> Option<Vasp1InitialRequestData> {
        self.uma_request_cache.get(key).cloned()
    }

    pub fn save_lnurlp_response_data(
        &mut self,
        lnurlp_response: &LnurlpResponse,
        receiver_id: &str,
        vasp2_domain: &str,
    ) -> String {
        let uuid = uuid::Uuid::new_v4().to_string();
        let data = Vasp1InitialRequestData {
            lnurl_response: lnurlp_response.clone(),
            receiver_id: receiver_id.to_string(),
            vasp2_domain: vasp2_domain.to_string(),
        };
        self.uma_request_cache.insert(uuid.clone(), data);
        uuid
    }

    pub fn delete_lnurlp_response_data(&mut self, key: &str) {
        self.uma_request_cache.remove(key);
    }

    pub fn get_pay_req_data(&self, key: &str) -> Option<Vasp1PayReqData> {
        self.pay_req_cache.get(key).cloned()
    }

    pub fn save_pay_req_data(
        &mut self,
        request_uuid: &str,
        encoded_invoice: &str,
        utxo_callback: &str,
        invoice_data: &InvoiceData,
    ) -> String {
        let data = Vasp1PayReqData {
            encoded_invoice: encoded_invoice.to_string(),
            utxo_callback: utxo_callback.to_string(),
            invoice_data: invoice_data.clone(),
        };
        self.pay_req_cache.insert(request_uuid.to_string(), data);
        request_uuid.to_string()
    }

    pub fn delete_pay_req_data(&mut self, key: &str) {
        self.pay_req_cache.remove(key);
    }
}
