# ARCHITECTURE — signal-persona-message

The Signal contract for the engine's message-ingress path. It owns
**two named relations sharing one root family** (`MessageRequest` /
`MessageReply`), wired across two different sockets:

```text
Relation A — Client message
  endpoint:   message CLI (sender)             →  persona-message (receiver)
  socket:     message.sock (mode 0660)
  legal payloads (request):   MessageSubmission | InboxQuery
  legal payloads (reply):     SubmissionAccepted | SubmissionRejected | InboxListing | MessageRequestUnimplemented

Relation B — Router ingress
  endpoint:   persona-message (sender)         →  persona-router (receiver)
  socket:     router.sock (mode 0600)
  legal payloads (request):   StampedMessageSubmission
  legal payloads (reply):     SubmissionAccepted | SubmissionRejected | MessageRequestUnimplemented
```

When a user runs `message '(Send designer "hi")'`:

1. `message` CLI constructs a `MessageRequest::MessageSubmission(...)`,
   encodes it as a length-prefixed Signal frame, writes to
   `message.sock`.
2. `persona-message` decodes the frame, mints
   `MessageOrigin::External(ConnectionClass)` from SO_PEERCRED on the
   peer connection, packages the submission + origin + ingress
   timestamp as `StampedMessageSubmission`, and forwards it to
   `router.sock`.
3. `persona-router` accepts the stamped submission, persists a
   message slot with router-minted commit time, and replies with
   `SubmissionAccepted(slot)` or `SubmissionRejected(reason)`.
4. The daemon forwards the reply back to the CLI client.

**Payload-by-payload legality**: `MessageSubmission` is legal only on
Relation A (the daemon may not relay a plain `MessageSubmission` to
router without stamping it). `StampedMessageSubmission` is legal only
on Relation B (the CLI may not construct a stamped submission since
it does not own a `MessageOrigin` mint). Witnesses enforce both rules.

## Record source

This contract imports no domain records from
`signal-persona`; the payloads (`MessageSubmission`,
`SubmissionAcceptance`, `StampedMessageSubmission`, etc.) are defined
in this crate because they are the channel's *interface vocabulary*,
not records that travel beyond this channel. `MessageOrigin` (embedded
in `StampedMessageSubmission`) is imported from `signal-persona-auth`.

(If a payload turns out to belong to another relation, make or update the
relation-specific `signal-persona-*` contract for that relation. Do not lift
message-channel payloads into `signal-persona`; that crate is the top-level
engine-manager contract.)

## Messages

Closed enums declared via `signal_channel!`:

```
MessageRequest              MessageReply
├─ MessageSubmission         ├─ SubmissionAccepted
├─ StampedMessageSubmission  ├─ SubmissionRejected { reason }
└─ InboxQuery                ├─ InboxListing
                              └─ MessageRequestUnimplemented(MessageUnimplementedReason)
```

No `Unknown` variant; no string-tagged dispatch.

### Signal root verbs

Every `MessageRequest` variant declares its root verb through
`MessageRequest::signal_verb()`. The method currently returns
`signal_core::SemaVerb`; this crate keeps that spelling until the
`signal-core` breaking pass lands `SignalVerb`.

```text
MessageSubmission        -> Assert
StampedMessageSubmission -> Assert
InboxQuery               -> Match
```

`InboxQuery` is read-shaped. It is wrapped with `Request::match_records(...)`,
not `Request::assert(...)`. Query algebra such as projection or aggregation
belongs in typed domain query payloads that the receiver lowers to
`sema-engine`, not in the Signal frame root.

### `MessageKind` — typed body semantics

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

```text
MessageUnimplementedReason
  | NotInPrototypeScope
  | DependencyMissing(DependencyKind)
  | ResourceUnavailable(ResourceKind)
```

`MessageRequestUnimplemented(NotInPrototypeScope)` is the typed reply when a
valid request variant has no built behavior yet.

### Origin bridging — `StampedMessageSubmission`

`persona-message` mints `MessageOrigin::External(ConnectionClass)` from
SO_PEERCRED on each connecting peer and forwards a `MessageSubmission` to
router. The bridge record:

```text
StampedMessageSubmission
  | submission:  MessageSubmission
  | origin:      MessageOrigin              (from signal-persona-auth)
  | stamped_at:  TimestampNanos             (ingress observation time;
                                             minted by persona-message)
```

Router accepts `StampedMessageSubmission` on its internal `router.sock` from
`persona-message`. Plain `MessageSubmission` is the shape on the CLI
side (Relation A); the message component performs the stamping before
forwarding on Relation B.

**Timestamp authority**: two distinct timestamps with distinct minters:

| Field | Minted by | Meaning |
|---|---|---|
| `StampedMessageSubmission.stamped_at` | `persona-message` | Ingress observation time. Audit/provenance. |
| Router commit time (on the message slot when persisted) | `persona-router` | Durable commit time. Source of truth for "when did this land in the engine." |

Ingress timestamp is provenance; router commit time is durable message
state. Router does not adopt the ingress timestamp as durable truth.

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

;; inbox reads use the Match root
;; Frame { body: Request(Operation { verb: Match,
;;                                   payload: InboxQuery(InboxQuery {
;;                                       recipient: MessageRecipient::new("designer"),
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

- `signal-core/src/channel.rs` — the macro
