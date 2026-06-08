//! Schema-derived Signal contract for message ingress relations.

#[rustfmt::skip]
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
        self.component
    }

    pub fn instance(&self) -> &ComponentInstanceName {
        &self.instance
    }
}

impl Input {
    pub fn operation_kind(&self) -> MessageOperationKind {
        match self {
            Self::Submit(_) => MessageOperationKind::Submit,
            Self::SubmitStamped(_) => MessageOperationKind::SubmitStamped,
            Self::QueryInbox(_) => MessageOperationKind::QueryInbox,
        }
    }
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
