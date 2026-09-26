//! Ordinary Message Nexus Signal contract.
//!
//! A message is just a message: recipients, a Priority and a Content. The
//! Content, the refusals and the interrupt witness are Flow's own types,
//! imported from `meta-signal-flow`, because Flow is the pane writer that
//! renders and refuses them.

pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The portable rkyv Signal frame is one shared type across the estate.
/// A second copy here would be a different Rust type, forking the wire.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};
