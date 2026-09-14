use signal_message::{
    ByteViewable, DestinationAgentIdentifier, DispatcherProcessId, DispatcherProcessStartTime,
    MessageBody, MessageKind, MessageRecipient, MessageSubmission, PromptDeliveryHeader,
    PromptDeliveryIdentity, PromptDeliveryPayloadLength, PromptDeliveryProtocolVersion, Query,
    Response, Restorable, Signal, Signalizable, SourceAgentIdentifier, SourceEventIdentifier,
    ThreadSelection,
};
fn submission() -> MessageSubmission {
    MessageSubmission {
        message_recipient: MessageRecipient::from("router"),
        message_kind: MessageKind::Send,
        message_body: MessageBody::from("current signal"),
        thread_selection: ThreadSelection::None,
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

#[test]
fn two_phase_prompt_delivery_contract_restores_from_fresh_peer_bytes() {
    let identity = PromptDeliveryIdentity {
        source_agent_identifier: SourceAgentIdentifier::from("sandbox-hook"),
        destination_agent_identifier: DestinationAgentIdentifier::from("codex"),
        source_event_identifier: SourceEventIdentifier::from("event-42"),
    };
    let header = PromptDeliveryHeader {
        prompt_delivery_protocol_version: PromptDeliveryProtocolVersion::from(1),
        prompt_delivery_identity: identity.clone(),
        prompt_delivery_payload_length: PromptDeliveryPayloadLength::from(73),
        dispatcher_process_id: DispatcherProcessId::from(4242),
        dispatcher_process_start_time: DispatcherProcessStartTime::from(9001),
    };
    let query = Query::Header(header);
    let outgoing = query.signalize().expect("archive two-phase header");
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore two-phase header"), query);

    let query = Query::Reconcile(identity.clone());
    let outgoing = query.signalize().expect("archive reconciliation");
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore reconciliation"), query);

    for response in [
        Response::HeaderAccepted(identity.clone()),
        Response::HeaderRejected(identity.clone()),
        Response::PayloadAbsent(identity.clone()),
        Response::PayloadInProgress(identity.clone()),
        Response::PayloadComplete(identity.clone()),
        Response::RecipientObserved(identity.clone()),
    ] {
        let outgoing = response.signalize().expect("archive two-phase reply");
        let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
        assert_eq!(
            incoming.restore().expect("restore two-phase reply"),
            response
        );
    }
}
