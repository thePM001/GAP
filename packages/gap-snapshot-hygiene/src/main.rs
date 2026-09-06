// @PAD: gap-285-p2-snapshot-hygiene-cli
// @GCDE: gaplune.policy.v1
// GAP-285-P2 CLI. stdin README. stdout allow and closed walls.

use gap_snapshot_hygiene::{closed_walls, scan_readme_snapshot_hygiene};
use std::io::{self, Read, Write};

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let mut out = io::stdout();
    match scan_readme_snapshot_hygiene(&buf) {
        Ok(v) => {
            let _ = out.write_all(b"allow=true\nscan=");
            let _ = out.write_all(v.as_bytes());
            let _ = out.write_all(b"\n");
        }
        Err(e) => {
            let _ = out.write_all(b"allow=false\n");
            let rows = closed_walls(&e);
            let mut i = 0usize;
            while i < rows.len() {
                let _ = out.write_all(b"closed=");
                let _ = out.write_all(rows[i].as_bytes());
                let _ = out.write_all(b"\n");
                i += 1;
            }
        }
    }
}
