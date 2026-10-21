#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Unknown_Data {
    Address(signal_flow::Address),
    Lock(signal_flow::Lock),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SendRefusal {
    Unidentified(signal_flow::Process),
    Unknown(Unknown_Data),
    Ended(signal_flow::Address),
    Asleep,
    NoneAbove,
    OffRoute,
    Held(signal_flow::Lock),
    Locked,
    Lapsed,
    NotMessage,
    NoLayer,
    Store(String),
    FlowUnreachable,
    AlreadyConfigured,
    NotConfigured,
    NotSendServer(signal_flow::Process),
    Oversized(signal_flow::SendPayloadSize),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Send_Data {
    pub recipient: signal_flow::Recipient,
    pub request: signal_flow::Request,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendThread_Data {
    pub recipient: signal_flow::Recipient,
    pub request: signal_flow::Request,
    pub native_session_id: signal_flow::NativeSessionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Send(Send_Data),
    Configure(message_library::Configuration),
    SendThread(SendThread_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Delivered,
    Woken,
    Queued,
    Configured,
    Refused(SendRefusal),
}
