# signal-persona-message

The Signal contract for Persona message ingress. It carries the client-message
relation (`message` CLI to `persona-message-daemon`) and the router-ingress
relation (`persona-message-daemon` to `persona-router`) in one root family.

Read `src/lib.rs` for the public interface — two enums
(`MessageRequest`, `MessageReply`) declared via the
`signal_channel!` macro from `signal-core`. The variants
ARE the messages this channel carries.

## Quick reference

```rust
use signal_core::{FrameBody, Request};
use signal_persona_message::{
    Frame, MessageBody, MessageKind, MessageRecipient, MessageRequest,
    MessageSubmission,
};

let request = MessageRequest::MessageSubmission(MessageSubmission {
    recipient: MessageRecipient::new("designer"),
    kind: MessageKind::Send,
    body: MessageBody::new("stack test"),
});
let frame = Frame::new(FrameBody::Request(Request::assert(request)));
let bytes = frame.encode_length_prefixed()?;
// send bytes on message.sock; persona-message-daemon stamps before router.sock
```

Submissions are write-shaped and use the `Assert` root. `InboxQuery`
is read-shaped and uses `Request::match_records(...)`; the contract-owned
`MessageRequest::signal_verb()` method is the witness for this mapping.

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `~/primary/skills/contract-repo.md` — contract-repo discipline
- `signal-core` — kernel that supplies `Frame`, `Request`,
  `Reply`, `signal_channel!`
