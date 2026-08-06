# ARCHITECTURE — signal-message

`signal-message` owns the ordinary Interface vocabulary shared by two Message
relations:

- a client or component submits an unstamped message to the Message receiver;
- Message supplies the accepted-ingress origin and timestamp, then submits the
  stamped message to Router.

The Interface is self-contained. Its message, origin, agent-registry, thread,
configuration, acceptance, and rejection Types are local relation vocabulary.
Consumers project them into their own runtime state at the boundary.

## Authority and projection

`ethos/interface.ethos` is a role-free `Interface.{1 0 0}` document. The
producer-owned bootstrap manifest records the authority identity, grammar
seats, canonical order, and opaque declaration identities. `build.rs`
assembles the text with that authority state and asks schema-rust 0.15 for the
strict Rust Logos projection. Freshness is checked on every build.

The generated artifact defines encoded Types only. Human spellings remain
textual metadata used by Dotos. Rust consumers use the encoded coordinates,
which makes identity independent from a current spelling, source position, or
implementation substrate.

## Current behavior slice

Logos does not yet express the operational slice required by this contract.
`src/schema/lib/behavior.rs` therefore seats structural archive and Dotos
behavior by hand and defines the `Input` and `Output` roles explicitly. This is
canonical bootstrap behavior, not a second Type vocabulary.

The request role has nine ordered routes and the reply role has thirteen. Both
are carried by the allocated Signal frame contract at wire revision 2. Every
route is witnessed through frame bytes; representative Dotos witnesses ensure
that human-facing role names remain legible while Rust Type names stay strict.

## Boundaries

This repository owns Type identity, role legality, Dotos structure, archive
shape, and the Signal frame binding. It owns no actors, sockets, daemon state,
routing policy, persistence policy, process supervision, or transport retry.

The orchestrator mints agent identity. Message records the durable consumer
view and binds live endpoints. A thread is a sender-chosen name with an
optional repository/feature relation; its participants arise from traffic and
explicit subscription. Message origin is derived from the accepted ingress,
never trusted from an ordinary client payload.

## Code map

```text
ethos/interface.ethos             authored Interface text
src/bootstrap_manifest.rs         producer-owned authority seats
build.rs                          verified assembly and strict projection
src/schema/lib/generated.rs       checked encoded Type projection
src/schema/lib/behavior.rs        handwritten bootstrap behavior and roles
tests/interface_contract.rs       role-free source and strictness witnesses
tests/message_roles.rs            role seating and order witnesses
tests/round_trip.rs               byte and Dotos witnesses
tests/dependency_boundary.rs      build/runtime boundary and source fence
```
