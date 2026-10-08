//! Runs every `tests/frontend/*.bas` according to its mode line (design D10, spec `testing/compiler-tests`).
//!
//! The first line of each file is `' TEST: <mode>`:
//! - `parse-ok`: no syntax errors, and the tree prints back to the file's bytes;
//! - `check-ok`: no errors after `sema`;
//! - `check-fail`: at least one error; the diagnostics are a snapshot;
//! - `typed`, `ir`, `cpp`: no errors; the typed tree, the IR or the emitted fragments are a snapshot.
//!
//! Snapshots live in `tests/snapshots/` next to this file, named after the `.bas` file; review changes with
//! `cargo insta review`. `QB64RUST_FRONTEND_DIR` points the harness at another folder (used once to check that
//! the harness itself fails bad files).

use qb64rust_driver::{FileLoader, dump_cpp, dump_ir, frontend, frontend_with};
use qb64rust_syntax::tree::print;
use std::path::{Path, PathBuf};

const MODES: &[&str] = &["parse-ok", "check-ok", "check-fail", "typed", "ir", "cpp"];

fn frontend_dir() -> PathBuf {
    match std::env::var_os("QB64RUST_FRONTEND_DIR") {
        Some(d) => PathBuf::from(d),
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/frontend"),
    }
}

fn mode_of(bytes: &[u8]) -> Option<&'static str> {
    let first = bytes.split(|&b| b == b'\n').next()?;
    let first = first.strip_suffix(b"\r").unwrap_or(first);
    let rest = first.strip_prefix(b"' TEST: ")?;
    MODES.iter().copied().find(|m| m.as_bytes() == rest)
}

/// Runs one file. Returns the snapshot text for snapshot modes, or an error message.
fn run(name: &str, path: &Path, bytes: Vec<u8>, mode: &str) -> Result<Option<String>, String> {
    // Included files are found next to the including file, then under the compiler root `inc\root` (a folder of
    // its own, so that a file next to the main file is not found through the root); diagnostics name them
    // relative to the test file.
    let root = frontend_dir().join("inc/root");
    let fe = frontend_with(name, bytes.clone(), |file| Box::new(FileLoader::new(&root, file, path)));
    let syntax_errors = fe.parsed.diagnostics().has_errors();
    match mode {
        "parse-ok" => {
            if syntax_errors {
                return Err(format!("syntax errors:\n{}", fe.render_diagnostics()));
            }
            if print(&fe.parsed.main().green, &bytes) != bytes {
                return Err("the tree does not print back to the file".into());
            }
            Ok(None)
        }
        "check-fail" => {
            if !fe.has_errors() {
                return Err("expected at least one error, got none".into());
            }
            Ok(Some(fe.render_diagnostics()))
        }
        _ => {
            if fe.has_errors() {
                return Err(format!("unexpected errors:\n{}", fe.render_diagnostics()));
            }
            Ok(match mode {
                "check-ok" => None,
                "typed" => Some(qb64rust_sema::dump_typed(&fe.program)),
                "ir" => Some(dump_ir(&fe)),
                "cpp" => Some(dump_cpp(&fe)),
                _ => unreachable!(),
            })
        }
    }
}

#[test]
fn frontend_files() {
    let dir = frontend_dir();
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bas")))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no .bas files in {}", dir.display());

    let mut failures = Vec::new();
    let mut snapshots = Vec::new();
    for f in &files {
        let file_name = f.file_name().unwrap().to_string_lossy().to_string();
        let bytes = std::fs::read(f).unwrap();
        let Some(mode) = mode_of(&bytes) else {
            failures.push(format!(
                "{file_name}: no valid `' TEST: <mode>` first line ({})",
                MODES.join(", ")
            ));
            continue;
        };
        match run(&file_name, f, bytes, mode) {
            Ok(Some(snap)) => snapshots.push((f.file_stem().unwrap().to_string_lossy().to_string(), mode, snap)),
            Ok(None) => {}
            Err(e) => failures.push(format!("{file_name} ({mode}): {e}")),
        }
    }
    assert!(failures.is_empty(), "frontend test failures:\n{}", failures.join("\n"));

    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        for (stem, mode, snap) in snapshots {
            insta::assert_snapshot!(format!("{stem}.{mode}"), snap);
        }
    });
}

/// Statements past the error cap are not checked: the parser no longer records their errors, so they may be
/// malformed (`x =` used to panic in the checker).
#[test]
fn error_cap_stops_checking() {
    let src = format!("$CONSOLE:ONLY\n{}", "x =\n".repeat(qb64rust_base::MAX_ERRORS + 5));
    let fe = frontend("cap.bas", src.into_bytes());
    assert!(fe.diagnostics.is_capped());
    assert_eq!(fe.diagnostics.error_count(), qb64rust_base::MAX_ERRORS);
}
