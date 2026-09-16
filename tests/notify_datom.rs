#![cfg(feature = "datom")]

use datom_codec::{Datomizable};
use protos::{Protosizable, Textualizable};
use signal_message::{notify::{parse_one, NotifyTextError}, Notify};

fn parse(value: Notify) -> Notify {
    let text = value.clone().datomize(vec![]).protosize().textualize();
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
    assert_eq!(parse_one(&["Notify.{ «bob@example.org/resource» «body» }".into()]), Err(NotifyTextError::Recipient));
    let oversized = Notify { notify_recipient: "bob@example.org".into(), notify_body: "x".repeat(1025) };
    let text = oversized.datomize(vec![]).protosize().textualize();
    assert_eq!(parse_one(&[text]), Err(NotifyTextError::Body));
}
