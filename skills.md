# skills — signal-persona-message

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
  (`persona-message/`, `persona-router/`)

If your change adds a new request or reply variant, edit
`src/lib.rs` first, then push, then update the consumers.

## What this repo owns

- The closed `MessageRequest` enum (the messages
  `message-cli` sends to `persona-router`).
- The closed `MessageReply` enum (the responses the router
  sends back).
- The `Frame` type alias = `signal_core::Frame<MessageRequest, MessageReply>`.
- The wire-form round-trip tests in `tests/round_trip.rs`.

## What this repo does not own

- The CLI itself — that's `persona-message`.
- The router — that's `persona-router`.
- Transport (UDS path, reconnect, timeouts) — those are per
  consumer.
- Routing / delivery logic — that's `persona-router`'s
  internal state machine.
- Persistence policy — that's owned by `persona-router` and implemented
  through its own state actor and router-owned Sema layer.
