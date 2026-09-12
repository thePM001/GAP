// crate: gap-schema-profile-v13 fit tests
// GAP-OSS-FIT-P1. Fit document must name remaining public-tree holes.

#[test]
fn fit_document_does_not_claim_the_set_is_closed() {
    let body = include_str!("../../../docs/CLASSIC-GAP-VS-AEP-2.8.5.md");
    assert_eq ! (body.contains("no holes remain"), false);
    assert_eq ! (body.contains("none remain in this set"), false);
    assert_eq ! (body.contains("the set is not closed"), true);
}

#[test]
fn fit_document_names_remaining_and_repaired_holes() {
    let body = include_str!("../../../docs/CLASSIC-GAP-VS-AEP-2.8.5.md");
    assert_eq ! (body.contains("who-may"), true);
    assert_eq ! (body.contains("gap:leftover_rank_field"), true);
    assert_eq ! (body.contains("sha256-structure"), true);
    assert_eq ! (body.contains("Apache-2.0"), true);
    assert_eq ! (body.contains("teaching-envelope"), true);
    assert_eq ! (body.contains("GAP-OSS-AEP-285-COMPLIANCE-AND-ENVELOPE.md"), true);
    assert_eq ! (body.contains("action-and-snapshot machine"), true);
    assert_eq ! (body.contains("optional authoring"), true);
}
