# ARCHITECTURE — signal-message

The Signal contract for the engine's message-ingress path. It owns
**two named relations sharing one root family** (`MessageRequest` /
`MessageReply`), wired across two different sockets:

## Three-layer model

**Layer 1 — Contract Operations on the wire (this crate).** Drop the
Sema-shaped wrappers entirely. The contract-local public operations are
`Submit(MessageSubmission)`, `SubmitStamped(StampedMessageSubmission)`, and
`QueryInbox(InboxQuery)`. The cross-contract reuse pattern applies:
`Submit` here means "submit a message for routing" while `Submit` in
`signal-mind` means "submit a thought to the graph" — both
legitimate per the contract-locality principle.

**Layer 2 — Component Commands (message daemon).** The
message daemon owns its typed Command enum (e.g.
`MessageCommand::RecordSubmission`,
`MessageCommand::StampOrigin`,
`MessageCommand::ReadInbox`) plus a `CommandExecutor`. Lowering from
contract operation (`Submit`/`SubmitStamped`/`QueryInbox`) to commands
happens in the daemon, not in this contract.

**Layer 3 — Sema classification (signal-sema).** Each Component
Command projects to a payloadless `SemaOperation` class via
`ToSemaOperation`. The daemon emits the class label at observation
publish time.

**Frame layer.** This crate uses `signal-frame`, not `signal-core`.

```text
Relation A — Message ingress
  endpoint:   message CLI or component client  →  message (receiver)
  sockets:    message.sock for owner/external clients
              message-ingress/<instance>.sock for manager-created
              component-instance clients
  legal payloads (request):   MessageSubmission | InboxQuery
  legal payloads (reply):     SubmissionAccepted | SubmissionRejected | InboxListing | MessageRequestUnimplemented

Relation B — Router ingress
  endpoint:   message (sender)         →  router (receiver)
  socket:     router.sock (mode 0600)
  legal payloads (request):   StampedMessageSubmission
  legal payloads (reply):     SubmissionAccepted | SubmissionRejected | MessageRequestUnimplemented
```

When a user runs `message '(Send designer "hi")'` through the owner
ingress:

1. `message` CLI constructs a `MessageRequest::Submit(...)`,
   encodes it as a length-prefixed Signal frame, writes to
   `message.sock`.
2. `message` decodes the frame, mints
   `MessageOrigin::External(ConnectionClass)` from SO_PEERCRED on the
   peer connection, packages the submission + origin + ingress
   timestamp as `StampedMessageSubmission`, and forwards it to
   `router.sock`.
3. `router` accepts the stamped submission, persists a
   message slot with router-minted commit time, and replies with
   `SubmissionAccepted(slot)` or `SubmissionRejected(reason)`.
4. The daemon forwards the reply back to the CLI client.

When a supervised component uses its manager-created ingress socket, the
payload shape is still `MessageSubmission`; the accepted socket chooses
the origin. The component client does **not** send its own sender or origin
in-band. `message` stamps
`MessageOrigin::InternalComponentInstance(...)` from the
`ComponentMessageIngress` entry configured by the engine manager.

**Payload-by-payload legality**: `MessageSubmission` is legal only on
Relation A (the daemon may not relay a plain `MessageSubmission` to
router without stamping it). `StampedMessageSubmission` is legal only
on Relation B (the CLI may not construct a stamped submission since
it does not own a `MessageOrigin` mint). Witnesses enforce both rules.

## Record source

This contract imports no manager-domain records. The payloads
(`MessageSubmission`, `SubmissionAcceptance`, `StampedMessageSubmission`,
etc.) are defined in this crate because they are the channel's *interface
vocabulary*, not records that travel beyond this channel. `MessageOrigin`
(embedded in `StampedMessageSubmission`) is imported from
`signal-persona-origin`.

(If a payload turns out to belong to another relation, make or update the
relation-specific `signal-*` contract for that relation. Do not lift
message-channel payloads into manager contract crates; engine-management
crates are not relation buckets.)

## Messages

Closed enums declared via `signal_channel!`:

```
MessageRequest              MessageReply
├─ Submit                    ├─ SubmissionAccepted
├─ SubmitStamped             ├─ SubmissionRejected { reason }
└─ QueryInbox                ├─ InboxListing
                              └─ MessageRequestUnimplemented(MessageUnimplementedReason)
```

No `Unknown` variant; no string-tagged dispatch.

### Sema-class projections (Layer 3)

Each contract-local operation's daemon-side Component Command
projects to a payloadless Sema class via `ToSemaOperation`:

```text
Submit                   -> Assert    (records new submission)
SubmitStamped            -> Assert    (records stamped submission to router)
QueryInbox               -> Match     (reads inbox)
```

`QueryInbox` is read-shaped. The daemon lowers it into Component
Commands that include a `Match`-shaped read plan; the Sema class
label is the daemon-side projection, not encoded into the wire
frame. Query algebra such as projection or aggregation belongs in
typed domain query payloads that the receiver lowers to its
`CommandExecutor`, not in the wire envelope.

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

`message` owns the local provenance bridge. It accepts a plain
`MessageSubmission`, derives the origin from the accepted ingress, and
forwards a stamped record to router.

Ingress classes:

| Ingress | Origin minted by `message` | Who configures it |
|---|---|---|
| `message.sock` | `MessageOrigin::External(ConnectionClass)` from SO_PEERCRED / owner context | `MessageDaemonConfiguration.message_socket_path` |
| `message-ingress/<instance>.sock` | `MessageOrigin::InternalComponentInstance(InternalComponentInstanceOrigin)` | `MessageDaemonConfiguration.component_ingresses` written by the engine manager |

The bridge record:

```text
StampedMessageSubmission
  | submission:  MessageSubmission
  | origin:      MessageOrigin              (from signal-persona-origin)
  | stamped_at:  TimestampNanos             (ingress observation time;
                                             minted by message)
```

Router accepts `StampedMessageSubmission` on its internal `router.sock` from
`message`. Plain `MessageSubmission` is the shape on the CLI
or component-client side (Relation A); the message component performs the
stamping before forwarding on Relation B. Origin fields are never accepted
from a Relation A caller.

**Timestamp authority**: two distinct timestamps with distinct minters:

| Field | Minted by | Meaning |
|---|---|---|
| `StampedMessageSubmission.stamped_at` | `message` | Ingress observation time. Audit/provenance. |
| Router commit time (on the message slot when persisted) | `router` | Durable commit time. Source of truth for "when did this land in the engine." |

Ingress timestamp is provenance; router commit time is durable message
state. Router does not adopt the ingress timestamp as durable truth.

## Versioning

`signal_frame::Frame` carries the protocol version; this
contract inherits the kernel's version-skew guard.
Schema-level changes here (adding/removing variants) are
breaking and require a coordinated upgrade of
`message` + `router`.

## Examples

```nota
;; the CLI invocation
(Send designer [hi])

;; produces this wire frame (length-prefix omitted for clarity)
;; Frame { body: Request(Submit(MessageSubmission { recipient: MessageRecipient::new("designer"), body: MessageBody::new("hi") })) }
;; — the wire form carries the contract-local verb only; the daemon-
;; side Component Command will project to Sema Assert at observation
;; publish time.

;; inbox reads carry the contract-local QueryInbox operation
;; Frame { body: Request(QueryInbox(InboxQuery { recipient: MessageRecipient::new("designer") })) }
;; — the daemon-side Component Command projects to Sema Match.
```

## Round trips

Each variant of `MessageRequest` and `MessageReply` has a frame round-trip
test in `tests/round_trip.rs`. Representative NOTA text witnesses cover
`Submit(MessageSubmission)` and `SubmissionAcceptance`; root channel enum text codecs
come from `signal_frame::signal_channel!`.

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

- `signal-frame/macros/src/validate.rs` — the macro
- `~/primary/skills/component-triad.md` §"Verbs come in three layers".
