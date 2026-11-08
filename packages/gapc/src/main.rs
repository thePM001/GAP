// gapc command line.
//   gapc check <path>...             check .gap files and every .gap file under directories
//   gapc compile <file> [--out <f>]  write the JSON-encoded GAP instructions of one source
//   gapc schema                      print the GAP meta schema v1.3 gapc checks against
//   gapc version                     print the gapc, profile crate and schema versions
// Exit status: 0 clean, 1 a source failed the check, 2 usage or file error.

use gapc::{collect_gap_files, compile_source, Checker, Problem};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str =
    "usage: gapc check <path>... | gapc compile <file> [--out <file>] | gapc schema | gapc version";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("check") => check(&args[1..]),
        Some("compile") => compile(&args[1..]),
        Some("schema") if args.len() == 1 => {
            print!("{}", gap_schema_profile_v13::SCHEMA_V13_JSON);
            ExitCode::SUCCESS
        }
        Some("version") | Some("--version") if args.len() == 1 => {
            println!("gapc {}", gapc::VERSION);
            println!("gap-schema-profile-v13 {}", gap_schema_profile_v13::VERSION);
            println!("gap meta schema {}", gap_schema_profile_v13::PROFILE);
            ExitCode::SUCCESS
        }
        Some("help") | Some("--help") | Some("-h") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => usage_error(),
    }
}

fn usage_error() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

fn new_checker() -> Option<Checker> {
    match Checker::new() {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("gapc: {e}");
            None
        }
    }
}

fn print_problems(file: &str, problems: &[Problem]) {
    for p in problems {
        match p.document {
            Some(i) => eprintln!("FAIL {file} document {i}: {}", p.message),
            None => eprintln!("FAIL {file}: {}", p.message),
        }
    }
}

fn check(paths: &[String]) -> ExitCode {
    if paths.is_empty() {
        return usage_error();
    }
    let Some(checker) = new_checker() else {
        return ExitCode::from(2);
    };
    let roots: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let files = match collect_gap_files(&roots) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("gapc: {e}");
            return ExitCode::from(2);
        }
    };
    if files.is_empty() {
        eprintln!("gapc: no .gap files under the given paths");
        return ExitCode::from(2);
    }
    let mut failed = 0usize;
    let mut instructions = 0usize;
    for f in &files {
        let name = f.display().to_string();
        let source = match fs::read_to_string(f) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("gapc: {name}: {e}");
                return ExitCode::from(2);
            }
        };
        let report = checker.check_source(&source);
        instructions += report.instructions.len();
        if report.ok() {
            println!("ok {name} {}", report.instructions.join(" "));
        } else {
            failed += 1;
            print_problems(&name, &report.problems);
        }
    }
    println!(
        "gapc check: {} files, {} instructions, {} failed",
        files.len(),
        instructions,
        failed
    );
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn compile(args: &[String]) -> ExitCode {
    let (input, out) = match args {
        [input] => (input, None),
        [input, flag, out] if flag == "--out" => (input, Some(out)),
        _ => return usage_error(),
    };
    let Some(checker) = new_checker() else {
        return ExitCode::from(2);
    };
    let source = match fs::read_to_string(input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("gapc: {input}: {e}");
            return ExitCode::from(2);
        }
    };
    let instructions = match compile_source(&checker, &source) {
        Ok(i) => i,
        Err(problems) => {
            print_problems(input, &problems);
            return ExitCode::from(1);
        }
    };
    let value = if instructions.len() == 1 {
        instructions[0].clone()
    } else {
        serde_json::Value::Array(instructions)
    };
    let text = match serde_json::to_string_pretty(&value) {
        Ok(t) => t + "\n",
        Err(e) => {
            eprintln!("gapc: {e}");
            return ExitCode::from(2);
        }
    };
    match out {
        Some(path) => match fs::write(path, text) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("gapc: {path}: {e}");
                ExitCode::from(2)
            }
        },
        None => {
            print!("{text}");
            ExitCode::SUCCESS
        }
    }
}
