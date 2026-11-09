# Changelog

This is the public GAP open-source tree. Dated product changes belong here. README teaching and the package folders are not the same set of work and this file does not claim the public tree is fully upgraded.

## 2026-11-09 - Host extensions in the live schema

- Meta schema v1.3 accepts an optional top-level `extensions` object. Each key names one host and holds an object whose fields that host defines. gapc and live Admit ignore the content. AEP keeps its own policy keys such as `lrp_id`, `ede_repos` and the CAW `subprotocols` block under `extensions.aep`, so its reference policies pass `gapc check` while metadata remains closed.
- The change adds an optional field, so the schema version remains 1.3 as VERSIONING.md allows. The schema `$id` is unchanged.
- `gap-schema-profile-v13` is 0.2.1 because its embedded schema body changed. No Admit outcome changed.
- A gapc test proves that a host object passes, that a host value which is not an object fails and that a host name outside lower case letters, digits and underscores fails.

## 2026-11-08 - GAP CI, gapc and one source for the profile crate

GAP now checks itself. `.github/workflows/ci.yml` runs cargo fmt, cargo clippy with warnings denied, cargo test and `gapc check` over every fixture on each push from a clean checkout.

- `packages/gapc` is the GAP compiler. `gapc check` validates `.gap` sources against the live meta schema v1.3 and the load time profile walls. `gapc compile` writes JSON-encoded GAP.
- Meta schemas moved to kebab case names under `schemas/`. The v1 file is `schemas/gap-meta-schema-v1.1.json` to match its `$id`. The old root names remain as byte copies for one release.
- Meta schema v1.3 accepts the optional `metadata.aep_version` field that live AEP reference policies carry.
- `gap-schema-profile-v13` 0.2.0 is the one source of the live Admit profile. It takes the live AEP semantics: trust_score and trust_tier close the leftover rank wall, an undeclared agent_permission leaves that wall open and a declared empty list closes it. The crate depends on serde_json and yaml-rust2 only and AEP holds a byte pinned copy.
- The profile reader is a full YAML 1.2 reader. The old reader stopped at the first block scalar and dropped every field below it, so a trust_ring under a block scalar never reached the leftover rank wall. Reals such as `weight: 1.0` now read as numbers.
- Fixtures validate against the live schema: the duplicate pattern wrap and prefix keys are gone, agent_permission names agent-a as an object record and aep_version is 2.9.0.
- README examples pass `gapc check`. The YAML example quotes `guard: "true"` and the JSON example uses a template action.
- `VERSIONING.md` and `gap-versions.toml` link the language specification, schema, crate and AEP versions. The README names AEP 2.9.0.
- The orphan `src/probe.rs` file is removed.

## 2026-09-12 - agent_permission on the public tree

Live README teaching now names collect-all Admit after freeze-at-seal and the 1000 ms kernel pulse, with Layer 1 constrained decoding as optional authoring rather than Admit. Live trust bundle teaching is sha256-structure, presence of trust_ring is Deny and agent permission is agent_permission.

Public schema, the profile package, README and fixtures now name agent_permission. Remaining package work is the leftover rank wall id, closed-wall capsule digest teaching of sha256-structure while the package still hashes with FNV-1a, package license files that still say UNLICENSED and the fit document that still claims no remaining holes.

This snapshot does not ship a teaching envelope that runs collect-all then Apply, a teaching runtime that loads, seals, freezes, waits and Applies after Admit or a constrained decode engine that takes language model API inputs.
