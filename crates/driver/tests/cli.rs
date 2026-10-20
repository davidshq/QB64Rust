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
    let clone = qb64rust_driver::build::find_root(None).expect("the QB64pe reference clone");
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
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT 1\nFILES\n").unwrap();
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
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nSTOP\nPRINT 1 +\nFILES\n").unwrap();
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
    assert!(!d.join("p.exe").exists());
}

/// A build without the copy of libqb fails naming its option and variable; `-z` needs no copy (spec `compiler/cli`,
/// scenario "No copy of libqb found"). The clone is a stand-in folder: this test builds nothing.
#[test]
fn no_libqb_copy_found() {
    let d = scratch("nolibqb");
    std::fs::write(d.join("p.bas"), SLICE_PROGRAM).unwrap();
    std::fs::create_dir_all(d.join("clone/internal/c")).unwrap();
    std::fs::write(d.join("clone/internal/c/qbx.cpp"), "").unwrap();
    std::fs::create_dir_all(d.join("empty")).unwrap();
    let compile = |extra: &[&str]| {
        let mut cmd = compiler();
        cmd.current_dir(&d)
            .args(["-q", "--qb64pe-root", "clone", "--libqb-root", "empty"])
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
    assert_eq!(
        build_and_run_tracing(&d, program, "D-003\n"),
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
    assert_eq!(
        build_and_run_tracing(&d, program, "D-004\n"),
        "error 9 \nafter a(9): 70 \nerror 9 \nafter a(-1): 70 \nerror 5 \nafter both: 70 \nerror 5 \n\
         after raising index: 70 \nerror 5 \na(2).m: 0 \n"
    );
}

/// The same for an element of a member array (`DIVERGENCES.md` D-004, design D8 of `m2-defxxx-common-arrays`): a
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

/// `-f:ExtraLinkerFlags=` puts a source file on the link line: the screen-state file of `tools/screen`, which writes
/// the state of the screen when the program exits (scenario "An extra source file on the link line").
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn extra_linker_flags_link_the_screen_state_file() {
    let d = scratch("extra-link");
    std::fs::write(d.join("p.bas"), "$CONSOLE:ONLY\nPRINT \"hi\"\nSYSTEM\n").unwrap();
    let exe = d.join("p.exe");
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/screen/screen_state.cpp");
    let flag = format!("-f:ExtraLinkerFlags={}", file.to_str().unwrap().replace('\\', "/"));
    let o = qb64rust_build(&d, &[&flag, "-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let state = d.join("state.txt");
    let run = run(
        unattended(&exe).current_dir(&d).env("QB64_SCREEN_STATE", &state),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(run.status.code(), Some(0), "{}", stdout(&run));
    let text = std::fs::read(&state).unwrap();
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
    std::fs::write(
        d.join("p.bas"),
        "SCREEN 13\nLINE (0, 0)-(9, 9), 4, BF\nDO: k$ = INKEY$: LOOP WHILE k$ = \"\"\nSYSTEM\n",
    )
    .unwrap();
    std::fs::write(d.join("p.keys"), "# nothing is announced\nprompt\ntext x\n").unwrap();
    let exe = d.join("p.exe");
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/screen/screen_state.cpp");
    let flag = format!("-f:ExtraLinkerFlags={}", file.to_str().unwrap().replace('\\', "/"));
    let o = qb64rust_build(&d, &[&flag, "-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let state = d.join("state.txt");
    let run = run(
        unattended(&exe)
            .current_dir(&d)
            .env("QB64_SCREEN_STATE", &state)
            .env("QB64_SCREEN_KEYS", d.join("p.keys"))
            // Long enough that the box is drawn on a loaded machine (500 ms failed once, `study\archive\30` §8).
            .env("QB64_SCREEN_PROMPT_MS", "5000"),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(run.status.code(), Some(124));
    let text = std::fs::read(&state).unwrap();
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
    std::fs::write(d.join("p.bas"), format!("SCREEN 13\n{}{}SYSTEM\n", wait(7), wait(9))).unwrap();
    std::fs::write(d.join("p.keys"), "rule 7 key space\nplay until 14:M\ntext e\n").unwrap();
    let exe = d.join("p.exe");
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/screen/screen_state.cpp");
    let flag = format!("-f:ExtraLinkerFlags={}", file.to_str().unwrap().replace('\\', "/"));
    let o = qb64rust_build(&d, &[&flag, "-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let state = d.join("state.txt");
    let run = run(
        unattended(&exe)
            .current_dir(&d)
            .env("QB64_SCREEN_STATE", &state)
            .env("QB64_SCREEN_KEYS", d.join("p.keys")),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(run.status.code(), Some(125));
    let text = std::fs::read(&state).unwrap();
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
    std::fs::write(d.join("p.bas"), "SCREEN 13\nPSET (1, 1), 4\nSYSTEM\n").unwrap();
    let exe = d.join("p.exe");
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/screen/screen_state.cpp");
    let flag = format!("-f:ExtraLinkerFlags={}", file.to_str().unwrap().replace('\\', "/"));
    let o = qb64rust_build(&d, &[&flag, "-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    for (keys, line) in [
        ("mouse 40 30\npress 9\n", 2),
        ("# a move needs x and y\nmouse 10\npress 1\n", 2),
        ("mouse 4 3\nrelease 0\n", 2),
        ("wheel 0\n", 1),
    ] {
        std::fs::write(d.join("p.keys"), keys).unwrap();
        let run = run(
            unattended(&exe)
                .current_dir(&d)
                .env("QB64_SCREEN_STATE", d.join("state.txt"))
                .env("QB64_SCREEN_KEYS", d.join("p.keys")),
            None,
            PROGRAM_LIMIT,
            "program",
        );
        assert_eq!(run.status.code(), Some(125), "{keys:?}");
        let err = run.stderr.escape_ascii().to_string();
        assert!(
            err.contains("p.keys line ") && err.contains(&format!("line {line}: not a step")),
            "{keys:?}: {err}"
        );
    }
}

/// A mouse step waits until the window shows the displayed page, since `_MOUSEX` maps an event by the frame libqb
/// last drew (`tools/screen/README.md`). A window that never shows (`$SCREENHIDE`) ends the wait after its time
/// with exit code 124 and a last line of the state that names the step.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn screen_state_mouse_step_of_a_hidden_window() {
    let d = scratch("mouse-hidden");
    std::fs::write(
        d.join("p.bas"),
        "$SCREENHIDE\nSCREEN 13\nDO: k$ = INKEY$: LOOP WHILE k$ = \"\"\nSYSTEM\n",
    )
    .unwrap();
    std::fs::write(d.join("p.keys"), "mouse 1 1\ntext x\n").unwrap();
    let exe = d.join("p.exe");
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/screen/screen_state.cpp");
    let flag = format!("-f:ExtraLinkerFlags={}", file.to_str().unwrap().replace('\\', "/"));
    let o = qb64rust_build(&d, &[&flag, "-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    let state = d.join("state.txt");
    let run = run(
        unattended(&exe)
            .current_dir(&d)
            .env("QB64_SCREEN_STATE", &state)
            .env("QB64_SCREEN_KEYS", d.join("p.keys"))
            .env("QB64_SCREEN_PROMPT_MS", "500"),
        None,
        PROGRAM_LIMIT,
        "program",
    );
    assert_eq!(run.status.code(), Some(124));
    let text = std::fs::read(&state).unwrap();
    assert!(
        text.ends_with(b"p.keys line 1: the window never showed the screen\n"),
        "{}",
        text.escape_ascii()
    );
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
    assert!(holds(&std::fs::read(&exe).unwrap(), &ico[offset..offset + len]));
}

/// Builds `program` as `p.bas` in `d` with `-q -x`, runs it and gives its output with `\n` line ends.
fn build_and_run(d: &Path, program: &str) -> String {
    build_and_run_tracing(d, program, "")
}

/// [`build_and_run`] of a program that takes the paths of rows of `DIVERGENCES.md`: the run-time trace
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
    assert_eq!(run.status.code(), Some(0), "{}", stdout(&run));
    let taken: Vec<u8> = std::fs::read(&trace)
        .unwrap_or_default()
        .into_iter()
        .filter(|&b| b != b'\r')
        .collect();
    assert_eq!(taken, rows.as_bytes(), "the rows of DIVERGENCES.md the program took");
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
        "coverage 1\nfile\t0\t{}\tp.bas\nfile\t1\t{}\tinc.bi\n0\t1\t1\t1\n0\t2\t1\t1\n0\t3\t4\t1\n0\t4\t5\t0\n\
         1\t2\t1\t1\n0\t7\t1\t1\nend\n",
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
        build_and_run_tracing(&d, program, "D-006\n"),
        "[handler 6 ]\n 0 \n[handler 6 ]\n 0 \n-7  0 -1073741824 \n"
    );
}

/// A FUNCTION in a bound of a `REDIM` that gives the same array new bounds or erases it (`DIVERGENCES.md` D-018:
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
/// fewer elements than declared, raises error 9, reads the rest and keeps that member as it was (`DIVERGENCES.md`
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
/// (`DIVERGENCES.md` D-020: the old compiler's size overflows unchecked, `verification\v35_g3_dyn_bound_too_large`).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn redim_of_a_dynamic_member_too_large() {
    let d = scratch("dyn-too-large");
    let program = "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\nv(1) _DYNAMIC AS _INTEGER64\nEND TYPE\n\
                   ON ERROR GOTO trap\nDIM x AS t\nDIM hi AS _INTEGER64: hi = 2147483647\n\
                   REDIM x.v(0 TO hi, 0 TO hi) AS _INTEGER64\nPRINT \"not reached\"\nSYSTEM\n\
                   trap:\nPRINT \"[\"; ERR; \"]\";\nRESUME NEXT\n";
    // "Out of memory" goes to stderr; the handler is not called and the program stops there.
    assert_eq!(build_and_run_tracing(&d, program, "D-020\n"), "");
}

/// `_MEM` of a dynamic member array that is not allocated, and `GET` of a whole array of its type into an erased
/// array, raise error 9 and go on (`DIVERGENCES.md` D-021: the old program crashes, or ends with "Out of memory",
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
    assert_eq!(build_and_run_tracing(&d, program, "D-021\n"), "[ 9 ]mem 0 \n[ 9 ]get\n");
}

/// D-003 and D-004 differ from the old compiler only in a program that goes on after the error: with no handler and
/// `QB64PE_NOPROMPT=y` the error ends the program as it ends the old one, and the trace names no row; with
/// `continue` the program goes on and the trace names both.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn an_error_that_ends_the_program_traces_nothing() {
    let d = scratch("trace-ends");
    let program = "$CONSOLE:ONLY\nTYPE t\nm AS LONG\nEND TYPE\nDIM a(3) AS t\nPRINT \"before\"\nRETURN back\nback:\n\
                   a(9).m = 5\nPRINT \"after\"\nSYSTEM\n";
    std::fs::write(d.join("p.bas"), program).unwrap();
    let exe = d.join("p.exe");
    let o = qb64rust_build(&d, &["-q", "-x", "p.bas", "-o", exe.to_str().unwrap()]);
    assert!(o.status.success(), "{}", stdout(&o));
    for (noprompt, rows, output) in [("y", "", "before\n"), ("continue", "D-003\nD-004\n", "before\nafter\n")] {
        let trace = d.join("divergences.txt");
        let _ = std::fs::remove_file(&trace);
        let run = run(
            unattended(&exe)
                .current_dir(&d)
                .env("QB64PE_NOPROMPT", noprompt)
                .env("QB64RUST_DIVERGENCES", &trace),
            None,
            PROGRAM_LIMIT,
            "program",
        );
        assert_eq!(stdout(&run).replace("\r\n", "\n"), output, "QB64PE_NOPROMPT={noprompt}");
        let taken: Vec<u8> = std::fs::read(&trace)
            .unwrap_or_default()
            .into_iter()
            .filter(|&b| b != b'\r')
            .collect();
        assert_eq!(taken, rows.as_bytes(), "QB64PE_NOPROMPT={noprompt}");
    }
}

/// `GET` of a whole array whose type has a dynamic member array unpacks the statement's buffer whatever its read
/// raised, as the old compiler does; where that statement never read, the buffer holds zeros (`DIVERGENCES.md`
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
/// copies nothing, a `STRING` member array included (`DIVERGENCES.md` D-004: the old compiler copies the strings
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
/// nothing, numbers and `STRING * n` alike (`DIVERGENCES.md` D-004: the old compiler copies through the member of
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

/// `_ARRAYCOPY` of `_FLOAT` arrays copies the elements the program names (`DIVERGENCES.md` D-025: the old compiler
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

/// A whole-array assignment of arrays of an owner type with other bounds raises error 9 and copies the target's
/// number of elements, as the old compiler does, but nothing when the source has fewer (`DIVERGENCES.md` D-022: the
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
/// own lock, so the block stays valid when an element changes (`DIVERGENCES.md` D-016: the old compiler reads the
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
/// (`DIVERGENCES.md` D-007: the old evaluator wraps it to -9223372036854775808).
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn const_power_beyond_integer64() {
    let d = scratch("const-pow");
    let program = "$CONSOLE:ONLY\nCONST c = 2 ^ 70\nPRINT c\nSYSTEM\n";
    assert_eq!(build_and_run(&d, program), " 1.180591620717411D+21 \n");
}

/// The `CONST` rulings of 2026-10-18, built and run: an unsuffixed literal beyond `_INTEGER64` and the limits
/// (`DIVERGENCES.md` D-027; the old compiler prints 9223372036854775807 and -9223372036854775807), `2 ^ -1` (D-028;
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
/// build does (spec `language/asserts-and-debug`, "The sockets part"; `DIVERGENCES.md` D-017, narrowed by
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
/// `STOP` stands for any statement not compiled yet; when it is, take another.
#[test]
fn report_unsupported_counts_a_folder() {
    let d = scratch("report");
    std::fs::create_dir(d.join("sub")).unwrap();
    std::fs::write(d.join("clean.bas"), SLICE_PROGRAM).unwrap();
    let uses = qb64rust_base::MAX_ERRORS + 5;
    let many = format!("$CONSOLE:ONLY\n{}SYSTEM\n", "STOP\n".repeat(uses));
    std::fs::write(d.join("sub").join("MANY.BAS"), many).unwrap();
    std::fs::write(d.join("one.bas"), "$CONSOLE:ONLY\nSTOP\nSYSTEM\n").unwrap();
    std::fs::write(d.join("rejected.bas"), "$CONSOLE:ONLY\nSTOP\nx =\n").unwrap();
    std::fs::write(d.join("rejected.err"), "Expected expression\n").unwrap();
    let o = qb64rust(&d, &["--report", "unsupported", "."]);
    assert!(o.status.success(), "{}", stdout(&o));
    let expected = format!(
        "3 programs: 1 without errors, 2 with only \"not supported yet\" errors, 0 with other errors\n\
         1 left out: an .err file beside them (the old compiler rejects them)\n\
         programs  alone   uses  not supported yet\n       \
         2      2 {:>6}  `STOP`\n",
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

/// The libraries of `verification\v37_lib` built into `dir` with the clone's toolchain, as `build_libs.py` builds
/// them: `v37c.a` (C), `v37cpp.a` (C++), `v37o.o` (C), `v37dll.dll`.
fn build_v37_libraries(dir: &Path) {
    let root = qb64rust_driver::build::find_root(None).expect("the QB64pe reference clone");
    let bin = root.join("internal/c/c_compiler/bin");
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
