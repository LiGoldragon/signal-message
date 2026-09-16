# UPGRADES

## 3.0.0 → 4.0.0 — the flow-delivery vocabulary

### What breaks

1. **`Query`, `Response`, `MessageOperationKind` and
   `FlowDeliveryRejectionReason` each gained variants.** These enums are
   closed vocabulary and consumers match them exhaustively, so every peer
   must be recompiled. A peer built against 3.0.0 that is handed a
   `FlowDeliver`, `DeliveryQueued`, `DeliveryLanded` or `FlowDeliveryRejected`
   discriminant fails bytecheck with no typed way to say why — which is the
   reason this is a major move rather than a minor one.
2. **`FlowDeliveryRejectionReason::ConflictingEnvelope` is new.** A source
   event identifier re-used for a *different* envelope is now a typed
   refusal. It was previously not expressible, and the messenger answered
   `DeliveryQueued` while dropping the second text.

The 3.0.0 vocabulary was published as 3.0.0 twice — once without the
flow-delivery types and once with them. That is the defect this bump repairs:
the crate's semver IS the wire's semver.

### Deploying

Nothing to deploy: this crate is a contract with no runtime of its own.
Repin every consumer in one pass — `meta-signal-message` and `message` both
pin this crate by rev, and cargo admits exactly one `signal-message` per
graph.

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
