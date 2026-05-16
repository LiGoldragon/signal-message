//! Architectural-truth tests for the
//! `signal-persona-message` channel.
//!
//! Each test names exactly what shape it pins down; per the
//! "blunt test names" convention.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_core::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalVerb, SubReply,
};
use signal_persona::{SocketMode, TimestampNanos, WirePath};
use signal_persona_auth::{ConnectionClass, MessageOrigin, OwnerIdentity, UnixUserId};
use signal_persona_message::{
    DependencyKind, Frame, FrameBody, InboxEntry, InboxListing, InboxQuery, MessageBody,
    MessageDaemonConfiguration, MessageKind, MessageOperationKind, MessageRecipient, MessageReply,
    MessageRequest, MessageRequestUnimplemented, MessageSender, MessageSlot, MessageSubmission,
    MessageUnimplementedReason, ResourceKind, StampedMessageSubmission, SubmissionAcceptance,
    SubmissionRejection, SubmissionRejectionReason,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn request_frame(request: MessageRequest) -> Frame {
    Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    })
}

fn reply_frame(reply: MessageReply) -> Frame {
    Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::completed(NonEmpty::single(SubReply::Ok {
            verb: SignalVerb::Assert,
            payload: reply,
        })),
    })
}

fn decode_single_reply(frame: Frame) -> MessageReply {
    match frame.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

#[test]
fn message_submission_request_round_trips_through_length_prefixed_frame() {
    let request = MessageRequest::MessageSubmission(MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        kind: MessageKind::Send,
        body: MessageBody::new("stack test"),
    });
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            let operation = decoded_request.operations().head();
            assert_eq!(operation.verb, SignalVerb::Assert);
            assert_eq!(request.signal_verb(), SignalVerb::Assert);
            assert_eq!(operation.payload, request);
        }
        other => panic!("expected Assert request, got {other:?}"),
    }
}

#[test]
fn stamped_message_submission_request_round_trips_through_length_prefixed_frame() {
    let request = MessageRequest::StampedMessageSubmission(StampedMessageSubmission {
        submission: MessageSubmission {
            recipient: MessageRecipient::new("designer"),
            kind: MessageKind::Send,
            body: MessageBody::new("stack test"),
        },
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
            let operation = decoded_request.operations().head();
            assert_eq!(operation.verb, SignalVerb::Assert);
            assert_eq!(request.signal_verb(), SignalVerb::Assert);
            assert_eq!(operation.payload, request);
        }
        other => panic!("expected Assert request, got {other:?}"),
    }
}

#[test]
fn inbox_query_round_trips_through_length_prefixed_frame() {
    let request = MessageRequest::InboxQuery(InboxQuery {
        recipient: MessageRecipient::new("designer"),
    });
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            let operation = decoded_request.operations().head();
            assert_eq!(operation.verb, SignalVerb::Match);
            assert_eq!(request.signal_verb(), SignalVerb::Match);
            assert_eq!(operation.payload, request);
        }
        other => panic!("expected Match request, got {other:?}"),
    }
}

#[test]
fn submission_accepted_reply_round_trips() {
    let reply = MessageReply::SubmissionAccepted(SubmissionAcceptance {
        message_slot: MessageSlot::new(1024),
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn submission_rejected_reply_round_trips_with_typed_reason() {
    let reply = MessageReply::SubmissionRejected(SubmissionRejection {
        reason: SubmissionRejectionReason::RecipientNotFound,
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn unimplemented_reply_round_trips_with_typed_reason() {
    let reply = MessageReply::MessageRequestUnimplemented(MessageRequestUnimplemented {
        operation: MessageOperationKind::StampedMessageSubmission,
        reason: MessageUnimplementedReason::ResourceUnavailable(ResourceKind::PeerCredentials),
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn inbox_listing_round_trips_through_length_prefixed_frame() {
    let reply = MessageReply::InboxListing(InboxListing {
        messages: vec![
            InboxEntry {
                message_slot: MessageSlot::new(1),
                sender: MessageSender::new("operator"),
                body: MessageBody::new("first"),
            },
            InboxEntry {
                message_slot: MessageSlot::new(2),
                sender: MessageSender::new("operator"),
                body: MessageBody::new("second"),
            },
        ],
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn payload_constructor_lifts_message_submission_into_request() {
    let payload = MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        kind: MessageKind::Send,
        body: MessageBody::new("via from"),
    };
    let request = MessageRequest::MessageSubmission(payload.clone());
    assert_eq!(request, MessageRequest::MessageSubmission(payload));
}

#[test]
fn payload_constructor_lifts_submission_acceptance_into_reply() {
    let acceptance = SubmissionAcceptance {
        message_slot: MessageSlot::new(7),
    };
    let reply = MessageReply::SubmissionAccepted(acceptance.clone());
    assert_eq!(reply, MessageReply::SubmissionAccepted(acceptance));
}

#[test]
fn message_request_exposes_contract_owned_operation_kind() {
    let submission = MessageRequest::MessageSubmission(MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        kind: MessageKind::Send,
        body: MessageBody::new("kind witness"),
    });
    let stamped = MessageRequest::StampedMessageSubmission(StampedMessageSubmission {
        submission: MessageSubmission {
            recipient: MessageRecipient::new("designer"),
            kind: MessageKind::Send,
            body: MessageBody::new("kind witness"),
        },
        origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(1),
    });
    let inbox = MessageRequest::InboxQuery(InboxQuery {
        recipient: MessageRecipient::new("designer"),
    });

    assert_eq!(
        submission.operation_kind(),
        MessageOperationKind::MessageSubmission
    );
    assert_eq!(
        stamped.operation_kind(),
        MessageOperationKind::StampedMessageSubmission
    );
    assert_eq!(inbox.operation_kind(), MessageOperationKind::InboxQuery);
}

#[test]
fn message_request_variants_declare_expected_signal_root_verbs() {
    let submission = MessageRequest::MessageSubmission(MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        kind: MessageKind::Send,
        body: MessageBody::new("verb witness"),
    });
    let stamped = MessageRequest::StampedMessageSubmission(StampedMessageSubmission {
        submission: MessageSubmission {
            recipient: MessageRecipient::new("designer"),
            kind: MessageKind::Send,
            body: MessageBody::new("verb witness"),
        },
        origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(1),
    });
    let inbox = MessageRequest::InboxQuery(InboxQuery {
        recipient: MessageRecipient::new("designer"),
    });

    assert_eq!(submission.signal_verb(), SignalVerb::Assert);
    assert_eq!(stamped.signal_verb(), SignalVerb::Assert);
    assert_eq!(inbox.signal_verb(), SignalVerb::Match);
}

#[test]
fn message_operation_kind_round_trips_through_nota_text() {
    let mut encoder = Encoder::new();
    MessageOperationKind::MessageSubmission
        .encode(&mut encoder)
        .expect("encode operation kind");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageOperationKind::decode(&mut decoder).expect("decode operation kind");

    assert_eq!(recovered, MessageOperationKind::MessageSubmission);
    assert_eq!(text, "MessageSubmission");
}

#[test]
fn message_submission_request_round_trips_through_nota_text() {
    let request = MessageRequest::MessageSubmission(MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        kind: MessageKind::Send,
        body: MessageBody::new("stack test"),
    });

    let mut encoder = Encoder::new();
    request.encode(&mut encoder).expect("encode request");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageRequest::decode(&mut decoder).expect("decode request");

    assert_eq!(recovered, request);
    assert_eq!(text, "(MessageSubmission designer Send \"stack test\")");
}

#[test]
fn stamped_message_submission_request_round_trips_through_nota_text() {
    let request = MessageRequest::StampedMessageSubmission(StampedMessageSubmission {
        submission: MessageSubmission {
            recipient: MessageRecipient::new("designer"),
            kind: MessageKind::Send,
            body: MessageBody::new("stack test"),
        },
        origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(99),
    });

    let mut encoder = Encoder::new();
    request.encode(&mut encoder).expect("encode request");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageRequest::decode(&mut decoder).expect("decode request");

    assert_eq!(recovered, request);
    assert!(text.contains("StampedMessageSubmission"));
    assert!(text.contains("Owner"));
}

#[test]
fn submission_accepted_reply_round_trips_through_nota_text() {
    let reply = MessageReply::SubmissionAccepted(SubmissionAcceptance {
        message_slot: MessageSlot::new(7),
    });

    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode reply");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageReply::decode(&mut decoder).expect("decode reply");

    assert_eq!(recovered, reply);
    assert_eq!(text, "(SubmissionAcceptance 7)");
}

#[test]
fn message_daemon_configuration_round_trips_through_nota_text() {
    let configuration = MessageDaemonConfiguration {
        message_socket_path: WirePath::new("/run/persona/X/message.sock"),
        message_socket_mode: SocketMode::new(0o660),
        supervision_socket_path: WirePath::new("/run/persona/X/message-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        owner_identity: OwnerIdentity::UnixUser(UnixUserId::new(1000)),
    };

    let mut encoder = Encoder::new();
    configuration.encode(&mut encoder).expect("encode configuration");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageDaemonConfiguration::decode(&mut decoder).expect("decode configuration");

    assert_eq!(recovered, configuration);
}

#[test]
fn message_daemon_configuration_round_trips_through_rkyv() {
    use nota_config::ConfigurationRecord;

    let configuration = MessageDaemonConfiguration {
        message_socket_path: WirePath::new("/run/persona/X/message.sock"),
        message_socket_mode: SocketMode::new(0o660),
        supervision_socket_path: WirePath::new("/run/persona/X/message-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        owner_identity: OwnerIdentity::UnixUser(UnixUserId::new(1000)),
    };

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("archive");
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
        let reply = MessageReply::MessageRequestUnimplemented(MessageRequestUnimplemented {
            operation: MessageOperationKind::InboxQuery,
            reason,
        });
        let mut encoder = Encoder::new();
        reply.encode(&mut encoder).expect("encode reply");
        let text = encoder.into_string();
        let mut decoder = Decoder::new(&text);
        let recovered = MessageReply::decode(&mut decoder).expect("decode reply");
        assert_eq!(recovered, reply);
    }
}
