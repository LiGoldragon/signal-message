# UPGRADES

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
