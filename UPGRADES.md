# UPGRADES

## 21.0.0 → 21.0.1 — signal-flow 57

Pins signal-flow 57.0.0 (`8e56457`), Flow 0.53.3's ordinary contract: a
Stop carries its background work, a Refresh of a working flow answers
`RefreshQueued`, a frame that decodes as no Query is answered
`FrameRefused` and observed through `Observe.Undecodable`, and
`Observe.TurnEnds` streams turn ends. The Message contract and every type it takes from
signal-flow are unchanged; the repin keeps one signal-flow in every build
that holds Message beside Flow. Repin meta-signal-message and Message
together.

## 20.0.0 → 21.0.0 — Flow request ids

Pins signal-flow 56.0.0 (`a150f3b`), the accepted Flow 0.53.3 ordinary
contract. Send outcomes and observations carry Flow's public visible forms:
the RequestId, cursor, sender-filtered state and StoreIdentity, without Flow's
private LogicalDeliveryId. The Message contract keeps its typed
`FlowProtocolMismatch`. Repin meta-signal-message and Message together.

## 19.0.0 → 20.0.0 — Flow 9f15 vocabulary, 0.33 codec stack

### What breaks

Pins signal-flow `9f159d9`, signal `7afb68b`, datom-codec `eabead4`, protos
`3c7ade3`, generator ethos-zero `5a5daca`. The imported Flow types change shape:
`Address` carries a `Model` where it carried a `Layer`; `Layer` is no longer
exported and `Model` is, in its place. `Recipient` is the `Address` itself
(there is no `Recipient::Up`); `SendRequest` is `{ recipient, sendable_message }`
where `SendableMessage` is `HardAbrupt`, `MiddleAbrupt` or `Soft` over a
`SendableRequest`; the former `Delivery` is gone. Address datoms read
`{ Mind flow Sol }`; the request datom reads
`Send.{ { Mind flow Sol } MiddleAbrupt.Question.«…» }`. Bare newtype text and
text compaction follow datom-codec `eabead4`.

`SendRefusal` gains one appended variant, `FlowProtocolMismatch`, carrying no
payload: Flow answered with a response kind the operation does not allow, or
with a public response that could not be decoded. It is distinct from
`FlowUnreachable`, which is only a failure to connect or exchange. Appending
leaves every earlier variant's archived bytes unchanged (a test compares them
against the earlier enum). Because 20.0.0 was never published, this is part of
the 20.0.0 wire, not a further number. Message's other refusals are unchanged,
including `NoLayer` and `Unidentified.Process`, which Flow's own
refusal no longer shares (Flow now carries seat evidence). Whether `NoLayer`
and the remaining shared names stay is a contract decision left open.

### Deploy

Repin `meta-signal-message` and `message` to this release together, with a Flow
that speaks the 9f15 vocabulary. Older peers cannot greet this contract.

## 18.0.0 → 18.0.1 — simplified Flow peer

Pins signal-flow `5dee03` for Flow 0.39.0. The imported Message
request and response types have unchanged layouts. Repin the Message
producer closure and rebuild Message with Flow; Flow’s own root Query
archive changes with the simplified launch profile.

## 16.0.0 → 17.0.0 — typed oversized Flow refusal

Adds `SendRefusal::Oversized(SendPayloadSize)` so Message preserves Flow’s
actual and maximum rendered-prompt byte counts. Pins signal-flow 27.0.0
(`cc524c8`), retaining its deployed Delivery contract. Repin Message peers
together.

## 15.0.0 → 16.0.0 — signal-flow 25.0.0

### What breaks

Pins signal-flow 25.0.0 (c67e561) and flow-ethos (flow dc2cba6), whose
`SentBy` appends `Flow`. The wire of this contract is unchanged; a crate
that links signal-flow beside it repins both.

### Deploy

Repin `meta-signal-message` and `message` to this release together.

## 12.0.0 → 13.0.0 — complete typed Flow refusals

### What breaks

`SendRefusal` adds `NoLayer` and `Store.String`. A missing recipient
layer remains distinct, and store failures carry Flow's diagnostic.
Older peers cannot greet this contract.

### Deploy

Repin `meta-signal-message` and `message` to this release together.
Their operation failure mapping must preserve these payloads.

## 11.0.0 → 12.0.0 — typed Flow recipient and request

### What breaks

`Send` is now `{ Recipient Request }` from `signal-flow` 12.0.0.
The flow-id vector, priority, content, message ledger, receipt and
withdrawal vocabularies are removed from this contract. Refusals carry
the typed Flow address, lock or process where applicable. `Woken` has
no flow-id payload.

This release pins `signal-flow` a991c149, its `flow-ethos` source
a3173348, and `signal` e0e3c055 so the request types and frame belong
to one dependency graph. Old peers cannot greet the new contract.

### Deploy

Update `meta-signal-message` and `message` together. Remove consumers of
the retired receipt and redelivery shapes; no compatibility adapter is
provided.

## 9.0.0 → 10.0.0 — signal 8.0.0, signal-flow 10.0.0, meta-signal-flow 14.0.0

### What breaks

- The contract depends on `signal` 8.0.0 (f35460de), `signal-flow` 10.0.0
  (f95034de) and `meta-signal-flow` 14.0.0 (54eb5618), where it depended on
  signal 7.0.0, signal-flow 8.0.0 and meta-signal-flow 12.0.0. The imported
  Flow types and the `signal::Contracted` that `Query` implements are now
  those releases'; a crate holding the older ones alongside sees two sets.
- The `datom` feature also enables `signal/datom`: signal 8.0.0 binds the
  same datom-codec and protos 0.32.2, so the graph holds one codec.
- The build reads the ethos with ethos-zero 16.0.0 at c2653dd8. The
  generated module is byte-identical (the build script asserts it), so
  `ETHOS`, the contract digest and every archive are unchanged from 9.0.0.

### Deploy

In meta-signal-message and message: repin signal-message with signal 8.0.0,
signal-flow 10.0.0 and meta-signal-flow 14.0.0 in one change. The wire is
unchanged; a 9.0.0 peer still greets this one.

## 8.0.0 → 9.0.0 — ethos-zero 16.0.0, signal 7.0.0, protos and datom-codec 0.32.2

### What breaks

- The contract depends on `signal` 7.0.0 (66e7b153), the exchange layer,
  where it depended on 5.0.0. The re-exported `Signal`, `Signalizable`,
  `ByteViewable` and `Restorable` are signal 7.0.0's types. `Query`
  implements `signal::Contracted` over `ETHOS`: the contract's wire identity
  is the digest of `ethos/signal.ethos`, and a peer greets with
  `Query::greeting()`.
- It depends on `signal-flow` 8.0.0 (c297d987) and `meta-signal-flow`
  12.0.0 (69f9c146). `MessageId`, `Content`, `BodyRefusal`,
  `DeliveryRejection` and `InterruptWitness` are those releases' types; a
  consumer still on meta-signal-flow 11.0.0 holds different Rust types.
- The `datom` feature pins protos 0.32.2 (15b41da8) and datom-codec 0.32.2
  (4dff16b4), where it pinned 0.31.0, and no longer enables `signal/datom`
  (signal 7.0.0's `datom` feature pins datom-codec 0.31.0, a second codec;
  nothing here holds a signal type).
- protos 0.32's `textualize` prints vertically. The one-line datom a CLI
  reads and writes is `Compactable::compact`.
- The build dependency is ethos-zero 16.0.0 (0edfc0c3). The ethos source and
  the generated Rust are unchanged; the rkyv archive of every value of this
  contract is unchanged.

### Deploying

Nothing runs from this crate. In each consumer (meta-signal-message,
message): repin `signal-message`, `signal` 66e7b153, `signal-flow`
c297d987, `meta-signal-flow` 69f9c146, and protos and datom-codec 0.32.2
wherever named; replace `.textualize()` with `.compact()` where one-line text
is meant; rebuild. Message must be rebuilt and restarted together with the
Flow it talks to (flow 0.19.0), because meta-signal-flow 12.0.0's frames are
not readable by 11.0.0.

## 7.0.0 → 8.0.0 — a refusal names a Retired or Exited flow

### What breaks

`RecipientRefused` carries `meta-signal-flow` 11.0.0's `DeliveryRejection`,
which gains `FlowRetired` and `FlowExited` (appended; every earlier tag
keeps its place). A reader pinned to 7.0.0 cannot decode either. Repin
`meta-signal-flow` 2ac045c alongside.

## 5.0.0 → 6.0.0 — a message is just a message

### What breaks

Every request and reply is replaced. Queries: `Send.SendRequest`,
`Withdraw.MessageId`, `Acknowledge.MessageId`, `QueryReceipts.MessageId`,
`Observe.MessageId`. Replies: `Submitted`, `SendRejected`, `Withdrawn`,
`Acknowledged`, `Receipts`, `ReceiptObserved`, `MessageRejected`.

Retired with no replacement on this wire: `Submit`, `SubmitStamped`,
`QueryInbox`, the thread queries, the agent registry, `FlowDeliver`,
`FlowAnnounceIdle`, `Deliver` with `ClusterMessage`, `QueryDeliveryReceipts`,
and `MessageDaemonConfiguration` (the meta contract owns configuration).

The contract now depends on `signal-flow` 7.0.0 and `meta-signal-flow` 9.0.0
and pins `datom-codec` 09e2a9d / `protos` 1febca7 to match them.


## 2.0.1 → 3.0.0 — shared frame, arity-split codec

### What breaks

1. **The portable frame is no longer declared here.** `Signal<T>`,
   `Signalizable`, `ByteViewable` and `Restorable<T>` are re-exported from
   `signal` 5.0.0 instead of vendored. `signal_message::Signal` still
   resolves, so most consumers need a repin rather than an edit — but the
   type *identity* changed. Code that mixed this contract's frame with
   another contract's vendored copy was relying on two distinct types and
   must now use the one shared frame.
2. **The generated projection derives `Composing`, not `Compositional`.**
   `datom-codec` 0.27.0 split the composing kind. A consumer writing its own
   `T: Compositional` bound over these types must change it to `T: Composing`.

The rkyv bytes and the Datom text are unchanged. This is a type-identity and
trait-name break, not a wire break.

### Deploying

Nothing to deploy: this crate is a contract with no runtime of its own. Every
consumer must be repinned in one pass, because `signal` declares
`links = "signal"` and Cargo admits exactly one package with a given `links`
key per graph. Two consumers pinning different `signal` revisions — or the
same revision spelled differently — fail at **resolution**, which masks every
compile error behind it.

Pin `signal` as `https://github.com/LiGoldragon/signal` with **no `.git`
suffix**: cargo source identity is the pin string, not the commit, so
`signal.git` and `signal` are two packages for one commit.

## 17.0.0 → 18.0.0 — stream durable delivery status

### What breaks

`Send` and `SendThread` now carry Flow's shared `SendRequest`. Message
returns `SendAccepted` then a sender-filtered status snapshot and updates.
Clients resume with Flow's durable `DeliveryCursor`.

### Deploy

Repin `meta-signal-message` and Message to this contract. Message must consume
Flow's authenticated sender stream and forward updates on the originating
connection.
