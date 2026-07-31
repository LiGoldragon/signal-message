use protos::WireContractFamily;
use schema_rust::build::{ContractCrateBuild, CrateName, SchemaVersion, UpdateEnvironmentVariable};

fn main() {
    ContractCrateBuild::from_environment(
        CrateName::new("signal-message"),
        SchemaVersion::new("0.5.0"),
        UpdateEnvironmentVariable::new("SIGNAL_MESSAGE_UPDATE_SCHEMA_ARTIFACTS"),
        WireContractFamily::SignalSpirit,
    )
    .expect_fresh();
}
