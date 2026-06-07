//! Signal contract — Message ingress relations.
//!
//! Read this file as the public interface of the messaging
//! channel. One root family serves two relations:
//! `message` CLI to `message-daemon`, and
//! `message-daemon` to `router`. The
//! variants name the legal payloads; relation-specific
//! legality is documented in `ARCHITECTURE.md`.
//!
//! See `ARCHITECTURE.md` for the channel's role and
//! boundaries; `~/primary/reports/designer/72-harmonized-implementation-plan.md`
//! §6 for the contract-creation discipline.

use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_engine_management::{SocketMode, TimestampNanos, WirePath};
use signal_frame::signal_channel;
use signal_persona_origin::{InternalComponentInstanceOrigin, MessageOrigin, OwnerIdentity};

// ─── Payloads ──────────────────────────────────────────────

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    NotaEncode,
    NotaDecode,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
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
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MessageSubmission {
    /// The recipient agent's name.
    pub recipient: MessageRecipient,
    /// Typed message semantics. The freeform body stays opaque text.
    pub kind: MessageKind,
    /// The body of the message — opaque text.
    pub body: MessageBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct StampedMessageSubmission {
    pub submission: MessageSubmission,
    pub origin: MessageOrigin,
    pub stamped_at: TimestampNanos,
}

/// Manager-created ingress endpoint for one local component instance.
///
/// A component connecting through this private socket is stamped with
/// the configured internal component-instance origin. The message text
/// remains sender-free; infrastructure mints the sender from the socket
/// relation.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentMessageIngress {
    pub origin: InternalComponentInstanceOrigin,
    pub socket_path: WirePath,
    pub socket_mode: SocketMode,
}

/// Reply to an accepted message submission. The router has committed
/// the message through router-owned state and assigned it a durable slot.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubmissionAcceptance {
    /// The slot under which the message was persisted.
    pub message_slot: MessageSlot,
}

/// Query the current inbox for a recipient. The reply
/// carries the messages currently visible to that recipient.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InboxQuery {
    pub recipient: MessageRecipient,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MessageOperationKind {
    Submit,
    SubmitStamped,
    QueryInbox,
}

/// Reply to an `Inbox` query — the messages currently
/// addressed to the recipient, in slot order.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InboxListing {
    pub messages: Vec<InboxEntry>,
}

/// One message visible in an inbox. Sender is the
/// router-resolved sender at submit time.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InboxEntry {
    pub message_slot: MessageSlot,
    pub sender: MessageSender,
    pub body: MessageBody,
}

/// A rejected submission — the router could not commit the
/// message. The router carries the typed reason; the caller
/// can pattern-match on it.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubmissionRejection {
    pub reason: SubmissionRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum SubmissionRejectionReason {
    /// The router could not persist the message.
    StoreRejected,
    /// The recipient name does not resolve to a known actor.
    RecipientNotFound,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MessageRequestUnimplemented {
    pub operation: MessageOperationKind,
    pub reason: MessageUnimplementedReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum MessageUnimplementedReason {
    NotInPrototypeScope,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum DependencyKind {
    Router,
    Mind,
    Harness,
    Terminal,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ResourceKind {
    MessageSocket,
    RouterSocket,
    PeerCredentials,
    Store,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    channel Message {
        operation Submit(MessageSubmission),
        operation SubmitStamped(StampedMessageSubmission),
        operation QueryInbox(InboxQuery),
    }
    reply MessageReply {
        SubmissionAccepted(SubmissionAcceptance),
        SubmissionRejected(SubmissionRejection),
        InboxListing(InboxListing),
        MessageRequestUnimplemented(MessageRequestUnimplemented),
    }
}

pub type MessageRequest = Operation;
pub type ChannelRequest = signal_frame::Request<MessageRequest>;
pub type ChannelReply = signal_frame::Reply<MessageReply>;
pub type MessageRequestBuilder = RequestBuilder;

impl MessageRequest {
    pub fn operation_kind(&self) -> MessageOperationKind {
        match self {
            Self::Submit(_) => MessageOperationKind::Submit,
            Self::SubmitStamped(_) => MessageOperationKind::SubmitStamped,
            Self::QueryInbox(_) => MessageOperationKind::QueryInbox,
        }
    }
}

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `message-daemon`. Human tooling may author
// this record through NOTA, but the live daemon consumes a signal-encoded rkyv
// archive path. The daemon does not parse NOTA.

/// Startup configuration for `message-daemon`.
///
/// Replaces the previous `PERSONA_SOCKET`,
/// `PERSONA_SOCKET_MODE`, `PERSONA_SUPERVISION_SOCKET_PATH`,
/// `PERSONA_MESSAGE_ROUTER_SOCKET`, `PERSONA_SPAWN_ENVELOPE`,
/// etc. environment-variable surface.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MessageDaemonConfiguration {
    /// Where the daemon binds its message-ingress Unix socket.
    pub message_socket_path: WirePath,
    /// chmod applied to the message-ingress socket after bind.
    pub message_socket_mode: SocketMode,
    /// Where the daemon binds its supervision Unix socket.
    pub supervision_socket_path: WirePath,
    /// chmod applied to the supervision socket after bind.
    pub supervision_socket_mode: SocketMode,
    /// The router socket this daemon forwards stamped submissions to.
    pub router_socket_path: WirePath,
    /// Manager-created local component ingress endpoints.
    pub component_ingresses: Vec<ComponentMessageIngress>,
    /// The engine owner identity stamped onto submissions when the
    /// peer credential matches.
    pub owner_identity: OwnerIdentity,
}

impl MessageDaemonConfiguration {
    pub fn from_rkyv_bytes(bytes: &[u8]) -> Result<Self, MessageDaemonConfigurationArchiveError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|_| MessageDaemonConfigurationArchiveError::Decode)
    }

    pub fn to_rkyv_bytes(&self) -> Result<Vec<u8>, MessageDaemonConfigurationArchiveError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| MessageDaemonConfigurationArchiveError::Encode)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MessageDaemonConfigurationArchiveError {
    #[error("failed to encode message daemon configuration archive")]
    Encode,

    #[error("failed to decode message daemon configuration archive")]
    Decode,
}
