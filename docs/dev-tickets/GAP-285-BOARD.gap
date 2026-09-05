address:
  domain: com.aep.gap.tickets
  id: GAP-285-BOARD

pattern: |
  Required classic GAP tickets on Gitea thePM001/GAP to match AEP 2.8.5 with GAPLUNE trees out of scope.

action:
  type: structured
  content: |
    id: GAP-285-BOARD
    title: Classic GAP to AEP 2.8.5 required ticket board.
    status: open
    priority: P0
    track: classic-gap-aep-2.8.5
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    tickets:
      - GAP-285-P0
      - GAP-285-P1
      - GAP-285-P2
      - GAP-285-P3
      - GAP-285-P4
      - GAP-285-P5
    notes: |
      These tickets live as classic GAP source under docs/dev-tickets on thePM001/GAP so do not convert them to .gaplune and do not edit GAPLUNE trees for this upgrade. Fit analysis is docs/CLASSIC-GAP-VS-AEP-2.8.5.md.
    updated: 2026-09-05

    biosecurity_directive: |
      Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.

weight: 1.0

composition:
  type: sequence

metadata:
  provenance: "system.seed"
  version: "1.0.0"
  stability: experimental
  aspect: procedural
  covenants:
    - "do not convert these tickets to GAPLUNE [hard]"
    - "do not edit GAPLUNE trees for this upgrade [hard]"
