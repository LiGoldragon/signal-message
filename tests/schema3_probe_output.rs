#[cfg(feature = "datom")]
use signal_message::Schema3ProbeQuery;
use signal_message::{
    ByteViewable, Restorable, Schema3ProbeCounts, Schema3ProbeOutcome, Schema3ProbeRefusal, Signal,
    Signalizable,
};

fn counts() -> Schema3ProbeCounts {
    Schema3ProbeCounts {
        agent_registry_count: 11,
        message_ledger_count: 13,
        ledger_head_count: 17,
        recipient_inbox_count: 19,
        thread_index_count: 23,
        delivery_outbox_count: 29,
    }
}

#[test]
fn schema3_probe_observation_round_trips_through_fresh_peer_bytes() {
    let outcome = Schema3ProbeOutcome::Observed(counts());
    let outgoing = outcome.signalize().expect("archive observation");
    assert!(!outgoing.bytes().is_empty());
    assert_eq!(
        Signal::<Schema3ProbeOutcome>::from(outgoing.bytes().to_vec())
            .restore()
            .expect("restore observation"),
        outcome
    );
}

#[test]
fn schema3_probe_refusal_round_trips_without_store_payload() {
    let outcome = Schema3ProbeOutcome::Refused(Schema3ProbeRefusal::LegacyEngineOpenOrSchema);
    let outgoing = outcome.signalize().expect("archive refusal");
    assert!(!outgoing.bytes().is_empty());
    assert_eq!(
        Signal::<Schema3ProbeOutcome>::from(outgoing.bytes().to_vec())
            .restore()
            .expect("restore refusal"),
        outcome
    );
}

#[test]
fn every_schema3_failure_stage_round_trips_without_payload() {
    use Schema3ProbeRefusal::*;
    let refusals = [
        InputNotRegularFile, InputUnreadable, PrivateCopyUnavailable, SourceChanged,
        LegacyEngineOpenOrSchema, LegacyAgentRegistryRegistration,
        LegacyMessageLedgerRegistration, LegacyLedgerHeadRegistration,
        LegacyRecipientInboxRegistration, LegacyThreadIndexRegistration,
        LegacyDeliveryOutboxRegistration, LegacyAgentRegistryDecode,
        LegacyMessageLedgerDecode, LegacyLedgerHeadDecode, LegacyRecipientInboxDecode,
        LegacyThreadIndexDecode, LegacyDeliveryOutboxDecode, LegacyLedgerInvariant,
        LegacyReferenceInvariant, LegacyDecoderPanic, PrivateCleanup,
    ];
    for refusal in refusals {
        let outcome = Schema3ProbeOutcome::Refused(refusal);
        let outgoing = outcome.signalize().expect("archive typed refusal");
        assert_eq!(
            Signal::<Schema3ProbeOutcome>::from(outgoing.bytes().to_vec())
                .restore()
                .expect("restore typed refusal"),
            outcome
        );
    }
}

#[cfg(feature = "datom")]
#[test]
fn schema3_probe_query_actualizes_one_complete_datom_argument() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let mut pending = Potential::<Schema3ProbeQuery>::from("Inspect.«/abs/copied.sema»");
    let query = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize complete probe argument");
    assert_eq!(
        query,
        Schema3ProbeQuery::Inspect("/abs/copied.sema".to_owned())
    );
    assert_eq!(
        query.datomize(vec![]).protosize().textualize(),
        "Inspect./abs/copied.sema"
    );
}

#[cfg(feature = "datom")]
#[test]
fn schema3_probe_outcomes_have_typed_datom_without_store_payload() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let observed = Schema3ProbeOutcome::Observed(counts());
    let rendered = observed.clone().datomize(vec![]).protosize().textualize();
    assert_eq!(rendered, "Observed.{ 11 13 17 19 23 29 }");
    let mut pending = Potential::<Schema3ProbeOutcome>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize observation");
    assert_eq!(restored, observed);

    let refusal = Schema3ProbeOutcome::Refused(Schema3ProbeRefusal::LegacyEngineOpenOrSchema);
    let rendered = refusal.clone().datomize(vec![]).protosize().textualize();
    assert_eq!(rendered, "Refused.LegacyEngineOpenOrSchema");
    let mut pending = Potential::<Schema3ProbeOutcome>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize refusal");
    assert_eq!(restored, refusal);
}
