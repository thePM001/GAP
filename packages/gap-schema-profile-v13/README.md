# gap-schema-profile-v13 0.2.1

## What it is

This package is the live Admit profile of the GAP meta schema v1.3. It holds the reader of GAP source, the embedded live meta schema and the four walls that read an instruction. It is the one source of that profile and AEP holds a byte pinned copy of it.

## Place in GAP

The package is the instruction side of Layer 3 in section 03 of the root README, Write. Check. Admit. Its walls enforce the governance fields of section 05, Structural Governance. gapc uses its reader and two of its walls for Layer 2.

## What ships

| Wall id | Closes when |
| --- | --- |
| `gap:leftover_rank_field` | The metadata carries `trust_ring`, a rank, a trust score or a trust tier. Presence alone is Deny |
| `gap:agent_permission` | The request names an agent or an action, the instruction declares `agent_permission` and no grant matches. A declared empty list closes on every agent action |
| `gap:wrap:bind` | The instruction names a wrap or an action path prefix and neither binds the request |
| `gap:pattern:guard` | The guard is a structured object without an expression. It also closes when the guard is neither a text nor an object |

- All four walls are judged in one pass and the result lists every closed wall. The answer is allow only when no wall closed.
- An instruction without `agent_permission` leaves the agent permission wall open, so other walls decide.
- The field `enabled` is read at load time. An instruction with the value false is still judged.
- The reader is a full YAML 1.2 reader that also reads JSON. Reals such as `weight: 1.0` read as numbers and a block scalar does not hide the fields below it.
- Tests prove each point above and the four fixtures under `packages/gap-schema-profile-v13/fixtures/` are the reference instructions for them.

## Interfaces

The binary `gap-schema-profile-v13` reads lines of the form key=value from standard input and judges one request:

| Key | Meaning |
| --- | --- |
| `source` | The instruction as JSON-encoded GAP on one line |
| `agent_id` | The agent of the request |
| `action` | The action of the request |
| `wrap` | The wrap of the node that asks |
| `action_path` | The action path of the request |

It prints `allow=true` or `allow=false` and then one line per closed wall with the wall id, a bar and the reason. A request of agent-b against an instruction that grants the action to agent-a prints:

```
allow=false
closed=gap:agent_permission:agent-b:finance.pay|this agent does not have permission for this action
```

The library exposes `live_admit_gap_profile` for the whole pass, one function per wall, `admit_collect_all` for the collect-all rule and `read_gap_documents`, `parse_gap_source` and `parse_yaml_gap` for the reader. The constant `SCHEMA_V13_JSON` holds the live meta schema, a byte copy of `schemas/gap-meta-schema-v1.3.json`.

## Limits

The package judges an instruction against one request and nothing else. It does not validate an instruction against the meta schema, which is the work of gapc. It does not freeze a clock, wait for the kernel pulse or apply an action, which a Base Node of AEP does. The walls that read the sealed capsule live in the closed wall package.

## Run and test

From the folder `packages`:

```
cargo build --locked -p gap-schema-profile-v13
cargo test --locked -p gap-schema-profile-v13
```

To judge a request by hand, write the five keys into a file with one key per line. Then pass the file to `target/debug/gap-schema-profile-v13` on standard input.

## Related documents

- [The root README](../../README.md), section 05 for the governance fields.
- [docs/LIVE-EVALUATION.md](../../docs/LIVE-EVALUATION.md) for the path of one action through Admit.
- [docs/GAP-AND-AEP.md](../../docs/GAP-AND-AEP.md) for the pinned copy that AEP holds.
- [VERSIONING.md](../../VERSIONING.md) for the rule that links this package to the schema version.
