# Changelog

# v0.9.1
- Fix a signing issue with RSA keys.

# v0.9.0
- Update API objects.

# v0.8.1
- Remove openssl dependency from reqwest.

# v0.8.0

- Add a function for cancelling unpaid invoices.
- Add UMA invites support.

# v0.7.2
- Add preimage to OutgoingPayment.

# v0.7.1
- Add UMA related operations in LightsparkClient.

# v0.7.0
- Refine requester.
- Separate crate into features.

# v0.6.4
- Remove all openssl dependencies.

# v0.6.3
- Replace openssl with native rust crypto lib.

# v0.6.1
- Fix a minor bug in `create_test_mode_payment`.

# v0.6.0
- Make UMA protocol a sepearte crate. See https://crates.io/crates/uma
- Upgrade to new graphql endpoint.

# v0.5.0
- Add UMA support.

# v0.4.0
- Add webhook handling.
- Change the interface wrapper to enum.
- Updated graphql schema.

# v0.3.0
- Add test mode operations: `create_test_mode_invoice` and `create_test_mode_payment`

# v0.2.0

- Adding the ability to manage wallets tied to the current account. See `Account.get_wallets()` and the `Wallet` object.

# v0.1.0

First draft of the SDK.
