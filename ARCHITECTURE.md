# ARCHITECTURE — signal-persona-message

The Signal contract between `message-cli` (sender) and
`persona-router` (receiver). The whole channel is one
`signal_channel!` invocation in `src/lib.rs`.

## Channel

| Side | Component |
|---|---|
| Sender (request side) | `persona-message` (the `message` CLI) |
| Receiver (reply side) | `persona-router` (the routing daemon) |

When a user runs `message designer "hi"`, message-cli
constructs a `MessageRequest::Submit(...)`, wraps it in a
`Frame`, encodes via `encode_length_prefixed`, and writes
the bytes to persona-router's UDS. The router decodes,
matches on the variant, and replies with `MessageReply::SubmitOk(...)`
or `MessageReply::SubmitFailed(...)`.

## Record source

This contract imports no domain records from
`signal-persona`; the payloads (`SubmitMessage`,
`SubmitReceipt`, etc.) are defined in this crate because
they are the channel's *interface vocabulary*, not records
that travel beyond this channel.

(If a payload turns out to be widely shared, we lift it to
`signal-persona`'s umbrella records and import it here.)

## Messages

Closed enums declared via `signal_channel!`:

```
MessageRequest         MessageReply
├─ Submit(SubmitMessage)   ├─ SubmitOk(SubmitReceipt)
└─ Inbox(InboxQuery)       ├─ SubmitFailed(SubmitFailed { reason })
                           └─ InboxResult(InboxResult)
```

No `Unknown` variant; no string-tagged dispatch.

## Versioning

`signal_core::Frame` carries the protocol version; this
contract inherits the kernel's version-skew guard.
Schema-level changes here (adding/removing variants) are
breaking and require a coordinated upgrade of
`persona-message` + `persona-router`.

## Examples

```nota
;; the CLI invocation
(Submit designer "hi")

;; produces this wire frame (length-prefix omitted for clarity)
;; Frame { auth: Some(LocalOperatorProof("operator")),
;;         body: Request(Operation { verb: Assert,
;;                                   payload: Submit(SubmitMessage { recipient: "designer", body: "hi" }) }) }
```

## Round trips

Each variant of `MessageRequest` and `MessageReply` has a
round-trip test in `tests/round_trip.rs`:
text → typed → frame → length-prefixed bytes →
decoded frame → typed.

Architectural-truth tests (per
`~/primary/skills/architectural-truth-tests.md`) fire when:
- A new variant is added without a round-trip test.
- The Frame's encode/decode bytes don't match.
- A consumer tries to dispatch on a variant that isn't in
  the closed enum.

## Non-ownership

- No actors. No daemons. No `tokio`.
- No transport (UDS path, reconnect, timeouts).
- No routing logic. No storage. No terminal logic.

## Code map

```
src/
└── lib.rs    — payloads + signal_channel! invocation
tests/
└── round_trip.rs — per-variant wire-form round trips
```

## See also

- `~/primary/reports/designer/72-harmonized-implementation-plan.md`
  §2.1 — channel inventory
- `~/primary/reports/designer/73-signal-derive-research.md` —
  the `signal_channel!` macro decision
- `signal-core/src/channel.rs` — the macro
- `signal-persona-store/ARCHITECTURE.md` — the next-hop
  channel from the router
