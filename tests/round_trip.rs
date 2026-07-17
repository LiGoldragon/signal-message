//! Architectural-truth tests for the
//! `signal-message` channel.
//!
//! Each test names exactly what shape it pins down; per the
//! "blunt test names" convention.

use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_message::{
    AgentDeathMark, AgentEndpoint, AgentEndpointBinding, AgentEndpointKind, AgentIdentifier,
    AgentIdentityAssignment, AgentRegistryEntry, AgentRegistryListing, AgentRegistryRejection,
    AgentRegistryRejectionReason, AssignedAgentIdentity, ComponentInstanceName,
    ComponentMessageIngress, ComponentName, ConnectionClass, DependencyKind, EndpointPath,
    EndpointSelection, HarnessPid, HarnessProcessPin, HarnessStartTime, IdentityProvenance,
    ProcessPinSelection, ResumeIdentity,
    ResumeSelection,
    Frame, FrameBody, InboxEntry, InboxListing, InboxQuery, Input, InternalComponentInstanceOrigin,
    MessageBody, MessageDaemonConfiguration, MessageDaemonConfigurationParts, MessageKind,
    MessageOperationKind, MessageOrigin, MessageRecipient, MessageRequestUnimplemented,
    MessageSender, MessageSlot, MessageSubmission, MessageUnimplementedReason, Output,
    OwnerIdentity, ResourceKind, SocketMode, StampedMessageSubmission, SubmissionAcceptance,
    SubmissionRejection, SubmissionRejectionReason, ThreadName, ThreadSelection, TimestampNanos,
    UnixUserIdentifier, WirePath,
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
        message_recipient: recipient(recipient_value),
        message_kind: MessageKind::Send,
        message_body: body(body_value),
        thread_selection: ThreadSelection::None,
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
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("stack test"),
        thread_selection: ThreadSelection::None,
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
        message_submission: submission("designer", "stack test"),
        message_origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(42).into(),
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
        message_operation_kind: MessageOperationKind::SubmitStamped,
        message_unimplemented_reason: MessageUnimplementedReason::ResourceUnavailable(
            ResourceKind::PeerCredentials,
        ),
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn inbox_listing_round_trips_through_length_prefixed_frame() {
    let reply = Output::InboxListing(InboxListing::from_entries(vec![
        InboxEntry {
            message_slot: MessageSlot::new(1),
            message_sender: sender("operator"),
            message_body: body("first"),
        },
        InboxEntry {
            message_slot: MessageSlot::new(2),
            message_sender: sender("operator"),
            message_body: body("second"),
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
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("via from"),
        thread_selection: ThreadSelection::None,
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
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("kind witness"),
        thread_selection: ThreadSelection::None,
    });
    let stamped = Input::SubmitStamped(StampedMessageSubmission {
        message_submission: submission("designer", "kind witness"),
        message_origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(1).into(),
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
        &[
            "Submit",
            "SubmitStamped",
            "QueryInbox",
            "AssignAgentIdentity",
            "BindAgentEndpoint",
            "QueryAgentRegistry",
        ]
    );
}

#[test]
fn message_operation_kind_round_trips_through_nota_text() {
    round_trip_nota(MessageOperationKind::Submit, "Submit");
}

#[test]
fn message_submission_request_round_trips_through_nota_text() {
    let request = Input::Submit(MessageSubmission {
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("stack test"),
        thread_selection: ThreadSelection::None,
    });

    round_trip_nota(request, "(Submit (designer Send [stack test] None))");
}

#[test]
fn message_submission_without_explicit_thread_round_trips_as_typed_none_slot() {
    let submission = MessageSubmission {
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("no thread"),
        thread_selection: ThreadSelection::None,
    };

    round_trip_nota(submission, "(designer Send [no thread] None)");
}

#[test]
fn message_submission_with_named_thread_round_trips_through_length_prefixed_frame() {
    let request = Input::Submit(MessageSubmission {
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("thread test"),
        thread_selection: ThreadSelection::Named(ThreadName::new(text("launch-plan"))),
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
fn message_submission_with_named_thread_round_trips_through_nota_text() {
    let submission = MessageSubmission {
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("thread test"),
        thread_selection: ThreadSelection::Named(ThreadName::new(text("launch-plan"))),
    };

    round_trip_nota(
        submission,
        "(designer Send [thread test] (Named launch-plan))",
    );
}

#[test]
fn stamped_message_submission_request_round_trips_through_nota_text() {
    let request = Input::SubmitStamped(StampedMessageSubmission {
        message_submission: submission("designer", "stack test"),
        message_origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(99).into(),
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
    let configuration = MessageDaemonConfiguration::from(MessageDaemonConfigurationParts {
        message_socket_path: path("/run/persona/X/message.sock"),
        message_socket_mode: SocketMode::new(0o660),
        supervision_socket_path: path("/run/persona/X/message-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        router_socket_path: path("/run/persona/X/router.sock"),
        component_ingresses: vec![ComponentMessageIngress {
            internal_component_instance_origin: InternalComponentInstanceOrigin {
                component_name: ComponentName::Harness,
                component_instance_name: instance("initiator"),
            },
            ingress_socket_path: path("/run/persona/X/message-ingress/initiator.sock").into(),
            socket_mode: SocketMode::new(0o600),
        }],
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    });

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
    let configuration = MessageDaemonConfiguration::from(MessageDaemonConfigurationParts {
        message_socket_path: path("/run/persona/X/message.sock"),
        message_socket_mode: SocketMode::new(0o660),
        supervision_socket_path: path("/run/persona/X/message-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        router_socket_path: path("/run/persona/X/router.sock"),
        component_ingresses: vec![ComponentMessageIngress {
            internal_component_instance_origin: InternalComponentInstanceOrigin {
                component_name: ComponentName::Harness,
                component_instance_name: instance("reviewer"),
            },
            ingress_socket_path: path("/run/persona/X/message-ingress/reviewer.sock").into(),
            socket_mode: SocketMode::new(0o600),
        }],
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    });

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
            message_operation_kind: MessageOperationKind::QueryInbox,
            message_unimplemented_reason: reason,
        });
        let text = reply.to_nota();
        let recovered = NotaSource::new(&text)
            .parse::<Output>()
            .expect("decode reply");
        assert_eq!(recovered, reply);
    }
}

#[test]
fn assign_agent_identity_request_round_trips_through_length_prefixed_frame() {
    let request = Input::AssignAgentIdentity(AgentIdentityAssignment {
        agent_identifier: AgentIdentifier::new(text("x7f2")),
        process_pin_selection: ProcessPinSelection::Pinned(HarnessProcessPin {
            harness_pid: HarnessPid::new(4242),
            harness_start_time: HarnessStartTime::new(987654321),
        }),
        resume_selection: ResumeSelection::Resumed(ResumeIdentity::new(text("session-abc123"))),
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
        other => panic!("expected AssignAgentIdentity request, got {other:?}"),
    }
}

#[test]
fn assigned_agent_identity_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::AgentIdentityAssigned(AssignedAgentIdentity {
        agent_identifier: AgentIdentifier::new(text("x7f2")),
        identity_provenance: IdentityProvenance::Seated,
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn bind_agent_endpoint_request_round_trips_through_length_prefixed_frame() {
    let request = Input::BindAgentEndpoint(AgentEndpointBinding {
        agent_identifier: AgentIdentifier::new(text("x7f2")),
        agent_endpoint: AgentEndpoint {
            agent_endpoint_kind: AgentEndpointKind::PtySocket,
            endpoint_path: EndpointPath::new(path("/run/terminal-cell/session-a/data.sock")),
        },
        harness_pid: HarnessPid::new(4242),
        harness_start_time: HarnessStartTime::new(987654321),
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
        other => panic!("expected BindAgentEndpoint request, got {other:?}"),
    }
}

#[test]
fn agent_registry_listing_reply_round_trips_with_every_field_populated() {
    let reply = Output::AgentRegistryListing(AgentRegistryListing::from_entries(vec![
        AgentRegistryEntry {
            agent_identifier: AgentIdentifier::new(text("x7f2")),
            endpoint_selection: EndpointSelection::Bound(AgentEndpoint {
                agent_endpoint_kind: AgentEndpointKind::HarnessSocket,
                endpoint_path: EndpointPath::new(path("/run/harness/harness.sock")),
            }),
            resume_selection: ResumeSelection::Resumed(ResumeIdentity::new(text("session-abc"))),
            agent_death_mark: AgentDeathMark::NotDead,
            process_pin_selection: ProcessPinSelection::Pinned(HarnessProcessPin {
                harness_pid: HarnessPid::new(11),
                harness_start_time: HarnessStartTime::new(22),
            }),
        },
        AgentRegistryEntry {
            agent_identifier: AgentIdentifier::new(text("9k4w")),
            endpoint_selection: EndpointSelection::None,
            resume_selection: ResumeSelection::None,
            agent_death_mark: AgentDeathMark::Killed,
            process_pin_selection: ProcessPinSelection::None,
        },
    ]));
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn agent_registry_rejection_reasons_are_typed_not_strings() {
    let reasons = [
        AgentRegistryRejectionReason::UnknownAgentIdentifier,
        AgentRegistryRejectionReason::StoreRejected,
    ];

    for reason in reasons {
        let reply = Output::AgentRegistryRejected(AgentRegistryRejection::new(reason));
        let text = reply.to_nota();
        let recovered = NotaSource::new(&text)
            .parse::<Output>()
            .expect("decode reply");
        assert_eq!(recovered, reply);
    }
}
