# GAP version policy

GAP carries four kinds of version number. This file states what each one means and how they move together. The values live in `gap-versions.toml` and CI fails when the repository disagrees with that file.

## Current values

| Key in gap-versions.toml | Value | Meaning |
| --- | --- | --- |
| language_spec | 1.1 | GAP language specification version |
| live_schema | 1.3 | Meta schema that live Admit and `gapc check` use |
| aep | 2.9.0 | AEP release this GAP tree is aligned to |
| gap-schema-profile-v13 | 0.2.0 | Live Admit profile crate |
| gap-closed-wall-deny | 0.1.0 | Closed wall Deny crate |
| gapc | 0.1.0 | GAP compiler crate |

## Language specification

- The language specification version names the GAP language. It changes when the meaning of a GAP construct changes.
- A new language specification version needs a new meta schema version when the legal document shape changes.

## Meta schemas

- Each meta schema lives at `schemas/gap-meta-schema-v<version>.json` and its `$id` is `https://gap-lang.aep.dev/schemas/gap-meta-schema-v<version>.json`. The `$id` of a published schema never changes.
- The schema named by `live_schema` is the one live Admit and `gapc check` validate against. `packages/gap-schema-profile-v13/src/schema_v13.body` is a byte copy of it and the crate loads it with `include_str!`.
- A field may be added to the live schema as an optional property. Removing a field, making a field required or tightening a rule needs a new schema version.
- `schemas/gap-meta-schema-v1.1.json` and `schemas/gap-meta-schema-v1.2.json` are historical schemas for old documents.
- The old root file names `GAP meta schema v1.json`, `GAP meta schema v1.2.json` and `GAP meta schema v1.3.json` are byte copies of the kebab case files for one release. They are removed in the release after aep 2.9.0.

## Crates

- Crates follow semantic versioning on their own. A crate version does not copy the AEP version.
- A change to a public item or to an Admit outcome is a breaking change. Before 1.0.0 a breaking change raises the minor number.
- `gapc --version` prints the gapc version, the profile crate version and the live schema version.
- Crates are not published to crates.io. Publishing is an operator decision.

## AEP alignment

- The `aep` value names the AEP release whose live evaluation this tree teaches. The README names that release and no other.
- Every fixture carries `metadata.aep_version` equal to the `aep` value.
- AEP keeps a byte pinned copy of the profile crate, the meta schemas and `gap-versions.toml` together with the GAP commit they came from. AEP CI fails when a pinned byte differs or when the pinned `aep` value differs from the AEP release.
- Raising the `aep` value is one GAP commit that updates `gap-versions.toml`, the README and the fixtures, followed by a new AEP pin of that commit.

## How CI enforces it

- `cargo test` runs `packages/gapc/tests/repo_policy.rs`. It checks schema names, legacy aliases, the one schema body, crate versions, README version mentions, fixture `aep_version` values and this file.
- `gapc check packages` validates every fixture against the live schema and the load time profile walls.
