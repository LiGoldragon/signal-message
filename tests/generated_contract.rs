//! The falsifiable specification of every record kind: each one lands as a
//! concrete datom that round-trips, and every request and reply survives
//! the rkyv archive.

use meta_signal_flow::{Content, DeliveryRejection, InterruptWitness};
use signal_message::{Grade, Priority, Query, Receipt, Response, SendRequest, Submission};

fn archived_query(query: &Query) -> Query {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(query).expect("archives");
    rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).expect("restores")
}

fn archived_response(response: &Response) -> Response {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(response).expect("archives");
    rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes).expect("restores")
}

#[test]
fn a_send_request_survives_the_archive() {
    let query = Query::Send(SendRequest {
        flow_id_vector: vec!["7d41e0".into(), "88475f".into()],
        priority: Priority::Soft,
        content: Content::Text("Stage 1 is deployed; run the tier tests.".into()),
    });
    assert_eq!(archived_query(&query), query);
}

#[test]
fn a_refused_receipt_carries_flows_own_rejection() {
    let response = Response::ReceiptObserved(Receipt {
        flow_id: "7d41e0".into(),
        interrupt_witness: InterruptWitness::NotRequested,
        grade: Grade::Refused(DeliveryRejection::RecipientBlocked),
    });
    assert_eq!(archived_response(&response), response);
    let submitted = Response::Submitted(Submission {
        message_id: "m-7f3a2c".into(),
        receipt_vector: vec![],
    });
    assert_ne!(archived_response(&submitted), response);
}

#[cfg(feature = "datom")]
mod datom {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    use signal_message::{Query, Response};

    fn budget() -> Budget {
        Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn every_request_kind_has_a_concrete_datom() {
        for text in [
            "Send.{ [ 7d41e0 ] Soft Text.«Stage 1 is deployed; run the tier tests.» }",
            "Send.{ [ 7d41e0 88475f ] HardAbrupt Psyche.{ «on build hosts» «Prometheus should be doing the builds.» } }",
            "Send.{ [ 7d41e0 ] MiddleAbrupt Text./compact }",
            "Withdraw.m-7f3a2c",
            "Acknowledge.m-7f3a2c",
            "QueryReceipts.m-7f3a2c",
            "Observe.m-7f3a2c",
        ] {
            let query = Potential::<Query>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        }
    }

    #[test]
    fn every_reply_kind_has_a_concrete_datom() {
        for text in [
            "Submitted.{ m-7f3a2c [ { 7d41e0 NotRequested Parked } ] }",
            "Submitted.{ m-81b0e4 [ { 7d41e0 Observed Transported } { 88475f NotRequested Presented } ] }",
            "SendRejected.BodyRefused.{ 7d41e0 HarnessCommand./compact }",
            "SendRejected.BodyRefused.{ 7d41e0 ControlCharacter.14 }",
            "SendRejected.UnknownRecipient.ffffff",
            "SendRejected.RecipientRefused.{ 7d41e0 FlowStopped }",
            "SendRejected.RecipientRefused.{ 7d41e0 FlowExited }",
            "SendRejected.SenderUnknown",
            "SendRejected.EmptyRecipients",
            "SendRejected.FlowUnreachable",
            "Withdrawn.m-7f3a2c",
            "Acknowledged.m-7f3a2c",
            "Receipts.{ m-7f3a2c [ { 7d41e0 NotRequested Read } ] }",
            "ReceiptObserved.{ 7d41e0 NotRequested Refused.RecipientWorking }",
            "ReceiptObserved.{ 7d41e0 Unobserved Uncertain }",
            "ReceiptObserved.{ 7d41e0 NotRequested Withdrawn }",
            "MessageRejected.NotParked",
            "MessageRejected.NotRecipient",
            "MessageRejected.NotUncertain",
        ] {
            let response = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(response.datomize(vec![]).protosize().compact(), text);
        }
    }

    #[test]
    fn a_request_datom_is_not_a_reply() {
        assert!(
            Potential::<Response>::from("Send.{ [ 7d41e0 ] Soft Text.hello }")
                .actualize(&mut budget())
                .is_err()
        );
    }
}
