# GAP: Governed Agentic Programming

GAP is the native instructions language for governed agency in the Agent Element Protocol and an instruction is the atomic unit so agents, workflows, validators, compositions and governance rules are all instructions that generate further instructions.

Live evaluation in AEP 2.9.0 is described in docs/LIVE-EVALUATION.md.

## Public GAP tree

Classic GAP on this repository is the public language surface for schema files, product packages and this README.

- Schema files live in schemas: `schemas/gap-meta-schema-v1.1.json`, `schemas/gap-meta-schema-v1.2.json` and `schemas/gap-meta-schema-v1.3.json`. The live schema is v1.3.
- The old root names `GAP meta schema v1.json`, `GAP meta schema v1.2.json` and `GAP meta schema v1.3.json` are byte copies kept for one release. VERSIONING.md names the release that removes them.
- Product packages live under packages. Live package folders include `packages/gap-schema-profile-v13/`, `packages/gap-closed-wall-deny/` and `packages/gapc/`.
- `gapc` in `packages/gapc/` checks every `.gap` source against the live schema and the load time profile walls and compiles it to JSON-encoded GAP. GAP compiles its own fixtures with it.
- Run `gapc check` on every edited `.gap` file before the vendor compile script.
- File format notes live at https://github.com/thePM001/AEP-agent-element-protocol path AEP-Components/gap/FILE-FORMAT.md
- How AEP uses GAP, with the sandbox profile for a coding agent, is described in docs/GAP-AND-AEP.md

## Three-layer enforcement

GAP uses three layers and no layer requires the LLM to have seen GAP during training.

- Authoring a `.gap` file is an instruction write and Layer 3 Admit runs after a sealed capsule.
- Layer 1 constrained decoding is optional authoring. Constrained decoding is not Admit.
- Layer 2 structural validation runs after generation at load.
- Layer 3 Base Node kernel is closed-wall collect-all Admit after freeze-at-seal and the 1000 ms kernel pulse, then Apply.
- Any LLM can author GAP instructions.
- This snapshot does not ship a wired Layer 1 Admit engine.
- The lattice enforces policy together after the kernel pulse.

## Live evaluation

The AEP 2.9.0 live path seals a lattice-channel capsule, freezes the clock at seal and waits 1000 ms for the compiled Base Node kernel pulse. Every check then runs together as collect-all Admit and the allowed action is Applied.

The detail of that path, the derived ledger of fifteen rows, the kernel pulse and the live trust bundle are described in docs/LIVE-EVALUATION.md.

## File format

The rules of a GAP file are described in docs/FILE-FORMAT.md.

### YAML GAP source

> ```yaml
> address:
>   domain: com.example.finance
>   id: pay.v1
> pattern:
>   guard: "true"
> action:
>   type: template
>   content: pay
> weight: 1.0
> composition:
>   type: atomic
> metadata:
>   provenance: "system.seed"
>   version: "1.0.0"
>   stability: experimental
>   wrap: finance
>   agent_permission:
>     - finance.pay
> ```

### JSON-encoded kernel policy

AEP 2.9.0 reference policies are JSON objects with pattern.guard and the same instruction fields. The source below is legal GAP because the kernel reads the instruction object not the skin.

> ```json
> {
>   "address": {"domain": "aep.reference.writing", "id": "conventions.v1"},
>   "pattern": {"guard": "true"},
>   "action": {"type": "template", "content": "Apply AEP 2.9.0 writing conventions."},
>   "weight": 1.0,
>   "composition": {"type": "atomic"},
>   "metadata": {"provenance": "AEP 2.9.0", "version": "1.0.0", "stability": "stable"}
> }
> ```

## First example

### Minimal instruction

> ```yaml
> address:
>   domain: com.myorg.dev
>   id: code-reviewer.v1
> pattern: |
>   Review pull requests for correctness, style and security.
> action:
>   type: structured
>   schema: CodeReview
>   structured_generation: true
>   content: |
>     You review pull requests for correctness, style and security.
>     Every comment must reference a specific file and line number.
> weight: 0.90
> composition:
>   type: atomic
> metadata:
>   provenance: "system.seed"
>   version: "1.0.0"
>   stability: stable
>   grade: 8
>   agent_permission:
>     - review.pull_request
> execution:
>   retry:
>     max_attempts: 3
>     backoff: exponential
>   timeout_ms: 30000
>   on_exhaustion: fail
> types:
>   CodeReview:
>     format: json
>     fields:
>       summary: string
>       issues_found: boolean
>       verdict:
>         type: enum
>         values: [approve, request_changes, comment_only]
> ```

- Instruction-runtime retry lives on the instruction.
- Live Admit is collect-all after the kernel pulse.

### With governance

> ```yaml
> metadata:
>   agent_permission:
>     - review.pull_request
>   wrap: governance
>   scanners:
>     - pii
>     - secrets
>     - injection
>   covenants:
>     - "every comment must reference a file and line number [hard]"
>     - "verdict must be one of: approve, request_changes, comment_only [hard]"
>   proof:
>     sign: true
>     algorithm: ed25519
>     ledger: true
>   budget:
>     max_tokens: 30000
>     max_cost: 0.50
> ```

- The proof.algorithm field names an optional proof algorithm.
- Live AEP 2.9.0 EPSCOM trust bundle mode is sha256-structure so Ed25519 and ML-DSA-65 stay optional rather than the default live bundle.
- Agent permission is `agent_permission` so Agent A may X and Agent B may Y.
- A declared empty agent_permission list closes every agent action. A document without agent_permission leaves agent permission to other walls.
- Presence of trust_ring is Deny.
- Presence of trust_ring on a live GAP document is Deny.
- Closed wall gap:leftover_rank_field.
- Agent permission is agent_permission.
- Do not set trust_ring on live documents.
- `enabled` is load-time: when false the instruction is still loaded and live Admit still evaluates walls.
- A closed-wall close names the closed walls, the reasons and a prescribed repair.
- A retry must seal a new capsule.

### With self-generation

> ```yaml
> composition:
>   type: abstraction
>   self_generate: true
>   generation_constraints:
>     max_variants: 10
>     min_quality_threshold: 0.92
>     lrt:
>       enabled: true
>       significance: 0.01
> ```

## Governance

- Every instruction carries governance inline.

- agent_permission: agent permission grants. Agent A may X. Agent B may Y.
- wrap: lattice wrap or action path prefix this wall binds to. Writing and security stay always on.
- action_path_prefix: action path prefix this wall binds to.
- scanners: PII, Injection, Secrets, Jailbreak, Toxicity, URL, Data Profiler, Prediction, Brand, Regulatory and Temporal.
- covenants: behavioural constraints with `[hard]` or `[soft]` severity.
- budget: token and cost limits recorded on the derived ledger.
- proof: live AEP 2.9.0 EPSCOM trust bundle mode is sha256-structure. Optional proof algorithms may name Ed25519.
- fleet: multi-instance governance with spawn policy.
- knowledge: scoped knowledge base with anti-context-rot.
- tools: allowed and forbidden tool lists.
- aspect: classification label used on the derived-ledger intent-drift row.
- Same sealed bytes plus same governance state is replay and replay is keyed by capsule hash.

## GAP toolchain

Classic GAP source lives on this repository and `.gap` files stay GAP source.

- Live Admit still waits for freeze-at-seal, the 1000 ms kernel pulse and collect-all walls.
- Agent permission is agent_permission.
- Presence of trust_ring is Deny.
- Live AEP 2.9.0 EPSCOM trust bundle mode is sha256-structure.
- `gapc check <path>...` validates `.gap` sources and `gapc compile <file>` writes JSON-encoded GAP. Both read GAP source with the reader live Admit uses.
- Compile CAW profiles with the vendor compile script at https://github.com/thePM001/AEP-agent-element-protocol path AEP-Components/gap/lib/gap-compile.mjs
- The live classic GAP binaries live in packages/gap-schema-profile-v13/, packages/gap-closed-wall-deny/ and packages/gapc/.
- `gapc` ships in packages/gapc/. There is no `structural` CLI here.

## GAP commands

> ```
> cargo run --manifest-path packages/gap-schema-profile-v13/Cargo.toml --bin gap-schema-profile-v13
> cargo run --manifest-path packages/gap-closed-wall-deny/Cargo.toml --bin gap-closed-wall-deny
> ```

Build and run gapc from the packages workspace:

> ```
> cd packages
> cargo build -p gapc
> target/debug/gapc check .
> target/debug/gapc compile gap-schema-profile-v13/fixtures/writing.gap
> ```

## Adoption ladder

Migrate incrementally because each rung adds governance without rewriting existing logic.

- Rung 1 Structured generation: optional Layer 1 authoring when a decoder is wired.
- Rung 2 Types and constraints: mathematical type validation.
- Rung 3 Covenants and scanners: behavioural governance plus content scanning.
- Rung 4 Composition: sequence, conditional, parallel orchestration.
- Rung 5 Self-generation: automatic specialization from quality data.
- Rung 6 Full governance: agent_permission, sha256-structure, collect-all Admit then Apply.

## Repository files

This OSS snapshot does not include `GAP v1 spec sheet.md` or `BIOSECURITY.md` because those files live in the GAPLUNE tree and are not copied here.

- `README.md` is this file. It teaches live evaluation plus optional Layer 1 authoring.
- Schema files live in schemas: `schemas/gap-meta-schema-v1.1.json`, `schemas/gap-meta-schema-v1.2.json` and `schemas/gap-meta-schema-v1.3.json`. The live schema is v1.3.
- The old root names `GAP meta schema v1.json`, `GAP meta schema v1.2.json` and `GAP meta schema v1.3.json` are byte copies kept for one release. VERSIONING.md names the release that removes them.
- Product packages live under packages. Named package folders include `packages/gap-schema-profile-v13/`, `packages/gap-closed-wall-deny/` and `packages/gapc/`.
- File format notes live at https://github.com/thePM001/AEP-agent-element-protocol path AEP-Components/gap/FILE-FORMAT.md
- `packages/gapc/` is the GAP compiler. CAW profile compile scripts live at https://github.com/thePM001/AEP-agent-element-protocol path AEP-Components/gap/lib/gap-compile.mjs
- `VERSIONING.md` links the language specification, schema, crate and AEP versions. `gap-versions.toml` holds the values.
- Reference instructions live at https://github.com/thePM001/AEP-agent-element-protocol path AEP-Components/gap/policies/reference/
- A dated fit analysis of classic GAP written against the earlier 2.8.5 release is at docs/CLASSIC-GAP-VS-AEP-2.8.5.md

## Comparison: GAP vs dottxt

The comparison of GAP with dottxt and with constrained decoding is in docs/GAP-AND-CONSTRAINED-DECODING.md.

## License

- VERSIONING.md links the language specification, schema, crate and AEP versions.
- License Apache-2.0. GAP is Governed Agentic Programming, language specification v1.1 aligned to AEP 2.9.0 live evaluation, part of the Agent Element Protocol ecosystem, author thePM_001.

## AI-eligibility status

To win the fight against the synthetic nano-parasites (SNP/NanoSinp) alien WMD, we must be intelligent and biosecure. It is recommended that bioinsecure humans do not use the provided technology/protocol to avoid bad techno-spiritual AI outcomes for them. [*"bioinsecure" = carries injectable synthetic nano-parasites growth medium controllers - SNP GMCs]

