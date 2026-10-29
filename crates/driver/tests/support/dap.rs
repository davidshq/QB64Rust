//! A small client of the Debug Adapter Protocol for the CLI tests of `qb64rust dap` (design D13 of `m5-dap-proxy`):
//! it starts the adapter, sends requests, and waits for responses and events, each wait with a deadline; past it the
//! adapter's process tree is killed and the test fails, showing every message seen. Each message is printed to the
//! test's stderr (`--nocapture` shows the session).
//!
//! Each session has `lldb-dap` record its traffic with the proxy (`LLDBDAP_LOG`) to a file of the session's own, and
//! a failure shows the file's end: when a wait runs out, it tells whether `lldb-dap` went quiet or the proxy did
//! (the Linux CI hangs of 2026-10-29, a different test each run).

use qb64rust_dap::wire::{read_message, write_message};
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

/// The deadline of one wait: a step or a launch takes a second or two, a build is not part of it.
pub const DAP_WAIT: Duration = Duration::from_secs(60);
/// The lines of `lldb-dap`'s log a failure shows, from its end.
const LOG_TAIL: usize = 200;

pub struct Dap {
    child: Child,
    to: Option<ChildStdin>,
    from: Receiver<Value>,
    /// What came and was not yet taken by a wait, in order.
    backlog: VecDeque<Value>,
    /// Everything that came, for the message of a failure.
    seen: Vec<Value>,
    seq: i64,
    /// `lldb-dap`'s log of the session, removed at its end.
    log: PathBuf,
    /// Held for the session's life: one debugger session at a time ([`super::debugger_alone`]).
    _alone: std::sync::MutexGuard<'static, ()>,
}

impl Dap {
    /// Starts the adapter `cmd` (`qb64rust dap …`) with its pipes.
    pub fn start(mut cmd: Command) -> Dap {
        let alone = super::debugger_alone();
        static SESSIONS: AtomicU32 = AtomicU32::new(0);
        let log = std::env::temp_dir().join(format!(
            "qb64rust-cli-lldb-dap-{}-{}.log",
            std::process::id(),
            SESSIONS.fetch_add(1, Ordering::Relaxed)
        ));
        cmd.env("LLDBDAP_LOG", &log)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        super::own_group(&mut cmd);
        let mut child = cmd.spawn().expect("qb64rust dap starts");
        let to = child.stdin.take();
        let out = child.stdout.take().unwrap();
        let (tx, from) = channel();
        std::thread::spawn(move || {
            let mut out = BufReader::new(out);
            while let Ok(Some(m)) = read_message(&mut out) {
                if tx.send(m).is_err() {
                    break;
                }
            }
        });
        Dap {
            child,
            to,
            from,
            backlog: VecDeque::new(),
            seen: Vec::new(),
            seq: 0,
            log,
            _alone: alone,
        }
    }

    /// Sends a request; gives its `seq`.
    pub fn send(&mut self, command: &str, arguments: Value) -> i64 {
        self.seq += 1;
        let m = json!({"seq": self.seq, "type": "request", "command": command, "arguments": arguments});
        eprintln!("dap -> {m}");
        write_message(self.to.as_mut().expect("the adapter's stdin"), &m).expect("the adapter reads");
        self.seq
    }

    #[expect(clippy::disallowed_methods, reason = "lldb-dap's log, for a person to read")]
    fn fail(&mut self, what: &str) -> ! {
        super::kill_tree(&mut self.child);
        let seen: Vec<String> = self.seen.iter().map(Value::to_string).collect();
        let log = match std::fs::read(&self.log) {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let lines: Vec<&str> = text.lines().collect();
                lines[lines.len().saturating_sub(LOG_TAIL)..].join("\n")
            }
            Err(e) => format!("(none: {e})"),
        };
        panic!(
            "dap: {what} (limit {} s)\nseen:\n{}\nlldb-dap's log, its last {LOG_TAIL} lines:\n{log}",
            DAP_WAIT.as_secs(),
            seen.join("\n")
        );
    }

    /// The first message, of the backlog or still to come, that `wanted` takes.
    fn take(&mut self, what: &str, wanted: impl Fn(&Value) -> bool) -> Value {
        if let Some(i) = self.backlog.iter().position(&wanted) {
            return self.backlog.remove(i).unwrap();
        }
        let deadline = Instant::now() + DAP_WAIT;
        loop {
            match self
                .from
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            {
                Ok(m) => {
                    eprintln!("dap <- {m}");
                    self.seen.push(m.clone());
                    if wanted(&m) {
                        return m;
                    }
                    self.backlog.push_back(m);
                }
                Err(RecvTimeoutError::Timeout) => self.fail(&format!("no {what} in time")),
                Err(RecvTimeoutError::Disconnected) => self.fail(&format!("the adapter ended before {what}")),
            }
        }
    }

    /// The response to the request `seq`.
    pub fn response(&mut self, seq: i64) -> Value {
        self.take(&format!("response to request {seq}"), |m| {
            m["type"] == "response" && m["request_seq"] == seq
        })
    }

    /// Sends a request and waits for its response.
    pub fn request(&mut self, command: &str, arguments: Value) -> Value {
        let seq = self.send(command, arguments);
        self.response(seq)
    }

    /// [`Dap::request`] for one that must succeed; gives its body.
    pub fn ok(&mut self, command: &str, arguments: Value) -> Value {
        let r = self.request(command, arguments);
        if r["success"] != true {
            self.fail(&format!("`{command}` failed: {r}"));
        }
        r["body"].clone()
    }

    /// The next event `name`.
    pub fn event(&mut self, name: &str) -> Value {
        self.take(&format!("event `{name}`"), |m| {
            m["type"] == "event" && m["event"] == name
        })
    }

    /// The next reverse request `command` of the adapter (`runInTerminal`).
    pub fn reverse_request(&mut self, command: &str) -> Value {
        self.take(&format!("reverse request `{command}`"), |m| {
            m["type"] == "request" && m["command"] == command
        })
    }

    /// Answers the reverse request `request` with success and `body`.
    pub fn reply(&mut self, request: &Value, body: Value) {
        self.seq += 1;
        let m = json!({"seq": self.seq, "type": "response", "request_seq": request["seq"],
                       "command": request["command"], "success": true, "body": body});
        eprintln!("dap -> {m}");
        write_message(self.to.as_mut().expect("the adapter's stdin"), &m).expect("the adapter reads");
    }

    /// Every event `name` that has come so far, taken from the backlog.
    pub fn drain_events(&mut self, name: &str) -> Vec<Value> {
        while let Ok(m) = self.from.recv_timeout(Duration::from_millis(200)) {
            eprintln!("dap <- {m}");
            self.seen.push(m.clone());
            self.backlog.push_back(m);
        }
        let (taken, kept) = std::mem::take(&mut self.backlog)
            .into_iter()
            .partition(|m| m["type"] == "event" && m["event"] == name);
        self.backlog = kept;
        taken.into()
    }

    /// Closes the adapter's stdin and waits for it to end.
    pub fn finish(mut self) -> ExitStatus {
        self.to = None;
        let deadline = Instant::now() + DAP_WAIT;
        loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                return status;
            }
            if Instant::now() >= deadline {
                self.fail("the adapter did not end in time");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

impl Drop for Dap {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            super::kill_tree(&mut self.child);
        }
        let _ = std::fs::remove_file(&self.log);
    }
}
