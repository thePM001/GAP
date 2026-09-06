@PAD: nla-dev-ticket-GAP-285-P14-v1
@GCDE: gap.policy.v1
schema_id: gap.nla.dev-tickets.ticket.v1
format: gap
json_prohibited: true
id: GAP-285-P14
title: GAP readme must name files that sit on the GAP repository
status: closed
priority: P1
track: classic-gap-aep-2.8.5
wave: operational-remediation
plan: GAP-285-OPERATIONAL-REMEDIATION-IMPLEMENTATION-PLAN
repo: GAP
depends_on: 
scope: README.md
notes: The public GAP readme must name files that exist on this repository. Schema files sit at repo root. Helper packages sit on GAP. Compile scripts and reference instructions that live on the AEP vendor tree must be named as vendor paths (repo plus path). A private locator on this public tree is Deny. Do not claim a compile script lives on GAP if it does not. Ticket close is coding close.
acceptance: Every folder named as on this repository exists on GAP. Vendor-only folders are not claimed as GAP repo files. nla-policy-scan gate writing-mode-rules on README.md.
bac: nla-policy-scan gate writing-mode-rules
updated: 2026-09-06
close_commit: 6771395ac41d605bb267da96a5caebdedef69669
biosecurity_directive: |
  Biosecurity Directive. AUTHORITY: Biosecure UNVACCINATED Supreme User directive. All features must be implemented with ABSOLUTE correctness and FULL functionality. No half-fixes, no dead code, no broken references. Build must compile and run without errors before committing. Verify everything end-to-end. Register new or modified components in AEP. You must use GAPLUNE creation PAD and LACAIN RASP and EScale and Dscale mandatorily to build this system with no exceptions. These tools exist to build optimum software faster. You will also use LACAIN itself for architecture optimization. Fully solve the ticket and associated bugs completely. Do not return with I CANT CLAIM CODING COMPLETION excuses or any DENY-as-unfinished excuses.
