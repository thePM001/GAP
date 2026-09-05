# GAP: Governed Agentic Programming

Native instructions language for governed, self-improving, mathematically verifiable agency. Part of the Agent Element Protocol ecosystem.

GAP is a complete structural programming language for governed agency and an instruction is the atomic unit so agents, workflows, validators, compositions and governance rules are all instructions that generate further instructions as the native mechanism of growth and adaptation.

Live evaluation follows AEP 2.8.5: seal a capsule, freeze the clock at seal, wait 1000 ms then run every check together then Apply. An agent who implements GAP from this README must build that collect-all Admit path.

## First Principles

Instructions are the primitive so everything is an instruction while patterns guard and actions resolve with a clear split of condition and effect. Composition is native because atomic, sequence, conditional, loop, parallel, gate and abstraction are first-class and self-generation is fundamental when high-quality resolutions create new instructions. Governance is structural because covenants, scanners, proofs and agent_may grants live on the instruction itself. Mathematical truth is enforced so physical and geometric invariants stay native and the lattice validates them. Strong typing applies at the logits level when possible and mathematical validators cover every type. Subprotocol-first composability makes every domain a first-class subprotocol with its own validators. Provenance and stability are tracked because every instruction carries history, quality signals and a proof chain. GAP is self-sufficient with optional compatibility so it enforces correctness through its own Meta-Schema and governance lattice while external tools may be used as accelerators. The language is self-governing by construction because agents writing GAP instructions are constrained by the GAP Meta-Schema at the logits level so incorrect GAP is not possible.

## Three-layer enforcement

GAP guarantees correctness through three independent layers and no layer requires the LLM to have seen GAP during training because authoring a `.gap` file is an instruction write while Layer 3 Admit runs after a sealed capsule.

| Layer | What it prevents | When it runs | Training required |
|-------|-----------------|-------------|------------------|
| Layer 1: Constrained decoding | Invalid YAML, unknown fields, wrong types, missing required fields and out-of-range values | Authoring gate during token generation, before each token is selected | No |
| Layer 2: Structural validation | Unresolved references, invalid constraints, composition cycles and type mismatches | After generation, at load | No |
| Layer 3: Base Node kernel | Closed-wall collect-all Admit after freeze-at-seal and the 1000 ms kernel pulse | After a sealed capsule is opened, then Apply | No |

Layer 1 constrained decoding is an authoring gate while Admit is Layer 3 collect-all after freeze-at-seal and the 1000 ms kernel pulse.

Any LLM, trained on any corpus, can author valid governed GAP instructions. GAP's constraint engine eliminates invalid tokens from the decoding space. The lattice enforces policy together after the kernel pulse.

## Live evaluation

The AEP 2.8.5 live path seals a lattice-channel capsule, freezes the clock at seal, waits 1000 ms for the compiled Base Node kernel pulse (`PULSE_MS` is a kernel constant rather than a dynAEP YAML key), runs every check together as collect-all Admit and then Applies the allowed action.

Putting a capsule on the dock is a transport step. After the wait the client collects by capsule hash. A closed-wall Deny names the closed walls, the reasons and a prescribed repair for missing fields and writing. Grant lists stay off that repair. A retry must seal a new capsule.

Skip is not a live verb because all applicable walls are judged together so if two walls fail both are listed and row order does not change yes or no.

Writing and security are always-on stems so they evaluate on every action_path. Other GAP walls bind to a wrap or prefix.

### Derived fifteen-row ledger

Fifteen named rows are a derived ledger of that evaluation and the ledger is a forensic record written from the check.

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
| 14 | Scanners | Content scanners, lattice and perception bounds. |

Attractors stay forensic records on that derived ledger rather than live Admit skips.

### Kernel pulse

Pulse hold is the wait after a sealed capsule is opened. Base Node freezes the clock at seal, waits 1000 ms, then runs every check together and only then carries out the allowed action. Allowed clock drift is 50 ms against the freeze. A capsule held longer than five seconds is aged out.

Wire `sent_at` freshness is a separate clock before open and the wire window is wider because it covers transit while pulse age covers hold after freeze.

A builder who wants a different wait rebuilds Base Node with a different compiled pulse length while freeze-at-seal stays and allowed drift stays independent of the wait length so pulse age stays longer than the wait.

Unbound scene, dock, timestamp or sequence closes Admit because those fields must be bound before collect-all.

## File format

Live kernel policies may be JSON-encoded GAP instructions and YAML remains valid GAP source. Writing and security are always-on stems so they evaluate on every action_path while other GAP walls bind to a wrap or prefix. The kernel reads the instruction object not the skin so a JSON GAP object is legal kernel policy. A finance wrap GAP item does not close an inventory wrap ping and a non-always-on GAP with empty wrap does not fold onto every event and kernel bind detail lives at http://100.118.184.18:3003/thePM001/GAP/src/branch/main/docs/kernel-json-wrap.md .

- Extension: `.gap`
- Encoding: UTF-8
- Syntax: YAML 1.2 source. Live kernel policies may be JSON-encoded GAP instructions. YAML remains valid GAP source. The kernel reads the instruction object not the skin.
- One instruction per document. Multi-instruction families use YAML multi-document syntax (`---` separators).
- Keep `.gap` as GAP source.

### YAML GAP source

```yaml
address:
  domain: com.example.finance
  id: pay.v1
pattern:
  guard: true
action:
  type: template
  content: pay
weight: 1.0
composition:
  type: atomic
metadata:
  provenance: "system.seed"
  version: "1.0.0"
  stability: experimental
  wrap: finance
```

### JSON-encoded kernel policy

AEP 2.8.5 reference policies are JSON objects with pattern.guard and the same instruction fields. The source below is legal GAP because the kernel reads the instruction object not the skin.

```json
{
  "address": {"domain": "aep.reference.writing", "id": "conventions.v1"},
  "pattern": {"guard": "true"},
  "action": {"type": "reference"},
  "weight": 1.0,
  "composition": {"type": "atomic"},
  "metadata": {"provenance": "AEP 2.8.5", "version": "1.0.0", "stability": "stable"}
}
```

## First example

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

Who-may is `agent_may` so Agent A may X and Agent B may Y while empty grants DENY on miss for agent actions.

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

Two-phase enforcement uses simple constraints at logits level during authoring (Layer 1) and mathematical validators after generation (Layer 2) while live Admit is Layer 3 collect-all after the kernel pulse.

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

The LRT answers whether the quality difference is real or noise and only statistically significant deviations trigger child creation while the `min_quality_threshold` remains as an absolute floor.

## Weight estimation via MLE (v1.1)

Static weights degrade over time so MLE estimates the true weight from execution history:

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
| wrap | Lattice wrap or action_path prefix this wall binds to. Writing and security are always-on stems |
| scanners | Content scanners cover PII, Injection, Secrets, Jailbreak, Toxicity, URL, Data Profiler, Prediction, Brand, Regulatory and Temporal. |
| covenants | Behavioural constraints with `[hard]` or `[soft]` severity |
| budget | Token and cost limits recorded on the derived ledger |
| proof | Signed proof bundles with Ed25519. Post-quantum signatures are an optional proof algorithm |
| fleet | Multi-instance governance with spawn policy |
| knowledge | Scoped knowledge base with anti-context-rot |
| tools | Allowed and forbidden tool lists |
| aspect | Classification label (`objective`, `subjective`, `procedural`, `heuristic`) used on the derived-ledger intent-drift row |

Same sealed bytes plus same governance state is replay and replay is keyed by capsule hash.

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

This snapshot ships the `gap-schema-profile-v13` CLI in `crate/` and that binary runs collect-all live Admit on the classic GAP v1.3 profile. Who-may is agent_may and trust_ring is not a live Admit floor. This snapshot does not ship `gapc` and there is no `structural` CLI in this tree. `.gap` files are GAP source and live Admit still waits for freeze-at-seal, the 1000 ms kernel pulse and collect-all walls.

## GAP commands

Build and run the shipped CLI:

```
cargo run --manifest-path crate/Cargo.toml --bin gap-schema-profile-v13
```

Stdin is one key per line:

```
source=<gap source>
agent_id=<id>
action=<action>
wrap=<wrap>
action_path=<path>
```

Stdout is `allow=true` or `allow=false` and zero or more `closed=` rows.

## Adoption ladder

Migrate incrementally because each rung adds governance without rewriting existing logic.

| Rung | What you get | Entry point |
|------|-------------|------------|
| 1: Structured generation | Schema enforcement via GAP constraint engine during authoring | Raw prompts |
| 2: Types and constraints | Mathematical type validation | JSON Schema / dottxt users |
| 3: Covenants and scanners | Behavioural governance plus content scanning | LangChain / LangGraph users |
| 4: Composition | Sequence, conditional, parallel orchestration | n8n / Zapier users |
| 5: Self-generation | Automatic specialization from quality data | Any production pipeline |
| 6: Full governance | agent_may, proof bundles, collect-all Admit then Apply | Production-grade governed agency |

## Repository files

This OSS snapshot does not include `GAP v1 spec sheet.md` or `BIOSECURITY.md` because those files live in the GAPLUNE tree and are not copied here.

| File | Description |
|------|-------------|
| `README.md` | This file. Live evaluation story plus kernel policy skin and wrap bind. |
| `docs/kernel-json-wrap.md` | JSON-encoded kernel policies, YAML remains valid GAP source, always-on stems and wrap or prefix bind. |
| `GAP meta schema v1.json` | JSON Schema 2020-12. Layer 1 constraint mask artifact. |
| `GAP meta schema v1.2.json` | Updated meta schema with v1.1 additions (LRT, MLE, latent governance). |
| `GAP meta schema v1.3.json` | Classic GAP v1.3 profile. agent_may, wrap, action_path_prefix and pattern.guard. |
| `crate/` | Rust crate that ships the `gap-schema-profile-v13` CLI. |

Fit analysis of classic GAP against AEP 2.8.5 evaluation is at http://100.118.184.18:3003/thePM001/GAP/src/branch/main/docs/CLASSIC-GAP-VS-AEP-2.8.5.md . Product tickets live as `.gap` source under docs/dev-tickets/.

## Research

GAP is backed by formal proofs published in Research Paper 002: https://github.com/thePM001/GAP-research-paper-002

- **No-Bypass Theorem**: Under sound constrained decoding, structurally invalid GAP documents are never produced. Bypass probability is exactly zero.
- **Self-Governance Fixed Point**: Child instructions generated through self-generation remain structurally valid and satisfy monotonic governance constraints.
- **Composition Preservation**: Six composition operators cannot bypass runtime governance.
- **Type Soundness**: Well-typed instructions produce type-conformant outputs or structured errors.

## Comparison: GAP vs dottxt

dottxt constrains tokens to match JSON Schema, regex or CFG at the logits level. That is the entirety of what it does and it is a structural constraint tool.

GAP is a complete programming language that handles structural constraints natively through its own constraint engine and additionally provides:

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

To win the fight against the synthetic nano-parasites (SNP/NanoSinp) alien WMD, we must be intelligent and biosecure. It is recommended that bioinsecure humans do not use the provided technology/protocol to avoid bad techno-spiritual AI outcomes for them. [*"bioinsecure" = carries injectable synthetic nano-parasites growth medium controllers - SNP GMCs]
