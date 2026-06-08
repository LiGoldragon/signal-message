#[test]
fn message_contract_declares_local_nota_text_feature_for_signal_frame_macros() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(
        cargo_toml.contains("default = [\"nota-text\"]"),
        "direct signal-message users keep the NOTA projection by default",
    );
    assert!(
        cargo_toml.contains("nota-text = [\"signal-frame/nota-text\"]"),
        "signal-frame macro-generated NOTA traits are gated through a local feature",
    );
}
