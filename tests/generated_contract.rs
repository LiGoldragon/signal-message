use signal_message::{
    ByteViewable, ClusterMember, ClusterMessage, ClusterRelay, ClusterTarget, CompactReceipt, Context,
    DeliveryQueueState, DeliveryQueuedAcknowledgment, FlowDeliveryRequest, FlowIdleAcknowledgment,
    FlowIdleAnnouncement, MessageBody,
    MessageKind, MessageRecipient, MessageSubmission, PromptInterpretationSelection,
    PromptVariant, Query, Response, Restorable, Signal, Signalizable, ThreadSelection,
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
        prompt_sha256: "5350d56d15c2a6a8b240f3385a5f941a4c53f4c9b91272070bbb146620bfc28c"
            .to_owned(),
        context: Context {
            flow_identifier: "cf7879".to_owned(),
            source_turn_identifier: "turn-01".to_owned(),
            transcript_path: "/transcripts/primary.jsonl".to_owned(),
            prompt_sha256: "5350d56d15c2a6a8b240f3385a5f941a4c53f4c9b91272070bbb146620bfc28c".to_owned(),
            what_living_said: "Start Luna and derive the context.".to_owned(),
            context_about: "The selected request asks for transcript context.".to_owned(),
            context_answered: "The model-selection question is answered by Luna.".to_owned(),
            context_corrected: "No correction is identified from the supplied transcript.".to_owned(),
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
fn cluster_relay_metadata_round_trips_in_the_producer_frame() {
    let relay = cluster_relay();
    let outgoing = relay.signalize().expect("archive relay");
    let restored = Signal::<ClusterMessage>::from(outgoing.bytes().to_vec())
        .restore()
        .expect("restore relay");
    assert_eq!(restored, relay);
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
    assert!(rendered.contains('«'), "Datom renders string values through its guillemet form");
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
