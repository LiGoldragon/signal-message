# UPGRADES

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
