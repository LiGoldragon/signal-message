# INTENT — signal-message

*The wire vocabulary contract for the engine's message-ingress path. Defines the
typed request/reply channel for the client-message relation (`message` CLI to
`message-daemon`) and the router-ingress relation (`message-daemon` to
`router`), carried in one root family.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-message` contract.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `message/INTENT.md`.

## Why this repo exists

`signal-message` is the **ordinary peer-callable wire contract** for message
ingress. It carries two named relations sharing one root family
(`MessageRequest` / `MessageReply`): the client-message relation, where a
`message` CLI or component client submits a message, and the router-ingress
relation, where the message daemon forwards a stamped submission to
`router`. Routing policy, delivery state, channel authority, and the
ingress daemon's runtime all live elsewhere (`router`, `message`); this crate is
the typed vocabulary the two relations speak.

## The channel shape

The MessageChannel carries:

- **Requests:** `Submit(MessageSubmission)` (a client submits a message for
  routing), `SubmitStamped(StampedMessageSubmission)` (the daemon forwards a
  provenance-stamped submission to the router), `QueryInbox(InboxQuery)` (read
  inbox state).
- **Replies:** `SubmissionAccepted`, `SubmissionRejected`, `InboxListing`, and
  `MessageRequestUnimplemented` (skeleton honesty for accepted-but-unimplemented
  shapes).

The wire vocabulary is contract-local: the daemon lowers these public operations
into component-local commands; Sema classification happens at observation time,
not on the wire.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch; unimplemented paths reply
  `MessageRequestUnimplemented`.
- Request payloads do not mint owner identity, origin, ingress timestamp, or
  sequence — provenance is the daemon's to stamp.
- The `message` daemon stamps owner identity, SO_PEERCRED-derived origin, and
  ingress time at the boundary; `StampedMessageSubmission` carries that minted
  provenance forward to the router. Caller identity is never accepted from the
  CLI or model payload.
- No stringly-typed dispatch. Message kinds, recipients, and rejection reasons
  are typed closed enums.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use contract-local
operation verbs":

- Operation roots are domain verbs in verb form: `Submit` (the client submits a
  message), `SubmitStamped` (router-side stamped ingress), `QueryInbox` (inbox
  read) — not Sema class words. `Submit` here means "submit a message for
  routing"; the same verb in another contract carries that contract's domain
  meaning (contract-locality).
- Reply success variants are past-tense / outcome-named matching the operation;
  rejections are typed (`SubmissionRejected`) carrying a closed-enum reason.
- Payload record names are domain nouns the operation carries
  (`MessageSubmission`, `StampedMessageSubmission`, `InboxQuery`), not `Request`,
  `Data`, or generic containers.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses.
- No runtime code: no actors, no tokio, no socket binding, no redb, no routing
  or delivery logic.
- No durable message ledger here — both the CLI and the daemon are stateless
  boundary surfaces; routing policy and delivery state stay in `router`.
- Contract types derive NOTA in this crate. Consumers do not carry shadow types.
- Every operation and reply variant round-trips through both rkyv frames and
  NOTA text.
- The two relations share one root family but address two different sockets
  (`message.sock` for clients, `router.sock` for router ingress); the contract
  names that split, not the socket binding.

## Three-layer model

Layer 1 (this crate): contract operations on the wire (`Submit`,
`SubmitStamped`, `QueryInbox`).
Layer 2 (daemon): component-local `MessageCommand` enum (e.g. `RecordSubmission`,
`StampOrigin`, `ReadInbox`) that the daemon executes.
Layer 3 (observation): payloadless Sema class labels for cross-component
introspection.

The contract names the public action at the boundary; the daemon decides what
internal work and Sema class label each action maps to. Sema classification
never appears on the wire.

## Code map

```text
src/lib.rs                       — Message/StampedMessage/InboxQuery records, NOTA codecs, signal_channel! invocation
schema/signal-message.concept.schema — concept-schema source for the contract
tests/round_trip.rs              — rkyv frame and NOTA round-trip witnesses per operation
```

## Non-ownership

This crate does not own:

- `message` CLI/daemon runtime, actors, or component lifecycle;
- any message ledger, redb store, or storage tables;
- socket binding, transport, provenance stamping execution, or version handshake;
- routing policy, delivery state, or channel authority (those live in `router`);
- NOTA projection policy or surface (CLI formatting, audit wrapping).

## See also

- `ARCHITECTURE.md` — detailed two-relation channel shape, the three-layer
  migration, and closed-enum discipline.
- `../message/INTENT.md` — daemon-side intent (boundary surfaces, provenance
  stamping, configuration).
- `../signal-router/INTENT.md` — the router observation contract this path feeds.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire layers.
