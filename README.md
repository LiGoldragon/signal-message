# signal-message

The ordinary Message ingress Interface. It carries the client-to-message and
message-to-router relations in one typed family.

`ethos/interface.ethos` is the sole authored Interface projection. The build
assembles it as an authority-approved transaction and checks the committed
strict Rust projection in `src/schema/lib/generated.rs`. Every Type is exposed
only by its encoded identity. The current bootstrap stage keeps `Input` and
`Output` as explicit handwritten roles over those Types; Dotos retains the
human domain names at the textual boundary.

The request role contains `Submit`, `SubmitStamped`, `QueryInbox`, agent
registry operations, and thread operations. The reply role contains the
corresponding accepted, rejected, listing, and unimplemented outcomes. Runtime
provenance stamping, persistence, routing, sockets, and supervision belong to
the consuming components.

Run `nix --option substituters https://cache.nixos.org flake check
--print-build-logs` for the complete proof matrix.
