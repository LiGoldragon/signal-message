//! Requests and replies survive their archive and have concrete datoms.

use signal_flow::{
    Address, Aspect, Delivery, DeliveryState, Layer, Message, Recipient, Request, SendAcceptance,
    SendObservation,
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
fn typed_address_and_up_requests_survive_the_archive() {
    let address = Address {
        aspect: Aspect::Mind,
        topic: "flow".into(),
        layer: Layer::Secondary,
    };
    let request = |recipient| signal_flow::SendRequest {
        recipient,
        delivery: Delivery::Wake,
        message: Message::MiddleAbrupt(Request::Question("What is current?".into())),
    };
    let addressed = Query::Send(request(Recipient::Address(address)));
    let upward = Query::Send(request(Recipient::Up));
    assert_eq!(archived_query(&addressed), addressed);
    assert_eq!(archived_query(&upward), upward);
}

#[test]
fn accepted_and_observed_delivery_states_survive_archive() {
    let accepted = Response::SendAccepted(SendAcceptance {
        logical_delivery_id: "logical-1".into(),
        delivery_cursor: 1,
        delivery_state: DeliveryState::Queued,
    });
    assert_eq!(archived_response(&accepted), accepted);
    let observed = Response::SendsObserved(SendObservation {
        delivery_cursor: 1,
        delivery_snapshot_vector: vec![],
    });
    assert_eq!(archived_response(&observed), observed);
}

#[test]
fn flow_and_message_refusals_survive_the_archive() {
    for response in [
        Response::Configured,
        Response::Refused(SendRefusal::AlreadyConfigured),
        Response::Refused(SendRefusal::FlowUnreachable),
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
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn requests_use_the_typed_recipient_and_request() {
        for text in [
            "Configure.{ /run/user/1001/message/message.sock /run/user/1001/message/message-owner.sock /run/user/1001/flow/flow.sock }",
            "Send.{ Address.{ Mind flow Secondary } Wake MiddleAbrupt.Question.«What is current?» }",
            "Send.{ Up Wake MiddleAbrupt.Order.«Refresh the current flow.» }",
            "SendThread.{ { Up Wake MiddleAbrupt.Question.«whole body» } recorded-thread }",
            "ObserveSends.1",
            "ObserveThread.{ 1 recorded-thread }",
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
            "Refused.Unidentified.{ 42 100 }",
            "Refused.Unknown.Address.{ Mind flow Secondary }",
            "Refused.Unknown.Lock.{ { Mind flow Primary } { Mind flow Secondary } 12345 }",
            "Refused.Ended.{ Mind flow Primary }",
            "Refused.Asleep",
            "Refused.NoneAbove",
            "Refused.OffRoute",
            "Refused.Held.{ { Mind flow Primary } { Mind flow Secondary } 12345 }",
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
            Potential::<Response>::from("Send.{ Up Wake MiddleAbrupt.Order.«refresh» }")
                .actualize(&mut budget())
                .is_err()
        );
    }
}

#[test]
fn thread_send_and_identification_refusals_survive_archive() {
    let query = Query::SendThread(signal_message::SendThread_Data {
        send_request: signal_flow::SendRequest {
            recipient: Recipient::Up,
            delivery: Delivery::Wake,
            message: Message::MiddleAbrupt(Request::Question("whole body".into())),
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
