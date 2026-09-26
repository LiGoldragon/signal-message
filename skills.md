# skills — signal-message

Read `ARCHITECTURE.md` before editing this repository.

`ethos/signal.ethos` is the sole authored source. Regenerate
`src/generated/signal.rs` with `ethos-zero 'Generate.{ <abs ethos> <abs src/generated> }'`;
`build.rs` refuses a stale projection.

Every record kind gets a concrete datom round trip in
`tests/generated_contract.rs` before its type is final. The complete gate is
`nix flake check`.
