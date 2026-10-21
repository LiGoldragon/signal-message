//! Typed Message requests addressed through Flow.

extern crate self as message_library;

pub mod generated;
pub use generated::message_library::*;
pub use generated::signal::*;

pub const LIBRARY_ETHOS: &str = include_str!("../ethos/message_library.ethos");
pub const ETHOS: &str = concat!(
    include_str!("../ethos/message_library.ethos"),
    "\n",
    include_str!("../ethos/signal.ethos"),
);
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The portable rkyv Signal frame is one shared type across the estate.
/// A second copy here would be a different Rust type, forking the wire.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// Flow's request vocabulary is part of Message's public wire API.
pub use signal_flow::Aspect;
pub use signal_flow::{Address, Layer, Lock, Process, Recipient, Request};

/// The contract is identified on the wire by the digest of its authored
/// Ethos source; the querying side greets with it.
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
