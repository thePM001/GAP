# Classic GAP versus AEP 2.8.5 evaluation

This document names remaining public-tree holes and the set is not closed. The longer instruction and envelope comparison lives at docs/GAP-OSS-AEP-285-COMPLIANCE-AND-ENVELOPE.md.

Classic GAP already teaches the live AEP 2.8.5 path: seal a capsule, freeze the clock at seal, wait 1000 ms, run every check together as collect-all Admit and then Apply the allowed action. Language files already teach collect-all Admit, agent permission grants, presence Deny and sha256-structure.

The public GAP readme names schema files and product packages that sit on the GAP repository. Vendor compile and reference paths are named as vendor-tree paths on the AEP 2.8.5 tree. Private locators do not belong on this public tree. Live AEP 2.8.5 workspace members include the GAP v1.3 profile package and the policy loader compiles leftover rank presence and agent_permission from the instruction object. Empty grants Deny on miss when an agent action is judged. Writing and security stay always-on stems and other walls bind wrap or action path prefix.

Fifteen named rows stay a derived ledger rather than a live sequential combinator and Skip is not a live verb. Presence of trust_ring is Deny and presence of rank is Deny so leftover rank wall id is gap:leftover_rank_field. Live EPSCOM trust bundle stays sha256-structure on the collect-all path.

## Holes named

- who-may rename: live agent permission is agent_permission and that rename already sits on the public schema and profile package.
- leftover rank wall id: compiled wall id is gap:leftover_rank_field with presence Deny for that field, for trust_ring and for rank. This pass repaired that wall id.
- digest: closed-wall capsule digest is sha256-structure and this pass repaired the package hash so it matches live trust bundle teaching.
- license files: public GAP package license files are Apache-2.0 and this pass replaced UNLICENSED package license files.
- teaching-envelope subset: the public closed-wall envelope still compiles scene, dock, timestamp, sequence, writing and enabled as collect-all teaching rather than the live action-and-snapshot machine.

## Remaining holes after this pass

This pass repaired leftover rank wall id, sha256-structure digest and Apache-2.0 package license files. Remaining holes are still live:

- Teaching-envelope subset versus the live action-and-snapshot machine. Live sealed action still carries action_path, agent_id, payload, tool, dest_dock, scene_id, agent_ts_ms, sequence_number and anomaly_score. Live freeze-at-seal snapshot still carries lattice nodes, satisfied actions, pulse drift 50 ms, pulse age 5000 ms, docks, rate, tools and scanners.
- Missing optional authoring engine. Layer 1 constrained decoding stays optional authoring and is not Admit so the public tree still does not ship that engine.
- Missing simplified teaching runtime that loads, seals, freezes, waits, collect-all then Apply.

Do not scaffold gap-constrained-decode or gap-envelope-min or gap-runtime-min until those hangar plans are approved. Do not dump the hangar language tree into public GAP.
