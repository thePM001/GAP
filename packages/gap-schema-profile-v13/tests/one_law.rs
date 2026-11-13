// @PAD: gap-285-p6-one-law-reference
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
        assert!(body.contains("Presence of trust_ring is Deny"));
        assert!(body.contains("Agent permission is agent_permission"));
        assert!(!body.contains("documentary"));
        assert!(!body.contains("unused at Admit"));
    }

    #[test]
    fn writing_fixture_always_on_has_grants() {
        let src = include_str!("../fixtures/writing.gap");
        let mut ring = true;
        let mut may = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        assert!(!ring);
        assert!(may);
        assert!(src.contains("\"guard\": \"true\""));
        assert!(!src.contains("\"wrap\""));
    }

    #[test]
    fn security_fixture_always_on_has_grants() {
        let src = include_str!("../fixtures/security.gap");
        let mut ring = true;
        let mut may = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        assert!(!ring);
        assert!(may);
        assert!(src.contains("\"guard\": \"true\""));
        assert!(!src.contains("\"wrap\""));
    }

    #[test]
    fn deployment_fixture_binds_wrap_and_prefix() {
        let src = include_str!("../fixtures/deployment.gap");
        let mut ring = true;
        let mut may = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        assert!(!ring);
        assert!(may);
        assert!(src.contains("\"wrap\": \"deployment\""));
        assert!(src.contains("\"action_path_prefix\": \"ops:\""));
    }

    #[test]
    fn governance_fixture_binds_wrap() {
        let src = include_str!("../fixtures/governance.gap");
        let mut ring = true;
        let mut may = false;
        let mut wrap = false;
        has_trust_ring_key(src, &mut ring);
        has_agent_permission_key(src, &mut may);
        metadata_wrap_present(src, &mut wrap);
        assert!(!ring);
        assert!(may);
        assert!(wrap);
        assert!(src.contains("\"wrap\": \"governance\""));
    }
}
