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
