#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Priority {
    HardAbrupt,
    MiddleAbrupt,
    Soft,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendRequest {
    pub flow_id_vector: std::vec::Vec<signal_flow::FlowId>,
    pub priority: Priority,
    pub content: meta_signal_flow::Content,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Grade {
    Submitted,
    Parked,
    Transported,
    Presented,
    Uncertain,
    Read,
    Withdrawn,
    Refused(meta_signal_flow::DeliveryRejection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Receipt {
    pub flow_id: signal_flow::FlowId,
    pub interrupt_witness: meta_signal_flow::InterruptWitness,
    pub grade: Grade,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Submission {
    pub message_id: meta_signal_flow::MessageId,
    pub receipt_vector: std::vec::Vec<Receipt>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BodyRefused_Data {
    pub flow_id: signal_flow::FlowId,
    pub body_refusal: meta_signal_flow::BodyRefusal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecipientRefused_Data {
    pub flow_id: signal_flow::FlowId,
    pub delivery_rejection: meta_signal_flow::DeliveryRejection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SendRejection {
    SenderUnknown,
    EmptyRecipients,
    UnknownRecipient(signal_flow::FlowId),
    BodyRefused(BodyRefused_Data),
    RecipientRefused(RecipientRefused_Data),
    FlowUnreachable,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MessageRejection {
    UnknownMessage,
    NotSender,
    NotRecipient,
    NotParked,
    NotUncertain,
    FlowUnreachable,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Send(SendRequest),
    Withdraw(meta_signal_flow::MessageId),
    Acknowledge(meta_signal_flow::MessageId),
    QueryReceipts(meta_signal_flow::MessageId),
    Observe(meta_signal_flow::MessageId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Submitted(Submission),
    SendRejected(SendRejection),
    Withdrawn(meta_signal_flow::MessageId),
    Acknowledged(meta_signal_flow::MessageId),
    Receipts(Submission),
    ReceiptObserved(Receipt),
    MessageRejected(MessageRejection),
}
