//! Architectural-truth tests for the
//! `signal-persona-message` channel.
//!
//! Each test names exactly what shape it pins down; per the
//! "blunt test names" convention.

use signal_core::{FrameBody, Request, SemaVerb};
use signal_persona_message::{
    Frame, InboxMessage, InboxQuery, InboxResult, MessageReply, MessageRequest, SubmitFailed,
    SubmitFailureReason, SubmitMessage, SubmitReceipt,
};

#[test]
fn submit_request_round_trips_through_length_prefixed_frame() {
    let request = MessageRequest::Submit(SubmitMessage {
        recipient: "designer".into(),
        body: "stack test".into(),
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SemaVerb::Assert);
            assert_eq!(payload, request);
        }
        other => panic!("expected Assert request, got {other:?}"),
    }
}

#[test]
fn inbox_query_round_trips_through_length_prefixed_frame() {
    let request = MessageRequest::Inbox(InboxQuery {
        recipient: "designer".into(),
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { payload, .. }) => {
            assert_eq!(payload, request);
        }
        other => panic!("expected request, got {other:?}"),
    }
}

#[test]
fn submit_ok_reply_round_trips() {
    let reply = MessageReply::SubmitOk(SubmitReceipt { message_slot: 1024 });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn submit_failed_reply_round_trips_with_typed_reason() {
    let reply = MessageReply::SubmitFailed(SubmitFailed {
        reason: SubmitFailureReason::UnknownRecipient,
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected SubmitFailed reply, got {other:?}"),
    }
}

#[test]
fn inbox_result_round_trips_through_length_prefixed_frame() {
    let reply = MessageReply::InboxResult(InboxResult {
        messages: vec![
            InboxMessage {
                message_slot: 1,
                sender: "operator".into(),
                body: "first".into(),
            },
            InboxMessage {
                message_slot: 2,
                sender: "operator".into(),
                body: "second".into(),
            },
        ],
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected InboxResult reply, got {other:?}"),
    }
}

#[test]
fn from_impl_lifts_submit_message_into_request() {
    let payload = SubmitMessage {
        recipient: "designer".into(),
        body: "via from".into(),
    };
    let request: MessageRequest = payload.clone().into();
    assert_eq!(request, MessageRequest::Submit(payload));
}

#[test]
fn from_impl_lifts_submit_receipt_into_reply() {
    let receipt = SubmitReceipt { message_slot: 7 };
    let reply: MessageReply = receipt.clone().into();
    assert_eq!(reply, MessageReply::SubmitOk(receipt));
}
