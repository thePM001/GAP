
// @PAD: gap-285-p14-readme-tree-cli
// @GCDE: gaplune.policy.v1
// GAP-285-P14 CLI. stdin README. stdout allow and closed walls.

use gap_285_p14_readme_tree::{closed_walls, scan_readme_tree_text, TICKET};
use std::io::{self, Read, Write};

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let mut out = io::stdout();
    let _ = out.write_all(b"ticket=");
    let _ = out.write_all(TICKET.as_bytes());
    let _ = out.write_all(b"\n");

    match scan_readme_tree_text(&buf) {
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
