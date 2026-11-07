//! Requests and replies survive their archive and have concrete datoms.

use signal_flow::{
    Address, Aspect, Model, SendableMessage, SendableRequest, VisibleDeliveryState,
    VisibleSendAcceptance, VisibleSendObservation,
};
use signal_message::{Configuration, Query, Response, SendRefusal};

fn archived_query(query: &Query) -> Query {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(query).expect("archives");
    rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).expect("restores")
}

fn archived_response(response: &Response) -> Response {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(response).expect("archives");
    rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes).expect("restores")
}

fn configuration() -> Configuration {
    Configuration {
        ordinary: "/run/user/1001/message/message.sock".into(),
        meta: "/run/user/1001/message/message-owner.sock".into(),
        flow: "/run/user/1001/flow/flow.sock".into(),
    }
}

#[test]
fn configuration_keeps_its_archive_shape() {
    let query = Query::Configure(configuration());
    assert_eq!(archived_query(&query), query);
}

#[test]
fn typed_address_requests_survive_the_archive() {
    let address = Address {
        aspect: Aspect::Mind,
        topic: "flow".into(),
        model: Model::Sol,
    };
    let request = |sendable_message| signal_flow::SendRequest {
        recipient: address.clone(),
        sendable_message,
    };
    for message in [
        SendableMessage::MiddleAbrupt(SendableRequest::Question("What is current?".into())),
        SendableMessage::Soft(SendableRequest::Notice("Noted.".into())),
        SendableMessage::HardAbrupt(SendableRequest::Order("Stop.".into())),
    ] {
        let query = Query::Send(request(message));
        assert_eq!(archived_query(&query), query);
    }
}

#[test]
fn accepted_and_observed_delivery_states_survive_archive() {
    let accepted = Response::SendAccepted(VisibleSendAcceptance {
        request_id: "k7ph".into(),
        delivery_cursor: 1,
        visible_delivery_state: VisibleDeliveryState::Queued,
    });
    assert_eq!(archived_response(&accepted), accepted);
    let observed = Response::SendsObserved(VisibleSendObservation {
        store_identity: "store-1".into(),
        delivery_cursor: 1,
        visible_delivery_snapshot_vector: vec![],
    });
    assert_eq!(archived_response(&observed), observed);
}

#[test]
fn flow_and_message_refusals_survive_the_archive() {
    for response in [
        Response::Configured,
        Response::Refused(SendRefusal::AlreadyConfigured),
        Response::Refused(SendRefusal::FlowUnreachable),
        Response::Refused(SendRefusal::FlowProtocolMismatch),
        Response::Refused(SendRefusal::NoLayer),
        Response::Refused(SendRefusal::Store("database unavailable".into())),
        Response::Refused(SendRefusal::Oversized(signal_flow::SendPayloadSize {
            actual_bytes: 65_537,
            maximum_bytes: 65_536,
        })),
    ] {
        assert_eq!(archived_response(&response), response);
    }
}

#[cfg(feature = "datom")]
mod datom {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    use signal_message::{Query, Response};

    fn budget() -> Budget {
        Budget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn requests_use_the_typed_recipient_and_request() {
        for text in [
            "Configure.{ /run/user/1001/message/message.sock /run/user/1001/message/message-owner.sock /run/user/1001/flow/flow.sock }",
            "Send.{ { Mind flow Sol } MiddleAbrupt.Question.«What is current?» }",
            "Send.{ { Mind flow Sol } Soft.Notice.Noted. }",
            "SendThread.{ { { Mind flow Sol } MiddleAbrupt.Question.«whole body» } recorded-thread }",
            "ObserveSends.Some.1",
            "ObserveThread.{ Some.1 recorded-thread }",
        ] {
            let query = Potential::<Query>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        }
    }

    #[test]
    fn replies_keep_typed_refusal_paths() {
        for text in [
            "Configured",
            "Refused.AlreadyConfigured",
            "Refused.FlowUnreachable",
            "Refused.FlowProtocolMismatch",
            "Refused.Unidentified.{ 42 100 }",
            "Refused.Unknown.Address.{ Mind flow Sol }",
            "Refused.Unknown.Lock.{ { Mind flow Astra } { Mind flow Sol } 12345 }",
            "Refused.Ended.{ Mind flow Astra }",
            "Refused.Asleep",
            "Refused.NoneAbove",
            "Refused.OffRoute",
            "Refused.Held.{ { Mind flow Astra } { Mind flow Sol } 12345 }",
            "Refused.Locked",
            "Refused.Lapsed",
            "Refused.NotMessage",
            "Refused.NotConfigured",
            "Refused.NotSendServer.{ 42 100 }",
            "Refused.NoLayer",
            "Refused.Store.«database unavailable»",
            "Refused.Oversized.{ 65537 65536 }",
        ] {
            let response = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(response.datomize(vec![]).protosize().compact(), text);
        }
    }

    #[test]
    fn a_send_request_is_not_a_reply() {
        assert!(
            Potential::<Response>::from("Send.{ { Mind flow Sol } MiddleAbrupt.Order.«refresh» }")
                .actualize(&mut budget())
                .is_err()
        );
    }
}

#[test]
fn thread_send_and_identification_refusals_survive_archive() {
    let query = Query::SendThread(signal_message::SendThread_Data {
        send_request: signal_flow::SendRequest {
            recipient: Address {
                aspect: Aspect::Mind,
                topic: "flow".into(),
                model: Model::Sol,
            },
            sendable_message: SendableMessage::MiddleAbrupt(SendableRequest::Question(
                "whole body".into(),
            )),
        },
        native_session_id: "recorded-thread".into(),
    });
    assert_eq!(archived_query(&query), query);
    for response in [
        Response::Refused(SendRefusal::NotConfigured),
        Response::Refused(SendRefusal::NotSendServer(signal_message::Process {
            pid: 42,
            started: 100,
        })),
    ] {
        assert_eq!(archived_response(&response), response);
    }
}

/// The refusals that existed before `FlowProtocolMismatch` was appended keep their archived
/// bytes: the mirror is the earlier enum, variant for variant, in the earlier order.
#[derive(rkyv::Archive, rkyv::Serialize, Clone)]
#[expect(dead_code, reason = "a layout mirror: only some variants are compared")]
enum EarlierSendRefusal {
    Unidentified(signal_message::Process),
    Unknown(signal_message::Unknown_Data),
    Ended(Address),
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
    NotSendServer(signal_message::Process),
    Oversized(signal_flow::SendPayloadSize),
}

#[test]
fn appending_the_mismatch_refusal_keeps_the_earlier_archives() {
    let process = signal_message::Process {
        pid: 42,
        started: 100,
    };
    let address = Address {
        aspect: Aspect::Mind,
        topic: "flow".into(),
        model: Model::Sol,
    };
    let bytes = |value: &dyn Fn() -> Vec<u8>| value();
    let pairs: Vec<(SendRefusal, EarlierSendRefusal)> = vec![
        (
            SendRefusal::Unidentified(process.clone()),
            EarlierSendRefusal::Unidentified(process.clone()),
        ),
        (
            SendRefusal::Ended(address.clone()),
            EarlierSendRefusal::Ended(address.clone()),
        ),
        (SendRefusal::Asleep, EarlierSendRefusal::Asleep),
        (SendRefusal::OffRoute, EarlierSendRefusal::OffRoute),
        (SendRefusal::Lapsed, EarlierSendRefusal::Lapsed),
        (SendRefusal::NoLayer, EarlierSendRefusal::NoLayer),
        (
            SendRefusal::Store("database unavailable".into()),
            EarlierSendRefusal::Store("database unavailable".into()),
        ),
        (
            SendRefusal::FlowUnreachable,
            EarlierSendRefusal::FlowUnreachable,
        ),
        (
            SendRefusal::AlreadyConfigured,
            EarlierSendRefusal::AlreadyConfigured,
        ),
        (
            SendRefusal::NotSendServer(process.clone()),
            EarlierSendRefusal::NotSendServer(process),
        ),
        (
            SendRefusal::Oversized(signal_flow::SendPayloadSize {
                actual_bytes: 65_537,
                maximum_bytes: 65_536,
            }),
            EarlierSendRefusal::Oversized(signal_flow::SendPayloadSize {
                actual_bytes: 65_537,
                maximum_bytes: 65_536,
            }),
        ),
    ];
    for (current, earlier) in pairs {
        let now = bytes(&|| {
            rkyv::to_bytes::<rkyv::rancor::Error>(&current)
                .unwrap()
                .to_vec()
        });
        let then = bytes(&|| {
            rkyv::to_bytes::<rkyv::rancor::Error>(&earlier)
                .unwrap()
                .to_vec()
        });
        assert_eq!(now, then, "{current:?}");
    }
}
