//! The message loop (design D2, D5): the document store, a parse of each changed program after a debounce on a
//! worker thread, diagnostics per file, and requests answered from the last parse of their document. A check
//! (`qb64rust/check`, design D3 of `extension-diagnostics`) is a parse followed by the rest of the front end, on
//! the same worker.

use crate::analysis::{Analysis, OpenFile, analyze};
use crate::encoding::Encoding;
use crate::index::{Indexed, SymbolIndex};
use crate::uri::{to_path, uri_key};
use crate::{Hooks, completion, definition, folding, hover, rename, symbols};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    Cancel, DidChangeConfiguration, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
    Notification as _, PublishDiagnostics, ShowMessage,
};
use lsp_types::request::{
    Completion, DocumentSymbolRequest, FoldingRangeRequest, Formatting, GotoDefinition, HoverRequest,
    PrepareRenameRequest, RangeFormatting, References, Rename, Request as _, Shutdown,
};
use lsp_types::{
    CancelParams, CompletionList, CompletionOptions, CompletionParams, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DocumentFormattingParams, DocumentRangeFormattingParams,
    DocumentSymbolParams, DocumentSymbolResponse, FoldingRangeParams, GotoDefinitionParams, GotoDefinitionResponse,
    HoverParams, HoverProviderCapability, InitializeParams, Location, MessageType, NumberOrString, Position,
    PrepareRenameResponse, PublishDiagnosticsParams, Range, ReferenceParams, RenameOptions, RenameParams,
    ServerCapabilities, ShowMessageParams, TextDocumentPositionParams, TextDocumentSyncKind, TextEdit, Uri,
    WorkspaceEdit,
};
use qb64rust_base::layout::{KeywordStyle, Settings};
use qb64rust_base::{Diagnostics, FileId, Severity};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        references_provider: Some(lsp_types::OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![".".into()]),
            ..CompletionOptions::default()
        }),
        rename_provider: Some(lsp_types::OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: lsp_types::WorkDoneProgressOptions::default(),
        })),
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
    /// The semantic index of a program (design D2 of `m5b-language-features`): its parse and the index.
    Indexed {
        key: String,
        seq: u64,
        result: Result<Indexed, String>,
    },
    /// The index of a program with a rename's edit applied, for the check of the edit (design D5).
    RenameChecked {
        id: RequestId,
        result: Result<SymbolIndex, String>,
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
    /// The job builds the program's semantic index.
    index: bool,
    /// The job builds the index of the program with this rename request's edit applied (in `main` and `open`).
    rename: Option<RequestId>,
    /// For an index or a rename's job: cleared when its result would be dropped (the text changed, or the rename
    /// was cancelled), so that the worker skips the job if it has not started it.
    live: Option<Arc<AtomicBool>>,
}

/// A rename that passed the index's checks: the renamed program's main file and open files, for the job that
/// checks the edit.
struct PendingRename {
    main: OpenFile,
    open: HashMap<String, OpenFile>,
    check: RenameCheck,
}

/// A rename whose edit is being checked: the request, its program, and what it will answer.
struct RenameCheck {
    id: RequestId,
    key: String,
    plan: rename::Plan,
    edit: WorkspaceEdit,
    /// Its job's [`Job::live`].
    live: Arc<AtomicBool>,
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
                // An index or a rename's check whose result the server no longer wants (design D2 of
                // `m5b-language-features`): a burst of edits and requests leaves at most one such job running.
                if job.live.as_ref().is_some_and(|l| !l.load(Ordering::SeqCst)) {
                    continue;
                }
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
                if let Some(id) = &job.rename {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let analysis = parse();
                        let path = to_path(&job.main.uri);
                        (hooks.index)(&analysis.map, &analysis.parsed, path.as_deref(), &job.root)
                    }))
                    .map_err(|e| panic_message(&*e, "checking a rename"));
                    if events.send(Event::RenameChecked { id: id.clone(), result }).is_err() {
                        return;
                    }
                    continue;
                }
                if job.index {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let analysis = parse();
                        let path = to_path(&job.main.uri);
                        let index = (hooks.index)(&analysis.map, &analysis.parsed, path.as_deref(), &job.root);
                        Indexed { analysis, index }
                    }))
                    .map_err(|e| panic_message(&*e, "indexing"));
                    let event = Event::Indexed {
                        key: job.key.clone(),
                        seq: job.seq,
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
    /// The semantic index of each program (design D2 of `m5b-language-features`), and whether it is of the
    /// program's text as it is now; one of an older text stays for completion (spec "Completion": the last index).
    indexes: HashMap<String, (bool, Indexed)>,
    /// The index job under way for each program, by its number, and its [`Job::live`]; the result of any other is
    /// dropped.
    index_jobs: HashMap<String, (u64, Arc<AtomicBool>)>,
    /// Requests that wait for their program's index.
    index_waiting: Vec<Waiting>,
    /// Renames whose edit is being checked (design D5).
    renames: Vec<RenameCheck>,
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
            indexes: HashMap::new(),
            index_jobs: HashMap::new(),
            index_waiting: Vec::new(),
            renames: Vec::new(),
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
                Some(Event::Indexed { key, seq, result }) => self.indexed(key, seq, result),
                Some(Event::RenameChecked { id, result }) => self.rename_checked(id, result),
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
                    // A request waiting for an index: the index job goes on, for the next request. A rename whose
                    // edit is being checked: its job is skipped if the worker has not started it.
                    if let Some(i) = self.index_waiting.iter().position(|w| w.req.id == id) {
                        let w = self.index_waiting.remove(i);
                        self.send(Response::new_err(w.req.id, code, "cancelled".into()));
                    }
                    if let Some(i) = self.renames.iter().position(|r| r.id == id) {
                        let r = self.renames.remove(i);
                        r.live.store(false, Ordering::SeqCst);
                        self.send(Response::new_err(r.id, code, "cancelled".into()));
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
        self.stale_index(key);
        let covers = |files: &[crate::analysis::FileInfo]| files.iter().any(|f| f.key == key);
        let mut dependents: Vec<String> = self
            .analyses
            .iter()
            .filter(|(k, a)| k.as_str() != key && covers(&a.files))
            .map(|(k, _)| k.clone())
            .collect();
        dependents.extend(
            self.indexes
                .iter()
                .filter(|(k, (_, ix))| k.as_str() != key && covers(&ix.analysis.files))
                .map(|(k, _)| k.clone()),
        );
        for k in dependents {
            self.stale_index(&k);
            self.pending.insert(k, at);
        }
    }

    /// Program `key`'s text changed: its index is of an older text, an index job under way is dropped, and the
    /// requests waiting for one are answered with `ContentModified` (spec "Semantic index until the next change").
    fn stale_index(&mut self, key: &str) {
        if let Some((current, _)) = self.indexes.get_mut(key) {
            *current = false;
        }
        if let Some((_, live)) = self.index_jobs.remove(key) {
            live.store(false, Ordering::SeqCst);
        }
        let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.index_waiting)
            .into_iter()
            .partition(|w| w.key == key);
        self.index_waiting = rest;
        let (checks, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.renames)
            .into_iter()
            .partition(|r| r.key == key);
        self.renames = rest;
        let code = ErrorCode::ContentModified as i32;
        for id in mine.into_iter().map(|w| w.req.id).chain(checks.into_iter().map(|r| {
            r.live.store(false, Ordering::SeqCst);
            r.id
        })) {
            self.send(Response::new_err(id, code, "the document changed".into()));
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
        self.indexes.remove(key);
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
                index: false,
                rename: None,
                live: None,
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
            self.answer_parsed(w.req, &key);
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
            Completion::METHOD => {
                self.completion(req);
                return;
            }
            HoverRequest::METHOD | References::METHOD | PrepareRenameRequest::METHOD | Rename::METHOD => {
                // Each names the document as a `TextDocumentPositionParams` does.
                match serde_json::from_value::<lsp_types::TextDocumentPositionParams>(req.params.clone()) {
                    Ok(p) => self.need_index(req, uri_key(&p.text_document.uri)),
                    Err(e) => self.send(Response::new_err(
                        req.id,
                        ErrorCode::InvalidParams as i32,
                        e.to_string(),
                    )),
                }
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
            Some(_) => self.answer_parsed(req, &key),
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
            index: false,
            rename: None,
            live: None,
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
            index: false,
            rename: None,
            live: None,
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

    /// Answers a request from the last parse of program `key`. A definition the parser does not find (a name other
    /// than a procedure or a label) is taken from the index (design D4 of `m5b-language-features`).
    fn answer_parsed(&mut self, req: Request, key: &str) {
        if req.method == GotoDefinition::METHOD
            && self.docs.contains_key(key)
            && self
                .analyses
                .get(key)
                .is_some_and(|a| parser_definition(a, &req).is_none())
        {
            self.need_index(req, key.to_string());
            return;
        }
        let a = self.analyses.get(key);
        self.answer(req, a);
    }

    /// Answers a request from program `key`'s index: at once with an index of the current text, else when the
    /// index job (sent unless one is under way) comes back (design D2 of `m5b-language-features`). A document that
    /// is not open gets an empty answer.
    fn need_index(&mut self, req: Request, key: String) {
        if let Some((true, ix)) = self.indexes.get(&key) {
            let rename = self.answer_indexed(req, Some(ix));
            self.check_rename(key, rename);
            return;
        }
        if !self.docs.contains_key(&key) {
            self.answer_indexed(req, None);
            return;
        }
        self.start_index(&key);
        self.index_waiting.push(Waiting { key, req });
    }

    /// Sends the job that builds open program `key`'s index, unless one is under way.
    fn start_index(&mut self, key: &str) {
        let Some(doc) = self.docs.get(key) else {
            return;
        };
        if self.index_jobs.contains_key(key) {
            return;
        }
        let open: Arc<HashMap<String, OpenFile>> =
            Arc::new(self.docs.iter().map(|(k, d)| (k.clone(), open_file(d))).collect());
        self.seq += 1;
        let live = Arc::new(AtomicBool::new(true));
        self.index_jobs.insert(key.to_string(), (self.seq, Arc::clone(&live)));
        let job = Job {
            main: open_file(doc),
            key: key.to_string(),
            seq: self.seq,
            open,
            root: self.root.clone(),
            default: self.default,
            check: None,
            format: None,
            index: true,
            rename: None,
            live: Some(live),
        };
        // The worker is gone (it never panics out of its loop): requests waiting for it stay unanswered.
        let _ = self.jobs.send(job);
    }

    /// `textDocument/completion` (design D6): answered at once, from the program's index of the current text or the
    /// last one (spec "Completion"); without a current one, its job is started for the next request.
    fn completion(&mut self, req: Request) {
        let p = match serde_json::from_value::<CompletionParams>(req.params) {
            Ok(p) => p,
            Err(e) => {
                self.send(Response::new_err(
                    req.id,
                    ErrorCode::InvalidParams as i32,
                    e.to_string(),
                ));
                return;
            }
        };
        let pos = p.text_document_position.position;
        let key = uri_key(&p.text_document_position.text_document.uri);
        let Some(doc) = self.docs.get(&key) else {
            self.send(Response::new_ok(req.id, CompletionList::default()));
            return;
        };
        let before = line_before(&doc.text, pos);
        let ix = self.indexes.get(&key).map(|(_, ix)| ix);
        let offset = ix.map(|ix| {
            let main = ix.analysis.parsed.main().file;
            ix.analysis.info(main).columns.offset(ix.analysis.map.file(main), pos)
        });
        let items = completion::items(ix, offset, &before);
        self.send(Response::new_ok(
            req.id,
            CompletionList {
                is_incomplete: false,
                items,
            },
        ));
        if !self.indexes.get(&key).is_some_and(|(current, _)| *current) {
            self.start_index(&key);
        }
    }

    /// The result of an index job: kept as the program's index and given to the requests waiting for it. The
    /// result of a job that is no longer the program's (its text changed, or it closed) is dropped.
    fn indexed(&mut self, key: String, seq: u64, result: Result<Indexed, String>) {
        if self.index_jobs.get(&key).map(|(s, _)| *s) != Some(seq) {
            return;
        }
        self.index_jobs.remove(&key);
        let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.index_waiting)
            .into_iter()
            .partition(|w| w.key == key);
        self.index_waiting = rest;
        match result {
            Ok(ix) => {
                self.indexes.insert(key.clone(), (true, ix));
                for w in mine {
                    let ix = self.indexes.get(&key).map(|(_, ix)| ix);
                    let rename = self.answer_indexed(w.req, ix);
                    self.check_rename(key.clone(), rename);
                }
            }
            Err(msg) => {
                eprintln!("qb64rust: {msg} ({key})");
                for w in mine {
                    let code = ErrorCode::InternalError as i32;
                    self.send(Response::new_err(w.req.id, code, msg.clone()));
                }
            }
        }
    }

    /// Answers a request from an index of its program's current text, or with an empty result without one. A rename
    /// that passes the checks of the index is not answered yet: its edit is given back, to be checked (design D5).
    fn answer_indexed(&self, req: Request, ix: Option<&Indexed>) -> Option<PendingRename> {
        let position = |p: Position| {
            let ix = ix?;
            let main = ix.analysis.parsed.main().file;
            Some((ix, ix.analysis.info(main).columns.offset(ix.analysis.map.file(main), p)))
        };
        let refuse = |id: RequestId, msg: String| {
            self.send(Response::new_err(id, ErrorCode::RequestFailed as i32, msg));
            None
        };
        let result = match req.method.as_str() {
            PrepareRenameRequest::METHOD => match serde_json::from_value::<TextDocumentPositionParams>(req.params) {
                Ok(p) => match position(p.position).map(|(ix, at)| (ix, rename::prepare(ix, at))) {
                    None => Ok(Value::Null),
                    Some((ix, Ok((_, span)))) => {
                        serde_json::to_value(PrepareRenameResponse::Range(ix.analysis.range(span)))
                    }
                    Some((_, Err(msg))) => return refuse(req.id, msg),
                },
                Err(e) => Err(e),
            },
            Rename::METHOD => match serde_json::from_value::<RenameParams>(req.params) {
                Ok(p) => match position(p.text_document_position.position) {
                    None => Ok(Value::Null),
                    Some((ix, at)) => match rename::plan(ix, at, &p.new_name, &self.root) {
                        Ok(plan) => return Some(self.pending_rename(req.id, ix, plan)),
                        Err(msg) => return refuse(req.id, msg),
                    },
                },
                Err(e) => Err(e),
            },
            HoverRequest::METHOD => serde_json::from_value::<HoverParams>(req.params.clone())
                .map(|p| position(p.text_document_position_params.position).and_then(|(ix, at)| hover::hover(ix, at)))
                .and_then(serde_json::to_value),
            GotoDefinition::METHOD => serde_json::from_value::<GotoDefinitionParams>(req.params.clone())
                .map(|p| {
                    position(p.text_document_position_params.position).and_then(|(ix, at)| {
                        let main = ix.analysis.parsed.main().file;
                        let (id, _) = ix.index.entry_near(main, at)?;
                        let span = ix.index.entries[id].def;
                        let uri = ix.analysis.info(span.file).uri.clone();
                        Some(GotoDefinitionResponse::Scalar(Location::new(
                            uri,
                            ix.analysis.range(span),
                        )))
                    })
                })
                .and_then(serde_json::to_value),
            References::METHOD => serde_json::from_value::<ReferenceParams>(req.params.clone())
                .map(|p| {
                    let found = position(p.text_document_position.position);
                    found.map_or_else(Vec::new, |(ix, at)| {
                        definition::references(ix, at, p.context.include_declaration)
                    })
                })
                .and_then(serde_json::to_value),
            _ => unreachable!("only requests answered from an index come here"),
        };
        match result {
            Ok(v) => self.send(Response::new_ok(req.id, v)),
            Err(e) => self.send(Response::new_err(
                req.id,
                ErrorCode::InvalidParams as i32,
                e.to_string(),
            )),
        }
        None
    }

    /// A rename's edit, and the program's files with it applied: each file it changes, in the encoding the file was
    /// read in, over the open documents.
    #[expect(
        clippy::mutable_key_type,
        reason = "lsp_types' WorkspaceEdit keys its changes by Uri"
    )]
    fn pending_rename(&self, id: RequestId, ix: &Indexed, plan: rename::Plan) -> PendingRename {
        let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();
        for &s in &plan.edits {
            let uri = ix.analysis.info(s.file).uri.clone();
            let edit = TextEdit::new(ix.analysis.range(s), plan.new_name.clone());
            changes.entry(uri).or_default().push(edit);
        }
        let mut open: HashMap<String, OpenFile> = self.docs.iter().map(|(k, d)| (k.clone(), open_file(d))).collect();
        let mut files: Vec<FileId> = plan.edits.iter().map(|s| s.file).collect();
        files.sort_unstable_by_key(|f| f.0);
        files.dedup();
        for file in files {
            let info = ix.analysis.info(file);
            let encoding = self.docs.get(&info.key).map_or(self.default, |d| d.encoding);
            let bytes = rename::renamed_bytes(ix, &plan, file);
            let renamed = OpenFile {
                uri: info.uri.clone(),
                bytes: bytes.into(),
                encoding,
            };
            open.insert(info.key.clone(), renamed);
        }
        let main = ix.analysis.info(ix.analysis.parsed.main().file);
        PendingRename {
            main: open[&main.key].clone(),
            open,
            check: RenameCheck {
                id,
                key: String::new(),
                plan,
                edit: WorkspaceEdit::new(changes),
                live: Arc::new(AtomicBool::new(true)),
            },
        }
    }

    /// Sends the job that indexes the renamed program of a rename that passed the index's checks.
    fn check_rename(&mut self, key: String, rename: Option<PendingRename>) {
        let Some(PendingRename { main, open, mut check }) = rename else {
            return;
        };
        self.seq += 1;
        let job = Job {
            key: key.clone(),
            seq: self.seq,
            main,
            open: Arc::new(open),
            root: self.root.clone(),
            default: self.default,
            check: None,
            format: None,
            index: false,
            rename: Some(check.id.clone()),
            live: Some(Arc::clone(&check.live)),
        };
        check.key = key;
        self.renames.push(check);
        // The worker is gone (it never panics out of its loop): the request stays unanswered.
        let _ = self.jobs.send(job);
    }

    /// The index of a renamed program (design D5): the edit is given if the index has the program's symbols with the
    /// symbol renamed, and refused otherwise. A rename whose program changed meanwhile was answered already.
    fn rename_checked(&mut self, id: RequestId, result: Result<SymbolIndex, String>) {
        let Some(i) = self.renames.iter().position(|r| r.id == id) else {
            return;
        };
        let check = self.renames.remove(i);
        let Some((true, old)) = self.indexes.get(&check.key) else {
            let code = ErrorCode::ContentModified as i32;
            self.send(Response::new_err(id, code, "the document changed".into()));
            return;
        };
        let verdict = result.and_then(|new| rename::verify(&old.index, &check.plan, &new));
        match verdict {
            Ok(()) => {
                if check.plan.inactive {
                    let params = ShowMessageParams {
                        typ: MessageType::INFO,
                        message: "Code in `$IF` branches that are not compiled was not renamed.".into(),
                    };
                    self.send(Notification::new(ShowMessage::METHOD.into(), params));
                }
                self.send(Response::new_ok(id, check.edit));
            }
            Err(msg) => self.send(Response::new_err(id, ErrorCode::RequestFailed as i32, msg)),
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
                let found = a.and_then(|a| parser_definition(a, &req));
                serde_json::to_value(found.map(GotoDefinitionResponse::Scalar))
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

/// The text of `pos`'s line before it (its column counted in UTF-16 units, as the client counts).
fn line_before(text: &str, pos: Position) -> String {
    let line = text.split('\n').nth(pos.line as usize).unwrap_or("");
    let mut units = 0;
    line.chars()
        .take_while(|c| {
            units += c.len_utf16();
            units <= pos.character as usize
        })
        .collect()
}

/// The parser's definition of the name at a definition request's position: a procedure or a label (design D4).
fn parser_definition(a: &Analysis, req: &Request) -> Option<Location> {
    let p: GotoDefinitionParams = serde_json::from_value(req.params.clone()).ok()?;
    let main = a.parsed.main().file;
    let pos = p.text_document_position_params.position;
    let offset = a.info(main).columns.offset(a.map.file(main), pos);
    let span = definition::definition(a, offset)?;
    let uri = a.info(span.file).uri.clone();
    Some(Location::new(uri, a.range(span)))
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
