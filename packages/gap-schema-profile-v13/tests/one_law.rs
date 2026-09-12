// @PAD: gap-285-p6-one-law-reference
// @GCDE: gaplune.policy.v1
// GAP-285-P6 one-law reference crate. Presence of trust_ring is Deny. Agent permission is agent_permission.

pub const TICKET: &str = "GAP-285-P6";
pub const ONE_LAW: &str = "Presence of trust_ring is Deny. Agent permission is agent_permission.";
pub const WALL_LEFTOVER_RANK_FIELD: &str = "gap:leftover_rank_field";
pub const ALWAYS_ON: [&str; 2] = ["writing.gap", "security.gap"];

pub fn one_law_body(out: &mut String) {
    out.clear();
    out.push_str(ONE_LAW);
}

fn has_trust_ring_key(src: &str, out: &mut bool) {
    *out = src.contains("\"trust_ring\"");
}

fn has_agent_permission_key(src: &str, out: &mut bool) {
    *out = src.contains("\"agent_permission\"");
}

fn metadata_wrap_present(src: &str, out: &mut bool) {
    *out = src.contains("\"wrap\"");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_law_text_is_presence_deny() {
        let mut body = String::new();
        one_law_body(&mut body);
        assert_eq ! (body.contains("Presence of trust_ring is Deny"), true);
        assert_eq ! (body.contains("Agent permission is agent_permission"), true);
        assert_eq ! (body.contains("documentary"), false);
        assert_eq ! (body.contains("unused at Admit"), false);
    }

    #[test]
    fn writing_fixture_always_on_has_grants() {
        let src = include_str ! ("../fixtures/writing.gap");
        let mut ring = true;
        let mut may = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        assert_eq ! (ring, false);
        assert_eq ! (may, true);
        assert_eq ! (src.contains("\"guard\": \"true\""), true);
        assert_eq ! (src.contains("\"wrap\""), false);
    }

    #[test]
    fn security_fixture_always_on_has_grants() {
        let src = include_str ! ("../fixtures/security.gap");
        let mut ring = true;
        let mut may = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        assert_eq ! (ring, false);
        assert_eq ! (may, true);
        assert_eq ! (src.contains("\"guard\": \"true\""), true);
        assert_eq ! (src.contains("\"wrap\""), false);
    }

    #[test]
    fn deployment_fixture_binds_wrap_and_prefix() {
        let src = include_str ! ("../fixtures/deployment.gap");
        let mut ring = true;
        let mut may = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        assert_eq ! (ring, false);
        assert_eq ! (may, true);
        assert_eq ! (src.contains("\"wrap\": \"deployment\""), true);
        assert_eq ! (src.contains("\"action_path_prefix\": \"ops:\""), true);
    }

    #[test]
    fn governance_fixture_binds_wrap() {
        let src = include_str ! ("../fixtures/governance.gap");
        let mut ring = true;
        let mut may = false;
        let mut wrap = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        metadata_wrap_present(src, &mut wrap);
        assert_eq ! (ring, false);
        assert_eq ! (may, true);
        assert_eq ! (wrap, true);
        assert_eq ! (src.contains("\"wrap\": \"governance\""), true);
    }
}
