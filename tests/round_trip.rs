//! Architectural-truth tests for the
//! `signal-message` channel.
//!
//! Each test names exactly what shape it pins down; per the
//! "blunt test names" convention.

use dotos::{DotosDecode, DotosEncode, DotosSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, RootCode,
    SessionEpoch, SignalOperationHeads, SubReply, VariantCode, WireRoute,
};
use signal_message::{
    AgentDeathMark, AgentEndpoint, AgentEndpointBinding, AgentEndpointKind, AgentIdentifier,
    AgentIdentityAssignment, AgentRegistryEntry, AgentRegistryListing, AgentRegistryRejection,
    AgentRegistryRejectionReason, AssignedAgentIdentity, ComponentInstanceName,
    ComponentMessageIngress, ComponentName, ConnectionClass, DependencyKind, EndpointPath,
    EndpointSelection, Frame, FrameBody, HarnessPid, HarnessProcessPin, HarnessStartTime,
    IdentityProvenance, InboxEntry, InboxListing, InboxQuery, Input,
    InternalComponentInstanceOrigin, MessageBody, MessageDaemonConfiguration,
    MessageDaemonConfigurationParts, MessageKind, MessageOperationKind, MessageOrigin,
    MessageRecipient, MessageRequestUnimplemented, MessageSender, MessageSlot, MessageSubmission,
    MessageUnimplementedReason, Output, OwnerIdentity, ParticipantName, Participants,
    ProcessPinSelection, ResourceKind, ResumeIdentity, ResumeSelection, SocketMode, StampedAt,
    StampedMessageSubmission, SubmissionAcceptance, SubmissionRejection, SubmissionRejectionReason,
    ThreadContents, ThreadEntries, ThreadEntry, ThreadIndexEntries, ThreadIndexQuery, ThreadName,
    ThreadQuery, ThreadRejection, ThreadRejectionReason, ThreadRelation, ThreadRelationSelection,
    ThreadSelection, ThreadSubscription, ThreadSubscriptionAcknowledgment, ThreadSummary,
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
    Frame::new(
        WireRoute::new(RootCode::new(0), VariantCode::new(0)),
        FrameBody::Request {
            exchange: exchange(),
            request: request.into_request(),
        },
    )
}

fn reply_frame(reply: Output) -> Frame {
    Frame::new(
        WireRoute::new(RootCode::new(0), VariantCode::new(0)),
        FrameBody::Reply {
            exchange: exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
        },
    )
}

fn decode_single_request(frame: Frame) -> Input {
    match frame.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
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

fn round_trip_dotos<Value>(value: Value, expected: &str)
where
    Value: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_dotos();
    assert_eq!(text, expected);
    let recovered = DotosSource::new(&text).parse::<Value>().expect("decode");
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
            thread_selection: ThreadSelection::None,
            stamped_at: StampedAt::new(TimestampNanos::new(11)),
        },
        InboxEntry {
            message_slot: MessageSlot::new(2),
            message_sender: sender("operator"),
            message_body: body("second"),
            thread_selection: ThreadSelection::Named(ThreadName::new("triage".to_owned())),
            stamped_at: StampedAt::new(TimestampNanos::new(12)),
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
            "QueryThread",
            "SubscribeThread",
            "QueryThreads",
        ]
    );
}

#[test]
fn message_operation_kind_round_trips_through_dotos_text() {
    round_trip_dotos(MessageOperationKind::Submit, "Submit");
}

#[test]
fn message_submission_request_round_trips_through_dotos_text() {
    let request = Input::Submit(MessageSubmission {
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("stack test"),
        thread_selection: ThreadSelection::None,
    });

    round_trip_dotos(request, "Submit.{designer Send (stack test) None}");
}

#[test]
fn message_submission_without_explicit_thread_round_trips_as_typed_none_slot() {
    let submission = MessageSubmission {
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("no thread"),
        thread_selection: ThreadSelection::None,
    };

    round_trip_dotos(submission, "{designer Send (no thread) None}");
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
fn message_submission_with_named_thread_round_trips_through_dotos_text() {
    let submission = MessageSubmission {
        message_recipient: recipient("designer"),
        message_kind: MessageKind::Send,
        message_body: body("thread test"),
        thread_selection: ThreadSelection::Named(ThreadName::new(text("launch-plan"))),
    };

    round_trip_dotos(
        submission,
        "{designer Send (thread test) Named.launch-plan}",
    );
}

#[test]
fn stamped_message_submission_request_round_trips_through_dotos_text() {
    let request = Input::SubmitStamped(StampedMessageSubmission {
        message_submission: submission("designer", "stack test"),
        message_origin: MessageOrigin::External(ConnectionClass::Owner),
        stamped_at: TimestampNanos::new(99).into(),
    });

    let text = request.to_dotos();
    let recovered = DotosSource::new(&text)
        .parse::<Input>()
        .expect("decode request");

    assert_eq!(recovered, request);
    assert!(text.contains("SubmitStamped"));
    assert!(text.contains("Owner"));
}

#[test]
fn submission_accepted_reply_round_trips_through_dotos_text() {
    let reply = Output::SubmissionAccepted(acceptance(7));

    round_trip_dotos(reply, "SubmissionAccepted.7");
}

#[test]
fn message_daemon_configuration_round_trips_through_dotos_text() {
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

    let text = configuration.to_dotos();
    let recovered = DotosSource::new(&text)
        .parse::<MessageDaemonConfiguration>()
        .expect("decode configuration");

    assert_eq!(recovered, configuration);
    assert!(text.contains("/run/persona/X/message.sock"));
    assert!(text.contains("{Harness initiator}"));
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
        let text = reply.to_dotos();
        let recovered = DotosSource::new(&text)
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
        let text = reply.to_dotos();
        let recovered = DotosSource::new(&text)
            .parse::<Output>()
            .expect("decode reply");
        assert_eq!(recovered, reply);
    }
}

fn participant(value: &str) -> ParticipantName {
    ParticipantName::new(value.to_owned())
}

fn related_thread(name: &str) -> ThreadContents {
    ThreadContents {
        thread_name: ThreadName::new(name.to_owned()),
        thread_relation_selection: ThreadRelationSelection::Related(ThreadRelation {
            repository_name: signal_message::RepositoryName::new("orchestrate".to_owned()),
            feature_branch_name: signal_message::FeatureBranchName::new(
                "MessengerPromotion".to_owned(),
            ),
        }),
        participants: Participants::new(vec![participant("li7f"), participant("x2qb")]),
        thread_entries: ThreadEntries::new(vec![ThreadEntry {
            message_slot: MessageSlot::new(4),
            message_sender: sender("li7f"),
            message_body: body("worktree scaffolded"),
            stamped_at: StampedAt::new(TimestampNanos::new(99)),
        }]),
    }
}

#[test]
fn thread_query_round_trips_through_length_prefixed_frame() {
    let request = Input::QueryThread(ThreadQuery::new(ThreadName::new("triage".to_owned())));
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_request(decoded), request);
}

#[test]
fn thread_subscription_round_trips_with_relation() {
    let request = Input::SubscribeThread(ThreadSubscription {
        thread_name: ThreadName::new("subagents".to_owned()),
        participant_name: participant("li7f"),
        thread_relation_selection: ThreadRelationSelection::None,
    });
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_request(decoded), request);
}

#[test]
fn thread_listing_round_trips_with_relation_participants_and_entries() {
    let reply = Output::ThreadListing(related_thread("MessengerPromotion"));
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn thread_index_listing_round_trips_with_summaries() {
    let reply = Output::ThreadIndexListing(ThreadIndexEntries::from_threads(vec![ThreadSummary {
        thread_name: ThreadName::new("triage".to_owned()),
        thread_relation_selection: ThreadRelationSelection::None,
        participants: Participants::new(vec![participant("li7f")]),
        message_count: signal_message::MessageCount::new(3),
    }]));
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}

#[test]
fn thread_rejection_and_error_replies_are_typed() {
    let rejection =
        Output::ThreadRejected(ThreadRejection::new(ThreadRejectionReason::UnknownThread));
    let error = Output::Error(signal_message::ErrorReport::new(
        signal_message::ErrorMessage::new("store rejected the write".to_owned()),
    ));

    for reply in [rejection, error] {
        let frame = reply_frame(reply.clone());
        let bytes = frame.encode_length_prefixed().expect("encode");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
        assert_eq!(decode_single_reply(decoded), reply);
    }
}

#[test]
fn thread_index_query_round_trips_bare() {
    let request = Input::QueryThreads(ThreadIndexQuery::All);
    let frame = request_frame(request.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_request(decoded), request);
}

#[test]
fn thread_subscription_acknowledgment_round_trips() {
    let reply = Output::ThreadSubscribed(ThreadSubscriptionAcknowledgment {
        thread_name: ThreadName::new("subagents".to_owned()),
        participant_name: participant("li7f"),
    });
    let frame = reply_frame(reply.clone());

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    assert_eq!(decode_single_reply(decoded), reply);
}
