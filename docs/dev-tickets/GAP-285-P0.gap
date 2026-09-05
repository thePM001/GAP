address:
  domain: com.aep.gap.tickets
  id: GAP-285-P0

pattern: |
  Classic GAP README still teaches a sequential 15-step live chain.
  AEP 2.8.5 live path is seal a capsule, freeze the clock at seal, wait 1000 ms then run every check together then Apply.

action:
  type: structured
  content: |
    id: GAP-285-P0
    title: Rewrite classic GAP README live evaluation to AEP 2.8.5 collect-all Admit.
    status: open
    priority: P0
    track: classic-gap-aep-2.8.5
    wave: instruction-honesty
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    depends_on: none
    scope: README.md
    notes: |
      Product hole: an agent that implements GAP from the OSS README will build the old sequential evaluator.
      Layer 3 still says ring escalation and trust verification after validation before activation.
      AEP 2.8.5 live path is seal a capsule, freeze the clock at seal, wait 1000 ms then run every check together then Apply.
      The fifteen named rows are a derived ledger not a second live pass.
      Skip is not used. Closed-wall Deny names the closed walls.
      Layer 1 constrained decoding stays an authoring gate not Admit.
      Layer 3 must name Base Node kernel pulse plus collect-all Admit.
      Authoring a .gap file is not the check.
      Who-may is agent_may not trust_ring ranks.
      Keep .gap as GAP source. Do not rename classic files to .gaplune.
      Do not edit GAPLUNE trees for this ticket.
    acceptance: |
      Classic GAP README no longer teaches a sequential 15-step live combinator.
      Layer 3 names freeze-at-seal, 1000 ms kernel pulse, collect-all Admit then Apply.
      Fifteen named rows are a derived ledger. Skip is not a live verb.
      Who-may is agent_may. Constrained decoding is named as authoring not Admit.
      nla-policy-scan gate writing-mode-rules on the rewritten README.
    bac: nla-policy-scan gate writing-mode-rules
    updated: 2026-09-05

weight: 1.0

composition:
  type: atomic

metadata:
  provenance: "system.seed"
  version: "1.0.0"
  stability: experimental
  aspect: procedural
  covenants:
    - "do not teach a sequential 15-step live combinator [hard]"
    - "do not treat constrained decoding as Admit [hard]"
    - "do not edit GAPLUNE trees for this ticket [hard]"
