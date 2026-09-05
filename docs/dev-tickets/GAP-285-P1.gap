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
    status: closed
    priority: P1
    track: classic-gap-aep-2.8.5
    wave: schema
    plan: docs/CLASSIC-GAP-VS-AEP-2.8.5.md
    repo: GAP
    depends_on: none
    scope: GAP meta schema v1.json GAP meta schema v1.2.json GAP meta schema v1.3.json crate/
    notes: |
      The product hole is that live Admit still reads a four-name rank enum on Metadata.trust_ring. Classic GAP v1.2 has PatternObject.guard as a string and it has no metadata.agent_may, metadata.wrap or metadata.action_path_prefix. enabled says the instruction is never evaluated when false which is Admit skip so keep enabled as load-time only. Keep v1 and v1.2 for old documents. Add a v1.3 or 2.8.5 profile that allows metadata.agent_may, metadata.wrap, metadata.action_path_prefix and pattern.guard. Deprecate metadata.trust_ring as a live Admit floor so a rank use warns then denies on live Admit. Do not delete covenants or scanners from classic OSS v1.2. YAML remains valid GAP source. Do not edit GAPLUNE trees for this ticket.
      Close deposit: GAP meta schema v1.3.json commit 424373edbe00e54e644a2c586b1b307c2f447333. Crate gap-schema-profile-v13 live Admit lib commit 2342d27811e3ca0d48992acc0720fc50c5fc566a. README.md commit c2c1134f5f10ee4f406ca54013eb3eb165f62334. AEP vendor gap-meta-schema-v1.3.json commit a6fd56d1e707882f90033f63614b91f1c1cd0015. AEP crate aep-gap-schema-profile-v13 lib commit 0756aa9bb61c81cb0311ec86660a488ca32dca88. Who-may is agent_may. wrap and action_path_prefix bind. pattern.guard is a string or a structured ConstraintExpression. trust_ring rank use warns then denies on live Admit. enabled is load-time. v1 and v1.2 stay. covenants and scanners remain on classic OSS v1.2. YAML GAP source still parses. GAP crate cargo test 12 passed. AEP crate cargo test 13 passed. CLI rank document allow=false closed=gap:trust_ring:rank. Writing-mode scanners epscom_all, em_dash, oxford_comma, double_hyphen, epscom_forbidden_word, epscom_negative_positive and nla-writing scan returned ALLOW on the v1.3 README sections.
    acceptance: |
      A classic GAP 2.8.5 or v1.3 profile allows agent_may, wrap, action_path_prefix and pattern.guard. trust_ring is not a live Admit floor. v1 and v1.2 stay for old documents. covenants and scanners remain on classic OSS v1.2 Metadata. A document that uses trust_ring as rank fails live Admit. YAML GAP source still parses. enabled is load-time not Admit skip.
    bac: nla-policy-scan gate writing-mode-rules
    updated: 2026-09-05
    close_commit: 424373edbe00e54e644a2c586b1b307c2f447333

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
