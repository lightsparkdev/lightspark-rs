// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

#[cfg(test)]
mod tests {
    use ecies::utils::generate_keypair;

    use crate::uma::{
        get_lnurlp_response, get_pay_req_response, get_pay_request, get_signed_lnurlp_request_url,
        is_uma_lnurl_query, parse_lnurlp_request, parse_lnurlp_response, parse_pay_req_response,
        parse_pay_request, verify_pay_req_signature, verify_uma_lnurl_query_signature,
        verify_uma_lnurlp_response_signature, UmaInvoiceCreator,
    };

    use crate::{currency::Currency, payer_data::PayerDataOptions, protocol::LnurlpRequest};

    #[test]
    fn test_parse() {
        let timestamp = chrono::Utc::now().timestamp();
        let expected_query = LnurlpRequest {
            receiver_address: "bob@vasp2.com".to_string(),
            nonce: "12345".to_string(),
            signature: "signature".to_string(),
            is_subject_to_travel_rule: true,
            vasp_domain: "vasp1.com".to_string(),
            timestamp,
            uma_version: "0.1".to_string(),
        };

        let url_string = format!("https://vasp2.com/.well-known/lnurlp/bob?signature=signature&nonce=12345&vaspDomain=vasp1.com&umaVersion=0.1&isSubjectToTravelRule=true&timestamp={}",&timestamp);
        let url = url::Url::parse(&url_string).unwrap();

        let query = parse_lnurlp_request(&url).unwrap();
        assert_eq!(query, expected_query);
    }

    #[test]
    fn test_is_uma_query_valid() {
        let url_string = "https://vasp2.com/.well-known/lnurlp/bob?signature=signature&nonce=12345&vaspDomain=vasp1.com&umaVersion=0.1&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(is_uma_lnurl_query(&url));
    }

    #[test]
    fn test_is_uma_query_missing_params() {
        let url_string = "https://vasp2.com/.well-known/lnurlp/bob?nonce=12345&vaspDomain=vasp1.com&umaVersion=0.1&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/.well-known/lnurlp/bob?signature=signature&vaspDomain=vasp1.com&umaVersion=0.1&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/.well-known/lnurlp/bob?signature=signature&nonce=12345&umaVersion=0.1&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/.well-known/lnurlp/bob?signature=signature&nonce=12345&umaVersion=0.1&vaspDomain=vasp1.com&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        // isSubjectToTravelRule is optional
        assert!(is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/.well-known/lnurlp/bob?signature=signature&nonce=12345&umaVersion=0.1&vaspDomain=vasp1.com&isSubjectToTravelRule=true";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/.well-known/lnurlp/bob";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/.well-known/lnurlp/bob?signature=signature&nonce=12345&vaspDomain=vasp1.com&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));
    }

    #[test]
    fn test_is_uma_query_invalid_path() {
        let url_string = "https://vasp2.com/.well-known/lnurla/bob?signature=signature&nonce=12345&vaspDomain=vasp1.com&umaVersion=0.1&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/bob?signature=signature&nonce=12345&vaspDomain=vasp1.com&umaVersion=0.1&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));

        let url_string = "https://vasp2.com/?signature=signature&nonce=12345&vaspDomain=vasp1.com&umaVersion=0.1&isSubjectToTravelRule=true&timestamp=12345678";
        let url = url::Url::parse(url_string).unwrap();
        assert!(!is_uma_lnurl_query(&url));
    }

    #[test]
    fn test_sign_and_verify_lnurlp_request() {
        let (sk, pk) = generate_keypair();

        let query_url = get_signed_lnurlp_request_url(
            &sk.serialize(),
            "$bob@vasp2.com",
            "vasp1.com",
            true,
            None,
        )
        .unwrap();

        let query = parse_lnurlp_request(&query_url).unwrap();

        let result = verify_uma_lnurl_query_signature(query, &pk.serialize());
        assert!(result.is_ok());
    }

    #[test]
    fn test_sign_and_verify_lnurlp_request_invalid_signature() {
        let (sk, _) = generate_keypair();

        let query_url = get_signed_lnurlp_request_url(
            &sk.serialize(),
            "$bob@vasp2.com",
            "vasp1.com",
            true,
            None,
        )
        .unwrap();

        let query = parse_lnurlp_request(&query_url).unwrap();

        let (_, pk) = generate_keypair();
        let result = verify_uma_lnurl_query_signature(query, &pk.serialize());
        assert!(result.is_err());
    }

    #[test]
    fn test_sign_and_verify_lnurlp_response() {
        let (sk1, _) = generate_keypair();
        let (sk2, pk2) = generate_keypair();

        let request = get_signed_lnurlp_request_url(
            &sk1.serialize(),
            "$bob@vasp2.com",
            "vasp1.com",
            true,
            None,
        )
        .unwrap();
        let query = parse_lnurlp_request(&request).unwrap();

        let metadata = create_metadata_for_bob().unwrap();

        let currency_options = [Currency {
            code: "USD".to_string(),
            name: "US Doller".to_string(),
            symbol: "$".to_string(),
            millisatoshi_per_unit: 34150,
            min_sendable: 1,
            max_sendable: 10000000,
        }];

        let response = get_lnurlp_response(
            &query,
            &sk2.serialize(),
            true,
            "https://vasp2.com/api/lnurl/payreq/$bob",
            metadata.as_str(),
            1,
            10_000_000,
            &PayerDataOptions {
                name_required: false,
                email_required: false,
                compliance_required: true,
            },
            &currency_options,
            crate::kyc_status::KycStatus::KycStatusVerified,
        )
        .unwrap();

        let response_json = serde_json::to_vec(&response).unwrap();
        let response = parse_lnurlp_response(&response_json).unwrap();

        let result = verify_uma_lnurlp_response_signature(response, &pk2.serialize());
        assert!(result.is_ok());
    }

    #[test]
    fn test_pay_req_creation_and_parsing() {
        let (sk1, pk1) = generate_keypair();
        let (sk2, pk2) = generate_keypair();

        let payreq = get_pay_request(
            &pk1.serialize(),
            &sk2.serialize(),
            "USD",
            1000,
            "$alice@vasp1.com",
            None,
            None,
            Some("some TR info for VASP2"),
            crate::kyc_status::KycStatus::KycStatusVerified,
            &[],
            None,
            "/api/lnurl/utxocallback?txid=1234",
        )
        .unwrap();

        let payreq_json = serde_json::to_vec(&payreq).unwrap();

        let payreq = parse_pay_request(&payreq_json).unwrap();

        let result = verify_pay_req_signature(&payreq, &pk2.serialize());
        assert!(result.is_ok());

        let cipher_text = hex::decode(
            payreq
                .payer_data
                .compliance
                .encrypted_travel_rule_info
                .unwrap(),
        )
        .unwrap();
        let plain_text = ecies::decrypt(&sk1.serialize(), &cipher_text).unwrap();
        assert_eq!(plain_text, b"some TR info for VASP2");
    }

    #[test]
    fn test_pay_req_response_and_parsing() {
        let (_, pk1) = generate_keypair();
        let (sk2, _) = generate_keypair();

        let payreq = get_pay_request(
            &pk1.serialize(),
            &sk2.serialize(),
            "USD",
            1000,
            "$alice@vasp1.com",
            None,
            None,
            Some("some TR info for VASP2"),
            crate::kyc_status::KycStatus::KycStatusVerified,
            &[],
            None,
            "/api/lnurl/utxocallback?txid=1234",
        )
        .unwrap();

        let client = FakeInvoiceCreator {};

        let metadata = create_metadata_for_bob().unwrap();

        let response = get_pay_req_response(
            &payreq,
            &client,
            &metadata,
            "USD",
            34150,
            100_000,
            &["abcdef12345".to_owned()],
            None,
            "/api/lnurl/utxocallback?txid=1234",
        )
        .unwrap();

        let response_json = serde_json::to_vec(&response).unwrap();

        let result = parse_pay_req_response(&response_json);
        assert!(result.is_ok());
    }

    struct FakeInvoiceCreator {}

    impl UmaInvoiceCreator for FakeInvoiceCreator {
        fn create_uma_invoice(
            &self,
            _amount_msat: i64,
            _metadata: &str,
        ) -> Result<String, Box<dyn std::error::Error>> {
            Ok("lntb100n1p0z9j".to_owned())
        }
    }

    fn create_metadata_for_bob() -> Result<String, serde_json::Error> {
        let metadata = vec![
            vec!["text/plain", "Pay to vasp2.com user $bob"],
            vec!["text/identifier", "$bob@vasp2.com"],
        ];

        let json_metadata = serde_json::to_string(&metadata)?;
        Ok(json_metadata)
    }
}
