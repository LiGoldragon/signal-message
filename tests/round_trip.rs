//! Architectural-truth tests for the
//! `signal-message` channel.
//!
//! Each test names exactly what shape it pins down; per the
//! "blunt test names" convention.

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_message::{
    ComponentInstanceName, ComponentMessageIngress, ComponentName, ConnectionClass, DependencyKind,
    Frame, FrameBody, InboxEntry, InboxListing, InboxQuery, Input, InternalComponentInstanceOrigin,
    MessageBody, MessageDaemonConfiguration, MessageKind, MessageOperationKind, MessageOrigin,
    MessageRecipient, MessageRequestUnimplemented, MessageSender, MessageSlot, MessageSubmission,
    MessageUnimplementedReason, Output, OwnerIdentity, ResourceKind, SocketMode,
    StampedMessageSubmission, SubmissionAcceptance, SubmissionRejection, SubmissionRejectionReason,
    TimestampNanos, UnixUserIdentifier, WirePath,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn text(value: &str) -> String {
    value.to_owned()
}

fn recipient(value: &str) -> MessageRecipient {
    MessageRecipient::new(text(value))
}

fn sender(value: &str) -> MessageSender {
    MessageSender::new(text(value))
}

fn body(value: &str) -> MessageBody {
    MessageBody::new(text(value))
}

fn path(value: &str) -> WirePath {
    WirePath::new(text(value))
}

fn instance(value: &str) -> ComponentInstanceName {
    ComponentInstanceName::new(text(value))
}

fn submission(recipient_value: &str, body_value: &str) -> MessageSubmission {
    MessageSubmission {
        recipient: recipient(recipient_value),
        kind: MessageKind::Send,
        body: body(body_value),
    }
}

fn inbox_query(recipient_value: &str) -> InboxQuery {
    InboxQuery::new(recipient(recipient_value))
}

fn acceptance(slot: u64) -> SubmissionAcceptance {
    SubmissionAcceptance::new(MessageSlot::new(slot))
}

fn request_frame(request: Input) -> Frame {
    Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    })
}

fn reply_frame(reply: Output) -> Frame {
    Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    })
}

fn decode_single_reply(frame: Frame) -> Output {
    match frame.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    assert_eq!(text, expected);
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(recovered, value);
}

#[test]
fn message_submission_request_round_trips_through_length_prefixed_frame() {
    let request = Input::Submit(MessageSubmission {
        recipient: recipient("designer"),
        kind: MessageKind::Send,
        body: body("stack test"),
    });
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            assert_eq!(decoded_request.payloads().head(), &request);
        }
        other => panic!("expected Submit request, got {other:?}"),
    }
}

#[test]
fn stamped_message_submission_request_round_trips_through_length_prefixed_frame() {
    let request = Input::SubmitStamped(StampedMessageSubmission {
        submission: submission("designer", "stack test"),
        origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(42),
    });
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            assert_eq!(decoded_request.payloads().head(), &request);
        }
        other => panic!("expected SubmitStamped request, got {other:?}"),
    }
}

#[test]
fn inbox_query_round_trips_through_length_prefixed_frame() {
    let request = Input::QueryInbox(inbox_query("designer"));
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            assert_eq!(decoded_request.payloads().head(), &request);
        }
        other => panic!("expected QueryInbox request, got {other:?}"),
    }
}

#[test]
fn submission_accepted_reply_round_trips() {
    let reply = Output::SubmissionAccepted(acceptance(1024));
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn submission_rejected_reply_round_trips_with_typed_reason() {
    let reply = Output::SubmissionRejected(SubmissionRejection::new(
        SubmissionRejectionReason::RecipientNotFound,
    ));
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn unimplemented_reply_round_trips_with_typed_reason() {
    let reply = Output::MessageRequestUnimplemented(MessageRequestUnimplemented {
        operation: MessageOperationKind::SubmitStamped,
        reason: MessageUnimplementedReason::ResourceUnavailable(ResourceKind::PeerCredentials),
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn inbox_listing_round_trips_through_length_prefixed_frame() {
    let reply = Output::InboxListing(InboxListing::new(vec![
        InboxEntry {
            message_slot: MessageSlot::new(1),
            sender: sender("operator"),
            body: body("first"),
        },
        InboxEntry {
            message_slot: MessageSlot::new(2),
            sender: sender("operator"),
            body: body("second"),
        },
    ]));
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn payload_constructor_lifts_message_submission_into_request() {
    let payload = MessageSubmission {
        recipient: recipient("designer"),
        kind: MessageKind::Send,
        body: body("via from"),
    };
    let request = Input::Submit(payload.clone());
    assert_eq!(request, Input::Submit(payload));
}

#[test]
fn payload_constructor_lifts_submission_acceptance_into_reply() {
    let acceptance = acceptance(7);
    let reply = Output::SubmissionAccepted(acceptance.clone());
    assert_eq!(reply, Output::SubmissionAccepted(acceptance));
}

#[test]
fn message_request_exposes_contract_owned_operation_kind() {
    let plain_submission = Input::Submit(MessageSubmission {
        recipient: recipient("designer"),
        kind: MessageKind::Send,
        body: body("kind witness"),
    });
    let stamped = Input::SubmitStamped(StampedMessageSubmission {
        submission: submission("designer", "kind witness"),
        origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(1),
    });
    let inbox = Input::QueryInbox(inbox_query("designer"));

    assert_eq!(
        plain_submission.operation_kind(),
        MessageOperationKind::Submit
    );
    assert_eq!(
        stamped.operation_kind(),
        MessageOperationKind::SubmitStamped
    );
    assert_eq!(inbox.operation_kind(), MessageOperationKind::QueryInbox);
}

#[test]
fn message_request_variants_declare_contract_local_operation_heads() {
    assert_eq!(
        <Input as SignalOperationHeads>::HEADS,
        &["Submit", "SubmitStamped", "QueryInbox"]
    );
}

#[test]
fn message_operation_kind_round_trips_through_nota_text() {
    round_trip_nota(MessageOperationKind::Submit, "Submit");
}

#[test]
fn message_submission_request_round_trips_through_nota_text() {
    let request = Input::Submit(MessageSubmission {
        recipient: recipient("designer"),
        kind: MessageKind::Send,
        body: body("stack test"),
    });

    round_trip_nota(request, "(Submit (designer Send [stack test]))");
}

#[test]
fn stamped_message_submission_request_round_trips_through_nota_text() {
    let request = Input::SubmitStamped(StampedMessageSubmission {
        submission: submission("designer", "stack test"),
        origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(99),
    });

    let text = request.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<Input>()
        .expect("decode request");

    assert_eq!(recovered, request);
    assert!(text.contains("SubmitStamped"));
    assert!(text.contains("Owner"));
}

#[test]
fn submission_accepted_reply_round_trips_through_nota_text() {
    let reply = Output::SubmissionAccepted(acceptance(7));

    round_trip_nota(reply, "(SubmissionAccepted 7)");
}

#[test]
fn message_daemon_configuration_round_trips_through_nota_text() {
    let configuration = MessageDaemonConfiguration {
        message_socket_path: path("/run/persona/X/message.sock"),
        message_socket_mode: SocketMode::new(0o660),
        supervision_socket_path: path("/run/persona/X/message-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        router_socket_path: path("/run/persona/X/router.sock"),
        component_ingresses: vec![ComponentMessageIngress {
            origin: InternalComponentInstanceOrigin {
                component: ComponentName::Harness,
                instance: instance("initiator"),
            },
            socket_path: path("/run/persona/X/message-ingress/initiator.sock"),
            socket_mode: SocketMode::new(0o600),
        }],
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let text = configuration.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<MessageDaemonConfiguration>()
        .expect("decode configuration");

    assert_eq!(recovered, configuration);
    assert!(text.contains("/run/persona/X/message.sock"));
    assert!(text.contains("(Harness initiator)"));
}

#[test]
fn message_daemon_configuration_round_trips_through_rkyv() {
    let configuration = MessageDaemonConfiguration {
        message_socket_path: path("/run/persona/X/message.sock"),
        message_socket_mode: SocketMode::new(0o660),
        supervision_socket_path: path("/run/persona/X/message-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        router_socket_path: path("/run/persona/X/router.sock"),
        component_ingresses: vec![ComponentMessageIngress {
            origin: InternalComponentInstanceOrigin {
                component: ComponentName::Harness,
                instance: instance("reviewer"),
            },
            socket_path: path("/run/persona/X/message-ingress/reviewer.sock"),
            socket_mode: SocketMode::new(0o600),
        }],
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = MessageDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[test]
fn unimplemented_reason_variants_are_typed_not_strings() {
    let reasons = [
        MessageUnimplementedReason::NotInPrototypeScope,
        MessageUnimplementedReason::DependencyMissing(DependencyKind::Router),
        MessageUnimplementedReason::ResourceUnavailable(ResourceKind::RouterSocket),
    ];

    for reason in reasons {
        let reply = Output::MessageRequestUnimplemented(MessageRequestUnimplemented {
            operation: MessageOperationKind::QueryInbox,
            reason,
        });
        let text = reply.to_nota();
        let recovered = NotaSource::new(&text)
            .parse::<Output>()
            .expect("decode reply");
        assert_eq!(recovered, reply);
    }
}
