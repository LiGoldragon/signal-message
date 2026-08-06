use signal_frame::SignalOperationHeads;
use signal_message::schema::lib::{Input, InputRoute, Output, OutputRoute, z2VLZR, z2VXE7, z2VY2v};

#[test]
fn handwritten_roles_seat_strict_payload_types() {
    let request_constructor: fn(z2VY2v) -> Input = Input::Submit;
    let reply = Output::SubmissionAccepted(z2VXE7::new(z2VLZR::new(1)));
    let _ = request_constructor;
    assert_eq!(reply.route(), OutputRoute::SubmissionAccepted);
}

#[test]
fn role_order_is_explicit_and_stable() {
    assert_eq!(InputRoute::Submit as u8, 0);
    assert_eq!(InputRoute::QueryThreads as u8, 8);
    assert_eq!(OutputRoute::SubmissionAccepted as u8, 0);
    assert_eq!(OutputRoute::ThreadRejected as u8, 12);
    assert_eq!(
        <Input as SignalOperationHeads>::HEADS,
        &[
            "Submit",
            "SubmitStamped",
            "QueryInbox",
            "AssignAgentIdentity",
            "BindAgentEndpoint",
            "QueryAgentRegistry",
            "QueryThread",
            "SubscribeThread",
            "QueryThreads"
        ]
    );
}
