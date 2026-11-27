//! A client of the language server for the tests that drive it in-process with the driver's hooks (the check, the
//! formatter and the language features). Every wait has a deadline, so that a server that never answers fails the
//! test. The protocol's own rules are tested with stand-in hooks in `crates\lsp\tests\it\server.rs`.

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use qb64rust_lsp::Hooks;
use qb64rust_lsp::uri::from_path;
use serde_json::{Value, json};
use std::path::Path;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Long enough for a check, a format or the front end of a test program on a loaded machine.
pub const WAIT: Duration = Duration::from_secs(20);

pub struct Client {
    pub conn: Connection,
    server: Option<JoinHandle<Result<(), String>>>,
    /// The id of the last request sent.
    pub next: i32,
    /// The notifications received while waiting for answers.
    pub notes: Vec<Notification>,
}

impl Client {
    /// A server with the include root `root` and the driver's hooks.
    pub fn start(root: &Path) -> Client {
        Client::launch(root, Value::Null, qb64rust_driver::server_hooks())
    }

    /// [`Client::start`] with other hooks.
    pub fn start_with(root: &Path, hooks: Hooks) -> Client {
        Client::launch(root, Value::Null, hooks)
    }

    /// [`Client::start`], initialized with the client's `options` (`initializationOptions`).
    pub fn with_options(root: &Path, options: Value) -> Client {
        Client::launch(root, options, qb64rust_driver::server_hooks())
    }

    fn launch(root: &Path, options: Value, hooks: Hooks) -> Client {
        let (server_side, conn) = Connection::memory();
        let root = root.to_path_buf();
        let server = std::thread::Builder::new()
            .stack_size(qb64rust_base::STACK_SIZE)
            .spawn(move || qb64rust_lsp::serve(server_side, root, hooks))
            .unwrap();
        let mut c = Client {
            conn,
            server: Some(server),
            next: 0,
            notes: Vec::new(),
        };
        let init = json!({ "processId": null, "rootUri": null, "capabilities": {}, "initializationOptions": options });
        c.request("initialize", init);
        c.notify("initialized", json!({}));
        c
    }

    pub fn notify(&self, method: &str, params: Value) {
        let n = Notification::new(method.into(), params);
        self.conn.sender.send(n.into()).unwrap();
    }

    pub fn send(&mut self, method: &str, params: Value) -> RequestId {
        self.next += 1;
        let id = RequestId::from(self.next);
        self.conn
            .sender
            .send(Request::new(id.clone(), method.into(), params).into())
            .unwrap();
        id
    }

    /// The next message from the server, by `deadline`. The server sends no requests, so one fails the test.
    pub fn recv(&self, deadline: Instant) -> Message {
        let msg = self
            .conn
            .receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .expect("no message in time");
        if let Message::Request(r) = &msg {
            panic!("unexpected request {r:?}");
        }
        msg
    }

    /// The answer to the request `id`; the notifications received meanwhile go to [`Client::notes`].
    pub fn answer(&mut self, id: &RequestId) -> Response {
        let deadline = Instant::now() + WAIT;
        loop {
            match self.recv(deadline) {
                Message::Response(r) if r.id == *id => return r,
                Message::Notification(n) => self.notes.push(n),
                _ => {}
            }
        }
    }

    pub fn request(&mut self, method: &str, params: Value) -> Response {
        let id = self.send(method, params);
        self.answer(&id)
    }

    /// The params of the first notification of `method` received so far.
    pub fn note(&self, method: &str) -> Value {
        let n = self.notes.iter().find(|n| n.method == method);
        n.unwrap_or_else(|| panic!("no {method}")).params.clone()
    }

    pub fn open(&self, path: &Path, text: &str) {
        let doc = json!({ "uri": from_path(path), "languageId": "qb64rust", "version": 1, "text": text });
        self.notify("textDocument/didOpen", json!({ "textDocument": doc }));
    }

    pub fn change(&self, path: &Path, text: &str) {
        let doc = json!({ "uri": from_path(path), "version": 2 });
        self.notify(
            "textDocument/didChange",
            json!({ "textDocument": doc, "contentChanges": [{ "text": text }] }),
        );
    }

    /// Waits for the `qb64rust/program` notification of the document at `path`, and gives its program's uri.
    pub fn program(&mut self, path: &Path) -> Option<String> {
        let uri = from_path(path);
        let is_for = |n: &Notification| n.method == "qb64rust/program" && n.params["uri"] == uri.as_str();
        let deadline = Instant::now() + WAIT;
        while !self.notes.iter().any(is_for) {
            if let Message::Notification(n) = self.recv(deadline) {
                self.notes.push(n);
            }
        }
        let n = self.notes.iter().find(|n| is_for(n)).unwrap();
        n.params["program"].as_str().map(str::to_string)
    }

    pub fn shutdown(mut self) {
        self.stop();
    }

    /// Shuts the server down and waits for it; nothing when it is stopped already.
    pub fn stop(&mut self) {
        let Some(server) = self.server.take() else {
            return;
        };
        self.request("shutdown", Value::Null);
        self.notify("exit", Value::Null);
        server.join().unwrap().unwrap();
    }
}
