# Changelog

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
