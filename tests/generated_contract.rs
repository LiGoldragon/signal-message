//! Requests and replies survive their archive and have concrete datoms.

use signal_message::{
    Address, Aspect, Configuration, Layer, Query, Recipient, Request, Response, Send_Data,
    SendRefusal,
};

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
    let addressed = Query::Send(Send_Data {
        recipient: Recipient::Address(address),
        request: Request::Question("What is current?".into()),
    });
    let upward = Query::Send(Send_Data {
        recipient: Recipient::Up,
        request: Request::Order("Refresh the current flow.".into()),
    });
    assert_eq!(archived_query(&addressed), addressed);
    assert_eq!(archived_query(&upward), upward);
}

#[test]
fn flow_and_message_refusals_survive_the_archive() {
    for response in [
        Response::Delivered,
        Response::Woken,
        Response::Queued,
        Response::Configured,
        Response::Refused(SendRefusal::AlreadyConfigured),
        Response::Refused(SendRefusal::FlowUnreachable),
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
            "Send.{ Address.{ Mind flow Secondary } Question.«What is current?» }",
            "Send.{ Up Order.«Refresh the current flow.» }",
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
            "Delivered",
            "Woken",
            "Queued",
            "Configured",
            "Refused.AlreadyConfigured",
            "Refused.FlowUnreachable",
            "Refused.Unidentified.{ 42 100 }",
            "Refused.Unknown.Address.{ Mind flow Secondary }",
            "Refused.Unknown.Lock.{ { Mind flow Primary } { Mind flow Secondary } 12345 }",
            "Refused.Ended.Address.{ Mind flow Primary }",
            "Refused.Asleep",
            "Refused.NoneAbove",
            "Refused.OffRoute",
            "Refused.Held.Lock.{ { Mind flow Primary } { Mind flow Secondary } 12345 }",
            "Refused.Locked",
            "Refused.Lapsed",
            "Refused.NotMessage",
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
            Potential::<Response>::from("Send.{ Up Order.«refresh» }")
                .actualize(&mut budget())
                .is_err()
        );
    }
}
