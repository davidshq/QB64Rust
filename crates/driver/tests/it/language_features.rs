//! The language server's features over the driver's semantic index (OpenSpec change `m5b-language-features`, spec
//! `editor/language-server`): the index itself, then the index job, hover, references, rename and completion driven
//! in-process.

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use qb64rust_base::{SourceMap, Span};
use qb64rust_driver::{check_program, frontend, hover, index_for_server};
use qb64rust_lsp::Hooks;
use qb64rust_lsp::analysis::{Analysis, OpenFile, analyze};
use qb64rust_lsp::definition;
use qb64rust_lsp::encoding::Encoding;
use qb64rust_lsp::index::{EntryKind, Indexed, SymbolIndex};
use qb64rust_lsp::rename;
use qb64rust_lsp::uri::from_path;
use qb64rust_syntax::ParsedProgram;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Long enough for the front end of a test program on a loaded machine.
const WAIT: Duration = Duration::from_secs(20);

/// The error code of an answer for a text that changed (LSP `ContentModified`).
const CONTENT_MODIFIED: i32 = -32801;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf()
}

struct Client {
    conn: Connection,
    server: JoinHandle<Result<(), String>>,
    next: i32,
    /// The notifications received while waiting for answers.
    notes: Vec<Notification>,
}

impl Client {
    fn start(root: &Path) -> Client {
        Client::start_with(root, qb64rust_driver::server_hooks())
    }

    fn start_with(root: &Path, hooks: Hooks) -> Client {
        let (server_side, conn) = Connection::memory();
        let root = root.to_path_buf();
        let server = std::thread::Builder::new()
            .stack_size(qb64rust_base::STACK_SIZE)
            .spawn(move || qb64rust_lsp::serve(server_side, root, hooks))
            .unwrap();
        let mut c = Client {
            conn,
            server,
            next: 0,
            notes: Vec::new(),
        };
        let init = json!({ "processId": null, "rootUri": null, "capabilities": {} });
        c.request("initialize", init);
        c.notify("initialized", json!({}));
        c
    }

    fn notify(&self, method: &str, params: Value) {
        let n = Notification::new(method.into(), params);
        self.conn.sender.send(n.into()).unwrap();
    }

    fn send(&mut self, method: &str, params: Value) -> RequestId {
        self.next += 1;
        let id = RequestId::from(self.next);
        self.conn
            .sender
            .send(Request::new(id.clone(), method.into(), params).into())
            .unwrap();
        id
    }

    fn answer(&mut self, id: &RequestId) -> Response {
        let deadline = Instant::now() + WAIT;
        loop {
            let msg = self
                .conn
                .receiver
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .expect("no answer in time");
            match msg {
                Message::Response(r) if r.id == *id => return r,
                Message::Notification(n) => self.notes.push(n),
                _ => {}
            }
        }
    }

    /// The params of the first notification of `method` received so far.
    fn note(&self, method: &str) -> Value {
        let n = self.notes.iter().find(|n| n.method == method);
        n.unwrap_or_else(|| panic!("no {method}")).params.clone()
    }

    fn request(&mut self, method: &str, params: Value) -> Response {
        let id = self.send(method, params);
        self.answer(&id)
    }

    fn open(&self, path: &Path, text: &str) {
        let doc = json!({ "uri": from_path(path), "languageId": "qb64rust", "version": 1, "text": text });
        self.notify("textDocument/didOpen", json!({ "textDocument": doc }));
    }

    fn change(&self, path: &Path, text: &str) {
        let doc = json!({ "uri": from_path(path), "version": 2 });
        self.notify(
            "textDocument/didChange",
            json!({ "textDocument": doc, "contentChanges": [{ "text": text }] }),
        );
    }

    /// The params of a request at the `nth` (from 0) occurrence of `needle` in `text`, `offset` bytes into it.
    fn at(path: &Path, text: &str, needle: &str, nth: usize, offset: usize) -> Value {
        let (line, character) = position(text, needle, nth, offset);
        json!({ "textDocument": { "uri": from_path(path) }, "position": { "line": line, "character": character } })
    }

    /// The (file name, 0-based line, column) of each location of a references answer, with the declaration or not.
    fn references(&mut self, path: &Path, text: &str, needle: &str, nth: usize, decl: bool) -> Vec<Place> {
        let mut params = Client::at(path, text, needle, nth, 0);
        params["context"] = json!({ "includeDeclaration": decl });
        let r = self.request("textDocument/references", params);
        let v = r.response_result.unwrap();
        v.as_array().unwrap().iter().map(place).collect()
    }

    /// The place of a definition answer; `None` for no definition.
    fn definition(&mut self, path: &Path, text: &str, needle: &str, nth: usize) -> Option<Place> {
        let r = self.request("textDocument/definition", Client::at(path, text, needle, nth, 0));
        let v = r.response_result.unwrap();
        (!v.is_null()).then(|| place(&v))
    }

    /// A rename's answer: per file name, its edits as (line, column, end column, new text) in order; or the error's
    /// code and message.
    fn rename(&mut self, path: &Path, text: &str, needle: &str, nth: usize, to: &str) -> Result<Edits, (i32, String)> {
        let mut params = Client::at(path, text, needle, nth, 0);
        params["newName"] = json!(to);
        let r = self.request("textDocument/rename", params);
        let v = r.response_result.map_err(|e| (e.code, e.message))?;
        let mut out: Edits = Vec::new();
        for (uri, edits) in v["changes"].as_object().unwrap() {
            let file = place(&json!({ "uri": uri, "range": { "start": { "line": 0, "character": 0 } } })).0;
            let mut list: Vec<(u64, u64, u64, String)> = edits
                .as_array()
                .unwrap()
                .iter()
                .map(|e| {
                    let (s, end) = (&e["range"]["start"], &e["range"]["end"]);
                    assert_eq!(s["line"], end["line"], "an edit on one line");
                    let n = |v: &Value| v.as_u64().unwrap();
                    let new = e["newText"].as_str().unwrap().to_string();
                    (n(&s["line"]), n(&s["character"]), n(&end["character"]), new)
                })
                .collect();
            list.sort();
            out.push((file, list));
        }
        out.sort();
        Ok(out)
    }

    /// The labels and kinds of a completion answer.
    fn complete(&mut self, path: &Path, text: &str, needle: &str, nth: usize, offset: usize) -> Vec<(String, u64)> {
        let r = self.request("textDocument/completion", Client::at(path, text, needle, nth, offset));
        let v = r.response_result.unwrap();
        assert_eq!(v["isIncomplete"], json!(false));
        let items = v["items"].as_array().unwrap();
        items
            .iter()
            .map(|i| (i["label"].as_str().unwrap().to_string(), i["kind"].as_u64().unwrap()))
            .collect()
    }

    /// The Markdown of a hover; `None` for no hover.
    fn hover(&mut self, path: &Path, text: &str, needle: &str, nth: usize) -> Option<String> {
        let r = self.request("textDocument/hover", Client::at(path, text, needle, nth, 0));
        let v = r.response_result.unwrap();
        (!v.is_null()).then(|| v["contents"]["value"].as_str().unwrap().to_string())
    }

    fn shutdown(mut self) {
        self.request("shutdown", Value::Null);
        self.notify("exit", Value::Null);
        self.server.join().unwrap().unwrap();
    }
}

/// A location as (file name, 0-based line, column).
type Place = (String, u64, u64);

/// A rename's edits: per file name, (line, column, end column, new text).
type Edits = Vec<(String, Vec<(u64, u64, u64, String)>)>;

/// `text` with one file's edits of a rename applied.
fn apply(text: &str, edits: &[(u64, u64, u64, String)]) -> String {
    let mut lines: Vec<String> = text.split('\n').map(str::to_string).collect();
    for (line, from, to, new) in edits.iter().rev() {
        let at = |n: u64| usize::try_from(n).unwrap();
        lines[at(*line)].replace_range(at(*from)..at(*to), new);
    }
    lines.join("\n")
}

fn place(loc: &Value) -> Place {
    let uri: lsp_types::Uri = loc["uri"].as_str().unwrap().parse().unwrap();
    let path = qb64rust_lsp::uri::to_path(&uri).unwrap();
    let start = &loc["range"]["start"];
    (
        path.file_name().unwrap().to_string_lossy().into_owned(),
        start["line"].as_u64().unwrap(),
        start["character"].as_u64().unwrap(),
    )
}

fn p(file: &str, line: u64, column: u64) -> Place {
    (file.to_string(), line, column)
}

/// The 0-based line and column of the `nth` occurrence of `needle` in `text` (ASCII), `offset` bytes into it.
fn position(text: &str, needle: &str, nth: usize, offset: usize) -> (usize, usize) {
    let at = text
        .match_indices(needle)
        .nth(nth)
        .unwrap_or_else(|| panic!("{needle} #{nth} not in the text"))
        .0
        + offset;
    let before = &text[..at];
    let line = before.matches('\n').count();
    let column = at - before.rfind('\n').map_or(0, |i| i + 1);
    (line, column)
}

/// The index of a few front-end programs, with every hover text, scope and body (design D1): variables of every
/// storage class, `SHARED`, labels of three bodies, user types with member arrays, constants.
#[test]
fn index_of_frontend_programs() {
    for name in [
        "proc_scopes.bas",
        "labels_bodies.bas",
        "member_arrays_typed.bas",
        "const_suffixed.bas",
    ] {
        let bytes = std::fs::read(repo().join("tests/frontend").join(name)).unwrap();
        let fe = frontend(name, bytes);
        let index = hover::symbol_index(&fe.map, &fe.program, &fe.diagnostics);
        insta::assert_snapshot!(format!("index_{}", name.trim_end_matches(".bas")), index.dump(&fe.map));
    }
}

static RUNS: AtomicUsize = AtomicUsize::new(0);

/// The driver's index, counting its runs.
fn counting_index(map: &SourceMap, parsed: &ParsedProgram, main: Option<&Path>, root: &Path) -> SymbolIndex {
    RUNS.fetch_add(1, Ordering::SeqCst);
    index_for_server(map, parsed, main, root)
}

/// "Two hovers, one run": the front end runs once for two requests on the same text, and again after an edit.
#[test]
fn two_hovers_one_run() {
    let dir = tempfile::tempdir().unwrap();
    let hooks = Hooks {
        index: counting_index,
        ..qb64rust_driver::server_hooks()
    };
    let mut c = Client::start_with(dir.path(), hooks);
    let p = dir.path().join("p.bas");
    let text = "x = 1\nPRINT x\n";
    c.open(&p, text);
    assert!(c.hover(&p, text, "x", 0).is_some());
    assert!(c.hover(&p, text, "x", 1).is_some());
    assert_eq!(RUNS.load(Ordering::SeqCst), 1);
    let text = "y = 1\nPRINT y\n";
    c.change(&p, text);
    assert!(c.hover(&p, text, "y", 1).is_some());
    assert_eq!(RUNS.load(Ordering::SeqCst), 2);
    c.shutdown();
}

/// The driver's index, half a second late for a program with `SLOW` in it.
fn slow_index(map: &SourceMap, parsed: &ParsedProgram, main: Option<&Path>, root: &Path) -> SymbolIndex {
    let bytes = &map.file(parsed.main().file).bytes;
    if bytes.windows(4).any(|w| w == b"SLOW") {
        std::thread::sleep(Duration::from_millis(500));
    }
    index_for_server(map, parsed, main, root)
}

/// "Edit while waiting": a request that waits for the index of a text that then changes is answered with
/// `ContentModified`; the next one gets the new text's index.
#[test]
fn edit_while_waiting() {
    let dir = tempfile::tempdir().unwrap();
    let hooks = Hooks {
        index: slow_index,
        ..qb64rust_driver::server_hooks()
    };
    let mut c = Client::start_with(dir.path(), hooks);
    let p = dir.path().join("p.bas");
    let text = "x = 1 ' SLOW\nPRINT x\n";
    c.open(&p, text);
    let id = c.send("textDocument/references", {
        let mut v = Client::at(&p, text, "x", 1, 0);
        v["context"] = json!({ "includeDeclaration": true });
        v
    });
    let hover = c.send("textDocument/hover", Client::at(&p, text, "x", 1, 0));
    c.change(&p, "x = 2\nPRINT x\n");
    for id in [id, hover] {
        let code = c.answer(&id).response_result.err().map(|e| e.code);
        assert_eq!(code, Some(CONTENT_MODIFIED));
    }
    let text = "x = 2\nPRINT x\n";
    assert!(c.hover(&p, text, "x", 1).is_some());
    c.shutdown();
}

static SLOW_RUNS: AtomicUsize = AtomicUsize::new(0);

/// [`slow_index`], counting its runs.
fn counting_slow_index(map: &SourceMap, parsed: &ParsedProgram, main: Option<&Path>, root: &Path) -> SymbolIndex {
    SLOW_RUNS.fetch_add(1, Ordering::SeqCst);
    slow_index(map, parsed, main, root)
}

/// An index job whose text changed before the worker started it is skipped (design D2): while the first job runs,
/// a second is sent and made stale; only the first and a third run.
#[test]
fn stale_index_job_skipped() {
    let dir = tempfile::tempdir().unwrap();
    let hooks = Hooks {
        index: counting_slow_index,
        ..qb64rust_driver::server_hooks()
    };
    let mut c = Client::start_with(dir.path(), hooks);
    let p = dir.path().join("p.bas");
    let texts = ["x = 1 ' SLOW\nPRINT x\n", "x = 2 ' SLOW\nPRINT x\n", "x = 3\nPRINT x\n"];
    c.open(&p, texts[0]);
    let first = c.send("textDocument/hover", Client::at(&p, texts[0], "x", 1, 0));
    // The first job must have started, or under load the change makes it stale too and it is skipped.
    let deadline = Instant::now() + WAIT;
    while SLOW_RUNS.load(Ordering::SeqCst) == 0 {
        assert!(Instant::now() < deadline, "the first index job never started");
        std::thread::sleep(Duration::from_millis(5));
    }
    c.change(&p, texts[1]);
    let second = c.send("textDocument/hover", Client::at(&p, texts[1], "x", 1, 0));
    c.change(&p, texts[2]);
    for id in [first, second] {
        let code = c.answer(&id).response_result.err().map(|e| e.code);
        assert_eq!(code, Some(CONTENT_MODIFIED));
    }
    assert!(c.hover(&p, texts[2], "x", 1).is_some());
    assert_eq!(SLOW_RUNS.load(Ordering::SeqCst), 2);
    c.shutdown();
}

/// A cancelled request that waits for the index is answered `RequestCancelled` at once; the index is still built
/// for the next request.
#[test]
fn cancel_while_waiting() {
    let dir = tempfile::tempdir().unwrap();
    let hooks = Hooks {
        index: slow_index,
        ..qb64rust_driver::server_hooks()
    };
    let mut c = Client::start_with(dir.path(), hooks);
    let p = dir.path().join("p.bas");
    let text = "x = 1 ' SLOW\nPRINT x\n";
    c.open(&p, text);
    let id = c.send("textDocument/hover", Client::at(&p, text, "x", 1, 0));
    // `send` numbers the requests: the hover's is the last.
    c.notify("$/cancelRequest", json!({ "id": c.next }));
    let code = c.answer(&id).response_result.err().map(|e| e.code);
    assert_eq!(code, Some(-32800), "RequestCancelled");
    assert!(c.hover(&p, text, "x", 1).is_some());
    c.shutdown();
}

/// "Hover": the scenarios of the spec, a variable, a constant, a built-in and a keyword.
#[test]
fn hover_scenarios() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let p = dir.path().join("p.bas");
    let text = "CONST LIMIT = 10\na$ = \"hello\"\nx$ = LEFT$(a$, 2)\nFOR i = 1 TO LIMIT\nNEXT\na\nSUB a\n    DIM n AS _UNSIGNED LONG\n    n = 3\nEND SUB\n";
    c.open(&p, text);
    let n = c.hover(&p, text, "n = 3", 0).unwrap();
    assert_eq!(n, "```qb64\nn AS _UNSIGNED LONG\n```\nLocal to `SUB a`.");
    let limit = c.hover(&p, text, "LIMIT", 1).unwrap();
    // An integer constant without a suffix is typed `_INTEGER64`, as QB64pe types it.
    assert_eq!(
        limit,
        "```qb64\nCONST LIMIT = 10\n```\nConstant, `_INTEGER64`, in the main module."
    );
    let left = c.hover(&p, text, "LEFT$", 0).unwrap();
    assert!(left.contains("Left$(STRING, LONG) AS STRING"), "{left}");
    assert_eq!(c.hover(&p, text, "FOR", 0), None);
    assert_eq!(c.hover(&p, text, "1 TO", 0), None);
    c.shutdown();
}

/// `STATIC a(5)` in a SUB beside a shared static array of the name and type is the SUB's own array (`study\39` row 4,
/// `QB64PE-DIVERGENCES.md` D-041), and hover follows that one answer: the SUB's uses name its array, the main
/// module's the shared one.
#[test]
fn hover_static_beside_shared() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let p = dir.path().join("p.bas");
    let text = "DIM SHARED a(3) AS LONG\na(1) = 7\ns\nSUB s\n    STATIC a(5) AS LONG\n    a(2) = 1\nEND SUB\n";
    c.open(&p, text);
    let inner = c.hover(&p, text, "a(2)", 0).unwrap();
    assert_eq!(inner, "```qb64\na() AS LONG\n```\nStatic in `SUB s`.");
    let outer = c.hover(&p, text, "a(1)", 0).unwrap();
    assert!(outer.contains("main module") && !outer.contains("SUB s"), "{outer}");
    c.shutdown();
}

/// "Across an include" and "Variable declared in an include": a variable declared in an included file on disk.
#[test]
fn across_an_include() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("lib.bi"),
        "' the library\r\nDIM SHARED total AS LONG\r\n",
    )
    .unwrap();
    let mut c = Client::start(dir.path());
    let main = dir.path().join("main.bas");
    let text = "'$INCLUDE:'lib.bi'\ntotal = 1\nPRINT total\n";
    c.open(&main, text);
    // By file (the main file first, then each included file as it was loaded) and offset.
    let with = c.references(&main, text, "total", 0, true);
    assert_eq!(with, [p("main.bas", 1, 0), p("main.bas", 2, 6), p("lib.bi", 1, 11)]);
    let without = c.references(&main, text, "total", 1, false);
    assert_eq!(without, [p("main.bas", 1, 0), p("main.bas", 2, 6)]);
    assert_eq!(c.definition(&main, text, "total", 1), Some(p("lib.bi", 1, 11)));
    c.shutdown();
}

/// "Two variables that differ by suffix", and "Local variable": `n` of the main module and of `SUB a` are apart.
#[test]
fn symbols_kept_apart() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("p.bas");
    let text = "s$ = \"x\"\ns% = 1\nPRINT s$; s%\nn = 1\nt\nSUB t\n    n = 2\n    PRINT n\nEND SUB\n";
    c.open(&f, text);
    let refs = c.references(&f, text, "s$", 1, true);
    assert_eq!(refs, [p("p.bas", 0, 0), p("p.bas", 2, 6)]);
    assert_eq!(c.definition(&f, text, "n", 2), Some(p("p.bas", 6, 4)));
    assert_eq!(c.definition(&f, text, "n", 0), Some(p("p.bas", 3, 0)));
    // A procedure still from the parser, a name with no symbol nowhere.
    assert_eq!(c.definition(&f, text, "t\nSUB", 0), Some(p("p.bas", 5, 4)));
    assert_eq!(c.definition(&f, text, "PRINT", 0), None);
    c.shutdown();
}

/// "A FUNCTION and its result": the references of a FUNCTION, from its call or its result, are the header, the
/// assignment and the call, as a rename edits them.
#[test]
fn references_of_a_function() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("p.bas");
    let text = "PRINT twice(2)\nFUNCTION twice (n)\n    twice = n * 2\nEND FUNCTION\n";
    c.open(&f, text);
    let all = [p("p.bas", 0, 6), p("p.bas", 1, 9), p("p.bas", 2, 4)];
    assert_eq!(c.references(&f, text, "twice", 0, true), all);
    assert_eq!(c.references(&f, text, "twice", 2, true), all);
    let without = c.references(&f, text, "twice", 0, false);
    assert_eq!(without, [p("p.bas", 0, 6), p("p.bas", 2, 4)]);
    c.shutdown();
}

/// The programs `tests/corpus/slice.list` names, each once (as `inputs.rs` reads the list).
fn slice_programs() -> Vec<String> {
    #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
    let list = std::fs::read_to_string(repo().join("tests/corpus/slice.list")).unwrap();
    // Off Windows, the programs the corpus's Linux list marks as needing Windows (`m7-linux`, design D6).
    #[expect(clippy::disallowed_methods, reason = "a list of file names, not BASIC source")]
    let linux = std::fs::read_to_string(repo().join("tests/corpus/known.linux.list")).unwrap_or_default();
    let windows_only: Vec<&str> = linux
        .lines()
        .filter_map(|l| l.split_once('\t'))
        .filter(|(_, reason)| reason.starts_with("windows"))
        .map(|(name, _)| name.trim())
        .collect();
    let mut out: Vec<String> = Vec::new();
    for line in list.lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let (folder, file) = line.rsplit_once('/').unwrap_or(("", line));
        let program = match file.split_once('.') {
            Some((name, _script)) if folder.is_empty() => name.to_string(),
            Some((name, _script)) => format!("{folder}/{name}"),
            None => line.to_string(),
        };
        let skipped = !cfg!(windows) && windows_only.contains(&program.as_str());
        if !skipped && !out.contains(&program) {
            out.push(program);
        }
    }
    out
}

/// A slice program parsed as the server parses a document (its files beside it in `<name>.files\` when it has
/// some), with its index.
fn slice_indexed(program: &str) -> (Analysis, SymbolIndex) {
    let path = repo().join("tests/corpus").join(format!("{program}.bas"));
    let files = path.with_extension("files");
    let base = if files.is_dir() {
        files.join(path.file_name().unwrap())
    } else {
        path.clone()
    };
    let root = repo().join("tests/upstream/root");
    let main = OpenFile {
        uri: from_path(&base),
        bytes: std::fs::read(&path).unwrap().into(),
        encoding: Encoding::DEFAULT,
    };
    let a = analyze("main", &main, &HashMap::new(), &root, Encoding::DEFAULT);
    let index = index_for_server(&a.map, &a.parsed, Some(&base), &root);
    (a, index)
}

/// Design D4: for every use of a procedure or label in the main file of each slice program, the parser's definition,
/// where it finds one, is the index's. (Where it finds none, the server takes the index's: a procedure of a `DECLARE
/// LIBRARY` block, the target of `RESTORE` in a procedure or of `ON TIMER … GOSUB`.)
#[test]
fn parser_and_index_definitions_agree() {
    let mut differ = Vec::new();
    let mut n = 0;
    for program in slice_programs() {
        let (a, index) = slice_indexed(&program);
        let main = a.parsed.main().file;
        for e in &index.entries {
            if !matches!(e.kind, EntryKind::Procedure | EntryKind::Label) {
                continue;
            }
            for s in e.uses().filter(|s| s.file == main) {
                n += 1;
                let parser = definition::definition(&a, s.start);
                if parser.is_some_and(|d| d != e.def) {
                    let (l, c) = a.map.file(main).line_col(s.start);
                    differ.push(format!(
                        "{program}:{l}:{c} {}: parser {parser:?}, index {:?}",
                        e.name, e.def
                    ));
                }
            }
        }
    }
    assert!(n > 100, "only {n} uses");
    assert!(
        differ.is_empty(),
        "{} of {n} differ:\n{}",
        differ.len(),
        differ.join("\n")
    );
}

/// "Rename a variable": every use keeps its suffix and letter case aside, and nothing else changes; prepare gives
/// the name without its suffix.
#[test]
fn rename_a_variable() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("p.bas");
    let text = "count% = 1\ncount% = count% + 1\nPRINT COUNT%; \"count%\"\n";
    c.open(&f, text);
    let r = c.request("textDocument/prepareRename", Client::at(&f, text, "count%", 1, 0));
    let range = r.response_result.unwrap();
    assert_eq!(range["start"], json!({ "line": 1, "character": 0 }));
    assert_eq!(range["end"], json!({ "line": 1, "character": 5 }));
    let edits = c.rename(&f, text, "count%", 2, "total").unwrap();
    assert_eq!(edits.len(), 1);
    assert_eq!(
        apply(text, &edits[0].1),
        "total% = 1\ntotal% = total% + 1\nPRINT total%; \"count%\"\n"
    );
    c.shutdown();
}

/// "Rename a procedure across files": the header in the included file and each call (and its `DECLARE`) in the
/// main file.
#[test]
fn rename_a_procedure_across_files() {
    let dir = tempfile::tempdir().unwrap();
    let lib = "SUB bump (n)\n    PRINT n\nEND SUB\n";
    std::fs::write(dir.path().join("lib.bm"), lib).unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("main.bas");
    let text = "DECLARE SUB bump (n)\nbump 1\nCALL bump(2)\n'$INCLUDE:'lib.bm'\n";
    c.open(&f, text);
    let edits = c.rename(&f, text, "bump", 1, "step_up").unwrap();
    assert_eq!(edits.len(), 2);
    assert_eq!(edits[0].0, "lib.bm");
    assert_eq!(apply(lib, &edits[0].1), "SUB step_up (n)\n    PRINT n\nEND SUB\n");
    assert_eq!(edits[1].0, "main.bas");
    assert_eq!(
        apply(text, &edits[1].1),
        "DECLARE SUB step_up (n)\nstep_up 1\nCALL step_up(2)\n'$INCLUDE:'lib.bm'\n"
    );
    c.shutdown();
}

/// "Rename refused": each case of the spec, with its message, and no edit.
#[test]
fn rename_refused() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let refused = |r: Result<Edits, (i32, String)>| {
        let (code, msg) = r.expect_err("refused");
        assert_eq!(code, -32803, "RequestFailed: {msg}");
        msg
    };
    // "Program with an error".
    let f = dir.path().join("error.bas");
    let text = "y = 2\nPRINT y\nx$ = 1\n";
    c.open(&f, text);
    let msg = refused(c.rename(&f, text, "y", 1, "z"));
    assert!(msg.contains("error.bas, line 3"), "{msg}");
    // "Clash".
    let f = dir.path().join("clash.bas");
    let text = "a\nSUB a\n    i = 1: j = 2\n    PRINT i; j\nEND SUB\n";
    c.open(&f, text);
    let msg = refused(c.rename(&f, text, "i", 1, "j"));
    assert!(msg.contains("`j`"), "{msg}");
    // "Reserved word", a suffix, a built-in, a line number, no symbol.
    assert!(refused(c.rename(&f, text, "i", 1, "print")).contains("reserved"));
    assert!(refused(c.rename(&f, text, "i", 1, "k%")).contains("suffix"));
    assert!(refused(c.rename(&f, text, "i", 1, "chr")).contains("built-in"));
    assert!(refused(c.rename(&f, text, "PRINT", 0, "k")).contains("no name"));
    let f = dir.path().join("number.bas");
    let text = "10 PRINT 1\nGOTO 10\n";
    c.open(&f, text);
    assert!(refused(c.rename(&f, text, "10", 1, "k")).contains("line number"));
    c.shutdown();
}

/// A use in a file under the compiler root refuses the rename; on Windows also when the root is spelled in another
/// letter case than the files' paths.
#[test]
fn rename_refused_under_the_root() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("lib.bi"), "DIM SHARED total AS LONG\n").unwrap();
    std::fs::create_dir(dir.path().join("prog")).unwrap();
    let mut roots = vec![dir.path().to_path_buf()];
    if cfg!(windows) {
        roots.push(PathBuf::from(dir.path().to_string_lossy().to_uppercase()));
    }
    for root in roots {
        let mut c = Client::start(&root);
        let f = dir.path().join("prog").join("main.bas");
        // Found from the main file's folder, so that its path is spelled as the main file's, not as the root.
        let text = "'$INCLUDE:'../lib.bi'\ntotal = 1\n";
        c.open(&f, text);
        let mut params = Client::at(&f, text, "total", 0, 0);
        params["newName"] = json!("sum");
        let r = c.request("textDocument/rename", params);
        let msg = r.response_result.unwrap_err().message;
        assert!(
            msg.contains("lib.bi, a file of the compiler's own"),
            "{}: {msg}",
            root.display()
        );
        c.shutdown();
    }
}

/// The driver's index, with the last use of the first symbol that has one dropped in a program with `dropped` in
/// it: a renamed program whose index misses a use.
fn dropping_index(map: &SourceMap, parsed: &ParsedProgram, main: Option<&Path>, root: &Path) -> SymbolIndex {
    let index = index_for_server(map, parsed, main, root);
    let bytes = &map.file(parsed.main().file).bytes;
    if !bytes.windows(7).any(|w| w.eq_ignore_ascii_case(b"dropped")) {
        return index;
    }
    let mut entries = index.entries.clone();
    if let Some(e) = entries.iter_mut().find(|e| !e.refs.is_empty()) {
        e.refs.pop();
    }
    SymbolIndex::new(entries, index.bodies.clone(), index.error.clone())
}

/// The check of the edit (design D5): a renamed program whose index differs refuses the rename.
#[test]
fn rename_refused_by_the_check_of_the_edit() {
    let dir = tempfile::tempdir().unwrap();
    let hooks = Hooks {
        index: dropping_index,
        ..qb64rust_driver::server_hooks()
    };
    let mut c = Client::start_with(dir.path(), hooks);
    let f = dir.path().join("p.bas");
    let text = "x = 1\nPRINT x\n";
    c.open(&f, text);
    let (_, msg) = c.rename(&f, text, "x", 0, "dropped").unwrap_err();
    assert!(msg.contains("would change the program"), "{msg}");
    assert!(c.rename(&f, text, "x", 0, "kept").is_ok());
    c.shutdown();
}

/// A program with a `$IF` branch that is not compiled: the rename is made, and a message says that branch was not
/// renamed. The server reads the program as compiled on the host, so the test names the host's branch (`m7-linux`).
#[test]
fn rename_with_an_inactive_branch() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("p.bas");
    let host = if cfg!(windows) { "WIN" } else { "LINUX" };
    let text = format!("x = 1\n$IF {host} THEN\nPRINT x\n$ELSE\nPRINT x + 1\n$END IF\n");
    let text = text.as_str();
    c.open(&f, text);
    let edits = c.rename(&f, text, "x", 0, "y").unwrap();
    assert_eq!(
        apply(text, &edits[0].1),
        format!("y = 1\n$IF {host} THEN\nPRINT y\n$ELSE\nPRINT x + 1\n$END IF\n")
    );
    let note = c.note("window/showMessage");
    assert!(note["message"].as_str().unwrap().contains("not compiled"), "{note}");
    c.shutdown();
}

/// The LSP numbers of the completion item kinds the tests look for.
const VARIABLE: u64 = 6;
const FUNCTION: u64 = 3;
const FIELD: u64 = 5;
const KEYWORD: u64 = 14;

/// "Local names in a SUB" and "Built-in spelling"; the procedures, constants and shared names seen in a SUB.
#[test]
fn completion_scenarios() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("p.bas");
    let text = "CONST LIMIT = 3\nDIM SHARED total AS LONG\nouter = 1\na\nSUB a\n    inner = 2\n    x = _rg\nEND SUB\n";
    c.open(&f, text);
    // The first request finds no index and starts one; the hover waits for it.
    c.complete(&f, text, "_rg", 0, 3);
    assert!(c.hover(&f, text, "inner", 0).is_some());
    let items = c.complete(&f, text, "_rg", 0, 3);
    let has = |name: &str, kind: u64| items.iter().any(|(l, k)| l == name && *k == kind);
    assert!(has("inner", VARIABLE), "{items:?}");
    assert!(!items.iter().any(|(l, _)| l == "outer"), "outer is the main module's");
    assert!(has("total", VARIABLE) && has("a", FUNCTION));
    assert!(items.iter().any(|(l, _)| l == "LIMIT"));
    assert!(has("_RGB32", FUNCTION) && has("FOR", KEYWORD));
    // In the main module: `outer`, not `inner`.
    let items = c.complete(&f, text, "outer", 0, 0);
    assert!(items.iter().any(|(l, _)| l == "outer"));
    assert!(!items.iter().any(|(l, _)| l == "inner"));
    c.shutdown();
}

/// Design D7: after `p.` for a variable of a user type, that type's members alone; after a dot on anything else,
/// nothing.
#[test]
fn completion_of_members() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("p.bas");
    let text = "TYPE pt\n    x AS LONG\n    y AS LONG\nEND TYPE\nTYPE other\n    z AS LONG\nEND TYPE\nDIM p AS pt\nn = 1\np.x = 1\nPRINT n.\n";
    c.open(&f, text);
    assert!(c.hover(&f, text, "p.x", 0).is_some());
    let mut items = c.complete(&f, text, "p.x", 0, 2);
    items.sort();
    assert_eq!(items, [("x".to_string(), FIELD), ("y".to_string(), FIELD)]);
    assert_eq!(c.complete(&f, text, "n.", 0, 2), []);
    c.shutdown();
}

/// "Completion" with no index for the current text: the last index's names; the next index has the new ones.
#[test]
fn completion_from_the_last_index() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path());
    let f = dir.path().join("p.bas");
    let text = "first = 1\nPRINT fi\n";
    c.open(&f, text);
    assert!(c.hover(&f, text, "first", 0).is_some());
    let text = "first = 1\nsecond = 2\nPRINT fi\n";
    c.change(&f, text);
    let items = c.complete(&f, text, "fi\n", 0, 2);
    assert!(items.iter().any(|(l, _)| l == "first"));
    assert!(!items.iter().any(|(l, _)| l == "second"));
    assert!(c.hover(&f, text, "second", 0).is_some());
    let items = c.complete(&f, text, "fi\n", 0, 2);
    assert!(items.iter().any(|(l, _)| l == "second"));
    c.shutdown();
}

/// Symbols of the slice programs a rename misses or refuses wrongly, each with its reason (task 4.3): none.
const KNOWN_RENAME_MISSES: &[(&str, &str)] = &[];

/// The refusals a rename of every symbol meets on purpose: a line number, a procedure of a `DECLARE LIBRARY` block,
/// a name used in QB64pe's own files, one of QB64pe's constants.
fn refused_on_purpose(msg: &str) -> bool {
    [
        "line number",
        "DECLARE LIBRARY",
        "compiler's own",
        "QB64pe's own constants",
    ]
    .iter()
    .any(|m| msg.contains(m))
}

/// Task 4.3: each symbol of each slice program renamed to a fresh name is accepted by the index's checks (design
/// D5), and the program with every symbol renamed at once has no error and the same typed dump, names aside: no
/// use is missing from the table. About 3 s in a debug build (2026-10-25).
#[test]
fn rename_every_symbol_of_the_slice_programs() {
    let root = repo().join("tests/upstream/root");
    let bare = |name: &str| {
        let b = name.as_bytes();
        let end = b
            .iter()
            .position(|c| !(c.is_ascii_alphanumeric() || *c == b'_' || *c == b'.'))
            .unwrap_or(b.len());
        name[..end].to_ascii_uppercase()
    };
    let mut failures = Vec::new();
    let mut renamed = 0;
    for program in slice_programs() {
        let (analysis, index) = slice_indexed(&program);
        let ix = Indexed { analysis, index };
        let mut edits: Vec<(Span, String)> = Vec::new();
        let mut back: Vec<(String, String)> = Vec::new();
        let mut done = Vec::new();
        for (i, e) in ix.index.entries.iter().enumerate() {
            // The first letter kept, which a `DEFtype` may give a type, with the `_`s before it.
            let lead = e.name.bytes().take_while(|&b| b == b'_').count() + 1;
            let fresh = format!("{}zq{i:05}z", &e.name[..lead]);
            if done.contains(&i) || KNOWN_RENAME_MISSES.iter().any(|(p, n)| *p == program && *n == e.name) {
                continue;
            }
            done.extend(e.twin);
            match rename::plan_entry(&ix, i, &fresh, &root) {
                Ok(plan) => {
                    edits.extend(plan.edits.iter().map(|&s| (s, fresh.clone())));
                    back.push((fresh.to_ascii_uppercase(), bare(&e.name)));
                    renamed += 1;
                }
                Err(msg) if refused_on_purpose(&msg) => {}
                Err(msg) => failures.push(format!("{program} {}: refused: {msg}", e.name)),
            }
        }
        // The program with every rename at once.
        edits.sort_by_key(|(s, _)| (s.file, s.start));
        if let Some(w) = edits
            .windows(2)
            .find(|w| w[0].0.file == w[1].0.file && w[0].0.end > w[1].0.start)
        {
            failures.push(format!("{program}: two renames overlap at {:?}", w[1].0));
            continue;
        }
        let a = &ix.analysis;
        let mut open: HashMap<String, OpenFile> = HashMap::new();
        for (file, f) in a.map.files() {
            let mine: Vec<&(Span, String)> = edits.iter().filter(|(s, _)| s.file == file).collect();
            if mine.is_empty() {
                continue;
            }
            let mut bytes = Vec::with_capacity(f.bytes.len());
            let mut at = 0;
            for (s, name) in mine {
                bytes.extend_from_slice(&f.bytes[at..s.start as usize]);
                bytes.extend_from_slice(name.as_bytes());
                at = s.end as usize;
            }
            bytes.extend_from_slice(&f.bytes[at..]);
            let info = a.info(file);
            let file = OpenFile {
                uri: info.uri.clone(),
                bytes: bytes.into(),
                encoding: Encoding::DEFAULT,
            };
            open.insert(info.key.clone(), file);
        }
        let main_info = a.info(a.parsed.main().file);
        let main = open.get(&main_info.key).cloned().unwrap_or_else(|| OpenFile {
            uri: main_info.uri.clone(),
            bytes: a.map.file(a.parsed.main().file).bytes.clone(),
            encoding: Encoding::DEFAULT,
        });
        let new = analyze(&main_info.key, &main, &open, &root, Encoding::DEFAULT);
        let path = qb64rust_lsp::uri::to_path(&main_info.uri).unwrap();
        let disk = Some((path.as_path(), root.as_path()));
        let off = qb64rust_base::fixes::Fixes::OFF;
        let (before, _) = check_program(&a.map, &a.parsed, disk, off);
        let (after, diagnostics) = check_program(&new.map, &new.parsed, disk, off);
        if diagnostics.has_errors() {
            let first = diagnostics.errors().next().unwrap();
            failures.push(format!(
                "{program}: the renamed program has an error: {}",
                first.render(&new.map)
            ));
            continue;
        }
        let mut dump = qb64rust_sema::dump_typed(&after);
        // The longest fresh names first cannot matter: they all have the same length.
        for (fresh, old) in &back {
            dump = dump.replace(fresh.as_str(), old);
        }
        let expected = qb64rust_sema::dump_typed(&before);
        if dump != expected {
            let (old, new) = expected
                .lines()
                .zip(dump.lines())
                .find(|(a, b)| a != b)
                .unwrap_or(("(end)", "(end)"));
            failures.push(format!("{program}: the typed dump differs: `{old}` became `{new}`"));
        }
    }
    assert!(renamed > 1000, "only {renamed} renames");
    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
