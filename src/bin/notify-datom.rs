//! Offline Notify Datom validation command.  This binary has no transport.

use datom_codec::Datomizable;
use protos::{Protosizable, Textualizable};
use signal_message::{
    notify::{parse_one, NotifyTextError}, NotifyValidationOutcome, NotifyValidationReceipt,
    NotifyValidationRejection,
};

fn rejected(error: NotifyTextError) -> NotifyValidationOutcome {
    let rejection = match error {
        NotifyTextError::ArgumentCount(count) => NotifyValidationRejection::ArgumentCount(count as i64),
        NotifyTextError::InputTooLarge => NotifyValidationRejection::InputTooLarge,
        NotifyTextError::Malformed => NotifyValidationRejection::Malformed,
        NotifyTextError::Recipient => NotifyValidationRejection::Recipient,
        NotifyTextError::Body => NotifyValidationRejection::Body,
    };
    NotifyValidationOutcome::NotifyRejected(rejection)
}

fn text(outcome: NotifyValidationOutcome) -> String {
    outcome.datomize(vec![]).protosize().textualize()
}

fn main() -> std::process::ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match parse_one(&arguments) {
        Ok(_) => {
            println!("{}", text(NotifyValidationOutcome::NotifyValidatedOffline(NotifyValidationReceipt {})));
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}", text(rejected(error.clone())));
            std::process::ExitCode::from(2)
        }
    }
}
