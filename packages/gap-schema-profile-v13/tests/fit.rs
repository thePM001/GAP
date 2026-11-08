// crate: gap-schema-profile-v13 fit tests
// GAP-OSS-FIT-P1. Fit document must name remaining public-tree holes.

#[test]
fn fit_document_does_not_claim_the_set_is_closed() {
    let body = include_str!("../../../docs/CLASSIC-GAP-VS-AEP-2.8.5.md");
    assert!(!body.contains("no holes remain"));
    assert!(!body.contains("none remain in this set"));
    assert!(body.contains("the set is not closed"));
}

#[test]
fn fit_document_names_remaining_and_repaired_holes() {
    let body = include_str!("../../../docs/CLASSIC-GAP-VS-AEP-2.8.5.md");
    assert!(body.contains("who-may"));
    assert!(body.contains("gap:leftover_rank_field"));
    assert!(body.contains("sha256-structure"));
    assert!(body.contains("Apache-2.0"));
    assert!(body.contains("teaching-envelope"));
    assert!(body.contains("GAP-OSS-AEP-285-COMPLIANCE-AND-ENVELOPE.md"));
    assert!(body.contains("action-and-snapshot machine"));
    assert!(body.contains("optional authoring"));
}
