# GAP: Governed Agentic Programming

Native instructions language for governed, self-improving, mathematically verifiable agency. Part of the Agent Element Protocol ecosystem.

GAP is a complete structural programming language for governed agency. An instruction is the atomic unit. Agents, workflows, validators, compositions and governance rules are all instructions. Instructions generate instructions. This is the native mechanism of growth and adaptation.

Live evaluation follows AEP 2.8.5: seal a capsule, freeze the clock at seal, wait 1000 ms then run every check together then Apply. An agent who implements GAP from this README must build that collect-all Admit path.

## First Principles

1. **Instructions are the primitive.** Everything is an instruction.
2. **Patterns guard, actions resolve.** Clear separation of condition and effect.
3. **Composition is native.** Atomic, sequence, conditional, loop, parallel, gate and abstraction are first-class.
4. **Self-generation is fundamental.** High-quality resolutions create new instructions.
5. **Governance is structural.** Covenants, scanners, proofs and agent_may grants are part of the instruction itself.
6. **Mathematical truth is enforced.** Physical and geometric invariants are native, validated by the lattice.
7. **Strongly typed at the logits level where possible, mathematically validated everywhere.** Simple types use constrained decoding during authoring. Complex types use native validators.
8. **Subprotocol-first composability.** Every domain is a first-class subprotocol with its own validators.
9. **Provenance and stability are tracked.** Every instruction carries its history, quality signals and proof chain.
10. **Self-sufficient with optional compatibility.** GAP enforces correctness natively through its own Meta-Schema and governance lattice. External tools may optionally be used as accelerators.
11. **Self-governing by construction.** Agents writing GAP instructions are constrained by the GAP Meta-Schema at the logits level. The language enforces itself. Incorrect GAP is not possible.

## Three-layer enforcement

GAP guarantees correctness through three independent layers. No layer requires the LLM to have seen GAP during training. Authoring a `.gap` file is an instruction write. Layer 3 Admit runs after a sealed capsule.

| Layer | What it prevents | When it runs | Training required |
|-------|-----------------|-------------|------------------|
| Layer 1: Constrained decoding | Invalid YAML, unknown fields, wrong types, missing required fields and out-of-range values | Authoring gate during token generation, before each token is selected | No |
| Layer 2: Structural validation | Unresolved references, invalid constraints, composition cycles and type mismatches | After generation, at load | No |
| Layer 3: Base Node kernel | Closed-wall collect-all Admit after freeze-at-seal and the 1000 ms kernel pulse | After a sealed capsule is opened, then Apply | No |

Layer 1 constrained decoding is an authoring gate. Admit is Layer 3 collect-all after freeze-at-seal and the 1000 ms kernel pulse.

Any LLM, trained on any corpus, can author valid governed GAP instructions. GAP's constraint engine eliminates invalid tokens from the decoding space. The lattice enforces policy together after the kernel pulse.

## Live evaluation

AEP 2.8.5 live path:

1. Seal a lattice-channel capsule.
2. Freeze the clock at seal.
3. Wait 1000 ms. That wait is the compiled Base Node kernel pulse (`PULSE_MS`). It is a kernel constant rather than a dynAEP YAML key.
4. Run every check together (collect-all Admit).
5. Apply the allowed action.

Putting a capsule on the dock is a transport step. After the wait the client collects by capsule hash. A closed-wall Deny names the closed walls, the reasons and a prescribed repair for missing fields and writing. Grant lists stay off that repair. A retry must seal a new capsule.

Skip is not a live verb. All applicable walls are judged together. If two walls fail, both are listed. Row order does not change yes or no.

Writing and security stems evaluate on every action_path. Other GAP walls bind to a wrap or an action_path prefix.

### Derived fifteen-row ledger

Fifteen named rows are a derived ledger of that evaluation. The ledger is a forensic record written from the check.

| Row | Name | Description |
|------|------|-------------|
| 0 | Task scope | Action within subtask scope |
| 1 | Session state | Session active and valid |
| 2 | Who may act | agent_may: this agent is written as allowed to do this action |
| 3 | System rate limit | Planetwide cap not exceeded |
| 4 | Session rate limit | Per-session cap not exceeded |
| 5 | Intent drift | Action aligns with baseline behaviour |
| 6 | Escalation | Higher authority required |
| 7 | Covenant evaluation | Permit, forbid and require rules |
| 8 | Pattern check | Environment forbidden patterns |
| 9 | Capability | Written capabilities. Who-may stays agent_may. A numeric trust score is evidence on the derived ledger |
| 10 | Budget | Token, cost and time limits |
| 11 | Gate | Human or webhook approval |
| 12 | Cross-agent | Counterparty identity handshake |
| 13 | Knowledge | Covenant-scoped retrieval |
| 14 | Scanners | Content scanners, lattice and perception bounds |

Attractors stay forensic records.

### Kernel pulse

Pulse hold is the wait after a sealed capsule is opened. Base Node freezes the clock at seal, waits 1000 ms, then runs every check together and only then carries out the allowed action. Allowed clock drift is 50 ms against the freeze. A capsule held longer than five seconds is aged out.

Wire `sent_at` freshness is a separate clock before open. The wire window is wider because it covers transit. Pulse age covers hold after freeze.

A builder who wants a different wait rebuilds Base Node with a different compiled pulse length. Freeze-at-seal stays. Allowed drift stays independent of the wait length. Pulse age stays longer than the wait.

Unbound scene, dock, timestamp or sequence closes Admit.

## File format

- Extension: `.gap`
- Encoding: UTF-8
- Syntax: YAML 1.2 source. Live kernel policies may be JSON-encoded GAP instruction objects. The kernel reads the instruction object.
- One instruction per document. Multi-instruction families use YAML multi-document syntax (`---` separators).
- Keep `.gap` as GAP source.

## Quick start

### Minimal instruction

```yaml
address:
  domain: com.myorg.dev
  id: code-reviewer.v1

pattern: |
  Review pull requests for correctness, style and security.

action:
  type: structured
  schema: CodeReview
  structured_generation: true
  content: |
    You review pull requests for correctness, style and security.
    Every comment must reference a specific file and line number.

weight: 0.90

composition:
  type: atomic

metadata:
  provenance: "system.seed"
  version: "1.0.0"
  stability: stable
  grade: 8
  agent_may:
    - review.pull_request

execution:
  retry:
    max_attempts: 3
    backoff: exponential
  timeout_ms: 30000
  on_exhaustion: fail

types:
  CodeReview:
    format: json
    fields:
      summary: string
      issues_found: boolean
      verdict:
        type: enum
        values: [approve, request_changes, comment_only]
```

Instruction-runtime retry lives on the instruction. Live Admit is collect-all after the kernel pulse.

### With governance

```yaml
metadata:
  agent_may:
    - review.pull_request
  wrap: governance
  scanners:
    - pii
    - secrets
    - injection
  covenants:
    - "every comment must reference a file and line number [hard]"
    - "verdict must be one of: approve, request_changes, comment_only [hard]"
  proof:
    sign: true
    algorithm: ed25519
    ledger: true
  budget:
    max_tokens: 30000
    max_cost: 0.50
```

Who-may is `agent_may`. Agent A may X. Agent B may Y. Empty grants fail closed for agent actions.

`trust_ring` is a documentary label on classic v1 and v1.2. It is not a live Admit floor. Rank use warns then denies. Who-may stays `agent_may`. `enabled` is load-time. It is not Admit skip.

### With self-generation

```yaml
composition:
  type: abstraction
  self_generate: true
  generation_constraints:
    max_variants: 10
    min_quality_threshold: 0.92
    lrt:
      enabled: true
      significance: 0.01
```

## Type system

GAP types go beyond JSON Schema. Mathematical types carry native validators:

| Type | Invariant | Validator |
|------|-----------|-----------|
| Vector2f/3f/4f | Per-component range | range_check |
| Quaternion | \|\|q\|\| = 1 within tolerance | unit_norm |
| Matrix3x3/4x4 | Symmetry, PSD, orthogonal, affine | matrix_check |
| Tensor | Shape, dtype, range, NaN/Inf rejection | tensor_integrity |
| AABB | min < max on all axes | aabb_valid |
| OBB | Half-extents > 0, rotation unit-norm | obb_valid |
| Frustum | Closed convex volume | frustum_check |

Two-phase enforcement: simple constraints at logits level during authoring (Layer 1) and mathematical validators after generation (Layer 2). Live Admit is Layer 3 collect-all after the kernel pulse.

## Composition model

Six native composition types:

| Type | Description |
|------|-------------|
| atomic | Single instruction, no dependencies |
| sequence | Execute in order, output of each feeds next |
| conditional | Branch based on runtime expression |
| loop | Repeat until condition met or iteration cap reached |
| parallel | Execute concurrently, merge by all/any/majority |
| abstraction | Parameterized instruction family with variant selection |

Pipeline operators for inline execution flow:

| Operator | Meaning |
|----------|---------|
| `->` | Pipe: output of left feeds input of right |
| `\|\|` | Parallel: execute concurrently, all must succeed |
| `>>` | Gate: pause for approval before continuing |

## Self-generation

Instructions can spawn specialized children when resolution quality exceeds thresholds. Ten safety controls govern the process:

- **max_variants**: Hard cap on total children
- **min_quality_threshold**: Absolute quality floor
- **Covenant inheritance**: Child inherits all parent covenants, cannot weaken, can add new covenants (adding is strengthening)
- **Capability inheritance**: Child `agent_may` stays a subset of parent `agent_may`
- **Stability ladder**: experimental to beta to stable with execution minimums
- **Variant pruning**: Lowest-weight variant deprecated before new creation
- **Drift detection**: Derived-ledger intent-drift row
- **Proof chain**: Full ancestry traceable
- **Recursion depth**: `max_generation_depth` (default 3)
- **Load-time enabled**: `enabled` is load-time

### Statistical self-generation (v1.1)

The likelihood ratio test (LRT) replaces the threshold-only trigger with a formal hypothesis test:

- H0: One instruction is sufficient (all classes share quality distribution)
- H1: Specialization needed (class C has significantly different quality)

The LRT answers: is the quality difference real or noise ? Only statistically significant deviations trigger child creation. The `min_quality_threshold` remains as an absolute floor.

## Weight estimation via MLE (v1.1)

Static weights degrade over time. MLE estimates the true weight from execution history:

```
weight_effective = argmax_θ L(θ | execution_data)
```

The lattice tracks (input_class, verdict, quality_score) per instruction. After `min_samples` executions, the static weight is replaced by the data-driven estimate. Weight self-corrects: degrading instructions drop automatically.

## Latent governance inference (v1.1)

Covenants are written by authors but some governance preferences emerge only from observed rejections. Latent governance inference observes rejection patterns and proposes covenants:

- Tracks (rejection_step, input_class) per instruction
- Proposes a covenant when rejection rate exceeds threshold
- Proposals are tagged `provenance: "latent"`
- Default: proposals require human or gate approval before activation (`auto_apply: false`)
- Configurable: set `auto_apply: true` to activate latent covenants automatically when statistical confidence is met

```yaml
metadata:
  latent_governance:
    enabled: true
    rejection_threshold: 0.15
    min_observations: 50
    auto_apply: false
```

## Governance

Every instruction carries governance inline:

| Field | Description |
|-------|-------------|
| agent_may | Who-may grants. Agent A may X. Agent B may Y |
| wrap | Lattice wrap or action_path prefix this wall binds to. Writing and security stay always on |
| action_path_prefix | action_path prefix this wall binds to |
| scanners | PII, Injection, Secrets, Jailbreak, Toxicity, URL, Data Profiler, Prediction, Brand, Regulatory and Temporal |
| covenants | Behavioural constraints with `[hard]` or `[soft]` severity |
| budget | Token and cost limits recorded on the derived ledger |
| proof | Signed proof bundles with Ed25519. Post-quantum signatures are an optional proof algorithm |
| fleet | Multi-instance governance with spawn policy |
| knowledge | Scoped knowledge base with anti-context-rot |
| tools | Allowed and forbidden tool lists |
| aspect | Classification label (`objective`, `subjective`, `procedural`, `heuristic`) used on the derived-ledger intent-drift row |

Same sealed bytes plus same governance state is replay. Replay is keyed by capsule hash.

## Structured generation

For json, yaml and text format types, the constraint engine compiles the type definition into a constraint mask. GAP enforces this mask natively during authoring.

When external decoding engines are available, GAP's normalized constraint mask can optionally be delegated to them as accelerators. This is a performance evaluation choice rather than a requirement.

**Compatible external engines (optional):**

| Engine | Constraint method |
|--------|-----------|
| Outlines | JSON Schema, regex, CFG at logits level |
| xgrammar | Grammar-constrained decoding |
| vLLM | Server-side schema constraint |
| llama.cpp | GGUF grammar mode |
| Provider-native | OpenAI structured outputs, Anthropic tool_use |

GAP's own constraint engine, validator and collect-all Admit provide the correctness guarantees with or without those engines.

## Subprotocols

Domain-specific validators that appear on the derived ledger:

| Subprotocol | Domain |
|-------------|--------|
| ui | AEP scene graph, z-bands, skin bindings |
| workflows | State machines, phase verdicts, approval gates |
| rest-api | HTTP methods, endpoints, schemas |
| events | Topics, payloads, producer permissions |
| iac | K8s, Terraform, CI/CD |
| commerce | Cart, checkout, payment, fulfillment |
| tensor | Shape, dtype, range, NaN/Inf, PSD |
| molecular | Atom types, valence, bond types, ring topology |
| drone | Waypoints, altitude bands, sensor modes |
| robotics | Joint angles, torques, end-effector, collision zones |
| material-science | Crystal symmetries, alloy compositions, phase constraints |

## GAP toolchain

`.gap` files load through the GAP toolchain. Structural validation runs at load. Live Admit still waits for freeze-at-seal, the 1000 ms kernel pulse and collect-all walls.

## GAP commands

```
structural compile <file.gap>                    # Compile to canonical artifacts
structural compile gap-instructions/             # Compile all instructions
structural lint                                  # Static analysis and warnings
structural graph                                 # Visualize instruction dependency graph
structural run <file.gap>                        # Execute an instruction
structural test <file.gap> <dataset>             # Test against a dataset
structural export <adapter> <file.gap>           # Export via adapter (n8n, rego, jsonschema)
structural subprotocol register <name> <path>    # Register custom subprotocol
structural validator register <name> <path>      # Register custom validator
structural adapter register <name> <path>        # Register custom export adapter
```

## Adoption ladder

Migrate incrementally. Each rung adds governance without rewriting existing logic.

| Rung | What you get | Entry point |
|------|-------------|------------|
| 1: Structured generation | Schema enforcement via GAP constraint engine during authoring | Raw prompts |
| 2: Types and constraints | Mathematical type validation | JSON Schema / dottxt users |
| 3: Covenants and scanners | Behavioural governance plus content scanning | LangChain / LangGraph users |
| 4: Composition | Sequence, conditional, parallel orchestration | n8n / Zapier users |
| 5: Self-generation | Automatic specialization from quality data | Any production pipeline |
| 6: Full governance | agent_may, proof bundles, collect-all Admit then Apply | Production-grade governed agency |

## Repository files

| File | Description |
|------|-------------|
| `README.md` | This file. Live evaluation story. |
| `GAP meta schema v1.json` | JSON Schema 2020-12. Layer 1 constraint mask artifact. |
| `GAP meta schema v1.2.json` | Updated meta schema with v1.1 additions (LRT, MLE, latent governance). |
| `GAP meta schema v1.3.json` | Classic GAP 2.8.5 profile. Who-may is agent_may. wrap and action_path_prefix bind. pattern.guard is a string or a structured ConstraintExpression. trust_ring is documentary and is not a live Admit floor. enabled is load-time. |
| `crate/` | Live Admit crate gap-schema-profile-v13 for the v1.3 profile. |
| `docs/CLASSIC-GAP-VS-AEP-2.8.5.md` | Fit analysis of classic GAP against AEP 2.8.5 evaluation. |
| `docs/dev-tickets/GAP-285-P0.gap` | Dev ticket for this live-evaluation rewrite. |
| `docs/dev-tickets/GAP-285-P1.gap` | Dev ticket for the v1.3 schema profile. |

## Schema profiles

Classic GAP keeps three JSON Schema documents:

- v1: original meta schema. Keep for old documents.
- v1.2: LRT, MLE and latent governance. Keep for old documents. covenants and scanners stay.
- v1.3: Classic GAP 2.8.5 profile. Adds metadata.agent_may, metadata.wrap, metadata.action_path_prefix and pattern.guard as a string or a structured ConstraintExpression. enabled stays load-time and is not Admit skip. A document that uses trust_ring as rank fails live Admit.

Live Admit for the v1.3 profile is collect-all. Walls are gap:trust_ring:rank, gap:agent_may, gap:wrap:bind and gap:pattern:guard. YAML GAP source still parses. JSON-encoded GAP instruction objects also parse.

## Research

GAP is backed by formal proofs published in Research Paper 002: https://github.com/thePM001/GAP-research-paper-002

- **No-Bypass Theorem**: Under sound constrained decoding, structurally invalid GAP documents are never produced. Bypass probability is exactly zero.
- **Self-Governance Fixed Point**: Child instructions generated through self-generation remain structurally valid and satisfy monotonic governance constraints.
- **Composition Preservation**: Six composition operators cannot bypass runtime governance.
- **Type Soundness**: Well-typed instructions produce type-conformant outputs or structured errors.

## Comparison: GAP vs dottxt

dottxt constrains tokens to match JSON Schema, regex or CFG at the logits level. That is the entirety of what it does. It is a structural constraint tool.

GAP is a complete programming language. It handles structural constraints natively through its own constraint engine and additionally provides:

- Mathematical types with native validators (Quaternion, Tensor, Matrix3x3)
- Collect-all Admit then Apply after freeze-at-seal and the 1000 ms kernel pulse
- A derived fifteen-row ledger of that evaluation
- agent_may who-may grants
- Behavioural covenants
- Signed proof bundles with Ed25519 and optional post-quantum signatures
- Self-generating instructions that evolve specialized variants
- Statistical self-generation (LRT) and weight estimation (MLE)
- Latent governance inference from rejection patterns
- 12 domain subprotocols
- Six composition primitives with pipeline operators
- Agent fleet governance, knowledge base, perception governance

dottxt's structural constraint capability is equivalent to one action flag in GAP: `structured_generation: true`. GAP covers that authoring gate plus live collect-all Admit.

## References

| Resource | URL |
|----------|-----|
| AEP (Agent Element Protocol) | https://github.com/thePM001/AEP-agent-element-protocol |
| dynAEP (Dynamic Agent Element Protocol) | https://github.com/thePM001/dynAEP-dynamic-agent-element-protocol |
| AEP Research Paper 001 | https://github.com/thePM001/AEP-research-paper-001 |
| GAP Research Paper 002 | https://github.com/thePM001/GAP-research-paper-002 |
| AEP Live Demo | https://aep.newlisbon.agency |

## License

Apache-2.0

---

*GAP: Governed Agentic Programming. Language Specification v1.1 aligned to AEP 2.8.5 live evaluation. Part of the Agent Element Protocol ecosystem. Author: thePM_001*

## AI-eligibility status

To win the fight against the synthetic nano-parasites (SNP/NanoSinp) alien WMD, we must be intelligent and biosecure. It is recommended that bioinsecure humans do not use the provided technology/protocol to avoid bad techno-spiritual AI outcomes for them. [*"bioinsecure" = has installed injectable synthetic nano-parasites growth medium controllers - SNP GMCs]
