//! The message loop (design D2, D5): the document store, a parse of each changed program after a debounce on a
//! worker thread, diagnostics per file, and requests answered from the last parse of their document. A check
//! (`qb64rust/check`, design D3 of `extension-diagnostics`) is a parse followed by the rest of the front end, on
//! the same worker.

use crate::analysis::{Analysis, OpenFile, analyze};
use crate::encoding::Encoding;
use crate::uri::{to_path, uri_key};
use crate::{Hooks, definition, folding, symbols};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    Cancel, DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
    Notification as _, PublishDiagnostics, ShowMessage,
};
use lsp_types::request::{
    DocumentSymbolRequest, FoldingRangeRequest, Formatting, GotoDefinition, RangeFormatting, Request as _, Shutdown,
};
use lsp_types::{
    CancelParams, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DocumentFormattingParams, DocumentRangeFormattingParams, DocumentSymbolParams, DocumentSymbolResponse,
    FoldingRangeParams, GotoDefinitionParams, GotoDefinitionResponse, InitializeParams, Location, MessageType,
    NumberOrString, Position, PublishDiagnosticsParams, Range, ServerCapabilities, ShowMessageParams,
    TextDocumentSyncKind, TextEdit, Uri,
};
use qb64rust_base::layout::{KeywordStyle, Settings};
use qb64rust_base::{Diagnostics, FileId, Severity};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// The extension's notification of a document's encoding (design D3).
pub const DOCUMENT_ENCODING: &str = "qb64rust/documentEncoding";

/// The request that runs the whole front end over a document's program: `{ uri }`, answered with
/// `{ errors, warnings }` (spec `editor/language-server`, "Check request").
pub const CHECK: &str = "qb64rust/check";

/// How long after a change a parse starts; a newer change in that time starts the wait again.
pub const DEBOUNCE: Duration = Duration::from_millis(100);

/// A string field of a JSON object.
fn field<'v>(v: &'v Value, name: &str) -> Option<&'v str> {
    v.get(name).and_then(Value::as_str)
}

/// The capabilities the server advertises (spec `editor/language-server`, "Server start"). No
/// `positionEncoding`: UTF-16, the only one VS Code's client accepts (design D3).
pub fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncKind::FULL.into()),
        document_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        folding_range_provider: Some(true.into()),
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        document_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        document_range_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        ..ServerCapabilities::default()
    }
}

/// The layout settings of a client's `format` object (spec `editor/language-server`, "Format settings from the
/// client"): `autoIndent`, `indentSize` (1 to 64), `indentSubs`, `autoLayout`, `keywordCase` (`camel`, `upper`,
/// `lower`); a setting missing or of another kind takes `-y`'s default.
pub fn format_settings(v: Option<&Value>) -> Settings {
    let mut s = Settings::default();
    let Some(v) = v else {
        return s;
    };
    let flag = |name: &str| v.get(name).and_then(Value::as_bool);
    if let Some(b) = flag("autoIndent") {
        s.auto_indent = b;
    }
    if let Some(b) = flag("indentSubs") {
        s.indent_subs = b;
    }
    if let Some(b) = flag("autoLayout") {
        s.auto_layout = b;
    }
    if let Some(n) = v
        .get("indentSize")
        .and_then(Value::as_u64)
        .filter(|n| (1..=64).contains(n))
    {
        s.indent_size = u32::try_from(n).unwrap_or(4);
    }
    s.keywords = match field(v, "keywordCase") {
        Some("upper") => KeywordStyle::Upper,
        Some("lower") => KeywordStyle::Lower,
        _ => KeywordStyle::Camel,
    };
    s
}

/// Runs the server on `conn` until `exit`. `default_root` is the include root when the client names none (the
/// folder of the `qb64rust` binary, the compiler's own rule); `hooks` the front end a check and a formatting request
/// run after the parse. An `exit` without `shutdown` before it is an error.
pub fn serve(conn: Connection, default_root: PathBuf, hooks: Hooks) -> Result<(), String> {
    let (id, params) = conn.initialize_start().map_err(|e| e.to_string())?;
    let params: InitializeParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
    // The client's `initializationOptions` (design D6): `encoding`, VS Code's encoding id for documents the client
    // says nothing about and for files read from disk; `includeRoot`, where included files are looked up after the
    // including file's folder (empty: the server's default). Both optional.
    let options = params.initialization_options.unwrap_or_default();
    let result = serde_json::json!({
        "capabilities": capabilities(),
        "serverInfo": { "name": "qb64rust", "version": env!("CARGO_PKG_VERSION") },
    });
    conn.initialize_finish(id, result).map_err(|e| e.to_string())?;

    let root = match field(&options, "includeRoot").filter(|r| !r.is_empty()) {
        Some(r) => std::path::absolute(r).unwrap_or_else(|_| PathBuf::from(r)),
        None => default_root,
    };
    let default = field(&options, "encoding").map_or(Encoding::DEFAULT, Encoding::from_name);
    let format = format_settings(options.get("format"));

    let (events, inbox) = mpsc::channel();
    let forward = events.clone();
    let receiver = conn.receiver.clone();
    std::thread::spawn(move || {
        for msg in receiver {
            if forward.send(Event::Message(msg)).is_err() {
                return;
            }
        }
        let _ = forward.send(Event::Disconnected);
    });
    let jobs = start_worker(events, hooks);
    let mut server = Server::new(conn, jobs, root, default);
    server.format = format;
    server.run(&inbox)
}

enum Event {
    Message(Message),
    Parsed {
        key: String,
        seq: u64,
        result: Result<Analysis, String>,
    },
    /// The result of a check: its parse and every diagnostic of the front end.
    Checked {
        key: String,
        seq: u64,
        id: RequestId,
        result: Result<(Analysis, Diagnostics), String>,
    },
    /// The result of a formatting request: its parse and the formatted file, or the front end's diagnostics.
    Formatted {
        key: String,
        request: FormatRequest,
        result: Result<(Analysis, Result<Vec<u8>, Diagnostics>), String>,
    },
    Disconnected,
}

/// A formatting request on its way: whole document or a range, the settings it is formatted with, and the document's
/// version when it was asked, so that an answer for an older text is refused.
#[derive(Clone)]
struct FormatRequest {
    id: RequestId,
    range: Option<Range>,
    settings: Settings,
    version: u64,
}

struct Job {
    key: String,
    seq: u64,
    main: OpenFile,
    open: Arc<HashMap<String, OpenFile>>,
    root: PathBuf,
    default: Encoding,
    /// The check request this job answers; `None` for a parse.
    check: Option<RequestId>,
    /// The formatting request this job answers.
    format: Option<FormatRequest>,
}

/// The message of a panic caught on the worker.
fn panic_message(e: &(dyn std::any::Any + Send), doing: &str) -> String {
    let msg = e
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| e.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    format!("internal error while {doing}: {msg}")
}

/// The parse worker: one program at a time, on a thread with the compiler's stack size (the parser's depth limits
/// are tuned for it). A panic in the parser, or in `check` for a check, is reported for that job; the worker goes
/// on.
fn start_worker(events: mpsc::Sender<Event>, hooks: Hooks) -> mpsc::Sender<Job> {
    let (jobs, queue) = mpsc::channel::<Job>();
    std::thread::Builder::new()
        .name("qb64rust-parse".into())
        .stack_size(qb64rust_base::STACK_SIZE)
        .spawn(move || {
            for job in queue {
                let parse = || analyze(&job.key, &job.main, &job.open, &job.root, job.default);
                if let Some(request) = &job.format {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let a = parse();
                        let path = to_path(&job.main.uri);
                        let text = (hooks.format)(&a.map, &a.parsed, path.as_deref(), &job.root, &request.settings);
                        (a, text)
                    }))
                    .map_err(|e| panic_message(&*e, "formatting"));
                    let event = Event::Formatted {
                        key: job.key.clone(),
                        request: request.clone(),
                        result,
                    };
                    if events.send(event).is_err() {
                        return;
                    }
                    continue;
                }
                let check = hooks.check;
                let event = match &job.check {
                    None => Event::Parsed {
                        key: job.key.clone(),
                        seq: job.seq,
                        result: std::panic::catch_unwind(std::panic::AssertUnwindSafe(parse))
                            .map_err(|e| panic_message(&*e, "parsing")),
                    },
                    Some(id) => {
                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            let a = parse();
                            // No disk checks for a document that is not a file (`untitled:`).
                            let path = to_path(&job.main.uri);
                            let diagnostics = check(&a.map, &a.parsed, path.as_deref(), &job.root);
                            (a, diagnostics)
                        }))
                        .map_err(|e| panic_message(&*e, "checking"));
                        Event::Checked {
                            key: job.key.clone(),
                            seq: job.seq,
                            id: id.clone(),
                            result,
                        }
                    }
                };
                if events.send(event).is_err() {
                    return;
                }
            }
        })
        .expect("cannot start the parse thread");
    jobs
}

struct Doc {
    uri: Uri,
    text: String,
    encoding: Encoding,
    bytes: Arc<[u8]>,
    /// The server's count of document changes when this text and encoding came (`Server::versions`): a formatting
    /// answer for an older one is refused.
    version: u64,
}

/// A request that waits for the first parse of its document.
struct Waiting {
    key: String,
    req: Request,
}

struct Server {
    conn: Connection,
    jobs: mpsc::Sender<Job>,
    root: PathBuf,
    default: Encoding,
    /// Open documents by key (`uri::uri_key`).
    docs: HashMap<String, Doc>,
    /// Encodings the client named, by key; kept after a close, as the client sends it only on open and change.
    encodings: HashMap<String, Encoding>,
    /// The last completed parse of each open document's program, by the document's key.
    analyses: HashMap<String, Analysis>,
    /// Programs to parse, with the time the parse may start.
    pending: HashMap<String, Instant>,
    /// The number of the last job sent for each program, until its result comes; a result of an older one is
    /// dropped.
    latest_job: HashMap<String, u64>,
    /// The last check sent for each program, with its job's number, until it is answered.
    checks: HashMap<String, (u64, RequestId)>,
    seq: u64,
    waiting: Vec<Waiting>,
    /// For each file with diagnostics published: the program whose parse published them, and its URI.
    published: HashMap<String, (String, Uri)>,
    shutdown: bool,
    /// The layout settings of formatting, from the client (`format_settings`).
    format: Settings,
    /// Counts every open, change and change of encoding of any document, so that a document closed and opened again
    /// never takes a version it had before (`Doc::version`).
    versions: u64,
}

impl Server {
    fn new(conn: Connection, jobs: mpsc::Sender<Job>, root: PathBuf, default: Encoding) -> Server {
        Server {
            conn,
            jobs,
            root,
            default,
            docs: HashMap::new(),
            encodings: HashMap::new(),
            analyses: HashMap::new(),
            pending: HashMap::new(),
            latest_job: HashMap::new(),
            checks: HashMap::new(),
            seq: 0,
            waiting: Vec::new(),
            published: HashMap::new(),
            shutdown: false,
            format: Settings::default(),
            versions: 0,
        }
    }

    fn run(&mut self, inbox: &mpsc::Receiver<Event>) -> Result<(), String> {
        loop {
            let next = self.pending.values().min().copied();
            let event = match next {
                Some(at) => match inbox.recv_timeout(at.saturating_duration_since(Instant::now())) {
                    Ok(e) => Some(e),
                    Err(mpsc::RecvTimeoutError::Timeout) => None,
                    Err(mpsc::RecvTimeoutError::Disconnected) => return Err("connection lost".into()),
                },
                None => Some(inbox.recv().map_err(|_| "connection lost".to_string())?),
            };
            match event {
                None => self.dispatch_due(),
                Some(Event::Message(Message::Request(req))) => self.request(req),
                Some(Event::Message(Message::Notification(n))) => {
                    if n.method == "exit" {
                        return if self.shutdown {
                            Ok(())
                        } else {
                            Err("exit without shutdown".into())
                        };
                    }
                    self.notification(n);
                }
                Some(Event::Message(Message::Response(_))) => {}
                Some(Event::Parsed { key, seq, result }) => self.parsed(key, seq, result),
                Some(Event::Checked { key, seq, id, result }) => self.checked(key, seq, id, result),
                Some(Event::Formatted { key, request, result }) => self.formatted(&key, request, result),
                Some(Event::Disconnected) => return Err("connection closed without exit".into()),
            }
        }
    }

    fn send(&self, msg: impl Into<Message>) {
        // The client went away: the loop ends at the next receive.
        let _ = self.conn.sender.send(msg.into());
    }

    fn notification(&mut self, n: Notification) {
        match n.method.as_str() {
            DidOpenTextDocument::METHOD => {
                if let Ok(p) = serde_json::from_value::<DidOpenTextDocumentParams>(n.params) {
                    let key = uri_key(&p.text_document.uri);
                    let encoding = self.encodings.get(&key).copied().unwrap_or(self.default);
                    let bytes = encoding.encode(&p.text_document.text).into();
                    self.versions += 1;
                    let doc = Doc {
                        uri: p.text_document.uri,
                        text: p.text_document.text,
                        encoding,
                        bytes,
                        version: self.versions,
                    };
                    self.docs.insert(key.clone(), doc);
                    self.changed(&key);
                }
            }
            DidChangeTextDocument::METHOD => {
                if let Ok(p) = serde_json::from_value::<DidChangeTextDocumentParams>(n.params) {
                    let key = uri_key(&p.text_document.uri);
                    // Full synchronisation: the last change holds the whole text.
                    if let (Some(doc), Some(change)) = (self.docs.get_mut(&key), p.content_changes.into_iter().last()) {
                        doc.bytes = doc.encoding.encode(&change.text).into();
                        doc.text = change.text;
                        self.versions += 1;
                        doc.version = self.versions;
                        self.changed(&key);
                    }
                }
            }
            DidCloseTextDocument::METHOD => {
                if let Ok(p) = serde_json::from_value::<DidCloseTextDocumentParams>(n.params) {
                    self.closed(&uri_key(&p.text_document.uri));
                }
            }
            DOCUMENT_ENCODING => {
                // `{ uri, encoding }`, the encoding as VS Code's id.
                let uri = field(&n.params, "uri").and_then(|u| Uri::from_str(u).ok());
                if let (Some(uri), Some(name)) = (uri, field(&n.params, "encoding")) {
                    let key = uri_key(&uri);
                    let encoding = Encoding::from_name(name);
                    self.encodings.insert(key.clone(), encoding);
                    if let Some(doc) = self.docs.get_mut(&key)
                        && doc.encoding != encoding
                    {
                        doc.encoding = encoding;
                        doc.bytes = encoding.encode(&doc.text).into();
                        self.versions += 1;
                        doc.version = self.versions;
                        self.changed(&key);
                    }
                }
            }
            DidChangeConfiguration::METHOD => {
                // `{ settings: { format: {...} } }`: the layout settings replace the last ones.
                if let Some(format) = n.params.get("settings").and_then(|s| s.get("format")) {
                    self.format = format_settings(Some(format));
                }
            }
            Cancel::METHOD => {
                if let Ok(p) = serde_json::from_value::<CancelParams>(n.params) {
                    let id: RequestId = match p.id {
                        NumberOrString::Number(i) => i.into(),
                        NumberOrString::String(s) => s.into(),
                    };
                    let code = ErrorCode::RequestCanceled as i32;
                    if let Some(i) = self.waiting.iter().position(|w| w.req.id == id) {
                        let w = self.waiting.remove(i);
                        self.send(Response::new_err(w.req.id, code, "cancelled".into()));
                    }
                    // A check under way runs to its end; its result is dropped, and the parse it stood for is
                    // made instead, after the debounce: a client that cancels a check to send a newer one sends
                    // the newer one at once, which then takes that parse's place again.
                    let check = self.checks.iter().find(|(_, (_, c))| *c == id).map(|(k, _)| k.clone());
                    if let Some(key) = check {
                        self.checks.remove(&key);
                        self.send(Response::new_err(id, code, "cancelled".into()));
                        self.pending.insert(key, Instant::now() + DEBOUNCE);
                    }
                }
            }
            _ => {}
        }
    }

    /// A document's text or encoding changed: parse its program, and every open program whose last parse
    /// included it, after the debounce.
    fn changed(&mut self, key: &str) {
        let at = Instant::now() + DEBOUNCE;
        self.pending.insert(key.to_string(), at);
        let dependents: Vec<String> = self
            .analyses
            .iter()
            .filter(|(k, a)| k.as_str() != key && a.files.iter().any(|f| f.key == key))
            .map(|(k, _)| k.clone())
            .collect();
        for k in dependents {
            self.pending.insert(k, at);
        }
    }

    fn closed(&mut self, key: &str) {
        self.docs.remove(key);
        self.pending.remove(key);
        self.latest_job.remove(key);
        if let Some((_, id)) = self.checks.remove(key) {
            let code = ErrorCode::ContentModified as i32;
            self.send(Response::new_err(id, code, "the document was closed".into()));
        }
        // Programs that included it now read it from disk.
        self.changed(key);
        self.pending.remove(key);
        self.analyses.remove(key);
        // Its program's diagnostics go, unless another open program covers the file.
        let files: Vec<(String, Uri)> = self
            .published
            .iter()
            .filter(|(_, (program, _))| program == key)
            .map(|(f, (_, uri))| (f.clone(), uri.clone()))
            .collect();
        for (file, uri) in files {
            self.release(file, uri, key);
        }
        // Requests waiting for it get no answer from a parse: answer them now.
        let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.waiting)
            .into_iter()
            .partition(|w| w.key == key);
        self.waiting = rest;
        for w in mine {
            self.answer(w.req, None);
        }
    }

    /// The diagnostics of `file` that program `from` published are no longer its to keep: the file's own program
    /// (when the file is open) or another open program that covers it is parsed again at once and publishes them
    /// anew; without one they are cleared.
    fn release(&mut self, file: String, uri: Uri, from: &str) {
        self.published.remove(&file);
        let other = if self.docs.contains_key(&file) {
            Some(file)
        } else {
            self.analyses
                .iter()
                .find(|(k, a)| k.as_str() != from && a.files.iter().any(|f| f.key == file))
                .map(|(k, _)| k.clone())
        };
        match other {
            Some(program) => {
                self.pending.insert(program, Instant::now());
            }
            None => self.publish(uri, Vec::new()),
        }
    }

    /// Sends the jobs whose debounce is over. A program that includes another goes after it, so that its parse,
    /// which sees the include as the program does, is the last to publish the include's diagnostics.
    fn dispatch_due(&mut self) {
        let now = Instant::now();
        let mut due: Vec<String> = self
            .pending
            .iter()
            .filter(|(_, at)| **at <= now)
            .map(|(k, _)| k.clone())
            .collect();
        due.sort_by_key(|k| self.analyses.get(k).map_or(1, |a| a.files.len()));
        let open: Arc<HashMap<String, OpenFile>> =
            Arc::new(self.docs.iter().map(|(k, d)| (k.clone(), open_file(d))).collect());
        for key in due {
            self.pending.remove(&key);
            let Some(doc) = self.docs.get(&key) else {
                continue;
            };
            self.seq += 1;
            self.latest_job.insert(key.clone(), self.seq);
            let job = Job {
                key,
                seq: self.seq,
                main: open_file(doc),
                open: Arc::clone(&open),
                root: self.root.clone(),
                default: self.default,
                check: None,
                format: None,
            };
            if self.jobs.send(job).is_err() {
                // The worker is gone (it never panics out of its loop); nothing more can be parsed.
                return;
            }
        }
    }

    fn parsed(&mut self, key: String, seq: u64, result: Result<Analysis, String>) {
        if self.latest_job.get(&key) != Some(&seq) || !self.docs.contains_key(&key) {
            return;
        }
        self.latest_job.remove(&key);
        match result {
            Ok(analysis) => {
                let diagnostics = analysis.parse_diagnostics();
                self.publish_program(key, analysis, diagnostics);
            }
            Err(msg) => self.failed(&key, &msg),
        }
    }

    /// A parse of program `key` failed: the requests waiting for it get the error.
    fn failed(&mut self, key: &str, msg: &str) {
        eprintln!("qb64rust: {msg} ({key})");
        let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.waiting)
            .into_iter()
            .partition(|w| w.key == key);
        self.waiting = rest;
        for w in mine {
            self.send(Response::new_err(
                w.req.id,
                ErrorCode::InternalError as i32,
                msg.to_string(),
            ));
        }
    }

    /// The result of a check (design D4): published as a parse's would be, but with the whole front end's
    /// diagnostics, and answered with their counts. A check that is no longer the program's last (a newer one
    /// came, it was cancelled, or the document closed) was answered already and publishes nothing. A check that
    /// failed is answered with the error, and the program is parsed again (a check takes the place of a pending
    /// parse), which answers the requests waiting for it.
    fn checked(&mut self, key: String, seq: u64, id: RequestId, result: Result<(Analysis, Diagnostics), String>) {
        if self.checks.get(&key).is_none_or(|(s, _)| *s != seq) {
            return;
        }
        self.checks.remove(&key);
        let (analysis, diagnostics) = match result {
            Ok(r) => r,
            Err(msg) => {
                eprintln!("qb64rust: {msg} ({key})");
                self.send(Response::new_err(id, ErrorCode::InternalError as i32, msg));
                self.pending.insert(key, Instant::now());
                return;
            }
        };
        // In the CLI's order (`Frontend::render`): by file, then by offset.
        let mut list = diagnostics.list().to_vec();
        list.sort_by_key(|d| (d.span.file.0, d.span.start));
        let warnings = list.iter().filter(|d| d.severity == Severity::Warning).count();
        let per_file = analysis.lsp_diagnostics(&list);
        self.publish_program(key, analysis, per_file);
        let answer = serde_json::json!({ "errors": diagnostics.error_count(), "warnings": warnings });
        self.send(Response::new_ok(id, answer));
    }

    /// Publishes `diagnostics` for each file of program `key`'s new `analysis`, which replaces its last one, and
    /// answers the requests waiting for it.
    fn publish_program(
        &mut self,
        key: String,
        analysis: Analysis,
        diagnostics: Vec<(FileId, Vec<lsp_types::Diagnostic>)>,
    ) {
        // Diagnostics of every file the parse covered; files the program's previous parse covered and this one
        // does not lose the ones it published (unless another program covers them).
        let covered: Vec<String> = analysis.files.iter().map(|f| f.key.clone()).collect();
        if let Some(old) = self.analyses.get(&key) {
            let gone: Vec<(String, Uri)> = old
                .files
                .iter()
                .filter(|f| !covered.contains(&f.key))
                .filter(|f| self.published.get(&f.key).is_some_and(|(p, _)| *p == key))
                .map(|f| (f.key.clone(), f.uri.clone()))
                .collect();
            for (file, uri) in gone {
                self.release(file, uri, &key);
            }
        }
        for (file, diags) in diagnostics {
            let info = analysis.info(file);
            self.published.insert(info.key.clone(), (key.clone(), info.uri.clone()));
            self.publish(info.uri.clone(), diags);
        }
        self.analyses.insert(key.clone(), analysis);
        let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.waiting)
            .into_iter()
            .partition(|w| w.key == key);
        self.waiting = rest;
        for w in mine {
            let a = self.analyses.get(&key);
            self.answer(w.req, a);
        }
    }

    fn publish(&self, uri: Uri, diagnostics: Vec<lsp_types::Diagnostic>) {
        let params = PublishDiagnosticsParams::new(uri, diagnostics, None);
        self.send(Notification::new(PublishDiagnostics::METHOD.into(), params));
    }

    fn request(&mut self, req: Request) {
        if self.shutdown {
            self.send(Response::new_err(
                req.id,
                ErrorCode::InvalidRequest as i32,
                "shut down".into(),
            ));
            return;
        }
        let uri = match req.method.as_str() {
            Shutdown::METHOD => {
                self.shutdown = true;
                self.send(Response::new_ok(req.id, ()));
                return;
            }
            CHECK => {
                self.check(req);
                return;
            }
            Formatting::METHOD | RangeFormatting::METHOD => {
                self.format_request(req);
                return;
            }
            DocumentSymbolRequest::METHOD => {
                serde_json::from_value::<DocumentSymbolParams>(req.params.clone()).map(|p| p.text_document.uri)
            }
            FoldingRangeRequest::METHOD => {
                serde_json::from_value::<FoldingRangeParams>(req.params.clone()).map(|p| p.text_document.uri)
            }
            GotoDefinition::METHOD => serde_json::from_value::<GotoDefinitionParams>(req.params.clone())
                .map(|p| p.text_document_position_params.text_document.uri),
            _ => {
                let msg = format!("unknown method {}", req.method);
                self.send(Response::new_err(req.id, ErrorCode::MethodNotFound as i32, msg));
                return;
            }
        };
        let uri = match uri {
            Ok(u) => u,
            Err(e) => {
                self.send(Response::new_err(
                    req.id,
                    ErrorCode::InvalidParams as i32,
                    e.to_string(),
                ));
                return;
            }
        };
        let key = uri_key(&uri);
        match self.analyses.get(&key) {
            Some(a) => self.answer(req, Some(a)),
            // The first parse (or a check, which parses) is still to come: the request waits for it. (After a parse
            // that failed, none comes until the next change.)
            None if self.docs.contains_key(&key)
                && (self.pending.contains_key(&key)
                    || self.latest_job.contains_key(&key)
                    || self.checks.contains_key(&key)) =>
            {
                self.waiting.push(Waiting { key, req });
            }
            None => self.answer(req, None),
        }
    }

    /// `qb64rust/check { uri }` (design D3): a job that parses the document's program and runs the front end. A
    /// newer check of the same program replaces an older one, which is answered at once with `ContentModified`.
    fn check(&mut self, req: Request) {
        let uri = field(&req.params, "uri").and_then(|u| Uri::from_str(u).ok());
        let Some(key) = uri.map(|u| uri_key(&u)).filter(|k| self.docs.contains_key(k)) else {
            let msg = "qb64rust/check needs the uri of an open document".to_string();
            self.send(Response::new_err(req.id, ErrorCode::InvalidParams as i32, msg));
            return;
        };
        if let Some((_, older)) = self.checks.remove(&key) {
            let code = ErrorCode::ContentModified as i32;
            self.send(Response::new_err(older, code, "a newer check replaced it".into()));
        }
        // The check parses the program as it is now: a parse of it still to come after the debounce would only
        // publish the parser's diagnostics over the check's.
        self.pending.remove(&key);
        let open: Arc<HashMap<String, OpenFile>> =
            Arc::new(self.docs.iter().map(|(k, d)| (k.clone(), open_file(d))).collect());
        self.seq += 1;
        self.checks.insert(key.clone(), (self.seq, req.id.clone()));
        let job = Job {
            main: open_file(&self.docs[&key]),
            key,
            seq: self.seq,
            open,
            root: self.root.clone(),
            default: self.default,
            check: Some(req.id),
            format: None,
        };
        // The worker is gone (it never panics out of its loop): the check stays unanswered.
        let _ = self.jobs.send(job);
    }

    /// `textDocument/formatting` and `textDocument/rangeFormatting` (design D5 of `m5b-formatter`): a job on the
    /// worker that parses the document's program as a check does and formats it with the client's settings; the
    /// request's own `FormattingOptions` are not read.
    fn format_request(&mut self, req: Request) {
        let parsed = if req.method == Formatting::METHOD {
            serde_json::from_value::<DocumentFormattingParams>(req.params.clone()).map(|p| (p.text_document.uri, None))
        } else {
            serde_json::from_value::<DocumentRangeFormattingParams>(req.params.clone())
                .map(|p| (p.text_document.uri, Some(p.range)))
        };
        let (uri, range) = match parsed {
            Ok(p) => p,
            Err(e) => {
                let code = ErrorCode::InvalidParams as i32;
                self.send(Response::new_err(req.id, code, e.to_string()));
                return;
            }
        };
        let key = uri_key(&uri);
        let Some(doc) = self.docs.get(&key) else {
            let msg = "formatting needs an open document".to_string();
            self.send(Response::new_err(req.id, ErrorCode::InvalidParams as i32, msg));
            return;
        };
        let open: Arc<HashMap<String, OpenFile>> =
            Arc::new(self.docs.iter().map(|(k, d)| (k.clone(), open_file(d))).collect());
        self.seq += 1;
        let job = Job {
            main: open_file(doc),
            key,
            seq: self.seq,
            open,
            root: self.root.clone(),
            default: self.default,
            check: None,
            format: Some(FormatRequest {
                id: req.id,
                range,
                settings: self.format,
                version: doc.version,
            }),
        };
        let _ = self.jobs.send(job);
    }

    /// The result of a formatting request. An answer for a text that changed since the request is `ContentModified`.
    /// On success: one edit of the whole document, or one per changed line of the range, each line keeping its own
    /// line end; none when nothing changes. When the program has an error: its diagnostics published as a check's,
    /// a message naming the first, and no edits.
    fn formatted(
        &mut self,
        key: &str,
        request: FormatRequest,
        result: Result<(Analysis, Result<Vec<u8>, Diagnostics>), String>,
    ) {
        let Some(doc) = self.docs.get(key).filter(|d| d.version == request.version) else {
            let code = ErrorCode::ContentModified as i32;
            self.send(Response::new_err(request.id, code, "the document changed".into()));
            return;
        };
        let (analysis, text) = match result {
            Ok(r) => r,
            Err(msg) => {
                eprintln!("qb64rust: {msg} ({key})");
                self.send(Response::new_err(request.id, ErrorCode::InternalError as i32, msg));
                return;
            }
        };
        match text {
            Ok(bytes) => {
                let lines = formatted_lines(&bytes, doc.encoding);
                let edits = line_edits(&doc.text, &lines, request.range);
                self.send(Response::new_ok(request.id, edits));
            }
            Err(diagnostics) => {
                let mut list = diagnostics.list().to_vec();
                list.sort_by_key(|d| (d.span.file.0, d.span.start));
                let first = list.iter().find(|d| d.severity == Severity::Error).map(|d| {
                    let file = analysis.map.file(d.span.file);
                    let (line, _) = file.line_col(d.span.start);
                    let name = to_path(&analysis.info(d.span.file).uri)
                        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
                        .unwrap_or_else(|| file.name.clone());
                    format!("{name}, line {line}: {}", d.message)
                });
                let per_file = analysis.lsp_diagnostics(&list);
                self.publish_program(key.to_string(), analysis, per_file);
                if let Some(first) = first {
                    let params = ShowMessageParams {
                        typ: MessageType::ERROR,
                        message: format!("The program was not formatted: {first}"),
                    };
                    self.send(Notification::new(ShowMessage::METHOD.into(), params));
                }
                self.send(Response::new_ok(request.id, Vec::<TextEdit>::new()));
            }
        }
    }

    /// Answers a request from a parse of its document, or with an empty result without one.
    fn answer(&self, req: Request, a: Option<&Analysis>) {
        let result = match req.method.as_str() {
            DocumentSymbolRequest::METHOD => {
                let symbols = a.map(symbols::document_symbols).unwrap_or_default();
                serde_json::to_value(DocumentSymbolResponse::Nested(symbols))
            }
            FoldingRangeRequest::METHOD => serde_json::to_value(a.map(folding::folding_ranges).unwrap_or_default()),
            GotoDefinition::METHOD => {
                let found = a.and_then(|a| {
                    let p: GotoDefinitionParams = serde_json::from_value(req.params.clone()).ok()?;
                    let main = a.parsed.main().file;
                    let pos = p.text_document_position_params.position;
                    let offset = a.info(main).columns.offset(a.map.file(main), pos);
                    let span = definition::definition(a, offset)?;
                    let uri = a.info(span.file).uri.clone();
                    Some(GotoDefinitionResponse::Scalar(Location::new(uri, a.range(span))))
                });
                serde_json::to_value(found)
            }
            _ => unreachable!("only requests about a document are answered here"),
        };
        match result {
            Ok(v) => self.send(Response {
                id: req.id,
                response_result: Ok(v),
            }),
            Err(e) => self.send(Response::new_err(
                req.id,
                ErrorCode::InternalError as i32,
                e.to_string(),
            )),
        }
    }
}

/// The lines of the formatter's output (every line followed by CR LF) as text in the document's encoding.
fn formatted_lines(bytes: &[u8], encoding: Encoding) -> Vec<String> {
    let mut lines: Vec<&[u8]> = Vec::new();
    let mut rest = bytes;
    while let Some(i) = rest.windows(2).position(|w| w == b"\r\n") {
        lines.push(&rest[..i]);
        rest = &rest[i + 2..];
    }
    if !rest.is_empty() {
        lines.push(rest);
    }
    lines.into_iter().map(|l| decode(l, encoding)).collect()
}

/// Bytes in the document's encoding as text: a table's characters, or UTF-8 as it is (the formatter changes the case
/// of ASCII letters only, so the bytes stay UTF-8 where the document was).
#[expect(
    clippy::disallowed_methods,
    reason = "a UTF-8 document's bytes back to the editor's text"
)]
fn decode(bytes: &[u8], encoding: Encoding) -> String {
    encoding
        .decode(bytes)
        .unwrap_or_else(|| String::from_utf8_lossy(bytes).into_owned())
}

/// The lines of a document's text, each with its line end (CR LF, LF CR, LF or CR, as the formatter reads them;
/// none for a last line without one).
fn text_lines(text: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut start = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\r' || b[i] == b'\n' {
            let pair = if b[i] == b'\r' { b'\n' } else { b'\r' };
            let end = if b.get(i + 1) == Some(&pair) { i + 2 } else { i + 1 };
            out.push((&text[start..i], &text[i..end]));
            start = end;
            i = end;
        } else {
            i += 1;
        }
    }
    if start < b.len() {
        out.push((&text[start..], ""));
    }
    out
}

/// The edits that turn the document's text into the formatted lines: the whole document in one, or for a range each
/// line it touches that changed (spec `editor/language-server`, "Range formatting"). A range that ends at the start
/// of a line does not touch that line.
fn line_edits(text: &str, formatted: &[String], range: Option<Range>) -> Vec<TextEdit> {
    let lines = text_lines(text);
    let utf16 = |s: &str| u32::try_from(s.encode_utf16().count()).unwrap_or(u32::MAX);
    let Some(range) = range else {
        let mut new = String::with_capacity(text.len());
        for (i, (old, end)) in lines.iter().enumerate() {
            new.push_str(formatted.get(i).map_or(*old, String::as_str));
            new.push_str(end);
        }
        if new == text {
            return Vec::new();
        }
        // The end of the text: after its last line, or at the start of the line after its last line end.
        let end = match lines.last() {
            Some((last, "")) => Position::new(qb64rust_base::to_u32(lines.len() - 1), utf16(last)),
            _ => Position::new(qb64rust_base::to_u32(lines.len()), 0),
        };
        return vec![TextEdit::new(Range::new(Position::new(0, 0), end), new)];
    };
    let last = if range.end.character == 0 && range.end.line > range.start.line {
        range.end.line - 1
    } else {
        range.end.line
    };
    let mut edits = Vec::new();
    for line in range.start.line..=last {
        let i = line as usize;
        let (Some((old, _)), Some(new)) = (lines.get(i), formatted.get(i)) else {
            continue;
        };
        if old != new {
            let at = Range::new(Position::new(line, 0), Position::new(line, utf16(old)));
            edits.push(TextEdit::new(at, new.clone()));
        }
    }
    edits
}

fn open_file(d: &Doc) -> OpenFile {
    OpenFile {
        uri: d.uri.clone(),
        bytes: Arc::clone(&d.bytes),
        encoding: d.encoding,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A server without a worker: the test takes its jobs and gives their results.
    fn server() -> (Server, Connection, mpsc::Receiver<Job>) {
        let (server_side, client) = Connection::memory();
        let (jobs, queue) = mpsc::channel();
        let s = Server::new(server_side, jobs, PathBuf::new(), Encoding::DEFAULT);
        (s, client, queue)
    }

    fn response(client: &Connection) -> Response {
        match client.receiver.try_recv() {
            Ok(Message::Response(r)) => r,
            other => panic!("expected a response, got {other:?}"),
        }
    }

    #[test]
    fn request_after_a_failed_parse_is_answered() {
        let (mut s, client, queue) = server();
        let uri = "untitled:Untitled-1";
        let doc = json!({ "uri": uri, "languageId": "qb64rust", "version": 1, "text": "SUB a\nEND SUB\n" });
        let open = json!({ "textDocument": doc });
        s.notification(Notification::new(DidOpenTextDocument::METHOD.into(), open));
        // The debounce is over: the parse is sent, and a request waits for it.
        for at in s.pending.values_mut() {
            *at = Instant::now();
        }
        s.dispatch_due();
        let job = queue.try_recv().unwrap();
        let symbols = |id: i32| {
            let params = json!({ "textDocument": { "uri": uri } });
            Request::new(id.into(), DocumentSymbolRequest::METHOD.into(), params)
        };
        s.request(symbols(1));
        assert!(client.receiver.try_recv().is_err(), "answered before the parse");
        // The parse fails: the waiting request gets the error ...
        s.parsed(job.key, job.seq, Err("internal error while parsing: test".into()));
        let code = response(&client).response_result.unwrap_err().code;
        assert_eq!(code, ErrorCode::InternalError as i32);
        // ... and a later one an empty answer at once, not a wait for a parse that does not come.
        s.request(symbols(2));
        assert_eq!(response(&client).response_result.unwrap(), json!([]));
    }
}
