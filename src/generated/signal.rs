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
    SubmitPrompt,
    AssignAgentIdentity,
    BindAgentEndpoint,
    Header,
    Reconcile,
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
    pub prompt_relay_permissions: PromptRelayPermissions,
    pub owner_identity: OwnerIdentity,
}
#[rustfmt::skip]
pub type MessageSender = String;
#[rustfmt::skip]
pub type RouterSocketPath = WirePath;
#[rustfmt::skip]
pub type SourceAgentIdentifier = String;
#[rustfmt::skip]
pub type DestinationAgentIdentifier = String;
#[rustfmt::skip]
pub type PromptDeliveryProtocolVersion = i64;
#[rustfmt::skip]
pub type PromptDeliveryPayloadLength = i64;
#[rustfmt::skip]
pub type DispatcherProcessId = i64;
#[rustfmt::skip]
pub type DispatcherProcessStartTime = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptDeliveryIdentity {
    pub source_agent_identifier: SourceAgentIdentifier,
    pub destination_agent_identifier: DestinationAgentIdentifier,
    pub source_event_identifier: SourceEventIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptDeliveryHeader {
    pub prompt_delivery_protocol_version: PromptDeliveryProtocolVersion,
    pub prompt_delivery_identity: PromptDeliveryIdentity,
    pub prompt_delivery_payload_length: PromptDeliveryPayloadLength,
    pub dispatcher_process_id: DispatcherProcessId,
    pub dispatcher_process_start_time: DispatcherProcessStartTime,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptRelayPermission {
    pub source_agent_identifier: SourceAgentIdentifier,
    pub destination_agent_identifier: DestinationAgentIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptRelayDelivery {
    pub source_agent_identifier: SourceAgentIdentifier,
    pub destination_agent_identifier: DestinationAgentIdentifier,
    pub message_origin: MessageOrigin,
    pub typed_prompt_envelope: TypedPromptEnvelope,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptTargetReadiness {
    Ready,
    Busy,
    Dirty,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptDispatchRequest {
    pub destination_agent_identifier: DestinationAgentIdentifier,
    pub source_agent_identifier: SourceAgentIdentifier,
    pub source_event_identifier: SourceEventIdentifier,
    pub prompt_target_readiness: PromptTargetReadiness,
}
#[rustfmt::skip]
pub type PromptRelayPermissions = std::vec::Vec<PromptRelayPermission>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptRelaySubmission {
    pub destination_agent_identifier: DestinationAgentIdentifier,
    pub typed_prompt_envelope: TypedPromptEnvelope,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptReceiptObservation {
    pub destination_agent_identifier: DestinationAgentIdentifier,
    pub source_agent_identifier: SourceAgentIdentifier,
    pub source_event_identifier: SourceEventIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptRelayDeliveryDisposition {
    Pending,
    Busy,
    Dirty,
    RecordedOnly,
    DuplicatePending,
    InFlight,
    ByteAccepted,
    RecipientObserved,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptRelayAcceptance {
    pub source_event_identifier: SourceEventIdentifier,
    pub prompt_relay_delivery_disposition: PromptRelayDeliveryDisposition,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptRelayRejectionReason {
    RelayDisabled,
    UnregisteredSource,
    DestinationNotPermitted,
    StoreRejected,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptRelayRejection {
    pub prompt_relay_rejection_reason: PromptRelayRejectionReason,
}
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Submit(MessageSubmission),
    SubmitStamped(StampedMessageSubmission),
    SubmitPrompt(PromptRelaySubmission),
    ObservePromptReceipt(PromptReceiptObservation),
    DispatchPrompt(PromptDispatchRequest),
    Header(PromptDeliveryHeader),
    Reconcile(PromptDeliveryIdentity),
    QueryInbox(InboxQuery),
    AssignAgentIdentity(AgentIdentityAssignment),
    BindAgentEndpoint(AgentEndpointBinding),
    QueryAgentRegistry(AgentRegistryQuery),
    QueryThread(ThreadQuery),
    SubscribeThread(ThreadSubscription),
    QueryThreads(ThreadIndexQuery),
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
    PromptRelayAccepted(PromptRelayAcceptance),
    PromptRelayRejected(PromptRelayRejection),
    HeaderAccepted(PromptDeliveryIdentity),
    HeaderRejected(PromptDeliveryIdentity),
    PayloadAbsent(PromptDeliveryIdentity),
    PayloadInProgress(PromptDeliveryIdentity),
    PayloadComplete(PromptDeliveryIdentity),
    RecipientObserved(PromptDeliveryIdentity),
}
