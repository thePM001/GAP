// @PAD: gap-285-p4-proof-live-bundle-mode
// @GCDE: gaplune.policy.v1
// crate: gap-proof-live-bundle-mode
// GAP-285-P4. Live AEP 2.8.5 EPSCOM trust bundle mode is sha256-structure.
// Optional proof algorithms may name Ed25519 or ML-DSA-65.
// The signatures loader denies an ML-DSA claim on sha256-structure.

pub const TICKET: &str = "GAP-285-P4";
pub const LIVE_BUNDLE_MODE: &str = "sha256-structure";
pub const OPTIONAL_ALG_ED25519: &str = "ed25519";
pub const OPTIONAL_ALG_MLDSA65: &str = "ml-dsa-65";
pub const WALL_MLDSA_DEFAULT_LIVE: &str = "gap:proof:mldsa-default-live";
pub const WALL_MLDSA_ON_SHA256: &str = "gap:proof:mldsa-on-sha256-structure";
pub const WALL_LIVE_MODE: &str = "gap:proof:live-bundle-mode";
pub const WALL_SIGNED_LIVE_ATTACH: &str = "gap:proof:signed-live-attach";

pub fn lower(src: &str) -> String {
    src.to_ascii_lowercase()
}

pub fn integrity_only_mode(mode: &str) -> bool {
    let m = lower(mode);
    m == "sha256-structure" || m == "sha256" || m == "integrity" || m == "hash"
}

pub fn is_optional_proof_algorithm(alg: &str) -> bool {
    let a = lower(alg);
    a == OPTIONAL_ALG_ED25519 || a == OPTIONAL_ALG_MLDSA65 || a == "ml-dsa" || a == "ed25519"
}

pub fn claims_mldsa(src: &str) -> bool {
    lower(src).contains("ml-dsa")
}

pub fn names_live_sha256_structure(src: &str) -> bool {
    let l = lower(src);
    l.contains("sha256-structure") || l.contains("sha-256 structure")
}

pub fn names_optional_proof_algorithm(src: &str) -> bool {
    let l = lower(src);
    l.contains("optional proof algorithm") || l.contains("optional proof algorithms")
}

pub fn claims_every_attach_ships_mldsa(src: &str) -> bool {
    let l = lower(src);
    if l.contains("ml-dsa") == false {
        return false;
    }
    if l.contains("every") == false {
        return false;
    }
    l.contains("attach") && l.contains("ships")
}

pub fn claims_mldsa_as_default_live(src: &str) -> bool {
    let l = lower(src);
    if l.contains("ml-dsa") == false {
        return false;
    }
    if names_optional_proof_algorithm(&l) && names_live_sha256_structure(&l) {
        return false;
    }
    if l.contains("default") && (l.contains("live") || l.contains("bundle")) {
        return true;
    }
    false
}

pub fn claims_signed_ed25519_or_mldsa_as_live_attach(src: &str) -> bool {
    let l = lower(src);
    if l.contains("signed proof bundles with ed25519 or ml-dsa") {
        return true;
    }
    if l.contains("ed25519 or ml-dsa-65 signatures") && names_optional_proof_algorithm(&l) == false
    {
        return true;
    }
    if l.contains("signed proof bundles with ed25519") && names_live_sha256_structure(&l) == false
    {
        return true;
    }
    false
}

pub fn refuse_mldsa_claim_on_sha256_structure(mode: &str, claim: &str) -> Result<String, String> {
    let c = lower(claim);
    if integrity_only_mode(mode) == false {
        return Ok(String::from("ok live bundle mode"));
    }
    if c.contains("ml-dsa") == false {
        return Ok(String::from("ok live bundle mode sha256-structure"));
    }
    if c.contains("not claimed") || c.contains("does not claim") || c.contains("optional proof algorithm")
    {
        return Ok(String::from("ok live bundle mode sha256-structure"));
    }
    Err(String::from("trust_bundle_mldsa_claim_on_sha256_structure"))
}

pub fn scan_readme_live_bundle_honesty(src: &str) -> Result<String, String> {
    if src.trim().is_empty() {
        return Err(String::from("empty README source"));
    }
    if claims_every_attach_ships_mldsa(src) {
        return Err(String::from(WALL_MLDSA_DEFAULT_LIVE));
    }
    if claims_mldsa_as_default_live(src) {
        return Err(String::from(WALL_MLDSA_DEFAULT_LIVE));
    }
    if claims_signed_ed25519_or_mldsa_as_live_attach(src) {
        return Err(String::from(WALL_SIGNED_LIVE_ATTACH));
    }
    if names_live_sha256_structure(src) == false {
        return Err(String::from(WALL_LIVE_MODE));
    }
    if claims_mldsa(src) && names_optional_proof_algorithm(src) == false {
        match refuse_mldsa_claim_on_sha256_structure(LIVE_BUNDLE_MODE, src) {
            Ok(_) => {}
            Err(_) => return Err(String::from(WALL_MLDSA_ON_SHA256)),
        }
    }
    Ok(String::from(
        "ok live bundle mode sha256-structure optional proof algorithms",
    ))
}

pub fn live_bundle_mode(out: &mut String) {
    out.clear();
    out.push_str(LIVE_BUNDLE_MODE);
}

pub mod live_bundle_mode {
    pub struct LiveBundleMode {
        pub src: String,
        pub mode: String,
    }
    impl LiveBundleMode {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                mode: String::new(),
            }
        }
        pub fn process(&mut self) {
            super::live_bundle_mode(&mut self.mode);
        }
    }
}

pub mod optional_proof_algorithm {
    pub struct OptionalProofAlgorithm {
        pub src: String,
        pub algorithm: String,
    }
    impl OptionalProofAlgorithm {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                algorithm: String::new(),
            }
        }
        pub fn process(&mut self) {
            if super::is_optional_proof_algorithm(&self.src) {
                self.algorithm = super::lower(&self.src);
            } else {
                self.algorithm.clear();
            }
        }
    }
}

pub mod refuse_mldsa_claim_on_sha256_structure {
    pub struct RefuseMldsaClaimOnSha256Structure {
        pub src: String,
        pub scan: String,
    }
    impl RefuseMldsaClaimOnSha256Structure {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                scan: String::new(),
            }
        }
        pub fn process(&mut self) {
            self.scan = match super::refuse_mldsa_claim_on_sha256_structure(
                super::LIVE_BUNDLE_MODE,
                &self.src,
            ) {
                Ok(v) => v,
                Err(e) => e,
            };
        }
    }
}

pub mod scan_readme_live_bundle_honesty {
    pub struct ScanReadmeLiveBundleHonesty {
        pub src: String,
        pub scan: String,
        pub allow: bool,
    }
    impl ScanReadmeLiveBundleHonesty {
        pub fn new() -> Self {
            Self {
                src: String::new(),
                scan: String::new(),
                allow: false,
            }
        }
        pub fn process(&mut self) {
            match super::scan_readme_live_bundle_honesty(&self.src) {
                Ok(v) => {
                    self.scan = v;
                    self.allow = true;
                }
                Err(e) => {
                    self.scan = e;
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
            "Signed proof bundles with Ed25519 or ML-DSA-65 signatures as the live AEP 2.8.5 attach.\n",
        )
    }

    fn ed25519_as_live_readme() -> String {
        String::from(
            "| proof | Signed proof bundles with Ed25519. Post-quantum signatures are an optional proof algorithm |\n- Signed proof bundles with Ed25519 and optional post-quantum signatures\n",
        )
    }

    fn honest_readme() -> String {
        String::from(
            "Live AEP 2.8.5 EPSCOM trust bundle mode is sha256-structure. Optional proof algorithms may name Ed25519 or ML-DSA-65. The signatures loader denies an ML-DSA claim on sha256-structure.\n",
        )
    }

    #[test]
    fn live_mode_is_sha256_structure() {
        let mut mode = String::new();
        live_bundle_mode(&mut mode);
        if mode != LIVE_BUNDLE_MODE {
            fail();
        }
        if integrity_only_mode(LIVE_BUNDLE_MODE) == false {
            fail();
        }
    }

    #[test]
    fn optional_algorithms() {
        if is_optional_proof_algorithm("ed25519") == false {
            fail();
        }
        if is_optional_proof_algorithm("ML-DSA-65") == false {
            fail();
        }
        if is_optional_proof_algorithm("rsa") {
            fail();
        }
    }

    #[test]
    fn hole_phrases_denied() {
        match scan_readme_live_bundle_honesty(&hole_readme()) {
            Ok(_) => fail(),
            Err(e) => {
                if e != WALL_SIGNED_LIVE_ATTACH {
                    fail();
                }
            }
        }
        match scan_readme_live_bundle_honesty(&ed25519_as_live_readme()) {
            Ok(_) => fail(),
            Err(e) => {
                if e != WALL_SIGNED_LIVE_ATTACH {
                    fail();
                }
            }
        }
        match scan_readme_live_bundle_honesty("every AEP 2.8.5 attach ships ML-DSA") {
            Ok(_) => fail(),
            Err(e) => {
                if e != WALL_MLDSA_DEFAULT_LIVE {
                    fail();
                }
            }
        }
    }

    #[test]
    fn honest_readme_allowed() {
        match scan_readme_live_bundle_honesty(&honest_readme()) {
            Ok(v) => {
                if v.contains("sha256-structure") == false {
                    fail();
                }
            }
            Err(_) => fail(),
        }
    }

    #[test]
    fn loader_denies_mldsa_claim_on_sha256_structure() {
        match refuse_mldsa_claim_on_sha256_structure(LIVE_BUNDLE_MODE, "pq_signature=ML-DSA-65") {
            Ok(_) => fail(),
            Err(e) => {
                if e != "trust_bundle_mldsa_claim_on_sha256_structure" {
                    fail();
                }
            }
        }
        match refuse_mldsa_claim_on_sha256_structure(LIVE_BUNDLE_MODE, "ML-DSA is not claimed") {
            Ok(_) => {}
            Err(_) => fail(),
        }
    }

    #[test]
    fn wrappers_run() {
        let mut m = live_bundle_mode::LiveBundleMode::new();
        m.process();
        if m.mode != LIVE_BUNDLE_MODE {
            fail();
        }
        let mut s = scan_readme_live_bundle_honesty::ScanReadmeLiveBundleHonesty::new();
        s.src = honest_readme();
        s.process();
        if s.allow == false {
            fail();
        }
        s.src = hole_readme();
        s.process();
        if s.allow {
            fail();
        }
    }
}
