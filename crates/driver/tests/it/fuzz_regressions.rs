//! The fuzz regression test (spec `testing/fuzzing`, "Regression inputs"; OpenSpec change `fuzz-front-end`, design
//! D6): every file in `tests\fuzz\regressions\` goes through `probe` for all three targets, on a thread with the
//! compiler's stack, and none may panic or break a property. An empty folder passes.
//!
//! A file found by a timeout (more than 10 seconds in the fuzzer) is held only to return here: a time limit in a
//! test would fail under load.
//!
//! The fuzz dictionary is written here too (`write_fuzz_dictionary`), so that it comes from the compiler's tables
//! without a command-line option for it.

use qb64rust_driver::{Probe, STACK_SIZE, probe};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

fn regression_files() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fuzz/regressions");
    let mut out: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_file() && p.file_name().is_some_and(|n| n != "README.md"))
        .collect();
    out.sort();
    out
}

/// The problems of one input: one line per target that failed.
fn problems(bytes: &[u8]) -> Vec<String> {
    [Probe::Parse, Probe::Compile, Probe::Server]
        .into_iter()
        .filter_map(|kind| match catch_unwind(AssertUnwindSafe(|| probe(kind, bytes))) {
            Ok(Ok(_)) => None,
            Ok(Err(problem)) => Some(format!("{kind:?}: {problem}")),
            Err(e) => Some(format!(
                "{kind:?}: panic: {}",
                e.downcast_ref::<String>()
                    .cloned()
                    .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default()
            )),
        })
        .collect()
}

/// The fuzz dictionary (design D5, step 1), in libFuzzer's format: one quoted word per line, from the compiler's own
/// tables. The reserved words, the built-ins as they are written (`LEFT$`), and the words of QB64pe's layout, among
/// them every metacommand (`$INCLUDE`, `$RESIZE:STRETCH`). Each word once, sorted.
fn fuzz_dictionary() -> String {
    let mut words: Vec<String> = qb64rust_syntax::keywords().iter().map(|k| k.to_string()).collect();
    words.extend(
        qb64rust_builtins::BUILTINS
            .iter()
            .map(|b| format!("{}{}", b.name.to_ascii_uppercase(), b.musthave.unwrap_or(""))),
    );
    words.extend(qb64rust_format::words::WORDS.iter().map(|(upper, _)| upper.to_string()));
    words.retain(|w| !w.is_empty());
    words.sort();
    words.dedup();
    let mut out = String::from("# The fuzz dictionary, written by tools/fuzz/fuzz.py from the compiler's tables.\n");
    for w in words {
        out.push('"');
        for c in w.chars() {
            // libFuzzer's escapes: a backslash and a quote are escaped, anything outside printable ASCII as \xNN.
            match c {
                '\\' | '"' => {
                    out.push('\\');
                    out.push(c);
                }
                ' '..='~' => out.push(c),
                _ => {
                    let mut buf = [0u8; 4];
                    for b in c.encode_utf8(&mut buf).bytes() {
                        out.push_str(&format!("\\x{b:02X}"));
                    }
                }
            }
        }
        out.push_str("\"\n");
    }
    out
}

#[test]
fn fuzz_dictionary_has_keywords_builtins_and_metacommands() {
    let dict = fuzz_dictionary();
    for word in ["\"PRINT\"\n", "\"_RGB32\"\n", "\"$INCLUDE\"\n", "\"LEFT$\"\n"] {
        assert!(dict.contains(word), "the fuzz dictionary has no line {word:?}");
    }
}

/// Writes the fuzz dictionary to the file `QB64RUST_FUZZ_DICT` names. Run by `tools/fuzz/fuzz.py`. Without the
/// variable it does nothing, so that `cargo test -- --ignored` (the `cli` step) passes it.
#[test]
#[ignore = "run by tools/fuzz/fuzz.py"]
fn write_fuzz_dictionary() {
    let Some(path) = std::env::var_os("QB64RUST_FUZZ_DICT") else {
        return;
    };
    std::fs::write(&path, fuzz_dictionary()).unwrap();
}

#[test]
fn fuzz_regression_inputs_pass_every_probe() {
    let files = regression_files();
    let failures: Vec<String> = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            files
                .iter()
                .flat_map(|path| {
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();
                    let bytes = std::fs::read(path).unwrap();
                    problems(&bytes).into_iter().map(move |p| format!("{name}: {p}"))
                })
                .collect()
        })
        .expect("cannot start the regression thread")
        .join()
        .unwrap();
    assert!(
        failures.is_empty(),
        "fuzz regression inputs failed:\n{}",
        failures.join("\n")
    );
}
