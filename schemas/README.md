# GAP meta schemas

A meta schema is the JSON Schema that every GAP instruction must satisfy. This folder contains three of them and one is live.

| File | Status | Use |
| --- | --- | --- |
| `schemas/gap-meta-schema-v1.3.json` | Live | The schema that `gapc check` and live Admit validate against |
| `schemas/gap-meta-schema-v1.2.json` | Historical | Kept so that old documents can still be read |
| `schemas/gap-meta-schema-v1.1.json` | Historical | Kept so that old documents can still be read |

## Rules

- The id of a schema is `https://gap-lang.aep.dev/schemas/` followed by its file name. The id of a published schema never changes.
- The live schema is closed at the top level and in `metadata`, so an instruction with an unknown field fails the check.
- A field may be added to the live schema as an optional property. Removing a field, making a field required or tightening a rule needs a new schema version.
- The profile package embeds a byte copy of the live schema at `packages/gap-schema-profile-v13/src/schema_v13.body`. A test fails when the two files differ.

## The old names at the root

The repository root still carries `GAP meta schema v1.json`, `GAP meta schema v1.2.json` and `GAP meta schema v1.3.json`. They are byte copies of the three files in this folder under their old names and they are kept for one release. [VERSIONING.md](../VERSIONING.md) names the release that removes them. New work refers to the files in this folder.

## How to read the live schema

The eleven top level fields are listed in the root README with one line each. The blocks under `$defs` contain the detail: `Composition` for the six composition types, `Action` for the five action types, `Metadata` for the governance fields and `GenerationConstraints` for self-generation. From the folder `packages` the command `target/debug/gapc schema` prints the same schema that the check uses.
