use signal_message::{
    ByteViewable, ClusterMember, ClusterMessage, ClusterRelay, ClusterTarget, CompactReceipt,
    Context, DeliveryQueueState, DeliveryQueuedAcknowledgment, DeliveryReport, DeliveryRequest,
    FlowDeliveryRequest, FlowIdentifier, FlowIdleAcknowledgment, FlowIdleAnnouncement, MessageBody,
    MessageKind, MessageRecipient, MessageSubmission, PeerBody, PeerBodySha256, PeerEnvelope,
    PeerSender, PeerSourcePath, PromptInterpretationSelection, PromptVariant, Query, ReceiptKind,
    RecipientReceipt, Response, Restorable, Signal, Signalizable, ThreadSelection,
    TypedPromptEnvelope,
};
fn submission() -> MessageSubmission {
    MessageSubmission {
        message_recipient: MessageRecipient::from("router"),
        message_kind: MessageKind::Send,
        message_body: MessageBody::from("current signal"),
        thread_selection: ThreadSelection::None,
    }
}
fn flow_delivery_request() -> FlowDeliveryRequest {
    FlowDeliveryRequest {
        typed_prompt_envelope: TypedPromptEnvelope {
            prompt_variant: PromptVariant::PeerMessage,
            source_event_identifier: "msg-0042".to_string(),
            raw_prompt_text: "land this on the target flow".to_string(),
            prompt_interpretation_selection: PromptInterpretationSelection::None,
        },
        target_flow_name: "57a7aa".to_string(),
    }
}

fn cluster_relay() -> ClusterMessage {
    ClusterMessage::Relay(ClusterRelay {
        flow_identifier: "cf7879".to_owned(),
        session_identifier: "01a0a715".to_owned(),
        transcript_path: "/transcripts/primary.jsonl".to_owned(),
        prompt_first_six_words: "Well, fix whatever it is that".to_owned(),
        prompt_last_six_words: "it's right there in the transcript.".to_owned(),
        prompt_sha256: "98fbcb59fcbaecd28f9000aadab5439f3a4a010a8b5ad9ee97c608b16d9840d7"
            .to_owned(),
        context: Context {
            flow_identifier: "cf7879".to_owned(),
            source_turn_identifier: "turn-01".to_owned(),
            transcript_path: "/transcripts/primary.jsonl".to_owned(),
            prompt_sha256: "98fbcb59fcbaecd28f9000aadab5439f3a4a010a8b5ad9ee97c608b16d9840d7"
                .to_owned(),
            what_living_said: "Start Luna and derive the context.".to_owned(),
            context_about: "The selected request asks for transcript context.".to_owned(),
            context_answered: "The model-selection question is answered by Luna.".to_owned(),
            context_corrected: "No correction is identified from the supplied transcript."
                .to_owned(),
            context_uncertainties: vec!["The source does not settle delivery behavior.".to_owned()],
        },
        timestamp_nanos: 1_726_400_000_000_000_000,
        cluster_target: ClusterTarget::Primary,
        cluster_members: vec![ClusterMember {
            flow_identifier: "57a7aa".to_owned(),
            session_identifier: "57a7aa02-e52d-4266-8746-6770ff770d11".to_owned(),
        }],
    })
}

fn peer_message() -> ClusterMessage {
    ClusterMessage::Peer(PeerEnvelope {
        peer_sender: PeerSender {
            flow_identifier: "efa157".to_owned(),
            session_identifier: "efa15708-dc5d-42ce-af62-8ffb84c9815e".to_owned(),
        },
        source_event_identifier: "msg_01a0aa9c-b778-76d1-8b1f-2bb0d6430fb2".to_owned(),
        peer_source_path: PeerSourcePath::from("flows/efa157/log.md"),
        peer_body_sha256: PeerBodySha256::from(
            "27bed00000000000000000000000000000000000000000000000000000000000",
        ),
        peer_body: PeerBody::from("First peer line.\nSecond peer line carries the quoted body."),
    })
}
#[test]
fn message_query_and_response_restore_from_fresh_peer_bytes() {
    let query = Query::Submit(submission());
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
    let response = Response::SubmissionAccepted(41);
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}
#[test]
fn flow_deliver_query_and_two_stage_reply_restore_from_fresh_peer_bytes() {
    let query = Query::FlowDeliver(flow_delivery_request());
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);

    let queued = Response::DeliveryQueued(DeliveryQueuedAcknowledgment {
        source_event_identifier: "msg-0042".to_string(),
        target_flow_name: "57a7aa".to_string(),
        delivery_queue_state: DeliveryQueueState::Parked,
    });
    let outgoing = queued.signalize().expect("archive queued response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore queued response"), queued);

    let landed = Response::DeliveryLanded(CompactReceipt {
        source_event_identifier: "msg-0042".to_string(),
        landed_at: 1_726_400_000_000_000_000,
        byte_count: 29,
    });
    let outgoing = landed.signalize().expect("archive landed response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore landed response"), landed);
}

#[test]
fn flow_idle_announcement_and_receipts_restore_from_fresh_peer_bytes() {
    let query = Query::FlowAnnounceIdle(FlowIdleAnnouncement {
        target_flow_name: "57a7aa".to_owned(),
    });
    let outgoing = query.signalize().expect("archive idle announcement");
    assert_eq!(
        Signal::<Query>::from(outgoing.bytes().to_vec())
            .restore()
            .expect("restore idle announcement"),
        query
    );

    let reply = Response::FlowIdleAcknowledged(FlowIdleAcknowledgment {
        target_flow_name: "57a7aa".to_owned(),
        landed_receipts: vec![CompactReceipt {
            source_event_identifier: "msg-0042".to_owned(),
            landed_at: 1_726_400_000_000_000_000,
            byte_count: 29,
        }],
    });
    let outgoing = reply.signalize().expect("archive idle reply");
    assert_eq!(
        Signal::<Response>::from(outgoing.bytes().to_vec())
            .restore()
            .expect("restore idle reply"),
        reply
    );
}

#[test]
fn nexus_delivery_and_typed_receipts_restore_from_fresh_peer_bytes() {
    let request = DeliveryRequest {
        source_event_identifier: "fac697-0042".to_owned(),
        cluster_message: peer_message(),
        target_flows: vec![FlowIdentifier::from("da1e3f")],
    };
    let query = Query::Deliver(request);
    let outgoing = query.signalize().expect("archive delivery query");
    assert_eq!(
        Signal::<Query>::from(outgoing.bytes().to_vec())
            .restore()
            .expect("restore delivery query"),
        query
    );

    let reply = Response::DeliveryRecorded(DeliveryReport {
        source_event_identifier: "fac697-0042".to_owned(),
        recipient_receipts: vec![RecipientReceipt {
            flow_identifier: FlowIdentifier::from("da1e3f"),
            receipt_kind: ReceiptKind::TranscriptWitnessed,
        }],
    });
    let outgoing = reply.signalize().expect("archive delivery report");
    assert_eq!(
        Signal::<Response>::from(outgoing.bytes().to_vec())
            .restore()
            .expect("restore delivery report"),
        reply
    );
}

#[test]
fn cluster_relay_metadata_round_trips_in_the_producer_frame() {
    let relay = cluster_relay();
    let outgoing = relay.signalize().expect("archive relay");
    let restored = Signal::<ClusterMessage>::from(outgoing.bytes().to_vec())
        .restore()
        .expect("restore relay");
    assert_eq!(restored, relay);
}
#[test]
fn peer_cluster_message_round_trips_in_the_producer_frame() {
    let peer = peer_message();
    let outgoing = peer.signalize().expect("archive peer");
    let restored = Signal::<ClusterMessage>::from(outgoing.bytes().to_vec())
        .restore()
        .expect("restore peer");
    assert_eq!(restored, peer);
}
#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}
#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_message_payload() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::Submit(submission());
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, query);
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_cluster_relay_strings() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let relay = cluster_relay();
    let rendered = relay.clone().datomize(vec![]).protosize().textualize();
    assert!(
        rendered.contains('«'),
        "Datom renders string values through its guillemet form"
    );
    let mut pending = Potential::<ClusterMessage>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, relay);
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_multiline_peer_message() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let peer = peer_message();
    let rendered = peer.clone().datomize(vec![]).protosize().textualize();
    assert!(
        rendered.contains('«'),
        "Datom renders peer strings through its guillemet form"
    );
    assert!(
        rendered.contains('\n'),
        "Datom retains the multiline peer body"
    );
    let mut pending = Potential::<ClusterMessage>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, peer);
}
