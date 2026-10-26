//! The server driven in-process through `Connection::memory()` (design D7): the scenarios of spec
//! `editor/language-server`.

use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::Uri;
use qb64rust_base::layout::{KeywordStyle, Settings};
use qb64rust_base::{Diagnostics, FileId, SourceMap, Span, to_u32};
use qb64rust_lsp::Check;
use qb64rust_lsp::uri::from_path;
use qb64rust_syntax::ParsedProgram;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::path::Path;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Long enough for a parse after the debounce on a loaded test machine; a wait for something that must not come
/// lasts this long too.
const WAIT: Duration = Duration::from_secs(2);

struct Client {
    conn: Connection,
    server: JoinHandle<Result<(), String>>,
    next_id: i32,
    /// Notifications received while waiting for something else.
    inbox: VecDeque<Notification>,
}

/// The front end of a check in these tests (the real one is the driver's, tested in `crates\driver\tests`): an
/// error at each word `BAD`, a warning at each `WARN`, in every file of the program; a check of a program with
/// `SLOW` in it takes half a second.
fn stand_in(map: &SourceMap, _: &ParsedProgram, _: Option<&Path>, _: &Path) -> Diagnostics {
    let mut d = Diagnostics::new();
    for (id, file) in map.files() {
        let text = &file.bytes;
        for (word, warning) in [(&b"BAD"[..], false), (&b"WARN"[..], true)] {
            for start in (0..text.len()).filter(|&i| text[i..].starts_with(word)) {
                let span = Span::new(id, to_u32(start), to_u32(start + word.len()));
                if warning {
                    d.warning(span, "a WARN word");
                } else {
                    d.error(span, "a BAD word");
                }
            }
        }
        if text.windows(4).any(|w| w == b"SLOW") {
            std::thread::sleep(Duration::from_millis(500));
        }
    }
    d
}

/// A check that must not run: the live parse never calls it.
fn never(_: &SourceMap, _: &ParsedProgram, _: Option<&Path>, _: &Path) -> Diagnostics {
    panic!("the check ran")
}

/// The formatter in these tests (the real one is the driver's, tested in `crates\driver\tests\format_server.rs`):
/// the main file with each run of blanks made one, and the settings shown: the words in capitals or lower case by
/// the keyword style, and a line that starts with a blank indented by the indent size. A program with `BAD` in it
/// has an error there; one with `SLOW` takes half a second.
fn stand_in_format(
    map: &SourceMap,
    parsed: &ParsedProgram,
    path: Option<&Path>,
    root: &Path,
    settings: &Settings,
) -> Result<Vec<u8>, Diagnostics> {
    let check = stand_in(map, parsed, path, root);
    if check.has_errors() {
        return Err(check);
    }
    let text = &map.file(FileId(0)).bytes;
    let mut lines: Vec<&[u8]> = text.split(|&c| c == b'\n').collect();
    if lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    let mut out = Vec::new();
    for line in lines {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        if line.first() == Some(&b' ') {
            out.extend(std::iter::repeat_n(b' ', settings.indent_size as usize));
        }
        let mut blank = false;
        for &c in line.iter().skip_while(|c| **c == b' ') {
            if c == b' ' && blank {
                continue;
            }
            blank = c == b' ';
            out.push(match settings.keywords {
                KeywordStyle::Upper => c.to_ascii_uppercase(),
                KeywordStyle::Lower => c.to_ascii_lowercase(),
                KeywordStyle::Camel => c,
            });
        }
        out.extend_from_slice(b"\r\n");
    }
    Ok(out)
}

impl Client {
    fn start(root: &Path, options: Value) -> (Client, Value) {
        Client::start_with(root, options, stand_in)
    }

    fn start_with(root: &Path, options: Value, check: Check) -> (Client, Value) {
        let (server_side, conn) = Connection::memory();
        let root = root.to_path_buf();
        let hooks = qb64rust_lsp::Hooks {
            check,
            format: stand_in_format,
        };
        let server = std::thread::Builder::new()
            .stack_size(qb64rust_base::STACK_SIZE)
            .spawn(move || qb64rust_lsp::serve(server_side, root, hooks))
            .unwrap();
        let mut c = Client {
            conn,
            server,
            next_id: 0,
            inbox: VecDeque::new(),
        };
        let params =
            json!({ "processId": null, "rootUri": null, "capabilities": {}, "initializationOptions": options });
        let result = c.request("initialize", params).response_result.unwrap();
        c.notify("initialized", json!({}));
        (c, result)
    }

    fn send_request(&mut self, method: &str, params: Value) -> RequestId {
        self.next_id += 1;
        let id = RequestId::from(self.next_id);
        let req = Request::new(id.clone(), method.into(), params);
        self.conn.sender.send(req.into()).unwrap();
        id
    }

    fn response(&mut self, id: &RequestId) -> Response {
        let deadline = Instant::now() + WAIT;
        loop {
            match self.recv(deadline).expect("no response in time") {
                Message::Response(r) if &r.id == id => return r,
                Message::Notification(n) => self.inbox.push_back(n),
                m => panic!("unexpected message {m:?}"),
            }
        }
    }

    fn request(&mut self, method: &str, params: Value) -> Response {
        let id = self.send_request(method, params);
        self.response(&id)
    }

    fn notify(&self, method: &str, params: Value) {
        self.conn
            .sender
            .send(Notification::new(method.into(), params).into())
            .unwrap();
    }

    fn recv(&self, deadline: Instant) -> Option<Message> {
        self.conn
            .receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .ok()
    }

    /// The next diagnostics published for `uri`, or `None` when none come within [`WAIT`].
    fn diagnostics(&mut self, uri: &Uri) -> Option<Vec<Value>> {
        let is_for = |n: &Notification| {
            n.method == "textDocument/publishDiagnostics" && n.params["uri"].as_str() == Some(uri.as_str())
        };
        if let Some(i) = self.inbox.iter().position(is_for) {
            let n = self.inbox.remove(i).unwrap();
            return Some(n.params["diagnostics"].as_array().unwrap().clone());
        }
        let deadline = Instant::now() + WAIT;
        loop {
            match self.recv(deadline)? {
                Message::Notification(n) if is_for(&n) => {
                    return Some(n.params["diagnostics"].as_array().unwrap().clone());
                }
                Message::Notification(n) => self.inbox.push_back(n),
                m => panic!("unexpected message {m:?}"),
            }
        }
    }

    fn open(&self, uri: &Uri, text: &str) {
        let doc = json!({ "uri": uri, "languageId": "qb64rust", "version": 1, "text": text });
        self.notify("textDocument/didOpen", json!({ "textDocument": doc }));
    }

    fn change(&self, uri: &Uri, version: i32, text: &str) {
        let params = json!({
            "textDocument": { "uri": uri, "version": version },
            "contentChanges": [{ "text": text }],
        });
        self.notify("textDocument/didChange", params);
    }

    fn shutdown(mut self) -> Result<(), String> {
        let r = self.request("shutdown", Value::Null);
        assert!(r.response_result.is_ok());
        self.notify("exit", Value::Null);
        self.server.join().unwrap()
    }
}

fn temp() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

fn uri(dir: &Path, name: &str) -> Uri {
    from_path(&dir.join(name))
}

/// (line, character) of a diagnostic's start, and its message.
fn at(d: &Value) -> (u64, u64, String) {
    let s = &d["range"]["start"];
    let msg = d["message"].as_str().unwrap().to_string();
    (s["line"].as_u64().unwrap(), s["character"].as_u64().unwrap(), msg)
}

#[test]
fn initialize_and_exit() {
    let dir = temp();
    let (c, result) = Client::start(dir.path(), Value::Null);
    let caps = result["capabilities"].as_object().unwrap();
    let mut names: Vec<_> = caps.keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "definitionProvider",
            "documentFormattingProvider",
            "documentRangeFormattingProvider",
            "documentSymbolProvider",
            "foldingRangeProvider",
            "textDocumentSync"
        ]
    );
    assert_eq!(caps["textDocumentSync"], json!(1));
    assert_eq!(result["serverInfo"]["name"], "qb64rust");
    c.shutdown().unwrap();
}

#[test]
fn exit_without_shutdown_is_an_error() {
    let dir = temp();
    let (c, _) = Client::start(dir.path(), Value::Null);
    c.notify("exit", Value::Null);
    assert!(c.server.join().unwrap().is_err());
}

#[test]
fn error_appears_and_goes() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "PRINT 1\r\nx = 1 +\r\n");
    let d = c.diagnostics(&u).unwrap();
    assert_eq!(d.len(), 1);
    assert_eq!(at(&d[0]).0, 1);
    assert_eq!(d[0]["source"], "qb64rust");
    assert_eq!(d[0]["severity"], 1);
    c.change(&u, 2, "PRINT 1\r\nx = 1 + 2\r\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    c.shutdown().unwrap();
}

#[test]
fn unsupported_statement_is_no_diagnostic() {
    // `x$ = 1` is a type error of `sema`, but parses: the live parse publishes nothing for it, and never runs the
    // check (which would panic, and publish nothing at all).
    let dir = temp();
    let (mut c, _) = Client::start_with(dir.path(), Value::Null, never);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "PRINT 1\nx$ = 1\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    c.change(&u, 2, "PRINT 2\nx$ = 1\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    c.shutdown().unwrap();
}

fn check_params(u: &Uri) -> Value {
    json!({ "uri": u })
}

/// The error code of a response that failed.
fn error_code(r: Response) -> i32 {
    r.response_result.unwrap_err().code
}

#[test]
fn check_publishes_and_counts() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "PRINT 1 ' WARN\r\nx = BAD\r\ny = BAD\r\n");
    // The live parse: nothing.
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    let r = c.request(qb64rust_lsp::CHECK, check_params(&u));
    assert_eq!(r.response_result.unwrap(), json!({ "errors": 2, "warnings": 1 }));
    let d = c.diagnostics(&u).unwrap();
    let found: Vec<_> = d.iter().map(|d| (at(d), d["severity"].as_u64().unwrap())).collect();
    assert_eq!(
        found,
        [
            ((0, 10, "a WARN word".to_string()), 2),
            ((1, 4, "a BAD word".to_string()), 1),
            ((2, 4, "a BAD word".to_string()), 1),
        ]
    );
    assert!(d.iter().all(|d| d["source"] == "qb64rust"));
    c.shutdown().unwrap();
}

#[test]
fn check_of_a_document_not_open() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let r = c.request(qb64rust_lsp::CHECK, check_params(&uri(dir.path(), "none.bas")));
    assert_eq!(error_code(r), ErrorCode::InvalidParams as i32);
    let r = c.request(qb64rust_lsp::CHECK, json!({}));
    assert_eq!(error_code(r), ErrorCode::InvalidParams as i32);
    assert!(c.inbox.is_empty(), "published: {:?}", c.inbox);
    assert!(c.recv(Instant::now() + WAIT).is_none());
    c.shutdown().unwrap();
}

#[test]
fn check_error_in_an_included_file() {
    let dir = temp();
    std::fs::write(dir.path().join("lib.bi"), "' lib\r\nDIM a\r\nb = 1\r\ny$ = BAD\r\n").unwrap();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let main = uri(dir.path(), "main.bas");
    let lib = uri(dir.path(), "lib.bi");
    c.open(&main, "'$INCLUDE: 'lib.bi'\r\nPRINT 1\r\n");
    assert_eq!(c.diagnostics(&main).unwrap(), Vec::<Value>::new());
    assert_eq!(c.diagnostics(&lib).unwrap(), Vec::<Value>::new());
    let r = c.request(qb64rust_lsp::CHECK, check_params(&main));
    assert_eq!(r.response_result.unwrap()["errors"], 1);
    assert_eq!(c.diagnostics(&main).unwrap(), Vec::<Value>::new());
    let d = c.diagnostics(&lib).unwrap();
    assert_eq!(d.len(), 1);
    assert_eq!(at(&d[0]).0, 3);
    c.shutdown().unwrap();
}

#[test]
fn edit_after_a_check() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = BAD\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    c.request(qb64rust_lsp::CHECK, check_params(&u));
    assert_eq!(c.diagnostics(&u).unwrap().len(), 1);
    // The parse after the edit publishes the parser's diagnostics alone.
    c.change(&u, 2, "x = BAD + 1\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    c.shutdown().unwrap();
}

#[test]
fn check_right_after_an_edit_is_not_overwritten() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = 1\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    // A save within the debounce of an edit: the check sees the edit, and no parse publishes after it.
    c.change(&u, 2, "x = BAD\n");
    c.request(qb64rust_lsp::CHECK, check_params(&u));
    assert_eq!(c.diagnostics(&u).unwrap().len(), 1);
    assert_eq!(c.diagnostics(&u), None, "a parse published after the check");
    c.shutdown().unwrap();
}

#[test]
fn two_checks_in_a_row() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = BAD ' SLOW\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    // The first check is still running when the second comes.
    let first = c.send_request(qb64rust_lsp::CHECK, check_params(&u));
    let second = c.send_request(qb64rust_lsp::CHECK, check_params(&u));
    assert_eq!(error_code(c.response(&first)), ErrorCode::ContentModified as i32);
    assert_eq!(c.response(&second).response_result.unwrap()["errors"], 1);
    assert_eq!(c.diagnostics(&u).unwrap().len(), 1);
    assert_eq!(c.diagnostics(&u), None, "the first check published too");
    c.shutdown().unwrap();
}

#[test]
fn cancelled_check_publishes_the_parse_only() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = BAD ' SLOW\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    let id = c.send_request(qb64rust_lsp::CHECK, check_params(&u));
    let n: i32 = id.to_string().parse().unwrap();
    c.notify("$/cancelRequest", json!({ "id": n }));
    assert_eq!(error_code(c.response(&id)), ErrorCode::RequestCanceled as i32);
    // The parse made in its place publishes the parser's diagnostics (none), not the check's.
    assert_eq!(
        c.diagnostics(&u).unwrap(),
        Vec::<Value>::new(),
        "the cancelled check published"
    );
    assert_eq!(c.diagnostics(&u), None);
    c.shutdown().unwrap();
}

#[test]
fn cancelled_check_after_an_edit_parses_the_edit() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = 1 ' SLOW\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    // The check takes the place of the edit's parse; when it is cancelled, the edit is parsed all the same.
    c.change(&u, 2, "x = ( ' SLOW\n");
    let id = c.send_request(qb64rust_lsp::CHECK, check_params(&u));
    let n: i32 = id.to_string().parse().unwrap();
    c.notify("$/cancelRequest", json!({ "id": n }));
    assert_eq!(error_code(c.response(&id)), ErrorCode::RequestCanceled as i32);
    assert_eq!(c.diagnostics(&u).unwrap().len(), 1);
    c.shutdown().unwrap();
}

#[test]
fn failed_check_parses_and_answers_the_waiting_requests() {
    let dir = temp();
    let (mut c, _) = Client::start_with(dir.path(), Value::Null, never);
    let u = uri(dir.path(), "p.bas");
    // A check before the first parse, and an outline request while it runs: the request waits.
    c.open(&u, "SUB s\r\nEND SUB\r\nx = (\r\n");
    let check = c.send_request(qb64rust_lsp::CHECK, check_params(&u));
    let symbols = c.send_request("textDocument/documentSymbol", json!({ "textDocument": { "uri": u } }));
    assert_eq!(error_code(c.response(&check)), ErrorCode::InternalError as i32);
    let names = c.response(&symbols).response_result.unwrap();
    assert_eq!(names.as_array().unwrap().len(), 1, "{names}");
    assert_eq!(c.diagnostics(&u).unwrap().len(), 1);
    c.shutdown().unwrap();
}

#[test]
fn error_in_an_included_file_on_disk() {
    let dir = temp();
    std::fs::write(dir.path().join("lib.bi"), "' lib\r\nDIM a AS INTEGER\r\nx = (1\r\n").unwrap();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let main = uri(dir.path(), "main.bas");
    c.open(&main, "'$INCLUDE: 'lib.bi'\r\nPRINT 1\r\n");
    assert_eq!(c.diagnostics(&main).unwrap(), Vec::<Value>::new());
    let d = c.diagnostics(&uri(dir.path(), "lib.bi")).unwrap();
    assert_eq!(d.len(), 1);
    assert_eq!(at(&d[0]).0, 2);
    c.shutdown().unwrap();
}

#[test]
fn include_after_a_built_in_file() {
    // `$COLOR` adds QB64pe's constants as a file of the map before the include is loaded.
    let dir = temp();
    std::fs::write(dir.path().join("lib.bi"), "x = (\n").unwrap();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let main = uri(dir.path(), "main.bas");
    c.open(&main, "$COLOR:0\n'$INCLUDE: 'lib.bi'\n");
    assert_eq!(c.diagnostics(&main).unwrap(), Vec::<Value>::new());
    assert_eq!(c.diagnostics(&uri(dir.path(), "lib.bi")).unwrap().len(), 1);
    c.shutdown().unwrap();
}

#[test]
fn unsaved_include_and_its_includer() {
    let dir = temp();
    // On disk the include is fine; open in the editor it has an error on line 3.
    std::fs::write(dir.path().join("lib.bi"), "x = 1\n").unwrap();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let lib = uri(dir.path(), "lib.bi");
    let main = uri(dir.path(), "main.bas");
    c.open(&lib, "x = 1\ny = 2\nz = (3\n");
    assert_eq!(at(&c.diagnostics(&lib).unwrap()[0]).0, 2);
    c.open(&main, "'$INCLUDE: 'lib.bi'\nPRINT x\n");
    assert_eq!(c.diagnostics(&main).unwrap(), Vec::<Value>::new());
    // The main program's parse read the open document, not the disk.
    let d = c.diagnostics(&lib).unwrap();
    assert_eq!(at(&d[0]).0, 2);

    // Editing the include parses the includer again; the last parse to cover the include is the includer's.
    c.change(&lib, 2, "x = 1\ny = 2\nz = 3\n");
    assert_eq!(c.diagnostics(&main).unwrap(), Vec::<Value>::new());
    let mut last = None;
    while let Some(d) = c.diagnostics(&lib) {
        last = Some(d);
    }
    assert_eq!(last.unwrap(), Vec::<Value>::new());
    c.shutdown().unwrap();
}

/// A `$USELIBRARY` library under the include root (design D7 of `m4-builtins-remaining`): the line's own error
/// (a library with no descriptor) is reported at the `$USELIBRARY` line; an error in a library's file is reported
/// in that file, as an included file's is.
#[test]
fn library_errors_in_its_file_and_at_its_line() {
    let dir = temp();
    let libs = dir.path().join("libraries");
    std::fs::create_dir_all(libs.join("descriptors").join("test")).unwrap();
    std::fs::create_dir_all(libs.join("includes").join("test").join("a")).unwrap();
    std::fs::write(
        libs.join("descriptors").join("test").join("a.ini"),
        "[LIBRARY INCLUDES]\nIncAtTop=a_top.bi\n",
    )
    .unwrap();
    let top = libs.join("includes").join("test").join("a").join("a_top.bi");
    std::fs::write(&top, "' a\nx = (1\n").unwrap();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let main = uri(dir.path(), "main.bas");
    c.open(
        &main,
        "$CONSOLE:ONLY\n$USELIBRARY:'test/a'\n$USELIBRARY:'test/none'\nPRINT 1\n",
    );
    let d = c.diagnostics(&main).unwrap();
    assert_eq!(d.len(), 1, "{d:?}");
    assert_eq!(at(&d[0]).0, 2);
    assert!(d[0]["message"].as_str().unwrap().contains("test/none"), "{d:?}");
    let d = c
        .diagnostics(&uri(dir.path(), "libraries/includes/test/a/a_top.bi"))
        .unwrap();
    assert_eq!(d.len(), 1, "{d:?}");
    assert_eq!(at(&d[0]).0, 1);
    c.shutdown().unwrap();
}

#[test]
fn include_root_from_the_options() {
    let dir = temp();
    let root = dir.path().join("root");
    std::fs::create_dir(&root).unwrap();
    std::fs::write(root.join("lib.bi"), "x = (\n").unwrap();
    let options = json!({ "includeRoot": root.to_string_lossy() });
    let (mut c, _) = Client::start(Path::new("not-used"), options);
    let main = uri(dir.path(), "main.bas");
    c.open(&main, "'$INCLUDE: 'lib.bi'\n");
    assert_eq!(c.diagnostics(&main).unwrap(), Vec::<Value>::new());
    assert_eq!(c.diagnostics(&from_path(&root.join("lib.bi"))).unwrap().len(), 1);
    c.shutdown().unwrap();
}

#[test]
fn cp437_bytes_and_columns() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "box.bas");
    // Three CP437 bytes in the string: no diagnostic.
    c.open(&u, "PRINT \"\u{2551}\u{2550}\u{2557}\"\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    // An error after them: one column per byte, which is one per character.
    c.change(&u, 2, "PRINT \"\u{2551}\u{2550}\u{2557}\" +\n");
    let d = c.diagnostics(&u).unwrap();
    assert_eq!(d.len(), 1);
    // `expected an expression` at the line end: byte (and character) 13; a UTF-8 reading would say 19.
    assert_eq!(at(&d[0]).1, 13, "{:?}", at(&d[0]));
    c.shutdown().unwrap();
}

#[test]
fn utf8_column_after_a_multi_byte_character() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let ascii = uri(dir.path(), "ascii.bas");
    let utf8 = uri(dir.path(), "utf8.bas");
    c.notify("qb64rust/documentEncoding", json!({ "uri": utf8, "encoding": "utf8" }));
    c.open(&ascii, "PRINT \"e\" +\n");
    c.open(&utf8, "PRINT \"\u{e9}\" +\n");
    let a = at(&c.diagnostics(&ascii).unwrap()[0]);
    let u = at(&c.diagnostics(&utf8).unwrap()[0]);
    // `é` is two bytes in UTF-8 and one UTF-16 unit: the same column as with `e`.
    assert_eq!(u, a);
    c.shutdown().unwrap();
}

#[test]
fn encoding_change_reparses() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), json!({ "encoding": "utf8" }));
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "PRINT \"\u{e9}\" +\n");
    let utf8 = at(&c.diagnostics(&u).unwrap()[0]);
    c.notify("qb64rust/documentEncoding", json!({ "uri": u, "encoding": "cp437" }));
    let cp437 = at(&c.diagnostics(&u).unwrap()[0]);
    assert_eq!(utf8, cp437);
    c.shutdown().unwrap();
}

#[test]
fn burst_of_edits_parses_once() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = 1 +\n");
    c.change(&u, 2, "x = 1 + 2 +\n");
    c.change(&u, 3, "x = 1 + 2\ny = (\n");
    let d = c.diagnostics(&u).unwrap();
    assert_eq!(d.len(), 1);
    assert_eq!(at(&d[0]).0, 1);
    assert_eq!(c.diagnostics(&u), None, "a second parse was published");
    c.shutdown().unwrap();
}

#[test]
fn requests_on_a_program() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    let text = "bump 3\r\nDO\r\n  GOTO done\r\nLOOP\r\ndone:\r\nEND\r\nSUB bump (n)\r\n  PRINT n\r\nEND SUB\r\n";
    c.open(&u, text);
    let doc = json!({ "textDocument": { "uri": u } });

    let symbols = c
        .request("textDocument/documentSymbol", doc.clone())
        .response_result
        .unwrap();
    let names: Vec<_> = symbols
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["done", "bump"]);

    let folds = c.request("textDocument/foldingRange", doc).response_result.unwrap();
    let lines: Vec<_> = folds
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["startLine"].as_u64().unwrap(), f["endLine"].as_u64().unwrap()))
        .collect();
    assert!(lines.contains(&(1, 2)), "{lines:?}");
    assert!(lines.contains(&(6, 7)), "{lines:?}");

    let def = |c: &mut Client, line: u32, character: u32| {
        let params = json!({ "textDocument": { "uri": u }, "position": { "line": line, "character": character } });
        c.request("textDocument/definition", params).response_result.unwrap()
    };
    // Spec scenario: the call on line 1 goes to the header on line 7 (0-based 0 and 6).
    let d = def(&mut c, 0, 1);
    assert_eq!(d["uri"].as_str(), Some(u.as_str()));
    assert_eq!(d["range"]["start"], json!({ "line": 6, "character": 4 }));
    assert_eq!(
        def(&mut c, 2, 8)["range"]["start"],
        json!({ "line": 4, "character": 0 })
    );
    // `DO`, and a place without a name: nothing.
    assert_eq!(def(&mut c, 1, 1), Value::Null);
    assert_eq!(def(&mut c, 7, 0), Value::Null);
    c.shutdown().unwrap();
}

#[test]
fn request_for_a_document_not_open() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let doc = json!({ "textDocument": { "uri": uri(dir.path(), "none.bas") } });
    assert_eq!(
        c.request("textDocument/documentSymbol", doc).response_result.ok(),
        Some(json!([]))
    );
    c.shutdown().unwrap();
}

#[test]
fn cancelled_request() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "SUB a\nEND SUB\n");
    // Within the debounce no parse has completed, so the request waits; then the client cancels it.
    let id = c.send_request("textDocument/documentSymbol", json!({ "textDocument": { "uri": u } }));
    c.notify("$/cancelRequest", json!({ "id": 1_000_000 }));
    let n: i32 = id.to_string().parse().unwrap();
    c.notify("$/cancelRequest", json!({ "id": n }));
    let r = c.response(&id);
    assert_eq!(
        r.response_result.unwrap_err().code,
        lsp_server::ErrorCode::RequestCanceled as i32
    );
    // A request after the parse is answered.
    assert!(c.diagnostics(&u).is_some());
    let r = c.request("textDocument/documentSymbol", json!({ "textDocument": { "uri": u } }));
    assert_eq!(r.response_result.unwrap().as_array().unwrap().len(), 1);
    c.shutdown().unwrap();
}

#[test]
fn closing_one_includer_keeps_the_others_diagnostics() {
    let dir = temp();
    std::fs::write(dir.path().join("common.bi"), "x = (\n").unwrap();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let common = uri(dir.path(), "common.bi");
    let a = uri(dir.path(), "a.bas");
    let b = uri(dir.path(), "b.bas");
    c.open(&a, "'$INCLUDE: 'common.bi'\n");
    assert_eq!(c.diagnostics(&common).unwrap().len(), 1);
    c.open(&b, "'$INCLUDE: 'common.bi'\n");
    assert_eq!(c.diagnostics(&b).unwrap(), Vec::<Value>::new());
    assert_eq!(c.diagnostics(&common).unwrap().len(), 1);
    // `b` published the include's diagnostics last; closing it leaves `a`, which still includes it.
    c.notify("textDocument/didClose", json!({ "textDocument": { "uri": b } }));
    let mut last = None;
    while let Some(d) = c.diagnostics(&common) {
        last = Some(d);
    }
    assert_eq!(last.map(|d| d.len()), Some(1));
    c.shutdown().unwrap();
}

fn format_params(u: &Uri) -> Value {
    json!({ "textDocument": { "uri": u }, "options": { "tabSize": 8, "insertSpaces": false } })
}

fn range_params(u: &Uri, start: (u32, u32), end: (u32, u32)) -> Value {
    json!({
        "textDocument": { "uri": u },
        "range": { "start": { "line": start.0, "character": start.1 }, "end": { "line": end.0, "character": end.1 } },
        "options": { "tabSize": 8, "insertSpaces": false },
    })
}

/// The edits of a formatting answer, as (start line, start character, end line, end character, new text).
fn edits(r: Response) -> Vec<(u64, u64, u64, u64, String)> {
    let v = r.response_result.unwrap();
    v.as_array()
        .unwrap()
        .iter()
        .map(|e| {
            let (s, t) = (&e["range"]["start"], &e["range"]["end"]);
            let n = |v: &Value| v.as_u64().unwrap();
            let text = e["newText"].as_str().unwrap().to_string();
            (
                n(&s["line"]),
                n(&s["character"]),
                n(&t["line"]),
                n(&t["character"]),
                text,
            )
        })
        .collect()
}

/// Formatting answers with one edit of the whole document, each line keeping its own line end, and writes nothing
/// (spec `editor/language-server`, "Formatting"); the request's `FormattingOptions` are not read.
#[test]
fn formatting_replaces_the_document() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "a   b\r\n  c  d\nlast");
    let got = edits(c.request("textDocument/formatting", format_params(&u)));
    assert_eq!(got, [(0, 0, 2, 4, "a b\r\n    c d\nlast".to_string())]);
    assert!(
        std::fs::read_dir(dir.path()).unwrap().next().is_none(),
        "no file is written"
    );
    c.shutdown().unwrap();
}

/// A document already formatted gets no edit ("Already formatted").
#[test]
fn formatting_an_unchanged_document() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "a b\r\nc\r\n");
    assert_eq!(edits(c.request("textDocument/formatting", format_params(&u))), []);
    c.shutdown().unwrap();
}

/// The settings come from `initializationOptions.format` ("Upper case keywords"), and a
/// `workspace/didChangeConfiguration` replaces them ("Setting changed").
#[test]
fn format_settings_from_the_client() {
    let dir = temp();
    let options = json!({ "format": { "keywordCase": "upper" } });
    let (mut c, _) = Client::start(dir.path(), options);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "print 1\n x\n");
    let got = edits(c.request("textDocument/formatting", format_params(&u)));
    assert_eq!(got[0].4, "PRINT 1\n    X\n");
    let settings = json!({ "settings": { "format": { "indentSize": 2, "keywordCase": "lower" } } });
    c.notify("workspace/didChangeConfiguration", settings);
    let got = edits(c.request("textDocument/formatting", format_params(&u)));
    assert_eq!(got[0].4, "print 1\n  x\n");
    c.shutdown().unwrap();
}

/// A program with an error: no edits, the diagnostics published, and a message naming the first error's file and
/// line ("Format failure").
#[test]
fn formatting_a_program_with_an_error() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = 1\r\ny = BAD\r\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    assert_eq!(edits(c.request("textDocument/formatting", format_params(&u))), []);
    let d = c.diagnostics(&u).unwrap();
    assert_eq!(at(&d[0]), (1, 4, "a BAD word".to_string()));
    let shown = c
        .inbox
        .iter()
        .find(|n| n.method == "window/showMessage")
        .expect("a message");
    assert_eq!(shown.params["type"], 1);
    let text = shown.params["message"].as_str().unwrap();
    assert!(text.contains("p.bas, line 2"), "{text}");
    c.shutdown().unwrap();
}

/// Range formatting edits only the lines the range touches ("One line of a block"); a range that ends at the start of
/// a line leaves that line.
#[test]
fn range_formatting_edits_the_lines_of_the_range() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "a   1\n  b   2\nc   3\n");
    let got = edits(c.request("textDocument/rangeFormatting", range_params(&u, (1, 2), (1, 4))));
    assert_eq!(got, [(1, 0, 1, 7, "    b 2".to_string())]);
    let got = edits(c.request("textDocument/rangeFormatting", range_params(&u, (0, 0), (2, 0))));
    assert_eq!(got.iter().map(|e| e.0).collect::<Vec<_>>(), [0, 1]);
    c.shutdown().unwrap();
}

/// An edit that comes while a document is formatted makes the answer `ContentModified`, so that no edit for an older
/// text is applied.
#[test]
fn formatting_refused_after_an_edit() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "SLOW  1\n");
    let id = c.send_request("textDocument/formatting", format_params(&u));
    c.change(&u, 2, "SLOW  2\n");
    assert_eq!(error_code(c.response(&id)), ErrorCode::ContentModified as i32);
    c.shutdown().unwrap();
}

/// A document closed and opened again with another text while it is formatted is a newer text too: its version is
/// the server's count, not one that starts again at the open.
#[test]
fn formatting_refused_after_close_and_open() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "SLOW  1\n");
    let id = c.send_request("textDocument/formatting", format_params(&u));
    c.notify("textDocument/didClose", json!({ "textDocument": { "uri": u } }));
    c.open(&u, "SLOW  2\n");
    assert_eq!(error_code(c.response(&id)), ErrorCode::ContentModified as i32);
    c.shutdown().unwrap();
}

#[test]
fn closing_clears_the_diagnostics() {
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "x = (\n");
    assert_eq!(c.diagnostics(&u).unwrap().len(), 1);
    c.notify("textDocument/didClose", json!({ "textDocument": { "uri": u } }));
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
    c.shutdown().unwrap();
}
