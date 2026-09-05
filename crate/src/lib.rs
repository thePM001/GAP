// crate: gap-schema-profile-v13
// GAP-285-P1 classic GAP v1.3 live Admit profile.
use serde_json::{Map, Value};

pub const TICKET: &str = "GAP-285-P1";
pub const PROFILE: &str = "v1.3";
pub const WALL_TRUST_RING_RANK: &str = "gap:trust_ring:rank";
pub const WALL_AGENT_MAY: &str = "gap:agent_may";
pub const WALL_WRAP_BIND: &str = "gap:wrap:bind";
pub const WALL_PATTERN_GUARD: &str = "gap:pattern:guard";
pub const RANK_SANDBOX: &str = "sandbox";
pub const RANK_USER: &str = "user";
pub const RANK_SYSTEM: &str = "system";
pub const RANK_ENTERPRISE: &str = "enterprise";
pub const SCHEMA_V13_CONTRACT: &str = "GAP Instruction Meta-Schema v1.3\nagent_may\nwrap\naction_path_prefix\noneOf\nload-time\nNot a live Admit floor.\ncovenants\nscanners\n";

pub fn schema_v13_body(out: &mut String) {
    out.clear();
    out.push_str(SCHEMA_V13_CONTRACT);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitWall {
    pub id: String,
    pub closed: bool,
    pub reason: String,
}

impl AdmitWall {
    pub fn open_into(id: &str, wall: &mut AdmitWall) {
        wall.id = String::from(id);
        wall.closed = false;
        wall.reason = String::new();
    }
    pub fn close_into(id: &str, reason: &str, wall: &mut AdmitWall) {
        wall.id = String::from(id);
        wall.closed = true;
        wall.reason = String::from(reason);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitResult {
    pub allow: bool,
    pub closed: Vec<AdmitWall>,
}

impl AdmitResult {
    pub fn closed_set_key(self_ref: &AdmitResult, out: &mut String) {
        out.clear();
        let mut rows: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < self_ref.closed.len() {
            let w = &self_ref.closed[i];
            let mut s = w.id.clone();
            s.push(char::from(31));
            s.push_str(&w.reason);
            rows.push(s);
            i += 1;
        }
        rows.sort();
        rows.dedup();
        let mut j = 0usize;
        while j < rows.len() {
            if 0 < j {
                out.push(char::from(10));
            }
            out.push_str(&rows[j]);
            j += 1;
        }
    }
}

