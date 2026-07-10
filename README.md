# signal-message

The Signal contract for Message ingress. It carries the client-message
relation (`message` CLI to `message-daemon`) and the router-ingress
relation (`message-daemon` to `router`) in one root family.

Read `src/lib.rs` for the public interface — two enums
(`Input`, `Output`) generated from `schema/lib.schema` by
`schema-rust`. The variants ARE the messages this channel
carries.

## Quick reference

```rust
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, RequestPayload, SessionEpoch,
};
use signal_message::{
    Frame, FrameBody, MessageBody, MessageKind, MessageRecipient,
    Input, MessageSubmission, ThreadSelection,
};

let exchange = ExchangeIdentifier::new(
    SessionEpoch::new(1),
    ExchangeLane::Connector,
    LaneSequence::first(),
);
let request = Input::Submit(MessageSubmission {
    message_recipient: MessageRecipient::new(String::from("designer")),
    message_kind: MessageKind::Send,
    message_body: MessageBody::new(String::from("stack test")),
    // Optionality is a typed positional slot, never an omitted field:
    // `ThreadSelection::None` for the default derived thread, or
    // `ThreadSelection::Named(ThreadName::new(..))` for an explicit thread.
    thread_selection: ThreadSelection::None,
});
let frame = Frame::new(FrameBody::Request {
    exchange,
    request: request.into_request(),
});
let bytes = frame.encode_length_prefixed()?;
// send bytes on message.sock; message-daemon stamps before router.sock
```

The request operation heads are contract-local: `Submit`,
`SubmitStamped`, and `QueryInbox`. Sema classification labels such as
`Assert` and `Match` are daemon-side observation labels, not wire roots.

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `~/primary/skills/contract-repo.md` — contract-repo discipline
- `signal-frame` — kernel that supplies the request/reply frame envelope
- `schema/lib.schema` — authored contract vocabulary
