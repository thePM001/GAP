address:
  domain: com.aep.gap.tickets
  id: GAP-285-P5

pattern: |
  Classic GAP README requires Pure YAML 1.2 and no mixed formats while AEP 2.8.5 live kernel policies are JSON-encoded GAP objects with pattern.guard and wrap bind.

action:
  type: structured
  content: |
    id: GAP-285-P5
    title: Classic GAP must allow JSON-encoded kernel policies and wrap or prefix bind.
    status: open
    priority: P1
    track: classic-gap-aep-2.8.5
    wave: schema
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    depends_on: GAP-285-P1
    scope: README.md
    notes: |
      The product hole is that README says Pure YAML 1.2 and No mixed formats so a kernel JSON GAP object looks illegal. AEP 2.8.5 reference policies are JSON objects with pattern.guard often true. Policy-system GAP walls bind to LatticeNode.wrap or an action_path prefix. Writing and security stems still evaluate on every action_path. A finance wrap GAP item does not close an inventory wrap ping. A non-always-on GAP with empty wrap does not fold onto every event. The kernel cares about the instruction object not the skin. YAML remains valid GAP source. Do not edit GAPLUNE trees for this ticket.
    acceptance: |
      README says live kernel policies may be JSON-encoded GAP instructions. YAML remains valid GAP source. Writing and security are always-on stems. Other GAP walls bind to a wrap or prefix.
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
    - "do not forbid JSON-encoded GAP objects for kernel policies [hard]"
    - "do not teach a finance wrap as closing an inventory ping [hard]"
