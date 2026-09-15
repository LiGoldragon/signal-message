use signal_message::{
    ByteViewable, CompactReceipt, DeliveryQueueState, DeliveryQueuedAcknowledgment,
    FlowDeliveryRequest, MessageBody, MessageKind, MessageRecipient, MessageSubmission,
    PromptInterpretationSelection, PromptVariant, Query, Response, Restorable, Signal,
    Signalizable, ThreadSelection, TypedPromptEnvelope,
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
