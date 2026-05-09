# signal-persona-message

The Signal contract between **`message-cli`** (sender) and
**`persona-router`** (receiver).

Read `src/lib.rs` for the public interface — two enums
(`MessageRequest`, `MessageReply`) declared via the
`signal_channel!` macro from `signal-core`. The variants
ARE the messages this channel carries.

## Quick reference

```rust
use signal_persona_message::{Frame, MessageRequest, SubmitMessage};
use signal_core::{FrameBody, Request};

let request = MessageRequest::Submit(SubmitMessage {
    recipient: "designer".into(),
    body: "stack test".into(),
});
let frame = Frame::new(FrameBody::Request(Request::assert(request)));
let bytes = frame.encode_length_prefixed()?;
// send bytes on the persona-router UDS
```

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `~/primary/reports/designer/72-harmonized-implementation-plan.md`
  §2.1 — the channel inventory this contract belongs to
- `~/primary/reports/designer/73-signal-derive-research.md` —
  the `signal_channel!` macro decision
- `~/primary/skills/contract-repo.md` — contract-repo discipline
- `signal-core` — kernel that supplies `Frame`, `Request`,
  `Reply`, `signal_channel!`
