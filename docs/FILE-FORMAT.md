# GAP file format

A GAP file holds GAP source. This document states the rules that `gapc check` enforces on a file. The root README shows the fields of an instruction.

## Rules

| Subject | Rule |
| --- | --- |
| Extension | `.gap` |
| Encoding | UTF-8 |
| Syntax | YAML 1.2 source or JSON. Both are read by the same reader |
| Documents | One instruction per document. A YAML source may hold several documents, each separated by a line of three hyphens |
| Instruction | A document is an instruction when it carries an `address` object with a `domain` and an `id` |
| Closed shape | The top level and `metadata` accept only the fields that the schema names |
| Host data | Data that GAP does not read lives under the optional top level object `extensions` |

## The source form and the JSON form

YAML is the form people write. JSON-encoded GAP is the form a kernel loads. The command `gapc compile` writes it from the source. Both forms are legal GAP, because the kernel reads the instruction object and not its skin. Keep the `.gap` file as the source of record and treat the compiled JSON as output.

Collect-all Admit collects by capsule hash and the live hash bundle mode on that path is sha256-structure. [LIVE-EVALUATION.md](LIVE-EVALUATION.md) describes that path.

## Several instructions in one file

A family of instructions may share one file. Each document is checked on its own and `gapc check` reports a failure with the number of its document.

```yaml
address:
  domain: com.example.finance
  id: pay.v1
pattern:
  guard: "true"
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
---
address:
  domain: com.example.finance
  id: refund.v1
pattern:
  guard: "true"
action:
  type: template
  content: refund
weight: 0.8
composition:
  type: atomic
metadata:
  provenance: "system.seed"
  version: "1.0.0"
  stability: experimental
```

## Host data under extensions

Each key of `extensions` names one host such as `aep`. The key holds an object that this host defines. A host name uses lower case letters, digits and underscores. `gapc` and live Admit ignore the content. A host keeps its own keys there, so the fields of `metadata` remain the fields of the language.

```yaml
extensions:
  aep:
    lrp_id: example
```

## How an instruction binds

Writing and security are always-on stems and they evaluate on every action path. Every other GAP wall binds to a wrap or to an action path prefix. [kernel-json-wrap.md](kernel-json-wrap.md) holds the detail of that bind.

## Related notes in the AEP repository

The AEP repository carries its own notes on the file format, written for the policies of an AEP runtime: https://github.com/thePM001/AEP-agent-element-protocol/blob/main/AEP-Components/gap/FILE-FORMAT.md
