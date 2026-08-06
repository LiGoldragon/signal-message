//! Ordinary Message ingress Interface.
//!
//! `ethos/interface.ethos` is the canonical textual projection of one
//! authority-verified, role-free bootstrap Interface. Its checked Rust
//! projection carries only encoded Type identities. Input/Output roles and
//! Signal frame behavior remain handwritten until Logos owns that slice.

pub mod bootstrap_manifest;
pub mod schema;

pub const MESSAGE_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const MESSAGE_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");

pub use schema::lib::*;
