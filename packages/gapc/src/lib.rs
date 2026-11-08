// crate: gapc
// GAP compiler. Reads .gap sources (JSON or YAML 1.2, one instruction per document)
// with the same reader the live kernel uses, checks every instruction against the GAP
// meta schema v1.3 and the load-time live profile walls, and compiles instructions to
// JSON-encoded GAP.

use gap_schema_profile_v13::{
    compile_guard_wall, compile_leftover_rank_field_wall, read_gap_documents, AdmitWall,
    SCHEMA_V13_JSON,
};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// One parsed document of a .gap source with its zero-based position.
#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub index: usize,
    pub value: Value,
}

/// One problem found in a .gap source. document is None for whole-source problems.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    pub document: Option<usize>,
    pub message: String,
}

/// Result of checking one .gap source.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceReport {
    pub instructions: Vec<String>,
    pub skipped: usize,
    pub problems: Vec<Problem>,
}

impl SourceReport {
    pub fn ok(&self) -> bool {
        self.problems.is_empty()
    }
}

/// The GAP meta schema v1.3 validator built from the schema the profile crate embeds.
pub struct Checker {
    validator: jsonschema::Validator,
}

impl Checker {
    pub fn new() -> Result<Self, String> {
        let schema: Value = serde_json::from_str(SCHEMA_V13_JSON)
            .map_err(|e| format!("embedded GAP meta schema v1.3 is not JSON: {e}"))?;
        let validator = jsonschema::draft202012::new(&schema)
            .map_err(|e| format!("embedded GAP meta schema v1.3 does not compile: {e}"))?;
        Ok(Self { validator })
    }

    /// Schema errors of one instruction as path and message lines. Empty means valid.
    pub fn schema_errors(&self, instruction: &Value) -> Vec<String> {
        let mut out: Vec<String> = self
            .validator
            .iter_errors(instruction)
            .map(|e| {
                let path = e.instance_path().to_string();
                let at = if path.is_empty() {
                    String::from("/")
                } else {
                    path
                };
                format!("schema {at}: {e}")
            })
            .collect();
        out.sort();
        out.dedup();
        out
    }

    /// Check one .gap source text.
    pub fn check_source(&self, source: &str) -> SourceReport {
        let mut report = SourceReport::default();
        let docs = match read_documents(source) {
            Ok(d) => d,
            Err(e) => {
                report.problems.push(Problem {
                    document: None,
                    message: e,
                });
                return report;
            }
        };
        let mut instruction_count = 0usize;
        for doc in &docs {
            if !is_instruction(&doc.value) {
                report.skipped += 1;
                continue;
            }
            instruction_count += 1;
            report.instructions.push(address_of(&doc.value));
            for message in self.schema_errors(&doc.value) {
                report.problems.push(Problem {
                    document: Some(doc.index),
                    message,
                });
            }
            for message in load_time_wall_problems(&doc.value) {
                report.problems.push(Problem {
                    document: Some(doc.index),
                    message,
                });
            }
        }
        if instruction_count == 0 {
            report.problems.push(Problem {
                document: None,
                message: String::from(
                    "no GAP instruction document: an instruction has an address object with domain and id",
                ),
            });
        }
        report
    }
}

/// Read a .gap source into its documents with the live kernel reader.
/// JSON-encoded GAP is one document. YAML GAP source may hold several documents.
pub fn read_documents(source: &str) -> Result<Vec<Document>, String> {
    let mut values = Vec::new();
    let mut err = String::new();
    read_gap_documents(source, &mut values, &mut err);
    if !err.is_empty() {
        return Err(err);
    }
    Ok(values
        .into_iter()
        .enumerate()
        .map(|(index, value)| Document { index, value })
        .collect())
}

/// An instruction document carries an address object with a domain and an id.
pub fn is_instruction(v: &Value) -> bool {
    let address = v.get("address");
    let domain = address
        .and_then(|a| a.get("domain"))
        .and_then(Value::as_str);
    let id = address.and_then(|a| a.get("id")).and_then(Value::as_str);
    matches!((domain, id), (Some(d), Some(i)) if !d.is_empty() && !i.is_empty())
}

/// domain/id of an instruction.
pub fn address_of(v: &Value) -> String {
    let domain = v["address"]["domain"].as_str().unwrap_or("");
    let id = v["address"]["id"].as_str().unwrap_or("");
    format!("{domain}/{id}")
}

/// Walls that close on the document alone. A leftover rank field or a structured
/// pattern.guard without expr closes on every request, so the instruction can never Admit.
/// The agent permission and wrap walls depend on the request and are judged at Admit.
pub fn load_time_wall_problems(doc: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let mut rank = AdmitWall::open("");
    compile_leftover_rank_field_wall(doc, &mut rank);
    if rank.closed {
        out.push(format!("wall {} closed: {}", rank.id, rank.reason));
    }
    let mut guard = AdmitWall::open("");
    compile_guard_wall(doc, &mut guard);
    if guard.closed {
        out.push(format!("wall {} closed: {}", guard.id, guard.reason));
    }
    out
}

/// Compile a .gap source to its JSON-encoded GAP instructions. Fails when the check fails.
pub fn compile_source(checker: &Checker, source: &str) -> Result<Vec<Value>, Vec<Problem>> {
    let report = checker.check_source(source);
    if !report.ok() {
        return Err(report.problems);
    }
    let docs = read_documents(source).map_err(|e| {
        vec![Problem {
            document: None,
            message: e,
        }]
    })?;
    Ok(docs
        .into_iter()
        .filter(|d| is_instruction(&d.value))
        .map(|d| d.value)
        .collect())
}

/// Every .gap file under the given paths. Directories are walked in sorted order.
pub fn collect_gap_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    for p in paths {
        collect_into(p, &mut out, true)?;
    }
    Ok(out)
}

fn collect_into(p: &Path, out: &mut Vec<PathBuf>, named: bool) -> Result<(), String> {
    let meta = fs::metadata(p).map_err(|e| format!("{}: {e}", p.display()))?;
    if meta.is_file() {
        if named || p.extension().is_some_and(|x| x == "gap") {
            out.push(p.to_path_buf());
        }
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(p)
        .map_err(|e| format!("{}: {e}", p.display()))?
        .map(|e| e.map(|x| x.path()))
        .collect::<Result<_, _>>()
        .map_err(|e| format!("{}: {e}", p.display()))?;
    entries.sort();
    for e in entries {
        collect_into(&e, out, false)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML_OK: &str = "address:\n  domain: com.example.finance\n  id: pay.v1\npattern:\n  guard: \"true\"\naction:\n  type: template\n  content: pay\nweight: 1.0\ncomposition:\n  type: atomic\nmetadata:\n  provenance: system.seed\n  version: 1.0.0\n  stability: experimental\n  wrap: finance\n  agent_permission:\n    - agent_id: agent-a\n      action: finance.pay\n";

    fn checker() -> Checker {
        match Checker::new() {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn yaml_instruction_checks_clean() {
        let r = checker().check_source(YAML_OK);
        assert_eq!(r.problems, Vec::new());
        assert_eq!(
            r.instructions,
            vec![String::from("com.example.finance/pay.v1")]
        );
    }

    #[test]
    fn json_and_yaml_compile_to_the_same_instruction() {
        let c = checker();
        let from_yaml = compile_source(&c, YAML_OK).unwrap_or_default();
        let json = serde_json::to_string(&from_yaml[0]).unwrap_or_default();
        let from_json = compile_source(&c, &json).unwrap_or_default();
        assert_eq!(from_yaml.len(), 1);
        assert_eq!(from_yaml, from_json);
    }

    #[test]
    fn trust_ring_fails_schema_and_rank_wall() {
        let src = YAML_OK.replace("  wrap: finance\n", "  wrap: finance\n  trust_ring: user\n");
        let r = checker().check_source(&src);
        assert!(!r.ok());
        assert!(r.problems.iter().any(|p| p.message.contains("trust_ring")));
        assert!(r
            .problems
            .iter()
            .any(|p| p.message.contains("wall gap:leftover_rank_field closed")));
    }

    #[test]
    fn unknown_metadata_field_fails_schema() {
        let src = YAML_OK.replace("  wrap: finance\n", "  wrap: finance\n  framework: x\n");
        let r = checker().check_source(&src);
        assert!(r.problems.iter().any(|p| p.message.contains("framework")));
    }

    #[test]
    fn aep_version_must_be_a_release_number() {
        let good = YAML_OK.replace(
            "  wrap: finance\n",
            "  wrap: finance\n  aep_version: 2.9.0\n",
        );
        assert!(checker().check_source(&good).ok());
        let bad = YAML_OK.replace(
            "  wrap: finance\n",
            "  wrap: finance\n  aep_version: \"2.9\"\n",
        );
        assert!(!checker().check_source(&bad).ok());
    }

    #[test]
    fn missing_required_field_fails_schema() {
        let src = YAML_OK.replace("weight: 1.0\n", "");
        let r = checker().check_source(&src);
        assert!(r.problems.iter().any(|p| p.message.contains("weight")));
    }

    #[test]
    fn structured_guard_without_expr_closes_the_guard_wall() {
        let src = YAML_OK.replace("  guard: \"true\"\n", "  guard:\n    lang: gapdsl\n");
        let r = checker().check_source(&src);
        assert!(r
            .problems
            .iter()
            .any(|p| p.message.contains("wall gap:pattern:guard closed")));
    }

    #[test]
    fn multi_document_source_reads_every_instruction() {
        let other = YAML_OK.replace("id: pay.v1", "id: refund.v1");
        let src = format!("{YAML_OK}---\nkind: aep.caw.profile\nname: x\n---\n{other}");
        let r = checker().check_source(&src);
        assert_eq!(r.problems, Vec::new());
        assert_eq!(r.instructions.len(), 2);
        assert_eq!(r.skipped, 1);
    }

    #[test]
    fn source_without_instruction_fails() {
        let r = checker().check_source("kind: aep.caw.profile\nname: x\n");
        assert!(!r.ok());
        assert!(r.problems[0]
            .message
            .contains("no GAP instruction document"));
    }

    #[test]
    fn broken_json_is_a_parse_problem() {
        let r = checker().check_source("{\"address\": ");
        assert!(r.problems[0]
            .message
            .starts_with("JSON GAP source parse failed"));
    }

    #[test]
    fn block_scalars_check_like_the_kernel_reads_them() {
        let src = YAML_OK.replace(
            "  content: pay\n",
            "  content: |\n    Pay the invoice.\n    Name the ledger row.\n",
        );
        let r = checker().check_source(&src);
        assert_eq!(r.problems, Vec::new());
        let ring = src.replace("  wrap: finance\n", "  wrap: finance\n  trust_ring: user\n");
        let r = checker().check_source(&ring);
        assert!(r
            .problems
            .iter()
            .any(|p| p.message.contains("wall gap:leftover_rank_field closed")));
    }

    #[test]
    fn yaml_anchors_resolve() {
        let docs = read_documents("a: &x 1\nb: *x\n").unwrap_or_default();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].value["b"], Value::from(1));
    }
}
