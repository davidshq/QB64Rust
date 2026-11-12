//! The message loop (design D2, D5): the document store, a parse of each changed program after a debounce on a
//! worker thread, diagnostics per file, and requests answered from the last parse of their document. A check
//! (`qb64rust/check`, design D3 of `extension-diagnostics`) is a parse followed by the rest of the front end, on
//! the same worker.

use crate::analysis::{Analysis, OpenFile, analyze};
use crate::encoding::Encoding;
use crate::index::{Indexed, SymbolIndex};
use crate::owner::{self, Owner};
use crate::uri::{to_path, uri_key};
use crate::{Hooks, completion, definition, folding, hover, rename, symbols};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    Cancel, DidChangeConfiguration, DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument,
    DidOpenTextDocument, Notification as _, PublishDiagnostics, ShowMessage,
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
use std::collections::{HashMap, HashSet};
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

/// The notification that tells the client a document's program: `{ uri, program, source }` (spec
/// `editor/language-server`, "Program notification"; design D8 of `include-file-program`).
pub const PROGRAM: &str = "qb64rust/program";

/// The name of the file the fixes are set in (spec `compiler/fixes`), watched by the client.
pub const SETTINGS_FILE: &str = "qb64rust.toml";

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
    // The workspace folders bound the search for a document's program on disk (design D7 of
    // `include-file-program`).
    let folders = params
        .workspace_folders
        .unwrap_or_default()
        .iter()
        .filter_map(|f| to_path(&f.uri))
        .collect();
    let jobs = start_worker(events, hooks);
    let mut server = Server::new(conn, jobs, root, default);
    server.format = format;
    server.folders = folders;
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
        result: FormatResult,
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
    /// The program of document `doc` (design D7 of `include-file-program`): `None` when it is its own; and whether
    /// the settings file's `main` names the document.
    Owner {
        doc: String,
        seq: u64,
        result: Result<(Option<Owner>, bool), String>,
    },
    Disconnected,
}

/// What a formatting job gives back: its parse and the formatted file or the diagnostics that stop it, `None` when
/// the program no longer includes the document; or the message of a panic.
type FormatResult = Result<(Analysis, Option<Result<Vec<u8>, Diagnostics>>), String>;

/// A formatting request on its way: the document, whole or a range, the settings it is formatted with, and the
/// document's version when it was asked, so that an answer for an older text is refused.
#[derive(Clone)]
struct FormatRequest {
    id: RequestId,
    /// The key of the document to format, which may be an included file of the job's program.
    doc: String,
    range: Option<Range>,
    settings: Settings,
    version: u64,
}

/// What the lookup of a document's program needs from the server (design D7 of `include-file-program`).
struct LookupJob {
    /// The open `.bas` documents that are their own program, the one opened first first.
    programs: Vec<String>,
    folders: Vec<PathBuf>,
}

struct Job {
    /// The program's key: its main file's.
    key: String,
    /// The document the job is for: the program's main file for a parse, the document to format, the document
    /// whose program is looked up.
    doc: String,
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
    /// For a check: the program is an include file alone, whose check gives the parser's diagnostics only (design
    /// D9 of `include-file-program`).
    lone: bool,
    /// The job looks up the program of `doc`, which is `main`.
    lookup: Option<LookupJob>,
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
                if let Some(l) = &job.lookup {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let settings_main = to_path(&job.main.uri).and_then(|p| (hooks.main_of)(&p));
                        let named = settings_main
                            .as_deref()
                            .is_some_and(|m| crate::uri::path_key(m) == job.doc);
                        let owner = owner::find_owner(&owner::Lookup {
                            doc: &job.doc,
                            uri: &job.main.uri,
                            open: &job.open,
                            programs: &l.programs,
                            settings_main,
                            folders: &l.folders,
                            root: &job.root,
                            default: job.default,
                        });
                        (owner, named)
                    }))
                    .map_err(|e| panic_message(&*e, "looking up a document's program"));
                    let event = Event::Owner {
                        doc: job.doc.clone(),
                        seq: job.seq,
                        result,
                    };
                    if events.send(event).is_err() {
                        return;
                    }
                    continue;
                }
                let parse = || analyze(&job.key, &job.main, &job.open, &job.root, job.default);
                if let Some(request) = &job.format {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let a = parse();
                        let path = to_path(&job.main.uri);
                        // The document's own file, which may be an included file of the program (design D4 of
                        // `include-file-program`). `None` when the program no longer includes it: its lookup is
                        // still to come.
                        let format = hooks.format;
                        let text = a
                            .file_of(&request.doc)
                            .map(|file| format(&a.map, &a.parsed, path.as_deref(), &job.root, &request.settings, file));
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
                            // No disk checks for a document that is not a file (`untitled:`). An include file
                            // alone gets the parser's diagnostics only: the later stages' would be false.
                            let path = to_path(&job.main.uri);
                            let diagnostics = if job.lone {
                                a.parsed.diagnostics()
                            } else {
                                check(&a.map, &a.parsed, path.as_deref(), &job.root)
                            };
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
    /// The server's count of opens when it was opened: of two open programs that include a file, the one opened
    /// first is its program (design D7 of `include-file-program`).
    opened: u64,
}

/// A request that waits for the first parse (or the index) of its document's program.
struct Waiting {
    /// The program's key.
    key: String,
    /// The document the request names.
    doc: String,
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
    /// The program of each open document whose program is another file, by the document's key (design D5 of
    /// `include-file-program`). A document not in it is its own program. The maps keyed by program above
    /// (`analyses`, `pending`, `latest_job`, `checks`, `indexes`) may hold a program whose main file is not open.
    owners: HashMap<String, Owner>,
    /// The lookup under way for each document, by its job's number.
    lookups: HashMap<String, u64>,
    /// The documents whose program the client was told since they opened.
    announced: HashSet<String>,
    /// The open documents that the settings file's `main` names, by their last lookup: their own program whatever
    /// includes them, so a parse that covers one does not look it up again.
    named: HashSet<String>,
    /// The client's workspace folders.
    folders: Vec<PathBuf>,
    /// Counts the opens of documents (`Doc::opened`).
    opens: u64,
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
            owners: HashMap::new(),
            lookups: HashMap::new(),
            announced: HashSet::new(),
            named: HashSet::new(),
            folders: Vec::new(),
            opens: 0,
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
                Some(Event::Owner { doc, seq, result }) => self.owner_found(doc, seq, result),
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
                    self.opens += 1;
                    let doc = Doc {
                        uri: p.text_document.uri,
                        text: p.text_document.text,
                        encoding,
                        bytes,
                        version: self.versions,
                        opened: self.opens,
                    };
                    self.docs.insert(key.clone(), doc);
                    self.changed(&key);
                    // Its program is looked up (design D7 of `include-file-program`). A `.bas` document may be the
                    // program of the others: its first parse looks up again the documents it covers (`parsed`).
                    self.start_lookup(&key);
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
            // A `qb64rust.toml` created, changed or deleted (spec "Settings file changed"; design D3 of `fixes-switch`):
            // every open program is parsed again as after an edit, which drops its check's diagnostics and marks its
            // index stale; the next check and index read the file again. The server keeps no copy of the settings.
            DidChangeWatchedFiles::METHOD => {
                let settings = n
                    .params
                    .get("changes")
                    .and_then(Value::as_array)
                    .is_some_and(|changes| {
                        changes
                            .iter()
                            .any(|c| field(c, "uri").is_some_and(|u| u.rsplit('/').next() == Some(SETTINGS_FILE)))
                    });
                if settings {
                    let keys: Vec<String> = self.docs.keys().cloned().collect();
                    for key in keys {
                        self.changed(&key);
                    }
                    // Its `[project]` `main` may name another program (spec "The settings file changes").
                    self.lookup_all("");
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

    /// A document's text or encoding changed: parse its program, and every program whose last parse included it,
    /// after the debounce.
    fn changed(&mut self, key: &str) {
        let at = Instant::now() + DEBOUNCE;
        let program = self.program_of(key);
        self.pending.insert(program.clone(), at);
        self.stale_index(&program);
        let covers = |files: &[crate::analysis::FileInfo]| files.iter().any(|f| f.key == key);
        let mut dependents: Vec<String> = self
            .analyses
            .iter()
            .filter(|(k, a)| **k != program && covers(&a.files))
            .map(|(k, _)| k.clone())
            .collect();
        dependents.extend(
            self.indexes
                .iter()
                .filter(|(k, (_, ix))| **k != program && covers(&ix.analysis.files))
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
        self.lookups.remove(key);
        self.announced.remove(key);
        self.named.remove(key);
        if let Some(owner) = self.owners.get(key).cloned() {
            // A document of another program (design D5 of `include-file-program`): its program reads it from disk
            // now, and goes when no open document needs it.
            self.changed(key);
            self.owners.remove(key);
            // A program whose main file is not open publishes for open documents only, so it would not clear these.
            if !self.docs.contains_key(&owner.main)
                && let Some((from, uri)) = self.published.get(key).cloned()
                && from == owner.main
            {
                self.release(key.to_string(), uri, &owner.main);
            }
            if !self.is_live(&owner.main) {
                self.drop_program(&owner.main);
            }
            let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.waiting)
                .into_iter()
                .partition(|w| w.doc == key);
            self.waiting = rest;
            for w in mine {
                self.answer(w.req, None, key);
            }
            return;
        }
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
        // Open documents that belong to it: their program is now read from disk until their lookup ends.
        if self.is_live(key) {
            self.pending.insert(key.to_string(), Instant::now());
        }
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
        // Requests waiting for it get no answer from a parse: answer them now. Those of documents that belong to it
        // wait for its parse from disk.
        let live = self.is_live(key);
        let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.waiting)
            .into_iter()
            .partition(|w| w.doc == key || (w.key == key && !live));
        self.waiting = rest;
        for w in mine {
            let doc = w.doc.clone();
            self.answer(w.req, None, &doc);
        }
        // The documents that belonged to it look for their program again.
        let owned: Vec<String> = self
            .owners
            .iter()
            .filter(|(_, o)| o.main == key)
            .map(|(k, _)| k.clone())
            .collect();
        for k in owned {
            self.start_lookup(&k);
        }
    }

    /// The diagnostics of `file` that program `from` published are no longer its to keep: the file's program (when
    /// the file is open) or another open program that covers it is parsed again at once and publishes them anew;
    /// without one they are cleared.
    fn release(&mut self, file: String, uri: Uri, from: &str) {
        self.published.remove(&file);
        let own = self
            .docs
            .contains_key(&file)
            .then(|| self.program_of(&file))
            .filter(|p| p != from);
        let other = own.or_else(|| {
            self.analyses
                .iter()
                .find(|(k, a)| k.as_str() != from && a.files.iter().any(|f| f.key == file))
                .map(|(k, _)| k.clone())
        });
        match other {
            Some(program) => {
                self.pending.insert(program, Instant::now());
            }
            None => self.publish(uri, Vec::new()),
        }
    }

    /// The key of document `key`'s program: its owner's main file, else the document itself.
    fn program_of(&self, key: &str) -> String {
        self.owners.get(key).map_or_else(|| key.to_string(), |o| o.main.clone())
    }

    /// Whether program `key` is still wanted: its main file is open, or an open document belongs to it.
    fn is_live(&self, key: &str) -> bool {
        self.docs.contains_key(key) || self.owners.values().any(|o| o.main == key)
    }

    /// The main file of program `key` for a job (design D5 of `include-file-program`): its open document, else
    /// the file on disk, in the encoding the client named for it or the default. `None` for a program no open
    /// document needs, or a file that cannot be read.
    fn program_main(&self, key: &str) -> Option<OpenFile> {
        if let Some(d) = self.docs.get(key) {
            return Some(open_file(d));
        }
        let owner = self.owners.values().find(|o| o.main == key)?;
        let encoding = self.encodings.get(key).copied().unwrap_or(self.default);
        let bytes = std::fs::read(to_path(&owner.uri)?)
            .map_err(|e| eprintln!("qb64rust: cannot read {}: {e}", owner.uri.as_str()))
            .ok()?;
        Some(OpenFile {
            uri: owner.uri.clone(),
            bytes: bytes.into(),
            encoding,
        })
    }

    /// The open documents as the parser reads them.
    fn open_files(&self) -> Arc<HashMap<String, OpenFile>> {
        Arc::new(self.docs.iter().map(|(k, d)| (k.clone(), open_file(d))).collect())
    }

    /// Sends the job that looks up the program of open document `key` (design D7 of `include-file-program`). A
    /// document that is not a file is its own program, and the client is told so at once.
    fn start_lookup(&mut self, key: &str) {
        let Some(doc) = self.docs.get(key) else {
            return;
        };
        if to_path(&doc.uri).is_none() {
            self.set_owner(key, None);
            return;
        }
        let mut programs: Vec<(u64, String)> = self
            .docs
            .iter()
            .filter(|(k, d)| !self.owners.contains_key(*k) && to_path(&d.uri).is_some_and(|p| owner::is_bas(&p)))
            .map(|(k, d)| (d.opened, k.clone()))
            .collect();
        programs.sort();
        let main = open_file(doc);
        self.seq += 1;
        self.lookups.insert(key.to_string(), self.seq);
        let job = Job {
            key: key.to_string(),
            doc: key.to_string(),
            seq: self.seq,
            main,
            open: self.open_files(),
            root: self.root.clone(),
            default: self.default,
            check: None,
            format: None,
            index: false,
            rename: None,
            live: None,
            lone: false,
            lookup: Some(LookupJob {
                programs: programs.into_iter().map(|(_, k)| k).collect(),
                folders: self.folders.clone(),
            }),
        };
        let _ = self.jobs.send(job);
    }

    /// Looks up the program of every open document but `except`: the `.bas` documents first, each in the order it
    /// was opened, so that an includer's lookup runs before the lookup of what it includes.
    fn lookup_all(&mut self, except: &str) {
        let mut keys: Vec<(bool, u64, String)> = self
            .docs
            .iter()
            .filter(|(k, _)| k.as_str() != except)
            .map(|(k, d)| (!to_path(&d.uri).is_some_and(|p| owner::is_bas(&p)), d.opened, k.clone()))
            .collect();
        keys.sort();
        for (_, _, key) in keys {
            self.start_lookup(&key);
        }
    }

    /// The result of a lookup. The result of one that is no longer the document's last is dropped.
    fn owner_found(&mut self, doc: String, seq: u64, result: Result<(Option<Owner>, bool), String>) {
        if self.lookups.get(&doc) != Some(&seq) || !self.docs.contains_key(&doc) {
            return;
        }
        self.lookups.remove(&doc);
        match result {
            Ok((owner, named)) => {
                if named {
                    self.named.insert(doc.clone());
                } else {
                    self.named.remove(&doc);
                }
                self.set_owner(&doc, owner);
            }
            Err(msg) => eprintln!("qb64rust: {msg} ({doc})"),
        }
    }

    /// Document `doc`'s program is `owner` (`None`: itself). The client is told when it is new or the first since
    /// the document opened. When it changes, the old program's diagnostics for the document are released, the old
    /// program goes if nothing needs it, and the new one is parsed at once.
    fn set_owner(&mut self, doc: &str, owner: Option<Owner>) {
        let old = self.program_of(doc);
        let new = owner.as_ref().map_or_else(|| doc.to_string(), |o| o.main.clone());
        let same = self.owners.get(doc) == owner.as_ref();
        let first = self.announced.insert(doc.to_string());
        if same && !first {
            return;
        }
        match owner {
            Some(o) => self.owners.insert(doc.to_string(), o),
            None => self.owners.remove(doc),
        };
        self.announce(doc);
        if old == new {
            return;
        }
        // The requests that wait for the old program wait for the new one.
        for w in self.waiting.iter_mut().chain(self.index_waiting.iter_mut()) {
            if w.doc == doc {
                w.key.clone_from(&new);
            }
        }
        if old == doc || !self.is_live(&old) {
            self.drop_program(&old);
        }
        self.pending.insert(new.clone(), Instant::now());
        if self.index_waiting.iter().any(|w| w.key == new) {
            self.start_index(&new);
        }
        // The documents that belong to `doc` may belong to the program above it now.
        if self.owners.contains_key(doc) {
            let below: Vec<String> = self
                .owners
                .iter()
                .filter(|(_, o)| o.main == doc)
                .map(|(k, _)| k.clone())
                .collect();
            for k in below {
                self.start_lookup(&k);
            }
        }
    }

    /// Program `key` is no longer a program (design D5 of `include-file-program`): its parse, its index and its jobs
    /// go, a check under way is answered with `ContentModified`, and the diagnostics it published are released.
    fn drop_program(&mut self, key: &str) {
        self.pending.remove(key);
        self.latest_job.remove(key);
        self.stale_index(key);
        self.indexes.remove(key);
        self.analyses.remove(key);
        if let Some((_, id)) = self.checks.remove(key) {
            let code = ErrorCode::ContentModified as i32;
            self.send(Response::new_err(id, code, "the document's program changed".into()));
        }
        let files: Vec<(String, Uri)> = self
            .published
            .iter()
            .filter(|(_, (program, _))| program == key)
            .map(|(f, (_, uri))| (f.clone(), uri.clone()))
            .collect();
        for (file, uri) in files {
            self.release(file, uri, key);
        }
    }

    /// Sends `qb64rust/program` for open document `doc` (design D8 of `include-file-program`).
    fn announce(&self, doc: &str) {
        let Some(d) = self.docs.get(doc) else {
            return;
        };
        let (program, source) = match self.owners.get(doc) {
            Some(o) => (Value::String(o.uri.as_str().to_string()), o.source.name()),
            None => (Value::Null, owner::Source::None.name()),
        };
        let params = serde_json::json!({ "uri": d.uri.as_str(), "program": program, "source": source });
        self.send(Notification::new(PROGRAM.into(), params));
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
            let Some(main) = self.program_main(&key) else {
                continue;
            };
            self.seq += 1;
            self.latest_job.insert(key.clone(), self.seq);
            let job = Job {
                doc: key.clone(),
                key,
                seq: self.seq,
                main,
                open: Arc::clone(&open),
                root: self.root.clone(),
                default: self.default,
                check: None,
                format: None,
                index: false,
                rename: None,
                live: None,
                lone: false,
                lookup: None,
            };
            if self.jobs.send(job).is_err() {
                // The worker is gone (it never panics out of its loop); nothing more can be parsed.
                return;
            }
        }
    }

    fn parsed(&mut self, key: String, seq: u64, result: Result<Analysis, String>) {
        if self.latest_job.get(&key) != Some(&seq) || !self.is_live(&key) {
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
        let main_open = self.docs.contains_key(&key);
        for (file, diags) in diagnostics {
            let info = analysis.info(file);
            // A program whose main file is not open publishes for open documents only (design D6 of
            // `include-file-program`); a document that belongs to another program gets that program's only.
            if !main_open && !self.docs.contains_key(&info.key) {
                continue;
            }
            if self.owners.get(&info.key).is_some_and(|o| o.main != key) {
                continue;
            }
            self.published.insert(info.key.clone(), (key.clone(), info.uri.clone()));
            self.publish(info.uri.clone(), diags);
        }
        self.analyses.insert(key.clone(), analysis);
        // A document of this program that the parse no longer covers looks for its program again (design D7 of
        // `include-file-program`). So does a document that this open `.bas` program covers when it is its own
        // program (unless the settings file's `main` names it) or belongs to a program on disk: an open includer
        // comes first.
        let bas_program = main_open
            && !self.owners.contains_key(&key)
            && self
                .docs
                .get(&key)
                .and_then(|d| to_path(&d.uri))
                .is_some_and(|p| owner::is_bas(&p));
        let again: Vec<String> = self
            .docs
            .keys()
            .filter(|k| !self.lookups.contains_key(*k) && **k != key)
            .filter(|k| match self.owners.get(*k) {
                Some(o) if !covered.contains(k) => o.main == key,
                // Also when the program on disk is this one, now open.
                Some(o) => bas_program && o.source == owner::Source::Disk,
                None => bas_program && covered.contains(k) && !self.named.contains(*k),
            })
            .cloned()
            .collect();
        for k in again {
            self.start_lookup(&k);
        }
        let (mine, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.waiting)
            .into_iter()
            .partition(|w| w.key == key);
        self.waiting = rest;
        for w in mine {
            self.answer_parsed(w.req, &key, &w.doc);
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
                    Ok(p) => {
                        let doc = uri_key(&p.text_document.uri);
                        self.need_index(req, self.program_of(&doc), doc);
                    }
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
        let doc = uri_key(&uri);
        let key = self.program_of(&doc);
        match self.analyses.get(&key) {
            Some(_) => self.answer_parsed(req, &key, &doc),
            // The first parse (or a check, which parses) is still to come: the request waits for it. (After a parse
            // that failed, none comes until the next change.)
            None if self.docs.contains_key(&doc)
                && (self.pending.contains_key(&key)
                    || self.latest_job.contains_key(&key)
                    || self.checks.contains_key(&key)) =>
            {
                self.waiting.push(Waiting { key, doc, req });
            }
            None => self.answer(req, None, &doc),
        }
    }

    /// `qb64rust/check { uri }` (design D3): a job that parses the document's program and runs the front end. A
    /// newer check of the same program replaces an older one, which is answered at once with `ContentModified`. An
    /// include file that is its own program gets the parser's diagnostics only (design D9 of
    /// `include-file-program`).
    fn check(&mut self, req: Request) {
        let uri = field(&req.params, "uri").and_then(|u| Uri::from_str(u).ok());
        let Some(doc) = uri.map(|u| uri_key(&u)).filter(|k| self.docs.contains_key(k)) else {
            let msg = "qb64rust/check needs the uri of an open document".to_string();
            self.send(Response::new_err(req.id, ErrorCode::InvalidParams as i32, msg));
            return;
        };
        let key = self.program_of(&doc);
        let lone = key == doc && to_path(&self.docs[&doc].uri).is_some_and(|p| !owner::is_bas(&p));
        let Some(main) = self.program_main(&key) else {
            let msg = "the main file of the document's program cannot be read".to_string();
            self.send(Response::new_err(req.id, ErrorCode::InternalError as i32, msg));
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
            main,
            key,
            doc,
            seq: self.seq,
            open,
            root: self.root.clone(),
            default: self.default,
            check: Some(req.id),
            format: None,
            index: false,
            rename: None,
            live: None,
            lone,
            lookup: None,
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
        let doc_key = uri_key(&uri);
        let Some(doc) = self.docs.get(&doc_key) else {
            let msg = "formatting needs an open document".to_string();
            self.send(Response::new_err(req.id, ErrorCode::InvalidParams as i32, msg));
            return;
        };
        // The document is laid out inside its program, which may be another file (design D4 of
        // `include-file-program`).
        let key = self.program_of(&doc_key);
        let version = doc.version;
        let Some(main) = self.program_main(&key) else {
            let msg = "the main file of the document's program cannot be read".to_string();
            self.send(Response::new_err(req.id, ErrorCode::InternalError as i32, msg));
            return;
        };
        let open: Arc<HashMap<String, OpenFile>> =
            Arc::new(self.docs.iter().map(|(k, d)| (k.clone(), open_file(d))).collect());
        self.seq += 1;
        let job = Job {
            main,
            key,
            doc: doc_key.clone(),
            seq: self.seq,
            open,
            root: self.root.clone(),
            default: self.default,
            check: None,
            format: Some(FormatRequest {
                id: req.id,
                doc: doc_key,
                range,
                settings: self.format,
                version,
            }),
            index: false,
            rename: None,
            live: None,
            lone: false,
            lookup: None,
        };
        let _ = self.jobs.send(job);
    }

    /// The result of a formatting request of program `key`. An answer for a text that changed since the request is
    /// `ContentModified`. On success: one edit of the whole document, or one per changed line of the range, each
    /// line keeping its own line end; none when nothing changes. When the parser reports an error in the program:
    /// its diagnostics published as a check's, a message naming the first, and no edits. An answer from a program
    /// that is no longer the document's, or that no longer includes it, is `ContentModified` too.
    fn formatted(&mut self, key: &str, request: FormatRequest, result: FormatResult) {
        let Some(doc) = self.docs.get(&request.doc).filter(|d| d.version == request.version) else {
            let code = ErrorCode::ContentModified as i32;
            self.send(Response::new_err(request.id, code, "the document changed".into()));
            return;
        };
        let modified = |s: &Self, id: RequestId| {
            let code = ErrorCode::ContentModified as i32;
            s.send(Response::new_err(id, code, "the document's program changed".into()));
        };
        let (analysis, text) = match result {
            Ok((a, Some(text))) if self.program_of(&request.doc) == key => (a, text),
            Ok(_) => {
                modified(self, request.id);
                return;
            }
            // A panic is logged whatever else changed.
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

    /// Answers a request about document `doc` from the last parse of its program `key`. A definition the parser does
    /// not find (a name other than a procedure or a label) is taken from the index (design D4 of
    /// `m5b-language-features`).
    fn answer_parsed(&mut self, req: Request, key: &str, doc: &str) {
        if req.method == GotoDefinition::METHOD
            && self.docs.contains_key(doc)
            && self
                .analyses
                .get(key)
                .is_some_and(|a| parser_definition(a, &req, doc).is_none())
        {
            self.need_index(req, key.to_string(), doc.to_string());
            return;
        }
        let a = self.analyses.get(key);
        self.answer(req, a, doc);
    }

    /// Answers a request about document `doc` from its program `key`'s index: at once with an index of the current
    /// text, else when the index job (sent unless one is under way) comes back (design D2 of
    /// `m5b-language-features`). A document that is not open gets an empty answer.
    fn need_index(&mut self, req: Request, key: String, doc: String) {
        if let Some((true, ix)) = self.indexes.get(&key) {
            let rename = self.answer_indexed(req, Some(ix), &doc);
            self.check_rename(key, rename);
            return;
        }
        if !self.docs.contains_key(&doc) || !self.is_live(&key) {
            self.answer_indexed(req, None, &doc);
            return;
        }
        self.start_index(&key);
        self.index_waiting.push(Waiting { key, doc, req });
    }

    /// Sends the job that builds program `key`'s index, unless one is under way.
    fn start_index(&mut self, key: &str) {
        let Some(main) = self.program_main(key) else {
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
            main,
            key: key.to_string(),
            doc: key.to_string(),
            seq: self.seq,
            open,
            root: self.root.clone(),
            default: self.default,
            check: None,
            format: None,
            index: true,
            rename: None,
            live: Some(live),
            lone: false,
            lookup: None,
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
        let doc_key = uri_key(&p.text_document_position.text_document.uri);
        let Some(doc) = self.docs.get(&doc_key) else {
            self.send(Response::new_ok(req.id, CompletionList::default()));
            return;
        };
        let key = self.program_of(&doc_key);
        let before = line_before(&doc.text, pos);
        let ix = self.indexes.get(&key).map(|(_, ix)| ix);
        let offset = ix.and_then(|ix| {
            let file = ix.analysis.file_of(&doc_key)?;
            Some((
                file,
                ix.analysis.info(file).columns.offset(ix.analysis.map.file(file), pos),
            ))
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
                    let rename = self.answer_indexed(w.req, ix, &w.doc);
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

    /// Answers a request about document `doc` from an index of its program's current text, or with an empty result
    /// without one. A rename that passes the checks of the index is not answered yet: its edit is given back, to be
    /// checked (design D5).
    fn answer_indexed(&self, req: Request, ix: Option<&Indexed>, doc: &str) -> Option<PendingRename> {
        // The position as (index, file, offset): the document's file in its program (design D5 of
        // `include-file-program`).
        let position = |p: Position| {
            let ix = ix?;
            let file = ix.analysis.file_of(doc)?;
            Some((
                ix,
                (
                    file,
                    ix.analysis.info(file).columns.offset(ix.analysis.map.file(file), p),
                ),
            ))
        };
        let refuse = |id: RequestId, msg: String| {
            self.send(Response::new_err(id, ErrorCode::RequestFailed as i32, msg));
            None
        };
        let result = match req.method.as_str() {
            PrepareRenameRequest::METHOD => match serde_json::from_value::<TextDocumentPositionParams>(req.params) {
                Ok(p) => match position(p.position).map(|(ix, (file, at))| (ix, rename::prepare(ix, file, at))) {
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
                    Some((ix, (file, at))) => match rename::plan(ix, file, at, &p.new_name, &self.root) {
                        Ok(plan) => return Some(self.pending_rename(req.id, ix, plan)),
                        Err(msg) => return refuse(req.id, msg),
                    },
                },
                Err(e) => Err(e),
            },
            HoverRequest::METHOD => serde_json::from_value::<HoverParams>(req.params.clone())
                .map(|p| {
                    position(p.text_document_position_params.position)
                        .and_then(|(ix, (file, at))| hover::hover(ix, file, at))
                })
                .and_then(serde_json::to_value),
            GotoDefinition::METHOD => serde_json::from_value::<GotoDefinitionParams>(req.params.clone())
                .map(|p| {
                    position(p.text_document_position_params.position).and_then(|(ix, (file, at))| {
                        let (id, _) = ix.index.entry_near(file, at)?;
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
                    found.map_or_else(Vec::new, |(ix, (file, at))| {
                        definition::references(ix, file, at, p.context.include_declaration)
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
        let main_file = ix.analysis.parsed.main().file;
        let main = ix.analysis.info(main_file);
        // A main file that is not open and that the rename does not change: as the index read it.
        let main = open.get(&main.key).cloned().unwrap_or_else(|| OpenFile {
            uri: main.uri.clone(),
            bytes: ix.analysis.map.file(main_file).bytes.to_vec().into(),
            encoding: self.default,
        });
        PendingRename {
            main,
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
            doc: key.clone(),
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
            lone: false,
            lookup: None,
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

    /// Answers a request about document `doc` from a parse of its program, or with an empty result without one. The
    /// symbols and folding ranges are those of the document's own file (design D5 of `include-file-program`).
    fn answer(&self, req: Request, a: Option<&Analysis>, doc: &str) {
        let file = a.and_then(|a| Some((a, a.file_of(doc)?)));
        let result = match req.method.as_str() {
            DocumentSymbolRequest::METHOD => {
                let symbols = file
                    .map(|(a, f)| symbols::document_symbols_in(a, f))
                    .unwrap_or_default();
                serde_json::to_value(DocumentSymbolResponse::Nested(symbols))
            }
            FoldingRangeRequest::METHOD => {
                serde_json::to_value(file.map(|(a, f)| folding::folding_ranges_in(a, f)).unwrap_or_default())
            }
            GotoDefinition::METHOD => {
                let found = a.and_then(|a| parser_definition(a, &req, doc));
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

/// The parser's definition of the name at a definition request's position in document `doc`: a procedure or a
/// label (design D4).
fn parser_definition(a: &Analysis, req: &Request, doc: &str) -> Option<Location> {
    let p: GotoDefinitionParams = serde_json::from_value(req.params.clone()).ok()?;
    let file = a.file_of(doc)?;
    let pos = p.text_document_position_params.position;
    let offset = a.info(file).columns.offset(a.map.file(file), pos);
    let span = definition::definition_in(a, file, offset)?;
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

    /// The next response, past the notifications (`qb64rust/program`).
    fn response(client: &Connection) -> Response {
        loop {
            match client.receiver.try_recv() {
                Ok(Message::Response(r)) => return r,
                Ok(Message::Notification(_)) => {}
                other => panic!("expected a response, got {other:?}"),
            }
        }
    }

    fn no_check(
        _: &qb64rust_base::SourceMap,
        _: &qb64rust_syntax::ParsedProgram,
        _: Option<&std::path::Path>,
        _: &std::path::Path,
    ) -> Diagnostics {
        Diagnostics::new()
    }

    fn no_format(
        _: &qb64rust_base::SourceMap,
        _: &qb64rust_syntax::ParsedProgram,
        _: Option<&std::path::Path>,
        _: &std::path::Path,
        _: &Settings,
        _: FileId,
    ) -> Result<Vec<u8>, Diagnostics> {
        Ok(Vec::new())
    }

    fn no_index(
        _: &qb64rust_base::SourceMap,
        _: &qb64rust_syntax::ParsedProgram,
        _: Option<&std::path::Path>,
        _: &std::path::Path,
    ) -> SymbolIndex {
        SymbolIndex::default()
    }

    fn no_main(_: &std::path::Path) -> Option<PathBuf> {
        None
    }

    /// Runs the server's side of the events until `done` holds, with every debounce cut short; panics after two
    /// seconds.
    fn pump(s: &mut Server, inbox: &mpsc::Receiver<Event>, done: impl Fn(&Server) -> bool) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while !done(s) {
            assert!(Instant::now() < deadline, "not done in time");
            for at in s.pending.values_mut() {
                *at = Instant::now();
            }
            s.dispatch_due();
            match inbox.recv_timeout(Duration::from_millis(50)) {
                Ok(Event::Parsed { key, seq, result }) => s.parsed(key, seq, result),
                Ok(Event::Owner { doc, seq, result }) => s.owner_found(doc, seq, result),
                Ok(Event::Checked { key, seq, id, result }) => s.checked(key, seq, id, result),
                Ok(Event::Indexed { key, seq, result }) => s.indexed(key, seq, result),
                Ok(_) | Err(_) => {}
            }
        }
    }

    /// Task 5.1 of `include-file-program`: an include file open alone whose program is on disk is parsed as that
    /// program, after an edit too; when it closes, nothing of the program is left.
    #[test]
    fn program_not_open_goes_with_its_last_document() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("main.bas"), "'$INCLUDE: 'lib.bm'\nPRINT 1\n").unwrap();
        let (server_side, _client) = Connection::memory();
        let (events, inbox) = mpsc::channel();
        let hooks = Hooks {
            check: no_check,
            format: no_format,
            index: no_index,
            main_of: no_main,
        };
        let jobs = start_worker(events, hooks);
        let mut s = Server::new(server_side, jobs, PathBuf::new(), Encoding::DEFAULT);
        let lib = crate::uri::from_path(&dir.path().join("lib.bm"));
        let (lib_key, main_key) = (uri_key(&lib), crate::uri::path_key(&dir.path().join("main.bas")));
        let doc = json!({ "uri": lib, "languageId": "qb64rust", "version": 1, "text": "SUB s\nEND SUB\n" });
        s.notification(Notification::new(
            DidOpenTextDocument::METHOD.into(),
            json!({ "textDocument": doc }),
        ));
        pump(&mut s, &inbox, |s| s.analyses.contains_key(&main_key));
        assert_eq!(s.program_of(&lib_key), main_key);
        assert!(
            !s.analyses.contains_key(&lib_key),
            "the include file is no program of its own"
        );
        let change = json!({
            "textDocument": { "uri": lib, "version": 2 },
            "contentChanges": [{ "text": "SUB s\nPRINT 2\nEND SUB\n" }],
        });
        s.notification(Notification::new(DidChangeTextDocument::METHOD.into(), change));
        assert!(s.pending.contains_key(&main_key), "an edit parses the program again");
        pump(&mut s, &inbox, |s| s.pending.is_empty() && s.latest_job.is_empty());
        assert!(s.analyses.contains_key(&main_key));
        let close = json!({ "textDocument": { "uri": lib } });
        s.notification(Notification::new(DidCloseTextDocument::METHOD.into(), close));
        assert!(
            s.analyses.is_empty() && s.indexes.is_empty() && s.owners.is_empty(),
            "something is left"
        );
        assert!(s.pending.is_empty() && s.latest_job.is_empty() && s.published.is_empty());
    }

    /// The settings file's `main` in [`named_document_is_not_looked_up_after_each_parse`]: `game.bas` beside the
    /// document.
    fn main_is_game(doc: &std::path::Path) -> Option<PathBuf> {
        let game = doc.parent()?.join("game.bas");
        game.is_file().then_some(game)
    }

    /// A document that the settings file's `main` names is its own program, so a parse of an open program that
    /// covers it starts no lookup of it.
    #[test]
    fn named_document_is_not_looked_up_after_each_parse() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("game.bas"), "PRINT 1\n").unwrap();
        let (server_side, _client) = Connection::memory();
        let (events, inbox) = mpsc::channel();
        let hooks = Hooks {
            check: no_check,
            format: no_format,
            index: no_index,
            main_of: main_is_game,
        };
        let jobs = start_worker(events, hooks);
        let mut s = Server::new(server_side, jobs, PathBuf::new(), Encoding::DEFAULT);
        let (game, test) = (
            crate::uri::from_path(&dir.path().join("game.bas")),
            crate::uri::from_path(&dir.path().join("game_test.bas")),
        );
        let (game_key, test_key) = (uri_key(&game), uri_key(&test));
        for (uri, text) in [(&test, "'$INCLUDE: 'game.bas'\n"), (&game, "PRINT 1\n")] {
            let doc = json!({ "uri": uri, "languageId": "qb64rust", "version": 1, "text": text });
            s.notification(Notification::new(
                DidOpenTextDocument::METHOD.into(),
                json!({ "textDocument": doc }),
            ));
        }
        pump(&mut s, &inbox, |s| {
            s.lookups.is_empty() && s.named.contains(&game_key) && s.analyses.contains_key(&test_key)
        });
        assert_eq!(s.program_of(&game_key), game_key);
        let change = json!({
            "textDocument": { "uri": test, "version": 2 },
            "contentChanges": [{ "text": "'$INCLUDE: 'game.bas'\nPRINT 2\n" }],
        });
        s.notification(Notification::new(DidChangeTextDocument::METHOD.into(), change));
        // Stops right after the parse is taken, before the result of any lookup it started.
        pump(&mut s, &inbox, |s| s.pending.is_empty() && s.latest_job.is_empty());
        assert!(
            s.analyses[&test_key].files.iter().any(|f| f.key == game_key),
            "the parse covers game.bas"
        );
        assert!(s.lookups.is_empty(), "a lookup started: {:?}", s.lookups);
    }

    /// A format of an include file whose program no longer includes it, before its new lookup ends, is
    /// `ContentModified`: the program's main file is not laid out in its place.
    #[test]
    fn format_of_a_file_its_program_no_longer_includes() {
        let dir = tempfile::tempdir().unwrap();
        let (server_side, client) = Connection::memory();
        let (events, inbox) = mpsc::channel();
        let hooks = Hooks {
            check: no_check,
            format: no_format,
            index: no_index,
            main_of: no_main,
        };
        let jobs = start_worker(events, hooks);
        let mut s = Server::new(server_side, jobs, PathBuf::new(), Encoding::DEFAULT);
        let (main, lib) = (
            crate::uri::from_path(&dir.path().join("main.bas")),
            crate::uri::from_path(&dir.path().join("lib.bm")),
        );
        let (main_key, lib_key) = (uri_key(&main), uri_key(&lib));
        for (uri, text) in [(&main, "'$INCLUDE: 'lib.bm'\n"), (&lib, "SUB s\nEND SUB\n")] {
            let doc = json!({ "uri": uri, "languageId": "qb64rust", "version": 1, "text": text });
            s.notification(Notification::new(
                DidOpenTextDocument::METHOD.into(),
                json!({ "textDocument": doc }),
            ));
        }
        pump(&mut s, &inbox, |s| {
            s.program_of(&lib_key) == main_key && s.lookups.is_empty() && s.analyses.contains_key(&main_key)
        });
        // The `$INCLUDE` line goes; the parse starts the lookup of `lib.bm`, whose result is held back.
        let change = json!({
            "textDocument": { "uri": main, "version": 2 },
            "contentChanges": [{ "text": "PRINT 1\n" }],
        });
        s.notification(Notification::new(DidChangeTextDocument::METHOD.into(), change));
        pump(&mut s, &inbox, |s| s.lookups.contains_key(&lib_key));
        while client.receiver.try_recv().is_ok() {}
        let params = json!({ "textDocument": { "uri": lib }, "options": { "tabSize": 4, "insertSpaces": true } });
        s.request(Request::new(1.into(), Formatting::METHOD.into(), params));
        loop {
            match inbox
                .recv_timeout(Duration::from_secs(2))
                .expect("no format result in time")
            {
                Event::Formatted { key, request, result } => {
                    s.formatted(&key, request, result);
                    break;
                }
                Event::Owner { .. } | Event::Parsed { .. } => {}
                _ => panic!("an unexpected event"),
            }
        }
        assert_eq!(s.program_of(&lib_key), main_key, "the lookup's result was held back");
        let err = response(&client).response_result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ContentModified as i32, "{}", err.message);
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
        let answered = client.receiver.try_iter().any(|m| matches!(m, Message::Response(_)));
        assert!(!answered, "answered before the parse");
        // The parse fails: the waiting request gets the error ...
        s.parsed(job.key, job.seq, Err("internal error while parsing: test".into()));
        let code = response(&client).response_result.unwrap_err().code;
        assert_eq!(code, ErrorCode::InternalError as i32);
        // ... and a later one an empty answer at once, not a wait for a parse that does not come.
        s.request(symbols(2));
        assert_eq!(response(&client).response_result.unwrap(), json!([]));
    }
}
