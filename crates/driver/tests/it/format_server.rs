//! Formatting through the language server with the driver's formatter (`format_for_server`), driven in-process: the
//! scenarios of spec `editor/language-server`, "Formatting", "Format settings from the client", "Format failure"
//! and "Range formatting" (design D5 of `m5b-formatter`). The protocol's own rules are tested with a stand-in
//! formatter in `crates\lsp\tests\it\server.rs`.

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

    /// Waits for the `qb64rust/program` notification of the document at `path`, and gives its program's uri.
    fn program(&mut self, path: &Path) -> Option<String> {
        let uri = from_path(path);
        let is_for = |n: &Notification| n.method == "qb64rust/program" && n.params["uri"] == uri.as_str();
        if let Some(n) = self.notes.iter().find(|n| is_for(n)) {
            return n.params["program"].as_str().map(str::to_string);
        }
        let deadline = Instant::now() + WAIT;
        loop {
            let msg = self
                .conn
                .receiver
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .expect("no program notification in time");
            if let Message::Notification(n) = msg {
                let found = is_for(&n).then(|| n.params["program"].as_str().map(str::to_string));
                self.notes.push(n);
                if let Some(program) = found {
                    return program;
                }
            }
        }
    }

    /// Whether a message was shown, or an error published, so far.
    fn complained(&self) -> bool {
        self.notes.iter().any(|n| {
            n.method == "window/showMessage"
                || (n.method == "textDocument/publishDiagnostics"
                    && n.params["diagnostics"].as_array().is_some_and(|d| !d.is_empty()))
        })
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

/// "The project's file over the client": the `[format]` table of `qb64rust.toml` goes over the client's settings,
/// key by key (OpenSpec change `project-settings`, design D2).
#[test]
fn settings_file_over_the_client() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("qb64rust.toml"), "[format]\nkeyword-case = \"upper\"\n").unwrap();
    let options = json!({ "format": { "keywordCase": "camel", "indentSize": 2 } });
    let mut c = Client::start(dir.path(), options);
    let p = dir.path().join("p.bas");
    c.open(&p, "if 1 then\nprint 2\nend if\n");
    assert_eq!(c.format(&p).as_deref(), Some("IF 1 THEN\n  PRINT 2\nEND IF\n"));
    assert!(!c.complained(), "{:?}", c.notes);
    c.shutdown();
}

/// "An include file takes its program's file": the settings file is the one found from the program's main file.
#[test]
fn include_file_takes_its_programs_settings() {
    let dir = tempfile::tempdir().unwrap();
    let proj = dir.path().join("proj");
    std::fs::create_dir_all(proj.join("lib")).unwrap();
    std::fs::write(proj.join("qb64rust.toml"), "[format]\nkeyword-case = \"upper\"\n").unwrap();
    std::fs::write(proj.join("game.bas"), "'$INCLUDE: 'lib/turn.bm'\n").unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let turn = proj.join("lib").join("turn.bm");
    c.open(&turn, "print 1\n");
    assert_eq!(
        c.program(&turn),
        Some(from_path(&proj.join("game.bas")).as_str().to_string())
    );
    assert_eq!(c.format(&turn).as_deref(), Some("PRINT 1\n"));
    c.shutdown();
}

/// "A bad settings file" ("Format failure"): no edit, and a message that names `qb64rust.toml` and its line.
#[test]
fn bad_settings_file() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("qb64rust.toml"), "[format]\nkeyword-case = \"uper\"\n").unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let p = dir.path().join("p.bas");
    c.open(&p, "print 1\n");
    assert_eq!(c.format(&p), None);
    let message = c
        .notes
        .iter()
        .find(|n| n.method == "window/showMessage")
        .map(|n| n.params["message"].as_str().unwrap().to_string())
        .expect("a message");
    assert!(message.contains("qb64rust.toml:2: `keyword-case` must be"), "{message}");
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

/// "A SUB that is not defined" ("Format failure"): an error of a later stage does not stop formatting, and no
/// message is shown.
#[test]
fn sub_not_defined() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let p = dir.path().join("p.bas");
    c.open(&p, "call   missing\n");
    assert_eq!(c.format(&p).as_deref(), Some("Call missing\n"));
    assert!(!c.complained(), "{:?}", c.notes);
    c.shutdown();
}

/// A lone `lib.bm` that calls a SUB no file defines ("Include file with no program" of spec `editor/formatting`):
/// indented and spaced, keywords in the keyword case, a variable as typed, and no message.
#[test]
fn lone_include_file() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let lib = dir.path().join("lib.bm");
    c.open(&lib, "sub a\ncall   missing\nATKFAC=1\nend sub\n");
    assert_eq!(c.program(&lib), None);
    assert_eq!(
        c.format(&lib).as_deref(),
        Some("Sub a\n    Call missing\n    ATKFAC = 1\nEnd Sub\n")
    );
    assert!(!c.complained(), "{:?}", c.notes);
    c.shutdown();
}

/// "Case from the program" ("Formatting an included file"; "Include file of a program on disk" of spec
/// `editor/formatting`): the document's names are spelled as its program on disk declares them, and the answer is
/// an edit of the document alone. A range is laid out the same way.
#[test]
fn include_file_case_from_its_program() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("main.bas"),
        "DIM SHARED Total AS LONG\n'$INCLUDE: 'lib.bm'\n",
    )
    .unwrap();
    let mut c = Client::start(dir.path(), Value::Null);
    let lib = dir.path().join("lib.bm");
    c.open(&lib, "total=1\nprint   total\n");
    assert_eq!(
        c.program(&lib),
        Some(from_path(&dir.path().join("main.bas")).as_str().to_string())
    );
    assert_eq!(c.format(&lib).as_deref(), Some("Total = 1\nPrint Total\n"));
    let params = json!({
        "textDocument": { "uri": from_path(&lib) },
        "range": { "start": { "line": 1, "character": 0 }, "end": { "line": 1, "character": 3 } },
        "options": { "tabSize": 4, "insertSpaces": true },
    });
    let r = c
        .request("textDocument/rangeFormatting", params)
        .response_result
        .unwrap();
    let edits = r.as_array().unwrap();
    assert_eq!(edits.len(), 1, "{edits:?}");
    assert_eq!(edits[0]["newText"], "Print Total");
    assert!(!c.complained(), "{:?}", c.notes);
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
