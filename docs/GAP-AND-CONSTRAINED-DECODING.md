# GAP and constrained decoding

Constrained decoding restricts the tokens a language model may emit, so that its output matches a JSON Schema, a regular expression or a grammar. This document states how that technique relates to GAP and compares GAP with dottxt, a product built on it.

## Where constrained decoding sits in GAP

GAP has three layers and constrained decoding is the first one. Layer 1 is optional help for authoring: when a decoder is wired, it may constrain the tokens of a model to the declared type. The action flag `structured_generation: true` asks for it.

Constrained decoding is not Admit. A well formed output is still only a proposal. The decision on an action is the collect-all Admit of Layer 3. This tree does not ship a wired Layer 1 engine.

## GAP compared with dottxt

dottxt constrains tokens to match JSON Schema, regex or CFG at the level of the logits. That structural constraint is the whole of what it does. GAP is a complete programming language. The structural constraint of dottxt corresponds to one action flag in GAP.

| Subject | dottxt | GAP |
| --- | --- | --- |
| Structural constraint of the output | Yes, at the logits | Optional, as Layer 1 when a decoder is wired |
| Types | JSON Schema, regex and CFG | Mathematical types with native validators |
| Decision on an action | None | Collect-all Admit and then Apply, after the freeze at seal and the 1000 ms kernel pulse |
| Record of the decision | None | A derived ledger of fifteen rows |
| Who may act | None | agent_permission grants |
| Rules of behaviour | None | Behavioural covenants |
| Trust bundle | None | sha256-structure in live AEP 2.9.0, with optional proof algorithms |
| Change over time | None | Instructions with fields for self-generation, which evolve specialised variants once a runtime drives them |

GAP covers the authoring gate and the live collect-all Admit. A tool for constrained decoding covers the authoring gate alone.
