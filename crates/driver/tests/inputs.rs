//! Tier 1 (`study\19`, specs `testing/compiler-tests` and `testing/upstream-tests`): the front end over every
//! whole-program input set, without C++ (design D2 of `m2-upstream-tests`).
//!
//! | Prefix | Root | Verdict |
//! |---|---|---|
//! | `corpus/` | `tests\corpus` | rejected with an `.err` file, else accepted |
//! | `upstream/` | `tests\upstream\compile_tests` | `.bas` as `corpus/`; `.bi`/`.bm` include only |
//! | `snippets/` | `tests\snippets` | as `corpus/` |
//! | `differential/` | `tests\differential` | as `corpus/` (generated, spec `testing/differential-tests`) |
//! | `programs/` | `tests\programs` | as `corpus/` (real programs taken whole, `tests\programs\README.md`) |
//! | `qbasic/` | clone `tests\qbasic_testcases` | `.bas` accepted; `.bi`/`.bm` include only |
//! | `qb64pe-source/` | clone `source`, `internal\support` | `source\qb64pe.bas` accepted; the rest include only |
//!
//! The clone sets are read from the QB64pe reference clone (found as the driver finds it) and skipped, with a
//! note, when it is missing. Every file gets the no-panic and round-trip check; include-only files nothing else.

use qb64rust_driver::{FileLoader, build, emit, frontend_with, lower};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::tree::{Node, print};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// What the old compiler says about a file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    Accepted,
    Rejected,
    /// Not a program on its own (an include file).
    IncludeOnly,
}

/// One input file and what the front end made of it.
struct Outcome {
    /// `prefix/relative/path.bas`, with `/`.
    name: String,
    verdict: Verdict,
    /// The panic message, if the front end panicked.
    panic: Option<String>,
    round_trip: bool,
    errors: bool,
    /// The first error without the "not supported yet" marker, as `<line>:<column>: <message>`.
    first_real_error: Option<String>,
    /// The first parser diagnostic, else the first `Error` node, as `<line>:<column>: <message>`; `None` when the
    /// file parses cleanly.
    parse_gap: Option<String>,
    /// Whether the front end accepted the file, so it was lowered, validated and emitted.
    lowered: bool,
    /// The first problem `ir::validate` found in the lowered program.
    invalid_ir: Option<String>,
}

/// The first `Error` node below `n` in source order.
fn first_error_node(n: Node<'_>) -> Option<Node<'_>> {
    if n.kind() == SyntaxKind::Error {
        return Some(n);
    }
    n.child_nodes().find_map(first_error_node)
}

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn clone_root() -> Option<PathBuf> {
    build::find_root(None).ok()
}

fn has_ext(p: &Path, exts: &[&str]) -> bool {
    p.extension()
        .and_then(|x| x.to_str())
        .is_some_and(|x| exts.iter().any(|e| x.eq_ignore_ascii_case(e)))
}

/// Files under `dir` (recursively, sorted) with one of the extensions; none if `dir` does not exist.
fn files(dir: &Path, exts: &[&str]) -> Vec<PathBuf> {
    fn walk(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for e in entries {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, exts, out);
            } else if has_ext(&p, exts) {
                out.push(p);
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, exts, &mut out);
    out.sort();
    out
}

fn rel_name(prefix: &str, root: &Path, p: &Path) -> String {
    let rel = p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
    format!("{prefix}/{rel}")
}

/// `.bas` files rejected when they have an `.err` file, accepted otherwise; `.bi`/`.bm` include only.
fn labelled_set(prefix: &str, root: &Path, out: &mut Vec<(String, PathBuf, Verdict)>) {
    for p in files(root, &["bas", "bi", "bm"]) {
        let verdict = if !has_ext(&p, &["bas"]) {
            Verdict::IncludeOnly
        } else if p.with_extension("err").exists() {
            Verdict::Rejected
        } else {
            Verdict::Accepted
        };
        out.push((rel_name(prefix, root, &p), p, verdict));
    }
}

/// Every input file with its verdict. The clone sets are left out when the clone is missing.
fn inputs() -> Vec<(String, PathBuf, Verdict)> {
    let mut out = Vec::new();
    let tests = repo().join("tests");
    labelled_set("corpus", &tests.join("corpus"), &mut out);
    labelled_set("upstream", &tests.join("upstream/compile_tests"), &mut out);
    labelled_set("snippets", &tests.join("snippets"), &mut out);
    labelled_set("differential", &tests.join("differential"), &mut out);
    labelled_set("programs", &tests.join("programs"), &mut out);
    match clone_root() {
        Some(clone) => {
            let qbasic = clone.join("tests/qbasic_testcases");
            for p in files(&qbasic, &["bas", "bi", "bm"]) {
                let verdict = if has_ext(&p, &["bas"]) {
                    Verdict::Accepted
                } else {
                    Verdict::IncludeOnly
                };
                out.push((rel_name("qbasic", &qbasic, &p), p, verdict));
            }
            let main = clone.join("source/qb64pe.bas");
            for dir in ["source", "internal/support"] {
                for p in files(&clone.join(dir), &["bas", "bi", "bm"]) {
                    let verdict = if p == main {
                        Verdict::Accepted
                    } else {
                        Verdict::IncludeOnly
                    };
                    out.push((rel_name("qb64pe-source", &clone, &p), p, verdict));
                }
            }
        }
        None => eprintln!("skipped: no QB64pe clone (sets qbasic/ and qb64pe-source/)"),
    }
    out
}

/// The compiler root for included files of an input (design D9): `tests\upstream\root` for the repo's sets, as
/// the tier-2 runner passes with `--include-root` (it holds `tests/compile_tests/extra`, which two upstream tests
/// include relative to the compiler); the clone for the sets that depend on it.
fn include_root(name: &str) -> PathBuf {
    match clone_root() {
        Some(clone) if CLONE_SETS.iter().any(|s| name.starts_with(s)) => clone,
        _ => repo().join("tests/upstream/root"),
    }
}

/// Where an input's includes are looked up from: its own folder, except for the upstream tests of the old
/// compiler's own code (`upstream/qb64pe/`), which include its sources relative to their place in the clone
/// (`../../../source/…`): those use the clone's copy of their folder, when the clone is present.
fn include_base(name: &str, path: &Path) -> PathBuf {
    match (name.strip_prefix("upstream/"), clone_root()) {
        (Some(rel), Some(clone)) if name.starts_with(UPSTREAM_QB64PE) => clone.join("tests/compile_tests").join(rel),
        _ => path.to_path_buf(),
    }
}

fn run(name: String, path: &Path, verdict: Verdict) -> Outcome {
    let bytes = std::fs::read(path).unwrap();
    let root = include_root(&name);
    let base = include_base(&name, path);
    let result = catch_unwind(AssertUnwindSafe(|| {
        let fe = frontend_with(&name, bytes.clone(), |file| {
            Box::new(FileLoader::new(&root, file, &base))
        });
        // Every tree prints its own file back (spec compiler/pipeline, "Lossless syntax tree").
        let round_trip = fe
            .parsed
            .trees
            .iter()
            .all(|t| print(&t.green, &fe.map.file(t.file).bytes) == *fe.map.file(t.file).bytes);
        let mut list: Vec<_> = fe.diagnostics.list().to_vec();
        if fe.diagnostics.is_capped() {
            list.pop(); // "too many errors; stopping" is not an error of its own
        }
        list.sort_by_key(|d| d.span.start);
        let first_real_error = list.iter().find(|d| d.is_real_error()).map(|d| {
            let (line, col) = fe.map.file(d.span.file).line_col(d.span.start);
            format!("{line}:{col}: {}", d.message)
        });
        // The first parse gap of any tree, the main file's first (an included file's names its file).
        let parse_gap = fe.parsed.trees.iter().find_map(|t| {
            let file = fe.map.file(t.file);
            let at = |start: u32| {
                let (line, col) = file.line_col(start);
                if t.file == fe.file {
                    format!("{line}:{col}")
                } else {
                    let shown = file.name.replace('\\', "/");
                    let shown = shown.rsplit('/').next().unwrap_or(&shown).to_string();
                    format!("{shown}:{line}:{col}")
                }
            };
            t.diagnostics
                .list()
                .iter()
                .min_by_key(|d| d.span.start)
                .map(|d| {
                    let mark = if d.unsupported { "not supported yet: " } else { "" };
                    format!("{}: {mark}{}", at(d.span.start), d.message)
                })
                .or_else(|| {
                    first_error_node(t.root())
                        .map(|n| format!("{}: `Error` node without a parser diagnostic", at(n.span().start)))
                })
        });
        // Everything the front end accepts is lowered, validated and emitted (design D8 of `m2-arrays-and-types`);
        // a panic in either stage is caught below like one in the front end.
        let (lowered, invalid_ir) = if fe.has_errors() {
            (false, None)
        } else {
            let ir = lower(&fe);
            let invalid_ir = qb64rust_ir::validate(&ir).err().map(|p| p.join("; "));
            emit(&fe);
            (true, invalid_ir)
        };
        (
            round_trip,
            fe.has_errors(),
            first_real_error,
            parse_gap,
            lowered,
            invalid_ir,
        )
    }));
    match result {
        Ok((round_trip, errors, first_real_error, parse_gap, lowered, invalid_ir)) => Outcome {
            name,
            verdict,
            panic: None,
            round_trip,
            errors,
            first_real_error,
            parse_gap,
            lowered,
            invalid_ir,
        },
        Err(e) => Outcome {
            name,
            verdict,
            panic: Some(
                e.downcast_ref::<String>()
                    .cloned()
                    .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default(),
            ),
            round_trip: false,
            errors: false,
            first_real_error: None,
            parse_gap: None,
            lowered: false,
            invalid_ir: None,
        },
    }
}

/// The front end over every input, once per test binary, on all cores.
fn outcomes() -> &'static [Outcome] {
    static OUTCOMES: OnceLock<Vec<Outcome>> = OnceLock::new();
    OUTCOMES.get_or_init(|| {
        let inputs = inputs();
        let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
        let chunk = inputs.len().div_ceil(threads).max(1);
        std::thread::scope(|s| {
            let handles: Vec<_> = inputs
                .chunks(chunk)
                .map(|part| {
                    s.spawn(move || {
                        part.iter()
                            .map(|(name, path, verdict)| run(name.clone(), path, *verdict))
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
        })
    })
}

fn count(prefix: &str) -> usize {
    outcomes()
        .iter()
        .filter(|o| o.name.starts_with(&format!("{prefix}/")))
        .count()
}

#[test]
fn every_input_goes_through_the_front_end() {
    assert!(count("corpus") >= 270, "found only {} corpus programs", count("corpus"));
    assert!(
        count("upstream") >= 416,
        "found only {} upstream files",
        count("upstream")
    );
    assert!(
        count("differential") >= 59,
        "found only {} differential programs",
        count("differential")
    );
    assert!(count("programs") >= 1, "found no real program (tests\\programs)");
    let bad: Vec<String> = outcomes()
        .iter()
        .filter_map(|o| match &o.panic {
            Some(msg) => Some(format!("{}: panic: {msg}", o.name)),
            None if !o.round_trip => Some(format!("{}: round trip failed", o.name)),
            None => None,
        })
        .collect();
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// "Tier 1 corpus checks" (spec `testing/compiler-tests`): every input the front end accepts is lowered, validated
/// and emitted without a panic or an IR problem. The floor catches a set that went missing.
#[test]
fn accepted_inputs_lower_and_emit() {
    let lowered = outcomes().iter().filter(|o| o.lowered).count();
    eprintln!("lowered, validated and emitted: {lowered} inputs");
    assert!(lowered >= 130, "only {lowered} inputs were accepted and lowered");
    let bad: Vec<String> = outcomes()
        .iter()
        .filter_map(|o| o.invalid_ir.as_ref().map(|p| format!("{}: invalid IR: {p}", o.name)))
        .collect();
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn slice_list_programs_have_no_diagnostics() {
    let root = repo().join("tests/corpus");
    #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
    let list = std::fs::read_to_string(root.join("slice.list")).unwrap();
    let mut n = 0;
    for line in list.lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let name = format!("{line}.bas");
        let path = root.join(&name);
        let inc = repo().join("tests/upstream/root");
        let fe = frontend_with(&name, std::fs::read(&path).unwrap(), |file| {
            Box::new(FileLoader::new(&inc, file, &path))
        });
        // Warnings are allowed (`s18_const` has the chained `^` the constants spec warns about).
        assert!(!fe.has_errors(), "{line}:\n{}", fe.render_diagnostics());
        n += 1;
    }
    assert!(n > 0, "slice.list names no programs");
}

/// A program the old compiler rejects (it has an `.err` file) must get at least one error, so that support for a
/// new construct cannot make the new compiler accept it. No list can excuse this.
#[test]
fn rejected_programs_get_an_error() {
    let rejected: Vec<&Outcome> = outcomes().iter().filter(|o| o.verdict == Verdict::Rejected).collect();
    assert!(!rejected.is_empty(), "found no programs with an .err file");
    let accepted: Vec<&str> = rejected
        .iter()
        .filter(|o| o.panic.is_none() && !o.errors)
        .map(|o| o.name.as_str())
        .collect();
    assert!(
        accepted.is_empty(),
        "accepted, but the old compiler rejects them:\n{}",
        accepted.join("\n")
    );
}

/// The upstream tests of the old compiler's own code, which include its sources from the clone.
const UPSTREAM_QB64PE: &str = "upstream/qb64pe/";

/// Sets read from the clone, or that include files from it: their list entries are checked only when the clone
/// is present.
const CLONE_SETS: &[&str] = &["qbasic/", "qb64pe-source/", UPSTREAM_QB64PE];

/// A shrink-only list (design D4): `prefix/relative/path.bas`, sorted, one per line, `#` comments, optionally a
/// trailing ` # comment`.
struct List {
    file: &'static str,
    header: &'static str,
}

const FALSE_ERRORS: List = List {
    file: "known_false_errors.list",
    header: "# Programs the old compiler accepts that still get an error without the \"not supported yet\" marker \
             (spec\n# testing/upstream-tests, \"No false errors\"). Shrink-only: parser breadth empties it. The comment \
             is the first\n# such error. Regenerate with QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test \
             inputs.\n",
};

const UNSUPPORTED_REJECTIONS: List = List {
    file: "known_unsupported_rejections.list",
    header: "# Programs the old compiler rejects that get only errors marked \"not supported yet\" (spec\n# \
             testing/upstream-tests, \"Rejected programs stay rejected\"). Shrink-only: once a program gets a real \
             error it\n# leaves the list. Regenerate with QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver \
             --test inputs.\n",
};

const PARSE_GAPS: List = List {
    file: "known_parse_gaps.list",
    header: "# Programs the old compiler accepts, and every file of its own sources, whose parse reports a diagnostic \
             or\n# leaves an `Error` node (spec testing/upstream-tests, \"Parse gaps\"; design D1 of m2-parser-breadth). \
             Shrink-only:\n# parser breadth empties it. The comment is the first parser diagnostic, else the first \
             `Error` node.\n# Regenerate with QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs.\n",
};

/// Entries `want` has and the list `have` lacks (new), and entries of `have` that `want` lacks (stale); a
/// `have` entry is judged only when `checked` says its set was run.
fn list_diff<'a>(have: &'a [String], want: &[&'a str], checked: impl Fn(&str) -> bool) -> (Vec<&'a str>, Vec<&'a str>) {
    // Unchecked entries (a clone-dependent set without the clone: `upstream/qb64pe/` is read from the copy but
    // needs the clone for its includes) are neither new nor stale.
    let new = want
        .iter()
        .copied()
        .filter(|e| checked(e) && !have.iter().any(|h| h == e))
        .collect();
    let stale = have
        .iter()
        .map(String::as_str)
        .filter(|h| checked(h) && !want.contains(h))
        .collect();
    (new, stale)
}

impl List {
    fn path(&self) -> PathBuf {
        repo().join("tests").join(self.file)
    }

    /// The header comment (leading `#` lines) and the entries.
    fn read(&self) -> (String, Vec<String>) {
        #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
        let text = std::fs::read_to_string(self.path()).unwrap_or_default();
        let mut header = String::new();
        let mut entries = Vec::new();
        for line in text.lines() {
            let t = line.trim();
            if t.starts_with('#') && entries.is_empty() {
                header.push_str(line);
                header.push('\n');
            } else if !t.is_empty() && !t.starts_with('#') {
                entries.push(t.split(" #").next().unwrap().trim().to_string());
            }
        }
        if header.is_empty() {
            header = self.header.to_string();
        }
        (header, entries)
    }

    /// Compares the list with `want` (entry, comment). Returns the failure text, or rewrites the list when
    /// `QB64RUST_UPDATE_LISTS=1`. Entries of the clone sets are kept as they are when the clone is missing.
    fn check(&self, want: &[(String, Option<String>)], have_clone: bool) -> Option<String> {
        let (header, have) = self.read();
        let checked = |e: &str| have_clone || !CLONE_SETS.iter().any(|s| e.starts_with(s));
        let want_names: Vec<&str> = want.iter().map(|(e, _)| e.as_str()).collect();
        let (new, stale) = list_diff(&have, &want_names, checked);
        let mut counts: Vec<(String, usize)> = Vec::new();
        for (e, _) in want {
            let set = e.split('/').next().unwrap().to_string();
            match counts.iter_mut().find(|(s, _)| *s == set) {
                Some((_, n)) => *n += 1,
                None => counts.push((set, 1)),
            }
        }
        eprintln!("{}: {} entries {counts:?}", self.file, want.len());
        if new.is_empty() && stale.is_empty() {
            return None;
        }
        if std::env::var_os("QB64RUST_UPDATE_LISTS").is_some_and(|v| v == "1") {
            let mut lines: Vec<String> = want
                .iter()
                .map(|(e, c)| match c {
                    Some(c) => format!("{e}  # {c}"),
                    None => e.clone(),
                })
                .collect();
            // Clone-set entries cannot be recomputed without the clone: keep them.
            #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
            let old = std::fs::read_to_string(self.path()).unwrap_or_default();
            lines.extend(
                old.lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty() && !l.starts_with('#'))
                    .filter(|l| !checked(l.split(" #").next().unwrap().trim()))
                    .map(str::to_string),
            );
            lines.sort();
            let mut text = header;
            for l in lines {
                text.push_str(&l);
                text.push('\n');
            }
            std::fs::write(self.path(), text).unwrap();
            eprintln!("rewrote tests/{}", self.file);
            return None;
        }
        let mut msg = format!("tests/{} is out of date", self.file);
        if !new.is_empty() {
            msg.push_str(&format!(
                "\nnew entries (fix the cause, or add them):\n  {}",
                new.join("\n  ")
            ));
            for (e, c) in want.iter().filter(|(e, _)| new.contains(&e.as_str())) {
                if let Some(c) = c {
                    msg.push_str(&format!("\n  {e}: {c}"));
                }
            }
        }
        if !stale.is_empty() {
            msg.push_str(&format!("\nstale entries (remove them):\n  {}", stale.join("\n  ")));
        }
        msg.push_str("\nRegenerate with QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs");
        Some(msg)
    }
}

/// "Parse gaps", scenarios "New parse gap" and "Gap closed" (spec `testing/upstream-tests`): a program that newly
/// fails to parse is reported with its diagnostic, and one that parses cleanly asks for its entry to go.
#[test]
fn parse_gap_list_reports_new_and_closed_gaps() {
    let have = vec!["upstream/a.bas".to_string(), "upstream/closed.bas".to_string()];
    let (new, stale) = list_diff(&have, &["upstream/a.bas", "upstream/regressed.bas"], |_| true);
    assert_eq!(new, ["upstream/regressed.bas"]);
    assert_eq!(stale, ["upstream/closed.bas"]);
    // Without the clone, entries of its sets are kept, never reported stale.
    let have = vec!["qbasic/x.bas".to_string()];
    let checked = |e: &str| !CLONE_SETS.iter().any(|s| e.starts_with(s));
    assert_eq!(list_diff(&have, &[], checked), (vec![], vec![]));
    // The comment is the first parser diagnostic, else the first `Error` node.
    let gap = |src: &[u8]| run("t.bas".into(), write_temp(src).path(), Verdict::Accepted).parse_gap;
    assert_eq!(gap(b"PRINT 1\n"), None);
    assert!(
        gap(b"PRINT (1\n").is_some_and(|g| g.starts_with("1:")),
        "{:?}",
        gap(b"PRINT (1\n")
    );
}

/// A temporary `.bas` file holding `bytes`, deleted when dropped.
fn write_temp(bytes: &[u8]) -> tempfile::NamedTempFile {
    let mut f = tempfile::Builder::new().suffix(".bas").tempfile().unwrap();
    std::io::Write::write_all(&mut f, bytes).unwrap();
    f
}

/// "No false errors", "Rejected programs stay rejected" and "Parse gaps" (spec `testing/upstream-tests`): the
/// three lists name exactly the programs that need an entry.
#[test]
fn known_lists_are_exact() {
    let have_clone = clone_root().is_some();
    let ok = |o: &&Outcome| o.panic.is_none();
    let false_errors: Vec<(String, Option<String>)> = outcomes()
        .iter()
        .filter(ok)
        .filter(|o| o.verdict == Verdict::Accepted && o.first_real_error.is_some())
        .map(|o| (o.name.clone(), o.first_real_error.clone()))
        .collect();
    let unsupported_rejections: Vec<(String, Option<String>)> = outcomes()
        .iter()
        .filter(ok)
        .filter(|o| o.verdict == Verdict::Rejected && o.errors && o.first_real_error.is_none())
        .map(|o| (o.name.clone(), None))
        .collect();
    let parse_gaps: Vec<(String, Option<String>)> = outcomes()
        .iter()
        .filter(ok)
        .filter(|o| o.verdict == Verdict::Accepted || o.name.starts_with("qb64pe-source/"))
        .filter(|o| o.parse_gap.is_some())
        .map(|o| (o.name.clone(), o.parse_gap.clone()))
        .collect();
    let failures: Vec<String> = [
        FALSE_ERRORS.check(&false_errors, have_clone),
        UNSUPPORTED_REJECTIONS.check(&unsupported_rejections, have_clone),
        PARSE_GAPS.check(&parse_gaps, have_clone),
    ]
    .into_iter()
    .flatten()
    .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// Entries of `tests\upstream\<file>` (`category/name`, `#` comments); none if the file does not exist.
fn upstream_list(file: &str) -> Vec<String> {
    #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
    let text = std::fs::read_to_string(repo().join("tests/upstream").join(file)).unwrap_or_default();
    text.lines()
        .map(|l| l.split('#').next().unwrap().trim())
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

/// "Upstream progress" (spec `testing/upstream-tests`, design D7): `pass.list` and `deferred.list` name existing
/// upstream programs, no program is in both, and progress is the pass list out of the programs not deferred.
#[test]
fn upstream_progress() {
    let root = repo().join("tests/upstream/compile_tests");
    let deferred = upstream_list("deferred.list");
    let pass = upstream_list("pass.list");
    let mut problems = Vec::new();
    for (list, entries) in [("deferred.list", &deferred), ("pass.list", &pass)] {
        for e in entries {
            if !root.join(format!("{e}.bas")).is_file() {
                problems.push(format!("{list}: no upstream program {e}.bas"));
            }
        }
    }
    for e in pass.iter().filter(|e| deferred.contains(e)) {
        problems.push(format!("in both deferred.list and pass.list: {e}"));
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
    let programs = files(&root, &["bas"]).len();
    let in_reach = programs - deferred.len();
    eprintln!(
        "upstream progress: {} of {in_reach} ({programs} programs, {} deferred)",
        pass.len(),
        deferred.len()
    );
    assert!(pass.len() <= in_reach);
}

/// Programs that compile cleanly but are known not to pass tier 2, each with its reason (`study\26` §3).
const CLEAN_NOT_PASSING: &str = "known_clean_not_passing.list";

/// Entries of `tests\differential\pass.list` (`group/name`, `#` comments) as input names (`differential/<group>/<name>.bas`).
fn differential_pass_list() -> Vec<String> {
    #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
    let text =
        std::fs::read_to_string(repo().join("tests/differential/pass.list")).expect("tests/differential/pass.list");
    text.lines()
        .map(|l| l.split('#').next().unwrap().trim())
        .filter(|l| !l.is_empty())
        .map(|e| format!("differential/{e}.bas"))
        .collect()
}

/// "Clean programs are listed" (spec `testing/compiler-tests`, `study\26` §3): every corpus, upstream or
/// differential program the old compiler accepts and the new one compiles without an error is in `slice.list`, a
/// `pass.list` or `known_clean_not_passing.list`, so "never wrong code" covers every program that gets an
/// executable. The last list is shrink-only and hand-kept: an entry that no longer compiles cleanly, or that a pass
/// list names, must go.
#[test]
fn clean_programs_are_listed() {
    let have_clone = clone_root().is_some();
    let checked = |e: &str| have_clone || !CLONE_SETS.iter().any(|s| e.starts_with(s));
    #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
    let slice = std::fs::read_to_string(repo().join("tests/corpus/slice.list")).unwrap();
    let passing: Vec<String> = slice
        .lines()
        .map(|l| l.split('#').next().unwrap().trim())
        .filter(|l| !l.is_empty())
        .map(|e| format!("corpus/{e}.bas"))
        .chain(upstream_list("pass.list").iter().map(|e| format!("upstream/{e}.bas")))
        .chain(differential_pass_list())
        .collect();
    let known = List {
        file: CLEAN_NOT_PASSING,
        header: "",
    }
    .read()
    .1;
    let clean: Vec<&str> = outcomes()
        .iter()
        .filter(|o| o.panic.is_none() && o.verdict == Verdict::Accepted && !o.errors)
        .map(|o| o.name.as_str())
        .filter(|n| {
            ["corpus/", "upstream/", "differential/"]
                .iter()
                .any(|s| n.starts_with(s))
        })
        .collect();
    eprintln!("clean corpus, upstream and differential programs: {}", clean.len());
    let mut problems = Vec::new();
    for n in clean.iter().filter(|n| !passing.iter().chain(&known).any(|e| e == *n)) {
        problems.push(format!(
            "compiles cleanly but is on no list (run it in tier 2, then add it to a pass list or to tests/{CLEAN_NOT_PASSING} with the reason): {n}"
        ));
    }
    for e in known.iter().filter(|e| checked(e)) {
        if !clean.contains(&e.as_str()) {
            problems.push(format!(
                "tests/{CLEAN_NOT_PASSING}: no longer compiles cleanly, remove it: {e}"
            ));
        } else if passing.contains(e) {
            problems.push(format!("tests/{CLEAN_NOT_PASSING}: on a pass list too, remove it: {e}"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The BASIC code of a source, without comments and string literals (a `'` outside a string starts a comment; a
/// line starting with `REM` is one), so names in them do not count as calls.
fn code_only(source: &[u8]) -> String {
    let mut out = String::new();
    for line in source.split(|&b| b == b'\n') {
        let trimmed = line.trim_ascii_start();
        if trimmed.len() >= 3 && trimmed[..3].eq_ignore_ascii_case(b"REM") {
            out.push('\n');
            continue;
        }
        let mut in_string = false;
        for &b in line {
            match (b, in_string) {
                (b'"', _) => in_string = !in_string,
                (b'\'', false) => break,
                (_, true) => {}
                (_, false) => out.push(char::from(b.to_ascii_uppercase())),
            }
        }
        out.push('\n');
    }
    out
}

/// Whether `code` (from [`code_only`]) uses `name` (upper case, with its suffix) as a word: not part of a longer
/// name, and for a name without suffix not followed by one (`ERR` does not match `ERROR` or `ERR%`).
fn uses_word(code: &str, name: &str) -> bool {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.';
    let suffix = |c: char| matches!(c, '$' | '%' | '&' | '!' | '#' | '~' | '`');
    code.match_indices(name).any(|(at, _)| {
        let before = code[..at].chars().next_back();
        let after = code[at + name.len()..].chars().next();
        !before.is_some_and(ident) && !after.is_some_and(|c| ident(c) || (!name.ends_with('$') && suffix(c)))
    })
}

/// "Built-in coverage" (spec `testing/compiler-tests`, design D9 of `m2-core-builtins`): every built-in function
/// `sema` compiles is called in a program of `slice.list` (its output is compared with the old compiler's in tier 2)
/// and in a `typed` front-end test (its result type and argument conversions are pinned in tier 1).
#[test]
fn every_supported_builtin_is_covered() {
    let corpus = repo().join("tests/corpus");
    #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
    let list = std::fs::read_to_string(corpus.join("slice.list")).unwrap();
    let slice: Vec<String> = list
        .lines()
        .map(|l| l.split('#').next().unwrap().trim())
        .filter(|l| !l.is_empty())
        .map(|l| code_only(&std::fs::read(corpus.join(format!("{l}.bas"))).unwrap()))
        .collect();
    let typed: Vec<String> = files(&repo().join("tests/frontend"), &["bas"])
        .iter()
        .map(|p| std::fs::read(p).unwrap())
        .filter(|b| b.starts_with(b"' TEST: typed"))
        .map(|b| code_only(&b))
        .collect();
    assert!(!slice.is_empty() && !typed.is_empty());
    let mut missing = Vec::new();
    for s in qb64rust_sema::builtins::supported() {
        if !slice.iter().any(|c| uses_word(c, s.name)) {
            missing.push(format!(
                "`{}` is called in no program of tests/corpus/slice.list",
                s.name
            ));
        }
        if !typed.iter().any(|c| uses_word(c, s.name)) {
            missing.push(format!("`{}` is called in no `typed` test of tests/frontend", s.name));
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
}

#[test]
fn coverage_words() {
    let code = code_only(b"PRINT err; Chr$(1) ' LEN(x)\nx$ = \"INSTR(\"\nREM LEFT$(a, 1)\nERROR 5: e = errx\n");
    assert!(uses_word(&code, "ERR"));
    assert!(uses_word(&code, "CHR$"));
    assert!(!uses_word(&code, "LEN"), "in a comment");
    assert!(!uses_word(&code, "INSTR"), "in a string");
    assert!(!uses_word(&code, "LEFT$"), "in a REM line");
    assert!(!uses_word(&code_only(b"ERROR 1: e = errx: f = err%\n"), "ERR"));
}

/// Extensions `tools\upstream\copy_upstream_tests.py` copies (design D1).
const COPIED: &[&str] = &[
    "bas",
    "bi",
    "bm",
    "h",
    "c",
    "output",
    "err",
    "license",
    "noprompt",
    "compile-from-base",
    "md",
];

/// The copy in `tests\upstream\compile_tests` equals the clone's suite: same files, same bytes. A difference means
/// the clone was updated: rerun the copy script with `--commit-ok`.
#[test]
fn upstream_copy_matches_the_clone() {
    let Some(clone) = clone_root() else {
        eprintln!("skipped: no QB64pe clone (upstream copy not compared)");
        return;
    };
    let theirs_root = clone.join("tests/compile_tests");
    let ours_root = repo().join("tests/upstream/compile_tests");
    let rel = |root: &Path, p: &Path| p.strip_prefix(root).unwrap().to_path_buf();
    let theirs: Vec<PathBuf> = files(&theirs_root, COPIED)
        .iter()
        .map(|p| rel(&theirs_root, p))
        .collect();
    let ours: Vec<PathBuf> = files(&ours_root, COPIED).iter().map(|p| rel(&ours_root, p)).collect();
    let mut diff = Vec::new();
    for p in &theirs {
        if !ours.contains(p) {
            diff.push(format!("missing from the copy: {}", p.display()));
        } else if std::fs::read(theirs_root.join(p)).unwrap() != std::fs::read(ours_root.join(p)).unwrap() {
            diff.push(format!("differs from the clone: {}", p.display()));
        }
    }
    for p in &ours {
        if !theirs.contains(p) {
            diff.push(format!("not in the clone: {}", p.display()));
        }
    }
    assert!(
        diff.is_empty(),
        "{}\nrerun tools/upstream/copy_upstream_tests.py --commit-ok",
        diff.join("\n")
    );
}
