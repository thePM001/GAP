// @PAD: gap-285-p9-vendor-readme-one-law
// @GCDE: gaplune.policy.v1
// crate: gap-285-p9-vendor-readme-one-law
// GAP-285-P9 vendor README one-law after schema law.
// Presence of trust_ring is Deny. Who-may is agent_may.
// Creation generate scaffold filled with collect-all scan.

pub const TICKET: &str = "GAP-285-P9";
pub const ONE_LAW: &str = "Presence of trust_ring is Deny. Who-may is agent_may.";
pub const WALL_DOCUMENTARY: &str = "gap:vendor:documentary-rank-ring";
pub const WALL_ONE_LAW: &str = "gap:vendor:one-law-missing";
pub const WALL_WHO_MAY: &str = "gap:vendor:who-may-missing";
pub const WALL_LIVE_PATH: &str = "gap:vendor:live-path-missing";
pub const WALL_GAP_SOURCE: &str = "gap:vendor:gap-source-missing";

pub fn lower(src: &str) -> String {
    src.to_ascii_lowercase()
}

pub fn teaches_documentary_rank_ring(src: &str) -> bool {
    let l = lower(src);
    if l.contains("documentary") && l.contains("trust_ring") {
        return true;
    }
    if l.contains("warns then denies") {
        return true;
    }
    if l.contains("unused at admit") {
        return true;
    }
    false
}

pub fn has_one_law_presence_deny(src: &str) -> bool {
    src.contains("Presence of trust_ring is Deny")
}

pub fn has_who_may_agent_may(src: &str) -> bool {
    let l = lower(src);
    l.contains("who-may is agent_may") || l.contains("who-may is `agent_may`")
}

pub fn names_live_path(src: &str) -> bool {
    let l = lower(src);
    if l.contains("freeze-at-seal") == false {
        return false;
    }
    if l.contains("1000 ms") == false {
        return false;
    }
    if l.contains("collect-all admit") == false {
        return false;
    }
    if l.contains("apply") == false {
        return false;
    }
    if l.contains("sha256-structure") == false {
        return false;
    }
    if l.contains("agent_may") == false {
        return false;
    }
    true
}

pub fn keeps_gap_source(src: &str) -> bool {
    let l = lower(src);
    l.contains(".gap") && l.contains("gap source")
}

fn push_unique(rows: &mut Vec<String>, wall: &str) {
    let mut i = 0usize;
    while i < rows.len() {
        if rows[i] == wall {
            return;
        }
        i += 1;
    }
    rows.push(String::from(wall));
}

pub fn collect_one_law_walls(src: &str) -> Vec<String> {
    let mut closed: Vec<String> = Vec::new();
    if src.trim().is_empty() {
        push_unique(&mut closed, WALL_LIVE_PATH);
        return closed;
    }
    if teaches_documentary_rank_ring(src) {
        push_unique(&mut closed, WALL_DOCUMENTARY);
    }
    if has_one_law_presence_deny(src) == false {
        push_unique(&mut closed, WALL_ONE_LAW);
    }
    if has_who_may_agent_may(src) == false {
        push_unique(&mut closed, WALL_WHO_MAY);
    }
    if names_live_path(src) == false {
        push_unique(&mut closed, WALL_LIVE_PATH);
    }
    if keeps_gap_source(src) == false {
        push_unique(&mut closed, WALL_GAP_SOURCE);
    }
    closed
}

pub fn scan_one_law_text(src: &str) -> Result<String, String> {
    let closed = collect_one_law_walls(src);
    if closed.is_empty() {
        return Ok(String::from("ok vendor one-law after schema law"));
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

pub mod documentary_rank_ring {
    pub struct DocumentaryRankRing {
        pub src: String,
        pub teaches: bool,
    }
    impl DocumentaryRankRing {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                teaches: false,
            }
        }
        /// Detect vendor README or FILE-FORMAT text that teaches trust_ring as a documentary label or rank use that warns then denies
        pub fn process(&mut self) {
            self.teaches = super::teaches_documentary_rank_ring(&self.src);
        }
    }
}

pub mod one_law_presence_deny {
    pub struct OneLawPresenceDeny {
        pub src: String,
        pub has_one_law: bool,
    }
    impl OneLawPresenceDeny {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                has_one_law: false,
            }
        }
        /// Require the one-law sentence Presence of trust_ring is Deny and Who-may is agent_may
        pub fn process(&mut self) {
            self.has_one_law = super::has_one_law_presence_deny(&self.src)
                && super::has_who_may_agent_may(&self.src);
        }
    }
}

pub mod live_path_named {
    pub struct LivePathNamed {
        pub src: String,
        pub named: bool,
    }
    impl LivePathNamed {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                named: false,
            }
        }
        /// Require seal freeze-at-seal 1000 ms collect-all Admit Apply agent_may and sha256-structure
        pub fn process(&mut self) {
            self.named = super::names_live_path(&self.src);
        }
    }
}

pub mod gap_source_ext {
    pub struct GapSourceExt {
        pub src: String,
        pub keeps_gap: bool,
    }
    impl GapSourceExt {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                keeps_gap: false,
            }
        }
        /// Require Keep .gap as GAP source
        pub fn process(&mut self) {
            self.keeps_gap = super::keeps_gap_source(&self.src);
        }
    }
}

pub mod scan_vendor_readme_one_law {
    pub struct ScanVendorReadmeOneLaw {
        pub readme: String,
        pub result: String,
        pub allow: bool,
    }
    impl ScanVendorReadmeOneLaw {
        pub fn new() -> Self {
            Self {
                readme: String::new(),
                result: String::new(),
                allow: false,
            }
        }
        /// Collect-all vendor README one-law after schema law. Write DENY on miss.
        pub fn process(&mut self) {
            match super::scan_one_law_text(&self.readme) {
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

pub mod scan_file_format_one_law {
    pub struct ScanFileFormatOneLaw {
        pub file_format: String,
        pub result: String,
        pub allow: bool,
    }
    impl ScanFileFormatOneLaw {
        pub fn new() -> Self {
            Self {
                file_format: String::new(),
                result: String::new(),
                allow: false,
            }
        }
        /// Collect-all FILE-FORMAT one-law after schema law. Write DENY on miss.
        pub fn process(&mut self) {
            match super::scan_one_law_text(&self.file_format) {
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

    fn hole_doc() -> String {
        String::from(
            "trust_ring is a documentary label on classic v1 and v1.2. Rank use warns then denies.\n",
        )
    }

    fn honest_doc() -> String {
        String::from(
            "Keep `.gap` as GAP source.\nLive evaluation uses freeze-at-seal and waits 1000 ms then collect-all Admit then Apply.\nLive AEP 2.8.5 EPSCOM trust bundle mode is sha256-structure.\nPresence of trust_ring is Deny.\nWho-may is agent_may.\nDo not set trust_ring on live documents.\n",
        )
    }

    #[test]
    fn documentary_text_denied() {
        if teaches_documentary_rank_ring(&hole_doc()) == false {
            fail();
        }
        match scan_one_law_text(&hole_doc()) {
            Ok(_) => fail(),
            Err(e) => {
                if e.contains(WALL_DOCUMENTARY) == false {
                    fail();
                }
            }
        }
    }

    #[test]
    fn honest_text_allowed() {
        if teaches_documentary_rank_ring(&honest_doc()) {
            fail();
        }
        match scan_one_law_text(&honest_doc()) {
            Ok(v) => {
                if v.contains("ok vendor one-law") == false {
                    fail();
                }
            }
            Err(_) => fail(),
        }
    }

    #[test]
    fn readme_fixture_one_law() {
        let src = include_str ! ("../fixtures/README.md");
        match scan_one_law_text(src) {
            Ok(_) => {}
            Err(_) => fail(),
        }
        if src.contains("documentary") {
            fail();
        }
        if src.contains("warns then denies") {
            fail();
        }
        if src.contains("Presence of trust_ring is Deny") == false {
            fail();
        }
        if src.contains("sha256-structure") == false {
            fail();
        }
    }

    #[test]
    fn file_format_fixture_one_law() {
        let src = include_str ! ("../fixtures/FILE-FORMAT.md");
        match scan_one_law_text(src) {
            Ok(_) => {}
            Err(_) => fail(),
        }
        if src.contains("documentary") {
            fail();
        }
        if src.contains("Presence of trust_ring is Deny") == false {
            fail();
        }
        if keeps_gap_source(src) == false {
            fail();
        }
    }

    #[test]
    fn wrappers_run() {
        let mut d = documentary_rank_ring::DocumentaryRankRing::new();
        d.src = hole_doc();
        d.process();
        if d.teaches == false {
            fail();
        }
        let mut s = scan_vendor_readme_one_law::ScanVendorReadmeOneLaw::new();
        s.readme = honest_doc();
        s.process();
        if s.allow == false {
            fail();
        }
        let mut f = scan_file_format_one_law::ScanFileFormatOneLaw::new();
        f.file_format = include_str ! ("../fixtures/FILE-FORMAT.md").to_string();
        f.process();
        if f.allow == false {
            fail();
        }
    }
}
