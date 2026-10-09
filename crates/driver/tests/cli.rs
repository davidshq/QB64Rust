//! One test per scenario of the `compiler/cli` spec. Tests that build an executable need the QB64pe reference
//! clone and its toolchain; they are `#[ignore]`d and run by hand: `cargo test -p qb64rust-driver --test cli --
//! --ignored`.

use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nREDIM c(2)\nPRINT 1 +\nCLS\n").unwrap();
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

/// `RETURN label` with no `GOSUB` pending raises error 3 and leaves the `GOSUB` stack intact, so a later `GOSUB`
/// and `RETURN` work (`DIVERGENCES.md` D-003; the old compiler's program crashes at that `GOSUB`, so this cannot be
/// a corpus program recorded with `qb64pe.exe`). The error is serviced at the label `RETURN` jumps to.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn return_label_with_nothing_pending() {
    let d = scratch("return-label");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO handler\nRETURN back\nPRINT \"not printed\"\nback:\n\
                   PRINT \"at back\"\nGOSUB sub1\nPRINT \"after sub1\"\nGOSUB sub1\nPRINT \"after sub1 again\"\nSYSTEM\n\
                   sub1:\nPRINT \"in sub1\"\nRETURN\nhandler:\nPRINT \"error\"; ERR\nRESUME NEXT\n";
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let run = Command::new(&exe)
        .current_dir(&d)
        .env("QB64PE_NOPROMPT", "y")
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert_eq!(run.status.code(), Some(0), "{}", stdout(&run));
    assert_eq!(
        stdout(&run).replace("\r\n", "\n"),
        "error 3 \nat back\nin sub1\nafter sub1\nin sub1\nafter sub1 again\n"
    );
}

/// A store into a member of an element whose index is out of range, or raises with no error pending before it,
/// stores nothing (`DIVERGENCES.md` D-004; the old compiler writes element 0, so this cannot be a corpus program
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
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let run = Command::new(&exe)
        .current_dir(&d)
        .env("QB64PE_NOPROMPT", "y")
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert_eq!(run.status.code(), Some(0), "{}", stdout(&run));
    assert_eq!(
        stdout(&run).replace("\r\n", "\n"),
        "error 9 \nafter a(9): 70 \nerror 9 \nafter a(-1): 70 \nerror 5 \nafter both: 70 \nerror 5 \n\
         after raising index: 70 \nerror 5 \na(2).m: 0 \n"
    );
}

/// A `_BIT * n` scalar wider than 32 bits has 8 bytes of its own, so a store into it leaves the `_BIT` scalar
/// declared before it alone (`DIVERGENCES.md` D-009; the old compiler reserves 4 bytes and overwrites that
/// neighbour, printing ` 4294967295 ` and ` 0 ` here, so this cannot be a corpus program recorded with `qb64pe.exe`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn wide_bit_scalars_do_not_overlap() {
    let d = scratch("wide-bit");
    let program = "$CONSOLE:ONLY\nDIM a AS _BIT * 33, b AS _BIT * 33\na = 5: b = -1\nPRINT a; b\n\
                   DIM u AS _UNSIGNED _BIT * 3, w AS _BIT * 40\nu = 2: w = 2\nPRINT u; w\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 5 -1 \n 2  2 \n");
}

/// Builds `program` as `p.bas` in `d` with `-q -x`, runs it and gives its output with `\n` line ends.
fn build_and_run(d: &Path, program: &str) -> String {
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust(d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let run = Command::new(&exe)
        .current_dir(d)
        .env("QB64PE_NOPROMPT", "y")
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert_eq!(run.status.code(), Some(0), "{}", stdout(&run));
    stdout(&run).replace("\r\n", "\n")
}

/// `a IMP b IMP c` is `(a IMP b) IMP c` (`DIVERGENCES.md` D-005: the old compiler computes `5 OR 3 OR 0`, 7, so this
/// cannot be a recorded corpus program).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn imp_chain_is_left_to_right() {
    let d = scratch("imp-chain");
    let program = "$CONSOLE:ONLY\nPRINT 5 IMP 3 IMP 0\na = 5: b = 3: c = 0\nPRINT a IMP b IMP c\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 4 \n 4 \n");
}

/// The smallest LONG or `_INTEGER64` divided by -1 raises error 6, and `MOD -1` gives 0 (`DIVERGENCES.md` D-006: the
/// old program crashes).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn smallest_integer_divided_by_minus_one() {
    let d = scratch("min-div");
    let program = "$CONSOLE:ONLY\nON ERROR GOTO h\nl& = -2147483648: m& = -1\nPRINT l& \\ m&\nPRINT l& MOD m&\n\
                   q&& = -9223372036854775808: r&& = -1\nPRINT q&& \\ r&&\nPRINT q&& MOD r&&\n\
                   PRINT 7 \\ m&; -7 MOD m&; l& \\ 2\nSYSTEM\nh:\nPRINT \"[handler\"; ERR; \"]\"\nRESUME NEXT\n";
    assert_eq!(
        build_and_run(&d, program),
        "[handler 6 ]\n 0 \n[handler 6 ]\n 0 \n-7  0 -1073741824 \n"
    );
}

/// An integer power beyond `_INTEGER64` in a `CONST` is a DOUBLE (`DIVERGENCES.md` D-007: the old evaluator wraps it
/// to -9223372036854775808).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn const_power_beyond_integer64() {
    let d = scratch("const-pow");
    let program = "$CONSOLE:ONLY\nCONST c = 2 ^ 70\nPRINT c\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 1.180591620717411D+21 \n");
}

/// A label and a `CONST` on one line (`DIVERGENCES.md` D-008: the old compiler fails to compile it).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn label_then_const() {
    let d = scratch("label-const");
    let program = "$CONSOLE:ONLY\nlbl1: CONST k = 4\nPRINT k: GOTO done\nPRINT \"skipped\"\ndone:\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 4 \n");
}

/// Each call of a FUNCTION `f$n` gives its own string (`DIVERGENCES.md` D-014: the old compiler returns bytes the
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
    let o = qb64rust(&d, &["-w", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    let out = stdout(&o);
    assert!(
        out.lines().any(|l| l.starts_with("p.bas:2:") && l.contains("warning:")),
        "{out}"
    );
    assert!(!out.contains("error"), "{out}");
    assert!(exe.is_file());
    let run = Command::new(&exe)
        .current_dir(&d)
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert_eq!(stdout(&run).replace("\r\n", "\n"), " 512 \n");

    std::fs::remove_file(&exe).unwrap();
    let o = qb64rust(&d, &["-q", "-m", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
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
        ("negations", format!("x = {}1", "- ".repeat(20_000)), false),
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

/// Scenario "Subcommand" (change `m2-language-server`): the language server over stdin and stdout, ending with
/// exit code 0 after `shutdown` and `exit`.
#[test]
fn language_server() {
    use std::io::{Read as _, Write as _};
    use std::process::Stdio;
    let d = scratch("lsp");
    let mut child = Command::new(env!("CARGO_BIN_EXE_qb64rust"))
        .current_dir(&d)
        .arg("lsp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let messages = [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":null,"capabilities":{}}}"#,
        r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"shutdown"}"#,
        r#"{"jsonrpc":"2.0","method":"exit"}"#,
    ];
    let mut stdin = child.stdin.take().unwrap();
    for m in messages {
        write!(stdin, "Content-Length: {}\r\n\r\n{m}", m.len()).unwrap();
    }
    drop(stdin);
    let mut out = Vec::new();
    child.stdout.take().unwrap().read_to_end(&mut out).unwrap();
    let status = child.wait().unwrap();
    #[expect(clippy::disallowed_methods, reason = "protocol messages, not BASIC source")]
    let out = String::from_utf8_lossy(&out).to_string();
    assert_eq!(status.code(), Some(0), "{out}");
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

/// Scenario "Not a file name": `lsp.bas` is a program.
#[test]
fn lsp_named_program() {
    let d = scratch("lsp-file");
    std::fs::write(d.join("lsp.bas"), SLICE_PROGRAM).unwrap();
    let o = qb64rust(&d, &["lsp.bas", "--dump", "typed"]);
    assert!(o.status.success(), "{}", stdout(&o));
    assert!(stdout(&o).contains("INSTR"), "{}", stdout(&o));
}
