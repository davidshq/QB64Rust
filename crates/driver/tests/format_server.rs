//! Formatting through the language server with the driver's formatter (`format_for_server`), driven in-process: the
//! scenarios of spec `editor/language-server`, "Formatting", "Format settings from the client", "Format failure"
//! and "Range formatting" (design D5 of `m5b-formatter`). The protocol's own rules are tested with a stand-in
//! formatter in `crates\lsp\tests\server.rs`.

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use qb64rust_lsp::uri::from_path;
use serde_json::{Value, json};
use std::path::Path;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Long enough for a format of a test program on a loaded machine.
const WAIT: Duration = Duration::from_secs(20);

struct Client {
    conn: Connection,
    server: JoinHandle<Result<(), String>>,
    next: i32,
    notes: Vec<Notification>,
}

impl Client {
    fn start(root: &Path, options: Value) -> Client {
        let (server_side, conn) = Connection::memory();
        let root = root.to_path_buf();
        let server = std::thread::Builder::new()
            .stack_size(qb64rust_base::STACK_SIZE)
            .spawn(move || qb64rust_lsp::serve(server_side, root, qb64rust_driver::server_hooks()))
            .unwrap();
        let mut c = Client {
            conn,
            server,
            next: 0,
            notes: Vec::new(),
        };
        let init = json!({ "processId": null, "rootUri": null, "capabilities": {}, "initializationOptions": options });
        c.request("initialize", init);
        c.notify("initialized", json!({}));
        c
    }

    fn notify(&self, method: &str, params: Value) {
        let n = Notification::new(method.into(), params);
        self.conn.sender.send(n.into()).unwrap();
    }

    fn request(&mut self, method: &str, params: Value) -> Response {
        self.next += 1;
        let id = RequestId::from(self.next);
        self.conn
            .sender
            .send(Request::new(id.clone(), method.into(), params).into())
            .unwrap();
        let deadline = Instant::now() + WAIT;
        loop {
            let msg = self
                .conn
                .receiver
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .expect("no answer in time");
            match msg {
                Message::Response(r) if r.id == id => return r,
                Message::Notification(n) => self.notes.push(n),
                _ => {}
            }
        }
    }

    fn open(&self, path: &Path, text: &str) {
        let doc = json!({ "uri": from_path(path), "languageId": "qb64rust", "version": 1, "text": text });
        self.notify("textDocument/didOpen", json!({ "textDocument": doc }));
    }

    /// The text of the whole-document edit of a formatting answer; `None` for no edit.
    fn format(&mut self, path: &Path) -> Option<String> {
        let params =
            json!({ "textDocument": { "uri": from_path(path) }, "options": { "tabSize": 4, "insertSpaces": true } });
        let r = self.request("textDocument/formatting", params);
        let edits = r.response_result.unwrap();
        let edits = edits.as_array().unwrap();
        assert!(edits.len() <= 1, "{edits:?}");
        edits.first().map(|e| e["newText"].as_str().unwrap().to_string())
    }

    fn shutdown(mut self) {
        self.request("shutdown", Value::Null);
        self.notify("exit", Value::Null);
        self.server.join().unwrap().unwrap();
    }
}

/// "Unsaved text": the open document's text is formatted, and nothing is written to disk.
#[test]
fn unsaved_text() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let p = dir.path().join("p.bas");
    c.open(&p, "for i=1 to 3:next\n");
    assert_eq!(c.format(&p).as_deref(), Some("For i = 1 To 3: Next\n"));
    assert!(
        std::fs::read_dir(dir.path()).unwrap().next().is_none(),
        "nothing is written"
    );
    c.shutdown();
}

/// "Unsaved include": a name declared in an open, unsaved included document respells the main document.
#[test]
fn unsaved_include() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let main = dir.path().join("main.bas");
    let inc = dir.path().join("inc.bi");
    c.open(&inc, "DIM SHARED total AS LONG\n");
    c.open(&main, "'$INCLUDE:'inc.bi'\nTOTAL = 1\n");
    assert_eq!(c.format(&main).as_deref(), Some("'$Include:'inc.bi'\ntotal = 1\n"));
    c.shutdown();
}

/// "Already formatted": no edit.
#[test]
fn already_formatted() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let p = dir.path().join("p.bas");
    c.open(&p, "For i = 1 To 3\r\n    Print i\r\nNext\r\n");
    assert_eq!(c.format(&p), None);
    c.shutdown();
}

/// "Upper case keywords" and "Setting changed": the client's settings, and a change of them without a restart.
#[test]
fn client_settings() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), json!({ "format": { "keywordCase": "upper" } }));
    let p = dir.path().join("p.bas");
    c.open(&p, "print 1\nif 1 then\nprint 2\nend if\n");
    assert_eq!(
        c.format(&p).as_deref(),
        Some("PRINT 1\nIF 1 THEN\n    PRINT 2\nEND IF\n")
    );
    let change = json!({ "settings": { "format": { "indentSize": 2 } } });
    c.notify("workspace/didChangeConfiguration", change);
    assert_eq!(c.format(&p).as_deref(), Some("Print 1\nIf 1 Then\n  Print 2\nEnd If\n"));
    c.shutdown();
}

/// "Syntax error": no edit, an error on line 2 published, and a message that names line 2.
#[test]
fn syntax_error() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let p = dir.path().join("p.bas");
    c.open(&p, "print 1\nx = 1 +\n");
    assert_eq!(c.format(&p), None);
    let published = c
        .notes
        .iter()
        .filter(|n| n.method == "textDocument/publishDiagnostics")
        .flat_map(|n| n.params["diagnostics"].as_array().unwrap().clone())
        .any(|d| d["range"]["start"]["line"] == 1 && d["severity"] == 1);
    assert!(published, "{:?}", c.notes);
    let message = c
        .notes
        .iter()
        .find(|n| n.method == "window/showMessage")
        .map(|n| n.params["message"].as_str().unwrap().to_string())
        .expect("a message");
    assert!(message.contains("line 2"), "{message}");
    c.shutdown();
}

/// "One line of a block": a range over the line inside a `FOR` changes that line alone, indented as in the whole
/// program.
#[test]
fn one_line_of_a_block() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let p = dir.path().join("p.bas");
    c.open(&p, "for i=1 to 3\nprint   i\nnext\n");
    let params = json!({
        "textDocument": { "uri": from_path(&p) },
        "range": { "start": { "line": 1, "character": 0 }, "end": { "line": 1, "character": 9 } },
        "options": { "tabSize": 4, "insertSpaces": true },
    });
    let r = c
        .request("textDocument/rangeFormatting", params)
        .response_result
        .unwrap();
    let edits = r.as_array().unwrap();
    assert_eq!(edits.len(), 1, "{edits:?}");
    assert_eq!(edits[0]["newText"], "    Print i");
    assert_eq!(edits[0]["range"]["start"]["line"], 1);
    assert_eq!(edits[0]["range"]["end"]["line"], 1);
    c.shutdown();
}
