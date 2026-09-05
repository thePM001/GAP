address:
  domain: com.aep.gap.tickets
  id: GAP-285-P2

pattern: |
  Classic GAP README names files that are not in the three-file OSS snapshot and it ships truncated principle 5, a truncated scanners row and a structural toolchain name.

action:
  type: structured
  content: |
    id: GAP-285-P2
    title: Classic GAP OSS snapshot hygiene. README claims must match the tree.
    status: open
    priority: P1
    track: classic-gap-aep-2.8.5
    wave: instruction-honesty
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    depends_on: GAP-285-P0
    scope: README.md
    notes: |
      The product hole is that README lists GAP v1 spec sheet.md and BIOSECURITY.md while those files are not in thePM001/GAP. Principle 5 ends at Trust, covenants. The scanners table row starts mid-list at Injection so PII is dropped. Restore the named files in OSS or stop listing them. Repair truncated lines. Name the real CLI that this repo ships or say this snapshot has no CLI. Keep .gap as GAP source and do not rename classic files to .gaplune. Do not publish GAPLUNE trees into public GAP. Do not edit GAPLUNE trees for this ticket.
    acceptance: |
      Every file named in classic GAP README exists in thePM001/GAP or is removed from the file table. Principle 5 and the scanners row are complete sentences. Toolchain names the real CLI or states that this snapshot has no CLI. No GAPLUNE tree dump in public GAP. Extension law stays: .gap is GAP source.
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
    - "do not list files that are not in the OSS tree [hard]"
    - "do not dump GAPLUNE trees into public GAP [hard]"
