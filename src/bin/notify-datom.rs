//! Offline Notify Datom validation command.  This binary has no transport.

use signal_message::notify::{parse_one, NotifyTextError};

fn describe(error: NotifyTextError) -> String {
    match error {
        NotifyTextError::ArgumentCount(count) => {
            format!("expected exactly one inline Datom value, received {count}")
        }
        NotifyTextError::InputTooLarge => "Notify input exceeds 4096 UTF-8 bytes".into(),
        NotifyTextError::Malformed => "expected a Notify Datom".into(),
        NotifyTextError::Recipient => "recipient must be a bare JID".into(),
        NotifyTextError::Body => "Notify body must be 1 through 1024 UTF-8 bytes".into(),
    }
}

fn main() -> std::process::ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match parse_one(&arguments) {
        Ok(_) => {
            println!("NotifyValidatedOffline.{{}}");
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("malformed Notify Datom: {}", describe(error));
            std::process::ExitCode::from(2)
        }
    }
}
