@PAD: nla-dev-ticket-GAP-285-P10-v1
@GCDE: gap.policy.v1
schema_id: gap.nla.dev-tickets.ticket.v1
format: gap
json_prohibited: true
id: GAP-285-P10
title: Leftover yaml and rego files are not live Admit skins.
status: backlog
priority: P1
track: classic-gap-aep-2.8.5
wave: operational-remediation
plan: GAP-285-OPERATIONAL-REMEDIATION-IMPLEMENTATION-PLAN
repo: GAP
depends_on: GAP-285-P6
scope: AEP-Policy-System aep-builder.policy.yaml coding-agent.policy.yaml content-safety.policy.yaml covenant-only.policy.yaml full-governance.policy.yaml multi-agent.policy.yaml network-egress-no-smtp.policy.yaml readonly-auditor.policy.yaml strict-production.policy.yaml aep-memory-policy.rego aep-policy.rego
notes: Leftover yaml and rego files sit beside live GAP reference docs. They are not collect-all Admit skins. Name them as leftover. Do not load them on collect-all. Point operators to reference GAP files from GAP-285-P6. Ticket close is coding close.
acceptance: Each leftover yaml or rego file is named as not live Admit. Collect-all still loads only GAP reference docs. nla-policy-scan gate writing-mode-rules on the naming text.
bac: nla-policy-scan gate writing-mode-rules
updated: 2026-09-05
biosecurity_directive: |
  Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.
