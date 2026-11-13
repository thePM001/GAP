# gapc 0.1.0

## What it is

gapc is the GAP compiler. It reads `.gap` sources in YAML 1.2 or JSON with the same reader that live Admit uses, checks every instruction and writes JSON-encoded GAP. It is the load check of GAP, so an instruction that passes gapc has the shape a runtime accepts.

## Place in GAP

gapc is Layer 2 of section 03 in the root README, Write. Check. Admit. It enforces the composition rules of section 01, the instruction shape of section 02 and the field ranges of sections 04 and 05.

## What ships

- The check of one instruction against the live meta schema v1.3, with every schema error reported by its path.
- The two load time walls of the profile package. The wall `gap:leftover_rank_field` closes on a leftover rank field and the wall `gap:pattern:guard` closes on a structured guard without an expression. Both close on the document alone, so such an instruction could never pass Admit.
- The read of several instructions from one YAML source. A document without an `address` is skipped and counted.
- The compile of a source to JSON-encoded GAP. A source that fails the check is not compiled.
- Tests that prove each point above, including that the JSON form and the YAML form of one instruction compile to the same object.

## Interfaces

| Command | Result |
| --- | --- |
| `gapc check <path>...` | Checks each file and every `.gap` file under each folder, in sorted order |
| `gapc compile <file>` | Writes the JSON-encoded GAP to standard output |
| `gapc compile <file> --out <file>` | Writes the JSON-encoded GAP to a file |
| `gapc schema` | Prints the meta schema that the check uses |
| `gapc version` | Prints the versions of gapc, of the profile package and of the meta schema |

The exit status is 0 for a clean run, 1 when a source fails the check and 2 for a usage or file error. A failure is printed to standard error as one line with the file, the number of the document and the message. One source with one instruction compiles to one JSON object and a source with several instructions compiles to a JSON array.

The library exposes the same steps as functions: `Checker::new`, `Checker::check_source`, `Checker::schema_errors`, `read_documents`, `compile_source`, `load_time_wall_problems` and `collect_gap_files`.

## Limits

gapc does not judge an action. The agent permission wall and the wrap bind wall depend on a request, so they are judged at Admit by the profile package and by a runtime. gapc does not execute an instruction, does not run a scanner and does not evaluate a covenant. It also does not compile sandbox profiles for AEP runtimes, which is a script of the AEP repository.

## Run and test

From the folder `packages`:

```
cargo build --locked -p gapc
target/debug/gapc check .
target/debug/gapc compile gap-schema-profile-v13/fixtures/writing.gap
cargo test --locked -p gapc
```

The tests of this package include the repository policy test at `packages/gapc/tests/repo_policy.rs`, which holds the schemas, the versions and the README files of the tree consistent.

## Related documents

- [The root README](../../README.md) with the language and the quick start.
- [docs/FILE-FORMAT.md](../../docs/FILE-FORMAT.md) with the rules of a GAP file.
- [The profile package](../gap-schema-profile-v13/README.md), whose reader and walls gapc uses.
- [VERSIONING.md](../../VERSIONING.md) with the version rules.
