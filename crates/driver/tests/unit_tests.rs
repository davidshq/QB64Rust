//! One test per scenario of the `compiler/unit-tests` spec (`qb64rust test`), on the fixtures of `tests\unit_tests\`
//! (its README). The scenarios that list or reject options build nothing and always run; those that build a test
//! file need the QB64pe reference clone and its toolchain, and are `#[ignore]`d like the CLI's:
//! `cargo test -p qb64rust-driver --test unit_tests -- --ignored`.

#[expect(dead_code, reason = "the CLI tests' helpers, of which these tests use a few")]
mod support;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Instant;

use support::{COMPILER_LIMIT, run};

/// The fixtures' folder.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/unit_tests")
}

/// `qb64rust test <args>` in `dir`, with a temp folder of its own, removed after it: the folders kept after the
/// fixtures' failures go with it.
fn qb64rust_test_in(dir: &Path, args: &[&str]) -> Output {
    let temp = tempfile::Builder::new().prefix("qb64rust-unit-").tempdir().unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_qb64rust"));
    cmd.env(qb64rust_driver::build::OBJECTS_LOCK_ENV, support::objects_lock());
    for var in ["TEMP", "TMP", "TMPDIR"] {
        cmd.env(var, temp.path());
    }
    run(
        cmd.current_dir(dir).arg("test").args(args),
        None,
        COMPILER_LIMIT,
        "qb64rust test",
    )
}

/// `qb64rust test <args>` in the fixtures' folder.
fn qb64rust_test(args: &[&str]) -> Output {
    qb64rust_test_in(&fixtures(), args)
}

/// Its stdout, with `/` for `\` and `\n` line ends, so that one text holds on every platform.
#[expect(clippy::disallowed_methods, reason = "the command's own text")]
fn text(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout)
        .replace("\r\n", "\n")
        .replace('\\', "/")
}

/// Its stdout as written, for `--format json`.
#[expect(clippy::disallowed_methods, reason = "the command's own text")]
fn json_text(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

/// The result line of `file:sub` in a text run: its first word.
fn result_word(out: &str, test: &str) -> String {
    let at = format!(" {test} (");
    out.lines()
        .find(|l| l.contains(&at) && !l.starts_with("---"))
        .and_then(|l| l.split_whitespace().next())
        .unwrap_or_else(|| panic!("no result of {test}:\n{out}"))
        .to_string()
}

/// The details of `file:sub` in a text run: the lines after its `---` header, up to the next blank line.
fn details(out: &str, word: &str, test: &str) -> String {
    let head = format!("--- {word} {test}\n");
    let start = out
        .find(&head)
        .unwrap_or_else(|| panic!("no details of {test}:\n{out}"))
        + head.len();
    out[start..].split("\n\n").next().unwrap_or_default().to_string()
}

// Test files and test SUBs.

/// "Found in a folder": `TestAdd` and `test_sub` are tests; `TestWith` takes parameters (noted) and `TestF` is a
/// FUNCTION.
#[test]
fn found_in_a_folder() {
    let o = qb64rust_test(&["lib", "--list"]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(0), "{out}");
    assert!(out.contains("lib/math_test.bas:4: TestAdd\n"), "{out}");
    assert!(out.contains("lib/math_test.bas:8: test_sub\n"), "{out}");
    assert!(
        out.contains("lib/math_test.bas:12: TestWith takes parameters: not a test\n"),
        "{out}"
    );
    assert!(!out.contains("TestF"), "{out}");
    assert_eq!(out.lines().count(), 3, "{out}");
}

/// "A SUB of an included file": `math.bm`'s `TestHelper` is not a test of `math_test.bas`.
#[test]
fn a_sub_of_an_included_file() {
    let out = text(&qb64rust_test(&["lib/math_test.bas", "--list"]));
    assert!(!out.contains("TestHelper"), "{out}");
}

/// "A commented-out SUB": neither `' SUB TestOld` nor the string `"SUB TestStr"` is a test.
#[test]
fn a_commented_out_sub() {
    let out = text(&qb64rust_test(&["lib/math_test.bas", "--list"]));
    assert!(!out.contains("TestOld") && !out.contains("TestStr"), "{out}");
}

/// "Not a test file by name": `helpers.bas` is taken only when named.
#[test]
fn not_a_test_file_by_name() {
    assert!(!text(&qb64rust_test(&["lib", "--list"])).contains("helpers.bas"));
    let o = qb64rust_test(&["lib/helpers.bas", "--list"]);
    assert_eq!(text(&o), "lib/helpers.bas:2: TestX\n");
}

// The entry point.

/// "Includes resolve from the test file": `math_test.bas` includes `math.bm` from its folder, builds, and nothing is
/// added to the folder.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn includes_resolve_from_the_test_file() {
    let names = || {
        let mut n: Vec<_> = std::fs::read_dir(fixtures().join("lib"))
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        n.sort();
        n
    };
    let before = names();
    let o = qb64rust_test(&["lib"]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(0), "{out}");
    assert!(out.ends_with("2 passed, 0 failed, 0 errors, 0 timed out\n"), "{out}");
    assert_eq!(names(), before);
}

/// "Module-level code runs before each test": `TestA` changes `n`, `TestB` still sees 5.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn module_level_code_runs_before_each_test() {
    let out = text(&qb64rust_test(&["setup", "--jobs", "1"]));
    assert_eq!(result_word(&out, "setup/setup_test.bas:TestA"), "ok", "{out}");
    assert_eq!(result_word(&out, "setup/setup_test.bas:TestB"), "ok", "{out}");
}

/// "The fixes of the test file's folder": `qb64rust.toml` beside the test file turns `resume-after-block` on, so the
/// test passes; with `--fixes off` it does not.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn the_fixes_of_the_test_files_folder() {
    let o = qb64rust_test(&["fixes/toml"]);
    assert_eq!(o.status.code(), Some(0), "{}", text(&o));
    let o = qb64rust_test(&["fixes/toml", "--fixes", "off", "--timeout", "5"]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    assert_ne!(result_word(&out, "fixes/toml/resume_test.bas:TestWhile"), "ok", "{out}");
}

/// "A compile error": both tests of `bad_test.bas` are build errors naming its line 4; `good_test.bas` still runs.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_compile_error() {
    let o = qb64rust_test(&["build_error"]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    for t in ["TestOne", "TestTwo"] {
        let test = format!("build_error/bad_test.bas:{t}");
        assert_eq!(result_word(&out, &test), "ERROR", "{out}");
        let d = details(&out, "ERROR", &test);
        assert!(d.contains("did not build") && d.contains("bad_test.bas:4:"), "{d}");
        // A front end's error writes nothing into the folder, so none is named as kept.
        assert!(!d.contains("its folder is kept"), "{d}");
    }
    assert_eq!(result_word(&out, "build_error/good_test.bas:TestGood"), "ok", "{out}");
}

// Running a test.

/// "A timeout": `TestLoop` is ended after about 2 seconds and reported as timed out; `TestQuick` still runs.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_timeout() {
    let start = Instant::now();
    let o = qb64rust_test(&["loop", "--timeout", "2"]);
    let out = text(&o);
    assert_eq!(result_word(&out, "loop/loop_test.bas:TestLoop"), "TIMEOUT", "{out}");
    assert!(
        details(&out, "TIMEOUT", "loop/loop_test.bas:TestLoop").contains("timed out after 2 s"),
        "{out}"
    );
    assert_eq!(result_word(&out, "loop/loop_test.bas:TestQuick"), "ok", "{out}");
    assert!(out.ends_with("1 passed, 0 failed, 0 errors, 1 timed out\n"), "{out}");
    // The build takes most of the rest.
    assert!(start.elapsed().as_secs() < 120, "{:?}", start.elapsed());
}

/// "Input does not wait": `LINE INPUT` reads the end of the null device at once; that raises error 62
/// (`QB64PE-DIVERGENCES.md` D-048), so the test is an error, not a wait.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn input_does_not_wait() {
    let out = text(&qb64rust_test(&["judge", "--filter", "TestInput", "--timeout", "10"]));
    let test = "judge/judge_test.bas:TestInput";
    assert_eq!(result_word(&out, test), "ERROR", "{out}");
    assert!(
        details(&out, "ERROR", test).starts_with("Input past end of file\n"),
        "{out}"
    );
}

/// "A folder of its own": two tests write `TestTempDir$ + "out.txt"` at once and each reads back what it wrote.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_folder_of_its_own() {
    let o = qb64rust_test(&["temp", "--jobs", "2"]);
    assert_eq!(o.status.code(), Some(0), "{}", text(&o));
}

// How a test is judged.

/// The text of a run of `judge_test.bas`, once for the scenarios that read it.
fn judged() -> &'static str {
    static OUT: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    OUT.get_or_init(|| text(&qb64rust_test(&["judge", "--timeout", "10"])))
}

/// "A runtime error": an error with libqb's message and its line.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_runtime_error() {
    let out = judged();
    let test = "judge/judge_test.bas:TestDiv";
    assert_eq!(result_word(out, test), "ERROR", "{out}");
    let d = details(out, "ERROR", test);
    assert!(
        d.starts_with("Subscript out of range\n  at judge/judge_test.bas:22\n"),
        "{d}"
    );
}

/// "Ending early": a SUB of the code under test runs `SYSTEM`; an error, not a pass.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn ending_early() {
    let out = judged();
    let test = "judge/judge_test.bas:TestEnd";
    assert_eq!(result_word(out, test), "ERROR", "{out}");
    assert!(
        details(out, "ERROR", test).starts_with("the test ended before its SUB returned\n"),
        "{out}"
    );
}

/// "Output that looks like a pass": `PASS` and `ok` printed, then `ERROR 5`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn output_that_looks_like_a_pass() {
    let out = judged();
    let test = "judge/judge_test.bas:TestFake";
    assert_eq!(result_word(out, test), "ERROR", "{out}");
    assert!(
        details(out, "ERROR", test).starts_with("Illegal function call\n"),
        "{out}"
    );
}

/// "A failed _ASSERT": it fails with the message `math` and line 9 of the test file.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_failed_assert() {
    let out = judged();
    let test = "judge/judge_test.bas:TestAssert";
    assert_eq!(result_word(out, test), "FAIL", "{out}");
    assert!(
        details(out, "FAIL", test).starts_with("math\n  at judge/judge_test.bas:9\n"),
        "{out}"
    );
}

// The assertion library.

/// "Expected and actual": `AssertEqualInt 4, 2 + 3, "sum"`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn expected_and_actual() {
    let out = judged();
    let test = "judge/judge_test.bas:TestAdd";
    assert_eq!(result_word(out, test), "FAIL", "{out}");
    assert!(
        details(out, "FAIL", test).starts_with("sum: expected 4, got 5\n"),
        "{out}"
    );
}

/// "The first failure ends the test": `Fail "one"`, then `Fail "two"`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn the_first_failure_ends_the_test() {
    let out = judged();
    let d = details(out, "FAIL", "judge/judge_test.bas:TestFailTwice");
    assert!(d.starts_with("one\n") && !d.contains("two"), "{d}");
}

/// "Another default type": `DEFSTR A-Z`, `OPTION _EXPLICIT` and `OPTION BASE 1` change nothing in the library.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn another_default_type() {
    let out = text(&qb64rust_test(&["deftype"]));
    assert_eq!(result_word(&out, "deftype/deftype_test.bas:TestStrings"), "ok", "{out}");
    let test = "deftype/deftype_test.bas:TestCount";
    assert_eq!(result_word(&out, test), "FAIL", "{out}");
    assert!(
        details(&out, "FAIL", test).starts_with("sum: expected 4, got 5\n"),
        "{out}"
    );
}

/// "A SINGLE compared as a SINGLE": `x! = 0.1` equals `0.1`, and the other assertions of `TestPass` hold.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_single_compared_as_a_single() {
    let out = judged();
    assert_eq!(result_word(out, "judge/judge_test.bas:TestSingle"), "ok", "{out}");
    assert_eq!(result_word(out, "judge/judge_test.bas:TestPass"), "ok", "{out}");
}

/// "A name of the library taken": the code under test has its own `SUB AssertTrue`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_name_of_the_library_taken() {
    let out = text(&qb64rust_test(&["clash"]));
    let test = "clash/clash_test.bas:TestMine";
    assert_eq!(result_word(&out, test), "ERROR", "{out}");
    assert!(
        details(&out, "ERROR", test).contains("name already in use: `AssertTrue`"),
        "{out}"
    );
}

// Selecting and listing tests.

/// "A filter": only `TestAdd` of `math_test.bas` is built and run, from the whole fixtures' folder.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_filter() {
    let o = qb64rust_test(&["--filter", "math_test.bas:TestAdd", "--format", "json"]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(0), "{out}");
    let names: Vec<String> = (json_text(&o).lines())
        .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap())
        .filter(|v| v["event"] == "test" || v["event"] == "result")
        .map(|v| format!("{} {}", v["event"].as_str().unwrap(), v["name"].as_str().unwrap()))
        .collect();
    assert_eq!(names, ["test TestAdd", "result TestAdd"], "{out}");
}

/// "Nothing found": a folder with no test file.
#[test]
fn nothing_found() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir(d.path().join("empty")).unwrap();
    let o = qb64rust_test_in(d.path(), &["empty"]);
    assert_eq!(o.status.code(), Some(1));
    assert_eq!(text(&o), "qb64rust test: no test found\n");
}

// Results and exit status.

/// "All pass": the last line counts them as passed, and the exit status is 0.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn all_pass() {
    let o = qb64rust_test(&["setup"]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(0), "{out}");
    assert!(out.ends_with("\n2 passed, 0 failed, 0 errors, 0 timed out\n"), "{out}");
}

/// "One fails": one test of three; its line starts with `FAIL`, its message and output follow; exit status 1.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn one_fails() {
    let o = qb64rust_test(&[
        "judge",
        "--filter",
        "TestPass",
        "--filter",
        "TestSingle",
        "--filter",
        "TestAssert",
    ]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    let fails: Vec<_> = out.lines().filter(|l| l.starts_with("FAIL ")).collect();
    assert_eq!(fails.len(), 1, "{out}");
    assert!(
        fails[0].starts_with("FAIL    judge/judge_test.bas:TestAssert ("),
        "{out}"
    );
    let d = details(&out, "FAIL", "judge/judge_test.bas:TestAssert");
    assert!(d.starts_with("math\n") && d.contains("output:\n  before\n"), "{d}");
    assert!(out.ends_with("\n2 passed, 1 failed, 0 errors, 0 timed out\n"), "{out}");
}

/// "JSON lines": every line is a JSON object; one names the test with its file and line, one gives `pass` and its
/// time.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn json_lines() {
    let o = qb64rust_test(&["setup", "--filter", "TestB", "--format", "json"]);
    assert_eq!(o.status.code(), Some(0), "{}", text(&o));
    let lines: Vec<serde_json::Value> = (json_text(&o).lines())
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| panic!("{e}: {l}")))
        .collect();
    let file = |v: &serde_json::Value| v["file"].as_str().unwrap().replace('\\', "/");
    let test = lines.iter().find(|v| v["event"] == "test").unwrap();
    assert_eq!(
        (file(test).as_str(), &test["name"], &test["line"]),
        ("setup/setup_test.bas", &"TestB".into(), &11.into())
    );
    let result = lines.iter().find(|v| v["event"] == "result").unwrap();
    assert_eq!(result["outcome"], "pass");
    assert!(result["ms"].is_u64(), "{result}");
    assert_eq!(lines.last().unwrap()["event"], "summary");
}

// Build options of a test run.

/// "A rejected option": `-o`, and `--dump` and `-y` with it, are usage errors.
#[test]
fn a_rejected_option() {
    for args in [&["-o", "x.exe", "lib"][..], &["--dump", "ir", "lib"], &["-y", "lib"]] {
        let o = qb64rust_test(args);
        let out = text(&o);
        assert_eq!(o.status.code(), Some(1), "{args:?}: {out}");
        assert!(
            out.contains("`test` does not take `") && out.contains("usage: qb64rust"),
            "{out}"
        );
    }
}

/// "A fix on the command line": `--fix whole-array-value=on` makes `PRINT a()` a compile error; without it the test
/// passes.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn a_fix_on_the_command_line() {
    let o = qb64rust_test(&["fixes/cmdline"]);
    assert_eq!(o.status.code(), Some(0), "{}", text(&o));
    let o = qb64rust_test(&["--fix", "whole-array-value=on", "fixes/cmdline"]);
    let out = text(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    let d = details(&out, "ERROR", "fixes/cmdline/array_test.bas:TestArray");
    assert!(d.contains("a whole array is not a value"), "{d}");
}

// Graphics (the design's non-goal, measured by task 4.5).

/// Images work under the entry point's `$CONSOLE:ONLY`: `_NEWIMAGE`, `_DEST` and `_SOURCE` of an image, `PSET` and
/// `_RGB32` into it, `_FREEIMAGE`. Drawing on the console itself, with no image as the destination, is an error.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn images_work_and_drawing_on_the_console_is_an_error() {
    let out = text(&qb64rust_test(&["graphics"]));
    for t in [
        "TestNewImage",
        "TestDestImage",
        "TestSourceImage",
        "TestPset",
        "TestRgb32",
        "TestFreeImage",
    ] {
        assert_eq!(
            result_word(&out, &format!("graphics/image_test.bas:{t}")),
            "ok",
            "{out}"
        );
    }
    let test = "graphics/image_test.bas:TestDrawOnConsole";
    assert_eq!(result_word(&out, test), "ERROR", "{out}");
    assert!(
        details(&out, "ERROR", test).starts_with("Illegal function call\n"),
        "{out}"
    );
}

// The check against the old compiler (design D8 of `basic-unit-tests`).

/// The entry point and the library are plain BASIC that `qb64pe.exe` compiles too, and the protocol does not rest
/// on D-012: `judge_test.bas` is built by both compilers from one entry point, and four of its tests are run with
/// each. The result file's lines are equal for a pass, a library failure and an end by `SYSTEM`; for a runtime
/// error the message on stderr is, the old program exits 0 where the new one exits 1, and both are an ERROR, since
/// neither wrote `DONE`.
#[test]
#[ignore = "needs the QB64pe reference clone"]
fn the_old_compiler_builds_the_same_protocol() {
    use qb64rust_driver::unit_tests::{discover, entry, judge};
    let clone = qb64rust_driver::build::find_root(None)
        .unwrap()
        .expect("the QB64pe reference clone");
    let d = tempfile::Builder::new().prefix("qb64rust-unit-old-").tempdir().unwrap();
    let dir = d.path();
    let source = std::fs::read(fixtures().join("judge/judge_test.bas")).unwrap();
    std::fs::write(dir.join("judge_test.bas"), &source).unwrap();
    std::fs::create_dir(dir.join("qb64rust-test")).unwrap();
    std::fs::write(dir.join(entry::LIBRARY_BI), entry::LIBRARY_BI_TEXT).unwrap();
    std::fs::write(dir.join(entry::LIBRARY_BM), entry::LIBRARY_BM_TEXT).unwrap();
    let found = discover::tests_of("judge_test.bas", source);
    std::fs::write(
        dir.join("entry.bas"),
        entry::entry_point("judge_test.bas", &found.tests),
    )
    .unwrap();

    let exe = |name: &str| dir.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let new = run(
        Command::new(env!("CARGO_BIN_EXE_qb64rust"))
            .env(qb64rust_driver::build::OBJECTS_LOCK_ENV, support::objects_lock())
            .current_dir(dir)
            .args(["-q", "-x", "entry.bas", "-o"])
            .arg(exe("new")),
        None,
        COMPILER_LIMIT,
        "qb64rust",
    );
    assert!(new.status.success(), "{}", text(&new));
    {
        // qb64pe.exe builds in the clone's internal\temp: the clone alone, as the runner takes it.
        // The clone's lock beside the objects' (`clone_lock.py`, `lock_clone`).
        let lock = std::fs::File::create(support::objects_lock().with_file_name("in-use.lock")).unwrap();
        lock.lock().unwrap();
        let mut cmd = Command::new(clone.join(format!("qb64pe{}", std::env::consts::EXE_SUFFIX)));
        let old = run(
            cmd.current_dir(dir)
                .args(["-q", "-m", "-x", "entry.bas", "-o"])
                .arg(exe("old")),
            None,
            COMPILER_LIMIT,
            "qb64pe",
        );
        assert!(exe("old").is_file(), "{}", text(&old));
    }

    /// One test run with one executable: the result file's lines, stderr and the verdict.
    fn run_test(dir: &Path, exe: &Path, name: &str) -> (String, String, judge::Outcome, Option<i32>) {
        let result = dir.join(format!("{}-{name}.txt", exe.file_stem().unwrap().to_string_lossy()));
        let o = run(
            Command::new(exe)
                .current_dir(dir)
                .env("QB64PE_NOPROMPT", "y")
                .env(entry::NAME_ENV, name)
                .env(entry::RESULT_ENV, &result),
            None,
            support::PROGRAM_LIMIT,
            "test",
        );
        #[expect(clippy::disallowed_methods, reason = "the program's own messages")]
        let (lines, stdout, stderr) = (
            String::from_utf8_lossy(&std::fs::read(&result).unwrap_or_default()).replace("\r\n", "\n"),
            String::from_utf8_lossy(&o.stdout).into_owned(),
            String::from_utf8_lossy(&o.stderr).replace("\r\n", "\n"),
        );
        let verdict = judge::judge(&judge::Run {
            result: &lines,
            stdout: &stdout,
            stderr: &stderr,
            exit: o.status.code(),
            timed_out: false,
        });
        (lines, stderr, verdict.outcome, o.status.code())
    }

    for (name, lines) in [
        ("TestPass", "BEGIN\nDONE\n"),
        ("TestAdd", "BEGIN\nFAIL\tsum: expected 4, got 5\n"),
        ("TestEnd", "BEGIN\n"),
    ] {
        let (new_lines, ..) = run_test(dir, &exe("new"), name);
        let (old_lines, ..) = run_test(dir, &exe("old"), name);
        assert_eq!(new_lines, lines, "{name}, qb64rust");
        assert_eq!(old_lines, lines, "{name}, qb64pe");
    }
    let (new_lines, new_err, new_outcome, new_code) = run_test(dir, &exe("new"), "TestDiv");
    let (old_lines, old_err, old_outcome, old_code) = run_test(dir, &exe("old"), "TestDiv");
    assert_eq!((new_lines.as_str(), old_lines.as_str()), ("BEGIN\n", "BEGIN\n"));
    let message = "Subscript out of range";
    assert!(
        new_err.contains(message) && old_err.contains(message),
        "{new_err}\n{old_err}"
    );
    assert_eq!((new_code, old_code), (Some(1), Some(0)));
    assert_eq!(
        (new_outcome, old_outcome),
        (judge::Outcome::Error, judge::Outcome::Error)
    );
}
