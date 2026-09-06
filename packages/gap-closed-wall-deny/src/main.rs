// @PAD: gap-285-p3-closed-wall-deny-cli
// @GCDE: gaplune.policy.v1
// GAP-285-P3 CLI. scene= dock= timestamp= sequence= writing= enabled= input=
// Collect-all closed-wall Deny. Retry reseals.

use gap_closed_wall_deny::{
    is_replay, live_admit, reseal_on_retry, seal_capsule, AdmitEnvelope, CapsuleSeal, ClosedWallDeny,
};
use std::io::{self, Read, Write};

fn emit(s: &str) {
    let mut o = io::stdout();
    let _ = o.write_all(s.as_bytes());
}

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let mut env = AdmitEnvelope {
        scene: String::new(),
        dock: String::new(),
        timestamp: String::new(),
        sequence: String::new(),
        writing: String::new(),
        enabled: true,
    };
    let mut input = String::new();
    for raw in buf.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("#") {
            continue;
        }
        if let Some(v) = line.strip_prefix("scene=") {
            env.scene = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("dock=") {
            env.dock = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("timestamp=") {
            env.timestamp = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("sequence=") {
            env.sequence = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("writing=") {
            env.writing = v.to_string();
            continue;
        }
        if let Some(v) = line.strip_prefix("enabled=") {
            env.enabled = v != "false";
            continue;
        }
        if let Some(v) = line.strip_prefix("input=") {
            input = v.to_string();
            continue;
        }
    }
    let mut result = ClosedWallDeny {
        allow: true,
        closed: Vec::new(),
    };
    live_admit(&env, &mut result);
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
        emit("|");
        emit(&result.closed[i].repair);
        emit("\n");
        i += 1;
    }
    if input.is_empty() == false {
        let mut first = CapsuleSeal::default();
        let mut second = CapsuleSeal::default();
        seal_capsule(&input, "0", &mut first);
        reseal_on_retry(&input, &first, &mut second);
        let mut replay = false;
        is_replay(&first, &first.sealed, &mut replay);
        emit("digest=");
        emit(&first.digest);
        emit("\n");
        emit("retry_digest=");
        emit(&second.digest);
        emit("\n");
        emit("replay=");
        if replay {
            emit("true\n");
        } else {
            emit("false\n");
        }
        emit("resealed=");
        if first.digest == second.digest {
            emit("false\n");
        } else {
            emit("true\n");
        }
    }
}
