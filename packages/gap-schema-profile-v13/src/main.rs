// @PAD: gap-285-p1-schema-profile-v13-cli
// GAP-285-P1 CLI. source= then agent_id= action= wrap= action_path=. Collect-all live Admit.

use gap_schema_profile_v13::{
    live_admit_gap_profile, AdmitResult, LiveAdmitRequest,
};
use std::io::{self, Read, Write};

fn emit(s: &str) {
    let mut o = io::stdout();
    let _ = o.write_all(s.as_bytes());
}

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let mut source = String::new();
    let mut req = LiveAdmitRequest::default();
    for raw in buf.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("#") {
            continue;
        }
        if let Some(v) = line.strip_prefix("source=") {
            source = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("agent_id=") {
            req.agent_id = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("action=") {
            req.action = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("wrap=") {
            req.wrap = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("action_path=") {
            req.action_path = v.to_string();
            continue;
        }
    }
    let mut result = AdmitResult {
        allow: true,
        closed: Vec::new(),
    };
    let mut err = String::new();
    live_admit_gap_profile(&source, &req, &mut result, &mut err);
    if err.is_empty() == false {
        emit("allow=false\nerror=");
        emit(&err);
        emit("\n");
        return;
    }
    emit("allow=");
    if result.allow {
        emit("true\n");
    } else {
        emit("false\n");
    }
    let mut i = 0usize;
    while i < result.closed.len() {
        emit("closed=");
        emit(&result.closed[i].id);
        emit("|");
        emit(&result.closed[i].reason);
        emit("\n");
        i += 1;
    }
}
