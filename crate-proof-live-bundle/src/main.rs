// @PAD: gap-285-p4-proof-live-bundle-mode-cli
// @GCDE: gaplune.policy.v1
// GAP-285-P4 CLI. stdin README. stdout allow and live_bundle_mode.

use gap_proof_live_bundle_mode::{
    scan_readme_live_bundle_honesty, LIVE_BUNDLE_MODE, OPTIONAL_ALG_ED25519, OPTIONAL_ALG_MLDSA65,
};
use std::io::{self, Read, Write};

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let mut out = io::stdout();
    let _ = out.write_all(b"live_bundle_mode=");
    let _ = out.write_all(LIVE_BUNDLE_MODE.as_bytes());
    let _ = out.write_all(b"\noptional_algorithm=");
    let _ = out.write_all(OPTIONAL_ALG_ED25519.as_bytes());
    let _ = out.write_all(b"\noptional_algorithm=");
    let _ = out.write_all(OPTIONAL_ALG_MLDSA65.as_bytes());
    let _ = out.write_all(b"\n");
    match scan_readme_live_bundle_honesty(&buf) {
        Ok(v) => {
            let _ = out.write_all(b"allow=true\nscan=");
            let _ = out.write_all(v.as_bytes());
            let _ = out.write_all(b"\n");
        }
        Err(e) => {
            let _ = out.write_all(b"allow=false\nclosed=");
            let _ = out.write_all(e.as_bytes());
            let _ = out.write_all(b"\n");
        }
    }
}
