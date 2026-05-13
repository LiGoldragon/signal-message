# ARCHITECTURE — signal-persona-message

The Signal contract between `message-cli` (sender) and
`persona-router` (receiver). It relates one CLI sender to the
router ingress: the CLI supplies recipient + body content, while
router/daemon ingress stamps provenance from the accepted socket
context and the router mints message slots. The whole channel is
one `signal_channel!` invocation in `src/lib.rs`.

## Channel

| Side | Component |
|---|---|
| Sender (request side) | `persona-message` (the `message` CLI) |
| Receiver (reply side) | `persona-router` (the routing daemon) |

When a user runs `message '(Send designer "hi")'`, message-cli
constructs a `MessageRequest::MessageSubmission(...)`, wraps it in a
sender-free `Frame`, encodes via `encode_length_prefixed`, and writes
the bytes to persona-router's UDS. The router decodes,
stamps provenance at ingress, matches on the variant, and replies with
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
                           ├─ InboxListing
                           └─ MessageRequestUnimplemented(MessageUnimplementedReason)
```

No `Unknown` variant; no string-tagged dispatch.

### `MessageKind` — typed body semantics

Per `/127 §4.1 D2` and
`~/primary/reports/designer/143-prototype-readiness-gap-audit.md` §3:
`MessageBody(String)` stays freeform; specificity grows via a closed
`MessageKind` enum carried alongside the body.

```text
MessageKind (closed enum, prototype-scope)
  | Send
  | Inbox
  -- future variants land as coordinated schema bumps
```

The `MessageSubmission` record carries `kind: MessageKind` so router and
harness can dispatch on the typed kind rather than parsing the freeform body.

### Skeleton honesty (Unimplemented reply)

Per
`~/primary/reports/designer/143-prototype-readiness-gap-audit.md` §4.3:

```text
MessageUnimplementedReason
  | NotInPrototypeScope
  | DependencyMissing(DependencyKind)
  | ResourceUnavailable(ResourceKind)
```

`MessageRequestUnimplemented(NotInPrototypeScope)` is the typed reply when a
valid request variant has no built behavior yet.

### Origin bridging — `StampedMessageSubmission`

`persona-message-daemon` mints `MessageOrigin::External(ConnectionClass)` from
SO_PEERCRED on each connecting peer and forwards a `MessageSubmission` to
router. The bridge record:

```text
StampedMessageSubmission
  | submission:  MessageSubmission
  | origin:      MessageOrigin              (from signal-persona-auth)
  | stamped_at:  TimestampNanos             (manager/daemon-side timestamp)
```

Router accepts `StampedMessageSubmission` on its internal `router.sock` from
`persona-message-daemon`. Plain `MessageSubmission` is the shape on the CLI
side (one NOTA in / one NOTA out); the daemon performs the stamping.

## Versioning

`signal_core::Frame` carries the protocol version; this
contract inherits the kernel's version-skew guard.
Schema-level changes here (adding/removing variants) are
breaking and require a coordinated upgrade of
`persona-message` + `persona-router`.

## Examples

```nota
;; the CLI invocation
(Send designer "hi")

;; produces this wire frame (length-prefix omitted for clarity)
;; Frame { body: Request(Operation { verb: Assert,
;;                                   payload: MessageSubmission(MessageSubmission {
;;                                       recipient: MessageRecipient::new("designer"),
;;                                       body: MessageBody::new("hi"),
;;                                   }) }) }
```

## Round trips

Each variant of `MessageRequest` and `MessageReply` has a frame round-trip
test in `tests/round_trip.rs`. Representative NOTA text witnesses cover
`MessageSubmission` and `SubmissionAcceptance`; root channel enum text codecs
come from `signal_core::signal_channel!`.

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
└── round_trip.rs — per-variant frame round trips + NOTA text witnesses
```

## See also

- `~/primary/reports/designer/72-harmonized-implementation-plan.md`
  §2.1 — channel inventory
- `~/primary/reports/designer/73-signal-derive-research.md` —
  the `signal_channel!` macro decision
- `signal-core/src/channel.rs` — the macro
- `~/primary/reports/designer/78-convergence-with-operator-77.md`
  — convergence on retiring the store-channel boundary
