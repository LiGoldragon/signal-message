//! Schema-derived Signal contract for message ingress relations.

#[rustfmt::skip]
#[allow(dead_code, private_interfaces)]
pub mod schema;

pub use schema::lib::*;

impl MessageRecipient {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl MessageSender {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl MessageBody {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl ThreadName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl MessageSlot {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl TimestampNanos {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl WirePath {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl SocketMode {
    pub fn into_u32(self) -> u32 {
        self.into_payload() as u32
    }
}

impl UnixUserIdentifier {
    pub fn as_u32(&self) -> u32 {
        *self.payload() as u32
    }
}

impl SystemPrincipal {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl EngineIdentifier {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl HostName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl NetworkPeer {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl ComponentInstanceName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl InternalComponentInstanceOrigin {
    pub fn component(&self) -> ComponentName {
        self.component_name
    }

    pub fn instance(&self) -> &ComponentInstanceName {
        &self.component_instance_name
    }
}

impl Input {
    pub fn operation_kind(&self) -> MessageOperationKind {
        match self {
            Self::Submit(_) => MessageOperationKind::Submit,
            Self::SubmitStamped(_) => MessageOperationKind::SubmitStamped,
            Self::QueryInbox(_) => MessageOperationKind::QueryInbox,
            Self::AssignAgentIdentity(_) => MessageOperationKind::AssignAgentIdentity,
            Self::BindAgentEndpoint(_) => MessageOperationKind::BindAgentEndpoint,
            Self::QueryAgentRegistry(_) => MessageOperationKind::QueryAgentRegistry,
        }
    }
}

impl AgentIdentifier {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl ResumeIdentity {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl HarnessPid {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl HarnessStartTime {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl AgentRegistryListing {
    pub fn from_entries(entries: Vec<AgentRegistryEntry>) -> Self {
        Self::new(Entries::new(entries))
    }

    pub fn entries(&self) -> &[AgentRegistryEntry] {
        self.payload().payload().as_slice()
    }

    pub fn into_entries(self) -> Vec<AgentRegistryEntry> {
        self.into_payload().into_payload()
    }
}

impl InboxListing {
    pub fn from_entries(entries: Vec<InboxEntry>) -> Self {
        Self::new(Messages::new(entries))
    }

    pub fn entries(&self) -> &[InboxEntry] {
        self.payload().payload().as_slice()
    }

    pub fn into_entries(self) -> Vec<InboxEntry> {
        self.into_payload().into_payload()
    }
}

impl Output {
    pub fn inbox_listing_entries(entries: Vec<InboxEntry>) -> Self {
        Self::InboxListing(InboxListing::from_entries(entries))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageDaemonConfigurationParts {
    pub message_socket_path: WirePath,
    pub message_socket_mode: SocketMode,
    pub supervision_socket_path: WirePath,
    pub supervision_socket_mode: SocketMode,
    pub router_socket_path: WirePath,
    pub component_ingresses: Vec<ComponentMessageIngress>,
    pub owner_identity: OwnerIdentity,
}

impl From<MessageDaemonConfigurationParts> for MessageDaemonConfiguration {
    fn from(parts: MessageDaemonConfigurationParts) -> Self {
        Self {
            message_socket_path: parts.message_socket_path.into(),
            message_socket_mode: parts.message_socket_mode.into(),
            supervision_socket_path: parts.supervision_socket_path.into(),
            supervision_socket_mode: parts.supervision_socket_mode.into(),
            router_socket_path: parts.router_socket_path.into(),
            component_ingresses: ComponentIngresses::new(parts.component_ingresses),
            owner_identity: parts.owner_identity,
        }
    }
}

impl MessageDaemonConfiguration {
    pub fn component_ingresses(&self) -> &[ComponentMessageIngress] {
        self.component_ingresses.payload().as_slice()
    }

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
