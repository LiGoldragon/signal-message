//! Signal contract — `message-cli` ↔ `persona-router`.
//!
//! Read this file as the public interface of the messaging
//! channel. Two enums; the variants are the messages the
//! channel carries; the per-variant payload types are the
//! wire vocabulary. The whole channel is one
//! `signal_channel!` invocation.
//!
//! See `ARCHITECTURE.md` for the channel's role and
//! boundaries; `~/primary/reports/designer/72-harmonized-implementation-plan.md`
//! §6 for the contract-creation discipline.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

// ─── Payloads ──────────────────────────────────────────────

/// Submit a message from the calling agent to a recipient.
/// The sender is resolved by the router from the calling
/// process's ancestry — not provided by the caller, per the
/// "infrastructure mints identity, time, sender" rule
/// (`ESSENCE.md` §"Infrastructure mints identity").
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubmitMessage {
    /// The recipient agent's name.
    pub recipient: String,
    /// The body of the message — opaque text.
    pub body: String,
}

/// Reply to a successful `Submit`. The router has committed
/// the message and assigned it a slot in `persona-sema`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubmitReceipt {
    /// The slot under which the message was persisted.
    pub message_slot: u64,
}

/// Query the current inbox for a recipient. The reply
/// carries the messages currently visible to that recipient.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InboxQuery {
    pub recipient: String,
}

/// Reply to an `Inbox` query — the messages currently
/// addressed to the recipient, in slot order.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InboxResult {
    pub messages: Vec<InboxMessage>,
}

/// One message visible in an inbox. Sender is the
/// router-resolved sender at submit time.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InboxMessage {
    pub message_slot: u64,
    pub sender: String,
    pub body: String,
}

/// A failure on `Submit` — the router could not commit the
/// message. The router carries the typed reason; the caller
/// can pattern-match on it.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubmitFailed {
    pub reason: SubmitFailureReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum SubmitFailureReason {
    /// The router could not persist the message.
    PersistenceRejected,
    /// The recipient name does not resolve to a known actor.
    UnknownRecipient,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    request MessageRequest {
        Submit(SubmitMessage),
        Inbox(InboxQuery),
    }
    reply MessageReply {
        SubmitOk(SubmitReceipt),
        SubmitFailed(SubmitFailed),
        InboxResult(InboxResult),
    }
}
