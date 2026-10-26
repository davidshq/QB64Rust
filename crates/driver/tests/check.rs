//! The language server's check with the driver's front end (`check_for_server`), driven in-process: what it
//! publishes is what the CLI reports with `-w` for the same files (spec `editor/language-server`, "Check
//! diagnostics"; design D2 of `extension-diagnostics`).

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use qb64rust_base::{Diagnostic, Severity};
use qb64rust_driver::{FileLoader, frontend_at};
use qb64rust_lsp::encoding::Encoding;
use qb64rust_lsp::uri::{from_path, to_path, uri_key};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Long enough for a check of a test program on a loaded machine.
const WAIT: Duration = Duration::from_secs(20);

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf()
}

/// One diagnostic as the editor gets it: the file's key, line and column from 0, message, severity.
type Shown = (String, u64, u64, String, u64);

/// The result of a check through the server: its answer, and every diagnostic published before it.
struct Checked {
    answer: Value,
    published: Vec<Shown>,
}

fn recv(conn: &Connection, deadline: Instant) -> Message {
    conn.receiver
        .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .expect("no message in time")
}

/// Opens `path` (its bytes read as the editor shows them, in the default encoding) in a new server with the
/// include root `root`, checks it at once (so that no parse publishes before the check) and shuts the server down.
fn check_in_server(path: &Path, root: &Path) -> Checked {
    let (server_side, conn) = Connection::memory();
    let root = root.to_path_buf();
    let server = std::thread::Builder::new()
        .stack_size(qb64rust_base::STACK_SIZE)
        .spawn(move || qb64rust_lsp::serve(server_side, root, qb64rust_driver::server_hooks()))
        .unwrap();
    let send = |id: i32, method: &str, params: Value| {
        let req = Request::new(RequestId::from(id), method.into(), params);
        conn.sender.send(req.into()).unwrap();
    };
    let notify = |method: &str, params: Value| {
        conn.sender
            .send(Notification::new(method.into(), params).into())
            .unwrap();
    };
    let options = json!({ "processId": null, "rootUri": null, "capabilities": {} });
    send(1, "initialize", options);
    assert!(matches!(recv(&conn, Instant::now() + WAIT), Message::Response(_)));
    notify("initialized", json!({}));
    let uri = from_path(path);
    let text = Encoding::DEFAULT.decode(&std::fs::read(path).unwrap()).unwrap();
    let doc = json!({ "uri": uri, "languageId": "qb64rust", "version": 1, "text": text });
    notify("textDocument/didOpen", json!({ "textDocument": doc }));
    send(2, qb64rust_lsp::CHECK, json!({ "uri": uri }));
    let deadline = Instant::now() + WAIT;
    let mut published = Vec::new();
    let answer = loop {
        match recv(&conn, deadline) {
            Message::Response(r) => break r.response_result.unwrap(),
            Message::Notification(n) if n.method == "textDocument/publishDiagnostics" => {
                let file = n.params["uri"].as_str().unwrap().parse().unwrap();
                let key = uri_key(&file);
                for d in n.params["diagnostics"].as_array().unwrap() {
                    let start = &d["range"]["start"];
                    published.push((
                        key.clone(),
                        start["line"].as_u64().unwrap(),
                        start["character"].as_u64().unwrap(),
                        d["message"].as_str().unwrap().to_string(),
                        d["severity"].as_u64().unwrap(),
                    ));
                    assert_eq!(d["source"], "qb64rust");
                }
            }
            m => panic!("unexpected message {m:?}"),
        }
    };
    send(3, "shutdown", Value::Null);
    assert!(matches!(recv(&conn, Instant::now() + WAIT), Message::Response(_)));
    notify("exit", Value::Null);
    server.join().unwrap().unwrap();
    Checked { answer, published }
}

/// What the CLI reports for `path` with `-w` (`frontend_at` with the disk checks, as `main.rs` runs it), in the
/// order it prints them, each as the server shows it: the column from 0 (one per byte in the default encoding),
/// the "not supported yet" marker and the old compiler's wording as the CLI's note gives them. A diagnostic given
/// twice is shown once.
fn cli(path: &Path, root: &Path) -> (Vec<Shown>, usize, usize) {
    // The main file named as the server names it, from its URI: a message that names a file then reads the same.
    let name = to_path(&from_path(path)).unwrap().to_string_lossy().into_owned();
    let fe = frontend_at(
        &name,
        std::fs::read(path).unwrap(),
        |file| Box::new(FileLoader::new(root, file, path)),
        Some((path, root)),
    );
    let mut list: Vec<Diagnostic> = fe.diagnostics.list().to_vec();
    list.sort_by_key(|d| (d.span.file.0, d.span.start));
    let mut seen = std::collections::HashSet::new();
    list.retain(|d| seen.insert((d.span, d.message.clone())));
    let shown = list
        .iter()
        .map(|d| {
            let file = fe.map.file(d.span.file);
            let (line, col) = file.line_col(d.span.start);
            let rendered = d.render(&fe.map);
            let mark = if d.unsupported { "not supported yet: " } else { "" };
            assert!(rendered.contains(&format!("{mark}{}", d.message)), "{rendered}");
            let message = match d.old_wording() {
                Some(old) => format!("{mark}{}\nQB64pe: {old}", d.message),
                None => format!("{mark}{}", d.message),
            };
            let severity = match d.severity {
                Severity::Error => 1,
                Severity::Warning => 2,
            };
            // A built-in file (`$COLOR`'s constants) is named relative to the compiler root.
            let key = uri_key(&from_path(&root.join(&file.name)));
            (key, u64::from(line - 1), u64::from(col - 1), message, severity)
        })
        .collect();
    let warnings = fe
        .diagnostics
        .list()
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    (shown, fe.diagnostics.error_count(), warnings)
}

/// The check of `path` through the server equals the CLI's report, diagnostics and counts alike.
fn same_as_cli(path: &Path, root: &Path) -> Checked {
    let checked = check_in_server(path, root);
    let (expected, errors, warnings) = cli(path, root);
    assert_eq!(checked.published, expected, "{}", path.display());
    assert_eq!(
        checked.answer,
        json!({ "errors": errors, "warnings": warnings }),
        "{}",
        path.display()
    );
    checked
}

/// Spec scenario "Same as the CLI", over every `check-fail` program of the front-end tests, with their included
/// files on disk and the front-end tests' compiler root.
#[test]
fn check_fail_programs_as_the_cli_reports_them() {
    let dir = repo().join("tests/frontend");
    let root = dir.join("inc/root");
    let mut programs: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "bas"))
        .filter(|p| std::fs::read(p).unwrap().starts_with(b"' TEST: check-fail"))
        .collect();
    programs.sort();
    assert!(programs.len() > 100, "{} check-fail programs", programs.len());
    for path in &programs {
        let checked = same_as_cli(path, &root);
        assert!(!checked.published.is_empty(), "{}: nothing published", path.display());
    }
}

fn program(dir: &Path, name: &str, text: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap();
    path
}

#[test]
fn type_error_on_check() {
    let dir = tempfile::tempdir().unwrap();
    let path = program(dir.path(), "p.bas", "PRINT 1\r\nx$ = 1\r\n");
    let checked = same_as_cli(&path, dir.path());
    assert_eq!(checked.answer, json!({ "errors": 1, "warnings": 0 }));
    assert_eq!(checked.published.len(), 1);
    assert_eq!(checked.published[0].1, 1);
}

#[test]
fn several_errors_in_one_run() {
    let dir = tempfile::tempdir().unwrap();
    let text = "PRINT 1\r\nPRINT 2\r\nGOTO nowhere\r\nPRINT 4\r\nPRINT 5\r\nPRINT 6\r\nx$ = 1\r\n";
    let path = program(dir.path(), "p.bas", text);
    let checked = same_as_cli(&path, dir.path());
    let lines: Vec<_> = checked.published.iter().map(|d| (d.1, d.4)).collect();
    assert_eq!(lines, [(2, 1), (6, 1)]);
}

#[test]
fn warning() {
    let dir = tempfile::tempdir().unwrap();
    let path = program(dir.path(), "p.bas", "$DEBUG\r\nPRINT 1\r\n");
    let checked = same_as_cli(&path, dir.path());
    assert_eq!(checked.answer["warnings"], 1);
    assert!(
        checked.published.iter().any(|d| d.1 == 0 && d.4 == 2),
        "{:?}",
        checked.published
    );
}

#[test]
fn missing_icon() {
    let dir = tempfile::tempdir().unwrap();
    let path = program(dir.path(), "p.bas", "$EXEICON:'./game.ico'\r\nPRINT 1\r\n");
    let checked = same_as_cli(&path, dir.path());
    assert_eq!(checked.answer["errors"], 1);
    let d = &checked.published[0];
    assert_eq!((d.1, d.4), (0, 1));
    assert!(d.3.contains("./game.ico"), "{}", d.3);
    // A file that is no icon is an error too (design D1 of `cpp-failure-errors`).
    std::fs::write(dir.path().join("game.ico"), [0u8; 4]).unwrap();
    let checked = same_as_cli(&path, dir.path());
    assert_eq!(checked.answer["errors"], 1);
    assert!(
        checked.published[0].3.contains("not an icon"),
        "{}",
        checked.published[0].3
    );
    // With the icon beside the program, the check is clean.
    let icon = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/frontend/inc/exeicon.ico");
    std::fs::copy(&icon, dir.path().join("game.ico")).unwrap();
    assert_eq!(same_as_cli(&path, dir.path()).answer["errors"], 0);
    // A name without `./` is looked for under the compiler root (design D3 of `m4-devices-continue-paths`): missing
    // there, the error names it as written; there, the check is clean.
    let root = tempfile::tempdir().unwrap();
    let path = program(dir.path(), "q.bas", "$EXEICON:'icons/game.ico'\r\nPRINT 1\r\n");
    let checked = same_as_cli(&path, root.path());
    assert_eq!(checked.answer["errors"], 1);
    assert!(
        checked.published[0].3.contains("`icons/game.ico`"),
        "{}",
        checked.published[0].3
    );
    std::fs::create_dir(root.path().join("icons")).unwrap();
    std::fs::copy(&icon, root.path().join("icons/game.ico")).unwrap();
    assert_eq!(same_as_cli(&path, root.path()).answer["errors"], 0);
}
