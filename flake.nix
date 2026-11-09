{
  description = "signal-message — Signal contract for Message ingress";

  inputs.base.url = "github:LiGoldragon/build-base";

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix shell github:LiGoldragon/build-base#toolchain github:LiGoldragon/build-base#crate2nix -c crate2nix generate
  outputs =
    { self, base }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;

      # Every test runs with every feature on.
      memberChecks = { check, ... }: { signal-message = check "signal-message" [ "default" "datom" ]; };

      extraChecks =
        { pkgs, toolchain, check, doc, clippy, ... }:
        let
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./src ./tests ];
          };
        in
        {
          # The tests under the default feature set alone.
          test-default = check "signal-message" [ "default" ];
          fmt = pkgs.runCommand "signal-message-fmt" { nativeBuildInputs = [ toolchain ]; } ''
            rustfmt --check --edition ${manifest.package.edition} ${rust}/src/lib.rs ${rust}/tests/*.rs
            touch $out
          '';
          clippy = clippy "signal-message" [ "default" "datom" ] [ "-D" "warnings" ];
          doc = doc "signal-message" [ "default" "datom" ] [ "-D" "warnings" ];
        };
    };
}
