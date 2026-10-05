//! One test per scenario of the `compiler/cli` spec. Tests that build an executable need the QB64pe reference
//! clone and its toolchain; they are `#[ignore]`d and run by hand: `cargo test -p qb64rust-driver --test cli --
//! --ignored`.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SLICE_PROGRAM: &str = "$CONSOLE:ONLY\nx = 1 / 4\nPRINT \"x=\"; x; INSTR(3, \"hello\", \"l\")\nEND\n";

/// A fresh scratch folder for one test (under the system temp folder, never the repo).
fn scratch(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("qb64rust-cli-test-{name}"));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn qb64rust(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qb64rust"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap()
}

#[expect(clippy::disallowed_methods, reason = "the compiler's own messages")]
fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).to_string()
}

#[test]
#[ignore = "needs the QB64pe reference clone"]
fn runners_compile_line() {
    let d = scratch("runner");
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM).unwrap();
    let exe = d.join("prog.exe");
    let o = qb64rust(&d, &["-q", "-m", "-x", "prog.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(exe.is_file());
    assert!(
        !d.join("prog.exe.qb64rust").exists(),
        "build folder deleted after success"
    );
}

#[test]
#[ignore = "needs the QB64pe reference clone"]
fn compile_suites_settings() {
    let d = scratch("settings");
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM).unwrap();
    let exe = d.join("prog - output.exe"); // the suite's naming: spaces in the name
    let o = qb64rust(
        &d,
        &[
            "-f:OptimizeCppProgram=true",
            "-f:StripDebugSymbols=false",
            "-q",
            "-m",
            "-x",
            "prog.bas",
            "-o",
            exe.to_str().unwrap(),
        ],
    );
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(exe.is_file());
}

#[test]
fn unknown_setting() {
    let d = scratch("setting");
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM).unwrap();
    std::fs::write(d.join("prog.exe"), b"stale").unwrap();
    let o = qb64rust(
        &d,
        &[
            "-f:GenerateLicenseFile=true",
            "-q",
            "-m",
            "-x",
            "prog.bas",
            "-o",
            "prog.exe",
        ],
    );
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    assert!(
        out.contains("setting `GenerateLicenseFile` is not supported yet"),
        "{out}"
    );
    assert!(!d.join("prog.exe").exists(), "no executable is left at the output path");
}

#[test]
fn program_with_an_error() {
    let d = scratch("error");
    std::fs::write(d.join("prog.bas"), "$CONSOLE:ONLY\nPRINT 1 +\n").unwrap();
    std::fs::write(d.join("prog.exe"), b"stale").unwrap();
    let o = qb64rust(&d, &["-q", "-m", "-x", "prog.bas", "-o", "prog.exe"]);
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    assert!(out.contains("prog.bas:2:10: error: expected an expression"), "{out}");
    assert!(out.ends_with("1 error\n"), "{out}");
    assert!(!d.join("prog.exe").exists(), "no executable is left at the output path");
}

#[test]
fn unknown_statement() {
    let d = scratch("unknown");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT 1\nCLS\n").unwrap();
    let o = qb64rust(&d, &["p.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(
        stdout(&o)
            .lines()
            .any(|l| l.starts_with("p.bas:3:1: error: not supported yet:")),
        "{}",
        stdout(&o)
    );
}

#[test]
fn summary() {
    let d = scratch("summary");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nCONST c = 2\nPRINT 1 +\nCLS\n").unwrap();
    let o = qb64rust(&d, &["p.bas"]);
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    assert_eq!(out.lines().last(), Some("3 errors (2 not supported yet)"), "{out}");
}

#[test]
fn generate_cpp_only() {
    let d = scratch("z");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let o = qb64rust(&d, &["-z", "p.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    let temp = PathBuf::from(stdout(&o).trim());
    assert!(temp.join("main0.txt").is_file());
    for f in qb64rust_codegen_cpp::FRAGMENTS {
        assert!(temp.join(f).is_file(), "{f}");
    }
    assert!(!d.join("p.exe").exists());
}

#[test]
fn typed_dump() {
    let d = scratch("typed");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\ni% = 1\nPRINT i% + 1\n").unwrap();
    let o = qb64rust(&d, &["--dump", "typed", "p.bas"]);
    assert_eq!(o.status.code(), Some(0));
    let out = stdout(&o);
    assert!(out.contains("Binary Add : I32 (qb I64)"), "{out}");
    assert!(out.contains("Var I : I16"), "{out}");
    let o = qb64rust(&d, &["--dump", "typed", "missing-and-bad.bas"]);
    assert_eq!(o.status.code(), Some(1));
}

/// Every tree is printed, headed by its number and file name (design D9 of `m2-parser-breadth`).
#[test]
fn tree_dump() {
    let d = scratch("tree");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT 1\n").unwrap();
    let o = qb64rust(&d, &["--dump", "tree", "p.bas"]);
    assert_eq!(o.status.code(), Some(0));
    let out = stdout(&o);
    assert!(out.starts_with("tree 0: p.bas\nSourceFile 0..22\n"), "{out}");
}

#[test]
fn no_clone_found() {
    let d = scratch("noclone");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    std::fs::write(d.join("p.exe"), b"stale").unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_qb64rust"))
        .current_dir(&d)
        .env("QB64RUST_QB64PE_ROOT", d.join("no-such-clone"))
        .args(["-q", "p.bas"])
        .output()
        .unwrap();
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    assert!(
        out.contains("--qb64pe-root") && out.contains("QB64RUST_QB64PE_ROOT"),
        "{out}"
    );
    assert!(!d.join("p.exe").exists());
}

/// Scenario "Forced panic" (change `m2-parser-breadth`, D11).
#[test]
fn internal_compiler_error() {
    let d = scratch("ice");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_qb64rust"))
            .current_dir(&d)
            .env("QB64RUST_TEST_PANIC", "1")
            .args(args)
            .output()
            .unwrap()
    };

    std::fs::write(d.join("p.exe"), b"stale").unwrap();
    let o = run(&["-q", "p.bas"]);
    assert_eq!(o.status.code(), Some(3));
    let out = stdout(&o);
    assert!(
        out.starts_with("qb64rust: internal compiler error: QB64RUST_TEST_PANIC=1 is set at "),
        "{out}"
    );
    assert!(out.contains("while compiling p.bas"), "{out}");
    assert!(o.stderr.is_empty(), "the default panic message is replaced");
    assert!(!d.join("p.exe").exists(), "no executable left behind");

    // A dump never touches the executable.
    std::fs::write(d.join("p.exe"), b"kept").unwrap();
    let o = run(&["--dump", "typed", "p.bas"]);
    assert_eq!(o.status.code(), Some(3));
    assert!(d.join("p.exe").exists());
}
