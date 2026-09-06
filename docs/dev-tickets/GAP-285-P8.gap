@PAD: nla-dev-ticket-GAP-285-P8-v1
@GCDE: gap.policy.v1
schema_id: gap.nla.dev-tickets.ticket.v1
format: gap
json_prohibited: true
id: GAP-285-P8
title: One law for trust_ring. Presence Deny. Crate keeps Deny on any non-empty field.
status: closed
priority: P0
track: classic-gap-aep-2.8.5
wave: operational-remediation
plan: GAP-285-OPERATIONAL-REMEDIATION-IMPLEMENTATION-PLAN
repo: GAP
depends_on: none
scope: GAP meta schema v1.3.json crate AEP-Components/gap/schemas
notes: Live GAP documents must not set trust_ring or rank. Schema v1.3 declares rank as Presence Deny. It is not a documentary label. gap-schema-profile-v13 keeps Deny on any non-empty rank value. AEP vendored schema matches GAP v1.3. Who-may is agent_may. Ticket close is coding close.
acceptance: A live GAP object with trust_ring set is Deny. A live GAP object with rank set is Deny. A live GAP object without those fields and with agent_may can Admit. Vendor schema matches GAP v1.3 live profile. Schema text does not call rank unused at Admit. nla-policy-scan gate writing-mode-rules on changed schema text.
bac: nla-policy-scan gate writing-mode-rules
updated: 2026-09-06
close_commit: ae143a68121104d1e4ad00c44ac38138da5ca364
crate_commit: d361411788d45ef0dfe569f31bd1312764803b75
vendor_commit: e39b6ed29c5c00d484b00e76e854197cb43cf909
biosecurity_directive: |
  Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.
