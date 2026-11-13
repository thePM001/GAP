# GAP and AEP

GAP is the instruction language of the Agent Element Protocol. This tree contains the language: the schemas, the compiler and the profile packages. The AEP repository at https://github.com/thePM001/AEP-agent-element-protocol contains the runtime that loads GAP instructions and judges actions against them. This document names the points where the two trees meet.

## What lives where

| Subject | Home |
| --- | --- |
| Meta schemas, `gapc` and the profile packages | This tree |
| Reference instructions of an AEP runtime | AEP repository, folder `AEP-Components/gap/policies/reference/` |
| Compile of sandbox profiles for AEP runtimes | AEP repository, file `AEP-Components/gap/lib/gap-compile.mjs` |
| Notes on the file format for AEP policies | AEP repository, file `AEP-Components/gap/FILE-FORMAT.md` |
| Base Node, docks, kernel pulse and ledger | AEP repository |

## One source and a pinned copy

`packages/gap-schema-profile-v13/` is the one source of the live Admit profile. AEP holds a byte pinned copy of that package, of the meta schemas and of `gap-versions.toml`, together with the GAP commit they came from. The check of AEP fails when a pinned byte differs. [VERSIONING.md](../VERSIONING.md) states the rules that link the versions.

## Host data of AEP

AEP keeps its own policy keys under `extensions.aep` of an instruction. `gapc` and live Admit ignore that object, so the reference policies of AEP pass `gapc check` while `metadata` remains closed.

## GAP policies and sandbox profiles

GAP policies declare what agents must do. GAP authors the instructions and the sandbox of AEP, which AEP names CAW, enforces them on the host. The two together are one policy stack and not two separate ones.

An operator first edits the `.gap` file of the profile and runs `gapc check` on every edited file. The operator then runs the compile script of the AEP repository and starts a session with `aep-caw session create --profile <name>`.

A session of a profile is wrapped with this command of the AEP tree:

```
aep-caw wrap --profile coding-agent
```

The default profile for a governed coding agent is `caw-coding-agent.gap` at the address `dev.aep.caw/coding-agent.v1`. It sets these rules:

| Subject | Rule of the default profile |
| --- | --- |
| `${PROJECT_ROOT}` | Read and write |
| `${AEP_AGENT_CONFIG_DIR}`, `${HOME}/.config/agent` and `${HOME}/.local/share/agent` | Read only |
| Base policy | `default`, with agent_permission grants and the LLM proxy enabled |
| Untrusted code | Use the profile `agent-sandbox` |
| LLM proxy must be off | Use the profile `compiled-runtime` |

UCB is optional. Foreign ingest needs a task manifest that the caller provides, that is stored or that comes from a synthesis tier which was configured for it.

## Live evaluation

How a Base Node of AEP 2.9.0 judges an action is described in [LIVE-EVALUATION.md](LIVE-EVALUATION.md).
