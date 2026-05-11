# ARCHITECTURE — signal-persona-message

The Signal contract between `message-cli` (sender) and
`persona-router` (receiver). It relates one CLI sender to the
router ingress: the CLI supplies recipient + body content, while
the router mints sender identity and message slots. The whole
channel is one `signal_channel!` invocation in `src/lib.rs`.

## Channel

| Side | Component |
|---|---|
| Sender (request side) | `persona-message` (the `message` CLI) |
| Receiver (reply side) | `persona-router` (the routing daemon) |

When a user runs `message designer "hi"`, message-cli
constructs a `MessageRequest::MessageSubmission(...)`, wraps it in a
`Frame`, encodes via `encode_length_prefixed`, and writes
the bytes to persona-router's UDS. The router decodes,
matches on the variant, and replies with
`MessageReply::SubmissionAccepted(...)` or
`MessageReply::SubmissionRejected(...)`.

## Record source

This contract imports no domain records from
`signal-persona`; the payloads (`MessageSubmission`,
`SubmissionAcceptance`, etc.) are defined in this crate because
they are the channel's *interface vocabulary*, not records
that travel beyond this channel.

(If a payload turns out to belong to another relation, make or update the
relation-specific `signal-persona-*` contract for that relation. Do not lift
message-channel payloads into `signal-persona`; that crate is the top-level
engine-manager contract.)

## Messages

Closed enums declared via `signal_channel!`:

```
MessageRequest         MessageReply
├─ MessageSubmission       ├─ SubmissionAccepted
└─ InboxQuery              ├─ SubmissionRejected { reason }
                           └─ InboxListing
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
;;                                   payload: MessageSubmission(MessageSubmission {
;;                                       recipient: MessageRecipient::new("designer"),
;;                                       body: MessageBody::new("hi"),
;;                                   }) }) }
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
- No routing logic. No persistence policy. No terminal logic.

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
- `~/primary/reports/designer/78-convergence-with-operator-77.md`
  — convergence on retiring the store-channel boundary
