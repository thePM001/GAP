# vendor claim fixture
- `schemas/` holds GAP meta-schema v1 and v1.2 as optional Layer 1 authoring constraint masks.
- `policies/reference/` holds reference `.gap` instructions.
- `lib/gap-compile.mjs` compiles GAP profiles.
- Coding governance validation runs in `AEP-Subprotocols/coding-governance/` in Rust.
- `FILE-FORMAT.md` holds file format notes.
node lib/gap-compile.mjs --list-profiles
