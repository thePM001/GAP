address:
  domain: com.aep.gap.tickets
  id: GAP-285-P4

pattern: |
  Classic GAP names Ed25519 or ML-DSA-65 on proof bundles while AEP 2.8.5 live trust bundle mode is sha256-structure.

action:
  type: structured
  content: |
    id: GAP-285-P4
    title: Classic GAP proof claims must not say every AEP 2.8.5 attach ships ML-DSA.
    status: open
    priority: P1
    track: classic-gap-aep-2.8.5
    wave: instruction-honesty
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    depends_on: GAP-285-P0
    scope: README.md
    notes: |
      The product hole is that README says signed proof bundles with Ed25519 or ML-DSA-65 signatures as if that is the live AEP 2.8.5 attach. AEP 2.8.5 EPSCOM trust bundle mode is sha256-structure and the signatures loader denies an ML-DSA claim on sha256-structure. Keep post-quantum signatures as an optional proof algorithm not as the default live bundle. Do not edit GAPLUNE trees for this ticket.
    acceptance: |
      Classic GAP README does not say every AEP 2.8.5 attach ships ML-DSA. Optional proof algorithms may still name Ed25519 or ML-DSA-65. Live bundle mode is not claimed as ML-DSA by default.
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
    - "do not claim ML-DSA as the default live AEP 2.8.5 bundle [hard]"
