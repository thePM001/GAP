# Classic GAP versus AEP 2.8.5 evaluation

Status: fit analysis
Date: 2026-09-05
Question: does classic GAP need adjustments to fit AEP 2.8.5 evaluation handling ?
Answer: yes. The instruction model still fits. The live evaluation story and the who-may model do not.

Sources:
- Classic GAP: Gitea thePM001/GAP (README.md, GAP meta schema v1.json, GAP meta schema v1.2.json)
- AEP 2.8.5: NLA-AEP-v2.8-open-source README.md, CHANGELOG.md, AEP-Policy-System/reference, AEP-Components/gap, crates aep-one-live-evaluation and aep-one-evaluation-story

## Verdict

Classic GAP can remain the written-policy language for AEP 2.8.5. Builders already load `.gap` files as live Admit walls.

Classic GAP as published on GitHub/Gitea still teaches a sequential 15-step live chain, trust-ring ranks and YAML-only documents. AEP 2.8.5 live path is: seal a capsule, freeze the clock, wait 1000 ms, run every check together (collect-all Admit) then Apply. The fifteen named rows are a derived ledger, not a second live pass. Skip is not used. Trust rings are removed. Who-may is `agent_may`.

Until classic GAP docs and schema catch that, an agent that implements GAP from the OSS README will build the old evaluator.

## What already fits

| Classic GAP idea | AEP 2.8.5 use |
|------------------|---------------|
| Instruction as atomic unit | Written policy nodes on the hyperlattice wrap |
| Pattern guards, action resolves | GAP walls bind to a wrap or an action_path prefix |
| Covenants `[hard]` / `[soft]` | Covenant evaluation remains a named derived-ledger row |
| Scanners | Content scanners remain derived-ledger rows 10-14 |
| Subprotocols | AEP-Subprotocols domain validators |
| `.gap` as GAP source | AEP-Policy-System live Admit GAP files |
| Constrained decoding | Optional authoring aid. Not the kernel Admit |
| CAW profiles authored in GAP | AEP-Components/gap compiles profiles for aep-caw |

AEP 2.8.5 still vendors classic GAP under `AEP-Components/gap/` and says GAP authors, CAW enforces.

## Required adjustments

### 1. Live evaluation is collect-all Admit then Apply

Classic GAP README:
- "The 15-step deterministic evaluation chain validates every instruction action."
- Adoption rung 6: "Trust rings, proof bundles, 15-step lattice"
- Budget "enforced at lattice Step 10"
- Aspect "used by lattice Step 5"
- Subprotocols "loaded at lattice Step 3.5"

AEP 2.8.5:
- Sole live evaluation is collect-all Admit then Apply (`aep-one-live-evaluation`, `aep-one-evaluation-story`)
- Fifteen named rows are a derived ledger under `AEP-Components/evaluation-chain`
- Order of rows does not change yes or no
- Skip is not used
- If two walls fail, both are listed
- `run_meet` is parked off the product SDK and is not live evaluation
- TypeScript `processEvent` is not product Admit
- GraphEngine execute requires `admitGate` (missing gate is deny)

Adjustment: rewrite classic GAP Layer 3. Keep the fifteen names as a forensic ledger. Do not describe them as a live sequential combinator.

### 2. Kernel pulse and freeze-at-seal

AEP 2.8.5 kernel:
- Open a sealed lattice-channel capsule
- Freeze the clock at seal
- Wait 1000 ms (compiled Base Node constant `PULSE_MS`, not a dynAEP YAML key)
- Then every check together, then carry out the allowed action
- Drift 50 ms against freeze
- Pulse age 5000 ms
- Unbound scene, dock, timestamp or sequence closes Admit
- Putting a capsule on the dock is not Admit
- After the wait the client collects by capsule hash
- A retry must seal a new capsule

Classic GAP has no pulse, no freeze-at-seal and no sealed-capsule transport story. Layer 3 is "after validation, before activation".

Adjustment: classic GAP Layer 3 must name Base Node kernel pulse plus collect-all Admit. Authoring a `.gap` file is not the check.

### 3. Who-may replaces trust rings

Classic GAP schema and README:
- `trust_ring`: sandbox (Ring 3), user (Ring 2), system (Ring 1), enterprise (Ring 0)
- Trust scoring and execution rings in the dottxt comparison
- AEP-Components/gap README still says CAW profiles carry a trust ring

AEP 2.8.5 CHANGELOG:
- Trust Rings four-stage rank is not a product member
- `ring_capability` evaluation-chain step replaced by `gap_capability`
- EnvelopeAction `rank` field removed
- Client `trust_tier` is not a floor
- Who-may is GAP capability dimensions: Agent A may X, Agent B may Y
- Empty grants fail closed for agent actions
- Attractors and cached forecast scores do not skip Admit

Adjustment: retire `trust_ring` as a live Admit floor. Add `agent_may` (and wrap/prefix binding). Keep an optional ring label only if it is documentary, never a skip or a rank compare.

### 4. Policy bind: wrap or prefix, writing and security always on

AEP 2.8.5:
- Policy-system GAP walls bind to `LatticeNode.wrap` or an action_path prefix
- Writing and security stems still evaluate on every action_path
- A finance wrap GAP item does not close an inventory wrap ping
- YAML GAP parse reads `guard` and `wrap`
- Deny report names closed wall ids, reasons and a prescribed repair for missing fields and writing
- Grant lists stay off repair

Classic GAP meta-schema has no `wrap` field and no `guard` object. Pattern is a YAML string or a PatternObject with input/output types.

Live AEP 2.8.5 reference policies (`AEP-Policy-System/reference/*.gap`) are JSON objects with `pattern.guard` (often `"true"`). Classic README requires Pure YAML 1.2 and "No mixed formats".

Adjustment:
- Schema: allow `wrap`, `action_path` prefix and `pattern.guard`
- Docs: live kernel policies may be JSON-encoded GAP instructions. YAML remains valid GAP source. The kernel cares about the instruction object, not the skin.
- Docs: writing and security are always-on stems. Other GAP walls bind to a wrap or prefix.

### 5. Closed-wall Deny, not silent skip

Classic GAP execution block uses retry / timeout / `on_exhaustion: fail`. That is instruction-runtime retry. It is not kernel Admit.

AEP 2.8.5 Deny:
- Structured closed-wall set
- Prescribed mechanical repair for unbound scene, dock, time, sequence and writing
- Digest replay is keyed at enqueue so retry reseals

Adjustment: classic GAP must not teach "skip a lattice step". Same input plus same sealed bytes is replay, not a new Admit.

### 6. Proof and signature claims

Classic GAP names Ed25519 or ML-DSA-65 on proof bundles.

AEP 2.8.5 EPSCOM trust bundle mode is `sha256-structure`. ML-DSA is not claimed on that bundle. The signatures loader denies an ML-DSA claim on `sha256-structure`.

Adjustment: do not say every AEP 2.8.5 attach ships ML-DSA. Keep post-quantum signatures as an optional proof algorithm, not as the default live bundle.

### 7. Vendored copy inside AEP 2.8.5

`AEP-Components/gap/README.md` is the classic GAP README plus a short AEP 2.8 integration header. It still teaches trust rings and a 15-step live lattice.

Adjustment: that vendored README must follow the same rewrite as public classic GAP, or AEP 2.8.5 will keep shipping the old evaluation story next to the new kernel.

## Recommended (docs and snapshot hygiene)

1. Restore `GAP v1 spec sheet.md` and `BIOSECURITY.md` in the OSS repo, or stop listing them in the file table. They are named in classic README and absent from the three-file snapshot.
2. Repair truncated classic README lines (principle 5, scanners row, `structural` CLI name).
3. Keep `.gap` as GAP source. Do not rename classic files to `.gaplune`.
4. Point Layer 1 at constrained decoding as an authoring gate, not as Admit.
5. Point builders at AEP 2.8.5 Kernel pulse and How a message is judged, not at dynAEP YAML for the wait.

## Suggested schema additions (classic v1.3 or a 2.8.5 profile)

Keep v1 / v1.2 for old documents. Add a profile or v1.3 that allows:

- `metadata.agent_may` (list of allowed actions for this agent id)
- `metadata.wrap` and/or `metadata.action_path_prefix`
- `pattern.guard` string or structured guard
- `enabled` already exists; keep it as load-time, not as Admit skip
- Deprecate `metadata.trust_ring` as a live floor (warn, then deny on live Admit if used as rank)

Do not delete covenants or scanners from classic OSS v1.2. AEP 2.8.5 still uses those ideas as derived-ledger rows and as always-on writing/security stems.

## Fit score

| Layer | Fits AEP 2.8.5 now ? | Work |
|-------|----------------------|------|
| Instruction object (address, pattern, action, composition) | yes | keep |
| `.gap` extension meaning | yes | keep |
| Constrained decoding as Layer 1 | yes as authoring | say it is not Admit |
| Sequential 15-step live chain | no | rewrite to derived ledger |
| Trust-ring ranks | no | replace with `agent_may` |
| YAML-only file skin | partial | allow JSON-encoded GAP objects for kernel policies |
| wrap / prefix bind | no | add fields and docs |
| Kernel pulse / freeze-at-seal | no | document as Layer 3 host |
| Attractors as skip | no (classic barely says this; do not add it) | keep attractors forensic |

Classic GAP does require adjustments to fit AEP 2.8.5. They are documentation and schema-profile work plus a vendored-README fix inside AEP-Components/gap. They are not a new language.
