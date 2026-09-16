# Notify Datom remote pass

The dedicated `test-notify-datom` Nix check for
`a050d7a18dbf16711a5bd2f4a6d9c734ef4d1919` completed on the configured
Prometheus remote builder with exit code 0.

The exact command ran `cargo test --release --locked --features datom --test
notify_datom`. Both named tests passed: one verifies producer Datom round-trip
with escaped text, and one verifies wrong-type, argument-count, recipient,
input, Unicode body-size, and trailing-text rejection. No delivery, account,
or live XMPP transport is part of this proposal.
