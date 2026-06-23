# skills — signal-message

*Per-repo agent guide.*

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md` — contract-repo
  discipline (what goes here vs. doesn't)
- `~/primary/skills/architecture-editor.md` — `ARCHITECTURE.md`
  conventions
- `~/primary/skills/architectural-truth-tests.md` — every
  contract change needs a witness test
- `~/primary/skills/nix-discipline.md` — flake-input rules,
  `nix flake check` is the gate
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`message/`, `router/`)

If your change adds a new request or reply variant, edit
`schema/lib.schema`, regenerate `src/schema/lib.rs`, then update the
methods/tests and consumers.

## What this repo owns

- The closed `Input` enum (`Submit`, `SubmitStamped`,
  `QueryInbox`) for message ingress and router ingress.
- The closed `Output` enum (the responses the router
  sends back).
- The generated `Frame` type emitted by the schema-rust
  WireContract target over `signal-frame`.
- The wire-form round-trip tests in `tests/round_trip.rs`.

## What this repo does not own

- The CLI itself — that's `message`.
- The router — that's `router`.
- Transport (UDS path, reconnect, timeouts) — those are per
  consumer.
- Routing / delivery logic — that's `router`'s
  internal state machine.
- Persistence policy — that's owned by `router` and implemented
  through its own state actor and router-owned Sema layer.
