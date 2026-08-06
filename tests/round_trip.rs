#[cfg(feature = "dotos-text")]
use dotos::{DotosEncode, DotosSource};
use signal_frame::{ExchangeIdentifier, ExchangeLane, LaneSequence, Reply, SessionEpoch, SubReply};
use signal_message::schema::lib::*;

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn recipient(name: &str) -> z2Vari {
    z2Vari::new(name.to_owned())
}

fn submission() -> z2VY2v {
    z2VY2v {
        field_0: recipient("designer"),
        field_1: z2VdsV::z2VXeo,
        field_2: z2VNcG::new("hello".to_owned()),
        field_3: z2VTiK::z2VR2m,
    }
}

fn input_cases() -> Vec<Input> {
    vec![
        Input::Submit(submission()),
        Input::SubmitStamped(z2Ve71 {
            field_0: submission(),
            field_1: z2VTJ1::z2VWSr(z2VY3v::z2VRFq),
            field_2: z2VY18::new(z2Vf2p::new(1)),
        }),
        Input::QueryInbox(z2VSVi::new(recipient("designer"))),
        Input::AssignAgentIdentity(z2VevD {
            field_0: z2VNPW::new("agent-a".to_owned()),
            field_1: z2Vcfd::z2VRLv,
            field_2: z2VXMQ::z2VNZi,
        }),
        Input::BindAgentEndpoint(z2VVAD {
            field_0: z2VNPW::new("agent-a".to_owned()),
            field_1: z2VMBf {
                field_0: z2VUs6::z2VZk6,
                field_1: z2VRqE::new(z2VQY5::new("/run/agent-a".to_owned())),
            },
            field_2: z2VPEW::new(10),
            field_3: z2VYrY::new(20),
        }),
        Input::QueryAgentRegistry(z2VYJe::z2VPkz),
        Input::QueryThread(z2VVrY::new(z2VUSt::new("work".to_owned()))),
        Input::SubscribeThread(z2VMd2 {
            field_0: z2VUSt::new("work".to_owned()),
            field_1: z2Vd8W::new("agent-a".to_owned()),
            field_2: z2VVDs::z2VRQJ,
        }),
        Input::QueryThreads(z2VLC8::z2VLjY),
    ]
}

fn output_cases() -> Vec<Output> {
    vec![
        Output::SubmissionAccepted(z2VXE7::new(z2VLZR::new(1))),
        Output::SubmissionRejected(z2VZEr::new(z2VPW5::z2VZW7)),
        Output::InboxListing(z2VRGD {
            field_0: z2Vb4S::new(vec![]),
        }),
        Output::AgentIdentityAssigned(z2VdZd {
            field_0: z2VNPW::new("agent-a".to_owned()),
            field_1: z2Vdpc::z2VRYp,
        }),
        Output::AgentEndpointBound(z2VQy1::new(z2VNPW::new("agent-a".to_owned()))),
        Output::AgentRegistryListing(z2VUzX {
            field_0: z2VS1e::new(vec![]),
        }),
        Output::AgentRegistryRejected(z2VP29::new(z2VW5p::z2VYA6)),
        Output::MessageRequestUnimplemented(z2VYf6 {
            field_0: z2VLsC::z2VQEV,
            field_1: z2Vc6L::z2VXy5,
        }),
        Output::Error(z2Vasi::new(z2VZuS::new("error".to_owned()))),
        Output::ThreadListing(z2VYbP {
            field_0: z2VUSt::new("work".to_owned()),
            field_1: z2VVDs::z2VRQJ,
            field_2: z2VMa5::new(vec![]),
            field_3: z2VWzi::new(vec![]),
        }),
        Output::ThreadSubscribed(z2VbGY {
            field_0: z2VUSt::new("work".to_owned()),
            field_1: z2Vd8W::new("agent-a".to_owned()),
        }),
        Output::ThreadIndexListing(z2VR9d {
            field_0: z2VV6N::new(vec![]),
        }),
        Output::ThreadRejected(z2VPEF::new(z2Ve52::z2VQY2)),
    ]
}

#[test]
fn every_handwritten_request_role_round_trips() {
    for input in input_cases() {
        let expected = input.clone();
        let bytes = input
            .into_frame(exchange())
            .encode_length_prefixed()
            .expect("encode request");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");
        let FrameBody::Request { request, .. } = decoded.into_body() else {
            panic!("expected request")
        };
        assert_eq!(request.payloads().head(), &expected);
    }
}

#[test]
fn every_handwritten_reply_role_round_trips() {
    for output in output_cases() {
        let expected = output.clone();
        let bytes = output
            .into_reply_frame(exchange())
            .encode_length_prefixed()
            .expect("encode reply");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");
        let FrameBody::Reply { reply, .. } = decoded.into_body() else {
            panic!("expected reply")
        };
        let Reply::Accepted { per_operation, .. } = reply else {
            panic!("expected accepted reply")
        };
        let SubReply::Ok(actual) = per_operation.into_head() else {
            panic!("expected reply payload")
        };
        assert_eq!(actual, expected);
    }
}

#[cfg(feature = "dotos-text")]
#[test]
fn dotos_keeps_role_and_domain_names_visible() {
    let input = Input::Submit(submission());
    let text = input.to_dotos();
    assert!(text.starts_with("Submit."), "{text}");
    assert_eq!(
        DotosSource::new(&text).parse::<Input>().expect("decode"),
        input
    );

    let output = Output::SubmissionAccepted(z2VXE7::new(z2VLZR::new(1)));
    let text = output.to_dotos();
    assert!(text.starts_with("SubmissionAccepted."), "{text}");
    assert_eq!(
        DotosSource::new(&text).parse::<Output>().expect("decode"),
        output
    );
}
