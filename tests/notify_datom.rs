#![cfg(feature = "datom")]

use datom_codec::{Datomizable};
use protos::{Protosizable, Textualizable};
use signal_message::{notify::{parse_one, NotifyTextError}, Notify, NotifyEnvelope};

fn parse(value: Notify) -> Notify {
    let text = NotifyEnvelope::Notify(value.clone()).datomize(vec![]).protosize().textualize();
    parse_one(&[text]).expect("validated Notify").0
}

#[test]
fn producer_notify_round_trips_escaped_text() {
    let notify = Notify { notify_recipient: "bob@example.org".into(), notify_body: "hello «quoted» text".into() };
    assert_eq!(parse(notify.clone()), notify);
}

#[test]
fn notify_rejects_another_contract_type_and_bounds() {
    assert_eq!(parse_one(&["Submit.{ x }".into()]), Err(NotifyTextError::Malformed));
    assert_eq!(parse_one(&[]), Err(NotifyTextError::ArgumentCount(0)));
    assert_eq!(parse_one(&["Notify.{}".into(), "Notify.{}".into()]), Err(NotifyTextError::ArgumentCount(2)));
    for recipient in ["@example.org", "bob@", "@", "bob @example.org", "bob@example.org/resource"] {
        let notify = Notify { notify_recipient: recipient.into(), notify_body: "body".into() };
        let text = NotifyEnvelope::Notify(notify).datomize(vec![]).protosize().textualize();
        assert_eq!(parse_one(&[text]), Err(NotifyTextError::Recipient));
    }
    let oversized = Notify { notify_recipient: "bob@example.org".into(), notify_body: "x".repeat(1025) };
    let text = NotifyEnvelope::Notify(oversized).datomize(vec![]).protosize().textualize();
    assert_eq!(parse_one(&[text]), Err(NotifyTextError::Body));
    let unicode_oversized = Notify { notify_recipient: "bob@example.org".into(), notify_body: "é".repeat(513) };
    let text = NotifyEnvelope::Notify(unicode_oversized).datomize(vec![]).protosize().textualize();
    assert_eq!(parse_one(&[text]), Err(NotifyTextError::Body));
    assert_eq!(parse_one(&["x".repeat(4097)]), Err(NotifyTextError::InputTooLarge));
    assert_eq!(parse_one(&["Notify.{ bob@example.org «body» } trailing".into()]), Err(NotifyTextError::Malformed));
}
