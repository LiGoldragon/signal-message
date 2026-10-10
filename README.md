# signal-message

The ordinary Message Signal carries a typed request to one Flow metaflow.
`Send.{ Recipient Request }` accepts Flow's `Address` or `Up` recipient and
its typed request. Flow owns recipient resolution, queuing and wake behavior.

`Configure` carries the shared socket configuration until Meta configuration
has been accepted. Flow's request and refusal types are imported by identity.

`ethos/signal.ethos` is authored; the build checks its generated Rust.
`tests/generated_contract.rs` covers archive and datom forms.
