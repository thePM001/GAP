address:
  domain: com.aep.gap.tickets
  id: GAP-285-P3

pattern: |
  Classic GAP teaches skip and silent exhaustion while AEP 2.8.5 Deny is a closed-wall set and replay is keyed at enqueue.

action:
  type: structured
  content: |
    id: GAP-285-P3
    title: Classic GAP must not teach skip. Closed-wall Deny and reseal on retry.
    status: closed
    priority: P1
    track: classic-gap-aep-2.8.5
    wave: instruction-honesty
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    depends_on: GAP-285-P0
    scope: README.md GAP meta schema v1.2.json
    notes: |
      The product hole is that Metadata.enabled says when false the instruction is loaded but never evaluated by the lattice which is Admit skip. AEP 2.8.5 does not skip walls. Classic GAP execution.retry and on_exhaustion fail are instruction-runtime retry and they are not kernel Admit. AEP 2.8.5 Deny returns closed wall ids, reasons and a prescribed repair for unbound scene, dock, time, sequence and writing. Grant lists stay off repair. Digest replay is keyed at enqueue so a retry must seal a new capsule. Same input plus same sealed bytes is replay not a new Admit. Attractors stay forensic so do not add attractor skip of Admit. Do not edit GAPLUNE trees for this ticket.
    acceptance: |
      README does not teach skip a lattice step. Schema text for enabled is load-time not Admit skip. Closed-wall Deny is named and retry reseals. Attractors are not taught as a skip of Admit.
    bac: nla-policy-scan gate writing-mode-rules
    updated: 2026-09-05

    biosecurity_directive: |
      Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.

weight: 1.0

composition:
  type: atomic

metadata:
  provenance: "system.seed"
  version: "1.0.0"
  stability: experimental
  aspect: procedural
  covenants:
    - "do not teach skip as a live Admit verb [hard]"
    - "do not teach attractor skip of Admit [hard]"
