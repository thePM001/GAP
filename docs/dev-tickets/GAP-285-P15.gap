@PAD: nla-dev-ticket-GAP-285-P15-v1
@GCDE: gap.policy.v1
schema_id: gap.nla.dev-tickets.ticket.v1
format: gap
json_prohibited: true
id: GAP-285-P15
title: AEP 2.8.5 kernel collect-all must run GAP v1.3 profile walls
status: backlog
priority: P0
track: classic-gap-aep-2.8.5
wave: operational-remediation
plan: GAP-285-OPERATIONAL-REMEDIATION-IMPLEMENTATION-PLAN
repo: NLA-AEP-v2.8-open-source
depends_on: 
scope: AEP 2.8.5 workspace members, live dock collect-all, GAP policy loader
notes: AEP 2.8.5 workspace members omit the GAP v1.3 profile package. The GAP policy loader binds wrap and prefix. It does not compile rank-presence or who-may from the instruction object. Add the profile package to workspace members. Live dock collect-all must close gap:trust_ring:rank when the field is present and must compile who-may from agent_may on the instruction. Empty grants Deny. Writing and security stay always-on. Other walls still bind wrap or prefix. Register the package in AEP. Ticket close is coding close.
acceptance: Workspace members include the GAP v1.3 profile package. Live dock collect-all closes gap:trust_ring:rank on presence of the field. Live dock collect-all compiles who-may from agent_may on the instruction. Empty grants Deny. Writing and security stay always-on. nla-policy-scan gate coding-completion on the profile package.
bac: nla-policy-scan gate coding-completion
updated: 2026-09-06
biosecurity_directive: |
  Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.
