// @PAD: gap-285-p14-readme-tree
// @GCDE: gaplune.policy.v1
// package: gap-readme-tree
// GAP-285-P14 GAP README tree truth. Private locators are Deny.

pub const TICKET: &str = "GAP-285-P14";
pub const WALL_VENDOR_FOLDER: &str = "gap:readme:vendor-folder-as-gap";
pub const WALL_ROOT_SCHEMA: &str = "gap:readme:missing-root-schema";
pub const WALL_VENDOR_URL: &str = "gap:readme:missing-vendor-path";
pub const WALL_PRIVATE_LOCATOR: &str = "gap:readme:private-locator";
pub const WALL_HELPER: &str = "gap:readme:missing-helper-package";
pub const WALL_EMPTY: &str = "gap:readme:empty";

pub const SCHEMA_V1: &str = "GAP meta schema v1.json";
pub const SCHEMA_V12: &str = "GAP meta schema v1.2.json";
pub const SCHEMA_V13: &str = "GAP meta schema v1.3.json";
pub const HELPER_SENTENCE: &str = "Product packages live under packages";
pub const HELPER_PROFILE: &str = "`packages/gap-schema-profile-v13/`";
pub const HELPER_CLOSED: &str = "`packages/gap-closed-wall-deny/`";
pub const HELPER_PROOF: &str = "`packages/gap-proof-live-bundle-mode/`";
pub const HELPER_SNAPSHOT: &str = "`packages/gap-snapshot-hygiene/`";
pub const VENDOR_HOST: &str = "NLA-AEP-v2.8-open-source";
pub const COMPILE_PATH: &str = "AEP-Components/gap/lib/gap-compile.mjs";
pub const REFERENCE_PATH: &str = "AEP-Components/gap/policies/reference/";
pub const CODING_GOV_PATH: &str = "AEP-NOSHIP/AEP-Subprotocols/coding-governance/";
pub const FILE_FORMAT_PATH: &str = "AEP-Components/gap/FILE-FORMAT.md";
pub const PRIVATE_PORT: &str = ":3003/";

const VENDOR_LOCAL: [&str; 8] = [
    "`schemas/`",
    "`policies/reference/`",
    "`lib/gap-compile.mjs`",
    "`AEP-Subprotocols/coding-governance/`",
    "`AEP-Subprotocols/`",
    "node lib/gap-compile.mjs",
    "`FILE-FORMAT.md`",
    "schemas/gap-meta-schema",
];

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

pub fn line_vendor_local(line: &str) -> bool {
    if line.contains(VENDOR_HOST) {
        return false;
    }
    let mut i = 0usize;
    while i < VENDOR_LOCAL.len() {
        if line.contains(VENDOR_LOCAL[i]) {
            return true;
        }
        i += 1;
    }
    false
}

pub fn vendor_as_gap(src: &str) -> bool {
    for line in src.lines() {
        if line_vendor_local(line) {
            return true;
        }
    }
    false
}

pub fn names_root_schemas(src: &str) -> bool {
    src.contains(SCHEMA_V1) && src.contains(SCHEMA_V12) && src.contains(SCHEMA_V13)
}

pub fn names_helper_packages(src: &str) -> bool {
    src.contains(HELPER_SENTENCE)
        && src.contains(HELPER_PROFILE)
        && src.contains(HELPER_CLOSED)
        && src.contains(HELPER_PROOF)
        && src.contains(HELPER_SNAPSHOT)
}

pub fn names_vendor_paths(src: &str) -> bool {
    src.contains(VENDOR_HOST)
        && src.contains(COMPILE_PATH)
        && src.contains(REFERENCE_PATH)
        && src.contains(CODING_GOV_PATH)
        && src.contains(FILE_FORMAT_PATH)
}

pub fn has_private_locator(src: &str) -> bool {
    src.contains(PRIVATE_PORT)
}

pub fn collect_readme_tree_walls(src: &str) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    if src.trim().is_empty() {
        push_unique(&mut rows, WALL_EMPTY);
        return rows;
    }
    if vendor_as_gap(src) {
        push_unique(&mut rows, WALL_VENDOR_FOLDER);
    }
    if names_root_schemas(src) == false {
        push_unique(&mut rows, WALL_ROOT_SCHEMA);
    }
    if names_vendor_paths(src) == false {
        push_unique(&mut rows, WALL_VENDOR_URL);
    }
    if has_private_locator(src) {
        push_unique(&mut rows, WALL_PRIVATE_LOCATOR);
    }
    if names_helper_packages(src) == false {
        push_unique(&mut rows, WALL_HELPER);
    }
    rows
}

pub fn scan_readme_tree_text(src: &str) -> Result<String, String> {
    let rows = collect_readme_tree_walls(src);
    if rows.is_empty() {
        return Ok(String::from("ok GAP README tree truth"));
    }
    let mut out = String::new();
    let mut i = 0usize;
    while i < rows.len() {
        if 0 < i {
            out.push(char::from(10));
        }
        out.push_str(&rows[i]);
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

pub mod vendor_folder_as_gap {
    pub struct VendorFolderAsGap {
        pub src: String,
        pub claims: bool,
    }
    impl VendorFolderAsGap {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                claims: false,
            }
        }

        /// Detect GAP README lines that name vendor-tree folders as GAP repo files
        pub fn process(&mut self) {
            self.claims = super::vendor_as_gap(&self.src);
        }
    }
}

pub mod root_schema_named {
    pub struct RootSchemaNamed {
        pub src: String,
        pub named: bool,
    }
    impl RootSchemaNamed {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                named: false,
            }
        }

        /// Require GAP meta schema v1 v1.2 and v1.3 named at repo root
        pub fn process(&mut self) {
            self.named = super::names_root_schemas(&self.src);
        }
    }
}

pub mod vendor_url_named {
    pub struct VendorUrlNamed {
        pub src: String,
        pub named: bool,
    }
    impl VendorUrlNamed {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                named: false,
            }
        }

        /// Require vendor tree paths without a private locator
        pub fn process(&mut self) {
            self.named = super::names_vendor_paths(&self.src) && super::has_private_locator(&self.src) == false;
        }
    }
}

pub mod helper_package_named {
    pub struct HelperPackageNamed {
        pub src: String,
        pub named: bool,
    }
    impl HelperPackageNamed {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                named: false,
            }
        }

        /// Require helper package folders named on GAP
        pub fn process(&mut self) {
            self.named = super::names_helper_packages(&self.src);
        }
    }
}

pub mod scan_readme_tree {
    pub struct ScanReadmeTree {
        pub readme: String,
        pub result: String,
        pub allow: bool,
    }
    impl ScanReadmeTree {
        pub fn new() -> Self {
            Self {
                readme: String::new(),
                result: String::new(),
                allow: false,
            }
        }

        /// Collect-all GAP README tree truth. Write DENY on miss.
        pub fn process(&mut self) {
            match super::scan_readme_tree_text(&self.readme) {
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
        String::from("- `schemas/` holds GAP meta-schema v1.\n- `lib/gap-compile.mjs` compiles GAP.\n")
    }

    fn ok_doc() -> String {
        let mut s = String::from("Schema files sit at repo root: ");
        s.push_str(SCHEMA_V1);
        s.push_str(" ");
        s.push_str(SCHEMA_V12);
        s.push_str(" ");
        s.push_str(SCHEMA_V13);
        s.push(char::from(10));
        s.push_str(HELPER_SENTENCE);
        s.push(char::from(10));
        s.push_str(HELPER_PROFILE);
        s.push(char::from(10));
        s.push_str(HELPER_CLOSED);
        s.push(char::from(10));
        s.push_str(HELPER_PROOF);
        s.push(char::from(10));
        s.push_str(HELPER_SNAPSHOT);
        s.push(char::from(10));
        s.push_str(VENDOR_HOST);
        s.push(char::from(10));
        s.push_str(COMPILE_PATH);
        s.push(char::from(10));
        s.push_str(REFERENCE_PATH);
        s.push(char::from(10));
        s.push_str(CODING_GOV_PATH);
        s.push(char::from(10));
        s.push_str(FILE_FORMAT_PATH);
        s.push(char::from(10));
        s
    }

    fn locator_doc() -> String {
        let mut s = ok_doc();
        s.push_str("http://example.invalid");
        s.push_str(PRIVATE_PORT);
        s.push_str("thePM001/");
        s.push_str(VENDOR_HOST);
        s.push_str("/src/branch/main/x\n");
        s
    }

    #[test]
    fn vendor_text_denied() {
        if vendor_as_gap(&hole_doc()) == false {
            fail();
        }
        match scan_readme_tree_text(&hole_doc()) {
            Ok(_) => fail(),
            Err(e) => {
                if e.contains(WALL_VENDOR_FOLDER) == false {
                    fail();
                }
            }
        }
    }

    #[test]
    fn ok_text_allowed() {
        if vendor_as_gap(&ok_doc()) {
            fail();
        }
        if has_private_locator(&ok_doc()) {
            fail();
        }
        match scan_readme_tree_text(&ok_doc()) {
            Ok(v) => {
                if v.contains("ok GAP README tree truth") == false {
                    fail();
                }
            }
            Err(_) => fail(),
        }
    }

    #[test]
    fn private_locator_denied() {
        if has_private_locator(&locator_doc()) == false {
            fail();
        }
        match scan_readme_tree_text(&locator_doc()) {
            Ok(_) => fail(),
            Err(e) => {
                if e.contains(WALL_PRIVATE_LOCATOR) == false {
                    fail();
                }
            }
        }
    }

    #[test]
    fn empty_src_denied() {
        match scan_readme_tree_text("") {
            Ok(_) => fail(),
            Err(e) => {
                if e.contains(WALL_EMPTY) == false {
                    fail();
                }
            }
        }
    }

    #[test]
    fn vendor_fixture_denied() {
        let src = include_str ! ("../fixtures/readme-vendor-claim.md");
        match scan_readme_tree_text(src) {
            Ok(_) => fail(),
            Err(e) => {
                if e.contains(WALL_VENDOR_FOLDER) == false {
                    fail();
                }
            }
        }
    }

    #[test]
    fn tree_ok_fixture_allowed() {
        let src = include_str ! ("../fixtures/readme-tree-ok.md");
        match scan_readme_tree_text(src) {
            Ok(_) => {}
            Err(_) => fail(),
        }
        if vendor_as_gap(src) {
            fail();
        }
        if has_private_locator(src) {
            fail();
        }
    }

    #[test]
    fn hole_collects_all_walls() {
        let rows = collect_readme_tree_walls(&hole_doc());
        if rows.is_empty() {
            fail();
        }
        let joined = scan_readme_tree_text(&hole_doc()).err().unwrap();
        if joined.contains(WALL_VENDOR_FOLDER) == false {
            fail();
        }
        if joined.contains(WALL_ROOT_SCHEMA) == false {
            fail();
        }
        if joined.contains(WALL_VENDOR_URL) == false {
            fail();
        }
        if joined.contains(WALL_HELPER) == false {
            fail();
        }
    }

    #[test]
    fn wrappers_run() {
        let mut v = vendor_folder_as_gap::VendorFolderAsGap::new();
        v.src = hole_doc();
        v.process();
        if v.claims == false {
            fail();
        }
        let mut s = scan_readme_tree::ScanReadmeTree::new();
        s.readme = ok_doc();
        s.process();
        if s.allow == false {
            fail();
        }
        let mut h = helper_package_named::HelperPackageNamed::new();
        h.src = ok_doc();
        h.process();
        if h.named == false {
            fail();
        }
        s.readme = hole_doc();
        s.process();
        if s.allow {
            fail();
        }
        let mut u = vendor_url_named::VendorUrlNamed::new();
        u.src = ok_doc();
        u.process();
        if u.named == false {
            fail();
        }
        u.src = locator_doc();
        u.process();
        if u.named {
            fail();
        }
    }
}
