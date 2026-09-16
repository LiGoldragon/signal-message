//! Proposal-only Notify adapter. It validates Datom input; it does not deliver.

use datom_codec::{Actualizing, Budget, Potential};
use protos::ReaderBudget;
use crate::Notify;

const MAXIMUM_INPUT_BYTES: usize = 4096;
const MAXIMUM_BODY_BYTES: usize = 1024;

#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedNotify(pub Notify);

#[derive(Debug, PartialEq)]
pub enum NotifyTextError {
    ArgumentCount(usize),
    InputTooLarge,
    Malformed,
    Recipient,
    Body,
}

pub fn parse_one(arguments: &[String]) -> Result<ValidatedNotify, NotifyTextError> {
    let [text] = arguments else { return Err(NotifyTextError::ArgumentCount(arguments.len())); };
    if text.len() > MAXIMUM_INPUT_BYTES { return Err(NotifyTextError::InputTooLarge); }
    let mut pending = Potential::<Notify>::from(text.to_owned());
    let notify = pending.actualize(&mut Budget {
        remaining: MAXIMUM_INPUT_BYTES as i64,
        reader: ReaderBudget { remaining: MAXIMUM_INPUT_BYTES },
        depth: 0,
        maximum_depth: 256,
    }).map_err(|_| NotifyTextError::Malformed)?;
    let recipient = &notify.notify_recipient;
    if recipient.is_empty() || recipient.contains('/') || recipient.matches('@').count() != 1 {
        return Err(NotifyTextError::Recipient);
    }
    if notify.notify_body.is_empty() || notify.notify_body.len() > MAXIMUM_BODY_BYTES {
        return Err(NotifyTextError::Body);
    }
    Ok(ValidatedNotify(notify))
}
