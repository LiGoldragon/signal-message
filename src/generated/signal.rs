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
pub struct SendThread_Data {
    pub send_request: signal_flow::SendRequest,
    pub native_session_id: signal_flow::NativeSessionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObserveThread_Data {
    pub delivery_cursor_option: std::option::Option<signal_flow::DeliveryCursor>,
    pub native_session_id: signal_flow::NativeSessionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Send(signal_flow::SendRequest),
    SendThread(SendThread_Data),
    ObserveSends(std::option::Option<signal_flow::DeliveryCursor>),
    ObserveThread(ObserveThread_Data),
    Configure(message_library::Configuration),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObservationInterrupted_Data {
    pub delivery_cursor_option: std::option::Option<signal_flow::DeliveryCursor>,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    SendAccepted(signal_flow::SendAcceptance),
    SendsObserved(signal_flow::SendObservation),
    SendChanged(signal_flow::DeliverySnapshot),
    ObservationInterrupted(ObservationInterrupted_Data),
    Configured,
    Refused(SendRefusal),
}
