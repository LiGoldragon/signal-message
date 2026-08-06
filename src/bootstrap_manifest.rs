//! Explicit producer-owned bootstrap authority state for the ordinary Message Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    31, 75, 25, 21, 95, 86, 117, 126, 8, 106, 220, 179, 189, 37, 121, 98, 240, 124, 174, 161, 222,
    74, 244, 43, 69, 180, 177, 80, 65, 35, 197, 4,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 52514;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 9927;

pub const INTERFACE_SEAT: AuthoritySeat = AuthoritySeat::new("Interface", 1624, 0x9aa152ba9d5a6934);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 46754, 0x50cefea0defe91ed);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 39180, 0x9f416f1ffeda847d);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 27493, 0xd621e8168cdbf4ea);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 65147, 0xe928014697d3f200);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 40266, 0x95cbeb0ddbabd18a);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 20871, 0x6479092168ad49a3);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 17504, 0x8c39d8c1d9921906);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 10765, 0x093d33fc10517203);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 14696, 0x3e79381fe7437759);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 47118, 0xfb47e7570e16c45c);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 26023, 0xe4986cf84cba37a8);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 25144, 0x99519ab7ba1dbb20);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 46014, 0x7dd27733594a54be);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 35688, 0xfa81a2bc1ef951cc);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 23482, 0xc8401e61ef3ebcbe);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    1667, 33462, 907, 62677, 5515, 19559, 63143, 27142, 39892, 52699,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "MessageRecipient", 50775, 0x16d4b92c2fca45cf),
    DeclarationSeat::new(None, "MessageSender", 34671, 0xd4cebb9d5451dd89),
    DeclarationSeat::new(None, "MessageBody", 9569, 0x62f6bfc0300359ac),
    DeclarationSeat::new(None, "MessageSlot", 2676, 0x9b13263acc33775d),
    DeclarationSeat::new(None, "TimestampNanos", 64817, 0x89b7634306b48915),
    DeclarationSeat::new(None, "WirePath", 16054, 0xc31a181f60336da3),
    DeclarationSeat::new(None, "SocketMode", 43038, 0x4abde2ce318b0a72),
    DeclarationSeat::new(None, "UnixUserIdentifier", 13499, 0x3ca88d273018c328),
    DeclarationSeat::new(None, "SystemPrincipal", 3333, 0xd97c716225654f06),
    DeclarationSeat::new(None, "EngineIdentifier", 65177, 0x0565d30e37498d47),
    DeclarationSeat::new(None, "HostName", 57218, 0x5ce0db243b5d25c9),
    DeclarationSeat::new(None, "NetworkPeer", 2751, 0xc8da816047bea442),
    DeclarationSeat::new(None, "ComponentInstanceName", 26312, 0x3e51e1bd62af1535),
    DeclarationSeat::new(None, "MessageKind", 60912, 0x2a77ca64f7657884),
    DeclarationSeat::new(Some(60912), "Send", 39992, 0xac3fb1c3d64bd911),
    DeclarationSeat::new(Some(60912), "Inbox", 57268, 0xe5c24db4fcc230ab),
    DeclarationSeat::new(None, "ThreadName", 29209, 0xd95a41da61d1ff79),
    DeclarationSeat::new(None, "ThreadSelection", 26740, 0xa696904ea129c26e),
    DeclarationSeat::new(Some(26740), "None", 17718, 0x5c3cdc3da39acb35),
    DeclarationSeat::new(Some(26740), "Named", 12416, 0x3b3819177879f16d),
    DeclarationSeat::new(None, "MessageSubmission", 41275, 0x90ae9a8d1f5c3e22),
    DeclarationSeat::new(None, "Host", 46449, 0x77620fd796837dfa),
    DeclarationSeat::new(None, "OtherPersonaEngine", 50287, 0xb7086b772d5630d2),
    DeclarationSeat::new(None, "ConnectionClass", 41333, 0xad4741b8f91ad00f),
    DeclarationSeat::new(Some(41333), "Owner", 18476, 0x07dbec2e27caff96),
    DeclarationSeat::new(Some(41333), "NonOwnerUser", 7860, 0xf1653e4adb0f867b),
    DeclarationSeat::new(Some(41333), "System", 12736, 0x4e3249e829070d60),
    DeclarationSeat::new(Some(41333), "OtherPersona", 56872, 0x49404aa9d54c0c12),
    DeclarationSeat::new(Some(41333), "Network", 33957, 0x38ab421111578ae6),
    DeclarationSeat::new(None, "ComponentName", 60520, 0xc2941c19deb35278),
    DeclarationSeat::new(Some(60520), "Mind", 61923, 0x968375f0c106f1ed),
    DeclarationSeat::new(Some(60520), "Message", 41978, 0xc16577653106837c),
    DeclarationSeat::new(Some(60520), "Router", 4752, 0xefc2ce4803d1185a),
    DeclarationSeat::new(Some(60520), "Terminal", 29706, 0x32504ab7748a48a4),
    DeclarationSeat::new(Some(60520), "Harness", 13740, 0xd0e27f3ee8f1fd01),
    DeclarationSeat::new(Some(60520), "System", 41556, 0x32fc5fccf41920c9),
    DeclarationSeat::new(Some(60520), "Introspect", 45772, 0x1cde4e6df0d631db),
    DeclarationSeat::new(Some(60520), "Orchestrate", 27160, 0xf596898fa8796cd8),
    DeclarationSeat::new(Some(60520), "Spirit", 62491, 0xaab9faee89c53556),
    DeclarationSeat::new(
        None,
        "InternalComponentInstanceOrigin",
        13677,
        0x161ab07cf22ee657,
    ),
    DeclarationSeat::new(None, "MessageOrigin", 25330, 0x999b48e0cf96e03e),
    DeclarationSeat::new(Some(25330), "Internal", 34787, 0xd7a517f40ebdd31d),
    DeclarationSeat::new(
        Some(25330),
        "InternalComponentInstance",
        21183,
        0xbd7a268ec1710614,
    ),
    DeclarationSeat::new(Some(25330), "External", 35935, 0x4cf95a5c012d7f63),
    DeclarationSeat::new(None, "StampedAt", 41171, 0x3993fa10e3cbd3df),
    DeclarationSeat::new(None, "StampedMessageSubmission", 61696, 0xc9e03a1283b5716a),
    DeclarationSeat::new(None, "IngressSocketPath", 55277, 0x955662b02639dbbf),
    DeclarationSeat::new(None, "ComponentMessageIngress", 54660, 0x85016157fd2f6b10),
    DeclarationSeat::new(None, "SubmissionAcceptance", 38560, 0x86db58666eaa54fa),
    DeclarationSeat::new(None, "InboxQuery", 22645, 0x26c70ab9d816997c),
    DeclarationSeat::new(None, "MessageOperationKind", 3707, 0x2c63e62d80e39963),
    DeclarationSeat::new(Some(3707), "Submit", 15034, 0x1eb7c4bb15ae125c),
    DeclarationSeat::new(Some(3707), "SubmitStamped", 24636, 0x9fbe5e3f3ec6e967),
    DeclarationSeat::new(Some(3707), "QueryInbox", 22164, 0x69cbf20e74850988),
    DeclarationSeat::new(Some(3707), "AssignAgentIdentity", 4721, 0xc46a9fdb6b795d02),
    DeclarationSeat::new(Some(3707), "BindAgentEndpoint", 13680, 0xecb5d98eb0f38368),
    DeclarationSeat::new(Some(3707), "QueryAgentRegistry", 14473, 0x8c22c3b9e6183f80),
    DeclarationSeat::new(Some(3707), "QueryThread", 58058, 0x262b82863e3628f9),
    DeclarationSeat::new(Some(3707), "SubscribeThread", 53345, 0x2b2a812feb739a52),
    DeclarationSeat::new(Some(3707), "QueryThreads", 8090, 0x0182b87e338e82c0),
    DeclarationSeat::new(None, "AgentIdentifier", 8829, 0xac2a0ca662f83531),
    DeclarationSeat::new(None, "HarnessPid", 11671, 0xab7196b5f7a604de),
    DeclarationSeat::new(None, "HarnessStartTime", 44037, 0x46f3f6a952d7e90b),
    DeclarationSeat::new(None, "ResumeIdentity", 59592, 0x3746d8efa54a5a9c),
    DeclarationSeat::new(None, "ResumeSelection", 38983, 0x5b1862adc4be4f60),
    DeclarationSeat::new(Some(38983), "None", 9421, 0xf9dc9a6c89f9acde),
    DeclarationSeat::new(Some(38983), "Resumed", 60150, 0xbaf9c567e2603a48),
    DeclarationSeat::new(None, "AgentEndpointKind", 30613, 0xa96fef38b2557014),
    DeclarationSeat::new(Some(30613), "PtySocket", 47027, 0x7392b05b24c0d9b1),
    DeclarationSeat::new(Some(30613), "HarnessSocket", 26767, 0x3c58668bf60654b1),
    DeclarationSeat::new(None, "EndpointPath", 20413, 0xb76d500d6450072a),
    DeclarationSeat::new(None, "AgentEndpoint", 4778, 0x733faae716628896),
    DeclarationSeat::new(None, "EndpointSelection", 9512, 0x4bbf6b7885cbe594),
    DeclarationSeat::new(Some(9512), "None", 46082, 0xbc225f056a133a27),
    DeclarationSeat::new(Some(9512), "Bound", 51383, 0x1675c422c3de9355),
    DeclarationSeat::new(None, "AgentDeathMark", 53842, 0x495bcc7913208678),
    DeclarationSeat::new(Some(53842), "NotDead", 22176, 0x3adc7f366c8734fa),
    DeclarationSeat::new(Some(53842), "Killed", 51829, 0x8de857b7e6fdd8f6),
    DeclarationSeat::new(None, "HarnessProcessPin", 25098, 0x4e10bf66f3d9897e),
    DeclarationSeat::new(None, "ProcessPinSelection", 56860, 0x436d565366d01e2a),
    DeclarationSeat::new(Some(56860), "None", 18771, 0x4d444d7873ec03a3),
    DeclarationSeat::new(Some(56860), "Pinned", 10293, 0x354f2fbc2e84f1bf),
    DeclarationSeat::new(None, "AgentIdentityAssignment", 64434, 0x7f029739d685592b),
    DeclarationSeat::new(None, "IdentityProvenance", 60745, 0x14c2123e9fd5745e),
    DeclarationSeat::new(Some(60745), "Seated", 19461, 0xdeeb4a64317ca6d9),
    DeclarationSeat::new(Some(60745), "Reseated", 51389, 0xfe224917b121bf79),
    DeclarationSeat::new(None, "AssignedAgentIdentity", 59876, 0x8b6eb92bf292927d),
    DeclarationSeat::new(None, "AgentEndpointBinding", 31606, 0xb018f5b0c71e8098),
    DeclarationSeat::new(None, "BoundAgentEndpoint", 17500, 0x283139723d120244),
    DeclarationSeat::new(None, "AgentRegistryQuery", 42187, 0xb2b70f23bf7d173c),
    DeclarationSeat::new(Some(42187), "All", 13439, 0x3cd56ff93c550b43),
    DeclarationSeat::new(Some(42187), "ByAgent", 54245, 0xd2962e1c9bdf982a),
    DeclarationSeat::new(None, "AgentRegistryEntry", 54969, 0x8016c1de6201ea10),
    DeclarationSeat::new(None, "Entries", 21017, 0x8f6c4ca9894868a2),
    DeclarationSeat::new(None, "AgentRegistryListingReply", 31044, 0x7f717d855edfb5d0),
    DeclarationSeat::new(
        None,
        "AgentRegistryRejectionReason",
        34715,
        0xb5514a343649ed70,
    ),
    DeclarationSeat::new(
        Some(34715),
        "UnknownAgentIdentifier",
        41691,
        0x638d682847b57714,
    ),
    DeclarationSeat::new(Some(34715), "StoreRejected", 41805, 0x96631c9ee2641345),
    DeclarationSeat::new(None, "AgentRegistryRejection", 10954, 0x894a877d5b15c6ec),
    DeclarationSeat::new(None, "InboxEntry", 19001, 0x2f28ace554e29f9f),
    DeclarationSeat::new(None, "Messages", 51455, 0x010efccf6c9afa54),
    DeclarationSeat::new(None, "InboxListingReply", 18498, 0x9deff8bb606cc2bb),
    DeclarationSeat::new(None, "SubmissionRejectionReason", 12574, 0xa68e8179630521ee),
    DeclarationSeat::new(Some(12574), "StoreRejected", 64409, 0xee764092d3827d27),
    DeclarationSeat::new(Some(12574), "RecipientNotFound", 46216, 0x8f4c19336c12bacb),
    DeclarationSeat::new(None, "SubmissionRejection", 45331, 0xb7b5b86790a77976),
    DeclarationSeat::new(None, "ErrorMessage", 47569, 0x9ddd3a0d853b907d),
    DeclarationSeat::new(None, "ErrorReport", 50833, 0x8ec021832ec74db6),
    DeclarationSeat::new(None, "ThreadQuery", 33945, 0xefbe013b1978eb1d),
    DeclarationSeat::new(None, "RepositoryName", 23276, 0x414a45c1688c614f),
    DeclarationSeat::new(None, "FeatureBranchName", 43081, 0xc1d252d0f7e0e880),
    DeclarationSeat::new(None, "ThreadRelation", 57094, 0x22c82b33718f0fb0),
    DeclarationSeat::new(None, "ThreadRelationSelection", 31818, 0x34764c9ce51d2948),
    DeclarationSeat::new(Some(31818), "None", 18967, 0x53fcc373b604546b),
    DeclarationSeat::new(Some(31818), "Related", 49653, 0x82fcfce3d3f2a418),
    DeclarationSeat::new(None, "ParticipantName", 58419, 0x44892ffb3f78a448),
    DeclarationSeat::new(None, "ThreadSubscription", 6249, 0xb22650c0ac3f2dbd),
    DeclarationSeat::new(
        None,
        "ThreadSubscriptionAcknowledgment",
        52157,
        0x5bc7383aa9b2ae36,
    ),
    DeclarationSeat::new(None, "ThreadEntry", 17031, 0xc3f7b68970aa2223),
    DeclarationSeat::new(None, "Participants", 6078, 0xb29d63206eff7cb0),
    DeclarationSeat::new(None, "ThreadEntries", 37783, 0x5f31589a5616a710),
    DeclarationSeat::new(None, "ThreadContents", 43158, 0xf77480da26ff4870),
    DeclarationSeat::new(None, "ThreadIndexQuery", 1441, 0x421e0e6334a7fb91),
    DeclarationSeat::new(Some(1441), "All", 3263, 0x76438c41797b4b49),
    DeclarationSeat::new(None, "MessageCount", 14978, 0x6d8b058b1b2815b9),
    DeclarationSeat::new(None, "ThreadSummary", 3779, 0x88e4f7bc19453342),
    DeclarationSeat::new(None, "Threads", 31383, 0xe6d2a2da96913581),
    DeclarationSeat::new(None, "ThreadIndexEntries", 18116, 0x69b64147ec413635),
    DeclarationSeat::new(None, "ThreadRejectionReason", 61581, 0x5e66549b865cdfd4),
    DeclarationSeat::new(Some(61581), "UnknownThread", 16051, 0x1785a003cbe85614),
    DeclarationSeat::new(Some(61581), "StoreRejected", 61227, 0x4b9b6a142ccc0a0b),
    DeclarationSeat::new(None, "ThreadRejection", 11656, 0xa29f1c960c971d7e),
    DeclarationSeat::new(None, "DependencyKind", 48436, 0x7b2e0b38cbb7ec18),
    DeclarationSeat::new(Some(48436), "Router", 62689, 0x3edf0f443092481c),
    DeclarationSeat::new(Some(48436), "Mind", 44302, 0x3ed3787b216da8bf),
    DeclarationSeat::new(Some(48436), "Harness", 12636, 0xa27fcb7350221026),
    DeclarationSeat::new(Some(48436), "Terminal", 38968, 0xb96e1ed73c85af58),
    DeclarationSeat::new(None, "ResourceKind", 30467, 0xa86b75a5befe3273),
    DeclarationSeat::new(Some(30467), "MessageSocket", 25448, 0xebbdddc3e11f23c3),
    DeclarationSeat::new(Some(30467), "RouterSocket", 58353, 0x1dd384d2189c5ca1),
    DeclarationSeat::new(Some(30467), "PeerCredentials", 63905, 0xad752cd75cac46ec),
    DeclarationSeat::new(Some(30467), "Store", 47536, 0x3dcb3f2795926c2c),
    DeclarationSeat::new(
        None,
        "MessageUnimplementedReason",
        54929,
        0x62d430196d381195,
    ),
    DeclarationSeat::new(
        Some(54929),
        "NotInPrototypeScope",
        41052,
        0xa255d7884da31bf3,
    ),
    DeclarationSeat::new(Some(54929), "DependencyMissing", 37811, 0xa1c1b6016489999e),
    DeclarationSeat::new(
        Some(54929),
        "ResourceUnavailable",
        32613,
        0xbb54bba85d32593e,
    ),
    DeclarationSeat::new(
        None,
        "MessageRequestUnimplementedReply",
        43373,
        0x3cfecac93ec66f45,
    ),
    DeclarationSeat::new(None, "OwnerIdentity", 30526, 0x53ee94969f42a982),
    DeclarationSeat::new(Some(30526), "UnixUser", 58470, 0x02baf78a80d3356d),
    DeclarationSeat::new(Some(30526), "System", 45448, 0xe13e0f276f92e61e),
    DeclarationSeat::new(None, "MessageSocketPath", 29331, 0x07f44cec8755d6a1),
    DeclarationSeat::new(None, "MessageSocketMode", 12804, 0xba5db383cd06262d),
    DeclarationSeat::new(None, "SupervisionSocketPath", 18649, 0xcdb4026d38ab9aa1),
    DeclarationSeat::new(None, "SupervisionSocketMode", 49559, 0xc0733ad0951cf565),
    DeclarationSeat::new(None, "RouterSocketPath", 47610, 0xd9129cf59a470c67),
    DeclarationSeat::new(None, "ComponentIngresses", 18908, 0x184f5a4bce405df8),
    DeclarationSeat::new(None, "MessageDaemonConfiguration", 865, 0xce8d23c69cdfd0d2),
];
