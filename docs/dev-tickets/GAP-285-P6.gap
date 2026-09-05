@PAD: nla-dev-ticket-GAP-285-P6-v1
@GCDE: gap.policy.v1
schema_id: gap.nla.dev-tickets.ticket.v1
format: gap
json_prohibited: true
id: GAP-285-P6
title: AEP-Policy-System reference GAP docs drop trust_ring and add agent_may.
status: backlog
priority: P0
track: classic-gap-aep-2.8.5
wave: operational-remediation
plan: GAP-285-OPERATIONAL-REMEDIATION-IMPLEMENTATION-PLAN
repo: GAP
depends_on: GAP-285-P8
scope: AEP-Policy-System/reference writing.gap security.gap governance.gap deployment.gap network-egress-no-smtp.gap hipaa.gap gdpr.gap eu-ai-act.gap iso-42001.gap nist-ai-rmf.gap soc2-type2.gap README.md
notes: These reference files still set trust_ring and have zero agent_may while several still claim AEP 2.75 provenance. Live AEP 2.8.5 collect-all Admit loads them. Drop trust_ring. Add agent_may. Bind wrap or action_path_prefix where the wall is not always-on. Writing and security stay always-on stems. JSON-encoded GAP remains valid. YAML remains valid. Ticket close is coding close.
acceptance: Zero trust_ring keys in the named reference GAP files. Each file has agent_may. README no longer titles AEP 2.75 as the live lattice. writing.gap and security.gap remain always-on. nla-policy-scan gate writing-mode-rules on changed files.
bac: nla-policy-scan gate writing-mode-rules
updated: 2026-09-05
biosecurity_directive: |
  Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.
