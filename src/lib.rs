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

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

// ─── Payloads ──────────────────────────────────────────────

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct MessageRecipient(String);

impl MessageRecipient {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct MessageSender(String);

impl MessageSender {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct MessageBody(String);

impl MessageBody {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MessageSlot(u64);

impl MessageSlot {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// Submit a message from the calling agent to a recipient.
/// The sender is resolved by router/daemon ingress from the
/// accepted socket context, not provided by the caller, per
/// the "infrastructure mints identity, time, sender" rule
/// (`ESSENCE.md` §"Infrastructure mints identity").
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MessageSubmission {
    /// The recipient agent's name.
    pub recipient: MessageRecipient,
    /// The body of the message — opaque text.
    pub body: MessageBody,
}

/// Reply to an accepted message submission. The router has committed
/// the message through router-owned state and assigned it a durable slot.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SubmissionAcceptance {
    /// The slot under which the message was persisted.
    pub message_slot: MessageSlot,
}

/// Query the current inbox for a recipient. The reply
/// carries the messages currently visible to that recipient.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InboxQuery {
    pub recipient: MessageRecipient,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum MessageOperationKind {
    MessageSubmission,
    InboxQuery,
}

/// Reply to an `Inbox` query — the messages currently
/// addressed to the recipient, in slot order.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InboxListing {
    pub messages: Vec<InboxEntry>,
}

/// One message visible in an inbox. Sender is the
/// router-resolved sender at submit time.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InboxEntry {
    pub message_slot: MessageSlot,
    pub sender: MessageSender,
    pub body: MessageBody,
}

/// A rejected submission — the router could not commit the
/// message. The router carries the typed reason; the caller
/// can pattern-match on it.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SubmissionRejection {
    pub reason: SubmissionRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum SubmissionRejectionReason {
    /// The router could not persist the message.
    StoreRejected,
    /// The recipient name does not resolve to a known actor.
    RecipientNotFound,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    request MessageRequest {
        MessageSubmission(MessageSubmission),
        InboxQuery(InboxQuery),
    }
    reply MessageReply {
        SubmissionAccepted(SubmissionAcceptance),
        SubmissionRejected(SubmissionRejection),
        InboxListing(InboxListing),
    }
}

impl MessageRequest {
    pub fn operation_kind(&self) -> MessageOperationKind {
        match self {
            Self::MessageSubmission(_) => MessageOperationKind::MessageSubmission,
            Self::InboxQuery(_) => MessageOperationKind::InboxQuery,
        }
    }
}
