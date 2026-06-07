# signal-message

The Signal contract for Message ingress. It carries the client-message
relation (`message` CLI to `message-daemon`) and the router-ingress
relation (`message-daemon` to `router`) in one root family.

Read `src/lib.rs` for the public interface — two enums
(`MessageRequest`, `MessageReply`) declared via the
`signal_channel!` macro from `signal-frame`. The variants
ARE the messages this channel carries.

## Quick reference

```rust
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, RequestPayload, SessionEpoch,
};
use signal_message::{
    Frame, FrameBody, MessageBody, MessageKind, MessageRecipient,
    MessageRequest, MessageSubmission,
};

let exchange = ExchangeIdentifier::new(
    SessionEpoch::new(1),
    ExchangeLane::Connector,
    LaneSequence::first(),
);
let request = MessageRequest::Submit(MessageSubmission {
    recipient: MessageRecipient::new("designer"),
    kind: MessageKind::Send,
    body: MessageBody::new("stack test"),
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
- `signal-frame` — kernel that supplies `Frame`, `Request`,
  `Reply`, `signal_channel!`
