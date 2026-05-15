//! Signal contract — Persona message ingress relations.
//!
//! Read this file as the public interface of the messaging
//! channel. One root family serves two relations:
//! `message` CLI to `persona-message-daemon`, and
//! `persona-message-daemon` to `persona-router`. The
//! variants name the legal payloads; relation-specific
//! legality is documented in `ARCHITECTURE.md`.
//!
//! See `ARCHITECTURE.md` for the channel's role and
//! boundaries; `~/primary/reports/designer/72-harmonized-implementation-plan.md`
//! §6 for the contract-creation discipline.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
use signal_persona::TimestampNanos;
use signal_persona_auth::MessageOrigin;

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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum MessageKind {
    Send,
    Inbox,
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
    /// Typed message semantics. The freeform body stays opaque text.
    pub kind: MessageKind,
    /// The body of the message — opaque text.
    pub body: MessageBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct StampedMessageSubmission {
    pub submission: MessageSubmission,
    pub origin: MessageOrigin,
    pub stamped_at: TimestampNanos,
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
    StampedMessageSubmission,
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MessageRequestUnimplemented {
    pub operation: MessageOperationKind,
    pub reason: MessageUnimplementedReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageUnimplementedReason {
    NotInPrototypeScope,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}

impl NotaEncode for MessageUnimplementedReason {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::NotInPrototypeScope => {
                encoder.start_record("NotInPrototypeScope")?;
                encoder.end_record()
            }
            Self::DependencyMissing(dependency) => {
                encoder.start_record("DependencyMissing")?;
                dependency.encode(encoder)?;
                encoder.end_record()
            }
            Self::ResourceUnavailable(resource) => {
                encoder.start_record("ResourceUnavailable")?;
                resource.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for MessageUnimplementedReason {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "NotInPrototypeScope" => {
                decoder.expect_record_head("NotInPrototypeScope")?;
                decoder.expect_record_end()?;
                Ok(Self::NotInPrototypeScope)
            }
            "DependencyMissing" => {
                decoder.expect_record_head("DependencyMissing")?;
                let dependency = DependencyKind::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::DependencyMissing(dependency))
            }
            "ResourceUnavailable" => {
                decoder.expect_record_head("ResourceUnavailable")?;
                let resource = ResourceKind::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::ResourceUnavailable(resource))
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "MessageUnimplementedReason",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum DependencyKind {
    Router,
    Mind,
    Harness,
    Terminal,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ResourceKind {
    MessageSocket,
    RouterSocket,
    PeerCredentials,
    Store,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    channel MessageChannel {
        request MessageRequest {
            Assert MessageSubmission(MessageSubmission),
            Assert StampedMessageSubmission(StampedMessageSubmission),
            Match InboxQuery(InboxQuery),
        }
        reply MessageReply {
            SubmissionAccepted(SubmissionAcceptance),
            SubmissionRejected(SubmissionRejection),
            InboxListing(InboxListing),
            MessageRequestUnimplemented(MessageRequestUnimplemented),
        }
    }
}

pub type Frame = MessageChannelFrame;
pub type FrameBody = MessageChannelFrameBody;
pub type ChannelRequest = MessageChannelChannelRequest;
pub type ChannelReply = MessageChannelChannelReply;
pub type MessageRequestBuilder = MessageChannelRequestBuilder;

impl MessageRequest {
    pub fn operation_kind(&self) -> MessageOperationKind {
        match self {
            Self::MessageSubmission(_) => MessageOperationKind::MessageSubmission,
            Self::StampedMessageSubmission(_) => MessageOperationKind::StampedMessageSubmission,
            Self::InboxQuery(_) => MessageOperationKind::InboxQuery,
        }
    }
}
