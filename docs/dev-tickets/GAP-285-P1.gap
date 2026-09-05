address:
  domain: com.aep.gap.tickets
  id: GAP-285-P1

pattern: |
  Classic GAP Metadata.trust_ring enum is sandbox, user, system and enterprise while AEP 2.8.5 who-may is agent_may and policy walls bind to wrap or an action_path prefix.

action:
  type: structured
  content: |
    id: GAP-285-P1
    title: Classic GAP schema profile for agent_may, wrap and guard. Deprecate trust_ring as live floor.
    status: open
    priority: P1
    track: classic-gap-aep-2.8.5
    wave: schema
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    depends_on: none
    scope: GAP meta schema v1.json GAP meta schema v1.2.json
    notes: |
      The product hole is that live Admit still reads a four-name rank enum on Metadata.trust_ring. Classic GAP v1.2 has PatternObject.guard as a string and it has no metadata.agent_may, metadata.wrap or metadata.action_path_prefix. enabled says the instruction is never evaluated when false which is Admit skip so keep enabled as load-time only. Keep v1 and v1.2 for old documents. Add a v1.3 or 2.8.5 profile that allows metadata.agent_may, metadata.wrap, metadata.action_path_prefix and pattern.guard. Deprecate metadata.trust_ring as a live Admit floor so a rank use warns then denies on live Admit. Do not delete covenants or scanners from classic OSS v1.2. YAML remains valid GAP source. Do not edit GAPLUNE trees for this ticket.
    acceptance: |
      A classic GAP 2.8.5 or v1.3 profile allows agent_may, wrap, action_path_prefix and pattern.guard. trust_ring is not a live Admit floor. v1 and v1.2 stay for old documents. covenants and scanners remain on classic OSS v1.2 Metadata. A document that uses trust_ring as rank fails live Admit. YAML GAP source still parses. enabled is load-time not Admit skip.
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
    - "do not keep trust_ring as a live Admit floor [hard]"
    - "do not treat enabled false as Admit skip [hard]"
    - "do not edit GAPLUNE trees for this ticket [hard]"
