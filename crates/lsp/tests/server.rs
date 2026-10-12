//! The server driven in-process through `Connection::memory()` (design D7): the scenarios of spec
//! `editor/language-server`.

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::Uri;
use qb64rust_lsp::uri::from_path;
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

impl Client {
    fn start(root: &Path, options: Value) -> (Client, Value) {
        let (server_side, conn) = Connection::memory();
        let root = root.to_path_buf();
        let server = std::thread::Builder::new()
            .stack_size(qb64rust_base::STACK_SIZE)
            .spawn(move || qb64rust_lsp::serve(server_side, root))
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
    // `OPEN` does not compile yet, but parses: nothing from `sema` is published.
    let dir = temp();
    let (mut c, _) = Client::start(dir.path(), Value::Null);
    let u = uri(dir.path(), "p.bas");
    c.open(&u, "OPEN \"f\" FOR OUTPUT AS #1\nPRINT #1, 1\n");
    assert_eq!(c.diagnostics(&u).unwrap(), Vec::<Value>::new());
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
