# signal-message

The ordinary Message Nexus Signal contract. A message is just a message:
`Send.{ [ recipients ] Priority Content }`. The sender is never in the
payload; Message names it from the connection's peer through Flow.

Requests: `Send`, `Withdraw` (the sender, while parked), `Acknowledge` (a
recipient; the only source of Read), `QueryReceipts`, `Observe` (receipts on
open, then each grade change).

`Content`, `BodyRefusal`, `DeliveryRejection` and `InterruptWitness` are
imported from `meta-signal-flow`: Flow is the only pane writer, and it renders
and refuses exactly these values.

`ethos/signal.ethos` is the authored source; `src/generated/signal.rs` is its
committed projection, held fresh by `build.rs`. Every record kind has a
concrete datom round trip in `tests/generated_contract.rs`.

Run `nix flake check --print-build-logs` for the complete proof matrix.
