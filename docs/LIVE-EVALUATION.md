# Live evaluation of a GAP instruction

GAP is the language and AEP is the runtime. This document describes how a Base Node of AEP 2.9.0 judges an action against the GAP instructions it has loaded. The runtime itself lives in the AEP repository at https://github.com/thePM001/AEP-agent-element-protocol and its README describes the One Validation Envelope, the Admit Gate and the Base Node Pulse in full.

The GAP tree ships two packages that teach parts of this path: `packages/gap-schema-profile-v13/` contains the walls that read an instruction and `packages/gap-closed-wall-deny/` contains the walls that read the sealed capsule.

## The path of one action

Live evaluation follows AEP 2.9.0 in five steps. The proposal is first sealed in a capsule on a lattice channel and the Base Node freezes the clock at that seal. The node then waits 1000 ms, which is the kernel pulse, before it runs every check together as collect-all Admit. Only after that pass is the allowed action applied.

Putting a capsule on the dock is a transport step and not a verdict. After the wait the client collects the result by capsule hash. All applicable walls are judged together. If two walls fail both are listed, so the order of the rows does not change the answer. Skip is not a live verb.

## What a close reports

A closed-wall close names the closed walls, the reasons and a prescribed repair for missing fields and for writing. Grant lists are not part of that repair, so a close never tells an agent which permission it would need. A retry must seal a new capsule.

## Which walls an instruction binds to

Writing and security are always-on stems and they evaluate on every action path. Every other GAP wall binds to a wrap or to an action path prefix. A GAP item with the wrap finance does not close a ping on the wrap inventory. An instruction that is not always on and has an empty wrap does not fold onto every event.

The field `enabled` is read at load time. When it is false the instruction is still loaded and live Admit still evaluates its walls.

An unbound scene, dock, timestamp or sequence closes Admit, because those fields must be bound before the collect-all pass.

## Derived fifteen-row ledger

Fifteen named rows are a derived ledger of the evaluation. The ledger is a forensic record written from the check and it is not a sequence of steps.

| Row | Name | What it records |
| --- | --- | --- |
| 0 | Task scope | The action is within the scope of the subtask |
| 1 | Session state | The session is active and valid |
| 2 | Agent permission | The field agent_permission names this agent as allowed to do this action |
| 3 | System rate limit | The cap for the whole system is not exceeded |
| 4 | Session rate limit | The cap for the session is not exceeded |
| 5 | Intent drift | The action aligns with the baseline behaviour |
| 6 | Escalation | A higher authority is required |
| 7 | Covenant evaluation | The permit, forbid and require rules |
| 8 | Pattern check | The forbidden patterns of the environment |
| 9 | Capability | The written capabilities. A numeric trust score is evidence on the ledger and not a permission |
| 10 | Budget | The limits for tokens, cost and time |
| 11 | Gate | The approval of a human or a webhook |
| 12 | Cross-agent | The identity handshake with the counterparty |
| 13 | Knowledge | The retrieval within the scope of the covenant |
| 14 | Scanners | The content scanners with the lattice and perception bounds |

Attractors are forensic records on that ledger and they never let an action skip Admit. The field `aspect` of an instruction is the classification label that the intent drift row uses.

## Kernel pulse

Pulse hold is the wait after a sealed capsule is opened. The Base Node freezes the clock at seal, waits 1000 ms, runs every check together and only then carries out the allowed action.

- The allowed clock drift is 50 ms against the freeze.
- A capsule that is held longer than five seconds is aged out.
- The freshness of the wire field `sent_at` is a separate clock that runs before the capsule is opened. Its window is wider because it covers transit, while the pulse age covers the hold after the freeze.
- `PULSE_MS` is a constant of the kernel and not a key of the dynAEP configuration. A builder who wants a different wait rebuilds the Base Node with a different pulse length.
- A different pulse length changes neither the freeze at seal nor the allowed drift. The pulse age remains longer than the wait.

## Live trust bundle

The trust bundle mode of live AEP 2.9.0 is sha256-structure and collect-all Admit collects by capsule hash on the same path.

- The field `proof.algorithm` of an instruction may name Ed25519 or ML-DSA-65 as an optional proof algorithm.
- A signed proof is not the default attach of live AEP 2.9.0.
- The signatures loader denies an ML-DSA claim on a sha256-structure bundle.

The presence of `trust_ring` on a live GAP document is Deny and the closed wall is `gap:leftover_rank_field`. Agent permission is written in `agent_permission` and nowhere else.

## Replay

The same sealed bytes together with the same governance state are a replay. Replay is keyed by capsule hash, which is why a retry must seal a new capsule.

## What the GAP tree ships of this path

| Part of the path | Package | What it does |
| --- | --- | --- |
| Walls that read the instruction | `packages/gap-schema-profile-v13/` | Judges the leftover rank wall, the agent permission wall, the wrap bind wall and the guard wall in one collect-all pass |
| Walls that read the capsule | `packages/gap-closed-wall-deny/` | Judges the bind of scene, dock, timestamp and sequence and the writing wall, with a repair for each close |
| Digest and retry | `packages/gap-closed-wall-deny/` | Seals a capsule with a sha256-structure digest, detects a replay and reseals on retry |

The clock freeze, the 1000 ms wait, the docks, the ledger and Apply are not part of the GAP tree. They run in the Base Node of AEP.
