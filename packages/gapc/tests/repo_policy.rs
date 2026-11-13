// GAP repository policy. Schema names, legacy aliases, one schema body and the version
// rules of VERSIONING.md. These checks read the repository from a clean checkout.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    match fs::read_to_string(repo().join(rel)) {
        Ok(s) => s,
        Err(e) => panic!("{rel}: {e}"),
    }
}

fn bytes(rel: &str) -> Vec<u8> {
    match fs::read(repo().join(rel)) {
        Ok(b) => b,
        Err(e) => panic!("{rel}: {e}"),
    }
}

fn json(rel: &str) -> serde_json::Value {
    match serde_json::from_str(&read(rel)) {
        Ok(v) => v,
        Err(e) => panic!("{rel}: {e}"),
    }
}

/// key = "value" lines of gap-versions.toml.
fn versions() -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in read("gap-versions.toml").lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            panic!("gap-versions.toml line without '=': {line}");
        };
        let v = v.trim();
        assert!(
            v.len() >= 2 && v.starts_with('"') && v.ends_with('"'),
            "gap-versions.toml value must be a quoted string: {line}"
        );
        out.insert(k.trim().to_string(), v[1..v.len() - 1].to_string());
    }
    out
}

fn version(key: &str) -> String {
    match versions().get(key) {
        Some(v) => v.clone(),
        None => panic!("gap-versions.toml has no {key}"),
    }
}

fn cargo_version(manifest: &str) -> String {
    for line in read(manifest).lines() {
        if let Some(rest) = line.trim().strip_prefix("version = \"") {
            return rest.trim_end_matches('"').to_string();
        }
    }
    panic!("{manifest} has no package version");
}

/// Every "AEP <version>" mention in a text.
fn aep_mentions(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find("AEP ") {
        let after = &rest[i + 4..];
        let v: String = after
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let v = v.trim_end_matches('.').to_string();
        if v.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            out.push(v);
        }
        rest = after;
    }
    out
}

const LEGACY_ALIASES: [(&str, &str); 3] = [
    (
        "GAP meta schema v1.json",
        "schemas/gap-meta-schema-v1.1.json",
    ),
    (
        "GAP meta schema v1.2.json",
        "schemas/gap-meta-schema-v1.2.json",
    ),
    (
        "GAP meta schema v1.3.json",
        "schemas/gap-meta-schema-v1.3.json",
    ),
];

#[test]
fn schema_files_are_kebab_case_and_named_by_their_id() {
    let mut names: Vec<String> = match fs::read_dir(repo().join("schemas")) {
        Ok(d) => d
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n != "README.md")
            .collect(),
        Err(e) => panic!("schemas: {e}"),
    };
    names.sort();
    assert_eq!(
        names,
        vec![
            "gap-meta-schema-v1.1.json",
            "gap-meta-schema-v1.2.json",
            "gap-meta-schema-v1.3.json"
        ]
    );
    for name in &names {
        assert!(!name.contains(' '), "{name}");
        let schema = json(&format!("schemas/{name}"));
        assert_eq!(
            schema["$id"].as_str(),
            Some(format!("https://gap-lang.aep.dev/schemas/{name}").as_str()),
            "{name} $id"
        );
        assert_eq!(
            schema["$schema"].as_str(),
            Some("https://json-schema.org/draft/2020-12/schema")
        );
    }
}

#[test]
fn legacy_schema_names_are_byte_aliases_for_one_release() {
    for (old, new) in LEGACY_ALIASES {
        assert_eq!(bytes(old), bytes(new), "{old} must equal {new}");
    }
    // No other repository root file name carries a space.
    let root: Vec<String> = match fs::read_dir(repo()) {
        Ok(d) => d
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains(' '))
            .collect(),
        Err(e) => panic!("repo root: {e}"),
    };
    for name in root {
        assert!(
            LEGACY_ALIASES.iter().any(|(old, _)| *old == name),
            "{name} has a space and is not a listed legacy alias"
        );
    }
}

#[test]
fn live_schema_has_one_body() {
    let live = format!("schemas/gap-meta-schema-v{}.json", version("live_schema"));
    let canonical = bytes(&live);
    assert_eq!(
        bytes("packages/gap-schema-profile-v13/src/schema_v13.body"),
        canonical,
        "schema_v13.body must be a byte copy of {live}"
    );
    assert_eq!(
        gap_schema_profile_v13::SCHEMA_V13_JSON.as_bytes(),
        canonical.as_slice()
    );
    assert_eq!(
        gap_schema_profile_v13::PROFILE,
        format!("v{}", version("live_schema"))
    );
}

#[test]
fn historical_schemas_keep_covenants_and_scanners_without_rank() {
    for rel in [
        "schemas/gap-meta-schema-v1.1.json",
        "schemas/gap-meta-schema-v1.2.json",
    ] {
        let body = read(rel);
        assert!(body.contains("\"covenants\""), "{rel}");
        assert!(body.contains("\"scanners\""), "{rel}");
        assert!(!body.contains("\"trust_ring\""), "{rel}");
        assert!(!body.contains("subprotocol"), "{rel}");
    }
    assert!(!read("schemas/gap-meta-schema-v1.2.json").contains("sandbox"));
}

#[test]
fn crate_versions_match_gap_versions() {
    for (key, manifest) in [
        (
            "gap-schema-profile-v13",
            "packages/gap-schema-profile-v13/Cargo.toml",
        ),
        (
            "gap-closed-wall-deny",
            "packages/gap-closed-wall-deny/Cargo.toml",
        ),
        ("gapc", "packages/gapc/Cargo.toml"),
    ] {
        assert_eq!(cargo_version(manifest), version(key), "{manifest}");
    }
    assert_eq!(
        gap_schema_profile_v13::VERSION,
        version("gap-schema-profile-v13")
    );
    assert_eq!(gapc::VERSION, version("gapc"));
}

#[test]
fn readme_names_one_aep_version_and_the_language_spec() {
    let readme = read("README.md");
    let aep = version("aep");
    let mentions = aep_mentions(&readme);
    assert!(!mentions.is_empty(), "README names no AEP version");
    for m in &mentions {
        assert_eq!(m, &aep, "README names AEP {m}, the policy says AEP {aep}");
    }
    let spec = format!("language specification v{}", version("language_spec"));
    assert!(readme.contains(&spec), "README must name {spec}");
    assert!(readme.contains("VERSIONING.md"));
    for (_, new) in LEGACY_ALIASES {
        assert!(readme.contains(new), "README must name {new}");
    }
}

#[test]
fn fixtures_name_the_policy_aep_version() {
    let aep = version("aep");
    let dir = repo().join("packages/gap-schema-profile-v13/fixtures");
    let mut seen = 0usize;
    let mut files: Vec<PathBuf> = match fs::read_dir(&dir) {
        Ok(d) => d.filter_map(|e| e.ok()).map(|e| e.path()).collect(),
        Err(e) => panic!("{}: {e}", dir.display()),
    };
    files.sort();
    for f in files {
        if f.extension().is_none_or(|x| x != "gap") {
            continue;
        }
        let text = match fs::read_to_string(&f) {
            Ok(t) => t,
            Err(e) => panic!("{}: {e}", f.display()),
        };
        let v: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => panic!("{}: {e}", f.display()),
        };
        assert_eq!(
            v["metadata"]["aep_version"].as_str(),
            Some(aep.as_str()),
            "{}",
            f.display()
        );
        for m in aep_mentions(&text) {
            assert_eq!(m, aep, "{} names AEP {m}", f.display());
        }
        seen += 1;
    }
    assert!(0 < seen);
}

#[test]
fn versioning_document_names_every_value() {
    let doc = read("VERSIONING.md");
    for (key, value) in versions() {
        assert!(doc.contains(&key), "VERSIONING.md must name {key}");
        assert!(
            doc.contains(&value),
            "VERSIONING.md must name {key} {value}"
        );
    }
}

#[test]
fn every_fixture_passes_gapc_check() {
    let checker = match gapc::Checker::new() {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    let files = match gapc::collect_gap_files(&[repo().join("packages")]) {
        Ok(f) => f,
        Err(e) => panic!("{e}"),
    };
    assert!(4 <= files.len());
    for f in files {
        let text = match fs::read_to_string(&f) {
            Ok(t) => t,
            Err(e) => panic!("{}: {e}", f.display()),
        };
        let report = checker.check_source(&text);
        assert!(report.ok(), "{}: {:?}", f.display(), report.problems);
    }
}

#[test]
fn aep_mentions_reads_versions() {
    assert_eq!(
        aep_mentions("AEP 2.9.0 and AEP 2.8.5. Not AEP itself."),
        vec!["2.9.0", "2.8.5"]
    );
}

/// Fenced yaml and json blocks of the README, with the "> " quote prefix removed.
fn readme_blocks() -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in read("README.md").lines() {
        let body = line
            .strip_prefix("> ")
            .unwrap_or(line.trim_start_matches('>'));
        match current.as_mut() {
            None => {
                if body == "```yaml" || body == "```json" {
                    current = Some(String::new());
                }
            }
            Some(block) => {
                if body == "```" {
                    out.push(std::mem::take(block));
                    current = None;
                } else {
                    block.push_str(body);
                    block.push('\n');
                }
            }
        }
    }
    out
}

#[test]
fn readme_instruction_examples_pass_gapc_check() {
    let checker = match gapc::Checker::new() {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    let mut checked = 0usize;
    for block in readme_blocks() {
        let docs = gapc::read_documents(&block).unwrap_or_default();
        if !docs.iter().any(|d| gapc::is_instruction(&d.value)) {
            continue;
        }
        let report = checker.check_source(&block);
        assert!(report.ok(), "README example {block}: {:?}", report.problems);
        checked += 1;
    }
    assert!(3 <= checked, "README holds {checked} instruction examples");
}

// README tests. The root README follows the seven sections of the GAP page, describes the
// language of the live schema once and every package and folder carries a README.

/// The seven sections of the GAP page in page order, as the root README heads them.
const PAGE_SECTIONS: [&str; 7] = [
    "## 01 / Six Native Primitives",
    "## 02 / Everything as Instruction",
    "## 03 / Write. Check. Admit.",
    "## 04 / Instructions Generate Instructions",
    "## 05 / Structural Governance",
    "## 06 / Mathematically Verified",
    "## 07 / Six-Rung Ladder",
];

/// The four labels every one of the seven sections carries.
const SECTION_LABELS: [&str; 4] = [
    "**What it is:**",
    "**What ships:**",
    "**Where it lives:**",
    "**Run and test:**",
];

/// Packages with their key in gap-versions.toml. Each carries a README of one shape.
const PACKAGES: [&str; 3] = ["gapc", "gap-schema-profile-v13", "gap-closed-wall-deny"];

/// The parts of a package README after its title, in order.
const PACKAGE_PARTS: [&str; 7] = [
    "## What it is",
    "## Place in GAP",
    "## What ships",
    "## Interfaces",
    "## Limits",
    "## Run and test",
    "## Related documents",
];

/// Folders of the tree whose inline code paths a document must resolve.
const TREE_ROOTS: [&str; 4] = ["packages/", "schemas/", "docs/", ".github/"];

/// Lines of a document outside fenced code blocks, with their one-based line number.
fn prose_lines(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut fenced = false;
    for (i, line) in text.lines().enumerate() {
        let body = line.trim_start_matches('>').trim();
        if body.starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if !fenced {
            out.push((i + 1, body.to_string()));
        }
    }
    out
}

/// Inline code spans of one line.
fn inline_code(line: &str) -> Vec<String> {
    line.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// Targets of the links of one line, the text between "](" and ")".
fn link_targets(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(i) = rest.find("](") {
        let after = &rest[i + 2..];
        let Some(end) = after.find(')') else {
            break;
        };
        out.push(after[..end].to_string());
        rest = &after[end..];
    }
    out
}

/// Every inline code span of the prose of a document.
fn code_spans(text: &str) -> Vec<String> {
    prose_lines(text)
        .iter()
        .flat_map(|(_, line)| inline_code(line))
        .collect()
}

/// The root README, the folder README files and every document under docs.
fn documents() -> Vec<String> {
    let mut out = vec![String::from("README.md")];
    for name in PACKAGES {
        out.push(format!("packages/{name}/README.md"));
    }
    out.push(String::from("schemas/README.md"));
    out.push(String::from("docs/README.md"));
    let entries = match fs::read_dir(repo().join("docs")) {
        Ok(e) => e,
        Err(e) => panic!("docs: {e}"),
    };
    let mut docs: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.ends_with(".md") && n != "README.md")
        .map(|n| format!("docs/{n}"))
        .collect();
    docs.sort();
    out.extend(docs);
    out
}

/// String values of a JSON array.
fn strings(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn readme_holds_the_seven_page_sections_in_order() {
    let readme = read("README.md");
    let lines = prose_lines(&readme);
    let mut starts = Vec::new();
    for heading in PAGE_SECTIONS {
        match lines.iter().position(|(_, l)| l == heading) {
            Some(i) => starts.push(i),
            None => panic!("README has no heading {heading}"),
        }
    }
    assert!(
        starts.windows(2).all(|w| w[0] < w[1]),
        "README sections are not in the order of the GAP page"
    );
    for (n, start) in starts.iter().enumerate() {
        let end = lines[start + 1..]
            .iter()
            .position(|(_, l)| l.starts_with("## "))
            .map_or(lines.len(), |i| start + 1 + i);
        let section = &lines[*start..end];
        for label in SECTION_LABELS {
            assert!(
                section.iter().any(|(_, l)| l.starts_with(label)),
                "README section {} has no label {label}",
                PAGE_SECTIONS[n]
            );
        }
    }
}

#[test]
fn readme_repeats_no_line() {
    let readme = read("README.md");
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for (number, line) in prose_lines(&readme) {
        let rule = line.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '));
        if line.is_empty() || rule {
            continue;
        }
        if let Some(first) = seen.insert(line.clone(), number) {
            panic!("README line {number} repeats line {first}: {line}");
        }
    }
}

#[test]
fn readme_names_the_vocabulary_of_the_live_schema() {
    let schema = json("schemas/gap-meta-schema-v1.3.json");
    let spans = code_spans(&read("README.md"));
    let mut words: Vec<String> = match schema["properties"].as_object() {
        Some(p) => p.keys().cloned().collect(),
        None => panic!("live schema has no properties"),
    };
    assert_eq!(words.len(), 11, "top level fields of the live schema");
    for pointer in [
        "/$defs/Composition/properties/type/enum",
        "/$defs/Action/properties/type/enum",
        "/$defs/Metadata/properties/scanners/items/enum",
        "/$defs/ConstraintExpression/properties/severity/enum",
    ] {
        let list = match schema.pointer(pointer) {
            Some(v) => strings(v),
            None => panic!("live schema has no {pointer}"),
        };
        assert!(!list.is_empty(), "{pointer} is empty");
        words.extend(list);
    }
    for word in words {
        assert!(
            spans.contains(&word),
            "README does not name `{word}` of the live schema"
        );
    }
}

#[test]
fn documents_resolve_their_links_and_tree_paths() {
    let root = repo();
    for doc in documents() {
        let text = read(&doc);
        let folder = Path::new(&doc).parent().unwrap_or(Path::new(""));
        for (number, line) in prose_lines(&text) {
            for target in link_targets(&line) {
                if target.contains("://") || target.starts_with('#') || target.is_empty() {
                    continue;
                }
                let file = target.split('#').next().unwrap_or("");
                assert!(
                    root.join(folder).join(file).exists(),
                    "{doc} line {number}: link {target} does not resolve"
                );
            }
            for span in inline_code(&line) {
                let path = span.trim_end_matches('/');
                let names_tree = TREE_ROOTS.iter().any(|r| span.starts_with(r));
                if !names_tree || span.contains(['<', '*', ' ']) {
                    continue;
                }
                assert!(
                    root.join(path).exists(),
                    "{doc} line {number}: path {span} is not in the tree"
                );
            }
        }
    }
}

#[test]
fn every_package_carries_a_readme_of_one_shape() {
    for name in PACKAGES {
        let rel = format!("packages/{name}/README.md");
        let text = read(&rel);
        let lines = prose_lines(&text);
        let title = format!("# {name} {}", version(name));
        assert_eq!(
            lines.first().map(|(_, l)| l.as_str()),
            Some(title.as_str()),
            "{rel} must start with its name and its version"
        );
        let mut last = 0usize;
        for part in PACKAGE_PARTS {
            match lines.iter().position(|(_, l)| l == part) {
                Some(i) => {
                    assert!(last < i, "{rel}: part {part} is out of order");
                    last = i;
                }
                None => panic!("{rel} has no part {part}"),
            }
        }
    }
}
