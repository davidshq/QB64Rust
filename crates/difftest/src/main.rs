//! `qb64rust-difftest gen [--check] [--out <dir>] [--types <KEY,...>]`: writes the differential programs (design D2
//! of `m2-numeric-types`) into `tests\differential` (or `<dir>`). With `--check` it writes nothing and fails when a
//! program is missing, differs from the generator or is no longer generated. `--types` (with `--out`) writes the
//! programs for some of the types only (keys as in the printed labels: `INTEGER,LONG,SINGLE`), for a look at part of
//! the type set; such programs need their own recording.

use qb64rust_difftest::{TYPES, existing_programs, generate, generate_for, stale};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!("usage: qb64rust-difftest gen [--check] [--out <dir>] [--types <KEY,...>]");
    ExitCode::from(2)
}

/// `tests\differential`, where `gen` writes by default and tier 1 checks.
fn default_out() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/differential")
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() != Some("gen") {
        return usage();
    }
    let mut check = false;
    let mut out = None;
    let mut types: Option<Vec<String>> = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--check" => check = true,
            "--out" => match args.next() {
                Some(d) => out = Some(PathBuf::from(d)),
                None => return usage(),
            },
            "--types" => match args.next() {
                Some(t) => types = Some(t.split(',').map(|k| k.trim().to_ascii_uppercase()).collect()),
                None => return usage(),
            },
            _ => return usage(),
        }
    }
    if let Some(keys) = &types {
        // A subset never goes into tests\differential, whose files tier 1 checks.
        let (Some(out), false) = (&out, check) else {
            eprintln!("--types needs --out and cannot be combined with --check");
            return ExitCode::from(2);
        };
        if std::fs::canonicalize(out).is_ok_and(|o| std::fs::canonicalize(default_out()).is_ok_and(|d| o == d)) {
            eprintln!("--types cannot write into tests/differential; give another --out");
            return ExitCode::from(2);
        }
        if let Some(k) = keys.iter().find(|k| !TYPES.iter().any(|t| t.key == k.as_str())) {
            let all: Vec<&str> = TYPES.iter().map(|t| t.key).collect();
            eprintln!("unknown type {k}; the types are {}", all.join(","));
            return ExitCode::from(2);
        }
        let files = generate_for(|t| keys.iter().any(|k| k == t.key));
        return write(&files, out);
    }
    let out = out.unwrap_or_else(default_out);
    if check {
        let problems = stale(&out);
        for p in &problems {
            eprintln!("{p}");
        }
        return if problems.is_empty() {
            eprintln!("{} programs are fresh", generate().len());
            ExitCode::SUCCESS
        } else {
            eprintln!("regenerate with: cargo run -p qb64rust-difftest -- gen");
            ExitCode::FAILURE
        };
    }
    write(&generate(), &out)
}

/// Writes the files that differ from what is in `out`; names the programs there that are not generated any more.
fn write(files: &[qb64rust_difftest::File], out: &Path) -> ExitCode {
    let mut written = 0;
    for f in files {
        let path = out.join(&f.path);
        if std::fs::read(&path).is_ok_and(|b| b == f.text.as_bytes()) {
            continue;
        }
        if let Some(dir) = path.parent()
            && let Err(e) = std::fs::create_dir_all(dir)
        {
            eprintln!("{}: {e}", dir.display());
            return ExitCode::FAILURE;
        }
        if let Err(e) = std::fs::write(&path, &f.text) {
            eprintln!("{}: {e}", path.display());
            return ExitCode::FAILURE;
        }
        eprintln!("wrote {}", f.path);
        written += 1;
    }
    for p in existing_programs(out) {
        if !files.iter().any(|f| f.path == p) {
            eprintln!("not generated any more (remove it and its recording): {p}");
        }
    }
    eprintln!("{} programs, {written} written", files.len());
    ExitCode::SUCCESS
}
