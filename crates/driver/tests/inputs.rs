//! Tier 1 (`study\19`, specs `testing/compiler-tests` and `testing/upstream-tests`): the front end over every
//! whole-program input set, without C++ (design D2 of `m2-upstream-tests`).
//!
//! | Prefix | Root | Verdict |
//! |---|---|---|
//! | `corpus/` | `tests\corpus` | rejected with an `.err` file, else accepted |
//! | `upstream/` | `tests\upstream\compile_tests` | `.bas` as `corpus/`; `.bi`/`.bm` include only |
//! | `snippets/` | `tests\snippets` | as `corpus/` |
//! | `qbasic/` | clone `tests\qbasic_testcases` | `.bas` accepted; `.bi`/`.bm` include only |
//! | `qb64pe-source/` | clone `source`, `internal\support` | `source\qb64pe.bas` accepted; the rest include only |
//!
//! The clone sets are read from the QB64pe reference clone (found as the driver finds it) and skipped, with a
//! note, when it is missing. Every file gets the no-panic and round-trip check; include-only files nothing else.

use qb64rust_driver::{build, frontend};
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

fn run(name: String, path: &Path, verdict: Verdict) -> Outcome {
    let bytes = std::fs::read(path).unwrap();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let fe = frontend(&name, bytes.clone());
        let round_trip = print(&fe.parsed.main().green, &bytes) == bytes;
        let mut list: Vec<_> = fe.diagnostics.list().to_vec();
        if fe.diagnostics.is_capped() {
            list.pop(); // "too many errors; stopping" is not an error of its own
        }
        list.sort_by_key(|d| d.span.start);
        let first_real_error = list.iter().find(|d| d.is_real_error()).map(|d| {
            let (line, col) = fe.map.file(d.span.file).line_col(d.span.start);
            format!("{line}:{col}: {}", d.message)
        });
        let file = fe.map.file(fe.file);
        let parse_gap = fe
            .parsed
            .main()
            .diagnostics
            .list()
            .iter()
            .min_by_key(|d| d.span.start)
            .map(|d| {
                let (line, col) = file.line_col(d.span.start);
                let mark = if d.unsupported { "not supported yet: " } else { "" };
                format!("{line}:{col}: {mark}{}", d.message)
            })
            .or_else(|| {
                first_error_node(fe.parsed.main().root()).map(|n| {
                    let (line, col) = file.line_col(n.span().start);
                    format!("{line}:{col}: `Error` node without a parser diagnostic")
                })
            });
        (round_trip, fe.has_errors(), first_real_error, parse_gap)
    }));
    match result {
        Ok((round_trip, errors, first_real_error, parse_gap)) => Outcome {
            name,
            verdict,
            panic: None,
            round_trip,
            errors,
            first_real_error,
            parse_gap,
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
        let fe = frontend(&name, std::fs::read(root.join(&name)).unwrap());
        assert!(fe.diagnostics.list().is_empty(), "{line}:\n{}", fe.render_diagnostics());
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

/// Sets read from the clone: their list entries are checked only when the clone is present.
const CLONE_SETS: &[&str] = &["qbasic/", "qb64pe-source/"];

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
    let new = want.iter().copied().filter(|e| !have.iter().any(|h| h == e)).collect();
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
    let gap = |src: &[u8]| run("t.bas".into(), &write_temp(src), Verdict::Accepted).parse_gap;
    assert_eq!(gap(b"PRINT 1\n"), None);
    assert!(
        gap(b"PRINT (1\n").is_some_and(|g| g.starts_with("1:")),
        "{:?}",
        gap(b"PRINT (1\n")
    );
}

/// A file in the test binary's temp folder holding `bytes`.
fn write_temp(bytes: &[u8]) -> PathBuf {
    use std::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!("qb64rust-inputs-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join(format!("{}.bas", N.fetch_add(1, Ordering::Relaxed)));
    std::fs::write(&p, bytes).unwrap();
    p
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
