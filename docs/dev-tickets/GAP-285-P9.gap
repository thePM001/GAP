@PAD: nla-dev-ticket-GAP-285-P9-v1
@GCDE: gap.policy.v1
schema_id: gap.nla.dev-tickets.ticket.v1
format: gap
json_prohibited: true
id: GAP-285-P9
title: Vendor refresh AEP-Components gap README and FILE-FORMAT including sha256-structure.
status: closed
priority: P1
track: classic-gap-aep-2.8.5
wave: operational-remediation
plan: GAP-285-OPERATIONAL-REMEDIATION-IMPLEMENTATION-PLAN
repo: GAP
depends_on: GAP-285-P6 GAP-285-P7 GAP-285-P8
scope: AEP-Components/gap/README.md AEP-Components/gap FILE-FORMAT
notes: Vendored AEP-Components/gap README still teaches trust rings and omits sha256-structure. Refresh it after live reference GAP is corrected. Name seal, freeze-at-seal, 1000 ms, collect-all Admit, Apply, agent_may and sha256-structure. Keep .gap as GAP source. Ticket close is coding close.
acceptance: README matches the live collect-all path. sha256-structure is named. trust rings are not the Admit floor. nla-policy-scan gate writing-mode-rules on the README.
bac: nla-policy-scan gate writing-mode-rules
updated: 2026-09-06
biosecurity_directive: |
  Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.
