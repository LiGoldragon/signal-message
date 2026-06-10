use schema_rust_next::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-message",
        "0.3.0",
        "SIGNAL_MESSAGE_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
