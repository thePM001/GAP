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


pub fn admit_collect_all(walls: &[AdmitWall], out: &mut AdmitResult) {
    let mut closed: Vec<AdmitWall> = Vec::new();
    let mut i = 0usize;
    while i < walls.len() {
        if walls[i].closed {
            closed.push(walls[i].clone());
        }
        i += 1;
    }
    closed.sort_by(|a, b| a.id.cmp(&b.id).then(a.reason.cmp(&b.reason)));
    closed.dedup_by(|a, b| a.id == b.id && a.reason == b.reason);
    out.allow = closed.is_empty();
    out.closed = closed;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentMayGrant {
    pub agent_id: String,
    pub action: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveAdmitRequest {
    pub agent_id: String,
    pub action: String,
    pub wrap: String,
    pub action_path: String,
}

fn json_str(v: &Value, key: &str, out: &mut String) {
    out.clear();
    if let Some(x) = v.get(key) {
        if let Some(s) = x.as_str() {
            out.push_str(s.trim());
        }
    }
}

fn meta_of(doc: &Value) -> &Value {
    if let Some(m) = doc.get("metadata") {
        m
    } else {
        &Value::Null
    }
}

pub fn enabled_is_load_time(out: &mut bool) {
    *out = true;
}

pub fn enabled_skips_admit(_doc: &Value, out: &mut bool) {
    *out = false;
}

fn is_rank_name(name: &str, out: &mut bool) {
    *out = name == RANK_SANDBOX || name == RANK_USER || name == RANK_SYSTEM || name == RANK_ENTERPRISE;
}

pub fn compile_trust_ring_rank_wall(doc: &Value, wall: &mut AdmitWall) {
    let mut ring = String::new();
    json_str(meta_of(doc), "trust_ring", &mut ring);
    if ring.is_empty() {
        AdmitWall::open_into(WALL_TRUST_RING_RANK, wall);
        return;
    }
    let mut rank = false;
    is_rank_name(&ring, &mut rank);
    AdmitWall::close_into(
        WALL_TRUST_RING_RANK,
        "warn: trust_ring is not a live Admit floor; who-may is agent_may; rank use denied",
        wall,
    );
}


pub fn admit_collect_all(walls: &[AdmitWall], out: &mut AdmitResult) {
    let mut closed: Vec<AdmitWall> = Vec::new();
    let mut i = 0usize;
    while i < walls.len() {
        if walls[i].closed {
            closed.push(walls[i].clone());
        }
        i += 1;
    }
    closed.sort_by(|a, b| a.id.cmp(&b.id).then(a.reason.cmp(&b.reason)));
    closed.dedup_by(|a, b| a.id == b.id && a.reason == b.reason);
    out.allow = closed.is_empty();
    out.closed = closed;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentMayGrant {
    pub agent_id: String,
    pub action: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveAdmitRequest {
    pub agent_id: String,
    pub action: String,
    pub wrap: String,
    pub action_path: String,
}

fn json_str(v: &Value, key: &str, out: &mut String) {
    out.clear();
    if let Some(x) = v.get(key) {
        if let Some(s) = x.as_str() {
            out.push_str(s.trim());
        }
    }
}

fn meta_of(doc: &Value) -> &Value {
    if let Some(m) = doc.get("metadata") {
        m
    } else {
        &Value::Null
    }
}

pub fn enabled_is_load_time(out: &mut bool) {
    *out = true;
}

pub fn enabled_skips_admit(_doc: &Value, out: &mut bool) {
    *out = false;
}

fn is_rank_name(name: &str, out: &mut bool) {
    *out = name == RANK_SANDBOX || name == RANK_USER || name == RANK_SYSTEM || name == RANK_ENTERPRISE;
}

pub fn compile_trust_ring_rank_wall(doc: &Value, wall: &mut AdmitWall) {
    let mut ring = String::new();
    json_str(meta_of(doc), "trust_ring", &mut ring);
    if ring.is_empty() {
        AdmitWall::open_into(WALL_TRUST_RING_RANK, wall);
        return;
    }
    let mut rank = false;
    is_rank_name(&ring, &mut rank);
    AdmitWall::close_into(
        WALL_TRUST_RING_RANK,
        "warn: trust_ring is not a live Admit floor; who-may is agent_may; rank use denied",
        wall,
    );
}


fn grant_from_value(v: &Value, out: &mut AgentMayGrant, ok: &mut bool) {
    *ok = false;
    if let Some(s) = v.as_str() {
        let s = s.trim();
        if s.is_empty() {
            return;
        }
        if let Some((a, act)) = s.split_once(char::from(47)) {
            let agent_id = a.trim();
            let action = act.trim();
            if agent_id.is_empty() == false && action.is_empty() == false {
                out.agent_id = String::from(agent_id);
                out.action = String::from(action);
                *ok = true;
                return;
            }
        }
        out.agent_id = String::from("*");
        out.action = String::from(s);
        *ok = true;
        return;
    }
    if v.is_object() {
        let mut agent_id = String::new();
        let mut action = String::new();
        json_str(v, "agent_id", &mut agent_id);
        json_str(v, "action", &mut action);
        if agent_id.is_empty() || action.is_empty() {
            return;
        }
        out.agent_id = agent_id;
        out.action = action;
        *ok = true;
    }
}

pub fn grants_from_doc(doc: &Value, out: &mut Vec<AgentMayGrant>) {
    out.clear();
    let meta = meta_of(doc);
    if let Some(arr) = meta.get("agent_may").and_then(|x| x.as_array()) {
        let mut i = 0usize;
        while i < arr.len() {
            let mut g = AgentMayGrant {
                agent_id: String::new(),
                action: String::new(),
            };
            let mut ok = false;
            grant_from_value(&arr[i], &mut g, &mut ok);
            if ok {
                out.push(g);
            }
            i += 1;
        }
    }
}

fn grant_matches(grant: &AgentMayGrant, agent_id: &str, action: &str, out: &mut bool) {
    let star = grant.agent_id == "*" && agent_id.is_empty() == false;
    let named = agent_id.is_empty() == false && grant.agent_id == agent_id;
    let unbound = grant.agent_id == "unbound" && agent_id.is_empty();
    let agent_ok = star || named || unbound;
    let action_ok = grant.action == "*" || grant.action == action;
    *out = agent_ok && action_ok;
}

pub fn agent_is_granted(agent_id: &str, action: &str, grants: &[AgentMayGrant], out: &mut bool) {
    *out = false;
    let mut i = 0usize;
    while i < grants.len() {
        let mut hit = false;
        grant_matches(&grants[i], agent_id, action, &mut hit);
        if hit {
            *out = true;
            return;
        }
        i += 1;
    }
}

pub fn agent_may_wall_id(agent: &str, action: &str, out: &mut String) {
    out.clear();
    out.push_str(WALL_AGENT_MAY);
    out.push(char::from(58));
    if agent.is_empty() {
        out.push_str("unbound");
    } else {
        out.push_str(agent);
    }
    out.push(char::from(58));
    out.push_str(action);
}

