//! Architectural-truth tests for the
//! `signal-persona-message` channel.
//!
//! Each test names exactly what shape it pins down; per the
//! "blunt test names" convention.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_core::{FrameBody, Request, SemaVerb};
use signal_persona_message::{
    Frame, InboxEntry, InboxListing, InboxQuery, MessageBody, MessageOperationKind,
    MessageRecipient, MessageReply, MessageRequest, MessageSender, MessageSlot, MessageSubmission,
    SubmissionAcceptance, SubmissionRejection, SubmissionRejectionReason,
};

#[test]
fn message_submission_request_round_trips_through_length_prefixed_frame() {
    let request = MessageRequest::MessageSubmission(MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        body: MessageBody::new("stack test"),
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
    let request = MessageRequest::InboxQuery(InboxQuery {
        recipient: MessageRecipient::new("designer"),
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
fn submission_accepted_reply_round_trips() {
    let reply = MessageReply::SubmissionAccepted(SubmissionAcceptance {
        message_slot: MessageSlot::new(1024),
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
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn submission_rejected_reply_round_trips_with_typed_reason() {
    let reply = MessageReply::SubmissionRejected(SubmissionRejection {
        reason: SubmissionRejectionReason::RecipientNotFound,
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
        other => panic!("expected SubmissionRejected reply, got {other:?}"),
    }
}

#[test]
fn inbox_listing_round_trips_through_length_prefixed_frame() {
    let reply = MessageReply::InboxListing(InboxListing {
        messages: vec![
            InboxEntry {
                message_slot: MessageSlot::new(1),
                sender: MessageSender::new("operator"),
                body: MessageBody::new("first"),
            },
            InboxEntry {
                message_slot: MessageSlot::new(2),
                sender: MessageSender::new("operator"),
                body: MessageBody::new("second"),
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
        other => panic!("expected InboxListing reply, got {other:?}"),
    }
}

#[test]
fn from_impl_lifts_message_submission_into_request() {
    let payload = MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        body: MessageBody::new("via from"),
    };
    let request: MessageRequest = payload.clone().into();
    assert_eq!(request, MessageRequest::MessageSubmission(payload));
}

#[test]
fn from_impl_lifts_submission_acceptance_into_reply() {
    let acceptance = SubmissionAcceptance {
        message_slot: MessageSlot::new(7),
    };
    let reply: MessageReply = acceptance.clone().into();
    assert_eq!(reply, MessageReply::SubmissionAccepted(acceptance));
}

#[test]
fn message_request_exposes_contract_owned_operation_kind() {
    let submission = MessageRequest::MessageSubmission(MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        body: MessageBody::new("kind witness"),
    });
    let inbox = MessageRequest::InboxQuery(InboxQuery {
        recipient: MessageRecipient::new("designer"),
    });

    assert_eq!(
        submission.operation_kind(),
        MessageOperationKind::MessageSubmission
    );
    assert_eq!(inbox.operation_kind(), MessageOperationKind::InboxQuery);
}

#[test]
fn message_operation_kind_round_trips_through_nota_text() {
    let mut encoder = Encoder::new();
    MessageOperationKind::MessageSubmission
        .encode(&mut encoder)
        .expect("encode operation kind");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageOperationKind::decode(&mut decoder).expect("decode operation kind");

    assert_eq!(recovered, MessageOperationKind::MessageSubmission);
    assert_eq!(text, "MessageSubmission");
}

#[test]
fn message_submission_request_round_trips_through_nota_text() {
    let request = MessageRequest::MessageSubmission(MessageSubmission {
        recipient: MessageRecipient::new("designer"),
        body: MessageBody::new("stack test"),
    });

    let mut encoder = Encoder::new();
    request.encode(&mut encoder).expect("encode request");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageRequest::decode(&mut decoder).expect("decode request");

    assert_eq!(recovered, request);
    assert_eq!(text, "(MessageSubmission designer \"stack test\")");
}

#[test]
fn submission_accepted_reply_round_trips_through_nota_text() {
    let reply = MessageReply::SubmissionAccepted(SubmissionAcceptance {
        message_slot: MessageSlot::new(7),
    });

    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode reply");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = MessageReply::decode(&mut decoder).expect("decode reply");

    assert_eq!(recovered, reply);
    assert_eq!(text, "(SubmissionAcceptance 7)");
}
