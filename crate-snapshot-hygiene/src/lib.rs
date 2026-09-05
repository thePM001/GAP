// crate: gap-snapshot-hygiene
// @PAD: gaplune-creation-pad emit ( zero-LLM )
// @GCDE: gaplune.policy.v1
// HVVCAS: scan_readme_snapshot_hygiene domain:admit type:service
// GAP-285-P2. Classic GAP OSS snapshot hygiene.
// README must name shipped crate binaries and must not list a structural CLI.

pub const TICKET: &str = "GAP-285-P2";
pub const WALL_STRUCTURAL_CLI: &str = "gap:snapshot:structural-cli";
pub const WALL_MISSING_BIN: &str = "gap:snapshot:missing-bin";
pub const WALL_MISSING_CARGO_RUN: &str = "gap:snapshot:missing-cargo-run";
pub const WALL_EMPTY: &str = "gap:snapshot:empty-readme";
pub const BIN_SCHEMA_PROFILE: &str = "gap-schema-profile-v13";
pub const BIN_CLOSED_WALL: &str = "gap-closed-wall-deny";
pub const BIN_PROOF_BUNDLE: &str = "gap-proof-live-bundle-mode";
pub const BIN_SNAPSHOT: &str = "gap-snapshot-hygiene";
pub const CARGO_SCHEMA_PROFILE: &str =
    "cargo run --manifest-path crate/Cargo.toml --bin gap-schema-profile-v13";
pub const CARGO_CLOSED_WALL: &str =
    "cargo run --manifest-path crate-closed-wall-deny/Cargo.toml --bin gap-closed-wall-deny";
pub const CARGO_PROOF_BUNDLE: &str =
    "cargo run --manifest-path crate-proof-live-bundle/Cargo.toml --bin gap-proof-live-bundle-mode";
pub const CARGO_SNAPSHOT: &str =
    "cargo run --manifest-path crate-snapshot-hygiene/Cargo.toml --bin gap-snapshot-hygiene";

const STRUCTURAL_CLI_VERBS: [&str; 10] = [
    "compile",
    "lint",
    "graph",
    "run",
    "test",
    "export",
    "subprotocol",
    "validator",
    "adapter",
    "commands",
];

pub fn required_shipped_bins() -> [&'static str; 4] {
    [
        BIN_SCHEMA_PROFILE,
        BIN_CLOSED_WALL,
        BIN_PROOF_BUNDLE,
        BIN_SNAPSHOT,
    ]
}

pub fn required_cargo_run_lines() -> [&'static str; 4] {
    [
        CARGO_SCHEMA_PROFILE,
        CARGO_CLOSED_WALL,
        CARGO_PROOF_BUNDLE,
        CARGO_SNAPSHOT,
    ]
}

fn strip_line_noise(line: &str) -> &str {
    let mut t = line.trim();
    loop {
        let next = t.trim_start_matches('`').trim_start_matches('$').trim();
        if next.len() == t.len() {
            return t;
        }
        t = next;
    }
}

pub fn is_structural_cli_line(line: &str) -> bool {
    let t = strip_line_noise(line);
    let lower = t.to_ascii_lowercase();
    if lower.starts_with("structural ") == false {
        return false;
    }
    let rest = &lower["structural ".len()..];
    let mut i = 0usize;
    while i < STRUCTURAL_CLI_VERBS.len() {
        let v = STRUCTURAL_CLI_VERBS[i];
        if rest.starts_with(v) {
            if rest.len() == v.len() {
                return true;
            }
            let c = rest.as_bytes()[v.len()];
            if c == b' ' || c == b'\t' || c == b'<' || c == b'#' || c == b'`' {
                return true;
            }
        }
        i += 1;
    }
    false
}

pub fn names_bin(src: &str, bin: &str) -> bool {
    src.contains(bin)
}

pub fn names_cargo_run(src: &str, line: &str) -> bool {
    src.contains(line)
}

fn push_unique(rows: &mut Vec<String>, wall: String) {
    let mut i = 0usize;
    while i < rows.len() {
        if rows[i] == wall {
            return;
        }
        i += 1;
    }
    rows.push(wall);
}

pub fn scan_readme_snapshot_hygiene(src: &str) -> Result<String, String> {
    if src.trim().is_empty() {
        return Err(String::from(WALL_EMPTY));
    }
    let mut closed: Vec<String> = Vec::new();
    for line in src.lines() {
        if is_structural_cli_line(line) {
            push_unique(&mut closed, String::from(WALL_STRUCTURAL_CLI));
        }
    }
    let bins = required_shipped_bins();
    let mut b = 0usize;
    while b < bins.len() {
        if names_bin(src, bins[b]) == false {
            let mut wall = String::from(WALL_MISSING_BIN);
            wall.push(':');
            wall.push_str(bins[b]);
            push_unique(&mut closed, wall);
        }
        b += 1;
    }
    let cargo = required_cargo_run_lines();
    let mut c = 0usize;
    while c < cargo.len() {
        if names_cargo_run(src, cargo[c]) == false {
            let mut wall = String::from(WALL_MISSING_CARGO_RUN);
            wall.push(':');
            wall.push_str(cargo[c]);
            push_unique(&mut closed, wall);
        }
        c += 1;
    }
    if closed.is_empty() {
        return Ok(String::from(
            "ok snapshot hygiene shipped crate binaries",
        ));
    }
    let mut out = String::new();
    let mut i = 0usize;
    while i < closed.len() {
        if 0 < i {
            out.push(char::from(10));
        }
        out.push_str(&closed[i]);
        i += 1;
    }
    Err(out)
}

pub fn closed_walls(err: &str) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    for line in err.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        rows.push(String::from(t));
    }
    rows
}

pub mod structural_cli_line {
    pub struct StructuralCliLine {
        pub line: String,
        pub is_cli: bool,
    }
    impl StructuralCliLine {
        pub fn new() -> Self {
            Self {
                line: String::new(),
                is_cli: false,
            }
        }
        pub fn process(&mut self) {
            self.is_cli = super::is_structural_cli_line(&self.line);
        }
    }
}

pub mod required_shipped_bins {
    pub struct RequiredShippedBins {
        pub unused: String,
        pub bins: String,
    }
    impl RequiredShippedBins {
        pub fn new() -> Self {
            Self {
                unused: String::new(),
                bins: String::new(),
            }
        }
        pub fn process(&mut self) {
            self.bins.clear();
            let src = super::required_shipped_bins();
            let mut i = 0usize;
            while i < src.len() {
                if 0 < i {
                    self.bins.push(char::from(10));
                }
                self.bins.push_str(src[i]);
                i += 1;
            }
        }
    }
}

pub mod required_cargo_run_lines {
    pub struct RequiredCargoRunLines {
        pub unused: String,
        pub lines: String,
    }
    impl RequiredCargoRunLines {
        pub fn new() -> Self {
            Self {
                unused: String::new(),
                lines: String::new(),
            }
        }
        pub fn process(&mut self) {
            self.lines.clear();
            let src = super::required_cargo_run_lines();
            let mut i = 0usize;
            while i < src.len() {
                if 0 < i {
                    self.lines.push(char::from(10));
                }
                self.lines.push_str(src[i]);
                i += 1;
            }
        }
    }
}

pub mod scan_readme_snapshot_hygiene {
    pub struct ScanReadmeSnapshotHygiene {
        pub readme: String,
        pub result: String,
        pub allow: bool,
    }
    impl ScanReadmeSnapshotHygiene {
        pub fn new() -> Self {
            Self {
                readme: String::new(),
                result: String::new(),
                allow: false,
            }
        }

        /// Collect-all snapshot hygiene. Closed walls name missing binaries and structural CLI commands. Write DENY on miss.
        pub fn process(&mut self) {
            match super::scan_readme_snapshot_hygiene(&self.readme) {
                Ok(v) => {
                    self.result = v;
                    self.allow = true;
                }
                Err(e) => {
                    self.result = e;
                    self.allow = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fail() {
        let none: Option<i32> = None;
        let _ = none.unwrap();
    }

    fn hole_readme() -> String {
        String::from(
            "## GAP commands\n\n```\nstructural compile <file.gap>\nstructural run <file.gap>\n```\n",
        )
    }

    fn honest_readme() -> String {
        let mut s = String::from(
            "This snapshot ships crate binaries gap-schema-profile-v13, gap-closed-wall-deny, gap-proof-live-bundle-mode and gap-snapshot-hygiene.\nThis snapshot does not ship a structural CLI.\n",
        );
        s.push_str(CARGO_SCHEMA_PROFILE);
        s.push(char::from(10));
        s.push_str(CARGO_CLOSED_WALL);
        s.push(char::from(10));
        s.push_str(CARGO_PROOF_BUNDLE);
        s.push(char::from(10));
        s.push_str(CARGO_SNAPSHOT);
        s.push(char::from(10));
        s
    }

    #[test]
    fn structural_cli_lines_denied() {
        if is_structural_cli_line("structural compile <file.gap>") == false {
            fail();
        }
        if is_structural_cli_line("structural run foo.gap") == false {
            fail();
        }
        if is_structural_cli_line("structural lint") == false {
            fail();
        }
        if is_structural_cli_line("GAP is a complete structural programming language") {
            fail();
        }
        if is_structural_cli_line("Structural validation runs at load.") {
            fail();
        }
        if is_structural_cli_line("This snapshot does not ship a structural CLI") {
            fail();
        }
    }

    #[test]
    fn hole_readme_closed_on_structural_cli() {
        match scan_readme_snapshot_hygiene(&hole_readme()) {
            Ok(_) => fail(),
            Err(e) => {
                if e.contains(WALL_STRUCTURAL_CLI) == false {
                    fail();
                }
                if e.contains(WALL_MISSING_BIN) == false {
                    fail();
                }
            }
        }
    }

    #[test]
    fn honest_readme_allowed() {
        match scan_readme_snapshot_hygiene(&honest_readme()) {
            Ok(v) => {
                if v.contains("ok snapshot hygiene") == false {
                    fail();
                }
            }
            Err(_) => fail(),
        }
    }

    #[test]
    fn missing_bin_named() {
        let src = String::from(
            "gap-schema-profile-v13 gap-closed-wall-deny gap-proof-live-bundle-mode\ncargo run --manifest-path crate/Cargo.toml --bin gap-schema-profile-v13\ncargo run --manifest-path crate-closed-wall-deny/Cargo.toml --bin gap-closed-wall-deny\ncargo run --manifest-path crate-proof-live-bundle/Cargo.toml --bin gap-proof-live-bundle-mode\ncargo run --manifest-path crate-snapshot-hygiene/Cargo.toml --bin gap-snapshot-hygiene\n",
        );
        match scan_readme_snapshot_hygiene(&src) {
            Ok(_) => fail(),
            Err(e) => {
                if e.contains("gap:snapshot:missing-bin:gap-snapshot-hygiene") == false {
                    fail();
                }
            }
        }
    }

    #[test]
    fn wrappers_run() {
        let mut s = scan_readme_snapshot_hygiene::ScanReadmeSnapshotHygiene::new();
        s.readme = honest_readme();
        s.process();
        if s.allow == false {
            fail();
        }
        s.readme = hole_readme();
        s.process();
        if s.allow {
            fail();
        }
        let mut c = structural_cli_line::StructuralCliLine::new();
        c.line = String::from("structural compile x");
        c.process();
        if c.is_cli == false {
            fail();
        }
    }
}
