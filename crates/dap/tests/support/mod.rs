//! A client and a scripted backend for the proxy's tests, each an end of two pipes, with the proxy between them on
//! a thread of its own. Every read has a deadline, so that a proxy that waits for ever fails the test.

#![allow(dead_code, reason = "each test binary uses its own part")]

use qb64rust_dap::wire::{read_message, write_message};
use serde_json::Value;
use std::io::{BufReader, PipeReader, PipeWriter, pipe};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};
use std::thread::JoinHandle;
use std::time::Duration;

const DEADLINE: Duration = Duration::from_secs(10);

/// One side as the proxy sees it: what the test sends it, and what it sent.
pub struct End {
    to: Option<PipeWriter>,
    from: Receiver<Value>,
}

impl End {
    fn new(to: PipeWriter, from: PipeReader) -> End {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let mut from = BufReader::new(from);
            while let Ok(Some(m)) = read_message(&mut from) {
                if tx.send(m).is_err() {
                    break;
                }
            }
        });
        End { to: Some(to), from: rx }
    }

    pub fn send(&mut self, m: Value) {
        write_message(self.to.as_mut().expect("open"), &m).expect("the proxy reads");
    }

    /// The next message the proxy sent this side.
    pub fn recv(&self) -> Value {
        self.from
            .recv_timeout(DEADLINE)
            .expect("a message from the proxy in time")
    }

    /// The next message, or none within a short wait.
    pub fn try_recv(&self) -> Option<Value> {
        self.from.recv_timeout(Duration::from_millis(300)).ok()
    }

    /// The next message if one has come, without waiting.
    pub fn poll(&self) -> Option<Value> {
        self.from.try_recv().ok()
    }
}

impl Launched {
    /// The next message to the client that `wanted` takes, answering the backend's requests from the model while
    /// waiting; the client's other messages are dropped.
    pub fn wait(&mut self, wanted: impl Fn(&Value) -> bool) -> Value {
        let deadline = std::time::Instant::now() + DEADLINE;
        loop {
            while let Some(m) = self.backend.poll() {
                self.fake.answer(&mut self.backend, &m);
            }
            if let Ok(m) = self.client.from.recv_timeout(Duration::from_millis(1))
                && wanted(&m)
            {
                return m;
            }
            assert!(std::time::Instant::now() < deadline, "no message to the client in time");
        }
    }

    /// The next `stopped` event the client is sent.
    pub fn stop(&mut self) -> Value {
        self.wait(|m| m["event"] == "stopped")
    }

    /// Sends the client's request and gives its response.
    pub fn request(&mut self, command: &str, arguments: Value) -> Value {
        self.client
            .send(serde_json::json!({"seq": 30, "type": "request", "command": command, "arguments": arguments}));
        self.wait(|m| m["type"] == "response" && m["command"] == command)
    }
}

pub struct Session {
    pub client: End,
    pub backend: End,
    pub done: JoinHandle<Result<(), String>>,
}

/// The proxy between a client and a scripted backend.
pub fn start() -> Session {
    let (from_client, client_to) = pipe().unwrap();
    let (client_from, to_client) = pipe().unwrap();
    let (from_proxy, to_backend) = pipe().unwrap();
    let (from_backend, backend_to) = pipe().unwrap();
    let done = std::thread::spawn(move || qb64rust_dap::serve(from_client, to_client, to_backend, from_backend));
    Session {
        client: End::new(client_to, client_from),
        backend: End::new(backend_to, from_proxy),
        done,
    }
}

/// A stand-in for `lldb-dap` that answers the proxy's requests from a model: the stack every `stackTrace` gives,
/// the address of each C name, the memory, the code in `ecx`. Every request it answered is in `seen`.
#[derive(Default)]
pub struct Fake {
    pub frames: Vec<Value>,
    pub pointers: std::collections::HashMap<String, u64>,
    pub memory: Vec<(u64, Vec<u8>)>,
    pub ecx: i64,
    pub next_id: i64,
    pub seen: Vec<Value>,
}

/// A frame as `lldb-dap` gives it.
pub fn frame(id: i64, name: &str, path: &str, line: u64, column: u64) -> Value {
    serde_json::json!({"id": id, "name": name, "line": line, "column": column, "source": {"path": path}})
}

impl Fake {
    pub fn put(&mut self, at: u64, bytes: &[u8]) {
        self.memory.push((at, bytes.to_vec()));
    }

    fn read(&self, at: u64, count: u64) -> Option<Vec<u8>> {
        let (start, bytes) = self
            .memory
            .iter()
            .rev()
            .find(|(s, b)| at >= *s && at.checked_add(count).is_some_and(|end| end <= s + b.len() as u64))?;
        let from = usize::try_from(at - start).ok()?;
        Some(bytes[from..from + usize::try_from(count).ok()?].to_vec())
    }

    fn write(&mut self, at: u64, data: &[u8]) {
        if let Some((start, bytes)) = self
            .memory
            .iter_mut()
            .rev()
            .find(|(s, b)| at >= *s && at + data.len() as u64 <= *s + b.len() as u64)
        {
            let from = usize::try_from(at - *start).unwrap();
            bytes[from..from + data.len()].copy_from_slice(data);
        }
    }

    /// Answers `req` from the model.
    pub fn answer(&mut self, backend: &mut End, req: &Value) {
        use serde_json::json;
        self.seen.push(req.clone());
        let a = &req["arguments"];
        let body: Result<Value, String> = match req["command"].as_str().unwrap_or("") {
            "stackTrace" => Ok(json!({"stackFrames": self.frames, "totalFrames": self.frames.len()})),
            "setFunctionBreakpoints" => {
                let n = a["breakpoints"].as_array().map_or(0, Vec::len);
                let bps: Vec<Value> = (0..n)
                    .map(|_| {
                        self.next_id += 1;
                        json!({"id": self.next_id, "verified": true})
                    })
                    .collect();
                Ok(json!({"breakpoints": bps}))
            }
            "evaluate" => {
                let e = a["expression"].as_str().unwrap_or("");
                if let Some(name) = e.strip_prefix("(unsigned long long)") {
                    self.pointers
                        .get(name)
                        .map(|p| json!({"result": p.to_string()}))
                        .ok_or_else(|| format!("use of undeclared identifier '{name}'"))
                } else if e == "(int)$ecx" {
                    Ok(json!({"result": self.ecx.to_string()}))
                } else if e.starts_with("((void*(*)(void*,void*))&qbs_set)") || e.starts_with('`') {
                    Ok(json!({"result": "0x1"}))
                } else {
                    Err(format!("error: no `{e}`"))
                }
            }
            "readMemory" => {
                let at =
                    u64::from_str_radix(a["memoryReference"].as_str().unwrap().trim_start_matches("0x"), 16).unwrap();
                let count = a["count"].as_u64().unwrap();
                self.read(at, count)
                    .map(|b| json!({"address": format!("0x{at:X}"), "data": qb64rust_dap::wire::base64_encode(&b)}))
                    .ok_or_else(|| "memory read failed".to_string())
            }
            "writeMemory" => {
                let at =
                    u64::from_str_radix(a["memoryReference"].as_str().unwrap().trim_start_matches("0x"), 16).unwrap();
                let data = qb64rust_dap::wire::base64_decode(a["data"].as_str().unwrap()).unwrap();
                self.write(at, &data);
                Ok(json!({"bytesWritten": data.len()}))
            }
            _ => Ok(json!({})),
        };
        let mut r = json!({"type": "response", "request_seq": req["seq"], "command": req["command"], "seq": 0});
        match body {
            Ok(b) => {
                r["success"] = json!(true);
                r["body"] = b;
            }
            Err(e) => {
                r["success"] = json!(false);
                r["message"] = json!(e);
            }
        }
        backend.send(r);
    }

    /// Answers requests until one of `command` comes; gives it, unanswered.
    pub fn until(&mut self, backend: &mut End, command: &str) -> Value {
        loop {
            let m = backend.recv();
            if m["command"] == command {
                return m;
            }
            self.answer(backend, &m);
        }
    }

    /// [`Fake::until`], then answers that one too.
    pub fn through(&mut self, backend: &mut End, command: &str) -> Value {
        let m = self.until(backend, command);
        self.answer(backend, &m);
        m
    }

    /// Answers every request that comes within a short wait.
    pub fn settle(&mut self, backend: &mut End) {
        while let Some(m) = backend.try_recv() {
            self.answer(backend, &m);
        }
    }
}

/// A launched session of the proxy on the fixture `kinds` (a copy of `tests\fixtures` in a temporary folder, with
/// an empty `kinds.exe`), its backend answered by a [`Fake`] whose program thread is 7, stopped by nothing yet: the
/// proxy's breakpoint on `QBMAIN` is set and has been hit.
pub struct Launched {
    pub client: End,
    pub backend: End,
    pub fake: Fake,
    pub dir: tempfile::TempDir,
}

pub const THREAD: i64 = 7;

pub fn launched(stop_on_entry: bool) -> Launched {
    launched_on("kinds", stop_on_entry)
}

/// [`launched`] on the fixture `name` (`tests\fixtures\<name>.bas` and its symbol file).
pub fn launched_on(name: &str, stop_on_entry: bool) -> Launched {
    use serde_json::json;
    let dir = tempfile::tempdir().unwrap();
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for f in [format!("{name}.bas"), format!("{name}.exe.debug.json")] {
        std::fs::copy(fixtures.join(&f), dir.path().join(&f)).unwrap();
    }
    std::fs::write(dir.path().join(format!("{name}.exe")), b"").unwrap();
    let Session {
        mut client,
        mut backend,
        ..
    } = start();
    let mut fake = Fake::default();
    client.send(json!({"seq": 1, "type": "request", "command": "initialize", "arguments": {}}));
    fake.through(&mut backend, "initialize");
    client.recv();
    client.send(json!({"seq": 2, "type": "request", "command": "launch",
                       "arguments": {"program": dir.path().join(format!("{name}.exe")), "stopOnEntry": stop_on_entry}}));
    fake.through(&mut backend, "launch");
    assert_eq!(client.recv()["success"], true);
    client.send(json!({"seq": 3, "type": "request", "command": "configurationDone", "arguments": {}}));
    let set = fake.through(&mut backend, "setFunctionBreakpoints");
    assert_eq!(set["arguments"]["breakpoints"], json!([{"name": "QBMAIN"}]));
    fake.through(&mut backend, "configurationDone");
    assert_eq!(client.recv()["success"], true);
    let main_id = fake.next_id;
    backend.send(json!({"seq": 9, "type": "event", "event": "stopped",
                        "body": {"reason": "breakpoint", "threadId": THREAD, "hitBreakpointIds": [main_id]}}));
    let set = fake.through(&mut backend, "setFunctionBreakpoints");
    assert_eq!(
        set["arguments"]["breakpoints"],
        json!([]),
        "the main module's breakpoint goes after its hit"
    );
    Launched {
        client,
        backend,
        fake,
        dir,
    }
}

/// The fixture's path of `kinds.bas` in a launched session.
pub fn bas(l: &Launched) -> String {
    l.dir.path().join("kinds.bas").to_string_lossy().to_string()
}

/// [`qb64rust_dap::run`] with a client: the backend is started from the path, or not at all.
pub fn start_run(backend: Result<PathBuf, String>) -> (End, JoinHandle<Result<(), String>>) {
    let (from_client, client_to) = pipe().unwrap();
    let (client_from, to_client) = pipe().unwrap();
    let done = std::thread::spawn(move || qb64rust_dap::run(backend, from_client, to_client));
    (End::new(client_to, client_from), done)
}
