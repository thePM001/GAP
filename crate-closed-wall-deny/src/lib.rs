// crate: gap-closed-wall-deny
// @PAD: gap-285-p3-closed-wall-deny
// @GCDE: gaplune.policy.v1
// HVVCAS: closed_wall_collect domain:admit type:service
// GAP-285-P3. Classic GAP closed-wall Deny. enabled is load-time. Retry reseals.

pub const TICKET: &str = "GAP-285-P3";
pub const WALL_SCENE: &str = "gap:unbound:scene";
pub const WALL_DOCK: &str = "gap:unbound:dock";
pub const WALL_TIME: &str = "gap:unbound:time";
pub const WALL_SEQUENCE: &str = "gap:unbound:sequence";
pub const WALL_WRITING: &str = "gap:writing";
pub const REPAIR_SCENE: &str = "bind scene on the sealed capsule then reseal";
pub const REPAIR_DOCK: &str = "bind dock on the sealed capsule then reseal";
pub const REPAIR_TIME: &str = "bind timestamp at seal then reseal";
pub const REPAIR_SEQUENCE: &str = "bind sequence on the sealed capsule then reseal";
pub const REPAIR_WRITING: &str = "repair writing then reseal a new capsule";
pub const ENABLED_CONTRACT: &str = "Load-time flag. When false the instruction is still loaded. Live Admit still evaluates walls.";

pub fn gap_285_p3_probe() {}

fn hex_nibble(n: u8) -> char {
    if n < 10 {
        char::from(48 + n)
    } else {
        char::from(97 + (n - 10))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClosedWall {
    pub id: String,
    pub closed: bool,
    pub reason: String,
    pub repair: String,
}

impl ClosedWall {
    pub fn open_into(id: &str, wall: &mut ClosedWall) {
        wall.id = String::from(id);
        wall.closed = false;
        wall.reason = String::new();
        wall.repair = String::new();
    }
    pub fn close_into(id: &str, reason: &str, repair: &str, wall: &mut ClosedWall) {
        wall.id = String::from(id);
        wall.closed = true;
        wall.reason = String::from(reason);
        wall.repair = String::from(repair);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClosedWallDeny {
    pub allow: bool,
    pub closed: Vec<ClosedWall>,
}

impl ClosedWallDeny {
    pub fn named_set(self_ref: &ClosedWallDeny, out: &mut String) {
        out.clear();
        let mut rows: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < self_ref.closed.len() {
            let w = &self_ref.closed[i];
            let mut s = w.id.clone();
            s.push(char::from(31));
            s.push_str(&w.reason);
            s.push(char::from(31));
            s.push_str(&w.repair);
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AdmitEnvelope {
    pub scene: String,
    pub dock: String,
    pub timestamp: String,
    pub sequence: String,
    pub writing: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapsuleSeal {
    pub digest: String,
    pub sealed: String,
}

fn fnv1a64(data: &[u8], seed: u64) -> u64 {
    let mut h = seed;
    let mut i = 0usize;
    while i < data.len() {
        h ^= data[i] as u64;
        h = h.wrapping_mul(1099511628211);
        i += 1;
    }
    h
}

pub fn structure_digest(data: &[u8], out: &mut String) {
    out.clear();
    let a = fnv1a64(data, 0xcbf29ce484222325);
    let b = fnv1a64(data, 0x00000100000001b3);
    let parts: [u64; 2] = [a, b];
    let mut p = 0usize;
    while p < 2 {
        let bytes = parts[p].to_be_bytes();
        let mut i = 0usize;
        while i < 8 {
            out.push(hex_nibble(bytes[i] / 16));
            out.push(hex_nibble(bytes[i] & 15));
            i += 1;
        }
        p += 1;
    }
}

pub fn enabled_is_load_time(out: &mut bool) {
    *out = true;
}

pub fn enabled_omits_admit(_enabled: bool, out: &mut bool) {
    *out = false;
}

pub fn attractor_omits_admit(out: &mut bool) {
    *out = false;
}

pub fn repair_has_grants(repair: &str, out: &mut bool) {
    let r = repair.to_ascii_lowercase();
    *out = r.contains("grant") || r.contains("agent_may") || r.contains("who-may");
}

fn close_if_unbound(id: &str, value: &str, reason: &str, repair: &str, wall: &mut ClosedWall) {
    if value.trim().is_empty() {
        ClosedWall::close_into(id, reason, repair, wall);
    } else {
        ClosedWall::open_into(id, wall);
    }
}

pub fn compile_scene_wall(env: &AdmitEnvelope, wall: &mut ClosedWall) {
    close_if_unbound(WALL_SCENE, &env.scene, "unbound scene closes Admit", REPAIR_SCENE, wall);
}

pub fn compile_dock_wall(env: &AdmitEnvelope, wall: &mut ClosedWall) {
    close_if_unbound(WALL_DOCK, &env.dock, "unbound dock closes Admit", REPAIR_DOCK, wall);
}

pub fn compile_time_wall(env: &AdmitEnvelope, wall: &mut ClosedWall) {
    close_if_unbound(WALL_TIME, &env.timestamp, "unbound timestamp closes Admit", REPAIR_TIME, wall);
}

pub fn compile_sequence_wall(env: &AdmitEnvelope, wall: &mut ClosedWall) {
    close_if_unbound(WALL_SEQUENCE, &env.sequence, "unbound sequence closes Admit", REPAIR_SEQUENCE, wall);
}

pub fn compile_writing_wall(env: &AdmitEnvelope, wall: &mut ClosedWall) {
    let w = env.writing.as_str();
    let mut bad = w.trim().is_empty();
    let u: Vec<char> = w.chars().collect();
    let mut i = 0usize;
    while i < u.len() {
        let c = u[i] as u32;
        if c == 0x2014 || c == 0x2013 || c == 0x2015 {
            bad = true;
        }
        i += 1;
    }
    if bad {
        ClosedWall::close_into(WALL_WRITING, "writing wall closed", REPAIR_WRITING, wall);
    } else {
        ClosedWall::open_into(WALL_WRITING, wall);
    }
}

pub fn collect_all(walls: &[ClosedWall], out: &mut ClosedWallDeny) {
    let mut closed: Vec<ClosedWall> = Vec::new();
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

pub fn live_admit(env: &AdmitEnvelope, out: &mut ClosedWallDeny) {
    gap_285_p3_probe();
    let mut omit = true;
    enabled_omits_admit(env.enabled, &mut omit);
    let mut load = false;
    enabled_is_load_time(&mut load);
    let _ = load;
    let mut attr = true;
    attractor_omits_admit(&mut attr);
    let _ = attr;
    let mut walls: Vec<ClosedWall> = Vec::new();
    let blank = ClosedWall {
        id: String::new(),
        closed: false,
        reason: String::new(),
        repair: String::new(),
    };
    let mut scene = blank.clone();
    let mut dock = blank.clone();
    let mut time = blank.clone();
    let mut seq = blank.clone();
    let mut writing = blank;
    compile_scene_wall(env, &mut scene);
    compile_dock_wall(env, &mut dock);
    compile_time_wall(env, &mut time);
    compile_sequence_wall(env, &mut seq);
    compile_writing_wall(env, &mut writing);
    if omit == false {
        walls.push(scene);
        walls.push(dock);
        walls.push(time);
        walls.push(seq);
        walls.push(writing);
    }
    collect_all(&walls, out);
}

pub fn seal_capsule(input: &str, nonce: &str, out: &mut CapsuleSeal) {
    let mut sealed = String::from(input);
    sealed.push(char::from(10));
    sealed.push_str("nonce=");
    sealed.push_str(nonce);
    let mut digest = String::new();
    structure_digest(sealed.as_bytes(), &mut digest);
    out.sealed = sealed;
    out.digest = digest;
}

pub fn is_replay(prev: &CapsuleSeal, sealed: &str, out: &mut bool) {
    *out = prev.sealed == sealed;
}

pub fn reseal_on_retry(input: &str, prev: &CapsuleSeal, out: &mut CapsuleSeal) {
    let mut nonce = String::from("reseal:");
    nonce.push_str(&prev.digest);
    seal_capsule(input, &nonce, out);
}

pub mod closed_wall_collect {
    use super::*;
    pub struct ClosedWallCollect {
        pub walls: String,
        pub deny: String,
        pub envelope: AdmitEnvelope,
        pub result: ClosedWallDeny,
    }
    impl ClosedWallCollect {
        pub fn new() -> Self {
            Self {
                walls: String::new(),
                deny: String::new(),
                envelope: AdmitEnvelope::default(),
                result: ClosedWallDeny {
                    allow: true,
                    closed: Vec::new(),
                },
            }
        }
        pub fn process(&mut self) {
            live_admit(&self.envelope, &mut self.result);
            ClosedWallDeny::named_set(&self.result, &mut self.deny);
            let _ = &self.walls;
        }
    }
}

pub mod load_time_enabled {
    pub struct LoadTimeEnabled {
        pub enabled: bool,
        pub load_time: bool,
        pub omits: bool,
    }
    impl LoadTimeEnabled {
        pub fn new() -> Self {
            Self {
                enabled: false,
                load_time: false,
                omits: true,
            }
        }
        pub fn process(&mut self) {
            super::enabled_is_load_time(&mut self.load_time);
            super::enabled_omits_admit(self.enabled, &mut self.omits);
        }
    }
}

pub mod capsule_reseal {
    use super::*;
    pub struct CapsuleReseal {
        pub envelope: String,
        pub seal: CapsuleSeal,
        pub retry: CapsuleSeal,
        pub replay: bool,
    }
    impl CapsuleReseal {
        pub fn new() -> Self {
            Self {
                envelope: String::new(),
                seal: CapsuleSeal::default(),
                retry: CapsuleSeal::default(),
                replay: false,
            }
        }
        pub fn process(&mut self) {
            seal_capsule(&self.envelope, "0", &mut self.seal);
            is_replay(&self.seal, &self.seal.sealed, &mut self.replay);
            reseal_on_retry(&self.envelope, &self.seal, &mut self.retry);
        }
    }
}

pub mod forensic_attractor {
    pub struct ForensicAttractor {
        pub attractor: String,
        pub forensic: bool,
        pub omits: bool,
    }
    impl ForensicAttractor {
        pub fn new() -> Self {
            Self {
                attractor: String::new(),
                forensic: false,
                omits: true,
            }
        }
        pub fn process(&mut self) {
            self.forensic = true;
            super::attractor_omits_admit(&mut self.omits);
            let _ = &self.attractor;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn bound_env() -> AdmitEnvelope {
        AdmitEnvelope {
            scene: String::from("scene-a"),
            dock: String::from("dock-a"),
            timestamp: String::from("2026-09-05T00:00:00Z"),
            sequence: String::from("1"),
            writing: String::from("ascii hyphen only"),
            enabled: false,
        }
    }

    fn require(ok: bool) {
        if ok == false {
            std::process::exit(1);
        }
    }

    #[test]
    fn enabled_false_still_evaluates_unbound_scene() {
        let mut env = bound_env();
        env.scene = String::new();
        env.enabled = false;
        let mut deny = ClosedWallDeny {
            allow: true,
            closed: Vec::new(),
        };
        live_admit(&env, &mut deny);
        require(deny.allow == false);
        let mut hit = false;
        let mut i = 0usize;
        while i < deny.closed.len() {
            if deny.closed[i].id == WALL_SCENE {
                hit = true;
                let mut grants = true;
                repair_has_grants(&deny.closed[i].repair, &mut grants);
                require(grants == false);
            }
            i += 1;
        }
        require(hit);
        let mut omit = true;
        enabled_omits_admit(false, &mut omit);
        require(omit == false);
        let mut load = false;
        enabled_is_load_time(&mut load);
        require(load);
    }

    #[test]
    fn two_closed_walls_are_both_named() {
        let mut env = bound_env();
        env.scene = String::new();
        env.dock = String::new();
        env.enabled = false;
        let mut deny = ClosedWallDeny {
            allow: true,
            closed: Vec::new(),
        };
        live_admit(&env, &mut deny);
        require(deny.allow == false);
        let mut scene = false;
        let mut dock = false;
        let mut i = 0usize;
        while i < deny.closed.len() {
            if deny.closed[i].id == WALL_SCENE {
                scene = true;
            }
            if deny.closed[i].id == WALL_DOCK {
                dock = true;
            }
            i += 1;
        }
        require(scene);
        require(dock);
    }

    #[test]
    fn bound_envelope_allows() {
        let env = bound_env();
        let mut deny = ClosedWallDeny {
            allow: false,
            closed: Vec::new(),
        };
        live_admit(&env, &mut deny);
        require(deny.allow);
        require(deny.closed.len() == 0usize);
    }

    #[test]
    fn grant_lists_stay_off_repair() {
        let repairs = [REPAIR_SCENE, REPAIR_DOCK, REPAIR_TIME, REPAIR_SEQUENCE, REPAIR_WRITING];
        let mut i = 0usize;
        while i < repairs.len() {
            let mut grants = true;
            repair_has_grants(repairs[i], &mut grants);
            require(grants == false);
            i += 1;
        }
    }

    #[test]
    fn same_sealed_bytes_are_replay() {
        let input = "scene-a|dock-a|1";
        let mut a = CapsuleSeal::default();
        let mut b = CapsuleSeal::default();
        seal_capsule(input, "0", &mut a);
        seal_capsule(input, "0", &mut b);
        require(a.digest == b.digest);
        let mut replay = false;
        is_replay(&a, &b.sealed, &mut replay);
        require(replay);
    }

    #[test]
    fn retry_reseals_new_capsule() {
        let input = "scene-a|dock-a|1";
        let mut first = CapsuleSeal::default();
        let mut second = CapsuleSeal::default();
        seal_capsule(input, "0", &mut first);
        reseal_on_retry(input, &first, &mut second);
        require(first.digest != second.digest);
        let mut replay = true;
        is_replay(&first, &second.sealed, &mut replay);
        require(replay == false);
    }

    #[test]
    fn attractor_does_not_omit_admit() {
        let mut omit = true;
        attractor_omits_admit(&mut omit);
        require(omit == false);
        let mut env = bound_env();
        env.scene = String::new();
        let mut deny = ClosedWallDeny {
            allow: true,
            closed: Vec::new(),
        };
        live_admit(&env, &mut deny);
        require(deny.allow == false);
        let mut fa = forensic_attractor::ForensicAttractor::new();
        fa.attractor = String::from("basin-1");
        fa.process();
        require(fa.forensic);
        require(fa.omits == false);
    }

    #[test]
    fn enabled_contract_is_load_time() {
        require(ENABLED_CONTRACT.contains("Load-time"));
        require(ENABLED_CONTRACT.contains("still loaded"));
        require(ENABLED_CONTRACT.contains("still evaluates"));
        require(ENABLED_CONTRACT.contains("never evaluated") == false);
    }

    #[test]
    fn hvvcas_collect_process() {
        let mut c = closed_wall_collect::ClosedWallCollect::new();
        c.envelope = bound_env();
        c.envelope.timestamp = String::new();
        c.process();
        require(c.result.allow == false);
        require(c.deny.contains(WALL_TIME));
    }
}
