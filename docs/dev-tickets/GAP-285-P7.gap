@PAD: nla-dev-ticket-GAP-285-P7-v1
@GCDE: gap.policy.v1
schema_id: gap.nla.dev-tickets.ticket.v1
format: gap
json_prohibited: true
id: GAP-285-P7
title: AEP-Components gap reference CAW and coding-governance live profile.
status: backlog
priority: P0
track: classic-gap-aep-2.8.5
wave: operational-remediation
plan: GAP-285-OPERATIONAL-REMEDIATION-IMPLEMENTATION-PLAN
repo: GAP
depends_on: GAP-285-P8
scope: AEP-Components/gap/policies/reference caw-agent-sandbox.gap caw-coding-agent.gap caw-compiled-runtime.gap caw-dev-multi-repo.gap caw-mount-policies.gap caw-restricted.gap coding-governance-propose.gap implementation-plan-v1.gap task-manifest-v1.gap semantic-impact-envelope.gap multi-base-node-federation-asip-v1.gap multi-base-node-federation-capsule-v1.gap multi-base-node-federation-gap-sync-v1.gap multi-base-node-federation-lattice-v1.gap multi-base-node-federation-smoke.gap
notes: CAW and coding-governance reference GAP still set trust_ring. Live profile after GAP-285-P8 forbids that field on live documents. Drop trust_ring. Add agent_may. Keep CAW as host sandbox compile input. Do not dump GAPLUNE into public GAP. Ticket close is coding close.
acceptance: Zero trust_ring keys in the named files. agent_may present. CAW compile still has a profile id. No GAPLUNE tree dump. nla-policy-scan gate writing-mode-rules on changed files.
bac: nla-policy-scan gate writing-mode-rules
updated: 2026-09-05
biosecurity_directive: |
  Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.
