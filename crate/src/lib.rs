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


pub fn compile_agent_may_profile_wall(doc: &Value, req: &LiveAdmitRequest, wall: &mut AdmitWall) {
    let agent_id = req.agent_id.trim();
    let action = req.action.trim();
    if agent_id.is_empty() && action.is_empty() {
        AdmitWall::open_into(WALL_AGENT_MAY, wall);
        return;
    }
    let mut grants: Vec<AgentMayGrant> = Vec::new();
    grants_from_doc(doc, &mut grants);
    let mut id = String::new();
    agent_may_wall_id(agent_id, action, &mut id);
    if grants.is_empty() {
        AdmitWall::close_into(
            &id,
            "GAP dimension agent_may closed: empty grants fail closed for agent actions",
            wall,
        );
        return;
    }
    let mut granted = false;
    agent_is_granted(agent_id, action, &grants, &mut granted);
    if granted {
        AdmitWall::open_into(&id, wall);
        return;
    }
    let mut reason = String::from("GAP dimension agent_may closed: agent ");
    if agent_id.is_empty() {
        reason.push_str("unbound");
    } else {
        reason.push_str(agent_id);
    }
    reason.push_str(" may not ");
    reason.push_str(action);
    AdmitWall::close_into(&id, &reason, wall);
}

pub fn action_path_matches_prefix(action_path: &str, prefix: &str, out: &mut bool) {
    let p = prefix.trim();
    let a = action_path.trim();
    *out = false;
    if p.is_empty() || a.is_empty() {
        return;
    }
    if a == p {
        *out = true;
        return;
    }
    if a.starts_with(p) == false {
        return;
    }
    let last = p.as_bytes()[p.len() - 1];
    if last == 58 || last == 47 {
        *out = true;
        return;
    }
    let rest = &a[p.len()..];
    *out = rest.starts_with(char::from(58).to_string().as_str()) || rest.starts_with(char::from(47).to_string().as_str());
}

pub fn wrap_or_prefix_binds(gap_wrap: &str, gap_prefix: &str, node_wrap: &str, action_path: &str, out: &mut bool) {
    *out = false;
    let w = gap_wrap.trim();
    let nw = node_wrap.trim();
    if w.is_empty() == false && w == nw {
        *out = true;
        return;
    }
    let pref = gap_prefix.trim();
    if pref.is_empty() == false {
        action_path_matches_prefix(action_path, pref, out);
    }
}

pub fn compile_wrap_prefix_bind(doc: &Value, req: &LiveAdmitRequest, wall: &mut AdmitWall) {
    let meta = meta_of(doc);
    let mut wrap = String::new();
    let mut prefix = String::new();
    json_str(meta, "wrap", &mut wrap);
    json_str(meta, "action_path_prefix", &mut prefix);
    if wrap.is_empty() && prefix.is_empty() {
        AdmitWall::open_into(WALL_WRAP_BIND, wall);
        return;
    }
    let mut binds = false;
    wrap_or_prefix_binds(&wrap, &prefix, &req.wrap, &req.action_path, &mut binds);
    if binds {
        AdmitWall::open_into(WALL_WRAP_BIND, wall);
    } else {
        AdmitWall::close_into(
            WALL_WRAP_BIND,
            "wrap or action_path_prefix does not bind this action_path",
            wall,
        );
    }
}


pub fn compile_agent_may_profile_wall(doc: &Value, req: &LiveAdmitRequest, wall: &mut AdmitWall) {
    let agent_id = req.agent_id.trim();
    let action = req.action.trim();
    if agent_id.is_empty() && action.is_empty() {
        AdmitWall::open_into(WALL_AGENT_MAY, wall);
        return;
    }
    let mut grants: Vec<AgentMayGrant> = Vec::new();
    grants_from_doc(doc, &mut grants);
    let mut id = String::new();
    agent_may_wall_id(agent_id, action, &mut id);
    if grants.is_empty() {
        AdmitWall::close_into(
            &id,
            "GAP dimension agent_may closed: empty grants fail closed for agent actions",
            wall,
        );
        return;
    }
    let mut granted = false;
    agent_is_granted(agent_id, action, &grants, &mut granted);
    if granted {
        AdmitWall::open_into(&id, wall);
        return;
    }
    let mut reason = String::from("GAP dimension agent_may closed: agent ");
    if agent_id.is_empty() {
        reason.push_str("unbound");
    } else {
        reason.push_str(agent_id);
    }
    reason.push_str(" may not ");
    reason.push_str(action);
    AdmitWall::close_into(&id, &reason, wall);
}

pub fn action_path_matches_prefix(action_path: &str, prefix: &str, out: &mut bool) {
    let p = prefix.trim();
    let a = action_path.trim();
    *out = false;
    if p.is_empty() || a.is_empty() {
        return;
    }
    if a == p {
        *out = true;
        return;
    }
    if a.starts_with(p) == false {
        return;
    }
    let last = p.as_bytes()[p.len() - 1];
    if last == 58 || last == 47 {
        *out = true;
        return;
    }
    let rest = &a[p.len()..];
    *out = rest.starts_with(char::from(58).to_string().as_str()) || rest.starts_with(char::from(47).to_string().as_str());
}

pub fn wrap_or_prefix_binds(gap_wrap: &str, gap_prefix: &str, node_wrap: &str, action_path: &str, out: &mut bool) {
    *out = false;
    let w = gap_wrap.trim();
    let nw = node_wrap.trim();
    if w.is_empty() == false && w == nw {
        *out = true;
        return;
    }
    let pref = gap_prefix.trim();
    if pref.is_empty() == false {
        action_path_matches_prefix(action_path, pref, out);
    }
}

pub fn compile_wrap_prefix_bind(doc: &Value, req: &LiveAdmitRequest, wall: &mut AdmitWall) {
    let meta = meta_of(doc);
    let mut wrap = String::new();
    let mut prefix = String::new();
    json_str(meta, "wrap", &mut wrap);
    json_str(meta, "action_path_prefix", &mut prefix);
    if wrap.is_empty() && prefix.is_empty() {
        AdmitWall::open_into(WALL_WRAP_BIND, wall);
        return;
    }
    let mut binds = false;
    wrap_or_prefix_binds(&wrap, &prefix, &req.wrap, &req.action_path, &mut binds);
    if binds {
        AdmitWall::open_into(WALL_WRAP_BIND, wall);
    } else {
        AdmitWall::close_into(
            WALL_WRAP_BIND,
            "wrap or action_path_prefix does not bind this action_path",
            wall,
        );
    }
}


pub fn compile_guard_wall(doc: &Value, wall: &mut AdmitWall) {
    let p = doc.get("pattern");
    if p.is_none() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    let p = p.unwrap();
    if p.is_string() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    let g = p.get("guard");
    if g.is_none() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    let g = g.unwrap();
    if g.is_string() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    if g.is_object() {
        let expr = g.get("expr").and_then(|x| x.as_str()).unwrap_or("").trim();
        if expr.is_empty() {
            AdmitWall::close_into(WALL_PATTERN_GUARD, "structured pattern.guard requires expr", wall);
        } else {
            AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        }
        return;
    }
    AdmitWall::close_into(WALL_PATTERN_GUARD, "pattern.guard must be a string or a structured object", wall);
}

fn parse_scalar(raw: &str, out: &mut Value) {
    let s = raw.trim();
    if s == "true" {
        *out = Value::Bool(true);
        return;
    }
    if s == "false" {
        *out = Value::Bool(false);
        return;
    }
    if s == "null" || s == "~" {
        *out = Value::Null;
        return;
    }
    if let Ok(n) = s.parse::<i64>() {
        *out = Value::Number(n.into());
        return;
    }
    let bytes = s.as_bytes();
    if 1 < s.len() && bytes[0] == 34 && bytes[s.len() - 1] == 34 {
        *out = Value::String(s[1..s.len() - 1].to_string());
        return;
    }
    *out = Value::String(String::from(s));
}

struct YamlLine {
    indent: usize,
    text: String,
}

fn yaml_lines(src: &str, out: &mut Vec<YamlLine>) {
    out.clear();
    for raw in src.lines() {
        let mut indent = 0usize;
        let bytes = raw.as_bytes();
        while indent < bytes.len() && bytes[indent] == 32 {
            indent += 1;
        }
        let text = raw[indent..].trim_end().to_string();
        if text.is_empty() || text.starts_with(char::from(35).to_string().as_str()) {
            continue;
        }
        out.push(YamlLine { indent, text });
    }
}


pub fn compile_guard_wall(doc: &Value, wall: &mut AdmitWall) {
    let p = doc.get("pattern");
    if p.is_none() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    let p = p.unwrap();
    if p.is_string() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    let g = p.get("guard");
    if g.is_none() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    let g = g.unwrap();
    if g.is_string() {
        AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        return;
    }
    if g.is_object() {
        let expr = g.get("expr").and_then(|x| x.as_str()).unwrap_or("").trim();
        if expr.is_empty() {
            AdmitWall::close_into(WALL_PATTERN_GUARD, "structured pattern.guard requires expr", wall);
        } else {
            AdmitWall::open_into(WALL_PATTERN_GUARD, wall);
        }
        return;
    }
    AdmitWall::close_into(WALL_PATTERN_GUARD, "pattern.guard must be a string or a structured object", wall);
}

fn parse_scalar(raw: &str, out: &mut Value) {
    let s = raw.trim();
    if s == "true" {
        *out = Value::Bool(true);
        return;
    }
    if s == "false" {
        *out = Value::Bool(false);
        return;
    }
    if s == "null" || s == "~" {
        *out = Value::Null;
        return;
    }
    if let Ok(n) = s.parse::<i64>() {
        *out = Value::Number(n.into());
        return;
    }
    let bytes = s.as_bytes();
    if 1 < s.len() && bytes[0] == 34 && bytes[s.len() - 1] == 34 {
        *out = Value::String(s[1..s.len() - 1].to_string());
        return;
    }
    *out = Value::String(String::from(s));
}

struct YamlLine {
    indent: usize,
    text: String,
}

fn yaml_lines(src: &str, out: &mut Vec<YamlLine>) {
    out.clear();
    for raw in src.lines() {
        let mut indent = 0usize;
        let bytes = raw.as_bytes();
        while indent < bytes.len() && bytes[indent] == 32 {
            indent += 1;
        }
        let text = raw[indent..].trim_end().to_string();
        if text.is_empty() || text.starts_with(char::from(35).to_string().as_str()) {
            continue;
        }
        out.push(YamlLine { indent, text });
    }
}


fn parse_yaml_block(lines: &[YamlLine], start: usize, parent_indent: usize, out: &mut Value, next_i: &mut usize) {
    if lines.len() <= start {
        *out = Value::Null;
        *next_i = start;
        return;
    }
    let is_list = lines[start].text.starts_with("- ");
    if is_list {
        let mut arr: Vec<Value> = Vec::new();
        let mut i = start;
        while i < lines.len() {
            if lines[i].indent < parent_indent {
                break;
            }
            if lines[i].indent == parent_indent && lines[i].text.starts_with("- ") == false {
                break;
            }
            if lines[i].indent != parent_indent || lines[i].text.starts_with("- ") == false {
                break;
            }
            let item = lines[i].text[2..].trim();
            if item.is_empty() {
                let mut child = Value::Null;
                let mut n = 0usize;
                parse_yaml_block(lines, i + 1, parent_indent + 2, &mut child, &mut n);
                arr.push(child);
                i = n;
                continue;
            }
            if let Some((k, v)) = item.split_once(char::from(58)) {
                let key = k.trim();
                let rest = v.trim();
                let mut map = Map::new();
                if rest.is_empty() {
                    let mut child = Value::Null;
                    let mut n = 0usize;
                    parse_yaml_block(lines, i + 1, lines[i].indent + 2, &mut child, &mut n);
                    map.insert(key.to_string(), child);
                    i = n;
                } else {
                    let mut sc = Value::Null;
                    parse_scalar(rest, &mut sc);
                    map.insert(key.to_string(), sc);
                    i += 1;
                }
                while i < lines.len() && parent_indent < lines[i].indent && lines[i].text.starts_with("- ") == false {
                    if let Some((k2, v2)) = lines[i].text.split_once(char::from(58)) {
                        let key2 = k2.trim();
                        let rest2 = v2.trim();
                        if rest2.is_empty() {
                            let mut child = Value::Null;
                            let mut n = 0usize;
                            parse_yaml_block(lines, i + 1, lines[i].indent + 2, &mut child, &mut n);
                            map.insert(key2.to_string(), child);
                            i = n;
                        } else {
                            let mut sc = Value::Null;
                            parse_scalar(rest2, &mut sc);
                            map.insert(key2.to_string(), sc);
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                arr.push(Value::Object(map));
                continue;
            }
            let mut sc = Value::Null;
            parse_scalar(item, &mut sc);
            arr.push(sc);
            i += 1;
        }
        *out = Value::Array(arr);
        *next_i = i;
        return;
    }
    let mut map = Map::new();
    let mut i = start;
    while i < lines.len() {
        if lines[i].indent < parent_indent {
            break;
        }
        if lines[i].indent == parent_indent && lines[i].text.starts_with("- ") {
            break;
        }
        if lines[i].indent != parent_indent {
            break;
        }
        if let Some((k, v)) = lines[i].text.split_once(char::from(58)) {
            let key = k.trim();
            let rest = v.trim();
            if rest.is_empty() {
                if i + 1 < lines.len() && parent_indent < lines[i + 1].indent {
                    let next_indent = lines[i + 1].indent;
                    let mut child = Value::Null;
                    let mut n = 0usize;
                    parse_yaml_block(lines, i + 1, next_indent, &mut child, &mut n);
                    map.insert(key.to_string(), child);
                    i = n;
                } else {
                    map.insert(key.to_string(), Value::Null);
                    i += 1;
                }
            } else {
                let mut sc = Value::Null;
                parse_scalar(rest, &mut sc);
                map.insert(key.to_string(), sc);
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    *out = Value::Object(map);
    *next_i = i;
}

