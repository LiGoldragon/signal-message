//! Ordinary Message Nexus Signal contract.
//!
//! A message is just a message: recipients, a Priority and a Content. The
//! MessageId, the Content, the refusals and the interrupt witness are
//! declared in `meta-signal-flow`, because Flow is the pane writer that
//! renders and refuses them: the id is typed at the head of the letter, so
//! a recipient can Acknowledge from what it reads in its own pane.

pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The portable rkyv Signal frame is one shared type across the estate.
/// A second copy here would be a different Rust type, forking the wire.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The id a message is known by on both contracts. Declared once, where
/// Flow renders it into the pane, and named here so a Message peer spells
/// it in Message's own vocabulary.
pub use meta_signal_flow::MessageId;

/// The contract is identified on the wire by the digest of its authored
/// Ethos source; the querying side greets with it.
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
