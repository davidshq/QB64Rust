//! Spec `compiler/call-trace`, and the call-trace requirements of spec `compiler/cli` (OpenSpec change
//! `call-trace`). The tests that build a program need the QB64pe reference clone and its toolchain, as those of
//! `cli.rs`, and are `#[ignore]`d. Most programs run with `QB64PE_NOPROMPT=continue`, so that one build holds
//! several errors, each reported with its own call stack.

use crate::support::{self, COMPILER_LIMIT, PROGRAM_LIMIT, run, unattended};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn scratch(name: &str) -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix(&format!("qb64rust-ct-{name}-"))
        .tempdir()
        .unwrap()
}

fn compiler(dir: &Path, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_qb64rust"));
    cmd.env(support::OBJECTS_LOCK_ENV, support::objects_lock());
    run(cmd.current_dir(dir).args(args), None, COMPILER_LIMIT, "compiler")
}

#[expect(clippy::disallowed_methods, reason = "the compiler's and the program's own messages")]
fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).replace("\r\n", "\n")
}

/// Writes `files` (path, text) into `d` and builds `p.bas` with `args`; its executable.
fn build(d: &Path, files: &[(&str, &str)], args: &[&str]) -> PathBuf {
    for (path, body) in files {
        let path = d.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }
    let exe = d.join(format!("p{}", std::env::consts::EXE_SUFFIX));
    let mut all = vec!["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()];
    all.extend_from_slice(args);
    let o = compiler(d, &all);
    assert!(o.status.success(), "{}", text(&o.stdout));
    exe
}

/// Runs `exe` in `d` with `QB64PE_NOPROMPT=noprompt`: its stderr and its exit code.
fn errors(d: &Path, exe: &Path, noprompt: &str) -> (String, Option<i32>) {
    let o = run(
        unattended(exe).current_dir(d).env("QB64PE_NOPROMPT", noprompt),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    (text(&o.stderr), o.status.code())
}

/// One report on stderr, as libqb writes it under `QB64PE_NOPROMPT`: its first line, the error's text, then the
/// call stack's rows.
fn report(line: &str, message: &str, rows: &[&str]) -> String {
    let mut out = format!("\nRuntime error: {line}\n{message}\nCall stack:\n");
    for r in rows {
        out.push_str(&format!("  {r}\n"));
    }
    out
}

const NESTED: &str = "$CONSOLE:ONLY\nouter\nx = twice&(1) + half&(2)\ndown 3\nERROR 5\nSYSTEM\n\
                      SUB outer\ninner\nEND SUB\nSUB inner\nERROR 5\nEND SUB\n\
                      FUNCTION twice& (n&)\ntwice& = 2 * n&\nEND FUNCTION\n\
                      FUNCTION half& (n&)\nERROR 6\nhalf& = n& \\ 2\nEND FUNCTION\n\
                      SUB down (n)\nIF n > 0 THEN down n - 1: EXIT SUB\nERROR 7\nEND SUB\n";

/// The four reports of [`NESTED`] built as `p.bas` with the call trace, run under `QB64PE_NOPROMPT=continue`.
fn nested_reports() -> String {
    let at = |n: u32| format!("Line: {n} (in main module)");
    [
        report(
            &at(11),
            "Illegal function call",
            &["SUB inner (p.bas:11)", "SUB outer (p.bas:8)", "Main module (p.bas:2)"],
        ),
        report(
            &at(17),
            "Overflow",
            &["FUNCTION half& (p.bas:17)", "Main module (p.bas:3)"],
        ),
        report(
            &at(22),
            "Out of memory",
            &[
                "SUB down (p.bas:22)",
                "SUB down (p.bas:21)",
                "SUB down (p.bas:21)",
                "SUB down (p.bas:21)",
                "Main module (p.bas:4)",
            ],
        ),
        report(&at(5), "Illegal function call", &["Main module (p.bas:5)"]),
    ]
    .concat()
}

/// "An error in a SUB", "An error in the main module", "A FUNCTION inside an expression", "A recursive procedure":
/// each error's block lists only what is running, so after `inner`, `half&` and `down` return (by their end and by
/// `EXIT SUB`), the next block has none of them. Spec `compiler/cli`, "Call-trace build", "The option".
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn the_call_stack_of_each_error() {
    let d = scratch("nested");
    let exe = build(d.path(), &[("p.bas", NESTED)], &["--call-trace"]);
    let (stderr, code) = errors(d.path(), &exe, "continue");
    assert_eq!(stderr, nested_reports());
    assert_eq!(code, Some(0));
}

/// Spec `compiler/cli`, "Call-trace build", "A wasm build": the same program built for the wasm target and run by
/// Node gives the same reports as the desktop build. Needs the Emscripten SDK, as `cli::wasm_builds_and_runs`;
/// without it the test says so and passes.
#[test]
#[ignore = "builds with the Emscripten SDK (QB64RUST_EMSDK_ROOT) and the C++ toolchain"]
fn the_same_call_stack_on_the_wasm_target() {
    let sdk = match qb64rust_driver::toolchain::find_emsdk(None) {
        Ok(sdk) => sdk,
        Err(why) => {
            eprintln!("skipped: no Emscripten SDK ({why})");
            return;
        }
    };
    let d = scratch("wasm");
    std::fs::write(d.path().join("p.bas"), NESTED).unwrap();
    let o = compiler(
        d.path(),
        &["-q", "-x", "--target", "wasm", "--call-trace", "p.bas", "-o", "p.js"],
    );
    assert!(o.status.success(), "{}", text(&o.stdout));
    let mut node = Command::new(&sdk.node);
    node.current_dir(d.path())
        .arg(d.path().join("p.js"))
        .env("QB64PE_NOPROMPT", "continue");
    let o = run(&mut node, None, PROGRAM_LIMIT, "node");
    assert_eq!(text(&o.stderr), nested_reports());
    assert_eq!(o.status.code(), Some(0));
}

/// Spec `compiler/cli`, "Call-trace build", "The build without the option": the same program prints QB64pe's
/// message, with no block, and ends at its first error.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn no_call_stack_without_the_option() {
    let d = scratch("plain");
    let exe = build(d.path(), &[("p.bas", NESTED)], &[]);
    let (stderr, code) = errors(d.path(), &exe, "y");
    assert_eq!(
        stderr,
        "\nRuntime error: Line: 11 (in main module)\nIllegal function call\n"
    );
    assert_eq!(code, Some(1));
}

/// "Pending GOSUBs": an error under a subroutine, a call from one, and a `GOSUB` inside a procedure.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn pending_gosubs() {
    let d = scratch("gosub");
    let program = "$CONSOLE:ONLY\nGOSUB work\na\nSYSTEM\nwork:\nERROR 5\nfail\nRETURN\n\
                   SUB fail\nERROR 6\nEND SUB\nSUB a\nGOSUB inner\nEXIT SUB\ninner:\nERROR 7\nRETURN\nEND SUB\n";
    let exe = build(d.path(), &[("p.bas", program)], &["--call-trace"]);
    let (stderr, _) = errors(d.path(), &exe, "continue");
    let at = |n: u32| format!("Line: {n} (in main module)");
    let expected = [
        report(
            &at(6),
            "Illegal function call",
            &["Main module (p.bas:6)", "Main module, GOSUB (p.bas:2)"],
        ),
        report(
            &at(10),
            "Overflow",
            &[
                "SUB fail (p.bas:10)",
                "Main module (p.bas:7)",
                "Main module, GOSUB (p.bas:2)",
            ],
        ),
        report(
            &at(16),
            "Out of memory",
            &["SUB a (p.bas:16)", "SUB a, GOSUB (p.bas:13)", "Main module (p.bas:3)"],
        ),
    ]
    .concat();
    assert_eq!(stderr, expected);
}

/// "A procedure of an included file": the rows name each file by its path from the main file's folder.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn procedures_of_included_files() {
    let d = scratch("include");
    let files = [
        (
            "p.bas",
            "$CONSOLE:ONLY\nresolve\nSYSTEM\n'$INCLUDE: 'rules/events.bm'\n'$INCLUDE: 'rules/army.bm'\n",
        ),
        ("rules/events.bm", "SUB resolve\nflag_x\nEND SUB\n"),
        ("rules/army.bm", "SUB flag_x\nERROR 5\nEND SUB\n"),
    ];
    let exe = build(d.path(), &files, &["--call-trace"]);
    let (stderr, code) = errors(d.path(), &exe, "y");
    assert_eq!(
        stderr,
        report(
            "Line: 2 (in army.bm)",
            "Illegal function call",
            &[
                "SUB flag_x (rules/army.bm:2)",
                "SUB resolve (rules/events.bm:2)",
                "Main module (p.bas:2)"
            ],
        )
    );
    assert_eq!(code, Some(1));
}

/// "Code run by an event or an error handler": a SUB and a label run by `ON TIMER`, each shown as started by an
/// event.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn code_run_by_an_event() {
    let d = scratch("event");
    let program = "$CONSOLE:ONLY\nON TIMER(0.05) tick\nTIMER ON\nDO: _LIMIT 200: LOOP UNTIL done\nTIMER OFF\n\
                   ON TIMER(0.05) GOSUB tock\nTIMER ON\nDO: _LIMIT 200: LOOP UNTIL done = 2\nSYSTEM\n\
                   tock:\ndone = 2\nERROR 6\nRETURN\nSUB tick\nSHARED done\ndone = 1\nERROR 5\nEND SUB\n";
    let exe = build(d.path(), &[("p.bas", program)], &["--call-trace"]);
    let (stderr, _) = errors(d.path(), &exe, "continue");
    let at = |n: u32| format!("Line: {n} (in main module)");
    let expected = [
        report(
            &at(17),
            "Illegal function call",
            &["SUB tick (p.bas:17)", "Main module (event)"],
        ),
        report(&at(12), "Overflow", &["Main module (p.bas:12)", "Main module (event)"]),
    ]
    .concat();
    assert_eq!(stderr, expected);
}

/// "An error in an error handler", and "After a handled error": the handler comes first, the procedure it was
/// handling has no line, and `a`, whose error was handled and which has returned, is not listed.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn an_error_in_an_error_handler() {
    let d = scratch("handler");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO trap\na\nPRINT \"after\"\nb\nSYSTEM\n\
                   trap:\nIF handled THEN ERROR 7\nhandled = 1\nRESUME NEXT\n\
                   SUB a\nERROR 5\nEND SUB\nSUB b\nc\nEND SUB\nSUB c\nERROR 6\nEND SUB\n";
    let exe = build(d.path(), &[("p.bas", program)], &["--call-trace"]);
    let (stderr, code) = errors(d.path(), &exe, "y");
    assert_eq!(
        stderr,
        report(
            "Line: 8 (in main module)",
            "Out of memory",
            &[
                "Error handler (p.bas:8)",
                "SUB c",
                "SUB b (p.bas:15)",
                "Main module (p.bas:5)"
            ],
        )
    );
    assert_eq!(code, Some(1));
}

/// "After RUN": `RUN` in a procedure leaves its frame, and the restarted main module's error has one row.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn after_run() {
    let d = scratch("run");
    let program = "$CONSOLE:ONLY\nIF _FILEEXISTS(\"ran.txt\") THEN ERROR 5\n\
                   OPEN \"ran.txt\" FOR OUTPUT AS #1: CLOSE #1\na\nSUB a\nRUN\nEND SUB\n";
    let exe = build(d.path(), &[("p.bas", program)], &["--call-trace"]);
    let (stderr, _) = errors(d.path(), &exe, "y");
    assert_eq!(
        stderr,
        report(
            "Line: 2 (in main module)",
            "Illegal function call",
            &["Main module (p.bas:2)"]
        )
    );
}

/// `RESUME label` in the main module leaves the code that raised the error, so it drops every frame (design D5,
/// "Found while building"): after `a`'s error is handled by `RESUME fresh`, `b`'s block lists neither `a` nor the
/// handler.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn after_resume_to_a_label() {
    let d = scratch("resume");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO trap\na\nSYSTEM\ntrap:\nRESUME fresh\nfresh:\nON ERROR GOTO 0\nb\n\
                   SYSTEM\nSUB a\nERROR 5\nEND SUB\nSUB b\nERROR 6\nEND SUB\n";
    let exe = build(d.path(), &[("p.bas", program)], &["--call-trace"]);
    let (stderr, code) = errors(d.path(), &exe, "y");
    assert_eq!(
        stderr,
        report(
            "Line: 15 (in main module)",
            "Overflow",
            &["SUB b (p.bas:15)", "Main module (p.bas:9)"]
        )
    );
    assert_eq!(code, Some(1));
}

/// "Long call stacks": 1,000 calls deep, the 16 innermost rows, a row of how many were left out, and the 7
/// outermost; and error 256, out of stack space, with its call stack.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn long_call_stacks() {
    let d = scratch("deep");
    let program = "$CONSOLE:ONLY\ndown 999\nforever\nSUB down (n AS LONG)\nIF n THEN down n - 1 ELSE ERROR 5\n\
                   END SUB\nSUB forever\nforever\nEND SUB\n";
    let exe = build(d.path(), &[("p.bas", program)], &["--call-trace"]);
    let (stderr, code) = errors(d.path(), &exe, "continue");
    let mut rows = vec!["SUB down (p.bas:5)"; 16];
    rows.push("... 978 more ...");
    rows.extend(["SUB down (p.bas:5)"; 6]);
    rows.push("Main module (p.bas:2)");
    let deep = report("Line: 5 (in main module)", "Illegal function call", &rows);
    let (first, rest) = stderr.split_at(deep.len().min(stderr.len()));
    assert_eq!(first, deep);
    // Error 256 is critical: it ends the program even under `continue`, and has no line without
    // `$ERRORLOCATION:ON`.
    let rest = rest
        .strip_prefix("\nRuntime error: Out of stack space\n")
        .unwrap_or_else(|| panic!("{rest}"));
    let rest = rest
        .strip_prefix("Enable $ErrorLocation:ON for source location details.\nCall stack:\n")
        .unwrap_or_else(|| panic!("{rest}"));
    let lines: Vec<&str> = rest.lines().collect();
    assert_eq!(lines.len(), 24, "{rest}");
    assert_eq!(lines[0], "  SUB forever");
    assert!(lines[1..16].iter().all(|l| *l == "  SUB forever (p.bas:8)"), "{rest}");
    assert!(
        lines[16].starts_with("  ... ") && lines[16].ends_with(" more ..."),
        "{rest}"
    );
    assert!(lines[17..23].iter().all(|l| *l == "  SUB forever (p.bas:8)"), "{rest}");
    assert_eq!(lines[23], "  Main module (p.bas:3)");
    assert_eq!(code, Some(1));
}

/// "The program is otherwise the same", "A SUB _GL beside the program": `SUB _GL` and the procedure it calls run
/// on libqb's display thread while the main program raises its error, and the block lists the main program's frames
/// only. Run 20 times, since the two threads meet at no fixed point. A window program: run it in the suites VM.
/// `$CONSOLE` gives it a console too, so that its stderr is the test's pipe.
#[test]
#[ignore = "needs the QB64pe reference clone; opens a window"]
fn sub_gl_beside_the_program() {
    let d = scratch("gl");
    let program = "$CONSOLE\nDIM SHARED frames AS LONG\nSCREEN _NEWIMAGE(64, 64, 32)\n\
                   DO: _LIMIT 100: LOOP UNTIL frames > 5\nouter\nSYSTEM\n\
                   SUB _GL\nframes = frames + 1\nhelper\nEND SUB\nSUB helper\nx = 1\nEND SUB\n\
                   SUB outer\nERROR 5\nEND SUB\n";
    let exe = build(d.path(), &[("p.bas", program)], &["--call-trace"]);
    let expected = report(
        "Line: 15 (in main module)",
        "Illegal function call",
        &["SUB outer (p.bas:15)", "Main module (p.bas:5)"],
    );
    for i in 1..=20 {
        let (stderr, code) = errors(d.path(), &exe, "y");
        assert_eq!(stderr, expected, "run {i}");
        assert_eq!(code, Some(1), "run {i}");
    }
}

/// Spec `compiler/cli`, "Call-trace build", "With the other builds": the coverage and value-trace records of a
/// build with `--call-trace` are those of the build without it.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn the_records_of_the_other_builds() {
    let d = scratch("records");
    let program = "$CONSOLE:ONLY\nn = 1\nbump n\nPRINT n\nSYSTEM\nSUB bump (k)\nk = k + 1\nEND SUB\n";
    let records = |args: &[&str]| {
        let mut all = vec!["--coverage", "--trace-values"];
        all.extend_from_slice(args);
        let exe = build(d.path(), &[("p.bas", program)], &all);
        let (coverage, trace) = (d.path().join("c.txt"), d.path().join("t.txt"));
        let _ = std::fs::remove_file(&coverage);
        let _ = std::fs::remove_file(&trace);
        let o = run(
            unattended(&exe)
                .current_dir(d.path())
                .env("QB64RUST_COVERAGE", &coverage)
                .env("QB64RUST_TRACE", &trace),
            None,
            PROGRAM_LIMIT,
            "program",
        );
        assert_eq!(text(&o.stdout), " 2 \n");
        (std::fs::read(coverage).unwrap(), std::fs::read(trace).unwrap())
    };
    assert_eq!(records(&["--call-trace"]), records(&[]));
}

/// Task 1.3: the build cache's key of a program holds its fragments, so a build with the call trace has a key of
/// its own (`build::program_inputs_key`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_key_of_its_own_in_the_build_cache() {
    let d = scratch("key");
    std::fs::write(d.path().join("p.bas"), NESTED).unwrap();
    let key = |args: &[&str]| {
        let mut all = vec!["-q", "--print-build-inputs", "p.bas", "-o", "p.exe"];
        all.extend_from_slice(args);
        let o = compiler(d.path(), &all);
        assert!(o.status.success(), "{}", text(&o.stdout));
        text(&o.stdout)
    };
    assert_ne!(key(&["--call-trace"]), key(&[]));
}

/// Whether the C++ that `--dump cpp` writes for `p.bas` in `d`, with `args`, has the call trace.
fn traced(d: &Path, args: &[&str]) -> bool {
    let mut all = vec!["--dump", "cpp", "p.bas"];
    all.extend_from_slice(args);
    let o = compiler(d, &all);
    assert!(o.status.success(), "{args:?}: {}", text(&o.stdout));
    text(&o.stdout).contains("qbr_ct_push_to(qbr_ct_s,")
}

/// Spec `compiler/cli`, "When a build has the call trace": the settings file, the command line over it, and a debug
/// build. No build: the C++ is enough.
#[test]
fn when_a_build_has_the_call_trace() {
    let d = scratch("when");
    std::fs::write(d.path().join("p.bas"), NESTED).unwrap();
    assert!(!traced(d.path(), &[]), "no option, no settings file");
    assert!(traced(d.path(), &["--call-trace"]));
    assert!(traced(d.path(), &["--debug"]), "a debug build");
    assert!(!traced(d.path(), &["--debug", "--no-call-trace"]));
    assert!(
        !traced(d.path(), &["--call-trace", "--no-call-trace"]),
        "the last one given"
    );
    let toml = d.path().join("qb64rust.toml");
    std::fs::write(&toml, "[build]\ncall_trace = true\n").unwrap();
    assert!(traced(d.path(), &[]), "the settings file");
    assert!(
        !traced(d.path(), &["--no-call-trace"]),
        "the command line over the settings file"
    );
    assert!(!traced(d.path(), &["--no-settings"]));
    std::fs::write(&toml, "[build]\ncall_trace = false\n").unwrap();
    assert!(!traced(d.path(), &["--debug"]), "the settings file over --debug");
    std::fs::write(&toml, "[build]\ncall_trace = \"yes\"\n").unwrap();
    let o = compiler(d.path(), &["--dump", "cpp", "p.bas"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(
        text(&o.stdout).contains("the value of `call_trace` must be `true` or `false`"),
        "{}",
        text(&o.stdout)
    );
}

/// `--print-paths` takes neither option: it would be ignored without a word.
#[test]
fn print_paths_takes_no_call_trace_option() {
    let d = scratch("paths");
    std::fs::write(d.path().join("p.bas"), NESTED).unwrap();
    for option in ["--call-trace", "--no-call-trace"] {
        let o = compiler(d.path(), &["--print-paths", "p.bas", option]);
        assert_eq!(o.status.code(), Some(1), "{option}");
        assert!(text(&o.stdout).contains("--call-trace"), "{}", text(&o.stdout));
    }
}
