#[test]
fn message_contract_is_schema_derived_without_retired_helper_dependencies() {
    let cargo_toml = include_str!("../Cargo.toml");
    let source = include_str!("../src/lib.rs");

    assert!(
        cargo_toml.contains("schema-rust-next"),
        "schema-rust-next owns generated contract emission",
    );
    assert!(
        cargo_toml
            .lines()
            .any(|line| line.trim() == "build        = \"build.rs\""),
        "contract artifacts must be generated from schema/lib.schema",
    );
    assert!(
        !cargo_toml.contains("signal-engine-management"),
        "wire contracts must not drag old engine-management helper types forward",
    );
    assert!(
        !cargo_toml.contains("signal-persona-origin"),
        "message origin vocabulary is schema-local until a schema-derived shared origin contract exists",
    );
    assert!(
        !source.contains("signal_channel!"),
        "signal_channel! is deprecated; signal-message is schema-derived",
    );
    assert!(
        cargo_toml.contains("default = [\"nota-text\"]"),
        "direct signal-message users keep the NOTA projection by default",
    );
    assert!(
        cargo_toml.contains("nota-text = [\"dep:nota-next\", \"signal-frame/nota-text\"]"),
        "generated NOTA traits and signal-frame NOTA support are gated through the local feature",
    );
}

#[test]
fn binary_only_dependency_tree_does_not_contain_nota_next() {
    let manifest = CargoManifest::from_environment();
    let tree = manifest.cargo_tree(&["--edges", "normal", "--no-default-features"]);

    assert!(
        !tree.contains("nota-next") && !tree.contains("nota_next"),
        "binary-only dependency tree must not contain nota-next:\n{tree}"
    );
}

struct CargoManifest {
    path: std::path::PathBuf,
}

impl CargoManifest {
    fn from_environment() -> Self {
        Self {
            path: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
        }
    }

    fn cargo_tree(&self, arguments: &[&str]) -> String {
        let output = std::process::Command::new("cargo")
            .arg("tree")
            .arg("--manifest-path")
            .arg(self.path())
            .args(arguments)
            .output()
            .expect("run cargo tree");

        assert!(
            output.status.success(),
            "cargo tree failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        String::from_utf8(output.stdout).expect("cargo tree stdout is utf8")
    }

    fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }
}
