#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type Messages = std::vec::Vec<InboxEntry>;
#[rustfmt::skip]
pub type EngineIdentifier = String;
#[rustfmt::skip]
pub type MessageSocketPath = WirePath;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum IdentityProvenance {
    Seated,
    Reseated,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InternalComponentInstanceOrigin {
    pub component_name: ComponentName,
    pub component_instance_name: ComponentInstanceName,
}
#[rustfmt::skip]
pub type MessageRecipient = String;
#[rustfmt::skip]
pub type ComponentIngresses = std::vec::Vec<ComponentMessageIngress>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadRelation {
    pub repository_name: RepositoryName,
    pub feature_branch_name: FeatureBranchName,
}
#[rustfmt::skip]
pub type InboxQuery = MessageRecipient;
#[rustfmt::skip]
pub type BoundAgentEndpoint = AgentIdentifier;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MessageKind {
    Send,
    Inbox,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MessageOperationKind {
    QueryThreads,
    Submit,
    QueryThread,
    SubscribeThread,
    QueryInbox,
    QueryAgentRegistry,
    SubmitStamped,
    AssignAgentIdentity,
    BindAgentEndpoint,
    FlowDeliver,
    FlowAnnounceIdle,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InboxEntry {
    pub message_slot: MessageSlot,
    pub message_sender: MessageSender,
    pub message_body: MessageBody,
    pub thread_selection: ThreadSelection,
    pub stamped_at: StampedAt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThreadRelationSelection {
    None,
    Related(ThreadRelation),
}
#[rustfmt::skip]
pub type ResumeIdentity = String;
#[rustfmt::skip]
pub type StampedAt = TimestampNanos;
#[rustfmt::skip]
pub type UnixUserIdentifier = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MessageRequestUnimplementedReply {
    pub message_operation_kind: MessageOperationKind,
    pub message_unimplemented_reason: MessageUnimplementedReason,
}
#[rustfmt::skip]
pub type ComponentInstanceName = String;
#[rustfmt::skip]
pub type RepositoryName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThreadIndexQuery {
    All,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ProcessPinSelection {
    Pinned(HarnessProcessPin),
    None,
}
#[rustfmt::skip]
pub type ParticipantName = String;
#[rustfmt::skip]
pub type HarnessStartTime = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AgentDeathMark {
    NotDead,
    Killed,
}
#[rustfmt::skip]
pub type SocketMode = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EndpointSelection {
    Bound(AgentEndpoint),
    None,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessProcessPin {
    pub harness_pid: HarnessPid,
    pub harness_start_time: HarnessStartTime,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum OwnerIdentity {
    UnixUser(UnixUserIdentifier),
    System(SystemPrincipal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResumeSelection {
    Resumed(ResumeIdentity),
    None,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadSubscriptionAcknowledgment {
    pub thread_name: ThreadName,
    pub participant_name: ParticipantName,
}
#[rustfmt::skip]
pub type HostName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThreadRejectionReason {
    UnknownThread,
    StoreRejected,
}
#[rustfmt::skip]
pub type ThreadEntries = std::vec::Vec<ThreadEntry>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MessageUnimplementedReason {
    DependencyMissing(DependencyKind),
    NotInPrototypeScope,
    ResourceUnavailable(ResourceKind),
}
#[rustfmt::skip]
pub type MessageBody = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadIndexEntries {
    pub threads: Threads,
}
#[rustfmt::skip]
pub type MessageCount = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AgentEndpoint {
    pub agent_endpoint_kind: AgentEndpointKind,
    pub endpoint_path: EndpointPath,
}
#[rustfmt::skip]
pub type Host = HostName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DependencyKind {
    Mind,
    Router,
    Harness,
    Terminal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AgentIdentityAssignment {
    pub agent_identifier: AgentIdentifier,
    pub process_pin_selection: ProcessPinSelection,
    pub resume_selection: ResumeSelection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AgentRegistryListingReply {
    pub entries: Entries,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AgentRegistryEntry {
    pub agent_identifier: AgentIdentifier,
    pub endpoint_selection: EndpointSelection,
    pub resume_selection: ResumeSelection,
    pub agent_death_mark: AgentDeathMark,
    pub process_pin_selection: ProcessPinSelection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentMessageIngress {
    pub internal_component_instance_origin: InternalComponentInstanceOrigin,
    pub ingress_socket_path: IngressSocketPath,
    pub socket_mode: SocketMode,
}
#[rustfmt::skip]
pub type SubmissionAcceptance = MessageSlot;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadSummary {
    pub thread_name: ThreadName,
    pub thread_relation_selection: ThreadRelationSelection,
    pub participants: Participants,
    pub message_count: MessageCount,
}
#[rustfmt::skip]
pub type AgentRegistryRejection = AgentRegistryRejectionReason;
#[rustfmt::skip]
pub type TimestampNanos = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AssignedAgentIdentity {
    pub agent_identifier: AgentIdentifier,
    pub identity_provenance: IdentityProvenance,
}
#[rustfmt::skip]
pub type ErrorReport = ErrorMessage;
#[rustfmt::skip]
pub type Entries = std::vec::Vec<AgentRegistryEntry>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MessageSubmission {
    pub message_recipient: MessageRecipient,
    pub message_kind: MessageKind,
    pub message_body: MessageBody,
    pub thread_selection: ThreadSelection,
}
#[rustfmt::skip]
pub type IngressSocketPath = WirePath;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MessageOrigin {
    External(ConnectionClass),
    InternalComponentInstance(InternalComponentInstanceOrigin),
    Internal(ComponentName),
}
#[rustfmt::skip]
pub type MessageSlot = i64;
#[rustfmt::skip]
pub type ErrorMessage = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InboxListingReply {
    pub messages: Messages,
}
#[rustfmt::skip]
pub type ThreadRejection = ThreadRejectionReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SubmissionRejectionReason {
    RecipientNotFound,
    StoreRejected,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThreadSelection {
    Named(ThreadName),
    None,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResourceKind {
    RouterSocket,
    Store,
    PeerCredentials,
    MessageSocket,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AgentEndpointKind {
    HarnessSocket,
    PtySocket,
}
#[rustfmt::skip]
pub type HarnessPid = i64;
#[rustfmt::skip]
pub type AgentIdentifier = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConnectionClass {
    Owner,
    Network(NetworkPeer),
    OtherPersona(OtherPersonaEngine),
    System(SystemPrincipal),
    NonOwnerUser(UnixUserIdentifier),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AgentEndpointBinding {
    pub agent_identifier: AgentIdentifier,
    pub agent_endpoint: AgentEndpoint,
    pub harness_pid: HarnessPid,
    pub harness_start_time: HarnessStartTime,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadSubscription {
    pub thread_name: ThreadName,
    pub participant_name: ParticipantName,
    pub thread_relation_selection: ThreadRelationSelection,
}
#[rustfmt::skip]
pub type Participants = std::vec::Vec<ParticipantName>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AgentRegistryQuery {
    All,
    ByAgent(AgentIdentifier),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AgentRegistryRejectionReason {
    UnknownAgentIdentifier,
    StoreRejected,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OtherPersonaEngine {
    pub engine_identifier: EngineIdentifier,
    pub host: Host,
}
#[rustfmt::skip]
pub type EndpointPath = WirePath;
#[rustfmt::skip]
pub type SubmissionRejection = SubmissionRejectionReason;
#[rustfmt::skip]
pub type MessageSocketMode = SocketMode;
#[rustfmt::skip]
pub type SupervisionSocketMode = SocketMode;
#[rustfmt::skip]
pub type FeatureBranchName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ComponentName {
    Introspect,
    Terminal,
    System,
    Mind,
    Spirit,
    Message,
    Harness,
    Router,
    Orchestrate,
}
#[rustfmt::skip]
pub type WirePath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadEntry {
    pub message_slot: MessageSlot,
    pub message_sender: MessageSender,
    pub message_body: MessageBody,
    pub stamped_at: StampedAt,
}
#[rustfmt::skip]
pub type NetworkPeer = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StampedMessageSubmission {
    pub message_submission: MessageSubmission,
    pub message_origin: MessageOrigin,
    pub stamped_at: StampedAt,
}
#[rustfmt::skip]
pub type SupervisionSocketPath = WirePath;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MessageDaemonConfiguration {
    pub message_socket_path: MessageSocketPath,
    pub message_socket_mode: MessageSocketMode,
    pub supervision_socket_path: SupervisionSocketPath,
    pub supervision_socket_mode: SupervisionSocketMode,
    pub router_socket_path: RouterSocketPath,
    pub component_ingresses: ComponentIngresses,
    pub owner_identity: OwnerIdentity,
}
#[rustfmt::skip]
pub type MessageSender = String;
#[rustfmt::skip]
pub type RouterSocketPath = WirePath;
#[rustfmt::skip]
pub type ThreadName = String;
#[rustfmt::skip]
pub type SystemPrincipal = String;
#[rustfmt::skip]
pub type Threads = std::vec::Vec<ThreadSummary>;
#[rustfmt::skip]
pub type ThreadQuery = ThreadName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadContents {
    pub thread_name: ThreadName,
    pub thread_relation_selection: ThreadRelationSelection,
    pub participants: Participants,
    pub thread_entries: ThreadEntries,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptVariant {
    HumanPrompt,
    PeerMessage,
    DeliveryReceipt,
}
#[rustfmt::skip]
pub type SourceEventIdentifier = String;
#[rustfmt::skip]
pub type RawPromptText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptInterpretationSelection {
    Interpreted(MessageBody),
    None,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TypedPromptEnvelope {
    pub prompt_variant: PromptVariant,
    pub source_event_identifier: SourceEventIdentifier,
    pub raw_prompt_text: RawPromptText,
    pub prompt_interpretation_selection: PromptInterpretationSelection,
}
#[rustfmt::skip]
pub type TargetFlowName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowDeliveryRequest {
    pub typed_prompt_envelope: TypedPromptEnvelope,
    pub target_flow_name: TargetFlowName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowIdleAnnouncement {
    pub target_flow_name: TargetFlowName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryQueueState {
    Parked,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryQueuedAcknowledgment {
    pub source_event_identifier: SourceEventIdentifier,
    pub target_flow_name: TargetFlowName,
    pub delivery_queue_state: DeliveryQueueState,
}
#[rustfmt::skip]
pub type LandedAt = TimestampNanos;
#[rustfmt::skip]
pub type ByteCount = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompactReceipt {
    pub source_event_identifier: SourceEventIdentifier,
    pub landed_at: LandedAt,
    pub byte_count: ByteCount,
}
#[rustfmt::skip]
pub type LandedReceipts = std::vec::Vec<CompactReceipt>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowIdleAcknowledgment {
    pub target_flow_name: TargetFlowName,
    pub landed_receipts: LandedReceipts,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowDeliveryRejectionReason {
    UnknownFlow,
    StoreRejected,
}
#[rustfmt::skip]
pub type FlowDeliveryRejection = FlowDeliveryRejectionReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ClusterTarget {
    Primary,
    Secondary,
    Core,
}
#[rustfmt::skip]
pub type FlowIdentifier = String;
#[rustfmt::skip]
pub type SessionIdentifier = String;
#[rustfmt::skip]
pub type TranscriptPath = String;
#[rustfmt::skip]
pub type PromptFirstSixWords = String;
#[rustfmt::skip]
pub type PromptLastSixWords = String;
#[rustfmt::skip]
pub type PromptSha256 = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClusterMember {
    pub flow_identifier: FlowIdentifier,
    pub session_identifier: SessionIdentifier,
}
#[rustfmt::skip]
pub type ClusterMembers = std::vec::Vec<ClusterMember>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Context {
    pub flow_identifier: FlowIdentifier,
    pub source_turn_identifier: SourceTurnIdentifier,
    pub transcript_path: TranscriptPath,
    pub prompt_sha256: PromptSha256,
    pub what_living_said: WhatLivingSaid,
    pub context_about: ContextAbout,
    pub context_answered: ContextAnswered,
    pub context_corrected: ContextCorrected,
    pub context_uncertainties: ContextUncertainties,
}
#[rustfmt::skip]
pub type SourceTurnIdentifier = String;
#[rustfmt::skip]
pub type WhatLivingSaid = String;
#[rustfmt::skip]
pub type ContextAbout = String;
#[rustfmt::skip]
pub type ContextAnswered = String;
#[rustfmt::skip]
pub type ContextCorrected = String;
#[rustfmt::skip]
pub type ContextUncertainties = std::vec::Vec<String>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClusterRelay {
    pub flow_identifier: FlowIdentifier,
    pub session_identifier: SessionIdentifier,
    pub transcript_path: TranscriptPath,
    pub prompt_first_six_words: PromptFirstSixWords,
    pub prompt_last_six_words: PromptLastSixWords,
    pub prompt_sha256: PromptSha256,
    pub context: Context,
    pub timestamp_nanos: TimestampNanos,
    pub cluster_target: ClusterTarget,
    pub cluster_members: ClusterMembers,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerSender {
    pub flow_identifier: FlowIdentifier,
    pub session_identifier: SessionIdentifier,
}
#[rustfmt::skip]
pub type PeerSourcePath = String;
#[rustfmt::skip]
pub type PeerBodySha256 = String;
#[rustfmt::skip]
pub type PeerBody = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PeerEnvelope {
    pub peer_sender: PeerSender,
    pub source_event_identifier: SourceEventIdentifier,
    pub peer_source_path: PeerSourcePath,
    pub peer_body_sha256: PeerBodySha256,
    pub peer_body: PeerBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ClusterMessage {
    Relay(ClusterRelay),
    Peer(PeerEnvelope),
}
#[rustfmt::skip]
pub type TargetFlows = std::vec::Vec<FlowIdentifier>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryRequest {
    pub source_event_identifier: SourceEventIdentifier,
    pub cluster_message: ClusterMessage,
    pub target_flows: TargetFlows,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReceiptKind {
    Accepted,
    TranscriptWitnessed,
    Parked,
    FileOnly,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecipientReceipt {
    pub flow_identifier: FlowIdentifier,
    pub receipt_kind: ReceiptKind,
}
#[rustfmt::skip]
pub type RecipientReceipts = std::vec::Vec<RecipientReceipt>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryReport {
    pub source_event_identifier: SourceEventIdentifier,
    pub recipient_receipts: RecipientReceipts,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Submit(MessageSubmission),
    SubmitStamped(StampedMessageSubmission),
    QueryInbox(InboxQuery),
    AssignAgentIdentity(AgentIdentityAssignment),
    BindAgentEndpoint(AgentEndpointBinding),
    QueryAgentRegistry(AgentRegistryQuery),
    QueryThread(ThreadQuery),
    SubscribeThread(ThreadSubscription),
    QueryThreads(ThreadIndexQuery),
    FlowDeliver(FlowDeliveryRequest),
    FlowAnnounceIdle(FlowIdleAnnouncement),
    Deliver(DeliveryRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    SubmissionAccepted(SubmissionAcceptance),
    SubmissionRejected(SubmissionRejection),
    InboxListing(InboxListingReply),
    AgentIdentityAssigned(AssignedAgentIdentity),
    AgentEndpointBound(BoundAgentEndpoint),
    AgentRegistryListing(AgentRegistryListingReply),
    AgentRegistryRejected(AgentRegistryRejection),
    MessageRequestUnimplemented(MessageRequestUnimplementedReply),
    Error(ErrorReport),
    ThreadListing(ThreadContents),
    ThreadSubscribed(ThreadSubscriptionAcknowledgment),
    ThreadIndexListing(ThreadIndexEntries),
    ThreadRejected(ThreadRejection),
    DeliveryQueued(DeliveryQueuedAcknowledgment),
    DeliveryLanded(CompactReceipt),
    FlowDeliveryRejected(FlowDeliveryRejection),
    FlowIdleAcknowledged(FlowIdleAcknowledgment),
    DeliveryRecorded(DeliveryReport),
}
