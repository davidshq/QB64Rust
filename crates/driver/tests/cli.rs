//! One test per scenario of the `compiler/cli` spec. Tests that build an executable need the QB64pe reference
//! clone and its toolchain; they are `#[ignore]`d and run by hand: `cargo test -p qb64rust-driver --test cli --
//! --ignored`. Every child process runs through `support`, with a deadline and no dialog to wait on; a test that
//! builds takes the reference clone's lock first.

mod support;

use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Duration;

use support::{COMPILER_LIMIT, PROGRAM_LIMIT, run, unattended};

const SLICE_PROGRAM: &str = "$CONSOLE:ONLY\nx = 1 / 4\nPRINT \"x=\"; x; INSTR(3, \"hello\", \"l\")\nEND\n";

/// A fresh scratch folder for one test (under the system temp folder, never the repo), deleted when dropped, also
/// when the test fails. Used as a `Path`.
struct Scratch(tempfile::TempDir);

impl Deref for Scratch {
    type Target = Path;
    fn deref(&self) -> &Path {
        self.0.path()
    }
}

impl AsRef<Path> for Scratch {
    fn as_ref(&self) -> &Path {
        self.0.path()
    }
}

fn scratch(name: &str) -> Scratch {
    Scratch(
        tempfile::Builder::new()
            .prefix(&format!("qb64rust-cli-{name}-"))
            .tempdir()
            .unwrap(),
    )
}

fn compiler() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_qb64rust"));
    cmd.env(support::OBJECTS_LOCK_ENV, support::objects_lock());
    cmd
}

/// A run of the compiler that stops before `make` (a dump, `-z`, a report, an error), so it needs no lock.
fn qb64rust(dir: &Path, args: &[&str]) -> Output {
    run(compiler().current_dir(dir).args(args), None, COMPILER_LIMIT, "compiler")
}

/// A run of the compiler that builds: `make` in the copy of libqb with the clone's toolchain. No lock of the clone, which
/// it only reads (design D5 of `libqb-copy`); `compiler` names the objects' lock.
fn qb64rust_build(dir: &Path, args: &[&str]) -> Output {
    qb64rust(dir, args)
}

/// The executable a build writes for the source `<stem>.bas` with no `-o`: `<stem>.exe` on Windows, `<stem>` on
/// Linux (spec `compiler/cli`).
fn default_exe(stem: &str) -> String {
    format!("{stem}{}", std::env::consts::EXE_SUFFIX)
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
    let o = qb64rust_build(&d, &["-q", "-m", "-x", "prog.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(exe.is_file());
    assert!(
        !d.join("prog.exe.qb64rust").exists(),
        "build folder deleted after success"
    );
}

/// A build changes no tracked file of the reference clone or of the copy of libqb, and nothing git sees in the copy:
/// the libqb objects it may build there are git-ignored (spec `compiler/pipeline`, scenarios "Reference clone
/// unchanged" and "The copy unchanged"). Built with `make`, not from the build cache.
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[cfg(windows)]
fn a_build_changes_nothing_git_sees() {
    let status = |dir: &Path, path: &str| {
        let o = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["status", "--porcelain", "--untracked-files=all", "--", path])
            .output()
            .unwrap();
        assert!(o.status.success());
        o.stdout
    };
    let clone = qb64rust_driver::build::clone_root().expect("the QB64pe reference clone");
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let before = (status(&clone, "."), status(&repo, "libqb"));
    let d = scratch("unchanged");
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM).unwrap();
    let o = run(
        compiler()
            .current_dir(&d)
            .env_remove("QB64RUST_BUILD_CACHE")
            .args(["-q", "prog.bas", "-o", "prog.exe"]),
        None,
        COMPILER_LIMIT,
        "compiler",
    );
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(before, (status(&clone, "."), status(&repo, "libqb")));
}

#[test]
#[ignore = "needs the QB64pe reference clone"]
fn compile_suites_settings() {
    let d = scratch("settings");
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM).unwrap();
    let exe = d.join("prog - output.exe"); // the suite's naming: spaces in the name
    let o = qb64rust_build(
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

/// A slice program built with `--debug` prints the output recorded from `qb64pe.exe` (spec `compiler/cli`, "Debug
/// build", scenario "Same program"): `-g` without `-O2` changes no value it prints.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_build_same_output() {
    let d = scratch("debug-same");
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/corpus/slice");
    // Ends with `SYSTEM`: an `END` would wait for a key.
    std::fs::copy(corpus.join("s09_functions.bas"), d.join("p.bas")).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["--debug", "-q", "-m", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let r = run(unattended(&exe).current_dir(&d), None, PROGRAM_LIMIT, "program");
    assert_eq!(r.status.code(), Some(0), "{}", stdout(&r));
    let recorded = text(&std::fs::read(corpus.join("s09_functions.output")).unwrap()).replace("\r\n", "\n");
    assert_eq!(stdout(&r).replace("\r\n", "\n"), recorded);
}

/// `-f:StripDebugSymbols=false` leaves the local symbols in the executable, the default strips them (spec
/// `compiler/cli`, scenario "Unstripped on request"), seen with the toolchain's `nm`. The `Makefile`'s
/// `objcopy --strip-unneeded` keeps the global ones (`QBMAIN` is listed either way), so a local one (type `t`) tells.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn strip_debug_symbols_false_keeps_symbols() {
    let d = scratch("strip");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let has_local = |settings: &[&str]| {
        let exe = d.join("p.exe");
        let mut args = settings.to_vec();
        args.extend(["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
        let o = qb64rust_build(&d, &args);
        assert!(o.status.success(), "{}", stdout(&o));
        let nm = run(
            Command::new(support::toolchain("nm.exe")).arg(&exe),
            None,
            COMPILER_LIMIT,
            "toolchain",
        );
        let listed = stdout(&nm);
        // On Linux `objcopy --strip-unneeded` leaves an executable no symbols at all, as with QB64pe's Linux build
        // (measured, design "Measured" of `m7-linux`).
        if cfg!(windows) || settings.contains(&"-f:StripDebugSymbols=false") {
            assert!(listed.contains(" T _Z6QBMAINPv"), "the global symbol stays: {listed}");
        }
        listed.lines().any(|l| l.split_whitespace().nth(1) == Some("t"))
    };
    assert!(has_local(&["-f:StripDebugSymbols=false"]));
    assert!(!has_local(&[]));
    assert!(!has_local(&["-f:StripDebugSymbols=true"]));
}

/// With a build cache, a debug build is no hit for a plain one's entry, and a second debug build is a hit (spec
/// `compiler/cli`, "Debug build", scenario "The build cache").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_build_is_cached_apart() {
    let d = scratch("debug-cache");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let cache = d.join("cache");
    let build = |debug: bool| {
        let mut cmd = compiler();
        cmd.current_dir(&d)
            .env("QB64RUST_BUILD_CACHE", &cache)
            .args(["-x", "p.bas", "-o", "p.exe"]);
        if debug {
            cmd.arg("--debug");
        }
        let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
        assert!(o.status.success(), "{}", stdout(&o));
        stdout(&o).contains("the same build is in")
    };
    assert!(!build(false), "the first plain build");
    assert!(!build(true), "a debug build after a plain one builds");
    assert!(build(true), "a second debug build is a hit");
    assert!(build(false), "the plain build's entry is kept");
}

/// A build is deterministic (`DECISIONS.md` 2026-10-27): the same program built twice with no cache is the same bytes,
/// so a cache of test passes keyed on the executable holds across a rebuild. A program that calls `_COMPILETIME$` is
/// the exception: it holds the time of its build.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn builds_are_deterministic() {
    let d = scratch("deterministic");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let build = || {
        let mut cmd = compiler();
        cmd.current_dir(&d)
            .env_remove("QB64RUST_BUILD_CACHE")
            .args(["-q", "-x", "p.bas", "-o", "p.exe"]);
        let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
        assert!(o.status.success(), "{}", stdout(&o));
        std::fs::read(d.join("p.exe")).unwrap()
    };
    let first = build();
    assert!(first == build(), "two builds of one program differ");
}

/// A build with the fixes on is not the cached build with them off (spec `compiler/fixes`, "Built again with a fix
/// turned on"), though this program's C++ is the same either way.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn fixes_build_is_cached_apart() {
    let d = scratch("fixes-cache");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let cache = d.join("cache");
    let build = |fixes: &str| {
        let mut cmd = compiler();
        cmd.current_dir(&d)
            .env("QB64RUST_BUILD_CACHE", &cache)
            .args(["-x", "--fixes", fixes, "p.bas", "-o", "p.exe"]);
        let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
        assert!(o.status.success(), "{}", stdout(&o));
        stdout(&o).contains("the same build is in")
    };
    assert!(!build("off"), "the first build");
    assert!(!build("on"), "a build with the fixes on after one with them off builds");
    assert!(build("on"), "a second build with them on is a hit");
    assert!(build("off"), "the first build's entry is kept");
}

/// Builds `program` as `<name>.bas` in `d` with `--debug`, by absolute paths (the `#line` directives then name the
/// file by its full path, as the proxy will launch it); gives the executable.
fn debug_build(d: &Path, name: &str, program: &str) -> PathBuf {
    let bas = d.join(format!("{name}.bas"));
    std::fs::write(&bas, program).unwrap();
    let exe = d.join(format!("{name}.exe"));
    let o = qb64rust_build(
        d,
        &[
            "--debug",
            "-q",
            "-x",
            bas.to_str().unwrap(),
            "-o",
            exe.to_str().unwrap(),
        ],
    );
    assert!(o.status.success(), "{}", stdout(&o));
    exe
}

/// Where `lldb` stopped, in order: frame 0's function and `file:line:column` (`QBMAIN steps.bas:3:1`; a function
/// without line information alone, `error`). Printed to the test's stderr, so that `--nocapture` shows the sequence.
/// A `bt` marks its frame 0 `* frame #0:`, a stop does not. Off Windows the column is given as 1: `g++` writes the
/// column of the C++ statement (`steps.bas:5:16`), which `clang` on Windows does not, and a step goes by line either
/// way (measured, task 4.2 of `m7-linux`; without columns, `-gno-column-info`, `lldb` 18 stepped differently).
fn stops(out: &str) -> Vec<String> {
    let stops: Vec<String> = out
        .lines()
        .filter(|l| l.trim_start().starts_with("frame #0:"))
        .filter_map(|l| {
            let frame = l.split('`').nth(1)?;
            let function = frame.split('(').next()?;
            Some(match frame.rsplit_once(" at ") {
                Some((_, at)) => format!("{function} {}", column_1(at.trim())),
                None => function.to_string(),
            })
        })
        .collect();
    eprintln!("lldb stops: {stops:?}");
    stops
}

/// `file:line:column` with the column 1 off Windows (see [`stops`]); anything else as it is.
fn column_1(at: &str) -> String {
    if cfg!(windows) {
        return at.to_string();
    }
    let number = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    match at.rsplit_once(':') {
        Some((head, column)) if number(column) && head.rsplit_once(':').is_some_and(|(_, line)| number(line)) => {
            format!("{head}:1")
        }
        _ => at.to_string(),
    }
}

/// The values `lldb`'s `p` printed, in order, without their types (`(int32_t) 7` is `7`).
fn values(out: &str) -> Vec<String> {
    out.lines()
        .filter(|l| l.starts_with('(') && !l.starts_with("(lldb)"))
        .filter_map(|l| l.split_once(") ").map(|(_, v)| v.to_string()))
        .collect()
}

/// A breakpoint by `.bas` line resolves in `QBMAIN` and stops there, once per pass of the loop it is in; the main
/// module's variables read through their C names (spec `compiler/cli`, "Debug build", scenario "Line information";
/// the probe of `study\36` §2).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_stops_at_a_line() {
    let d = scratch("lldb-line");
    let program = "$CONSOLE:ONLY\n' Line 11 is in a FOR loop.\nDIM SHARED total AS LONG\nTYPE pt\n    x AS INTEGER\n\
                   END TYPE\nDIM p AS pt\np.x = 1\ns$ = \"hi\"\nFOR i = 1 TO 2\n    total = total + i\nNEXT\n\
                   PRINT total; s$; p.x\nSYSTEM\n";
    let exe = debug_build(&d, "dbg", program);
    let out = support::lldb(
        &exe,
        &[
            "breakpoint set -f dbg.bas -l 11",
            "run",
            "p *__LONG_TOTAL",
            "continue",
            "p *__LONG_TOTAL",
        ],
    );
    assert!(out.contains("Breakpoint 1: where = dbg.exe`QBMAIN(void*) + "), "{out}");
    let at = out
        .lines()
        .find_map(|l| {
            l.strip_prefix("Breakpoint 1: where = ")?
                .split_once(" at ")?
                .1
                .split_once(", address = ")
        })
        .map(|(at, _)| column_1(at));
    assert_eq!(at.as_deref(), Some("dbg.bas:11:1"), "{out}");
    assert_eq!(stops(&out), ["QBMAIN dbg.bas:11:1", "QBMAIN dbg.bas:11:1"], "{out}");
    assert_eq!(values(&out), ["0", "1"], "{out}");
}

/// `next` from line 2 stops once on each BASIC line: a line of two statements is one stop, and a one-line `FOR`
/// loop is one stop for all its passes (`lldb` steps until the line changes, and the jump back stays on the line).
/// A breakpoint on that line stops there once, as the loop is entered.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_next_steps_by_line() {
    let d = scratch("lldb-next");
    let program = "$CONSOLE:ONLY\na = 1\nb = 2\nc = 3: d = 4\nFOR i = 1 TO 3: t = t + i: NEXT\ne = 5\n\
                   PRINT a; b; c; d; e; t\nSYSTEM\n";
    let exe = debug_build(&d, "steps", program);
    let mut commands = vec!["breakpoint set -f steps.bas -l 2", "run"];
    commands.extend(["next"; 6]);
    let out = support::lldb(&exe, &commands);
    let lines: Vec<String> = (2..=8).map(|l| format!("QBMAIN steps.bas:{l}:1")).collect();
    assert_eq!(stops(&out), lines, "{out}");
    let out = support::lldb(
        &exe,
        &[
            "breakpoint set -f steps.bas -l 5",
            "run",
            "continue",
            "continue",
            "continue",
        ],
    );
    // Once as the loop is entered, not once per pass: the jump back lands after the breakpoint's address.
    assert_eq!(stops(&out), ["QBMAIN steps.bas:5:1"], "{out}");
}

/// `next` over a `CALL` goes to the next line; `step` enters the SUB at its header line (the prologue's `#line`) and
/// may pass through inline functions of libqb's headers, which have line information; `finish` returns to the
/// `CALL`'s line; `next` at the end of the SUB stops at `END SUB` and at the caller's line before the next one.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_next_over_call_and_into_sub() {
    let d = scratch("lldb-call");
    let program = "$CONSOLE:ONLY\nx = 1\nCALL show(x)\ny = 2\nPRINT y\nSYSTEM\nSUB show (n)\n    n = n + 1\n\
                   PRINT n\nEND SUB\n";
    let exe = debug_build(&d, "call", program);
    let out = support::lldb(&exe, &["breakpoint set -f call.bas -l 3", "run", "next", "next"]);
    assert_eq!(
        stops(&out),
        ["QBMAIN call.bas:3:1", "QBMAIN call.bas:4:1", "QBMAIN call.bas:5:1"],
        "{out}"
    );
    // With g++ on Linux libqb's inline `is_error_pending` has line information in the program's object, and `step`
    // enters it on the way (two stops in `error_handle.h`; the adapter steps out of it), so two steps more.
    let mut commands = vec!["breakpoint set -f call.bas -l 3", "run"];
    commands.extend(["step"; 4]);
    if !cfg!(windows) {
        commands.extend(["step"; 2]);
    }
    commands.extend(["finish", "next"]);
    let out = support::lldb(&exe, &commands);
    let s = stops(&out);
    assert!(s[1].starts_with("SUB_SHOW call.bas:7:"), "the header line first: {out}");
    assert!(s.contains(&"SUB_SHOW call.bas:8:1".to_string()), "{out}");
    assert!(s[s.len() - 2].starts_with("QBMAIN call.bas:3:"), "finish: {out}");
    assert_eq!(s[s.len() - 1], "QBMAIN call.bas:4:1", "{out}");
    let out = support::lldb(
        &exe,
        &["breakpoint set -f call.bas -l 9", "run", "next", "next", "next"],
    );
    let s = stops(&out);
    assert_eq!(s[0], "SUB_SHOW call.bas:9:1", "{out}");
    assert!(s[1].starts_with("SUB_SHOW call.bas:10:"), "END SUB: {out}");
    assert!(s[2].starts_with("QBMAIN call.bas:3:"), "back at the CALL: {out}");
    assert_eq!(s[3], "QBMAIN call.bas:4:1", "{out}");
}

/// `step` into a FUNCTION of an included file stops in that file, at its header line first: the header and `END`
/// lines are written under the procedure's own file, as its statements are. The steps between pass through the
/// lines of `data1.txt`, the procedure's data fragment, which has no `#line` of its own; they are printed.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_step_into_an_included_function() {
    let d = scratch("lldb-include");
    std::fs::write(
        d.join("lib.bi"),
        "FUNCTION twice& (k AS LONG)\n    twice& = 2 * k\nEND FUNCTION\n",
    )
    .unwrap();
    let exe = debug_build(
        &d,
        "inc",
        "$CONSOLE:ONLY\nx& = twice&(3)\nPRINT x&\nSYSTEM\n'$INCLUDE:'lib.bi'\n",
    );
    let mut commands = vec!["breakpoint set -f inc.bas -l 2", "run"];
    commands.extend(["step"; 16]);
    let out = support::lldb(&exe, &commands);
    let s = stops(&out);
    assert_eq!(s[0], "QBMAIN inc.bas:2:1", "{out}");
    // On Windows the header is the first stop in the function; with g++ the lines of `data1.txt` come first.
    let first = s.iter().find(|s| s.starts_with("FUNC_TWICE lib.bi:"));
    assert!(
        first.is_some_and(|f| f.starts_with("FUNC_TWICE lib.bi:1:")),
        "the header, in the included file: {out}"
    );
    if cfg!(windows) {
        assert!(s[1].starts_with("FUNC_TWICE lib.bi:1:"), "{out}");
    }
    assert!(s.contains(&"FUNC_TWICE lib.bi:2:1".to_string()), "{out}");
    assert!(s.iter().all(|s| !s.starts_with("FUNC_TWICE inc.bas")), "{out}");
}

/// A `RESUME` retry: `next` over the raising line goes to the next line, the handler and the retry running inside
/// it; a breakpoint in the handler stops in a second `QBMAIN` frame, entered from libqb's `evnt` below the raising
/// line; the stops of `next` from there through the `RESUME` are printed.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_resume_retry() {
    let d = scratch("lldb-retry");
    // Error 5 (`CHR$(-1)`), not 11: a division by zero is critical and reaches no handler.
    let program = "$CONSOLE:ONLY\nON ERROR GOTO h\nn = -1\ns$ = CHR$(n)\nPRINT s$\nSYSTEM\nh:\nn = 65\nRESUME\n";
    let exe = debug_build(&d, "retry", program);
    let out = support::lldb(
        &exe,
        &["breakpoint set -f retry.bas -l 3", "run", "next", "next", "next"],
    );
    let lines: Vec<String> = (3..=6).map(|l| format!("QBMAIN retry.bas:{l}:1")).collect();
    assert_eq!(stops(&out), lines, "{out}");
    let out = support::lldb(
        &exe,
        &[
            "breakpoint set -f retry.bas -l 8",
            "run",
            "bt 4",
            "next",
            "next",
            "next",
        ],
    );
    let s = stops(&out);
    assert_eq!(s[0], "QBMAIN retry.bas:8:1", "{out}");
    assert!(
        out.contains("frame #1: ") && out.contains("`evnt(linenumber=4"),
        "{out}"
    );
    assert!(
        out.contains("frame #2: ") && out.contains("`QBMAIN(unused=") && out.contains(" at retry.bas:4:"),
        "{out}"
    );
    assert!(s.contains(&"QBMAIN retry.bas:9:1".to_string()), "{out}");
}

/// Each storage's value read by the C expression the symbol file's `c_name`, `reach` and layouts give: a main-module
/// variable, a `STATIC`, a local and a parameter of a SUB through their pointers, an element of a static array
/// through its descriptor, a `STRING` through its `qbs`, the members of a user type at their offsets.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_reads_each_storage() {
    let d = scratch("lldb-reads");
    let program = "$CONSOLE:ONLY\nTYPE pt\n    x AS INTEGER\n    nm AS STRING * 4\nEND TYPE\nDIM SHARED total AS LONG\n\
                   DIM a(1 TO 3) AS SINGLE\nDIM s AS STRING\nDIM p AS pt\n\
                   total = 7: a(2) = 1.5: s = \"hi\": p.x = 3: p.nm = \"abcd\"\nCALL show(total)\nSYSTEM\n\
                   SUB show (n AS LONG)\n    STATIC calls AS INTEGER\n    x = n + 1\n    calls = calls + 1\n\
                   PRINT x\nEND SUB\n";
    let exe = debug_build(&d, "reads", program);
    let info: serde_json::Value =
        serde_json::from_str(&text(&std::fs::read(d.join("reads.exe.debug.json")).unwrap())).unwrap();
    let var = |name: &str| {
        let v = info["variables"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["name"] == name)
            .unwrap_or_else(|| panic!("{name} in the symbol file"));
        (
            v["c_name"].as_str().unwrap().to_string(),
            v["reach"].as_str().unwrap().to_string(),
        )
    };
    let pointer = |name: &str, reach: &str| {
        let (c, r) = var(name);
        assert_eq!(r, reach, "{name}");
        format!("p *{c}")
    };
    let (a, reach_a) = var("A");
    assert_eq!(reach_a, "descriptor");
    let layouts = &info["layouts"];
    let data = layouts["array"]["data"].as_u64().unwrap();
    // One dimension (n = 1, k = 0): its lower bound is at the first slot.
    let lower = layouts["array"]["dimensions"]["first_slot"].as_u64().unwrap();
    let (s, reach_s) = var("S");
    assert_eq!(reach_s, "string");
    let (p, reach_p) = var("P");
    assert_eq!(reach_p, "bytes");
    let pt = &info["types"][0]["members"];
    let (x_at, nm_at) = (pt[0]["offset"].as_u64().unwrap(), pt[1]["offset"].as_u64().unwrap());
    let commands = [
        "breakpoint set -f reads.bas -l 17".to_string(),
        "run".to_string(),
        pointer("TOTAL", "pointer"),
        pointer("CALLS", "pointer"),
        pointer("X", "pointer"),
        pointer("N", "param-pointer"),
        format!("p ((float*){a}[{data}])[2 - {a}[{lower}]]"),
        // `lldb` 20 on Linux cannot read a `qbs` from memory (each member "extracting data from value failed", `len`
        // 0; measured, task 4.2 of `m7-linux`), so there `len` is read at its offset of the symbol file, as the
        // adapter reads it.
        if cfg!(windows) {
            format!("p {s}->len")
        } else {
            let len = layouts["qbs"]["len"].as_u64().unwrap();
            format!("p *(int32_t*)((char*){s} + {len})")
        },
        format!("p *(char(*)[2]){s}->chr"),
        format!("p *(short*)((char*){p} + {x_at})"),
        format!("p *(char(*)[4])((char*){p} + {nm_at})"),
    ];
    let commands: Vec<&str> = commands.iter().map(String::as_str).collect();
    let out = support::lldb(&exe, &commands);
    assert_eq!(stops(&out), ["SUB_SHOW reads.bas:17:1"], "{out}");
    assert_eq!(
        values(&out),
        ["7", "1", "8", "7", "1.5", "2", "\"hi\"", "3", "\"abcd\""],
        "{out}"
    );
}

/// A function breakpoint on libqb's `error` with a condition on its code stops at error 5, with the program's line
/// in the frame below it; libqb has no debug information, so the code is read from its register (the first argument:
/// `ecx` on x64 Windows, `edi` on Linux). With a condition on another code the program runs to its end.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_stops_at_error() {
    let d = scratch("lldb-error");
    let exe = debug_build(&d, "err", "$CONSOLE:ONLY\nx = 1\nERROR 5\nPRINT x\nSYSTEM\n");
    let reg = qb64rust_dap::FIRST_ARG;
    let out = support::lldb(
        &exe,
        &[
            &format!("breakpoint set -n error -c \"(int){reg} == 5\""),
            "run",
            "bt 3",
        ],
    );
    assert!(out.contains("Breakpoint 1: where = err.exe`error(int)"), "{out}");
    assert_eq!(stops(&out), ["error"], "{out}");
    assert!(
        out.lines().any(|l| l.contains("frame #1: ")
            && l.contains("`QBMAIN(unused=")
            && l.rsplit_once(" at ")
                .is_some_and(|(_, at)| column_1(at.trim()) == "err.bas:3:1")),
        "{out}"
    );
    let out = support::lldb(
        &exe,
        &[&format!("breakpoint set -n error -c \"(int){reg} == 6\""), "run"],
    );
    assert!(stops(&out).is_empty(), "{out}");
    assert!(out.contains("exited with status = "), "{out}");
}

/// A debug build writes `<exe>.debug.json` beside the executable (spec `compiler/debug-info`, "Written beside the
/// executable"), into another folder than the source's, and the file is what `--dump debug-info` prints but for the
/// main file's path.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_build_writes_symbol_file() {
    let d = scratch("debug-file");
    let program = "$CONSOLE:ONLY\nDIM SHARED total AS LONG\nCALL show(2)\nSYSTEM\nSUB show (n AS LONG)\n\
                   total = total + n\nEND SUB\n";
    std::fs::write(d.join("p.bas"), program).unwrap();
    std::fs::create_dir(d.join("out")).unwrap();
    let o = qb64rust_build(&d, &["--debug", "-q", "-x", "p.bas", "-o", "out/p.exe"]);
    assert!(o.status.success(), "{}", stdout(&o));
    let file = d.join("out/p.exe.debug.json");
    let text = text(&std::fs::read(&file).expect("the symbol file"));
    let json: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    assert_eq!(json["version"], 1);
    assert_eq!(json["procedures"][0]["c_name"], "SUB_SHOW");
    let dump = stdout(&qb64rust(&d, &["--dump", "debug-info", "p.bas"]));
    assert_eq!(text, dump);
}

/// A build without `--debug` writes no symbol file and removes one an earlier build left (spec `compiler/debug-info`,
/// "Not written otherwise").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn plain_build_removes_symbol_file() {
    let d = scratch("debug-stale");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let file = d.join("p.exe.debug.json");
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", "p.exe"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(!file.exists(), "a plain build writes none");
    std::fs::write(&file, b"stale").unwrap();
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", "p.exe"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(!file.exists(), "the earlier build's file is removed");
}

/// A debug build that fails removes the symbol file an earlier build left, with its executable (spec
/// `compiler/debug-info`, "A failed build").
#[test]
fn failed_build_removes_symbol_file() {
    let d = scratch("debug-failed");
    std::fs::write(d.join("p.bas"), "PRINT (\n").unwrap();
    std::fs::write(d.join("p.exe"), b"stale").unwrap();
    let file = d.join("p.exe.debug.json");
    std::fs::write(&file, b"stale").unwrap();
    let o = qb64rust(&d, &["--debug", "-q", "-x", "p.bas", "-o", "p.exe"]);
    assert!(!o.status.success(), "{}", stdout(&o));
    assert!(!d.join("p.exe").exists(), "the executable is removed");
    assert!(!file.exists(), "its symbol file is removed");
}

#[test]
fn unknown_setting() {
    let d = scratch("setting");
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM).unwrap();
    std::fs::write(d.join("prog.exe"), b"stale").unwrap();
    let o = qb64rust(
        &d,
        &[
            "-f:MaxCompilerProcesses=2",
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
        out.contains("setting `MaxCompilerProcesses` is not supported yet"),
        "{out}"
    );
    assert!(!d.join("prog.exe").exists(), "no executable is left at the output path");
    let o = qb64rust(
        &d,
        &["-f:GenerateLicenseFile=maybe", "-q", "-x", "prog.bas", "-o", "prog.exe"],
    );
    assert_eq!(o.status.code(), Some(1));
    assert!(stdout(&o).contains("needs `true` or `false`"), "{}", stdout(&o));
}

/// `-f:GenerateLicenseFile=true` writes `<exe>.license.txt` beside the executable, naming the parts of libqb the
/// program links (spec `compiler/cli`, "The licence file"; design D6 of `m3-asserts-debug-zlib`); without the
/// setting, or with `false`, none is written and one an earlier build left stays, as `qb64pe` does. Through the
/// build cache the file is kept and copied with the executable, and an entry without it is no hit for a build that
/// asks for it.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn license_file() {
    let d = scratch("license");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\nPRINT LEN(_INFLATE$(_DEFLATE$(\"abc\")))\nSYSTEM\n",
    )
    .unwrap();
    let cache = d.join("cache");
    let build = |settings: &[&str], cached: bool| {
        let mut args = settings.to_vec();
        args.extend(["-q", "-m", "-x", "p.bas", "-o", "p.exe"]);
        let mut cmd = compiler();
        cmd.current_dir(&d).args(&args);
        if cached {
            cmd.env("QB64RUST_BUILD_CACHE", &cache);
        }
        let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
        assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
        assert!(d.join("p.exe").is_file());
    };
    let license = d.join("p.exe.license.txt");
    build(&["-f:GenerateLicenseFile=true"], false);
    let text = std::fs::read(&license).unwrap();
    let has = |what: &str| text.windows(what.len()).any(|w| w == what.as_bytes());
    // The runtime's own licence, and that of the data-processing part (miniz), which this program links.
    assert!(has("License of QB64-PE runtime"));
    assert!(has("miniz"));
    // A build without the setting leaves the file an earlier build left, as `qb64pe` does.
    build(&[], false);
    assert_eq!(std::fs::read(&license).unwrap(), text);
    build(&["-f:GenerateLicenseFile=false"], false);
    assert_eq!(std::fs::read(&license).unwrap(), text);
    std::fs::remove_file(&license).unwrap();
    build(&[], false);
    assert!(!license.exists());
    // Through the cache: an entry made without the file is no hit for a build with it, and the one made with it
    // brings its file along.
    build(&[], true);
    build(&["-f:GenerateLicenseFile=true"], true);
    assert_eq!(std::fs::read(&license).unwrap(), text);
    std::fs::remove_file(&license).unwrap();
    std::fs::remove_file(d.join("p.exe")).unwrap();
    build(&["-f:GenerateLicenseFile=true"], true);
    assert_eq!(std::fs::read(&license).unwrap(), text);
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

/// Spec scenarios "Default root" and `--include-root` (design D9): a file found only under the root given is
/// included; without `--include-root` the root is the executable's folder, where it is not, and the include is
/// "not found"; the working directory plays no part (the file is also there, and not taken).
#[test]
fn include_root() {
    let d = scratch("include-root");
    let main = d.join("main");
    let root = d.join("root");
    std::fs::create_dir_all(root.join("extra")).unwrap();
    std::fs::create_dir_all(&main).unwrap();
    std::fs::write(root.join("extra/x.bi"), "x = 1\n").unwrap();
    std::fs::create_dir_all(d.join("extra")).unwrap();
    std::fs::write(d.join("extra/x.bi"), "x = 2\n").unwrap();
    std::fs::write(main.join("p.bas"), "$CONSOLE:ONLY\n'$INCLUDE:'extra/x.bi'\nPRINT x\n").unwrap();
    let p = main.join("p.bas");
    let o = qb64rust(&d, &["--dump", "typed", p.to_str().unwrap(), "--include-root", "root"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).contains("Int 1"), "{}", stdout(&o));
    let o = qb64rust(&d, &["--dump", "typed", p.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(1));
    assert!(
        stdout(&o).contains("included file `extra/x.bi` not found"),
        "{}",
        stdout(&o)
    );
}

#[test]
fn unknown_statement() {
    let d = scratch("unknown");
    // `VARSEG` of an element of a `_BIT` array stands for a construct not compiled yet; `FILES` did until
    // `m4-builtins-remaining`, `SUB _GL` until `m4-sub-gl`, `FOR` over an element of a member array until
    // `m4-left-marked`.
    std::fs::write(d.join("p.bas"), format!("{MARKED_PREAMBLE}{MARKED}")).unwrap();
    let o = qb64rust(&d, &["p.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(
        stdout(&o)
            .lines()
            .any(|l| l.starts_with("p.bas:3:14: error: not supported yet:")),
        "{}",
        stdout(&o)
    );
}

/// The `DIM` that makes `PRINT VARSEG(bb(1))` a statement not compiled yet (`VARSEG` of an element of a `_BIT`
/// array; not measured).
const MARKED_PREAMBLE: &str = "$CONSOLE:ONLY\nDIM bb(2) AS _BIT\n";
const MARKED: &str = "PRINT VARSEG(bb(1))\n";

#[test]
fn summary() {
    let d = scratch("summary");
    let text = format!("{MARKED_PREAMBLE}{MARKED}PRINT 1 +\n{MARKED}");
    std::fs::write(d.join("p.bas"), text).unwrap();
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

/// `--dump debug-info` prints the debug symbol file without building (spec `compiler/debug-info`, "Dump of a valid
/// program"), with the main file's path as given; a program with an error exits with 1.
#[test]
fn debug_info_dump() {
    let d = scratch("debug-dump");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\nDIM SHARED total AS LONG\nPRINT total\n",
    )
    .unwrap();
    let o = qb64rust(&d, &["--dump", "debug-info", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    let json: serde_json::Value = serde_json::from_str(&stdout(&o)).expect("JSON");
    assert_eq!(json["version"], 1);
    assert_eq!(json["files"][0]["path"], "p.bas");
    assert_eq!(json["variables"][0]["c_name"], "__LONG_TOTAL");
    assert!(!d.join("p.exe").exists(), "nothing is built");
    std::fs::write(d.join("bad.bas"), "PRINT 1 +\n").unwrap();
    let o = qb64rust(&d, &["--dump", "debug-info", "bad.bas"]);
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

/// A QB64pe folder named by the variable that is not one fails the build, naming the option and the variable, before
/// the toolchain is looked for (design D5 of `m6-toolchain`); the stale executable is gone.
#[test]
fn a_named_folder_that_is_not_qb64pe() {
    let d = scratch("noclone");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    std::fs::write(d.join(default_exe("p")), b"stale").unwrap();
    let o = run(
        compiler()
            .current_dir(&d)
            .env("QB64RUST_QB64PE_ROOT", d.join("no-such-clone"))
            .args(["-q", "p.bas"]),
        None,
        COMPILER_LIMIT,
        "compiler",
    );
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    assert!(
        out.contains("--qb64pe-root") && out.contains("QB64RUST_QB64PE_ROOT"),
        "{out}"
    );
    assert!(!d.join(default_exe("p")).exists());
}

/// A build without the copy of libqb fails naming its option and variable; `-z` needs no copy (spec `compiler/cli`,
/// scenario "No copy of libqb found"). The clone and the toolchain are stand-in folders: this test builds nothing.
#[test]
fn no_libqb_copy_found() {
    let d = scratch("nolibqb");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    std::fs::create_dir_all(d.join("clone/internal/c")).unwrap();
    std::fs::write(d.join("clone/internal/c/qbx.cpp"), "").unwrap();
    fake_toolchain(&d.join("tools"));
    std::fs::create_dir_all(d.join("empty")).unwrap();
    let compile = |extra: &[&str]| {
        let mut cmd = compiler();
        cmd.current_dir(&d)
            .args([
                "-q",
                "--qb64pe-root",
                "clone",
                "--toolchain-root",
                "tools",
                "--libqb-root",
                "empty",
            ])
            .args(extra)
            .arg("p.bas");
        run(&mut cmd, None, COMPILER_LIMIT, "compiler")
    };
    let o = compile(&[]);
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    assert!(
        out.contains("--libqb-root") && out.contains("QB64RUST_LIBQB_ROOT"),
        "{out}"
    );
    assert_eq!(compile(&["-z"]).status.code(), Some(0));
}

/// `--version` prints the release's version (spec `compiler/cli`, scenario "Version").
#[test]
fn version() {
    let d = scratch("version");
    let o = qb64rust(&d, &["--version"]);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(stdout(&o), format!("qb64rust {}\n", env!("CARGO_PKG_VERSION")));
}

/// `--help` and `-h` print the usage, which names `toolchain install` (spec `compiler/cli`, scenario "Help").
#[test]
fn help() {
    let d = scratch("help");
    for word in ["--help", "-h"] {
        let o = qb64rust(&d, &[word]);
        assert_eq!(o.status.code(), Some(0), "{word}");
        let out = stdout(&o);
        assert!(
            out.starts_with("usage: qb64rust") && out.contains("toolchain install"),
            "{word}: {out}"
        );
    }
}

/// `--version` with another argument is a usage error (spec `compiler/cli`, scenario "With other arguments").
#[test]
#[expect(clippy::disallowed_methods, reason = "the compiler's own messages")]
fn version_with_other_arguments() {
    let d = scratch("version-args");
    let o = qb64rust(&d, &["--version", "p.bas"]);
    assert_eq!(o.status.code(), Some(1));
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(
        err.contains("`--version` takes no other argument") && err.contains("usage: qb64rust"),
        "{err}"
    );
}

/// `--list-fixes` prints one line per fix (spec `compiler/fixes`, "The list"); with another argument it is a usage
/// error (spec `compiler/cli`, "List with another argument").
#[test]
#[expect(clippy::disallowed_methods, reason = "the compiler's own messages")]
fn fixes_list() {
    let d = scratch("fixes-list");
    let o = qb64rust(&d, &["--list-fixes"]);
    assert_eq!(o.status.code(), Some(0));
    let out = stdout(&o);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 8, "{out}");
    assert!(lines[0].starts_with("resume-after-block "), "{out}");
    assert!(lines[7].starts_with("bit-number-check "), "{out}");
    assert!(lines.iter().all(|l| l.split_once("  ").is_some()), "{out}");
    let o = qb64rust(&d, &["--list-fixes", "p.bas"]);
    assert_eq!(o.status.code(), Some(1));
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(err.contains("usage: qb64rust"), "{err}");
}

/// The `Settings:` line of `--dump ir` for `p.bas` in `d` with the arguments `args` before it.
fn dumped_settings(d: &Path, args: &[&str]) -> String {
    let mut all = args.to_vec();
    all.extend(["--dump", "ir", "p.bas"]);
    let o = qb64rust(d, &all);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(0), "{out}");
    out.lines().next().unwrap_or("").to_string()
}

/// The fixes set by `qb64rust.toml` and the command line (spec `compiler/fixes`, "Master switch with an override",
/// "Off overrides the file", "One fix on", "No file"), as the IR dump names them; `-w` names the file used.
#[test]
fn fixes_from_the_file_and_the_command_line() {
    let d = scratch("fixes-set");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT 1\n").unwrap();
    assert_eq!(dumped_settings(&d, &[]), "Settings: console, no window", "no file");
    std::fs::write(
        d.join("qb64rust.toml"),
        "[fixes]\nall = true\nresume-after-block = false\n",
    )
    .unwrap();
    let most = dumped_settings(&d, &[]);
    assert!(
        most.contains("fixes dim-empty-error static-local-dynamic") && !most.contains("resume-after-block"),
        "{most}"
    );
    assert_eq!(
        dumped_settings(&d, &["--fixes", "off"]),
        "Settings: console, no window",
        "off overrides the file"
    );
    assert_eq!(
        dumped_settings(&d, &["--fixes", "off", "--fix", "whole-array-value=on"]),
        "Settings: console, no window, fixes whole-array-value",
        "one fix on"
    );
    assert!(
        dumped_settings(&d, &["--fix", "resume-after-block=on"]).contains("fixes resume-after-block dim-empty-error"),
        "--fix over the file"
    );
    let o = qb64rust(&d, &["-w", "--dump", "ir", "p.bas"]);
    let out = stdout(&o);
    assert!(out.contains("fixes from ") && out.contains("qb64rust.toml"), "{out}");
}

/// A bad settings file fails the compile with its name, line and the known fixes, and exit status 1 (spec
/// `compiler/fixes`, "A misspelt name"); `--fixes` replaces the file, which is then not read; a bad option value is
/// an error too.
#[test]
fn fixes_errors() {
    let d = scratch("fixes-bad");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT 1\n").unwrap();
    std::fs::write(d.join("qb64rust.toml"), "[fixes]\nresume-after-blok = true\n").unwrap();
    for args in [&["--dump", "ir", "p.bas"][..], &["-z", "p.bas"], &["p.bas"]] {
        let o = qb64rust(&d, args);
        let out = stdout(&o);
        assert_eq!(o.status.code(), Some(1), "{args:?}: {out}");
        assert!(
            out.contains("qb64rust.toml:2: unknown fix `resume-after-blok`") && out.contains("resume-after-block"),
            "{args:?}: {out}"
        );
    }
    assert!(!d.join("p.exe").exists());
    let all = dumped_settings(&d, &["--fixes", "on"]);
    assert!(
        all.contains("fixes resume-after-block ") && all.ends_with(" checking-on-service bit-number-check"),
        "not read: {all}"
    );
    for bad in [
        &["--fixes", "maybe"][..],
        &["--fix", "nonsense=on"],
        &["--fix", "dim-empty-error"],
    ] {
        let mut args = bad.to_vec();
        args.push("p.bas");
        let o = qb64rust(&d, &args);
        assert_eq!(o.status.code(), Some(1), "{bad:?}");
        assert!(stdout(&o).contains("usage: qb64rust"), "{bad:?}");
    }
}

/// A program checked with `--dump typed` and the fix `fix` off, then on: off it compiles, on it fails with exit
/// status 1 and one error at `at` (line:column) that holds `words` and is not marked "not supported yet".
fn fix_makes_an_error(name: &str, program: &str, fix: &str, at: &str, words: &str) {
    let d = scratch(name);
    std::fs::write(d.join("p.bas"), program).unwrap();
    let o = qb64rust(&d, &["--fix", &format!("{fix}=off"), "--dump", "typed", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{fix} off: {}", stdout(&o));
    let o = qb64rust(&d, &["--fix", &format!("{fix}=on"), "--dump", "typed", "p.bas"]);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(1), "{fix} on: {out}");
    assert!(
        out.contains(&format!("p.bas:{at}: error: ")) && out.contains(words),
        "{fix} on: {out}"
    );
    assert!(
        out.contains("\n1 error") && !out.contains("not supported yet"),
        "{fix} on: {out}"
    );
}

/// The fix `whole-array-value` (spec `language/arrays-and-types`, "Fix whole-array-value"; spec `compiler/cli`, "A
/// dump with a fix"): `PRINT a()` and `PRINT a() + 1` are errors at `a()`; `READ a()` and `LEN(a())` compile.
#[test]
fn fix_whole_array_value() {
    let head = "$CONSOLE:ONLY\nDIM a(3) AS LONG\nDATA 5\nREAD a()\nPRINT LEN(a())\n";
    for (i, line) in ["PRINT a()", "PRINT a() + 1"].iter().enumerate() {
        let program = format!("{head}{line}\nSYSTEM\n");
        let name = format!("fix-whole-{i}");
        fix_makes_an_error(
            &name,
            &program,
            "whole-array-value",
            "6:7",
            "a whole array is not a value",
        );
    }
}

/// The fix `resume-in-procedure` (spec `language/error-handling`, "Fix resume-in-procedure"): `RESUME NEXT` in a SUB
/// is an error at it.
#[test]
fn fix_resume_in_procedure() {
    let program =
        "$CONSOLE:ONLY\nON ERROR GOTO h\nERROR 5\nSYSTEM\nh:\ns\nRESUME NEXT\nSUB s\n    RESUME NEXT\nEND SUB\n";
    fix_makes_an_error(
        "fix-resume",
        program,
        "resume-in-procedure",
        "9:5",
        "an error handler of the main module",
    );
}

/// The fix `run-in-procedure` (spec `language/control-flow`, "Fix run-in-procedure"): `RUN` in a SUB is an error at
/// it, with the hint.
#[test]
fn fix_run_in_procedure() {
    let program = "$CONSOLE:ONLY\ns\nSYSTEM\nSUB s\n    RUN\nEND SUB\n";
    fix_makes_an_error("fix-run", program, "run-in-procedure", "5:5", "set a flag there");
}

/// A machine without a QB64pe folder and without an installed toolchain, for `cmd`: `QB64RUST_QB64PE_ROOT` empty
/// (no QB64pe folder, not even the default clone), `QB64RUST_TOOLCHAIN_HOME` the empty folder `home`, no
/// `QB64RUST_TOOLCHAIN_ROOT` (design D5, D7 of `m6-toolchain`).
fn without_qb64pe<'a>(cmd: &'a mut Command, home: &Path) -> &'a mut Command {
    std::fs::create_dir_all(home).unwrap();
    cmd.env("QB64RUST_QB64PE_ROOT", "")
        .env("QB64RUST_TOOLCHAIN_HOME", home)
        .env_remove("QB64RUST_TOOLCHAIN_ROOT")
}

/// Spec `compiler/toolchain`, "No toolchain found": a build on a fresh machine fails with exit status 1 and the
/// message naming `qb64rust toolchain install`, the size and source of the download, `--toolchain-root` and the
/// variable; no network is reached (nothing is downloaded but by `toolchain install`), and the stale executable is
/// gone. Scenario "Checking needs no toolchain": `-z` works there.
#[test]
#[cfg(windows)]
fn no_toolchain_found() {
    let d = scratch("notoolchain");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    std::fs::write(d.join("p.exe"), b"stale").unwrap();
    let compile = |extra: &[&str]| {
        let mut cmd = compiler();
        without_qb64pe(&mut cmd, &d.join("home"))
            .current_dir(&d)
            .arg("-q")
            .args(extra)
            .arg("p.bas");
        run(&mut cmd, None, COMPILER_LIMIT, "compiler")
    };
    let o = compile(&[]);
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    for word in [
        "qb64rust toolchain install",
        "190 MB",
        "github.com/mstorsjo/llvm-mingw",
        "--toolchain-root",
        "QB64RUST_TOOLCHAIN_ROOT",
    ] {
        assert!(out.contains(word), "{word}: {out}");
    }
    assert!(!d.join("p.exe").exists());
    assert_eq!(
        std::fs::read_dir(d.join("home")).unwrap().count(),
        0,
        "nothing installed"
    );
    assert_eq!(compile(&["-z"]).status.code(), Some(0));
}

/// `qb64rust toolchain path` for each place of spec `compiler/toolchain`, "Toolchain lookup" (the default clone's is
/// the unit test's): the folder, then the source's word; with none, the message of "No toolchain found" and exit
/// status 1. Stand-in folders: nothing is built.
#[test]
#[cfg(windows)]
fn toolchain_path_names_each_source() {
    let d = scratch("toolchain-path");
    let home = d.join("home");
    let path = |extra: &[&str], vars: &[(&str, &Path)]| {
        let mut cmd = compiler();
        without_qb64pe(&mut cmd, &home)
            .current_dir(&d)
            .args(["toolchain", "path"])
            .args(extra);
        for (name, value) in vars {
            cmd.env(name, value);
        }
        run(&mut cmd, None, COMPILER_LIMIT, "compiler")
    };
    let o = path(&[], &[]);
    assert_eq!(o.status.code(), Some(1));
    assert!(stdout(&o).contains("qb64rust toolchain install"), "{}", stdout(&o));

    let installed = home.join(qb64rust_driver::toolchain::RELEASE);
    fake_toolchain(&installed);
    // Not installed until the marker is there.
    assert_eq!(path(&[], &[]).status.code(), Some(1));
    std::fs::write(installed.join(qb64rust_driver::toolchain::MARKER), "").unwrap();
    fake_toolchain(&d.join("opt"));
    fake_toolchain(&d.join("var"));
    std::fs::create_dir_all(d.join("q/internal/c")).unwrap();
    std::fs::write(d.join("q/internal/c/qbx.cpp"), "").unwrap();
    fake_toolchain(&d.join("q/internal/c/c_compiler"));
    let var = d.join("var");
    let q = d.join("q");
    for (extra, vars, folder, word) in [
        (&[][..], &[][..], installed.clone(), "installed:"),
        (
            &["--qb64pe-root", "q"][..],
            &[][..],
            q.join("internal/c/c_compiler"),
            "qb64pe:",
        ),
        (
            &[][..],
            &[("QB64RUST_QB64PE_ROOT", q.as_path())][..],
            q.join("internal/c/c_compiler"),
            "qb64pe:",
        ),
        (
            &["--qb64pe-root", "q"][..],
            &[("QB64RUST_TOOLCHAIN_ROOT", var.as_path())][..],
            var.clone(),
            "variable:",
        ),
        (
            &["--toolchain-root", "opt"][..],
            &[("QB64RUST_TOOLCHAIN_ROOT", var.as_path())][..],
            d.join("opt"),
            "option:",
        ),
    ] {
        let o = path(extra, vars);
        assert_eq!(o.status.code(), Some(0), "{extra:?}: {}", stdout(&o));
        let out = stdout(&o);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2, "{out}");
        assert_eq!(
            std::fs::canonicalize(lines[0]).unwrap(),
            std::fs::canonicalize(&folder).unwrap(),
            "{extra:?}"
        );
        assert!(lines[1].starts_with(word), "{extra:?}: {out}");
    }
    // A QB64pe folder given that is not one fails as a build fails, even with the toolchain installed.
    let opt = d.join("opt");
    for (extra, vars) in [
        (&["--qb64pe-root", "opt"][..], &[][..]),
        (&[][..], &[("QB64RUST_QB64PE_ROOT", opt.as_path())][..]),
    ] {
        let o = path(extra, vars);
        assert_eq!(o.status.code(), Some(1), "{extra:?}: {}", stdout(&o));
        assert!(stdout(&o).contains("is not a QB64pe folder"), "{}", stdout(&o));
    }
}

/// The text of `/etc/os-release` of this machine names Debian or Ubuntu.
#[cfg(unix)]
fn on_debian() -> bool {
    qb64rust_driver::toolchain::install_command(qb64rust_driver::toolchain::os_release().as_deref())
        .is_some_and(|l| l.starts_with("sudo apt-get"))
}

/// Spec `compiler/toolchain`, "No g++ on Ubuntu" and "Install on Linux": with nothing on `PATH`, a build fails with
/// exit status 1 naming `g++` and the install line, and so does `toolchain install`, which downloads nothing; `-z`
/// still works (design D1 of `m7-linux`).
#[test]
#[cfg(unix)]
fn linux_no_toolchain_found() {
    let d = scratch("linux-notoolchain");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let run_with = |args: &[&str]| {
        let mut cmd = compiler();
        cmd.current_dir(&d)
            .env("PATH", d.join("nothing"))
            .env_remove("QB64RUST_TOOLCHAIN_ROOT")
            .args(args);
        run(&mut cmd, None, COMPILER_LIMIT, "compiler")
    };
    for args in [&["-q", "p.bas"][..], &["toolchain", "install"], &["toolchain", "path"]] {
        let o = run_with(args);
        assert_eq!(o.status.code(), Some(1), "{args:?}");
        let out = stdout(&o);
        assert!(out.contains("g++") && out.contains("not on PATH"), "{args:?}: {out}");
        assert!(!out.contains("llvm-mingw"), "{args:?}: {out}");
        if on_debian() {
            assert!(
                out.contains("`sudo apt-get install build-essential "),
                "{args:?}: {out}"
            );
        }
    }
    assert!(!d.join("p").exists());
    assert_eq!(run_with(&["-z", "p.bas"]).status.code(), Some(0));
}

/// Spec `compiler/toolchain`, "System on Linux": `toolchain path` prints the folder of the `g++` on `PATH`, then
/// `system`; `toolchain install` says the system toolchain is used. Needs `g++`, `make` and `objcopy` on `PATH`.
#[test]
#[cfg(unix)]
fn linux_toolchain_path_is_the_system() {
    let d = scratch("linux-toolchain-path");
    let o = qb64rust(&d, &["toolchain", "path"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    let out = stdout(&o);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "{out}");
    assert!(Path::new(lines[0]).join("g++").is_file(), "{out}");
    assert!(lines[1].starts_with("system:"), "{out}");
    // The headers of the packages are there when a build works, as on the machines that run these tests.
    let o = qb64rust(&d, &["toolchain", "install"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(stdout(&o).contains("the system toolchain is used"), "{}", stdout(&o));
}

/// Spec `compiler/cli`, "Default name on Linux": `qb64rust -x game.bas` writes `game` beside the source, executable.
#[test]
#[cfg(unix)]
#[ignore = "builds a program"]
fn linux_default_name() {
    use std::os::unix::fs::PermissionsExt as _;
    let d = scratch("linux-default-name");
    std::fs::write(d.join("game.bas"), SLICE_PROGRAM).unwrap();
    let o = qb64rust_build(&d, &["-q", "-x", "game.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    let exe = d.join("game");
    let mode = std::fs::metadata(&exe).unwrap().permissions().mode();
    assert_eq!(mode & 0o111, 0o111, "{mode:o}");
    assert!(!d.join("game.exe").exists());
    let r = run(unattended(&exe).current_dir(&d), None, PROGRAM_LIMIT, "program");
    assert!(stdout(&r).starts_with("x= .25  3 \n"), "{}", stdout(&r));
}

/// The Windows-only metacommands on Linux (task 3.2 of `m7-linux`), as QB64pe's Linux build takes them (measured
/// 2026-10-28): `$EXEICON` and `$VERSIONINFO` put nothing into the executable and give the same diagnostics as on
/// Windows; a missing icon is still an error, but any file builds (no `windres` reads it).
#[test]
#[cfg(unix)]
#[ignore = "builds a program"]
fn linux_windows_only_metacommands() {
    let d = scratch("linux-metacommands");
    let game = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/programs/civil-war-strategy/cws.ico");
    std::fs::copy(game, d.join("cws.ico")).unwrap();
    std::fs::write(d.join("bad.ico"), "not an icon\n").unwrap();
    let program = |head: &str| format!("$CONSOLE:ONLY\n{head}\nPRINT \"ran\"\nSYSTEM\n");
    for (name, head, warning) in [
        ("icon", "$EXEICON:'./cws.ico'", ""),
        ("bad", "$EXEICON:'./bad.ico'", ""),
        (
            "ver",
            "$VERSIONINFO:CompanyName=Acme\n$VERSIONINFO:FILEVERSION#=1,2,3,4",
            "ver.bas:2:1: warning: Missing string bracket delimiters ('')\n    Acme\n",
        ),
    ] {
        std::fs::write(d.join(format!("{name}.bas")), program(head)).unwrap();
        let o = qb64rust_build(&d, &["-w", "-q", "-x", &format!("{name}.bas")]);
        assert!(o.status.success(), "{name}: {}", stdout(&o));
        assert_eq!(stdout(&o), warning, "{name}");
        let r = run(
            unattended(&d.join(name)).current_dir(&d),
            None,
            PROGRAM_LIMIT,
            "program",
        );
        assert_eq!(stdout(&r), "ran\n", "{name}");
    }
    std::fs::write(d.join("none.bas"), program("$EXEICON:'./none.ico'")).unwrap();
    let o = qb64rust(&d, &["-q", "-x", "none.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(
        stdout(&o).contains("the file `./none.ico` of `$EXEICON` is not found"),
        "{}",
        stdout(&o)
    );
}

/// Spec `language/external-libraries`, "A name the DLL does not export" and "A shared object that does not load", on
/// Linux (task 3.5 of `m7-linux`): `./v37dll` is `libv37dll.so` beside the program, loaded with `dlopen` as the
/// program starts; a name it does not export is error 260, and the file deleted after the build is error 259, both
/// before the first `PRINT` and past `ON ERROR`.
#[test]
#[cfg(unix)]
#[ignore = "builds a program"]
fn linux_shared_object_errors_at_the_start() {
    let d = scratch("linux-so-errors");
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../verification/v37_lib/v37dll.c");
    let so = d.join("libv37dll.so");
    let o = run(
        Command::new("gcc")
            .args(["-O2", "-fPIC", "-shared"])
            .arg(&src)
            .arg("-o")
            .arg(&so),
        None,
        COMPILER_LIMIT,
        "gcc",
    );
    assert!(o.status.success(), "{}", text(&o.stderr));
    let program = |extra: &str| {
        format!(
            "$CONSOLE:ONLY\nON ERROR GOTO handler\nDECLARE DYNAMIC LIBRARY \"./v37dll\"\n\
             FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)\n{extra}END DECLARE\nPRINT \"start\"; d_add&(1, 2)\n\
             SYSTEM\nhandler:\nPRINT \"handled\"; ERR\nRESUME NEXT\n"
        )
    };
    let cases = [
        (
            "none",
            program("FUNCTION d_none& (BYVAL a AS LONG)\n"),
            "Sub/Function does not exist in dynamic library",
        ),
        ("gone", program(""), "Cannot find dynamic library file"),
    ];
    for (name, source, message) in &cases {
        std::fs::write(d.join(format!("{name}.bas")), source).unwrap();
        let o = qb64rust_build(&d, &["-q", "-x", &format!("{name}.bas")]);
        assert!(o.status.success(), "{name}: {}", stdout(&o));
        if *name == "gone" {
            let r = run(
                unattended(&d.join(name)).current_dir(&d),
                None,
                PROGRAM_LIMIT,
                "program",
            );
            assert_eq!(stdout(&r), "start 3 \n", "with the file there");
            std::fs::remove_file(&so).unwrap();
        }
        let r = run(
            unattended(&d.join(name)).current_dir(&d),
            None,
            PROGRAM_LIMIT,
            "program",
        );
        assert_eq!(stdout(&r), "", "{name}");
        assert!(text(&r.stderr).contains(*message), "{name}: {}", text(&r.stderr));
    }
}

/// Spec `compiler/cli`, "Toolchain subcommand": any other word after `toolchain` prints the two forms and exits with
/// status 1 ("Unknown action"); `toolchain.bas` is a file to compile ("A file named toolchain").
#[test]
fn toolchain_subcommand_words() {
    let d = scratch("toolchain-words");
    for args in [
        &["toolchain", "remove"][..],
        &["toolchain"],
        &["toolchain", "install", "now"],
        &["toolchain", "path", "-x"],
        &["toolchain", "path", "--toolchain-root"],
    ] {
        let o = qb64rust(&d, args);
        assert_eq!(o.status.code(), Some(1), "{args:?}");
        let out = stdout(&o);
        assert!(
            out.contains("qb64rust toolchain install") && out.contains("qb64rust toolchain path"),
            "{args:?}: {out}"
        );
    }
    std::fs::write(d.join("toolchain.bas"), SLICE_PROGRAM).unwrap();
    let o = qb64rust(&d, &["toolchain.bas", "--dump", "typed"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).contains("INSTR"), "{}", stdout(&o));
}

/// Spec `compiler/cli`, "Locating the reference runtime", scenario "No clone found": with no QB64pe folder anywhere
/// and the toolchain given by `--toolchain-root` (the clone's own, which is the pinned release byte for byte), a
/// program builds and runs.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn build_without_a_qb64pe_folder() {
    let d = scratch("noqb64pe");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\nPRINT 1 / 4; INSTR(3, \"hello\", \"l\")\nSYSTEM\n",
    )
    .unwrap();
    let tools = support::toolchain_folder();
    let mut cmd = compiler();
    without_qb64pe(&mut cmd, &d.join("home"))
        .current_dir(&d)
        .env_remove("QB64RUST_BUILD_CACHE")
        .arg("--toolchain-root")
        .arg(&tools)
        .args(["-q", "p.bas"]);
    let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
    assert!(o.status.success(), "{}", stdout(&o));
    let r = run(
        unattended(&d.join(default_exe("p"))).current_dir(&d),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(r.status.code(), Some(0), "{}", stdout(&r));
    assert_eq!(stdout(&r).replace("\r\n", "\n"), " .25  3 \n");
}

/// Spec `compiler/cli`, scenario "A library without a clone": with no QB64pe folder, `DECLARE LIBRARY "lib/x"` of
/// `p\prog.bas` compiled with `--include-root r` finds `r\lib\x.h`, as it would under QB64pe's folder, and the
/// program builds and calls it.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn library_without_a_qb64pe_folder() {
    let d = scratch("noqb64pe-library");
    std::fs::create_dir_all(d.join("p")).unwrap();
    std::fs::create_dir_all(d.join("r/lib")).unwrap();
    std::fs::write(
        d.join("r/lib/x.h"),
        "#include <stdint.h>\nstatic int32_t x_add(int32_t a, int32_t b) { return a + b; }\n",
    )
    .unwrap();
    std::fs::write(
        d.join("p/prog.bas"),
        "$CONSOLE:ONLY\nDECLARE LIBRARY \"lib/x\"\n    FUNCTION x_add& (BYVAL a AS LONG, BYVAL b AS LONG)\n\
         END DECLARE\nPRINT x_add(2, 3)\nSYSTEM\n",
    )
    .unwrap();
    let tools = support::toolchain_folder();
    let mut cmd = compiler();
    without_qb64pe(&mut cmd, &d.join("home"))
        .current_dir(&d)
        .env_remove("QB64RUST_BUILD_CACHE")
        .arg("--toolchain-root")
        .arg(&tools)
        .args(["-q", "--include-root", "r", "p/prog.bas"]);
    let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
    assert!(o.status.success(), "{}", stdout(&o));
    let r = run(
        unattended(&d.join("p").join(default_exe("prog"))).current_dir(&d),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(stdout(&r).replace("\r\n", "\n"), " 5 \n");
}

/// `RETURN label` with no `GOSUB` pending raises error 3 and leaves the `GOSUB` stack intact, so a later `GOSUB`
/// and `RETURN` work (`QB64PE-DIVERGENCES.md` D-003; the old compiler's program crashes at that `GOSUB`, so this cannot be
/// a corpus program recorded with `qb64pe.exe`). The error is serviced at the label `RETURN` jumps to.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn return_label_with_nothing_pending() {
    let d = scratch("return-label");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO handler\nRETURN back\nPRINT \"not printed\"\nback:\n\
                   PRINT \"at back\"\nGOSUB sub1\nPRINT \"after sub1\"\nGOSUB sub1\nPRINT \"after sub1 again\"\nSYSTEM\n\
                   sub1:\nPRINT \"in sub1\"\nRETURN\nhandler:\nPRINT \"error\"; ERR\nRESUME NEXT\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-003\n"),
        "error 3 \nat back\nin sub1\nafter sub1\nin sub1\nafter sub1 again\n"
    );
}

/// A store into a member of an element whose index is out of range, or raises with no error pending before it,
/// stores nothing (`QB64PE-DIVERGENCES.md` D-004; the old compiler writes element 0, so this cannot be a corpus program
/// recorded with `qb64pe.exe`). The value is still evaluated first: with both raising, `ERR` is the value's error.
/// A raising value with a good index is stored as its placeholder, as in the old compiler.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn member_store_with_bad_index() {
    let d = scratch("member-bad-index");
    let program = "$CONSOLE:ONLY\nTYPE t\nm AS LONG\nEND TYPE\nDIM a(3) AS t\nDIM k AS LONG\nk = -1\n\
                   ON ERROR GOTO handler\na(0).m = 70\na(9).m = 5\nPRINT \"after a(9):\"; a(0).m\n\
                   a(-1).m = 6\nPRINT \"after a(-1):\"; a(0).m\na(9).m = INSTR(CHR$(k), \"x\")\n\
                   PRINT \"after both:\"; a(0).m\na(INSTR(CHR$(k), \"x\")).m = 7\nPRINT \"after raising index:\"; a(0).m\n\
                   a(2).m = 3: a(2).m = INSTR(CHR$(k), \"x\")\nPRINT \"a(2).m:\"; a(2).m\nSYSTEM\n\
                   handler:\nPRINT \"error\"; ERR\nRESUME NEXT\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-004\n"),
        "error 9 \nafter a(9): 70 \nerror 9 \nafter a(-1): 70 \nerror 5 \nafter both: 70 \nerror 5 \n\
         after raising index: 70 \nerror 5 \na(2).m: 0 \n"
    );
}

/// The same for an element of a member array (`QB64PE-DIVERGENCES.md` D-004, design D8 of `m2-defxxx-common-arrays`): a
/// store with an index outside its bounds, of the member array or of an array on the way to it, stores nothing,
/// a number and a string alike; the old compiler writes position 0 of that dimension (measured,
/// `verification\v24_h_member_store`, `v24_x313`). With the value raising too, `ERR` is the value's error; a
/// raising value with a good index is stored as its placeholder.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn member_array_store_with_bad_index() {
    let d = scratch("member-array-bad-index");
    let program = "$CONSOLE:ONLY\nTYPE t\nid AS LONG\nv(0 TO 3) AS LONG\ng(1 TO 2, 0 TO 3) AS INTEGER\n\
                   s(0 TO 1) AS STRING\nEND TYPE\nDIM x AS t, a(2) AS t, k AS LONG\nk = -1\n\
                   ON ERROR GOTO handler\nx.v(0) = 100: a(0).v(0) = 200: a(0).v(1) = 201: a(1).v(0) = 210\n\
                   x.g(2, 0) = 7: x.s(0) = \"old\"\n\
                   x.v(9) = 5\nPRINT \"x.v(9):\"; x.v(0)\na(9).v(1) = 5\nPRINT \"a(9).v(1):\"; a(0).v(1)\n\
                   a(1).v(9) = 6\nPRINT \"a(1).v(9):\"; a(1).v(0)\na(9).v(9) = 7\nPRINT \"a(9).v(9):\"; a(0).v(0)\n\
                   x.g(2, 9) = 8\nPRINT \"x.g(2, 9):\"; x.g(2, 0)\nx.s(9) = \"new\"\nPRINT \"x.s(9): \"; x.s(0)\n\
                   x.v(9) = INSTR(CHR$(k), \"x\")\nPRINT \"both:\"; x.v(0)\n\
                   x.v(1) = 3: x.v(1) = INSTR(CHR$(k), \"x\")\nPRINT \"x.v(1):\"; x.v(1)\nSYSTEM\n\
                   handler:\nPRINT \"error\"; ERR\nRESUME NEXT\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-004\n"),
        "error 9 \nx.v(9): 100 \nerror 9 \na(9).v(1): 201 \nerror 9 \na(1).v(9): 210 \nerror 9 \na(9).v(9): 200 \n\
         error 9 \nx.g(2, 9): 7 \nerror 9 \nx.s(9): old\nerror 5 \nboth: 100 \nerror 5 \nx.v(1): 0 \n"
    );
}

/// A `_BIT * n` scalar wider than 32 bits has 8 bytes of its own, so a store into it leaves the `_BIT` scalar
/// declared before it alone (`QB64PE-DIVERGENCES.md` D-009; the old compiler reserves 4 bytes and overwrites that
/// neighbour, printing ` 4294967295 ` and ` 0 ` here, so this cannot be a corpus program recorded with `qb64pe.exe`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn wide_bit_scalars_do_not_overlap() {
    let d = scratch("wide-bit");
    let program = "$CONSOLE:ONLY\nDIM a AS _BIT * 33, b AS _BIT * 33\na = 5: b = -1\nPRINT a; b\n\
                   DIM u AS _UNSIGNED _BIT * 3, w AS _BIT * 40\nu = 2: w = 2\nPRINT u; w\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 5 -1 \n 2  2 \n");
}

/// `-f:ExtraLinkerFlags=` puts a source file on the link line: the screen-state file of `tools/screen`, which writes
/// the state of the screen when the program exits (scenario "An extra source file on the link line").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn extra_linker_flags_link_the_screen_state_file() {
    let d = scratch("extra-link");
    let exe = build_with_screen_state(&d, "$CONSOLE:ONLY\nPRINT \"hi\"\nSYSTEM\n");
    let run = run_screen_state(&d, &exe, None, None);
    assert_eq!(run.status.code(), Some(0), "{}", stdout(&run));
    let text = std::fs::read(d.join("state.txt")).unwrap();
    assert!(
        text.starts_with(b"screen-state 1\npages 8 display 0 write image read image\n"),
        "{}",
        text.escape_ascii()
    );
    assert!(text.ends_with(b"end\n"), "{}", text.escape_ascii());
}

/// Spec `testing/screen-state`, scenario "A prompt that never comes": the program waits for a key without
/// announcing it, so the keys file's `prompt` is ended after its time (`QB64_SCREEN_PROMPT_MS`, shortened here)
/// with exit code 124, the state as it was then, and a last line that names the line of the keys file (libqb
/// points stderr at the console the program was started from, so the message on stderr is not read here).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn screen_state_prompt_that_never_comes() {
    let d = scratch("prompt");
    let exe = build_with_screen_state(
        &d,
        "SCREEN 13\nLINE (0, 0)-(9, 9), 4, BF\nDO: k$ = INKEY$: LOOP WHILE k$ = \"\"\nSYSTEM\n",
    );
    let keys = "# nothing is announced\nprompt\ntext x\n";
    // Long enough that the box is drawn on a loaded machine (500 ms failed once, `study\archive\30` §8).
    let run = run_screen_state(&d, &exe, Some(keys), Some("5000"));
    assert_eq!(run.status.code(), Some(124));
    let text = std::fs::read(d.join("state.txt")).unwrap();
    assert!(text.starts_with(b"screen-state 1\n"), "{}", text.escape_ascii());
    assert!(
        text.windows(10).any(|w| w == b"p 04*10 00"),
        "the box: {}",
        text.escape_ascii()
    );
    assert!(
        text.ends_with(b"p.keys line 2: no prompt in time\n"),
        "{}",
        text.escape_ascii()
    );
}

/// Spec `testing/screen-state`, scenario "A wait no rule answers" (design D4 of `cws-whole-game`): during `play until
/// 14:M` the program waits at place 9, for which the keys file has no rule; the run ends with exit code 125 and a
/// last line in the state file that names the place and the line of the `play until`. The wait at place 7 before it
/// is answered by its rule.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn screen_state_wait_no_rule_answers() {
    let d = scratch("no-rule");
    let wait = |place: u8| {
        format!(
            "DEF SEG = 0: POKE 1265, {place}: POKE 1266, 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG\n\
             DO: k$ = INKEY$: LOOP WHILE k$ = \"\"\n"
        )
    };
    let exe = build_with_screen_state(&d, &format!("SCREEN 13\n{}{}SYSTEM\n", wait(7), wait(9)));
    let run = run_screen_state(&d, &exe, Some("rule 7 key space\nplay until 14:M\ntext e\n"), None);
    assert_eq!(run.status.code(), Some(125));
    let text = std::fs::read(d.join("state.txt")).unwrap();
    assert!(text.starts_with(b"screen-state 1\n"), "{}", text.escape_ascii());
    assert!(
        text.ends_with(b"p.keys line 2: no rule for the wait at place 9\n"),
        "{}",
        text.escape_ascii()
    );
}

/// Spec `testing/screen-state`, scenario "A mistake in a mouse step" (design D4 of `m3-images-fonts-mouse`): a
/// button that is not 1 to 3, or a `mouse` step without its second number, ends the program with exit code 125
/// before it runs, and stderr names the keys file and the line. The keys file is read when the program starts,
/// before libqb points stderr elsewhere, so the message is read here.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn screen_state_mistake_in_a_mouse_step() {
    let d = scratch("mouse-step");
    let exe = build_with_screen_state(&d, "SCREEN 13\nPSET (1, 1), 4\nSYSTEM\n");
    for (keys, line) in [
        ("mouse 40 30\npress 9\n", 2),
        ("# a move needs x and y\nmouse 10\npress 1\n", 2),
        ("mouse 4 3\nrelease 0\n", 2),
        ("wheel 0\n", 1),
    ] {
        let run = run_screen_state(&d, &exe, Some(keys), None);
        assert_eq!(run.status.code(), Some(125), "{keys:?}");
        let err = run.stderr.escape_ascii().to_string();
        assert!(
            err.contains("p.keys line ") && err.contains(&format!("line {line}: not a step")),
            "{keys:?}: {err}"
        );
    }
}

/// The same for the held keys (`tools/screen/README.md`): a `hold` of a key already held, a `lift` of one not held
/// (another key than the held one too), and the `end` of a `repeat` block that leaves a key held. A key held at the
/// end of the file is not a mistake: it stays down until the program exits.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn screen_state_mistake_in_a_held_key() {
    let d = scratch("held-key");
    let exe = build_with_screen_state(&d, "SCREEN 13\nPSET (1, 1), 4\nSYSTEM\n");
    for (keys, line) in [
        ("hold lshift\nhold lshift\n", 2),
        ("lift up\n", 1),
        ("hold rshift\nlift lshift\n", 2),
        ("repeat 2\nhold up\nend\n", 3),
        ("hold up\nrepeat 2\nlift up\nend\n", 4),
    ] {
        let run = run_screen_state(&d, &exe, Some(keys), None);
        assert_eq!(run.status.code(), Some(125), "{keys:?}");
        let err = run.stderr.escape_ascii().to_string();
        assert!(
            err.contains("p.keys line ") && err.contains(&format!("line {line}: not a step")),
            "{keys:?}: {err}"
        );
    }
    for keys in ["repeat 2\nhold up\nlift up\nend\n", "hold lshift\n"] {
        let run = run_screen_state(&d, &exe, Some(keys), None);
        assert_eq!(run.status.code(), Some(0), "{keys:?}");
    }
}

/// A mouse step waits until the window shows the displayed page, since `_MOUSEX` maps an event by the frame libqb
/// last drew (`tools/screen/README.md`). A window that never shows (`$SCREENHIDE`) ends the wait after its time
/// with exit code 124 and a last line of the state that names the step.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn screen_state_mouse_step_of_a_hidden_window() {
    let d = scratch("mouse-hidden");
    let exe = build_with_screen_state(
        &d,
        "$SCREENHIDE\nSCREEN 13\nDO: k$ = INKEY$: LOOP WHILE k$ = \"\"\nSYSTEM\n",
    );
    let run = run_screen_state(&d, &exe, Some("mouse 1 1\ntext x\n"), Some("500"));
    assert_eq!(run.status.code(), Some(124));
    let text = std::fs::read(d.join("state.txt")).unwrap();
    assert!(
        text.ends_with(b"p.keys line 1: the window never showed the screen\n"),
        "{}",
        text.escape_ascii()
    );
}

/// Writes `p.bas` in `d` and builds it with the screen-state file of `tools/screen` on the link line
/// (`-f:ExtraLinkerFlags=`); the executable.
fn build_with_screen_state(d: &Path, program: &str) -> PathBuf {
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/screen/screen_state.cpp");
    let flag = format!("-f:ExtraLinkerFlags={}", file.to_str().unwrap().replace('\\', "/"));
    let o = qb64rust_build(d, &[&flag, "-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    exe
}

/// Runs a program of [`build_with_screen_state`] in `d`, its state written to `state.txt` there; with `keys`, given
/// a keys file `p.keys` of that text, and with `prompt_ms`, that time for a `prompt` (`QB64_SCREEN_PROMPT_MS`).
fn run_screen_state(d: &Path, exe: &Path, keys: Option<&str>, prompt_ms: Option<&str>) -> Output {
    let mut cmd = unattended(exe);
    cmd.current_dir(d).env("QB64_SCREEN_STATE", d.join("state.txt"));
    if let Some(keys) = keys {
        std::fs::write(d.join("p.keys"), keys).unwrap();
        cmd.env("QB64_SCREEN_KEYS", d.join("p.keys"));
    }
    if let Some(ms) = prompt_ms {
        cmd.env("QB64_SCREEN_PROMPT_MS", ms);
    }
    run(&mut cmd, None, PROGRAM_LIMIT, "program")
}

/// The smallest icon file: one image of 16 by 16 pixels with 32 bits each, every pixel of colour `fill` (four
/// bytes: blue, green, red, alpha). The resource compiler puts the image, the bytes from offset 22 on, into the
/// executable as they are.
fn icon_16(fill: [u8; 4]) -> Vec<u8> {
    let image_len: u32 = 40 + 16 * 16 * 4 + 16 * 4;
    let mut ico = Vec::new();
    // The file's header: reserved, type 1 (an icon), one image.
    ico.extend([0, 0, 1, 0, 1, 0]);
    // The entry of the image: width, height, no palette, reserved, 1 plane, 32 bits, its length and its offset.
    ico.extend([16, 16, 0, 0, 1, 0, 32, 0]);
    ico.extend(image_len.to_le_bytes());
    ico.extend(22u32.to_le_bytes());
    // The image: a bitmap header (its height counts the mask too), the pixels, the mask.
    ico.extend(40u32.to_le_bytes());
    ico.extend(16i32.to_le_bytes());
    ico.extend(32i32.to_le_bytes());
    ico.extend([1, 0, 32, 0]);
    ico.extend([0; 24]);
    for _ in 0..16 * 16 {
        ico.extend(fill);
    }
    ico.extend([0; 16 * 4]);
    ico
}

fn holds(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

/// Spec `language/embedded-files`, "Where the file is", scenario "Missing file or a folder" (measured,
/// `verification\v44_x07`, `x27`): an error at the line that names the file as written, for every command that
/// reports diagnostics; no executable. With the file there the program has no diagnostic.
#[test]
fn embedded_file_is_missing_or_a_folder() {
    let d = scratch("embed-missing");
    std::fs::create_dir_all(d.join("folder")).unwrap();
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\n$EMBED:'./none.txt','a'\n$EMBED:'./folder','b'\nPRINT _EMBEDDED$(\"a\")\nSYSTEM\n",
    )
    .unwrap();
    let lines = "p.bas:2:1: error: the file `./none.txt` of `$EMBED` is not found\n\
                 p.bas:3:1: error: the file `./folder` of `$EMBED` is not found\n2 errors\n";
    for args in [&["-x", "p.bas"][..], &["-z", "p.bas"], &["p.bas", "--dump", "ir"]] {
        let o = qb64rust(&d, args);
        assert_eq!(o.status.code(), Some(1), "{args:?}: {}", stdout(&o));
        assert_eq!(stdout(&o), lines, "{args:?}");
    }
    assert!(!d.join("p.exe").exists());
    let o = qb64rust(&d, &["--report", "unsupported", "p.bas"]);
    assert!(stdout(&o).contains("1 with other errors"), "{}", stdout(&o));
    std::fs::write(d.join("none.txt"), "x").unwrap();
    std::fs::remove_dir(d.join("folder")).unwrap();
    std::fs::write(d.join("folder"), "y").unwrap();
    let o = qb64rust(&d, &["p.bas", "--dump", "ir"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(
        stdout(&o).starts_with("Settings: console, no window, runtime parts: embedded files\n"),
        "{}",
        stdout(&o)
    );
}

/// Only a file a compiled `_EMBEDDED$` names is built in (spec `language/embedded-files`, "Only used files are
/// built in"; design D3 of `m3-network-embed`): the kept build folder's `embedded.cpp` holds the used file's
/// handle and not the other's, and the program prints the file's bytes. A program whose only file is unused links
/// no embed part and gets no `embedded.cpp`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn only_used_embedded_files_are_built_in() {
    let d = scratch("embed-used");
    std::fs::write(d.join("used.txt"), "used!").unwrap();
    std::fs::write(d.join("unused.txt"), "unused").unwrap();
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\n$EMBED:'./used.txt','u'\n$EMBED:'./unused.txt','n'\nPRINT _EMBEDDED$(\"u\")\nSYSTEM\n",
    )
    .unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap(), "--keep-build"]);
    assert!(o.status.success(), "{}", stdout(&o));
    let cpp = std::fs::read(d.join("p.exe.qb64rust/temp/embedded.cpp")).unwrap();
    assert!(holds(&cpp, b"qbs_new_txt(\"u\")"));
    assert!(!holds(&cpp, b"qbs_new_txt(\"n\")"));
    let ran = run(unattended(&exe).current_dir(&d), None, PROGRAM_LIMIT, "program");
    assert_eq!(stdout(&ran).replace("\r\n", "\n"), "used!\n");
    // Only the unused file: no embed part, and the earlier build's `embedded.cpp` is gone.
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\n$EMBED:'./unused.txt','n'\nPRINT \"none\"\nSYSTEM\n",
    )
    .unwrap();
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap(), "--keep-build"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(!d.join("p.exe.qb64rust/temp/embedded.cpp").exists());
    let ran = run(unattended(&exe).current_dir(&d), None, PROGRAM_LIMIT, "program");
    assert_eq!(stdout(&ran).replace("\r\n", "\n"), "none\n");
}

/// Spec `language/console-and-window`, "Errors of $EXEICON", scenario "A missing file": an error at the line for
/// every command that reports diagnostics, naming the file as written; no executable and nothing built.
#[test]
fn exe_icon_file_is_missing() {
    let d = scratch("icon-missing");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\n$EXEICON:'./none.ico'\nPRINT 1\nSYSTEM\n",
    )
    .unwrap();
    let line = "p.bas:2:1: error: the file `./none.ico` of `$EXEICON` is not found\n1 error\n";
    for args in [
        &["-x", "p.bas"][..],
        &["-z", "p.bas"],
        &["p.bas", "--dump", "ir"],
        &["p.bas", "--dump", "typed"],
    ] {
        let o = qb64rust(&d, args);
        assert_eq!(o.status.code(), Some(1), "{args:?}: {}", stdout(&o));
        assert_eq!(stdout(&o), line, "{args:?}");
    }
    assert!(!d.join("p.exe").exists());
    let o = qb64rust(&d, &["--report", "unsupported", "p.bas"]);
    assert!(
        stdout(&o).contains("0 without errors, 0 with only \"not supported yet\" errors, 1 with other errors"),
        "{}",
        stdout(&o)
    );
    // With the file there, the same program has no diagnostic.
    std::fs::write(d.join("none.ico"), icon_16([1, 2, 3, 255])).unwrap();
    let o = qb64rust(&d, &["p.bas", "--dump", "ir"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(
        stdout(&o).starts_with("Settings: console, no window, runtime parts: icon, an icon for the executable\n"),
        "{}",
        stdout(&o)
    );
}

/// Where the icon of a `./` name is looked for: the rest of the name is added to the source file's folder as text
/// (`qb64pe.bas` 3853), so a rest that starts with a slash stays in that folder; a name is taken as UTF-8, as an
/// included file's is. A drive with no slash after it is no absolute name.
#[test]
fn exe_icon_name_forms() {
    let d = scratch("icon-names");
    std::fs::create_dir_all(d.join("sub")).unwrap();
    std::fs::write(d.join("sub").join("a.ico"), icon_16([1, 2, 3, 255])).unwrap();
    std::fs::write(d.join("ícone.ico"), icon_16([1, 2, 3, 255])).unwrap();
    for name in [".//sub/a.ico", "./ícone.ico"] {
        std::fs::write(d.join("p.bas"), format!("$CONSOLE:ONLY\n$EXEICON:'{name}'\nSYSTEM\n")).unwrap();
        let o = qb64rust(&d, &["p.bas", "--dump", "ir"]);
        assert!(o.status.success(), "{name}: {}", stdout(&o));
    }
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\n$EXEICON:'./não.ico'\nSYSTEM\n").unwrap();
    let o = qb64rust(&d, &["p.bas", "--dump", "ir"]);
    assert_eq!(
        stdout(&o),
        "p.bas:2:1: error: the file `./n\\xC3\\xA3o.ico` of `$EXEICON` is not found\n1 error\n"
    );
    // A byte that is no UTF-8 is shown as written.
    std::fs::write(d.join("p.bas"), b"$CONSOLE:ONLY\n$EXEICON:'./n\xE3o.ico'\nSYSTEM\n").unwrap();
    let o = qb64rust(&d, &["p.bas", "--dump", "ir"]);
    assert_eq!(
        stdout(&o),
        "p.bas:2:1: error: the file `./n\\xE3o.ico` of `$EXEICON` is not found\n1 error\n"
    );
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\n$EXEICON:'C:game.ico'\nSYSTEM\n").unwrap();
    let o = qb64rust(&d, &["p.bas", "--dump", "ir"]);
    assert_eq!(
        stdout(&o),
        "p.bas:2:1: error: not supported yet: a `$EXEICON` name with a drive and no slash after it\n\
         1 error (1 not supported yet)\n"
    );
}

/// Spec `compiler/cli`, "Compiler root for included files", scenario "An icon under the root", and
/// `language/embedded-files`, "Relative to the compiler's folder" (design D3 of `m4-devices-continue-paths`): a
/// relative name without `./` is looked for under `--include-root`, whatever the working directory is, and not
/// beside the program; missing there, the error names it as written. A name with one leading slash goes to the root
/// of the compiler root's drive, where this test puts nothing.
#[test]
fn relative_names_under_the_compiler_root() {
    let d = scratch("names-under-root");
    let root = d.join("root");
    std::fs::create_dir_all(root.join("icons")).unwrap();
    std::fs::create_dir_all(d.join("work").join("icons")).unwrap();
    std::fs::write(root.join("icons").join("a.ico"), icon_16([1, 2, 3, 255])).unwrap();
    std::fs::write(root.join("icons").join("t.txt"), "t").unwrap();
    // Beside the program it is not looked for: the old compiler's `v29_e02`.
    std::fs::write(d.join("work").join("icons").join("b.ico"), icon_16([1, 2, 3, 255])).unwrap();
    let root_arg = root.to_str().unwrap();
    let program = |text: &str| std::fs::write(d.join("work").join("p.bas"), text).unwrap();
    program("$CONSOLE:ONLY\n$EXEICON:'icons/a.ico'\n$EMBED:'icons/t.txt','h'\nPRINT _EMBEDDED$(\"h\")\nSYSTEM\n");
    let o = qb64rust(&d.join("work"), &["--include-root", root_arg, "p.bas", "--dump", "ir"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).contains("an icon for the executable"), "{}", stdout(&o));
    program("$CONSOLE:ONLY\n$EXEICON:'icons/b.ico'\n$EMBED:'icons/none.txt','h'\nSYSTEM\n");
    let o = qb64rust(&d.join("work"), &["--include-root", root_arg, "p.bas", "--dump", "ir"]);
    assert_eq!(
        stdout(&o),
        "p.bas:2:1: error: the file `icons/b.ico` of `$EXEICON` is not found\n\
         p.bas:3:1: error: the file `icons/none.txt` of `$EMBED` is not found\n2 errors\n"
    );
    program("$CONSOLE:ONLY\n$EXEICON:'/qb64rust-test-none/a.ico'\nSYSTEM\n");
    let o = qb64rust(&d.join("work"), &["--include-root", root_arg, "p.bas", "--dump", "ir"]);
    assert_eq!(
        stdout(&o),
        "p.bas:2:1: error: the file `/qb64rust-test-none/a.ico` of `$EXEICON` is not found\n1 error\n"
    );
    program("$CONSOLE:ONLY\n$EMBED:'C:t.txt','h'\nSYSTEM\n");
    let o = qb64rust(&d.join("work"), &["--include-root", root_arg, "p.bas", "--dump", "ir"]);
    assert!(
        stdout(&o).contains("not supported yet: a `$EMBED` file name with a drive and no slash after it"),
        "{}",
        stdout(&o)
    );
}

/// Spec `compiler/pipeline`, "The build follows the program", scenarios "An icon in the executable" and "A changed
/// icon": the executable holds the image of the icon `$EXEICON` names, and after the file changes, the new one.
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[cfg(windows)]
fn exe_icon_is_in_the_executable() {
    let d = scratch("icon");
    let (first, second) = (icon_16([0x11, 0x22, 0x33, 0xff]), icon_16([0x44, 0x55, 0x66, 0xff]));
    std::fs::write(d.join("game.ico"), &first).unwrap();
    let program = "$CONSOLE:ONLY\nPRINT \"before\"\n$EXEICON:'./game.ico'\nPRINT \"after\"\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), "before\nafter\n");
    let exe = std::fs::read(d.join("p.exe")).unwrap();
    assert!(holds(&exe, &first[22..]));
    assert!(!holds(&exe, &second[22..]));
    std::fs::write(d.join("game.ico"), &second).unwrap();
    assert_eq!(build_and_run(&d, program), "before\nafter\n");
    let exe = std::fs::read(d.join("p.exe")).unwrap();
    assert!(holds(&exe, &second[22..]));
    // A program without the line carries neither.
    assert_eq!(build_and_run(&d, "$CONSOLE:ONLY\nPRINT \"none\"\nSYSTEM\n"), "none\n");
    assert!(!holds(&std::fs::read(d.join("p.exe")).unwrap(), &second[22..]));
}

/// Spec `compiler/pipeline`, scenario "A file that is not an icon": the front end accepts it and the build fails,
/// as the old compiler's does (`verification\v29_e16`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[cfg(windows)]
fn exe_icon_that_is_no_icon_fails_the_build() {
    let d = scratch("icon-text");
    std::fs::write(d.join("game.ico"), "this is not an icon\n").unwrap();
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\n$EXEICON:'./game.ico'\nSYSTEM\n").unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(1), "{}", stdout(&o));
    assert!(!exe.exists());
}

/// Spec `testing/compiler-tests`, "A real program taken whole compiles", scenario "The executable": Civil War
/// Strategy builds, from a copy of its source and icon (its executable and build folder never touch the
/// repository), and the executable holds the first image of `cws.ico`. The game is not started: it needs its data
/// files and a player (`tests\programs\README.md`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn civil_war_strategy_builds() {
    let d = scratch("cws");
    let game = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/programs/civil-war-strategy");
    for file in ["CWSTRAT.BAS", "cws.ico"] {
        std::fs::copy(game.join(file), d.join(file)).unwrap();
    }
    let exe = d.join("CWSTRAT.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "CWSTRAT.BAS", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(stdout(&o), "", "a diagnostic or a warning");
    let ico = std::fs::read(d.join("cws.ico")).unwrap();
    // The first entry of the icon's directory (at offset 6): the length of its image at 8, the offset at 12.
    let field = |at: usize| u32::from_le_bytes(ico[at..at + 4].try_into().unwrap()) as usize;
    let (len, offset) = (field(6 + 8), field(6 + 12));
    // The icon is a Windows resource; on Linux `$EXEICON` writes none, as with QB64pe (task 3.2 of `m7-linux`).
    if cfg!(windows) {
        assert!(holds(&std::fs::read(&exe).unwrap(), &ico[offset..offset + len]));
    }
}

/// Builds `program` as `p.bas` in `d` with `-q -x`, runs it and gives its output with `\n` line ends.
fn build_and_run(d: &Path, program: &str) -> String {
    build_and_run_tracing(d, program, "")
}

/// [`build_and_run`] of a program that takes the paths of rows of `QB64PE-DIVERGENCES.md`: the run-time trace
/// (`QB64RUST_DIVERGENCES`, design D1 of `m3-divergence-trace`) must name exactly `rows`, one `D-0nn` line each, in
/// the order the program first takes them.
fn build_and_run_tracing(d: &Path, program: &str, rows: &str) -> String {
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let trace = d.join("divergences.txt");
    let _ = std::fs::remove_file(&trace);
    let run = run(
        unattended(&exe).current_dir(d).env("QB64RUST_DIVERGENCES", &trace),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    // A program ended by a runtime error exits with 1 and traces D-012 (`QB64PE-DIVERGENCES.md`), as the runner
    // wants it (design D2 of `basic-unit-tests`).
    let code = if rows.contains("D-012") { 1 } else { 0 };
    assert_eq!(run.status.code(), Some(code), "{}", stdout(&run));
    let taken: Vec<u8> = std::fs::read(&trace)
        .unwrap_or_default()
        .into_iter()
        .filter(|&b| b != b'\r')
        .collect();
    assert_eq!(
        taken,
        rows.as_bytes(),
        "the rows of QB64PE-DIVERGENCES.md the program took"
    );
    stdout(&run).replace("\r\n", "\n")
}

/// A `--coverage` build (spec `compiler/cli`, "Coverage build"): three statements, the second inside a false `IF`,
/// counted 1, 0 and 1; a second run appends a second record; a statement of an included file is named by that file
/// and its own line; without `QB64RUST_COVERAGE` nothing is written. The `IF` block's own jump to `END IF` gets no
/// entry (design D1 of `cws-whole-game`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn coverage_build_counts_statements() {
    let d = scratch("coverage");
    std::fs::write(d.join("inc.bi"), "\nPRINT \"inc\"\n").unwrap();
    let program = "$CONSOLE:ONLY\nPRINT \"a\"\nIF 0 THEN\n    PRINT \"b\"\nEND IF\n'$INCLUDE: 'inc.bi'\nSYSTEM\n";
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "--coverage", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let counts = d.join("c.txt");
    let run_it = |with: bool| {
        let mut cmd = unattended(&exe);
        cmd.current_dir(&d).env_remove("QB64RUST_COVERAGE");
        if with {
            cmd.env("QB64RUST_COVERAGE", &counts);
        }
        let r = run(&mut cmd, None, PROGRAM_LIMIT, "program");
        assert_eq!(r.status.code(), Some(0), "{}", stdout(&r));
        assert_eq!(stdout(&r).replace("\r\n", "\n"), "a\ninc\n");
    };
    run_it(false);
    assert!(!counts.exists(), "a run without QB64RUST_COVERAGE writes no counts");
    run_it(true);
    let hash = |bytes: &[u8]| format!("{:016x}", qb64rust_driver::fnv1a64(bytes));
    let record = format!(
        "coverage 2\nfile\t0\t{}\tp.bas\nfile\t1\t{}\tinc.bi\n0\t1\t1\t1\t\n0\t2\t1\t1\t\n0\t3\t4\t1\t\n0\t4\t5\t0\t\n\
         1\t2\t1\t1\t\n0\t7\t1\t1\t\nend\n",
        hash(program.as_bytes()),
        hash(b"\nPRINT \"inc\"\n")
    );
    let read = || text(&std::fs::read(&counts).unwrap()).replace("\r\n", "\n");
    assert_eq!(read(), record);
    run_it(true);
    assert_eq!(
        read(),
        format!("{record}{record}"),
        "a second run appends a second record"
    );
}

/// A `--coverage` build names each statement's built-ins (spec `testing/coverage`, "Built-ins named in the counts";
/// design D1 of `m4-coverage-review`): both of `PRINT LEFT$(a$, LEN(b$))`; an `IF`'s condition names its own, its
/// body's statement its own; a name after `SYSTEM` with the count 0; a member named like a built-in names none, and
/// so does the program's own `FUNCTION beep`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn coverage_build_names_builtins() {
    let d = scratch("coverage-names");
    let program = "$CONSOLE:ONLY\nTYPE t: left AS LONG: END TYPE\nDIM r AS t\na$ = \"xy\": b$ = \"abc\"\n\
                   PRINT LEFT$(a$, LEN(b$)); r.left\nIF LEN(a$) THEN\n    PRINT UCASE$(a$)\nEND IF\nSYSTEM\n\
                   _TITLE \"x\"\nFUNCTION beep\n    beep = 1\nEND FUNCTION\n";
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "--coverage", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let counts = d.join("c.txt");
    let r = run(
        unattended(&exe).current_dir(&d).env("QB64RUST_COVERAGE", &counts),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(stdout(&r).replace("\r\n", "\n"), "xy 0 \nXY\n");
    let record = text(&std::fs::read(&counts).unwrap()).replace("\r\n", "\n");
    let entries: Vec<&str> = record.lines().filter(|l| l.starts_with("0\t")).collect();
    // The program's own FUNCTION beep is no built-in.
    assert!(!record.contains("BEEP"), "{record}");
    let entries: Vec<&str> = entries.into_iter().filter(|l| !l.starts_with("0\t12\t")).collect();
    assert_eq!(
        entries,
        [
            "0\t1\t1\t1\t",
            "0\t4\t1\t1\t",
            "0\t4\t12\t1\t",
            "0\t5\t1\t1\tLEFT$ LEN",
            "0\t6\t4\t1\tLEN",
            "0\t7\t5\t1\tUCASE$",
            "0\t9\t1\t1\t",
            "0\t10\t1\t0\t_TITLE",
        ],
        "{record}"
    );
}

/// `a IMP b IMP c` is `(a IMP b) IMP c` (`QB64PE-DIVERGENCES.md` D-005: the old compiler computes `5 OR 3 OR 0`, 7, so this
/// cannot be a recorded corpus program).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn imp_chain_is_left_to_right() {
    let d = scratch("imp-chain");
    let program = "$CONSOLE:ONLY\nPRINT 5 IMP 3 IMP 0\na = 5: b = 3: c = 0\nPRINT a IMP b IMP c\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 4 \n 4 \n");
}

/// The smallest LONG or `_INTEGER64` divided by -1 raises error 6, and `MOD -1` gives 0 (`QB64PE-DIVERGENCES.md` D-006: the
/// old program crashes).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn smallest_integer_divided_by_minus_one() {
    let d = scratch("min-div");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO h\nl& = -2147483648: m& = -1\nPRINT l& \\ m&\nPRINT l& MOD m&\n\
                   q&& = -9223372036854775808: r&& = -1\nPRINT q&& \\ r&&\nPRINT q&& MOD r&&\n\
                   PRINT 7 \\ m&; -7 MOD m&; l& \\ 2\nSYSTEM\nh:\nPRINT \"[handler\"; ERR; \"]\"\nRESUME NEXT\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-006\n"),
        "[handler 6 ]\n 0 \n[handler 6 ]\n 0 \n-7  0 -1073741824 \n"
    );
}

/// A FUNCTION in a bound of a `REDIM` that gives the same array new bounds or erases it (`QB64PE-DIVERGENCES.md` D-018:
/// the old compiler takes the old block before the bounds and frees it twice, ending the program with no output,
/// `verification\v34_d`). Here the old block is taken after the bounds: plain `REDIM` gives new elements,
/// `_PRESERVE` keeps the FUNCTION's elements by flat position and `_RETAIN` by their indexes.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn redim_with_a_bound_that_redims() {
    let d = scratch("redim-bound");
    let program = "$CONSOLE:ONLY\nREDIM SHARED z(1 TO 3) AS LONG\n\
                   z(3) = 3: REDIM z(2 TO shrinkz) AS LONG: PRINT LBOUND(z); UBOUND(z); z(2); z(3)\n\
                   z(3) = 3: REDIM _PRESERVE z(2 TO shrinkz) AS LONG: PRINT z(2); z(3)\n\
                   z(3) = 3: REDIM _RETAIN z(2 TO shrinkz) AS LONG: PRINT z(2); z(3)\n\
                   z(3) = 3: REDIM _RETAIN z(2 TO erasez) AS LONG: PRINT LBOUND(z); UBOUND(z); z(2); z(3)\n\
                   SYSTEM\nFUNCTION shrinkz\nREDIM z(1 TO 2) AS LONG\nz(2) = 20\nshrinkz = 3\nEND FUNCTION\n\
                   FUNCTION erasez\nERASE z\nerasez = 3\nEND FUNCTION\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-018\n"),
        " 2  3  0  0 \n 0  20 \n 20  0 \n 2  3  0  0 \n"
    );
}

/// `GET` of a whole array whose type has a dynamic member array, into an element whose member has other bounds and
/// fewer elements than declared, raises error 9, reads the rest and keeps that member as it was (`QB64PE-DIVERGENCES.md`
/// D-019: the old compiler writes the declared bytes past the member's block, `verification\v35_e_dyn_statements`
/// 6., `v38_b` 4.).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn get_into_a_dynamic_member_with_other_bounds() {
    let d = scratch("get-dyn-bounds");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\na AS INTEGER\nv(1) _DYNAMIC AS LONG\nEND TYPE\n\
                   ON ERROR GOTO trap\nREDIM p(1) AS t\np(0).a = 1: p(0).v(0) = 10: p(0).v(1) = 11: p(1).a = 2\n\
                   OPEN \"g.bin\" FOR BINARY AS #1\nPUT #1, 1, p()\nREDIM q(1) AS t\n\
                   REDIM q(0).v(5 TO 5) AS LONG: q(0).v(5) = 5\nGET #1, 1, q()\nCLOSE #1\n\
                   PRINT q(0).a; LBOUND(q(0).v); UBOUND(q(0).v); q(0).v(5); q(1).a\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    assert_eq!(build_and_run_tracing(&d, program, "D-019\n"), "[ 9 ] 1  5  5  5  2 \n");
}

/// A `REDIM` of a dynamic member array to more than memory holds ends the program with "Out of memory"
/// (`QB64PE-DIVERGENCES.md` D-020: the old compiler's size overflows unchecked, `verification\v35_g3_dyn_bound_too_large`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn redim_of_a_dynamic_member_too_large() {
    let d = scratch("dyn-too-large");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\nv(1) _DYNAMIC AS _INTEGER64\nEND TYPE\n\
                   ON ERROR GOTO trap\nDIM x AS t\nDIM hi AS _INTEGER64: hi = 2147483647\n\
                   REDIM x.v(0 TO hi, 0 TO hi) AS _INTEGER64\nPRINT \"not reached\"\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    // "Out of memory" goes to stderr; the handler is not called and the program stops there.
    assert_eq!(build_and_run_tracing(&d, program, "D-020\nD-012\n"), "");
}

/// `_MEM` of a dynamic member array that is not allocated, and `GET` of a whole array of its type into an erased
/// array, raise error 9 and go on (`QB64PE-DIVERGENCES.md` D-021: the old program crashes, or ends with "Out of memory",
/// `verification\v35_x66`, `x67`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dynamic_member_not_allocated_in_mem_and_get() {
    let d = scratch("dyn-not-allocated");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\na AS INTEGER\nv(1) _DYNAMIC AS LONG\nEND TYPE\n\
                   ON ERROR GOTO trap\nDIM x AS t\nDIM m AS _MEM\nERASE x.v\nm = _MEM(x.v())\n\
                   PRINT \"mem\"; _MEMEXISTS(m)\nREDIM p(1) AS t\nOPEN \"g.bin\" FOR BINARY AS #1\nPUT #1, 1, p()\n\
                   ERASE p\nGET #1, 1, p()\nCLOSE #1\nPRINT \"get\"\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    // The `GET` into the erased whole array is row 7's (D-042, `fixes-switch`): error 9 before anything is read.
    assert_eq!(
        build_and_run_tracing(&d, program, "D-021\nD-042\n"),
        "[ 9 ]mem 0 \n[ 9 ]get\n"
    );
}

/// D-003 and D-004 differ from the old compiler only in a program that goes on after the error: with no handler and
/// `QB64PE_NOPROMPT=y` the error ends the program, with exit code 1 where the old one exits 0, and the trace names
/// only D-012; with `continue` the program goes on and the trace names both.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn an_error_that_ends_the_program_traces_only_d012() {
    let d = scratch("trace-ends");
    let program = "$CONSOLE:ONLY\nTYPE t\nm AS LONG\nEND TYPE\nDIM a(3) AS t\nPRINT \"before\"\nRETURN back\nback:\n\
                   a(9).m = 5\nPRINT \"after\"\nSYSTEM\n";
    let exe = build_for_errors(&d, program);
    for (noprompt, rows, output, code) in [
        ("y", "D-012\n", "before\n", 1),
        ("continue", "D-003\nD-004\n", "before\nafter\n", 0),
    ] {
        let (run, taken) = run_with_noprompt(&d, &exe, noprompt);
        assert_eq!(stdout(&run).replace("\r\n", "\n"), output, "QB64PE_NOPROMPT={noprompt}");
        assert_eq!(taken, rows, "QB64PE_NOPROMPT={noprompt}");
        assert_eq!(run.status.code(), Some(code), "QB64PE_NOPROMPT={noprompt}");
    }
}

/// Builds `program` as `p.bas` in `d` and returns its executable.
fn build_for_errors(d: &Path, program: &str) -> PathBuf {
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    exe
}

/// Runs `exe` in `d` with `QB64PE_NOPROMPT=noprompt` and the run-time trace on; its output and the trace's rows,
/// without carriage returns.
fn run_with_noprompt(d: &Path, exe: &Path, noprompt: &str) -> (Output, String) {
    let trace = d.join("divergences.txt");
    let _ = std::fs::remove_file(&trace);
    let run = run(
        unattended(exe)
            .current_dir(d)
            .env("QB64PE_NOPROMPT", noprompt)
            .env("QB64RUST_DIVERGENCES", &trace),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    // The trace's rows are ASCII: one character per byte.
    let rows = std::fs::read(&trace).unwrap_or_default();
    let taken = rows.iter().filter(|&&b| b != b'\r').map(|&b| char::from(b)).collect();
    (run, taken)
}

#[expect(clippy::disallowed_methods, reason = "the program's own messages")]
fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).replace("\r\n", "\n")
}

const ERROR_5: &str = "$CONSOLE:ONLY\nPRINT \"before\"\nERROR 5\nSYSTEM\n";

/// Spec `language/error-handling`, "The exit code of a program ended by an error", scenarios "No handler" and "The
/// trace": the program prints the old compiler's message on stderr and exits with code 1 (`QB64PE-DIVERGENCES.md`
/// D-012, where the old one exits 0), and the trace names D-012.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn an_untrapped_error_exits_with_1() {
    let d = scratch("d012-no-handler");
    let exe = build_for_errors(&d, ERROR_5);
    let (run, taken) = run_with_noprompt(&d, &exe, "y");
    assert_eq!(stdout(&run).replace("\r\n", "\n"), "before\n");
    let err = stderr(&run);
    assert!(
        err.contains("Runtime error: Line: 3 (in main module)\nIllegal function call"),
        "{err}"
    );
    assert_eq!(run.status.code(), Some(1));
    assert_eq!(taken, "D-012\n");
}

/// The same spec, "A critical error": a failed `_ASSERT` ends the program with code 1 and traces D-012.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_critical_error_exits_with_1() {
    let d = scratch("d012-critical");
    let exe = build_for_errors(&d, "$CONSOLE:ONLY\n$ASSERTS\n_ASSERT 0\nSYSTEM\n");
    let (run, taken) = run_with_noprompt(&d, &exe, "y");
    let err = stderr(&run);
    assert!(err.contains("_ASSERT failed"), "{err}");
    assert_eq!(run.status.code(), Some(1));
    assert_eq!(taken, "D-012\n");
}

/// The same spec, "Going on after the error" and "The trace": with `QB64PE_NOPROMPT=continue` the program prints the
/// message, goes on to `SYSTEM` and exits with code 0, and the trace does not name D-012.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn going_on_after_an_error_exits_with_0() {
    let d = scratch("d012-continue");
    let exe = build_for_errors(&d, ERROR_5);
    let (run, taken) = run_with_noprompt(&d, &exe, "continue");
    assert!(stderr(&run).contains("Illegal function call"), "{}", stderr(&run));
    assert_eq!(run.status.code(), Some(0));
    assert_eq!(taken, "");
}

/// The same spec, "A trapped error": a handler that does `RESUME NEXT`, then the program runs off its last line and
/// exits with code 0.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_trapped_error_exits_with_0() {
    let d = scratch("d012-trapped");
    let exe = build_for_errors(
        &d,
        "$CONSOLE:ONLY\nON ERROR GOTO h\nGOTO main\nh:\nPRINT \"ERR\"; ERR\nRESUME NEXT\nmain:\nERROR 5\nPRINT \"after\"\n",
    );
    let (run, taken) = run_with_noprompt(&d, &exe, "y");
    assert!(
        stdout(&run).replace("\r\n", "\n").starts_with("ERR 5 \nafter\n"),
        "{}",
        stdout(&run)
    );
    assert_eq!(run.status.code(), Some(0));
    assert_eq!(taken, "");
}

/// `GET` of a whole array whose type has a dynamic member array unpacks the statement's buffer whatever its read
/// raised, as the old compiler does; where that statement never read, the buffer holds zeros (`QB64PE-DIVERGENCES.md`
/// D-021: the old program reads memory nobody wrote, which gave other values on each run, `verification\v38_a`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn get_after_a_failed_read_unpacks_zeros() {
    let d = scratch("get-failed-read");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\na AS INTEGER\nv(1) _DYNAMIC AS LONG\nEND TYPE\n\
                   ON ERROR GOTO trap\nREDIM p(1) AS t\np(0).a = 5: p(1).v(1) = 6\nGET #2, 1, p()\n\
                   PRINT p(0).a; p(1).v(1)\nOPEN \"g.bin\" FOR RANDOM AS #1 LEN = 4\np(0).a = 7\nGET #1, 1, p()\n\
                   PRINT p(0).a\nCLOSE #1\nSYSTEM\ntrap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-021\n"),
        "[ 52 ] 0  0 \n[ 59 ] 0 \n"
    );
}

/// A whole-array assignment to a member array of an element whose index is outside its bounds raises error 9 and
/// copies nothing, a `STRING` member array included (`QB64PE-DIVERGENCES.md` D-004: the old compiler copies the strings
/// into the member of element 0, `verification\v36_x48`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn whole_array_to_a_member_through_an_index_outside() {
    let d = scratch("whole-member-outside");
    let program = "$CONSOLE:ONLY\nTYPE t\nv(1) AS LONG\nw(1) AS STRING\nEND TYPE\nON ERROR GOTO trap\n\
                   DIM e(2) AS t, a(1) AS LONG, s(1) AS STRING\na(1) = 7: s(1) = \"z\"\ni = 5\n\
                   e(i).v() = a(): e(i).w() = s()\nPRINT e(0).v(1); \"[\"; e(0).w(1); \"]\"\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    assert_eq!(build_and_run_tracing(&d, program, "D-004\n"), "[ 9 ][ 9 ] 0 []\n");
}

/// `_ARRAYCOPY` to or from a member array of an element whose index is outside its bounds raises error 9 and copies
/// nothing, numbers and `STRING * n` alike (`QB64PE-DIVERGENCES.md` D-004: the old compiler copies through the member of
/// element 0, `verification\v41_j`; design D5 of `m3-arraycopy`). A FUNCTION in a bound is entered with the error
/// pending and so runs no line (its first line in both compilers).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn arraycopy_through_an_index_outside() {
    let d = scratch("arraycopy-outside");
    let program = "$CONSOLE:ONLY\nTYPE t\nb(1 TO 3) AS LONG\ns(1 TO 2) AS STRING * 2\nEND TYPE\nON ERROR GOTO trap\n\
                   DIM tt(1 TO 2) AS t, src(1 TO 3) AS LONG, d(1 TO 3) AS LONG, fs(1 TO 2) AS STRING * 2\n\
                   src(1) = 7: src(2) = 14: src(3) = 21: fs(1) = \"ab\": fs(2) = \"cd\"\n\
                   tt(1).b(1) = 51: tt(1).s(1) = \"zz\": tt(0 + 1).b(2) = 52\n\
                   _ARRAYCOPY src() TO tt(3).b()\n_ARRAYCOPY fs() TO tt(-5).s()\n\
                   _ARRAYCOPY tt(0).b(1 TO 2) TO d(2)\n_ARRAYCOPY src(1 TO f(2)) TO tt(3).b(1)\n\
                   PRINT tt(1).b(1); tt(1).b(2); \"[\"; tt(1).s(1); \"]\"; d(2)\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n\
                   FUNCTION f (n)\nPRINT \"f\";\nf = n\nEND FUNCTION\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-004\n"),
        "[ 9 ][ 9 ][ 9 ][ 9 ] 51  52 [zz] 0 \n"
    );
}

/// `_ARRAYCOPY` between a `_DYNAMIC` member that a `REDIM` gave another number of dimensions and a member array
/// through an index outside its bounds copies nothing and takes no D-004 trace: the old compiler copies nothing
/// either, its stand-in descriptor being not allocated (read from its C++: `UDTDynArrayDescGuard` never sets slot
/// 2; `verification\v41_l`; design D5 of `m3-arraycopy`). The trace writes a row once per run, so no other statement
/// stands here: a member of its own number beside an index outside, which takes the row, is
/// `arraycopy_through_an_index_outside`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn arraycopy_reshaped_member_beside_an_index_outside() {
    let d = scratch("arraycopy-reshaped");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE dy\n_DYNAMIC n(1 TO 3) AS LONG\nEND TYPE\n\
                   TYPE t\nb(1 TO 3) AS LONG\nEND TYPE\nON ERROR GOTO trap\nDIM v AS dy, tt(1 TO 2) AS t\n\
                   tt(1).b(1) = 51\nREDIM v.n(1 TO 2, 1 TO 2) AS LONG\n\
                   _ARRAYCOPY v.n() TO tt(9).b()\n_ARRAYCOPY tt(9).b() TO v.n()\nPRINT tt(1).b(1)\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    assert_eq!(build_and_run_tracing(&d, program, ""), "[ 9 ][ 9 ] 51 \n");
}

/// `_ARRAYCOPY` of `_FLOAT` arrays copies the elements the program names (`QB64PE-DIVERGENCES.md` D-025: the old compiler
/// passes 32 as the size of an element, which lie 16 bytes apart, and copies others, `verification\v41_i`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn arraycopy_of_float_arrays() {
    let d = scratch("arraycopy-float");
    let program = "$CONSOLE:ONLY\nDIM g1(1 TO 4) AS _FLOAT, g3(0 TO 7) AS _FLOAT\n\
                   DIM h(1 TO 2, 1 TO 2) AS _FLOAT, k(1 TO 2, 1 TO 2) AS _FLOAT\n\
                   FOR i = 1 TO 4: g1(i) = i + 0.25: NEXT\nh(1, 2) = 1.5: h(2, 2) = 2.5\n\
                   _ARRAYCOPY g1(2 TO 3) TO g3(0): PRINT g3(0); g3(1); g3(2)\n\
                   _ARRAYCOPY g1() TO g3(2): PRINT g3(2); g3(5); g3(6)\n\
                   _ARRAYCOPY h(1 TO 2, 2 TO 2) TO k(1, 1): PRINT k(1, 1); k(2, 1); k(1, 2)\nSYSTEM\n";
    assert_eq!(
        build_and_run(&d, program),
        " 2.25  3.25  0 \n 1.25  4.25  0 \n 1.5  2.5  0 \n"
    );
}

/// Event handlers built and run (design D4 of `m3-events`): a SUB with its argument, then a label of the main
/// module set up from a SUB, after an error handler, a loop and a `SELECT`, which `QBMAIN` is re-entered to reach.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn event_handlers_run() {
    let d = scratch("event-handlers");
    let program = "$CONSOLE:ONLY\nDIM SHARED n AS LONG\nON ERROR GOTO oops\nERROR 5\n\
                   FOR i = 1 TO 2\nSELECT CASE i\nCASE 1: n = 0\nEND SELECT\nNEXT\n\
                   ON TIMER(0.3) show 7\nTIMER ON: _DELAY 0.45: TIMER OFF\n\
                   setup\nTIMER ON: _DELAY 0.45: TIMER OFF\nPRINT n\nSYSTEM\n\
                   oops:\nPRINT \"e\"; ERR\nRESUME NEXT\ntick:\nn = n + 1\nPRINT \"tick\"\nRETURN\n\
                   SUB show (k AS INTEGER)\nPRINT \"show\"; k\nEND SUB\n\
                   SUB setup\nON TIMER(0.3) GOSUB tick\nEND SUB\n";
    assert_eq!(build_and_run(&d, program), "e 5 \nshow 7 \ntick\n 1 \n");
}

/// `ON STRIG` built and run (design D4 of `m4-devices-continue-paths`; `verification\v47_x13`, `x30`): its C++
/// builds with libqb's device-input part, a label and a SUB handler set up, the switch, and an odd button number
/// raising error 5 in libqb. No game controller fires an event.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn strig_handlers_build_and_run() {
    let d = scratch("strig-handlers");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO oops\nON STRIG(0) GOSUB h\nON STRIG(4, 2) s 3\nSTRIG(0) ON\n\
                   ON STRIG(1) GOSUB h\nPRINT _DEVICES > 1\nSYSTEM\n\
                   oops:\nPRINT \"e\"; ERR\nRESUME NEXT\nh:\nRETURN\n\
                   SUB s (k AS INTEGER)\nPRINT k\nEND SUB\n";
    assert_eq!(build_and_run(&d, program), "e 5 \n-1 \n");
}

/// `LEN` of a whole array built and run (design D5 of `m4-names-arrays`; `verification\v48_d_len_whole`): a
/// static, a fixed-length-string, a user-type and a whole member array, a dynamic one before and after `ERASE`, and
/// an array parameter.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn len_of_whole_arrays_builds_and_runs() {
    let d = scratch("len-whole");
    let program = "$CONSOLE:ONLY\nTYPE test\na(1 TO 50) AS LONG\nEND TYPE\n\
                   DIM a(3 TO 7) AS INTEGER, f(4) AS STRING * 3, parent(1) AS test\n\
                   n = 7\nREDIM d(3 TO n) AS INTEGER\n\
                   PRINT LEN(a()); LEN(f()); LEN(parent(0).a()); LEN(parent()); LEN(d())\n\
                   ERASE d\nPRINT LEN(d())\nparam a()\nSYSTEM\n\
                   SUB param (x() AS INTEGER)\nPRINT LEN(x())\nEND SUB\n";
    assert_eq!(build_and_run(&d, program), " 10  15  200  400  10 \n 0 \n 10 \n");
}

/// `_CAST` built and run (design D2 of `m4-builtins-param-types`; `verification\v49_a_cast`): an integer narrowed
/// to its low bits, a float truncated toward zero, the result in arithmetic and in `HEX$` as the type named.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn cast_builds_and_runs() {
    let d = scratch("cast");
    let program = "$CONSOLE:ONLY\nn& = 300: f! = -6.75\n\
                   PRINT _CAST(_BYTE, n&); _CAST(INTEGER, 2.5); _CAST(LONG, f!); _CAST(_UNSIGNED _BYTE, -1) + 1\n\
                   PRINT HEX$(_CAST(_BYTE, -1)); \" \"; HEX$(_CAST(INTEGER, -1))\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 44  2 -6  256 \nFF FFFF\n");
}

/// A whole-array assignment of arrays of an owner type with other bounds raises error 9 and copies the target's
/// number of elements, as the old compiler does, but nothing when the source has fewer (`QB64PE-DIVERGENCES.md` D-022: the
/// old compiler reads past the source's block, `verification\v36_x52`, `v38_d`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn whole_array_of_an_owner_type_with_other_bounds() {
    let d = scratch("whole-owner-bounds");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE own\nv(1) _DYNAMIC AS LONG\nk AS LONG\nEND TYPE\n\
                   ON ERROR GOTO trap\nDIM s(1 TO 2) AS own, d(0 TO 1) AS own, e(1 TO 3) AS own\n\
                   s(1).k = 4: s(2).k = 5: d(0).k = 9: e(1).k = 8\nd() = s(): PRINT d(0).k; d(1).k\n\
                   e() = s(): PRINT e(1).k; e(2).k; e(3).k\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-022\n"),
        "[ 9 ] 4  5 \n[ 9 ] 8  0  0 \n"
    );
}

/// `_MEM` of an element of a dynamic member array, where an index of the value the member is part of reads another
/// dynamic member, takes the element's own lock: `ERASE` of the other member leaves the block, `ERASE` of its own
/// ends it (`verification\v35_x66` 13. for the lock).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn mem_of_a_dynamic_element_through_a_dynamic_index() {
    let d = scratch("dyn-mem-index");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\na AS INTEGER\nv(1) _DYNAMIC AS LONG\nEND TYPE\n\
                   DIM x AS t\nDIM m AS _MEM\nREDIM p(1) AS t\nx.v(0) = 1\nm = _MEM(p(x.v(0)).v(1))\n\
                   ERASE x.v\ne = _MEMEXISTS(m)\nERASE p(1).v\nPRINT e; _MEMEXISTS(m)\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), "-1  0 \n");
}

/// `_MEM(q())` of an array parameter that nothing above it has given its number of dimensions takes the array's
/// own lock, so the block stays valid when an element changes (`QB64PE-DIVERGENCES.md` D-016: the old compiler reads the
/// lock from slot 0, the pointer to the elements, and the block ends at `q(0) = 6`, `verification\v32_a_blocks`
/// SUB `y`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn mem_of_an_array_parameter_with_no_number_yet() {
    let d = scratch("mem-param-slot");
    let program = "$CONSOLE:ONLY\nCALL zz\nSYSTEM\nSUB y (q() AS LONG)\nDIM m AS _MEM\nm = _MEM(q())\nq(0) = 6\n\
                   PRINT _MEMEXISTS(m)\nEND SUB\nSUB zz\nDIM q2(3) AS LONG\nCALL y(q2())\nEND SUB\n";
    assert_eq!(build_and_run_tracing(&d, program, "D-016\n"), "-1 \n");
}

/// An integer power beyond `_INTEGER64` in a `CONST` is the float it is, a `_FLOAT` by its text, printed as a DOUBLE
/// (`QB64PE-DIVERGENCES.md` D-007: the old evaluator wraps it to -9223372036854775808).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn const_power_beyond_integer64() {
    let d = scratch("const-pow");
    let program = "$CONSOLE:ONLY\nCONST c = 2 ^ 70\nPRINT c\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 1.180591620717411D+21 \n");
}

/// The `CONST` rulings of 2026-10-18, built and run: an unsuffixed literal beyond `_INTEGER64` and the limits
/// (`QB64PE-DIVERGENCES.md` D-027; the old compiler prints 9223372036854775807 and -9223372036854775807), `2 ^ -1` (D-028;
/// 0 there), and `_COTH` at run time from the edited copy of libqb (D-031; `2 * EXP(x) - 1` and error 5 above
/// 44.014845 there), error 5 at 0.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn const_rulings_run() {
    let d = scratch("const-rulings");
    let program = "$CONSOLE:ONLY\nCONST plain = 18446744073709551615, half = 2 ^ -1\n\
                   PRINT plain; half; _UINTEGER64_MAX; _INTEGER64_MIN\n\
                   x# = .2: y# = -.5: z# = 45: PRINT CLNG(_COTH(x#) * 1000); CLNG(_COTH(y#) * 1000); _COTH(z#)\n\
                   ON ERROR GOTO h\nPRINT _COTH(0)\nSYSTEM\nh: PRINT \"error\"; ERR: RESUME NEXT\n";
    assert_eq!(
        build_and_run_tracing(&d, program, "D-031\n"),
        " 18446744073709551615  .5  18446744073709551615 -9223372036854775808 \n 5066 -2164  1 \nerror 5 \n"
    );
}

/// `STOP` inside `SUB _GL` ends the program with error 271, as `END` and `SYSTEM` do there (`QB64PE-DIVERGENCES.md` D-036;
/// the old compiler's program hangs, `verification\v59_f9`). The program has a window, which libqb shows for the
/// moment the SUB takes; its critical error's message goes to stderr, not read here.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn stop_inside_sub_gl_ends_with_error_271() {
    let d = scratch("stop-gl");
    let program = "$CONSOLE\nt# = TIMER(.001)\nDO: _LIMIT 100: LOOP UNTIL TIMER(.001) - t# > 20\n\
                   _DEST _CONSOLE: PRINT \"main\"\nSYSTEM\n\
                   SUB _GL\n    _DEST _CONSOLE: PRINT \"in\"\n    STOP\nEND SUB\n";
    assert_eq!(build_and_run_tracing(&d, program, "D-036\nD-012\n"), "in\n");
}

/// A `TYPE` that holds a `STRING`, in conventional memory, has its strings made (`QB64PE-DIVERGENCES.md` D-037; design D3
/// of `m4-left-marked`): `verification\v56_a5` prints the old compiler's two lines, then the string it sets, where
/// the old compiler's program crashes (exit 139).
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[expect(
    clippy::disallowed_methods,
    reason = "the program and its record are ASCII text, compared as lines"
)]
fn type_with_string_in_conventional_memory_has_its_strings() {
    let d = scratch("cmem-type-string");
    let verification = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../verification");
    let read = |name: &str| String::from_utf8(std::fs::read(verification.join(name)).unwrap()).unwrap();
    let program = read("v56_a5_type_string_member.bas");
    let recorded = read("v56_a5_type_string_member.out.txt");
    let first: Vec<&str> = recorded.lines().take(2).collect();
    let out = build_and_run_tracing(&d, &program, "D-037\n");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[..2], first[..], "{out}");
    assert_eq!(lines[2..], ["abc 3 "], "{out}");
}

/// A label and a `CONST` on one line (`QB64PE-DIVERGENCES.md` D-008: the old compiler fails to compile it).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn label_then_const() {
    let d = scratch("label-const");
    let program = "$CONSOLE:ONLY\nlbl1: CONST k = 4\nPRINT k: GOTO done\nPRINT \"skipped\"\ndone:\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 4 \n");
}

/// Each call of a FUNCTION `f$n` gives its own string (`QB64PE-DIVERGENCES.md` D-014: the old compiler returns bytes the
/// FUNCTION has released, so `fs$5("ab") + fs$5("cd")` is `cd   cd   ` there).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn fixed_function_results_are_copies() {
    let d = scratch("fixed-function");
    let program = "$CONSOLE:ONLY\nPRINT \"[\"; fs$5(\"ab\") + fs$5(\"cd\"); \"]\"\n\
                   x$ = fs$5(\"12\") + fs$5(\"34\"): PRINT \"[\"; x$; \"]\"; LEN(x$)\nSYSTEM\n\
                   FUNCTION fs$5 (x AS STRING)\nfs$5 = x\nEND FUNCTION\n";
    assert_eq!(build_and_run(&d, program), "[ab   cd   ]\n[12   34   ] 10 \n");
}

/// The chained-`^` warning: printed with `-w` at the `CONST`'s line, not counted, the executable built (scenario
/// "Warning shown with -w"); not printed without `-w` (scenario "Warning hidden without -w").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn chained_power_warning() {
    let d = scratch("pow-warning");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nCONST c = 2 ^ 3 ^ 2\nPRINT c\nSYSTEM\n").unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-w", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    let out = stdout(&o);
    assert!(
        out.lines().any(|l| l.starts_with("p.bas:2:") && l.contains("warning:")),
        "{out}"
    );
    assert!(!out.contains("error"), "{out}");
    assert!(exe.is_file());
    let run = run(unattended(&exe).current_dir(&d), None, PROGRAM_LIMIT, "program");
    assert_eq!(stdout(&run).replace("\r\n", "\n"), " 512 \n");

    std::fs::remove_file(&exe).unwrap();
    let o = qb64rust_build(&d, &["-q", "-m", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(!stdout(&o).contains("warning"), "{}", stdout(&o));
    assert!(exe.is_file());
}

/// No warning for a parenthesised chain, with `-w` (the constants delta's scenario "Parenthesised chain"); the C++
/// only, so no clone is needed.
#[test]
fn parenthesised_power_no_warning() {
    let d = scratch("pow-parens");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\nCONST c = (2 ^ 3) ^ 2, e = 2 ^ (3 ^ 2)\nPRINT c; e\nSYSTEM\n",
    )
    .unwrap();
    let o = qb64rust(&d, &["-w", "-z", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(!stdout(&o).contains("warning"), "{}", stdout(&o));
    std::fs::write(d.join("q.bas"), "$CONSOLE:ONLY\nCONST c = 2 ^ 3 ^ 2\nSYSTEM\n").unwrap();
    let o = qb64rust(&d, &["-w", "-z", "q.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(stdout(&o).starts_with("q.bas:2:11: warning:"), "{}", stdout(&o));
}

/// `$DEBUG` writes no debugger code and, with `-w`, gives the old compiler's warning at its line (spec
/// `language/asserts-and-debug`, "The warning"; `qb64pe.bas` 3556, read by hand in task 1.1 of
/// `m3-asserts-debug-zlib`); without `-w` there is none. The C++ only, so no clone is needed.
#[test]
fn debug_warning() {
    let d = scratch("debug-meta");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT 1\n$DEBUG\nSYSTEM\n").unwrap();
    let o = qb64rust(&d, &["-w", "-z", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(
        stdout(&o)
            .lines()
            .any(|l| l == "p.bas:3:1: warning: $DEBUG features only work from the IDE"),
        "{}",
        stdout(&o)
    );
    let o = qb64rust(&d, &["-z", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(!stdout(&o).contains("warning"), "{}", stdout(&o));
}

/// A program with `$DEBUG` and no network function links libqb's sockets and HTTP part, as the old compiler's
/// build does (spec `language/asserts-and-debug`, "The sockets part"; `QB64PE-DIVERGENCES.md` D-017, narrowed by
/// `m3-network-embed`): `_SOCKETS_` is 1, so QB64pe's file included after the last line compiles `_WhatIsMyIP$`,
/// and the licence file names libcurl. The program prints what it prints without `$DEBUG`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn debug_links_the_sockets_part() {
    let d = scratch("debug-sockets");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\n$IF _SOCKETS_ THEN\nPRINT \"on\"\n$END IF\nSYSTEM\n$DEBUG\n",
    )
    .unwrap();
    let mut cmd = compiler();
    cmd.current_dir(&d)
        .args(["-f:GenerateLicenseFile=true", "-q", "-m", "-x", "p.bas", "-o", "p.exe"]);
    let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    let license = std::fs::read(d.join("p.exe.license.txt")).unwrap();
    let has = |what: &str| license.windows(what.len()).any(|w| w == what.as_bytes());
    assert!(has("License of libcurl"));
    let o = run(
        unattended(&d.join("p.exe")).current_dir(&d),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert_eq!(stdout(&o).replace("\r\n", "\n"), "on\n");
}

/// `$UNSTABLE:MIDI` and `$UNSTABLE:HTTP` compile and, with `-w`, give the old compiler's warning at their line
/// (spec `language/unstable-features`, "Stable flags"; `qb64pe.bas` 3891, 3895); `TYPEFIELDS` gives none. The C++
/// only, so no clone is needed.
#[test]
fn unstable_stable_flag_warning() {
    let d = scratch("unstable-meta");
    let src = "$CONSOLE:ONLY\n$UNSTABLE:MIDI\n$unstable: http\n$UNSTABLE:TYPEFIELDS\nPRINT 1\nSYSTEM\n";
    std::fs::write(d.join("p.bas"), src).unwrap();
    let o = qb64rust(&d, &["-w", "-z", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    let out = stdout(&o);
    let warnings: Vec<&str> = out.lines().filter(|l| l.contains("warning")).collect();
    let expected = [
        "p.bas:2:1: warning: No longer required, feature is stable now",
        "p.bas:3:1: warning: No longer required, feature is stable now",
    ];
    assert_eq!(warnings, expected, "{out}");
    let o = qb64rust(&d, &["-z", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(!stdout(&o).contains("warning"), "{}", stdout(&o));
}

/// A `$VERSIONINFO` string value without enclosing `'` compiles and, with `-w`, gives the old compiler's warning
/// with the value on the next line (spec `language/console-and-window`, "A value without quotes"; measured,
/// `verification\v40_d`, its `v40_rc\v40_d_forms.z.txt`). The C++ only, so no clone is needed.
#[test]
fn versioninfo_missing_quotes_warning() {
    let d = scratch("versioninfo-quotes");
    let src = "$CONSOLE:ONLY\n$VERSIONINFO:CompanyName='quoted'\n$VERSIONINFO:ProductName=no quotes\n\
               $VERSIONINFO:Comments='a' ' trailing comment\nPRINT 1\nSYSTEM\n";
    std::fs::write(d.join("p.bas"), src).unwrap();
    let o = qb64rust(&d, &["-w", "-z", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    let out = stdout(&o);
    let at = out.find("p.bas:3:1: warning").unwrap_or_else(|| panic!("{out}"));
    let expected = "p.bas:3:1: warning: Missing string bracket delimiters ('')\n    no quotes\n\
                    p.bas:4:1: warning: Missing string bracket delimiters ('')\n    'a' ' trailing comment\n";
    assert!(out[at..].replace("\r\n", "\n").starts_with(expected), "{out}");
    let o = qb64rust(&d, &["-z", "p.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(!stdout(&o).contains("warning"), "{}", stdout(&o));
}

/// Deeply nested expressions are one "not supported yet" error, not a stack overflow (the known bug of
/// 2026-10-07). Through the binary, which runs on its own large stack; a test thread's 2 MiB would not hold the
/// later walks of an expression 1,000 levels deep in a debug build.
#[test]
fn deep_expressions() {
    let d = scratch("deep");
    let depth = 990;
    let cases = [
        ("chain_ok", format!("x = 1{}", " + 1".repeat(depth)), true),
        (
            "parens_ok",
            format!("x = {}1{}", "(".repeat(depth), ")".repeat(depth)),
            true,
        ),
        ("chain", format!("x = 1{}", " + 1".repeat(20_000)), false),
        (
            "parens",
            format!("x = {}1{}", "(".repeat(3_000), ")".repeat(3_000)),
            false,
        ),
        // Not minus signs: pairs of them after an operator are dropped, as the old compiler does.
        ("negations", format!("x = {}1", "NOT ".repeat(20_000)), false),
        ("members", format!("x = a(1){}", ".b".repeat(20_000)), false),
        ("target", format!("a(1){} = 1", ".b".repeat(20_000)), false),
        // A call without `CALL`, whose arguments are parsed with errors held back.
        (
            "quiet_args",
            format!("CLS {}1{}", "(".repeat(3_000), ")".repeat(3_000)),
            false,
        ),
    ];
    for (name, line, fits) in cases {
        let file = format!("{name}.bas");
        std::fs::write(d.join(&file), format!("$CONSOLE:ONLY\n{line}\nPRINT x\n")).unwrap();
        let o = qb64rust(&d, &["--dump", "typed", &file]);
        let out = stdout(&o);
        assert!(
            o.stderr.is_empty(),
            "{name}: no stack overflow or other message on stderr"
        );
        if fits {
            assert_eq!(o.status.code(), Some(0), "{name}: {out}");
        } else {
            assert_eq!(o.status.code(), Some(1), "{name}: {out}");
            assert!(
                out.contains("error: not supported yet: an expression nested more than 1000 levels deep\n1 error (1 not supported yet)"),
                "{name}: {out}"
            );
        }
    }
}

/// Scenario "Forced panic" (change `m2-parser-breadth`, D11).
#[test]
fn internal_compiler_error() {
    let d = scratch("ice");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    let run = |args: &[&str]| {
        support::run(
            compiler().current_dir(&d).env("QB64RUST_TEST_PANIC", "1").args(args),
            None,
            COMPILER_LIMIT,
            "compiler",
        )
    };

    std::fs::write(d.join(default_exe("p")), b"stale").unwrap();
    let o = run(&["-q", "p.bas"]);
    assert_eq!(o.status.code(), Some(3));
    let out = stdout(&o);
    assert!(
        out.starts_with("qb64rust: internal compiler error: QB64RUST_TEST_PANIC=1 is set at "),
        "{out}"
    );
    assert!(out.contains("while compiling p.bas"), "{out}");
    assert!(o.stderr.is_empty(), "the default panic message is replaced");
    assert!(!d.join(default_exe("p")).exists(), "no executable left behind");

    // A dump never touches the executable.
    std::fs::write(d.join(default_exe("p")), b"kept").unwrap();
    let o = run(&["--dump", "typed", "p.bas"]);
    assert_eq!(o.status.code(), Some(3));
    assert!(d.join(default_exe("p")).exists());
}

/// Scenario "Subcommand" (change `m2-language-server`): the language server over stdin and stdout, ending with
/// exit code 0 after `shutdown` and `exit`.
#[test]
fn language_server() {
    let d = scratch("lsp");
    let messages = [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":null,"capabilities":{}}}"#,
        r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"shutdown"}"#,
        r#"{"jsonrpc":"2.0","method":"exit"}"#,
    ];
    let input: String = messages
        .iter()
        .map(|m| format!("Content-Length: {}\r\n\r\n{m}", m.len()))
        .collect();
    let o = run(
        compiler().current_dir(&d).arg("lsp"),
        Some(input.as_bytes()),
        COMPILER_LIMIT,
        "language server",
    );
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(0), "{out}");
    assert!(
        out.contains(r#""id":1"#) && out.contains("documentSymbolProvider"),
        "{out}"
    );
    assert!(out.contains(r#""id":2"#), "{out}");

    // Only the exact word, alone.
    let o = qb64rust(&d, &["lsp", "extra"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(o.stdout.is_empty(), "stdout is the protocol's");
}

/// `dap` (changes `m5-dap-proxy`, `m6-toolchain`), scenario "Another option": only `--toolchain-root` and
/// `--qb64pe-root` are taken, each once; and "Not a file name": `dap.bas` is a program.
#[test]
#[expect(clippy::disallowed_methods, reason = "the compiler's own messages")]
fn dap_rejects_other_options() {
    let d = scratch("dap-options");
    for args in [
        &["dap", "-x", "p.bas"][..],
        &["dap", "--qb64pe-root"],
        &["dap", "--toolchain-root"],
        &["dap", "--qb64pe-root", "a", "--qb64pe-root", "b"],
        &["dap", "--toolchain-root", "a", "--toolchain-root", "b"],
    ] {
        let o = qb64rust(&d, args);
        assert_eq!(o.status.code(), Some(1), "{args:?}");
        assert!(o.stdout.is_empty(), "stdout is the protocol's");
        assert!(
            String::from_utf8_lossy(&o.stderr)
                .contains("`dap` takes only --toolchain-root <dir> and --qb64pe-root <dir>"),
            "{args:?}"
        );
    }
    std::fs::write(d.join("dap.bas"), SLICE_PROGRAM).unwrap();
    let o = qb64rust(&d, &["dap.bas", "--dump", "typed"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).contains("INSTR"), "{}", stdout(&o));
}

/// `dap`, spec `editor/debug-adapter` scenario "No lldb-dap": a toolchain folder without `bin\lldb-dap.exe` given by
/// `--toolchain-root`; `initialize` fails naming that path, and `disconnect` ends the adapter with code 0.
#[test]
#[cfg(windows)]
fn dap_without_lldb_dap() {
    let d = scratch("dap-no-lldb-dap");
    fake_toolchain(&d.join("tools"));
    let mut cmd = compiler();
    cmd.current_dir(&d).args(["dap", "--toolchain-root", "tools"]);
    let mut dap = support::dap::Dap::start(cmd);
    let r = dap.request("initialize", serde_json::json!({"adapterID": "qb64rust"}));
    assert_eq!(r["success"], false, "{r}");
    let want = std::path::absolute(d.join("tools"))
        .unwrap()
        .join("bin")
        .join("lldb-dap.exe");
    assert!(
        r["message"].as_str().unwrap().contains(&want.display().to_string()),
        "{r}"
    );
    dap.ok("disconnect", serde_json::json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// `dap`, spec `editor/debug-adapter` scenario "No toolchain": `initialize` fails with the message naming
/// `qb64rust toolchain install`.
#[test]
#[cfg(windows)]
fn dap_without_a_toolchain() {
    let d = scratch("dap-no-toolchain");
    let mut cmd = compiler();
    without_qb64pe(&mut cmd, &d.join("home")).current_dir(&d).arg("dap");
    let mut dap = support::dap::Dap::start(cmd);
    let r = dap.request("initialize", serde_json::json!({"adapterID": "qb64rust"}));
    assert_eq!(r["success"], false, "{r}");
    assert!(
        r["message"].as_str().unwrap().contains("qb64rust toolchain install"),
        "{r}"
    );
    dap.ok("disconnect", serde_json::json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// `dap`, spec `editor/debug-adapter` scenario "No lldb-dap on Linux": a system toolchain on `PATH` but no `lldb-dap`
/// or `lldb-dap-<n>`; `initialize` fails naming `lldb-dap` and the package, and `disconnect` ends the adapter with
/// code 0.
#[test]
#[cfg(unix)]
fn dap_without_lldb_dap_on_linux() {
    let d = scratch("dap-no-lldb-dap-linux");
    let bin = d.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    for f in ["g++", "make", "objcopy"] {
        std::fs::write(bin.join(f), "").unwrap();
    }
    let mut cmd = compiler();
    cmd.current_dir(&d)
        .env("PATH", &bin)
        .env_remove("QB64RUST_TOOLCHAIN_ROOT")
        .arg("dap");
    let mut dap = support::dap::Dap::start(cmd);
    let r = dap.request("initialize", serde_json::json!({"adapterID": "qb64rust"}));
    assert_eq!(r["success"], false, "{r}");
    let message = r["message"].as_str().unwrap();
    assert!(
        message.contains("lldb-dap not found") && message.contains("lldb"),
        "{r}"
    );
    if on_debian() {
        assert!(message.contains("`sudo apt-get install lldb-20`"), "{r}");
    }
    dap.ok("disconnect", serde_json::json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// Debugging with no QB64pe folder and the toolchain given by `--toolchain-root` (the clone's own, which is the
/// pinned release byte for byte): a debug build, then `qb64rust dap` with that toolchain stops at a breakpoint on
/// line 3, and the adapter ends with code 0 after `disconnect` (spec `compiler/cli`, "Debug adapter subcommand";
/// spec `editor/debug-launch`, "Debugging without QB64pe").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_without_a_qb64pe_folder() {
    use serde_json::json;
    let d = scratch("dap-noqb64pe");
    let tools = support::toolchain_folder();
    std::fs::write(
        d.join("hello.bas"),
        "$CONSOLE:ONLY\nx = 1\nx = x + 1\nPRINT x\nSYSTEM\n",
    )
    .unwrap();
    let mut cmd = compiler();
    without_qb64pe(&mut cmd, &d.join("home"))
        .current_dir(&d)
        .env_remove("QB64RUST_BUILD_CACHE")
        .arg("--toolchain-root")
        .arg(&tools)
        .args(["--debug", "-q", "hello.bas"]);
    let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
    assert!(o.status.success(), "{}", stdout(&o));
    let mut cmd = compiler();
    without_qb64pe(&mut cmd, &d.join("home"))
        .current_dir(&d)
        .arg("dap")
        .arg("--toolchain-root")
        .arg(&tools);
    let mut dap = launch_session(
        support::dap::Dap::start(cmd),
        &d.join(default_exe("hello")),
        &[],
        &[3],
        json!({}),
    );
    let (_, reason, name, line) = dap_stop(&mut dap);
    assert_eq!((reason.as_str(), name.as_str(), line), ("breakpoint", "Main module", 3));
    let t = dap.ok("threads", json!({}));
    dap.ok("continue", json!({"threadId": t["threads"][0]["id"]}));
    dap.event("exited");
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// `dap`, scenario "Subcommand": the real `lldb-dap` answers `initialize` through the adapter, which adds its own
/// capabilities, and the adapter ends with exit code 0 after `disconnect`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_initialize_and_disconnect() {
    let d = scratch("dap-init");
    let mut cmd = compiler();
    cmd.current_dir(&d).arg("dap");
    let mut dap = support::dap::Dap::start(cmd);
    let caps = dap.ok(
        "initialize",
        serde_json::json!({"adapterID": "qb64rust", "linesStartAt1": true}),
    );
    assert_eq!(caps["supportsGotoTargetsRequest"], true, "{caps}");
    assert_eq!(caps["supportsDataBreakpoints"], true, "lldb-dap's own: {caps}");
    assert_eq!(caps["exceptionBreakpointFilters"][0]["label"], "BASIC runtime errors");
    dap.ok("disconnect", serde_json::json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// A breakpoint in a file included from a subfolder (`inc/lib.bi`) stops there, the frame naming that file by its
/// path: the source paths of `<exe>.debug.json` and the symbol reader on either platform (task 4.3 of `m7-linux`; on
/// Linux the paths are case-sensitive and use `/`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_breakpoint_in_a_file_included_from_a_subfolder() {
    use serde_json::json;
    let d = scratch("dap-include-folder");
    std::fs::create_dir_all(d.join("inc")).unwrap();
    std::fs::write(
        d.join("inc").join("lib.bi"),
        "FUNCTION twice& (k AS LONG)\n    twice& = 2 * k\nEND FUNCTION\n",
    )
    .unwrap();
    let exe = debug_build(
        &d,
        "main",
        "$CONSOLE:ONLY\ny& = twice&(3)\nPRINT y&\nSYSTEM\n'$INCLUDE:'inc/lib.bi'\n",
    );
    let mut cmd = compiler();
    cmd.current_dir(&d).arg("dap");
    let mut dap = support::dap::Dap::start(cmd);
    dap.ok(
        "initialize",
        json!({"adapterID": "qb64rust", "linesStartAt1": true, "columnsStartAt1": true}),
    );
    let launch = dap.send(
        "launch",
        json!({"program": exe, "cwd": d.to_path_buf(), "env": {"QB64PE_NOPROMPT": "y"}}),
    );
    dap.event("initialized");
    let lib = d.join("inc").join("lib.bi");
    let set = dap.ok(
        "setBreakpoints",
        json!({"source": {"path": lib}, "breakpoints": [{"line": 2}]}),
    );
    assert_eq!(set["breakpoints"][0]["verified"], true, "{set}");
    dap.ok("configurationDone", json!({}));
    assert_eq!(dap.response(launch)["success"], true);
    let (s, reason, name, line) = dap_stop(&mut dap);
    assert_eq!(
        (reason.as_str(), name.as_str(), line),
        ("breakpoint", "FUNCTION twice&", 2)
    );
    let st = dap.ok("stackTrace", json!({"threadId": s["body"]["threadId"], "levels": 1}));
    let source = &st["stackFrames"][0]["source"];
    assert_eq!(source["name"], "lib.bi", "{st}");
    assert_eq!(
        Path::new(source["path"].as_str().unwrap()).canonicalize().unwrap(),
        lib.canonicalize().unwrap(),
        "{st}"
    );
    dap.ok("continue", json!({"threadId": s["body"]["threadId"]}));
    dap.event("exited");
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// A session of `qb64rust dap` on a debug build: `initialize`, then `launch` with `extra` merged into its
/// arguments, the function breakpoints and the breakpoints by line of the main file (`lines`), and
/// `configurationDone`. The program then runs.
fn dap_launch(exe: &Path, functions: &[&str], lines: &[u64], extra: serde_json::Value) -> support::dap::Dap {
    let mut cmd = compiler();
    cmd.current_dir(exe.parent().unwrap()).arg("dap");
    launch_session(support::dap::Dap::start(cmd), exe, functions, lines, extra)
}

/// A session of the toolchain's `lldb-dap` itself (the one the proxy starts), with no proxy, in the environment the
/// proxy gives it (the measurements of task 3.1 of `m5-dap-proxy`).
#[cfg(windows)]
fn lldb_dap_launch(exe: &Path, functions: &[&str], lines: &[u64]) -> support::dap::Dap {
    use qb64rust_driver::toolchain;
    let found = toolchain::find(qb64rust_base::Platform::host(), None, None).expect("a C++ toolchain");
    let mut cmd = Command::new(toolchain::lldb_dap(&found, None).expect("lldb-dap"));
    cmd.current_dir(exe.parent().unwrap())
        .envs(qb64rust_dap::LLDB_ENV.iter().copied());
    launch_session(
        support::dap::Dap::start(cmd),
        exe,
        functions,
        lines,
        serde_json::json!({}),
    )
}

fn launch_session(
    mut dap: support::dap::Dap,
    exe: &Path,
    functions: &[&str],
    lines: &[u64],
    extra: serde_json::Value,
) -> support::dap::Dap {
    use serde_json::json;
    dap.ok(
        "initialize",
        json!({"adapterID": "qb64rust", "linesStartAt1": true, "columnsStartAt1": true}),
    );
    let mut args = json!({"program": exe, "cwd": exe.parent().unwrap(), "env": {"QB64PE_NOPROMPT": "y"}});
    if let (Some(a), Some(e)) = (args.as_object_mut(), extra.as_object()) {
        a.extend(e.clone());
    }
    let launch = dap.send("launch", args);
    dap.event("initialized");
    if !functions.is_empty() {
        let bps: Vec<_> = functions.iter().map(|f| json!({"name": f})).collect();
        dap.ok("setFunctionBreakpoints", json!({"breakpoints": bps}));
    }
    if !lines.is_empty() {
        let bas = exe.with_extension("bas");
        let bps: Vec<_> = lines.iter().map(|l| json!({"line": l})).collect();
        dap.ok("setBreakpoints", json!({"source": {"path": bas}, "breakpoints": bps}));
    }
    dap.ok("configurationDone", json!({}));
    let r = dap.response(launch);
    assert_eq!(r["success"], true, "launch: {r}");
    dap
}

/// The top frame of the thread of a `stopped` event: its name, line and column.
#[cfg(windows)]
fn dap_top(dap: &mut support::dap::Dap, stopped: &serde_json::Value) -> (String, u64, u64) {
    let thread = stopped["body"]["threadId"].clone();
    let st = dap.ok("stackTrace", serde_json::json!({"threadId": thread, "levels": 1}));
    let f = &st["stackFrames"][0];
    (
        f["name"].as_str().unwrap_or("").to_string(),
        f["line"].as_u64().unwrap_or(0),
        f["column"].as_u64().unwrap_or(0),
    )
}

/// Measured (task 3.1 of `m5-dap-proxy`, design "Measured"), the clone's `lldb-dap` alone: a function breakpoint on
/// `QBMAIN` stops in `qbx.cpp`, before any BASIC line; `evaluate` of `(unsigned long long)<c_name>` gives the
/// address in decimal, `readMemory` there the value's bytes (base64), `writeMemory` changes it; `dataBreakpointInfo`
/// of one of `lldb-dap`'s own variables gives a `dataId` `<hex address>/<size>`; libqb's `qbs_set` and
/// `qbs_new_txt_len`, which have no debug information, can be called through function pointers of their C types;
/// `` `thread jump --line n`` in the `repl` context moves the next line. Windows only: it measures the toolchain's
/// `lldb-dap`; Ubuntu's 19 and 20 have no `writeMemory` and end when sent one (`m7-linux` design, "Measured"), which
/// the adapter's own tests cover (`set_value_without_write_memory`).
#[test]
#[cfg(windows)]
#[ignore = "needs the QB64pe reference clone"]
fn lldb_dap_measured_memory() {
    use serde_json::json;
    let d = scratch("dap-measure");
    let program = "$CONSOLE:ONLY\nDIM SHARED total AS LONG\ns$ = \"hi\"\ntotal = 7\ntotal = total + 1\n\
                   PRINT total; s$\nSYSTEM\n";
    let exe = debug_build(&d, "probe", program);
    let mut dap = lldb_dap_launch(&exe, &["QBMAIN"], &[5]);
    let s = dap.event("stopped");
    let thread = s["body"]["threadId"].clone();
    let st = dap.ok("stackTrace", json!({"threadId": thread, "levels": 1}));
    assert!(
        st["stackFrames"][0]["name"].as_str().unwrap().starts_with("QBMAIN"),
        "{st}"
    );
    assert_eq!(st["stackFrames"][0]["source"]["name"], "qbx.cpp", "{st}");
    dap.ok("continue", json!({"threadId": thread}));
    let s = dap.event("stopped");
    assert_eq!(dap_top(&mut dap, &s).1, 5);
    let st = dap.ok("stackTrace", json!({"threadId": thread, "levels": 1}));
    let frame = st["stackFrames"][0]["id"].clone();
    let watch = |dap: &mut support::dap::Dap, e: &str| {
        dap.ok(
            "evaluate",
            json!({"expression": e, "frameId": frame, "context": "watch"}),
        )["result"]
            .as_str()
            .unwrap()
            .to_string()
    };
    let addr: u64 = watch(&mut dap, "(unsigned long long)__LONG_TOTAL").parse().unwrap();
    let r = dap.ok(
        "readMemory",
        json!({"memoryReference": format!("0x{addr:x}"), "count": 4}),
    );
    assert_eq!(r["data"], "BwAAAA==", "7 as a LONG: {r}");
    dap.ok(
        "writeMemory",
        json!({"memoryReference": format!("0x{addr:x}"), "data": "KgAAAA=="}),
    );
    assert_eq!(watch(&mut dap, "*__LONG_TOTAL"), "42");
    let sc = dap.ok("scopes", json!({"frameId": frame}));
    let locals = sc["scopes"][0]["variablesReference"].clone();
    let v = dap.ok("variables", json!({"variablesReference": locals}));
    let name = v["variables"][0]["name"].clone();
    let i = dap.ok(
        "dataBreakpointInfo",
        json!({"variablesReference": locals, "name": name}),
    );
    let id = i["dataId"].as_str().unwrap();
    let (hex, size) = id.split_once('/').unwrap();
    assert!(
        u64::from_str_radix(hex, 16).is_ok() && size.parse::<u64>().is_ok(),
        "{id}"
    );
    let set = "((void*(*)(void*,void*))&qbs_set)((void*)__STRING_S, \
               ((void*(*)(const char*,int))&qbs_new_txt_len)(\"xyz\", 3))";
    watch(&mut dap, set);
    assert_eq!(watch(&mut dap, "__STRING_S->len"), "3");
    let j = dap.ok(
        "evaluate",
        json!({"expression": "`thread jump --line 4", "frameId": frame, "context": "repl"}),
    );
    assert!(!j["result"].as_str().unwrap().contains("error:"), "{j}");
    assert_eq!(dap_top(&mut dap, &s).1, 4);
    dap.ok("disconnect", json!({"terminateDebuggee": true}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// Measured (task 3.1), the clone's `lldb-dap` alone: from the `QBMAIN` breakpoint, `next` reaches the main file
/// through `qbx.cpp`, `maindata.txt`, `mainerr.txt` and `main0.txt` (22 steps on 2026-10-25); a `dataId` of the
/// proxy's form stops with reason `data breakpoint` at the line that writes the value; a `pause` stops the process
/// on a thread of its own (reason `exception`), while the program's thread is at the loop's line; `lldb-dap`'s
/// `threads` need not list the program's thread at all; the program's output comes as `output` events of category
/// `stdout`. Windows only, as `lldb_dap_measured_memory`: Ubuntu's `lldb-dap` 19 and 20 name a stop at a data
/// breakpoint `breakpoint` (`m7-linux` design, "Measured"), which the adapter names `data breakpoint`.
#[test]
#[cfg(windows)]
#[ignore = "needs the QB64pe reference clone"]
fn lldb_dap_measured_stops() {
    use serde_json::json;
    let d = scratch("dap-measure2");
    let program = "$CONSOLE:ONLY\nDIM SHARED total AS LONG\nPRINT \"start\"\ntotal = 1\ntotal = total + 1\nx = 0\n\
                   DO: x = x + 1: LOOP\n";
    let exe = debug_build(&d, "probe2", program);
    let mut dap = lldb_dap_launch(&exe, &["QBMAIN"], &[]);
    let s = dap.event("stopped");
    let thread = s["body"]["threadId"].clone();
    let mut files = Vec::new();
    for _ in 0..200 {
        let st = dap.ok("stackTrace", json!({"threadId": thread, "levels": 1}));
        let file = st["stackFrames"][0]["source"]["name"]
            .as_str()
            .unwrap_or("")
            .to_string();
        if !files.contains(&file) {
            files.push(file.clone());
        }
        if file == "probe2.bas" {
            break;
        }
        dap.ok("next", json!({"threadId": thread}));
        dap.event("stopped");
    }
    eprintln!("lldb-dap entry files: {files:?}");
    assert_eq!(files.first().map(String::as_str), Some("qbx.cpp"));
    assert_eq!(files.last().map(String::as_str), Some("probe2.bas"));
    let st = dap.ok("stackTrace", json!({"threadId": thread, "levels": 1}));
    let frame = st["stackFrames"][0]["id"].clone();
    let r = dap.ok(
        "evaluate",
        json!({"expression": "(unsigned long long)__LONG_TOTAL", "frameId": frame, "context": "watch"}),
    );
    let addr: u64 = r["result"].as_str().unwrap().parse().unwrap();
    dap.ok(
        "setDataBreakpoints",
        json!({"breakpoints": [{"dataId": format!("{addr:X}/4"), "accessType": "write"}]}),
    );
    dap.ok("continue", json!({"threadId": thread}));
    let s = dap.event("stopped");
    assert_eq!(s["body"]["reason"], "data breakpoint", "{s}");
    assert_eq!(dap_top(&mut dap, &s).1, 4, "the line that writes it");
    dap.ok("setDataBreakpoints", json!({"breakpoints": []}));
    dap.ok("continue", json!({"threadId": thread}));
    std::thread::sleep(Duration::from_secs(1));
    dap.ok("pause", json!({"threadId": thread}));
    let s = dap.event("stopped");
    assert_eq!(s["body"]["reason"], "exception", "{s}");
    assert_ne!(s["body"]["threadId"], thread, "a thread of its own: {s}");
    let top = dap_top(&mut dap, &json!({"body": {"threadId": thread}}));
    assert!(top.0.starts_with("QBMAIN") && top.1 == 7, "{top:?}");
    let out = dap.drain_events("output");
    assert!(
        out.iter()
            .any(|o| o["body"]["category"] == "stdout" && o["body"]["output"].as_str().unwrap().contains("start")),
        "{out:?}"
    );
    dap.ok("disconnect", json!({"terminateDebuggee": true}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// The variables a reference lists: name to (value, type, reference).
fn dap_vars(dap: &mut support::dap::Dap, reference: &serde_json::Value) -> Vec<(String, String, String, i64)> {
    let v = dap.ok("variables", serde_json::json!({"variablesReference": reference}));
    v["variables"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| {
            (
                x["name"].as_str().unwrap_or("").to_string(),
                x["value"].as_str().unwrap_or("").to_string(),
                x["type"].as_str().unwrap_or("").to_string(),
                x["variablesReference"].as_i64().unwrap_or(0),
            )
        })
        .collect()
}

/// The frames `stackTrace` gives the program's thread: name and line.
fn dap_frames(dap: &mut support::dap::Dap, stopped: &serde_json::Value) -> Vec<(String, u64, i64)> {
    let st = dap.ok(
        "stackTrace",
        serde_json::json!({"threadId": stopped["body"]["threadId"]}),
    );
    st["stackFrames"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["name"].as_str().unwrap().to_string(),
                f["line"].as_u64().unwrap(),
                f["id"].as_i64().unwrap(),
            )
        })
        .collect()
}

/// The next `stopped` event, its reason and the top frame's name and line.
fn dap_stop(dap: &mut support::dap::Dap) -> (serde_json::Value, String, String, u64) {
    let s = dap.event("stopped");
    let frames = dap_frames(dap, &s);
    let (name, line) = frames.first().map_or((String::new(), 0), |f| (f.0.clone(), f.1));
    let reason = s["body"]["reason"].as_str().unwrap_or("").to_string();
    (s, reason, name, line)
}

/// What the program printed (the `stdout` output events so far).
fn dap_output(dap: &mut support::dap::Dap) -> String {
    dap.drain_events("output")
        .iter()
        .filter(|o| o["body"]["category"] == "stdout")
        .map(|o| o["body"]["output"].as_str().unwrap_or("").to_string())
        .collect()
}

/// The program of the spec's scenario "Each kind" (spec `editor/debug-adapter`, "Variables"), with a SUB called
/// from another SUB.
const DAP_KINDS: &str = "$CONSOLE:ONLY\nTYPE pt\n    x AS INTEGER\n    nm AS STRING * 4\nEND TYPE\n\
DIM SHARED total AS LONG\nDIM s AS STRING\nDIM a(1 TO 3) AS SINGLE\nDIM p AS pt\n\
total = 7: s = \"hi\": a(2) = 1.5: p.x = 3: p.nm = \"abcd\"\nCALL outer(5)\nPRINT total\nSYSTEM\n\
SUB outer (k AS LONG)\n    DIM y AS LONG\n    y = k * 2\n    CALL show(y)\nEND SUB\n\
SUB show (n AS LONG)\n    STATIC calls AS INTEGER\n    DIM x AS SINGLE\n    x = n + 0.5\n    calls = calls + 1\n\
    total = total + n\nEND SUB\n";

/// "Stop on entry": the first statement is on line 3, after a metacommand and a comment.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_stop_on_entry() {
    use serde_json::json;
    let d = scratch("dap-entry");
    let exe = debug_build(&d, "entry", "$CONSOLE:ONLY\n' first\nx = 1\nPRINT x\nSYSTEM\n");
    let mut dap = dap_launch(&exe, &[], &[], json!({"stopOnEntry": true}));
    let (_, reason, name, line) = dap_stop(&mut dap);
    assert_eq!((reason.as_str(), name.as_str(), line), ("entry", "Main module", 3));
    let t = dap.ok("threads", json!({}));
    assert_eq!(t["threads"].as_array().unwrap().len(), 1, "{t}");
    assert_eq!(t["threads"][0]["name"], "Program");
    dap.ok("continue", json!({"threadId": t["threads"][0]["id"]}));
    dap.event("exited");
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// "The stack in BASIC terms", "Variables" (each kind, a frame further down), "Evaluate in BASIC terms" (hover,
/// hex), "Set value" (a number, out of range): a breakpoint in a SUB called from another SUB.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_stack_variables_evaluate_set() {
    use serde_json::json;
    let d = scratch("dap-kinds");
    let exe = debug_build(&d, "kinds", DAP_KINDS);
    let mut dap = dap_launch(&exe, &[], &[24], json!({}));
    let (s, reason, _, _) = dap_stop(&mut dap);
    assert_eq!(reason, "breakpoint");
    let frames = dap_frames(&mut dap, &s);
    let names: Vec<(&str, u64)> = frames.iter().map(|f| (f.0.as_str(), f.1)).collect();
    assert_eq!(names, [("SUB show", 24), ("SUB outer", 17), ("Main module", 11)]);
    let scopes = dap.ok("scopes", json!({"frameId": frames[0].2}));
    assert_eq!(scopes["scopes"][0]["name"], "Locals");
    assert_eq!(scopes["scopes"][1]["name"], "Main module");
    let locals = dap_vars(&mut dap, &scopes["scopes"][0]["variablesReference"]);
    let short: Vec<(&str, &str, &str)> = locals
        .iter()
        .map(|v| (v.0.as_str(), v.1.as_str(), v.2.as_str()))
        .collect();
    assert_eq!(
        short,
        [("N", "10", "LONG"), ("CALLS", "1", "INTEGER"), ("X", "10.5", "SINGLE")]
    );
    let main = dap_vars(&mut dap, &scopes["scopes"][1]["variablesReference"]);
    let get = |n: &str| {
        main.iter()
            .find(|v| v.0 == n)
            .unwrap_or_else(|| panic!("{n}: {main:?}"))
            .clone()
    };
    assert_eq!(get("TOTAL").1, "7");
    assert_eq!(get("S").1, "\"hi\"");
    let a = get("A");
    assert_eq!(a.1, "(1 TO 3) AS SINGLE");
    let elements = dap_vars(&mut dap, &json!(a.3));
    let e: Vec<(&str, &str)> = elements.iter().map(|v| (v.0.as_str(), v.1.as_str())).collect();
    assert_eq!(e, [("A(1)", "0"), ("A(2)", "1.5"), ("A(3)", "0")]);
    let members = dap_vars(&mut dap, &json!(get("P").3));
    let m: Vec<(&str, &str)> = members.iter().map(|v| (v.0.as_str(), v.1.as_str())).collect();
    assert_eq!(m, [("X", "3"), ("NM", "\"abcd\"")]);
    // The caller's own locals, read in its frame.
    let outer = dap.ok("scopes", json!({"frameId": frames[1].2}));
    let o = dap_vars(&mut dap, &outer["scopes"][0]["variablesReference"]);
    let o: Vec<(&str, &str)> = o.iter().map(|v| (v.0.as_str(), v.1.as_str())).collect();
    assert_eq!(o, [("K", "5"), ("Y", "10")]);
    // Hover from the SUB: the main module's TOTAL; an element; a member; hex; a name of neither.
    let ev = |dap: &mut support::dap::Dap, e: &str| {
        dap.request(
            "evaluate",
            json!({"expression": e, "frameId": frames[0].2, "context": "hover"}),
        )
    };
    assert_eq!(ev(&mut dap, "total")["body"]["result"], "7");
    assert_eq!(ev(&mut dap, "a(2)")["body"]["result"], "1.5");
    assert_eq!(ev(&mut dap, "p.nm")["body"]["result"], "\"abcd\"");
    assert_eq!(ev(&mut dap, "n,h")["body"]["result"], "&HA");
    let none = ev(&mut dap, "nosuch");
    assert_eq!(none["success"], false);
    assert!(
        none["message"].as_str().unwrap().contains("not a variable here"),
        "{none}"
    );
    // Set value: a LONG, and an INTEGER out of range.
    let set = dap.request(
        "setExpression",
        json!({"expression": "total", "value": "42", "frameId": frames[0].2}),
    );
    assert_eq!(set["body"]["value"], "42", "{set}");
    let bad = dap.request(
        "setExpression",
        json!({"expression": "calls", "value": "40000", "frameId": frames[0].2}),
    );
    assert_eq!(bad["success"], false, "{bad}");
    assert_eq!(ev(&mut dap, "calls")["body"]["result"], "1");
    // A STRING through libqb's own assignment.
    let set = dap.request(
        "setExpression",
        json!({"expression": "s", "value": "\"x\" + CHR$(0)", "frameId": frames[0].2}),
    );
    assert_eq!(set["body"]["value"], "\"x\" + CHR$(0)", "{set}");
    dap.ok("continue", json!({"threadId": s["body"]["threadId"]}));
    dap.event("exited");
    let out = dap_output(&mut dap);
    assert!(out.contains(" 52 "), "42 + 10 printed: {out:?}");
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// "Stepping by BASIC line": into a SUB, out of it, into a FUNCTION of an included file, and `next` past a
/// handler's `RESUME`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_stepping() {
    use serde_json::json;
    let d = scratch("dap-steps");
    std::fs::write(
        d.join("lib.bi"),
        "FUNCTION twice& (k AS LONG)\n    twice& = 2 * k\nEND FUNCTION\n",
    )
    .unwrap();
    let program = "$CONSOLE:ONLY\nx = 1\nCALL show(x)\ny& = twice&(3)\nPRINT y&\nSYSTEM\nSUB show (n)\n    n = n + 1\n\
                   PRINT n\nEND SUB\n'$INCLUDE:'lib.bi'\n";
    let exe = debug_build(&d, "steps", program);
    let mut dap = dap_launch(&exe, &[], &[3], json!({}));
    let (s, _, _, line) = dap_stop(&mut dap);
    assert_eq!(line, 3);
    let thread = s["body"]["threadId"].clone();
    let step = |dap: &mut support::dap::Dap, how: &str| {
        dap.ok(how, json!({"threadId": thread}));
        let (s, reason, name, line) = dap_stop(dap);
        let st = dap.ok("stackTrace", json!({"threadId": s["body"]["threadId"], "levels": 1}));
        let file = st["stackFrames"][0]["source"]["name"]
            .as_str()
            .unwrap_or("")
            .to_string();
        (reason, name, file, line)
    };
    assert_eq!(
        step(&mut dap, "stepIn"),
        ("step".into(), "SUB show".into(), "steps.bas".into(), 8)
    );
    assert_eq!(
        step(&mut dap, "stepOut"),
        ("step".into(), "Main module".into(), "steps.bas".into(), 4)
    );
    assert_eq!(
        step(&mut dap, "stepIn"),
        ("step".into(), "FUNCTION twice&".into(), "lib.bi".into(), 2)
    );
    assert_eq!(step(&mut dap, "next").3, 3, "END FUNCTION");
    assert_eq!(
        step(&mut dap, "next"),
        ("step".into(), "Main module".into(), "steps.bas".into(), 5)
    );
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));

    // `stepIn` on `END SUB` goes past the rest of the CALL to the next line.
    let mut dap = dap_launch(&exe, &[], &[10], json!({}));
    let (s, _, name, line) = dap_stop(&mut dap);
    assert_eq!((name.as_str(), line), ("SUB show", 10));
    dap.ok("stepIn", json!({"threadId": s["body"]["threadId"]}));
    let (_, reason, name, line) = dap_stop(&mut dap);
    assert_eq!((reason.as_str(), name.as_str(), line), ("step", "Main module", 4));
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));

    let program = "$CONSOLE:ONLY\nON ERROR GOTO h\nn = -1\ns$ = CHR$(n)\nPRINT s$\nSYSTEM\nh:\nn = 65\nRESUME\n";
    let exe = debug_build(&d, "retry", program);
    let mut dap = dap_launch(&exe, &[], &[9], json!({}));
    let (s, _, _, line) = dap_stop(&mut dap);
    assert_eq!(line, 9);
    let thread = s["body"]["threadId"].clone();
    for _ in 0..2 {
        dap.ok("next", json!({"threadId": thread}));
        let s = dap.event("stopped");
        let st = dap.ok("stackTrace", json!({"threadId": s["body"]["threadId"], "levels": 1}));
        let f = &st["stackFrames"][0];
        assert_eq!(f["source"]["name"], "retry.bas", "{st}");
        assert!(f["line"].as_u64().unwrap() <= 9, "not past the file's end: {st}");
    }
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// "Stepping by BASIC line": `stepIn` on lines that call no procedure goes one line on each time, also when `lldb`
/// steps into a function of libqb's inline headers and out of it again, back on the same line.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_step_in_on_lines_without_a_call() {
    use serde_json::json;
    let d = scratch("dap-plain-steps");
    let program = "$CONSOLE:ONLY\nx = 1\ny = 2\nz = x + y\ns$ = STR$(z)\nPRINT s$\nSYSTEM\n";
    let exe = debug_build(&d, "plain", program);
    let mut dap = dap_launch(&exe, &[], &[2], json!({}));
    let (s, _, _, line) = dap_stop(&mut dap);
    assert_eq!(line, 2);
    let thread = s["body"]["threadId"].clone();
    for want in 3..=6 {
        dap.ok("stepIn", json!({"threadId": thread}));
        let (_, reason, name, line) = dap_stop(&mut dap);
        assert_eq!((reason.as_str(), name.as_str(), line), ("step", "Main module", want));
    }
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// "Data breakpoints" and "Set next line".
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_data_breakpoint_and_goto() {
    use serde_json::json;
    let d = scratch("dap-data");
    let program = "$CONSOLE:ONLY\nDIM SHARED total AS LONG\ntotal = 1\nx = 2\nx = 3\ntotal = total + x\nPRINT total\n\
                   SYSTEM\n";
    let exe = debug_build(&d, "data", program);
    let mut dap = dap_launch(&exe, &[], &[4], json!({}));
    let (s, _, _, _) = dap_stop(&mut dap);
    let thread = s["body"]["threadId"].clone();
    let frames = dap_frames(&mut dap, &s);
    let info = dap.ok("dataBreakpointInfo", json!({"name": "total", "frameId": frames[0].2}));
    assert!(info["dataId"].as_str().unwrap().ends_with("/4"), "{info}");
    let big = dap.ok("dataBreakpointInfo", json!({"name": "x", "frameId": frames[0].2}));
    assert!(!big["dataId"].is_null(), "a SINGLE: {big}");
    dap.ok(
        "setDataBreakpoints",
        json!({"breakpoints": [{"dataId": info["dataId"], "accessType": "write"}]}),
    );
    dap.ok("continue", json!({"threadId": thread}));
    let (_, reason, _, line) = dap_stop(&mut dap);
    assert_eq!(
        (reason.as_str(), line),
        ("data breakpoint", 6),
        "stopped at the line that writes it"
    );
    dap.ok("setDataBreakpoints", json!({"breakpoints": []}));
    // Back to line 4 from line 6.
    let source = json!({"path": exe.with_extension("bas")});
    let t = dap.ok("gotoTargets", json!({"source": source, "line": 4}));
    let target = t["targets"][0]["id"].clone();
    assert!(!target.is_null(), "{t}");
    let none = dap.ok("gotoTargets", json!({"source": source, "line": 99}));
    assert_eq!(none["targets"], json!([]));
    dap.ok("goto", json!({"threadId": thread, "targetId": target}));
    let (_, reason, _, line) = dap_stop(&mut dap);
    assert_eq!((reason.as_str(), line), ("goto", 4));
    dap.ok("next", json!({"threadId": thread}));
    let (_, _, _, line) = dap_stop(&mut dap);
    assert_eq!(line, 5, "line 4 ran");
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// "Stop on a runtime error": with the filter, error 5 on line 3 stops there; without it, a handler runs on.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_error_stop() {
    use serde_json::json;
    let d = scratch("dap-error");
    let program =
        "$CONSOLE:ONLY\nON ERROR GOTO h\nx$ = CHR$(-1)\nPRINT \"after\"\nSYSTEM\nh:\nPRINT \"handled\"\nRESUME NEXT\n";
    let exe = debug_build(&d, "err", program);
    let launch = |filters: serde_json::Value| {
        let mut cmd = compiler();
        cmd.current_dir(&d).arg("dap");
        let mut dap = support::dap::Dap::start(cmd);
        dap.ok("initialize", json!({"adapterID": "qb64rust", "linesStartAt1": true}));
        let l = dap.send("launch", json!({"program": exe, "noPrompt": true}));
        dap.event("initialized");
        dap.ok("setExceptionBreakpoints", json!({"filters": filters}));
        dap.ok("configurationDone", json!({}));
        assert_eq!(dap.response(l)["success"], true);
        dap
    };
    let mut dap = launch(json!(["qbError"]));
    let (s, reason, name, line) = dap_stop(&mut dap);
    assert_eq!((reason.as_str(), name.as_str(), line), ("exception", "Main module", 3));
    assert!(s["body"]["description"].as_str().unwrap().contains("Error 5"), "{s}");
    let info = dap.ok("exceptionInfo", json!({"threadId": s["body"]["threadId"]}));
    assert_eq!(info["description"], "Error 5: Illegal function call");
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
    let mut dap = launch(json!([]));
    dap.event("exited");
    let out = dap_output(&mut dap);
    assert!(out.contains("handled") && out.contains("after"), "{out:?}");
    assert!(dap.drain_events("stopped").is_empty());
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// "One thread, the program's", scenario "Pause"; and `disconnect` ends a program that would run for ever.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_pause_and_disconnect() {
    use serde_json::json;
    let d = scratch("dap-pause");
    let exe = debug_build(&d, "spin", "$CONSOLE:ONLY\nx = 0\nDO: x = x + 1: LOOP\n");
    let mut dap = dap_launch(&exe, &[], &[], json!({}));
    std::thread::sleep(Duration::from_secs(1));
    let t = dap.ok("threads", json!({}));
    let thread = t["threads"][0]["id"].clone();
    dap.ok("pause", json!({"threadId": thread}));
    let (s, reason, name, line) = dap_stop(&mut dap);
    assert_eq!((reason.as_str(), name.as_str(), line), ("pause", "Main module", 3));
    let t = dap.ok("threads", json!({}));
    assert_eq!(t["threads"], json!([{"id": s["body"]["threadId"], "name": "Program"}]));
    dap.ok("disconnect", json!({"terminateDebuggee": true}));
    assert_eq!(dap.finish().code(), Some(0));
    std::thread::sleep(Duration::from_millis(500));
    let alive = if cfg!(windows) {
        let o = Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq spin.exe", "/NH"])
            .output()
            .unwrap();
        stdout(&o).contains("spin.exe")
    } else {
        // By its path, which is this test's own.
        let o = Command::new("pgrep").arg("-f").arg(&exe).output().unwrap();
        !stdout(&o).trim().is_empty()
    };
    assert!(!alive, "the program ended with the session");
}

/// Measured for change `m5-debug-launch` (its design D6, "Measured"): `console` `integratedTerminal` through the
/// proxy. `lldb-dap` asks the client to start a command (`runInTerminal`); the test starts it as a terminal would,
/// with its stdin a pipe, types a number there for the program's `INPUT`, and the program stops at a breakpoint
/// after it with `X` holding the number.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_run_in_terminal() {
    use serde_json::json;
    use std::io::Write as _;
    let d = scratch("dap-terminal");
    let exe = debug_build(&d, "ask", "$CONSOLE:ONLY\nINPUT x\ny = x * 2\nPRINT y\nSYSTEM\n");
    let mut cmd = compiler();
    cmd.current_dir(&d).arg("dap");
    let mut dap = support::dap::Dap::start(cmd);
    dap.ok(
        "initialize",
        json!({"adapterID": "qb64rust", "linesStartAt1": true, "columnsStartAt1": true,
               "supportsRunInTerminalRequest": true}),
    );
    let launch = dap.send(
        "launch",
        json!({"program": exe, "cwd": exe.parent().unwrap(), "noPrompt": true, "console": "integratedTerminal"}),
    );
    // `lldb-dap` asks for the terminal while it handles `launch`, before `initialized`.
    let r = dap.reverse_request("runInTerminal");
    eprintln!("runInTerminal: {r}");
    let a = &r["arguments"];
    let argv: Vec<String> = a["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect();
    let mut term = Command::new(&argv[0]);
    term.args(&argv[1..])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    if let Some(cwd) = a["cwd"].as_str() {
        term.current_dir(cwd);
    }
    if let Some(env) = a["env"].as_object() {
        for (k, v) in env {
            match v.as_str() {
                Some(v) => term.env(k, v),
                None => term.env_remove(k),
            };
        }
    }
    let mut term = term.spawn().expect("the terminal's command starts");
    dap.reply(&r, json!({"processId": term.id()}));
    dap.event("initialized");
    dap.ok(
        "setBreakpoints",
        json!({"source": {"path": exe.with_extension("bas")}, "breakpoints": [{"line": 3}]}),
    );
    dap.ok("configurationDone", json!({}));
    let l = dap.response(launch);
    assert_eq!(l["success"], true, "launch: {l}");
    term.stdin.as_mut().unwrap().write_all(b"7\r\n").unwrap();
    let (s, reason, name, line) = dap_stop(&mut dap);
    assert_eq!((reason.as_str(), name.as_str(), line), ("breakpoint", "Main module", 3));
    let frames = dap_frames(&mut dap, &s);
    let x = dap.ok(
        "evaluate",
        json!({"expression": "x", "frameId": frames[0].2, "context": "hover"}),
    );
    assert_eq!(x["result"], "7", "{x}");
    dap.ok("disconnect", json!({"terminateDebuggee": true}));
    assert_eq!(dap.finish().code(), Some(0));
    // The launcher may have ended with the program already.
    let _ = term.kill();
    let _ = term.wait();
}

/// Measured for change `m5-debug-launch` (its design D3): `launch`'s `args` reach the program as they are, through
/// the proxy and `lldb-dap`, which joins them into a Windows command line: a word, one with a space (spec
/// `editor/debug-launch`, "Run arguments") and one ending in a backslash.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_args() {
    use serde_json::json;
    let d = scratch("dap-args");
    let program = "$CONSOLE:ONLY\nFOR i = 1 TO _COMMANDCOUNT: PRINT \"[\"; COMMAND$(i); \"]\": NEXT\nSYSTEM\n";
    let exe = debug_build(&d, "args", program);
    let mut dap = dap_launch(&exe, &[], &[], json!({"args": ["a", "b c", "d\\"]}));
    dap.event("exited");
    let out = dap_output(&mut dap).replace("\r\n", "\n");
    assert_eq!(out, "[a]\n[b c]\n[d\\]\n");
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// "Launch", scenario "Not a debug build".
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dap_refuses_a_build_without_debug() {
    use serde_json::json;
    let d = scratch("dap-plain");
    std::fs::write(d.join("plain.bas"), SLICE_PROGRAM).unwrap();
    let o = qb64rust_build(&d, &["-q", "-x", "plain.bas", "-o", "plain.exe"]);
    assert!(o.status.success(), "{}", stdout(&o));
    let mut cmd = compiler();
    cmd.current_dir(&d).arg("dap");
    let mut dap = support::dap::Dap::start(cmd);
    dap.ok("initialize", json!({"adapterID": "qb64rust"}));
    let r = dap.request("launch", json!({"program": d.join("plain.exe")}));
    assert_eq!(r["success"], false);
    let message = r["message"].as_str().unwrap();
    assert!(
        message.contains("plain.exe.debug.json") && message.contains("--debug"),
        "{message}"
    );
    dap.ok("disconnect", json!({}));
    assert_eq!(dap.finish().code(), Some(0));
}

/// Scenario "Not a file name": `lsp.bas` is a program.
#[test]
fn lsp_named_program() {
    let d = scratch("lsp-file");
    std::fs::write(d.join("lsp.bas"), SLICE_PROGRAM).unwrap();
    let o = qb64rust(&d, &["lsp.bas", "--dump", "typed"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).contains("INSTR"), "{}", stdout(&o));
}

/// "Demand report", scenario "A folder of programs": every `.bas` file below the folder is counted, a program the
/// old compiler rejects is left out, and a program with more errors than a compile run reports is counted in full.
/// `VARSEG` of an element of a `_BIT` array stands for any statement not compiled yet (it was `STOP` until
/// `m4-qb64pe-sources`, `RUN` until `m4-statements-metacommands`, `FILES` until `m4-builtins-remaining`, `FOR` over
/// an element of a member array until `m4-left-marked`); when it is, take another.
#[test]
fn report_unsupported_counts_a_folder() {
    let d = scratch("report");
    std::fs::create_dir(d.join("sub")).unwrap();
    std::fs::write(d.join("clean.bas"), SLICE_PROGRAM).unwrap();
    let uses = qb64rust_base::MAX_ERRORS + 5;
    let many = format!("{MARKED_PREAMBLE}{}SYSTEM\n", MARKED.repeat(uses));
    std::fs::write(d.join("sub").join("MANY.BAS"), many).unwrap();
    std::fs::write(d.join("one.bas"), format!("{MARKED_PREAMBLE}{MARKED}SYSTEM\n")).unwrap();
    std::fs::write(d.join("rejected.bas"), format!("{MARKED_PREAMBLE}{MARKED}x =\n")).unwrap();
    std::fs::write(d.join("rejected.err"), "Expected expression\n").unwrap();
    let o = qb64rust(&d, &["--report", "unsupported", "."]);
    assert!(o.status.success(), "{}", stdout(&o));
    let expected = format!(
        "3 programs: 1 without errors, 2 with only \"not supported yet\" errors, 0 with other errors\n\
         1 left out: an .err file beside them (the old compiler rejects them)\n\
         programs  alone   uses  not supported yet\n       \
         2      2 {:>6}  `VARSEG` of a `_BIT` array\n",
        uses + 1
    );
    assert_eq!(stdout(&o), expected);
    // A program the inputs reach more than once, under other spellings too, is counted once.
    let o = qb64rust(&d, &["--report", "unsupported", ".", "sub", "./sub", "one.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(stdout(&o), expected);
}

/// A program with a library block is clean in the demand report, which looks for no file; the dumps show its
/// declarations and calls.
#[test]
fn a_library_block_in_the_report_and_the_dumps() {
    let d = scratch("report-library");
    let program = "$CONSOLE:ONLY\nDECLARE LIBRARY\n    FUNCTION isdigit& (BYVAL c AS LONG)\nEND DECLARE\n\
                   PRINT isdigit&(48)\n";
    std::fs::write(d.join("lib.bas"), program).unwrap();
    let o = qb64rust(&d, &["--report", "unsupported", "lib.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).starts_with("1 program: 1 without errors"), "{}", stdout(&o));
    // A block with no name needs no lookup, so no clone either.
    for (stage, call) in [
        ("ir", "CallProc ISDIGIT:I32 [Value(Const 48:I32)]"),
        ("cpp", "((int32)isdigit(48))"),
    ] {
        let o = qb64rust(&d, &["--dump", stage, "lib.bas"]);
        assert!(o.status.success(), "{stage}: {}", stdout(&o));
        assert!(stdout(&o).contains(call), "{stage}: {}", stdout(&o));
    }
    let o = qb64rust(&d, &["--dump", "typed", "lib.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(
        stdout(&o).contains("external isdigit of LIBRARY 0 (byval)"),
        "{}",
        stdout(&o)
    );
}

/// "Demand report", scenarios "A program with another error" and "Unknown report".
#[test]
fn report_unsupported_names_programs_with_other_errors() {
    let d = scratch("report-other");
    std::fs::write(d.join("bad.bas"), "$CONSOLE:ONLY\nx =\n").unwrap();
    let o = qb64rust(&d, &["--report", "unsupported", "bad.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(
        stdout(&o),
        "1 program: 0 without errors, 0 with only \"not supported yet\" errors, 1 with other errors\n\
         with other errors:\n       1  bad.bas\n"
    );
    let o = qb64rust(&d, &["--report", "typed", "bad.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(stdout(&o).contains("unknown --report `typed`"), "{}", stdout(&o));
    // Without --report there is one input file, as before.
    let o = qb64rust(&d, &["bad.bas", "bad.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(stdout(&o).contains("more than one input file"), "{}", stdout(&o));
}

#[expect(clippy::disallowed_methods, reason = "a tool's messages")]
fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).to_string()
}

/// A stand-in toolchain folder: the two programs a lookup asks for, empty (spec `compiler/toolchain`, "Toolchain
/// lookup"). Nothing can be built with it.
fn fake_toolchain(dir: &Path) {
    std::fs::create_dir_all(dir.join("bin")).unwrap();
    // Windows' names and Linux's, so that the folder is a toolchain on both.
    for f in ["c++.exe", "mingw32-make.exe", "g++", "make"] {
        std::fs::write(dir.join("bin").join(f), "").unwrap();
    }
}

/// The libraries of `verification\v37_lib` built into `dir` with the toolchain a build uses, as `build_libs.py`
/// builds them: `v37c.a` (C), `v37cpp.a` (C++), `v37o.o` (C), `v37dll.dll`.
#[cfg(windows)]
fn build_v37_libraries(dir: &Path) {
    let bin = support::toolchain("");
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../verification/v37_lib");
    let out = |name: &str| dir.join(name).to_string_lossy().into_owned();
    let run = |tool: &str, args: &[&str]| {
        let o = support::run(
            Command::new(bin.join(format!("{tool}.exe")))
                .current_dir(&src)
                .args(args),
            None,
            COMPILER_LIMIT,
            "toolchain",
        );
        assert!(o.status.success(), "{tool} {args:?}: {}", text(&o.stderr));
    };
    run("gcc", &["-O2", "-c", "v37c.c", "-o", &out("v37c.o")]);
    run("ar", &["rcs", &out("v37c.a"), &out("v37c.o")]);
    run("gcc", &["-O2", "-c", "v37o.c", "-o", &out("v37o.o")]);
    run("g++", &["-O2", "-c", "v37cpp.cpp", "-o", &out("v37cpp.o")]);
    run("ar", &["rcs", &out("v37cpp.a"), &out("v37cpp.o")]);
    run("gcc", &["-O2", "-shared", "v37dll.c", "-o", &out("v37dll.dll")]);
}

/// `language/external-libraries`, "A library beside the program", "A C++ library without a header", and
/// `compiler/pipeline`, "A library on the link command": a C `.a`, a C++ `.a` (its linkage from `nm`), a `.o`, a
/// `CUSTOMTYPE` function of the C++ library and a DLL, each found by `./` beside the program.
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[cfg(windows)]
fn library_files_are_linked_and_called() {
    let d = scratch("library-files");
    build_v37_libraries(&d);
    let program = "$CONSOLE:ONLY\n\
                   DECLARE LIBRARY \"./v37c\"\n\
                   FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nSUB c_inc (v AS LONG)\nFUNCTION c_hello$\n\
                   END DECLARE\n\
                   DECLARE LIBRARY \"./v37cpp\"\nFUNCTION cpp_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nEND DECLARE\n\
                   DECLARE STATIC LIBRARY \"./v37o\"\nSUB o_tick\nFUNCTION o_ticks&\nEND DECLARE\n\
                   DECLARE CUSTOMTYPE LIBRARY \"./v37cpp\"\nFUNCTION ct_half# ALIAS cpp_half (BYVAL v AS DOUBLE)\n\
                   END DECLARE\n\
                   DECLARE DYNAMIC LIBRARY \"./v37dll\"\nFUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)\n\
                   FUNCTION d_hello$\nEND DECLARE\n\
                   DIM x AS LONG\nx = 41: c_inc x\no_tick: o_tick\n\
                   PRINT c_add&(1, 2); x; c_hello$; cpp_add&(20, 22); o_ticks&; ct_half#(5); d_add&(2, 3); d_hello$\nSYSTEM\n";
    std::fs::write(d.join("lib.bas"), program).unwrap();
    let exe = d.join("lib.exe");
    let o = qb64rust_build(&d, &["-q", "lib.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let run = run(unattended(&exe).current_dir(&d), None, PROGRAM_LIMIT, "program");
    assert_eq!(text(&run.stdout), " 3  42 c-hello 42  2  2.5  5 d-hello\r\n");
}

/// `language/external-libraries`, "A name the DLL does not export": error 260 before the first statement, which
/// `ON ERROR` does not catch (measured, `verification\v37_j`). libqb writes the error to stderr and, unless
/// `QB64PE_NOPROMPT` is set, then opens a dialog that waits for a person (`gui_alert`); `support::unattended` sets
/// it.
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[cfg(windows)]
fn a_function_the_dll_does_not_export_is_error_260() {
    let d = scratch("library-260");
    build_v37_libraries(&d);
    let program = "$CONSOLE:ONLY\nON ERROR GOTO handler\nDECLARE DYNAMIC LIBRARY \"./v37dll\"\n\
                   FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nFUNCTION d_none& (BYVAL a AS LONG)\n\
                   END DECLARE\nPRINT \"start\"\nSYSTEM\nhandler:\nPRINT \"handled\"; ERR\nRESUME NEXT\n";
    std::fs::write(d.join("dll.bas"), program).unwrap();
    let exe = d.join("dll.exe");
    let o = qb64rust_build(&d, &["-q", "dll.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let run = run(unattended(&exe).current_dir(&d), None, PROGRAM_LIMIT, "program");
    assert_eq!(text(&run.stdout), "");
    assert!(
        text(&run.stderr).contains("Runtime error: Sub/Function does not exist in dynamic library"),
        "{}",
        text(&run.stderr)
    );
}

/// The lookup's and `nm`'s errors at the block, before the C++ (`language/external-libraries`, "Rejected library
/// names"; `DECISIONS.md` 2026-10-16: the old compiler's way where it rejects a program, a clear error where its
/// C++ fails): a library that is not found (`v37_x18`), a name the library does not hold (`x13`), a name of the
/// first of two libraries, looked for in the last only (`x24`), a header without a guard that defines and that two
/// blocks name (`x16`; one with a guard and one that only declares are accepted, `x68`), two `DYNAMIC` blocks of
/// one DLL (`x67`) and of two DLLs whose names give one handle, a block without a header whose last name finds no
/// file (`x69`), and a library file whose path holds a `$`. With `-z`, which runs `nm` too.
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[cfg(windows)]
fn library_errors_before_the_cpp() {
    let d = scratch("library-errors");
    build_v37_libraries(&d);
    std::fs::write(d.join("g.h"), "int g_one(void) { return 1; }\n").unwrap();
    std::fs::write(d.join("p.h"), "#pragma once\nint p_one(void) { return 1; }\n").unwrap();
    std::fs::write(d.join("q.h"), "int q_one(void);\nint q_two(void);\n").unwrap();
    for copy in ["v37.dll.dll", "v37_dll.dll"] {
        std::fs::copy(d.join("v37dll.dll"), d.join(copy)).unwrap();
    }
    let cases = [
        (
            "DECLARE LIBRARY \"v37c\"\nFUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nEND DECLARE\n",
            Some("2:17: error: LIBRARY not found"),
        ),
        (
            "DECLARE LIBRARY \"./v37c\"\nFUNCTION c_none& (BYVAL v AS LONG)\nEND DECLARE\n",
            Some("2:1: error: Could not find sub/function 'c_none' in './v37c.a'"),
        ),
        (
            "DECLARE LIBRARY \"./v37c\", \"./v37cpp\"\nFUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)\n\
             END DECLARE\n",
            Some("2:1: error: Could not find sub/function 'c_add' in './v37cpp.a'"),
        ),
        (
            "DECLARE LIBRARY \"./g\"\nFUNCTION g_one& ()\nEND DECLARE\n\
             DECLARE LIBRARY \"./g\"\nFUNCTION g_two& ()\nEND DECLARE\n",
            Some(
                "5:17: error: the header of `./g` is named by an earlier `DECLARE LIBRARY` too, defines what it \
                 declares and has no include guard",
            ),
        ),
        (
            "DECLARE LIBRARY \"./p\"\nFUNCTION p_one& ()\nEND DECLARE\n\
             DECLARE LIBRARY \"./p\"\nFUNCTION p_two& ALIAS p_one ()\nEND DECLARE\n",
            None,
        ),
        (
            "DECLARE LIBRARY \"./q\"\nFUNCTION q_one& ()\nEND DECLARE\n\
             DECLARE LIBRARY \"./q\"\nFUNCTION q_two& ()\nEND DECLARE\n",
            None,
        ),
        (
            "DECLARE DYNAMIC LIBRARY \"kernel32\"\nFUNCTION lstrlenA& (s AS STRING)\nEND DECLARE\n\
             DECLARE DYNAMIC LIBRARY \"kernel32\"\nFUNCTION GetTickCount&\nEND DECLARE\n",
            Some(
                "5:1: error: an earlier `DECLARE DYNAMIC LIBRARY` block has the DLL handle `DLL_kernel32` too: the \
                 old compiler declares it once per block and its C++ fails; declare the functions in one block",
            ),
        ),
        (
            "DECLARE DYNAMIC LIBRARY \"./v37dll\"\nFUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nEND DECLARE\n\
             DECLARE DYNAMIC LIBRARY \"v37dll\"\nFUNCTION d_hello$\nEND DECLARE\n",
            Some("5:1: error: an earlier `DECLARE DYNAMIC LIBRARY` block has the DLL handle `DLL_v37dll` too"),
        ),
        (
            "DECLARE DYNAMIC LIBRARY \"./v37.dll\"\nFUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nEND DECLARE\n\
             DECLARE DYNAMIC LIBRARY \"./v37_dll\"\nFUNCTION d_hello$\nEND DECLARE\n",
            Some(
                "5:1: error: an earlier `DECLARE DYNAMIC LIBRARY` block has the DLL handle `DLL_v37_dll` too: the \
                 old compiler declares it once per block and its C++ fails; `./v37.dll` and `./v37_dll` are two \
                 DLLs whose names give one handle; rename one of them",
            ),
        ),
        (
            "DECLARE LIBRARY \"./v37c\", \"sub/\"\nFUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nEND DECLARE\n",
            Some("2:1: error: the last name of the block, `sub/`, finds no library file"),
        ),
    ];
    for (i, (blocks, error)) in cases.iter().enumerate() {
        let name = format!("e{i}.bas");
        std::fs::write(d.join(&name), format!("$CONSOLE:ONLY\n{blocks}PRINT 1\n")).unwrap();
        let o = qb64rust(&d, &["-z", &name]);
        match error {
            Some(e) => {
                assert_eq!(o.status.code(), Some(1), "{name}: {}", stdout(&o));
                assert!(stdout(&o).contains(&format!("{name}:{e}")), "{name}: {}", stdout(&o));
            }
            None => assert!(o.status.success(), "{name}: {}", stdout(&o)),
        }
    }
    // A library beside a program in a folder whose name holds a `$`.
    let dollar = d.join("lib$dir");
    std::fs::create_dir_all(&dollar).unwrap();
    std::fs::copy(d.join("v37c.a"), dollar.join("v37c.a")).unwrap();
    std::fs::write(
        dollar.join("e.bas"),
        "DECLARE LIBRARY \"./v37c\"\nFUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)\nEND DECLARE\nPRINT 1\n",
    )
    .unwrap();
    let o = qb64rust(&dollar, &["-z", "e.bas"]);
    assert_eq!(o.status.code(), Some(1), "{}", stdout(&o));
    assert!(
        stdout(&o).contains("e.bas:1:17: error: the path of the library file of `./v37c` holds a `$`"),
        "{}",
        stdout(&o)
    );
}

/// `support::try_run` past its deadline (`study\archive\31` item 1): the child is ended in time with the process it
/// started, and what it printed is kept. The child, `cmd`, starts a second `cmd` that writes a tick to a file every
/// second; once the tree is killed, the file stops growing.
#[cfg(windows)]
#[test]
fn a_child_past_its_deadline_is_killed_with_its_tree() {
    use std::os::windows::process::CommandExt as _;
    let d = scratch("deadline");
    let ticks = d.join("ticks.txt");
    let mut cmd = Command::new("cmd");
    cmd.current_dir(&d).raw_arg(
        r#"/D /C cmd /D /C "for /L %i in (1,1,60) do @(echo tick%i & echo x>>ticks.txt & ping -n 2 127.0.0.1 >nul)""#,
    );
    let start = std::time::Instant::now();
    let Err(timeout) = support::try_run(&mut cmd, None, Duration::from_secs(3)) else {
        panic!("the child ended by itself");
    };
    assert!(start.elapsed() < Duration::from_secs(15), "{:?}", start.elapsed());
    assert_eq!(timeout.reason, "did not end in time; its process tree was killed");
    assert!(timeout.stdout.starts_with(b"tick1"), "{}", timeout);
    let written = std::fs::metadata(&ticks).unwrap().len();
    assert!(written > 0);
    std::thread::sleep(Duration::from_millis(2500));
    assert_eq!(
        std::fs::metadata(&ticks).unwrap().len(),
        written,
        "the grandchild still writes"
    );
}

/// A child that writes more than a pipe holds to both stdout and stderr ends, with both read whole.
#[cfg(windows)]
#[test]
fn a_child_with_much_output_is_read_whole() {
    use std::os::windows::process::CommandExt as _;
    let d = scratch("much-output");
    let text = "0123456789abcdef".repeat(4) + "\r\n";
    let text = text.repeat(20_000).into_bytes();
    std::fs::write(d.join("big.txt"), &text).unwrap();
    let mut cmd = Command::new("cmd");
    cmd.current_dir(&d)
        .raw_arg(r#"/D /C "type big.txt 1>&2 & type big.txt""#);
    let o = run(&mut cmd, None, PROGRAM_LIMIT, "type");
    assert!(o.status.success());
    assert!(o.stdout == text, "stdout: {} of {} bytes", o.stdout.len(), text.len());
    assert!(o.stderr == text, "stderr: {} of {} bytes", o.stderr.len(), text.len());
}

/// QB64pe's six layout defaults, as the format suites give them (spec `compiler/format`).
const LAYOUT_DEFAULTS: [&str; 6] = [
    "-f:autoindent=1",
    "-f:autoindentsize=4",
    "-f:indentsubs=1",
    "-f:autolayout=1",
    "-f:keywordcapitals=0",
    "-f:keywordlowercase=0",
];

/// `-y` without `-o` writes nothing and says why (spec `compiler/format`, "No output file").
#[test]
fn format_needs_an_output_file() {
    let d = scratch("format-no-out");
    std::fs::write(d.join("prog.bas"), "print 1\r\n").unwrap();
    let o = qb64rust(&d, &["-y", "prog.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(
        stdout(&o).contains("formatting requires an output file"),
        "{}",
        stdout(&o)
    );
    let names: Vec<_> = std::fs::read_dir(&*d)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(names, ["prog.bas"], "no file is written");
}

/// `-y -o` writes the file and leaves the input as it was, with CRLF after each line; nothing is built (spec
/// `compiler/format`, "Format command").
#[test]
fn format_writes_the_output_file() {
    let d = scratch("format-out");
    std::fs::write(d.join("prog.bas"), "PRINT 1\nPRINT 2").unwrap();
    let o = qb64rust(&d, &["-y", "-q", "-m", "prog.bas", "-o", "out.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(std::fs::read(d.join("prog.bas")).unwrap(), b"PRINT 1\nPRINT 2");
    let out = std::fs::read(d.join("out.bas")).unwrap();
    assert!(
        out.ends_with(b"\r\n") && out.split(|&c| c == b'\n').count() == 3,
        "{out:?}"
    );
    let mut names: Vec<_> = std::fs::read_dir(&*d)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    names.sort();
    assert_eq!(names, ["out.bas", "prog.bas"], "no executable or build folder");
}

/// The format suite's command line on QB64pe's `format_tests\general` gives its `full-cc.out` (spec `compiler/format`,
/// "Runner's format line"). The test's files are read from the reference clone.
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[cfg(windows)]
fn runners_format_line() {
    let clone = qb64rust_driver::build::clone_root().expect("the QB64pe reference clone");
    let tests = clone.join("tests/format_tests/general");
    let d = scratch("format-general");
    for f in ["general.bas", "included.file"] {
        std::fs::copy(tests.join(f), d.join(f)).unwrap();
    }
    let mut args = vec!["-y", "-m"];
    args.extend(LAYOUT_DEFAULTS);
    args.extend(["general.bas", "-o", "out"]);
    let o = qb64rust(&d, &args);
    assert!(o.status.success(), "{}", stdout(&o));
    let lf = |b: Vec<u8>| b.into_iter().filter(|&c| c != b'\r').collect::<Vec<u8>>();
    let got = lf(std::fs::read(d.join("out")).unwrap());
    let want = lf(std::fs::read(tests.join("full-cc.out")).unwrap());
    assert!(got.trim_ascii_end() == want.trim_ascii_end(), "{}", stdout(&o));
}

/// Both keyword settings true is CamelCase, as `qb64pe` takes them (spec `compiler/format`, "Both keyword
/// settings").
#[test]
fn format_both_keyword_settings() {
    let d = scratch("format-keywords");
    std::fs::write(d.join("prog.bas"), "print 1\r\n").unwrap();
    let o = qb64rust(
        &d,
        &[
            "-y",
            "-f:KeywordCapitals=true",
            "-f:KeywordLowercase=true",
            "prog.bas",
            "-o",
            "out.bas",
        ],
    );
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(std::fs::read(d.join("out.bas")).unwrap(), b"Print 1\r\n");
    let o = qb64rust(&d, &["-y", "-f:KeywordCapitals=true", "prog.bas", "-o", "up.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(std::fs::read(d.join("up.bas")).unwrap(), b"PRINT 1\r\n");
}

/// An indent size out of range is an error (spec `compiler/format`, "Indent size out of range"), and so is a value
/// that is not boolean; the boolean words `qb64pe` takes are taken.
#[test]
fn format_layout_setting_values() {
    let d = scratch("format-settings");
    std::fs::write(d.join("prog.bas"), "print 1\r\n").unwrap();
    let o = qb64rust(&d, &["-y", "-f:AutoIndentSize=65", "prog.bas", "-o", "out.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(stdout(&o).contains("range 1 to 64"), "{}", stdout(&o));
    assert!(!d.join("out.bas").exists());
    let o = qb64rust(&d, &["-y", "-f:AutoLayout=maybe", "prog.bas", "-o", "out.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(stdout(&o).contains("needs `true` or `false`"), "{}", stdout(&o));
    let words = [
        "-f:AUTOINDENT=yes",
        "-f:IndentSubs=off",
        "-f:autolayout=-1",
        "-f:KeywordCapitals=on",
    ];
    let o = qb64rust(
        &d,
        &[
            "-y", words[0], words[1], words[2], words[3], "prog.bas", "-o", "out.bas",
        ],
    );
    assert!(o.status.success(), "{}", stdout(&o));
}

/// No `-f:` setting formats as the six defaults given (spec `compiler/format`, "No settings").
#[test]
fn format_defaults_are_qb64pes() {
    let d = scratch("format-defaults");
    let program = "sub s\r\nif x   =   1 then\r\nprint   \"a\"\r\nend if\r\nend sub\r\n";
    std::fs::write(d.join("prog.bas"), program).unwrap();
    let o = qb64rust(&d, &["-y", "prog.bas", "-o", "plain.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    let mut args = vec!["-y"];
    args.extend(LAYOUT_DEFAULTS);
    args.extend(["prog.bas", "-o", "given.bas"]);
    let o = qb64rust(&d, &args);
    assert!(o.status.success(), "{}", stdout(&o));
    assert_eq!(
        std::fs::read(d.join("plain.bas")).unwrap(),
        std::fs::read(d.join("given.bas")).unwrap()
    );
}

/// A program with an error is reported as a compile reports it, and nothing is written (spec `compiler/format`,
/// "Syntax error").
#[test]
fn format_of_a_program_with_an_error() {
    let d = scratch("format-error");
    std::fs::write(d.join("prog.bas"), "print 1\r\nx = 1 +\r\n").unwrap();
    let o = qb64rust(&d, &["-y", "-q", "prog.bas", "-o", "out.bas"]);
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    assert!(out.contains("prog.bas:2:") && out.contains("1 error"), "{out}");
    assert!(!d.join("out.bas").exists(), "no output file");
}

/// A warning does not stop `-y` (spec `compiler/format`, "Program with an error").
#[test]
fn format_with_a_warning() {
    let d = scratch("format-warning");
    std::fs::write(d.join("prog.bas"), "$DEBUG\r\nx = 1\r\n").unwrap();
    let o = qb64rust(&d, &["-y", "-w", "prog.bas", "-o", "out.bas"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).contains("warning"), "{}", stdout(&o));
    assert!(d.join("out.bas").is_file());
}

/// A layout setting given to a run that is not `-y` is checked and changes nothing (spec `compiler/cli`, "A layout
/// setting in a build"): the C++ is the same as without it.
#[test]
fn layout_setting_in_a_build_changes_nothing() {
    let d = scratch("layout-in-build");
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM).unwrap();
    let plain = qb64rust(&d, &["--dump", "cpp", "prog.bas"]);
    let with = qb64rust(&d, &["-f:AutoIndent=false", "--dump", "cpp", "prog.bas"]);
    assert!(plain.status.success() && with.status.success(), "{}", stdout(&with));
    assert_eq!(plain.stdout, with.stdout);
    let bad = qb64rust(&d, &["-f:AutoIndentSize=0", "-z", "prog.bas"]);
    assert_eq!(bad.status.code(), Some(1), "{}", stdout(&bad));
}

/// The same with a build (spec `compiler/cli`, "A layout setting in a build").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn layout_setting_in_a_build() {
    let d = scratch("layout-build");
    // Ends with `SYSTEM`: an `END` would wait for a key.
    std::fs::write(d.join("prog.bas"), SLICE_PROGRAM.replace("END\n", "SYSTEM\n")).unwrap();
    let exe = d.join("prog.exe");
    let o = qb64rust_build(
        &d,
        &[
            "-f:AutoIndent=false",
            "-q",
            "-x",
            "prog.bas",
            "-o",
            exe.to_str().unwrap(),
        ],
    );
    assert!(o.status.success(), "{}", stdout(&o));
    let ran = run(&mut unattended(&exe), None, PROGRAM_LIMIT, "program");
    assert!(stdout(&ran).starts_with("x= .25"), "{}", stdout(&ran));
}

// The always-on fixes of `study\39` (OpenSpec change `fixes-switch`, design D5), each with the rows of
// `QB64PE-DIVERGENCES.md` its program takes.

/// The handler that every program below sets: it prints `ERR` and resumes next.
const ERR_HANDLER: &str = "SYSTEM\nh:\nPRINT \"error\"; ERR\nRESUME NEXT\n";

/// `DIM` and `REDIM` with a count below zero raise error 5 and allocate nothing (D-040, spec
/// `language/arrays-and-types`, "Fewer than no element"); a `REDIM` releases the array.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn dim_below_zero_raises_error_5() {
    let d = scratch("dim-below-zero");
    let program = format!(
        "$CONSOLE:ONLY\nON ERROR GOTO h\nn = -3\nDIM d(n) AS LONG\nPRINT UBOUND(d)\nREDIM r(4) AS LONG\n\
         REDIM r(2 TO n) AS LONG\nPRINT UBOUND(r)\nPRINT \"end\"\n{ERR_HANDLER}"
    );
    let out = build_and_run_tracing(&d, &program, "D-040\n");
    assert_eq!(out, "error 5 \nerror 9 \nerror 5 \nerror 9 \nend\n");
}

/// A whole array with no storage used whole raises error 9 and reads and writes nothing (D-042, spec
/// `language/arrays-and-types`, "An erased array"): no later `PRINT` shows the text a `READ` would have stored, and a
/// `SWAP` with an allocated array leaves it as it was.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn whole_array_without_storage_raises_error_9() {
    let d = scratch("whole-erased");
    let program = format!(
        "$CONSOLE:ONLY\nON ERROR GOTO h\nDATA \"xyz\"\nREDIM a(2) AS STRING, b(2) AS STRING\nb(0) = \"kept\"\n\
         ERASE a\nPRINT a()\nREAD a()\nSWAP a(), b()\nPRINT b(0)\nPRINT \"after\"\n{ERR_HANDLER}"
    );
    let out = build_and_run_tracing(&d, &program, "D-042\n");
    assert_eq!(out, "error 9 \nerror 9 \nerror 9 \nkept\nafter\n");
}

/// `RESUME NEXT` that leaves a SUB leaves it through its exit, so the `_MEM` block of a local is no longer valid
/// (D-045, spec `language/error-handling`, "A block of a local after RESUME").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn resume_leaves_a_sub_through_its_exit() {
    let d = scratch("resume-exit");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO eh\nDIM SHARED gm AS _MEM\nERROR 5\nback:\nPRINT _MEMEXISTS(gm)\nSYSTEM\n\
                   eh:\ns\nGOTO back\nSUB s\nDIM x AS LONG\ngm = _MEM(x)\nRESUME NEXT\nEND SUB\n";
    assert_eq!(build_and_run_tracing(&d, program, "D-045\n"), " 0 \n");
}

/// `RUN` from a SUB leaves the active procedures first, so a program runs it 100,000 times and ends normally (D-046,
/// spec `language/control-flow`, "RUN from a SUB many times"; the old compiler's runs out of stack after about 2,400);
/// in a `$CONSOLE:ONLY` program the output after `RUN label` stays on the console ("Output after RUN label").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn run_from_a_sub_unwinds() {
    let d = scratch("run-unwind");
    let program = "$CONSOLE:ONLY\nc = VAL(ENVIRON$(\"QBR_RUNS\")) + 1\nENVIRON \"QBR_RUNS=\" + LTRIM$(STR$(c))\n\
                   IF c < 100000 THEN s\nPRINT \"done\"; c\nSYSTEM\nSUB s\nDIM t AS STRING\nt = SPACE$(100)\nRUN\nEND SUB\n";
    assert_eq!(build_and_run_tracing(&d, program, "D-046\n"), "done 100000 \n");
    let d = scratch("run-console");
    let program = "$CONSOLE:ONLY\nIF ENVIRON$(\"QBR_RAN\") = \"\" THEN PRINT \"one\"\nagain:\n\
                   IF ENVIRON$(\"QBR_RAN\") = \"\" THEN ENVIRON \"QBR_RAN=1\": RUN again\nPRINT \"two\"\nSYSTEM\n";
    assert_eq!(build_and_run_tracing(&d, program, "D-046\n"), "one\ntwo\n");
}

/// With its standard input from the null device, `END` prints its line and ends at once, and a `LINE INPUT` with no
/// handler ends the program with error 62 rather than waiting (D-048, spec `language/console-and-window`, "END with
/// its input from the null device", "LINE INPUT with no handler"; the old compiler's programs wait for ever). The
/// limit of 20 s is far below the runner's 60 s, so a wait fails the test.
#[test]
#[ignore = "needs the QB64pe reference clone"]
#[expect(clippy::disallowed_methods, reason = "the runtime's own message")]
fn end_and_input_at_the_end_of_the_input() {
    let limit = std::time::Duration::from_secs(20);
    let d = scratch("end-null-input");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT \"before END\"\nEND\n").unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let ran = run(&mut unattended(&exe), None, limit, "program");
    assert_eq!(ran.status.code(), Some(0));
    // libqb's own words on each platform, as QB64pe's builds print them.
    let prompt = if cfg!(windows) {
        "Press any key to continue"
    } else {
        "Press enter to continue"
    };
    assert_eq!(stdout(&ran).replace("\r\n", "\n"), format!("before END\n\n{prompt}"));
    let d = scratch("line-input-null");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\nLINE INPUT s$\nPRINT \"not reached\"\nSYSTEM\n",
    )
    .unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let ran = run(&mut unattended(&exe), None, limit, "program");
    let (out, err) = (stdout(&ran), String::from_utf8_lossy(&ran.stderr).to_string());
    assert!(
        err.contains("Input past end of file") && !out.contains("not reached"),
        "{out}{err}"
    );
}

/// `INPUT$` reads a redirected input in text mode, as `INPUT` and `LINE INPUT` read it: a CR LF pair comes as one LF,
/// a CR alone stays, and a byte 26 (Ctrl+Z) ends the input, so the next read raises error 62 (D-048). The old
/// compiler's `INPUT$` waits for ever on such an input, so no recording can hold this; `verification\v63_a6` records
/// how its `LINE INPUT` and `INPUT` read the same kind of input. On Linux standard input has no text mode: the bytes
/// come as they are, a byte 26 among them, and only the end of the input raises error 62 (task 5.5 of `m7-linux`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn input_string_reads_redirected_input_in_text_mode() {
    let d = scratch("input-string-crlf");
    std::fs::write(
        d.join("p.bas"),
        "$CONSOLE:ONLY\nON ERROR GOTO h\ns$ = INPUT$(6)\nFOR i = 1 TO LEN(s$): PRINT ASC(s$, i);: NEXT\nPRINT\n\
         s$ = INPUT$(1)\nPRINT \"not reached\"\nSYSTEM\nh:\nPRINT \"error\"; ERR\nSYSTEM\n",
    )
    .unwrap();
    let exe = d.join(default_exe("p"));
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let (input, expected): (&[u8], _) = if cfg!(windows) {
        (b"ab\r\ne\r\r\n\x1azz\r\n", " 97  98  10  101  13  10 \nerror 62 \n")
    } else {
        (b"ab\r\ne\x1a", " 97  98  13  10  101  26 \nerror 62 \n")
    };
    let ran = run(&mut unattended(&exe), Some(input), PROGRAM_LIMIT, "program");
    assert_eq!(stdout(&ran).replace("\r\n", "\n"), expected);
}

/// `_CAST` of a float constant out of range gives what the same conversion of a variable gives, with and without
/// constant folding (D-047, spec `language/builtin-functions`, "A constant out of range", "Without folding").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn cast_of_a_float_constant_is_a_variables() {
    let program = "$CONSOLE:ONLY\nx# = 3E9: PRINT _CAST(LONG, 3E9); _CAST(LONG, x#)\n\
                   y# = 1E20: PRINT _CAST(_BYTE, 1000.7); _CAST(_UNSIGNED _INTEGER64, 1E20); _CAST(_UNSIGNED _INTEGER64, y#)\n\
                   SYSTEM\n";
    let expected = "-2147483648 -2147483648 \n-24  9223372036854775808  9223372036854775808 \n";
    let d = scratch("cast-const");
    assert_eq!(build_and_run(&d, program), expected);
    let d = scratch("cast-const-nofold");
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = run(
        compiler().current_dir(&d).env("QB64RUST_NO_FOLD", "1").args([
            "-q",
            "-x",
            "p.bas",
            "-o",
            exe.to_str().unwrap(),
        ]),
        None,
        COMPILER_LIMIT,
        "compiler",
    );
    assert!(o.status.success(), "{}", stdout(&o));
    let ran = run(&mut unattended(&exe), None, PROGRAM_LIMIT, "program");
    assert_eq!(stdout(&ran).replace("\r\n", "\n"), expected);
}

/// `ERASE` and `CLEAR` of a dynamic array make its `_MEM` blocks invalid, as `REDIM` does (D-043, spec
/// `language/mem-blocks`, "Block of an erased array"); `ERASE` of a static array leaves them valid.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn erase_makes_blocks_invalid() {
    let d = scratch("erase-blocks");
    let program = "$CONSOLE:ONLY\nDIM SHARED m AS _MEM, k AS _MEM\nREDIM d(5) AS LONG\nm = _MEM(d())\nERASE d\n\
                   PRINT _MEMEXISTS(m)\nDIM s(5) AS LONG\nk = _MEM(s())\nERASE s\nPRINT _MEMEXISTS(k)\n\
                   REDIM e(5) AS LONG\nm = _MEM(e())\nCLEAR\nPRINT _MEMEXISTS(m)\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 0 \n-1 \n 0 \n");
}

/// A `STRING * n` element and member passed to a library's `char*` reach C with a NUL after their bytes and get its
/// writes back, the element's index taken once (D-044, spec `language/external-libraries`, "A member read by strlen",
/// "A write comes back").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn fixed_strings_to_char_pointer() {
    let d = scratch("fixed-char");
    std::fs::write(
        d.join("h.h"),
        "#include <string.h>\nint32_t h_len(const char *s) { return (int32_t)strlen(s); }\n\
         void h_upper(char *s) { for (; *s; s++) if (*s >= 'a' && *s <= 'z') *s -= 32; }\n",
    )
    .unwrap();
    let program = "$CONSOLE:ONLY\nDECLARE LIBRARY \"h\"\n    FUNCTION h_len& (s AS STRING)\n    SUB h_upper (s AS STRING)\n\
                   END DECLARE\nTYPE t\n    a AS STRING * 4\n    b AS STRING * 4\nEND TYPE\nDIM SHARED calls AS LONG\n\
                   DIM v AS t, e(3) AS STRING * 3\nv.a = \"abcd\": v.b = \"efgh\"\ne(1) = \"def\": e(2) = \"ghi\"\n\
                   PRINT h_len&(v.a); h_len&(e(1))\nh_upper e(ix): h_upper v.a\nPRINT v.a; v.b; e(1); e(2); calls\n\
                   SYSTEM\nFUNCTION ix&\ncalls = calls + 1\nix& = 1\nEND FUNCTION\n";
    assert_eq!(build_and_run(&d, program), " 4  3 \nABCDefghDEFghi 1 \n");
}
