//! Tier 1 check of the format oracle (spec `testing/format-oracle`, "Tier 1 checks"; design D6 of `m5b-formatter`):
//! every recording under `tests\format\` against the formatter, in process, with QB64pe's default layout settings,
//! and each recording under `tests\format\settings\<variant>\` with the defaults and that variant's setting
//! (`m5b-format-settings`, design D1: the folder name is the `-f:` setting, read by `Settings::set`). The programs
//! under each variant must be those `tests\format\settings.list` names.
//!
//! For each recorded program:
//! - the source checks without errors and is formatted; the text must equal the recording with line endings taken
//!   as LF, unless `tests\format\known_differences.list` names the program (shrink-only: a listed program that
//!   comes to equal its recording fails until it is taken off), or the recording's `<name>.divergences` names a row
//!   of `QB64PE-DIVERGENCES.md` (design D8);
//! - the formatted text is the same program: its typed dump equals the source's (the dump names no source position
//!   and writes names in capitals, and formatting keeps every line where it is);
//! - formatting the formatted text again changes nothing.
//!
//! The sources are loaded as `inputs.rs` loads them: includes from the program's folder (a corpus program's from its
//! `<name>.files`), the compiler root `tests\upstream\root`; the upstream programs of QB64pe's own code
//! (`upstream/qb64pe/`) from the clone's copy of their folder, and skipped with a note when the clone is missing.
//!
//! The test prints the statement forms of the parser (node kinds, and built-in statements by name) that no recorded
//! program reaches, for the close of the change (design D6).
//!
//! Time: measured at the close of each group of `m5b-formatter` and of `m5b-format-settings` (debug build, all
//! cores), in the lines below.
//! 2026-10-25, layout not built yet (every line written as read): 0.84 s for 1,180 programs.
//! 2026-10-25, layout built (group 4): 1.0 s for 1,180 programs, 1,179 equal to their recording.
//! 2026-10-25, with the other settings (`m5b-format-settings` group 3): 1.5 s for 1,345 programs, 162 of them
//! under the six variants; all but one of the 1,345 equal to their recording or covered by a divergence.

use qb64rust_base::layout::Settings;
use qb64rust_base::show_bytes;
use qb64rust_driver::{FileLoader, Frontend, build, format_program, frontend_with};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::tree::{Element, Node};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The set of the oracle and the folder of its sources.
const SETS: [(&str, &str); 4] = [
    ("corpus", "tests/corpus"),
    ("upstream", "tests/upstream/compile_tests"),
    ("snippets", "tests/snippets"),
    ("programs", "tests/programs"),
];

/// The upstream programs that include QB64pe's own sources relative to their place in the clone: their path within
/// the `upstream` set starts with this.
const UPSTREAM_QB64PE: &str = "qb64pe/";

/// The folder of the recordings under the other settings: `settings/<variant>/<set>/<path>`, the variant named by
/// its `-f:` setting (`autoindentsize=2`).
const SETTINGS: &str = "settings";

/// One recorded program.
struct Program {
    /// `<set>/<path within the set without .bas>`, with `/`; under another setting
    /// `settings/<variant>/<set>/<path without .bas>`.
    name: String,
    source: PathBuf,
    recording: PathBuf,
    /// Where its includes are looked up from (as the program standing in that folder).
    base: PathBuf,
    /// The defaults, or the defaults with the variant's setting.
    settings: Settings,
    /// It includes QB64pe's own sources, so it checks only with the reference clone.
    needs_clone: bool,
}

/// The recordings of each set under `dir` (the defaults' folder, or a variant's), named with `prefix`.
fn recordings_in(dir: &Path, prefix: &str, settings: Settings, out: &mut Vec<Program>) {
    for (set, folder) in SETS {
        let mut found = Vec::new();
        walk(&dir.join(set), &mut found);
        found.sort();
        for rec in found {
            let rel = rec.strip_prefix(dir.join(set)).unwrap().to_path_buf();
            let rel_name = rel.with_extension("").to_string_lossy().replace('\\', "/");
            let name = format!("{prefix}{set}/{rel_name}");
            let source = repo().join(folder).join(&rel);
            let needs_clone = set == "upstream" && rel_name.starts_with(UPSTREAM_QB64PE);
            let base = match build::clone_root() {
                Some(clone) if needs_clone => clone.join("tests/compile_tests").join(&rel),
                _ => beside_files(&source),
            };
            out.push(Program {
                name,
                source,
                recording: rec,
                base,
                settings,
                needs_clone,
            });
        }
    }
}

/// The variants recorded: each folder under `settings/`, with its settings.
fn variants() -> Vec<(String, Settings)> {
    let Ok(entries) = std::fs::read_dir(repo().join("tests/format").join(SETTINGS)) else {
        return Vec::new();
    };
    let mut out: Vec<_> = entries
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .map(|p| {
            let variant = p.file_name().unwrap().to_string_lossy().into_owned();
            let (name, value) = variant
                .split_once('=')
                .unwrap_or_else(|| panic!("{SETTINGS}/{variant}: not a <setting>=<value> folder"));
            let mut settings = Settings::default();
            match settings.set(name, value) {
                Ok(true) => {}
                Ok(false) => panic!("{SETTINGS}/{variant}: not a layout setting"),
                Err(e) => panic!("{SETTINGS}/{variant}: {e}"),
            }
            (variant, settings)
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn recordings() -> Vec<Program> {
    let format = repo().join("tests/format");
    let mut out = Vec::new();
    recordings_in(&format, "", Settings::default(), &mut out);
    for (variant, settings) in variants() {
        recordings_in(
            &format.join(SETTINGS).join(&variant),
            &format!("{SETTINGS}/{variant}/"),
            settings,
            &mut out,
        );
    }
    out
}

/// What `settings.list` names and a variant's folder lacks or holds besides, as messages.
fn settings_list_mismatches(programs: &[Program]) -> Vec<String> {
    let listed: BTreeSet<String> = list_lines(&repo().join("tests/format/settings.list"))
        .into_iter()
        .map(|l| l.trim().trim_end_matches(".bas").trim_end_matches(".BAS").to_string())
        .collect();
    let mut problems = Vec::new();
    for (variant, _) in variants() {
        let prefix = format!("{SETTINGS}/{variant}/");
        let recorded: BTreeSet<String> = programs
            .iter()
            .filter_map(|p| p.name.strip_prefix(&prefix).map(str::to_string))
            .collect();
        for missing in listed.difference(&recorded) {
            problems.push(format!("{prefix}{missing}: listed in settings.list, not recorded"));
        }
        for extra in recorded.difference(&listed) {
            problems.push(format!("{prefix}{extra}: recorded, not in settings.list"));
        }
    }
    problems
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bas")) {
            out.push(p);
        }
    }
}

/// A corpus program's included files lie in its `<name>.files` folder (`inputs.rs` `beside_files`).
fn beside_files(path: &Path) -> PathBuf {
    let files = path.with_extension("files");
    match path.file_name() {
        Some(name) if files.is_dir() => files.join(name),
        _ => path.to_path_buf(),
    }
}

fn load(p: &Program, bytes: Vec<u8>) -> Frontend {
    let root = repo().join("tests/upstream/root");
    let display = p.source.to_string_lossy().into_owned();
    frontend_with(&display, bytes, |file| Box::new(FileLoader::new(&root, file, &p.base)))
}

/// The text with each CR LF as LF, byte for byte otherwise.
fn lf(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    for (i, &c) in bytes.iter().enumerate() {
        if !(c == b'\r' && bytes.get(i + 1) == Some(&b'\n')) {
            out.push(c);
        }
    }
    out
}

/// The first line where two texts differ, for a message.
fn first_difference(want: &[u8], got: &[u8]) -> String {
    let (want, got) = (lf(want), lf(got));
    let (a, b): (Vec<_>, Vec<_>) = (
        want.split(|&c| c == b'\n').collect(),
        got.split(|&c| c == b'\n').collect(),
    );
    for (n, (x, y)) in a.iter().zip(&b).enumerate() {
        if x != y {
            return format!("line {}: recorded {:?}, got {:?}", n + 1, show_bytes(x), show_bytes(y));
        }
    }
    format!("line {}: one text goes on", a.len().min(b.len()) + 1)
}

/// The lines of a list file without its comments ('#' to the end of the line) and blank lines, each as text (the
/// lists are ASCII).
fn list_lines(path: &Path) -> Vec<String> {
    let bytes = std::fs::read(path).unwrap_or_default();
    bytes
        .split(|&c| c == b'\n')
        .map(|l| l.split(|&c| c == b'#').next().unwrap_or_default())
        .map(|l| {
            l.iter()
                .map(|&b| char::from(b))
                .collect::<String>()
                .trim_end_matches('\r')
                .to_string()
        })
        .filter(|l| !l.trim().is_empty())
        .collect()
}

fn first_error(fe: &Frontend) -> String {
    let d = fe.diagnostics.errors().next().expect("an error");
    let (line, col) = fe.map.file(d.span.file).line_col(d.span.start);
    format!("{line}:{col}: {}", d.message)
}

/// The rows a `.divergences` sidecar names ('#' starts a comment).
fn divergence_rows(path: &Path) -> Vec<String> {
    list_lines(path).into_iter().map(|l| l.trim().to_string()).collect()
}

/// `known_differences.list`: name -> reason.
fn known_differences() -> BTreeMap<String, String> {
    list_lines(&repo().join("tests/format/known_differences.list"))
        .into_iter()
        .map(|l| {
            let (name, reason) = l.split_once('\t').unwrap_or((&l, ""));
            (name.trim().to_string(), reason.trim().to_string())
        })
        .collect()
}

/// What one program gave.
#[derive(Default)]
struct Outcome {
    name: String,
    /// The output differs from the recording (and no sidecar allows it): the first line that differs.
    differs: Option<String>,
    /// A check that must never fail: the source does not check, the program changed, not idempotent.
    broken: Option<String>,
    skipped: Option<String>,
    /// The statement forms of the source's main file.
    forms: BTreeSet<String>,
    /// Every line that differs from the recording, for `QB64RUST_FORMAT_DIFFS`.
    lines: Vec<String>,
}

/// Each line where two texts differ: `<line>\t<recorded>\t<got>`.
fn differing_lines(want: &[u8], got: &[u8]) -> Vec<String> {
    let (want, got) = (lf(want), lf(got));
    let a: Vec<_> = want.split(|&c| c == b'\n').collect();
    let b: Vec<_> = got.split(|&c| c == b'\n').collect();
    a.iter()
        .zip(&b)
        .enumerate()
        .filter(|(_, (x, y))| x != y)
        .map(|(n, (x, y))| format!("{}\t{}\t{}", n + 1, show_bytes(x), show_bytes(y)))
        .collect()
}

fn run(p: &Program) -> Outcome {
    let mut o = Outcome {
        name: p.name.clone(),
        ..Outcome::default()
    };
    let bytes = std::fs::read(&p.source).unwrap_or_else(|e| panic!("{}: {e}", p.source.display()));
    let fe = load(p, bytes);
    if fe.has_errors() {
        if p.needs_clone && build::clone_root().is_none() {
            o.skipped = Some("needs the QB64pe reference clone".into());
        } else if fe.diagnostics.errors().all(|d| !d.is_real_error()) {
            // Only "not supported yet": the program cannot be formatted until the compiler takes it.
            o.differs = Some(format!("not formatted: {}", first_error(&fe)));
        } else {
            o.broken = Some(format!("the source does not check: {}", first_error(&fe)));
        }
        return o;
    }
    collect_forms(fe.parsed.trees[0].root(), &fe.map.file(fe.file).bytes, &mut o.forms);
    let settings = p.settings;
    let out = format_program(&fe, &settings);
    let recorded = std::fs::read(&p.recording).unwrap();
    if lf(&out) != lf(&recorded) && divergence_rows(&p.recording.with_extension("divergences")).is_empty() {
        o.differs = Some(first_difference(&recorded, &out));
        o.lines = differing_lines(&recorded, &out);
    }
    let again = load(p, out.clone());
    if again.has_errors() {
        o.broken = Some(format!("the formatted text does not check: {}", first_error(&again)));
        return o;
    }
    let (before, after) = (
        qb64rust_sema::dump_typed(&fe.program),
        qb64rust_sema::dump_typed(&again.program),
    );
    if before != after {
        let line = before.lines().zip(after.lines()).find(|(a, b)| a != b).map_or_else(
            || "one dump goes on".to_string(),
            |(a, b)| format!("{a:?} became {b:?}"),
        );
        o.broken = Some(format!("the formatted text is another program: {line}"));
        return o;
    }
    let twice = format_program(&again, &settings);
    if twice != out {
        o.broken = Some(format!(
            "formatting again changes it: {}",
            first_difference(&out, &twice)
        ));
    }
    o
}

/// The statement kinds of the parser: a node kind that stands for a statement, a block or a line's label.
const STATEMENT_KINDS: &[SyntaxKind] = &[
    SyntaxKind::MetaStmt,
    SyntaxKind::InactiveCode,
    SyntaxKind::MetaCommentStmt,
    SyntaxKind::PrintStmt,
    SyntaxKind::LprintStmt,
    SyntaxKind::WriteStmt,
    SyntaxKind::InputStmt,
    SyntaxKind::LineInputStmt,
    SyntaxKind::CloseStmt,
    SyntaxKind::FieldStmt,
    SyntaxKind::LsetStmt,
    SyntaxKind::SwapStmt,
    SyntaxKind::MemStmt,
    SyntaxKind::ArrayCopyStmt,
    SyntaxKind::BuiltinStmt,
    SyntaxKind::DimStmt,
    SyntaxKind::AssignStmt,
    SyntaxKind::EndStmt,
    SyntaxKind::SystemStmt,
    SyntaxKind::ProcDef,
    SyntaxKind::CallStmt,
    SyntaxKind::OnJumpStmt,
    SyntaxKind::OnEventStmt,
    SyntaxKind::KeyStmt,
    SyntaxKind::StopStmt,
    SyntaxKind::RunStmt,
    SyntaxKind::PaletteUsingStmt,
    SyntaxKind::ExitStmt,
    SyntaxKind::ContinueStmt,
    SyntaxKind::DeclareStmt,
    SyntaxKind::SharedStmt,
    SyntaxKind::StaticStmt,
    SyntaxKind::LabelDef,
    SyntaxKind::OnErrorStmt,
    SyntaxKind::ResumeStmt,
    SyntaxKind::ErrorStmt,
    SyntaxKind::LineNumber,
    SyntaxKind::GotoStmt,
    SyntaxKind::GosubStmt,
    SyntaxKind::ReturnStmt,
    SyntaxKind::DataStmt,
    SyntaxKind::ReadStmt,
    SyntaxKind::RestoreStmt,
    SyntaxKind::ConstStmt,
    SyntaxKind::OptionStmt,
    SyntaxKind::RedimStmt,
    SyntaxKind::CommonStmt,
    SyntaxKind::EraseStmt,
    SyntaxKind::DefTypeStmt,
    SyntaxKind::CrossCloser,
    SyntaxKind::IfBlock,
    SyntaxKind::IfStmt,
    SyntaxKind::ForBlock,
    SyntaxKind::DoBlock,
    SyntaxKind::WhileBlock,
    SyntaxKind::SelectBlock,
    SyntaxKind::TypeBlock,
    SyntaxKind::DeclareLibraryBlock,
    SyntaxKind::DefFnBlock,
    SyntaxKind::DefFnStmt,
];

/// A built-in statement by its name: the first word of a `BuiltinStmt`, or the name a `CallStmt` without `CALL`
/// gives when it is a built-in SUB.
fn builtin_name(n: Node<'_>, source: &[u8]) -> Option<String> {
    let first = n.first_token()?;
    let word = &source[first.span.start as usize..first.span.end as usize];
    let upper = show_bytes(word).to_ascii_uppercase();
    let bare = upper.trim_end_matches(['$', '%', '&', '!', '#', '`', '~']);
    qb64rust_builtins::find_any(bare.as_bytes())
        .any(|b| b.kind == qb64rust_builtins::Kind::Sub)
        .then(|| format!("statement {upper}"))
}

fn collect_forms(n: Node<'_>, source: &[u8], out: &mut BTreeSet<String>) {
    let kind = n.kind();
    if STATEMENT_KINDS.contains(&kind) {
        match kind {
            SyntaxKind::BuiltinStmt | SyntaxKind::CallStmt => {
                out.insert(builtin_name(n, source).unwrap_or_else(|| format!("{kind:?}")));
            }
            _ => {
                out.insert(format!("{kind:?}"));
            }
        }
    }
    for c in n.children() {
        if let Element::Node(child) = c {
            collect_forms(child, source, out);
        }
    }
}

/// The built-in SUBs whose statements the parser reads into a node kind of their own (`PrintStmt`, `CloseStmt`), or
/// as an assignment (`MID$(a$, 2) = …`, `ASC(a$, 1) = …`): counted by that kind, not by name.
const OWN_KIND: &[&str] = &[
    "PRINT",
    "LPRINT",
    "WRITE",
    "INPUT",
    "CLOSE",
    "LSET",
    "RSET",
    "SWAP",
    "END",
    "SYSTEM",
    "ERROR",
    "READ",
    "OPTION",
    "_MEMPUT",
    "_MEMFILL",
    "_CONTINUE",
    "MID$",
    "ASC",
];

/// Every form `collect_forms` can give: the statement kinds, and each built-in SUB by its name (with the suffix it
/// must have).
fn all_forms() -> BTreeSet<String> {
    let mut all: BTreeSet<String> = STATEMENT_KINDS
        .iter()
        .filter(|k| !matches!(k, SyntaxKind::BuiltinStmt))
        .map(|k| format!("{k:?}"))
        .collect();
    for b in qb64rust_builtins::BUILTINS {
        let name = format!("{}{}", b.name.to_ascii_uppercase(), b.musthave.unwrap_or(""));
        if b.kind == qb64rust_builtins::Kind::Sub && !OWN_KIND.contains(&name.as_str()) {
            all.insert(format!(
                "statement {}{}",
                b.name.to_ascii_uppercase(),
                b.musthave.unwrap_or("")
            ));
        }
    }
    all
}

#[test]
fn formatter_equals_the_old_compilers_layout() {
    let programs = recordings();
    assert!(programs.len() > 1000, "found only {} recordings", programs.len());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let next = AtomicUsize::new(0);
    let done = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..threads {
            std::thread::Builder::new()
                .stack_size(qb64rust_driver::STACK_SIZE)
                .spawn_scoped(s, || {
                    while let Some(p) = programs.get(next.fetch_add(1, Ordering::Relaxed)) {
                        let o = run(p);
                        done.lock().unwrap().push(o);
                    }
                })
                .unwrap();
        }
    });
    let mut outcomes = done.into_inner().unwrap();
    outcomes.sort_by(|a, b| a.name.cmp(&b.name));

    // For work on the formatter: every differing line of every program, to a file.
    if let Some(path) = std::env::var_os("QB64RUST_FORMAT_DIFFS") {
        let text: String = outcomes
            .iter()
            .flat_map(|o| o.lines.iter().map(move |l| format!("{}:{l}\n", o.name)))
            .collect();
        std::fs::write(path, text).unwrap();
    }
    let known = known_differences();
    let mut failures = Vec::new();
    let mut new_differences = Vec::new();
    let mut now_equal = Vec::new();
    let mut reached = BTreeSet::new();
    for o in &outcomes {
        reached.extend(o.forms.iter().cloned());
        if let Some(why) = &o.skipped {
            eprintln!("skipped {}: {why}", o.name);
            continue;
        }
        if let Some(why) = &o.broken {
            failures.push(format!("{}: {why}", o.name));
        }
        match (&o.differs, known.contains_key(&o.name)) {
            (Some(at), false) => new_differences.push(format!("{}\t{at}", o.name)),
            (None, true) if o.broken.is_none() => now_equal.push(o.name.clone()),
            _ => {}
        }
    }
    let names: BTreeSet<_> = outcomes.iter().map(|o| o.name.as_str()).collect();
    let stale: Vec<_> = known.keys().filter(|k| !names.contains(k.as_str())).collect();
    let mismatches = settings_list_mismatches(&programs);

    let unreached: Vec<_> = all_forms().into_iter().filter(|f| !reached.contains(f)).collect();
    eprintln!(
        "statement forms no recorded program reaches ({} of {}): {}",
        unreached.len(),
        all_forms().len(),
        unreached.join(", ")
    );
    let differing = outcomes.iter().filter(|o| o.differs.is_some()).count();
    let under_variants = outcomes.iter().filter(|o| o.name.starts_with(SETTINGS)).count();
    eprintln!(
        "{} programs ({} of them under the other settings), {} equal their recording, {} differ ({} listed in \
         known_differences.list)",
        outcomes.len(),
        under_variants,
        outcomes.len() - differing - outcomes.iter().filter(|o| o.skipped.is_some()).count(),
        differing,
        known.len()
    );

    let mut message = String::new();
    if !failures.is_empty() {
        message += &format!(
            "\nformatting broke {} programs:\n  {}",
            failures.len(),
            failures.join("\n  ")
        );
    }
    if !new_differences.is_empty() {
        message += &format!(
            "\n{} programs differ from their recording and are not in known_differences.list:\n  {}",
            new_differences.len(),
            new_differences.join("\n  ")
        );
    }
    if !now_equal.is_empty() {
        message += &format!(
            "\n{} programs in known_differences.list now equal their recording; take them off:\n  {}",
            now_equal.len(),
            now_equal.join("\n  ")
        );
    }
    if !stale.is_empty() {
        message += &format!("\nknown_differences.list names programs with no recording: {stale:?}");
    }
    if !mismatches.is_empty() {
        message += &format!(
            "\nthe recordings under the other settings do not match settings.list (record again with qb64pe):\n  {}",
            mismatches.join("\n  ")
        );
    }
    assert!(message.is_empty(), "{message}");
}
