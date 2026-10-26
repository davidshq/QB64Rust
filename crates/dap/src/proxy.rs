//! The loop of the proxy (design D2 of `m5-dap-proxy`): two threads read the client and the backend into one
//! channel, and one thread takes the messages in order and owns all state. Every message the proxy writes is
//! numbered per side; the `seq` of each request sent to the backend maps to where its answer goes. A request of
//! the proxy's own ([`Proxy::call`]) blocks until its answer comes, queueing what arrives meanwhile.

use crate::wire::{read_message, write_message};
use serde_json::{Value, json};
use std::collections::{HashMap, VecDeque};
use std::io::{BufReader, Read, Write};
use std::sync::mpsc::{Receiver, Sender, channel};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Side {
    Client,
    Backend,
}

pub(crate) enum Input {
    Message(Side, Value),
    Ended(Side),
}

/// Where the answer to a request sent to the backend goes.
enum Origin {
    /// A client's request passed on: its answer goes to the client under the client's `seq`.
    Client { seq: i64, command: String },
    /// The proxy's own request, whose answer [`Proxy::call`] waits for.
    Call,
    /// The proxy's own request, whose answer is dropped.
    Drop,
}

/// The `seq` of a message, 0 when it has none.
pub(crate) fn seq_of(m: &Value) -> i64 {
    m["seq"].as_i64().unwrap_or(0)
}

pub(crate) fn command_of(m: &Value) -> &str {
    m["command"].as_str().unwrap_or("")
}

/// The text of a failed response: its `message`, or the `format` of its `body.error`.
fn failure(m: &Value) -> String {
    m["body"]["error"]["format"]
        .as_str()
        .or_else(|| m["message"].as_str())
        .unwrap_or("the request failed")
        .to_string()
}

pub struct Proxy {
    client: Box<dyn Write>,
    backend: Box<dyn Write>,
    input: Receiver<Input>,
    /// What arrived while a [`Proxy::call`] waited, handled after the current message, in order.
    queue: VecDeque<Input>,
    client_seq: i64,
    backend_seq: i64,
    pending: HashMap<i64, Origin>,
    /// The `seq` of each reverse request passed to the client, to the backend's `seq` of it.
    reverse: HashMap<i64, i64>,
    /// The client was sent `terminated` (`lldb-dap`'s, or the proxy's own): it is sent once.
    terminated: bool,
    pub(crate) state: crate::State,
}

fn reader(side: Side, from: impl Read + Send + 'static, to: Sender<Input>) {
    std::thread::spawn(move || {
        let mut from = BufReader::new(from);
        loop {
            match read_message(&mut from) {
                Ok(Some(m)) => {
                    if to.send(Input::Message(side, m)).is_err() {
                        return;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!("qb64rust dap: from the {side:?}: {e}");
                    break;
                }
            }
        }
        let _ = to.send(Input::Ended(side));
    });
}

/// Runs the proxy between a client and a backend until either side ends. When the backend ends first, the client
/// is sent `terminated` unless it already was.
pub fn serve(
    client_in: impl Read + Send + 'static,
    client_out: impl Write + 'static,
    backend_in: impl Write + 'static,
    backend_out: impl Read + Send + 'static,
) -> Result<(), String> {
    let (to, input) = channel();
    reader(Side::Client, client_in, to.clone());
    reader(Side::Backend, backend_out, to);
    let mut p = Proxy {
        client: Box::new(client_out),
        backend: Box::new(backend_in),
        input,
        queue: VecDeque::new(),
        client_seq: 0,
        backend_seq: 0,
        pending: HashMap::new(),
        reverse: HashMap::new(),
        terminated: false,
        state: crate::State::default(),
    };
    p.run()
}

/// Answers a client when there is no backend: `initialize` and every other request fail with `message`;
/// `disconnect` succeeds and ends the session.
pub fn serve_without_backend(
    client_in: impl Read + Send + 'static,
    mut client_out: impl Write,
    message: &str,
) -> Result<(), String> {
    let mut client_in = BufReader::new(client_in);
    let mut seq = 0;
    while let Some(m) = read_message(&mut client_in)? {
        if m["type"] != "request" {
            continue;
        }
        seq += 1;
        let disconnect = command_of(&m) == "disconnect";
        let answer = if disconnect {
            response(seq, &m, Ok(Value::Null))
        } else {
            response(seq, &m, Err(message.to_string()))
        };
        write_message(&mut client_out, &answer)?;
        if disconnect {
            break;
        }
    }
    Ok(())
}

/// A response to `request`: its body, or an error shown to the user.
fn response(seq: i64, request: &Value, result: Result<Value, String>) -> Value {
    let mut r = json!({
        "seq": seq,
        "type": "response",
        "request_seq": seq_of(request),
        "command": command_of(request),
    });
    match result {
        Ok(body) => {
            r["success"] = json!(true);
            if !body.is_null() {
                r["body"] = body;
            }
        }
        Err(message) => {
            r["success"] = json!(false);
            r["message"] = json!(message);
            r["body"] = json!({"error": {"id": 1, "format": message, "showUser": true}});
        }
    }
    r
}

impl Proxy {
    fn run(&mut self) -> Result<(), String> {
        loop {
            let input = match self.queue.pop_front() {
                Some(i) => i,
                None => match self.input.recv() {
                    Ok(i) => i,
                    Err(_) => return Ok(()),
                },
            };
            match input {
                Input::Message(Side::Client, m) => self.on_client(m)?,
                Input::Message(Side::Backend, m) => self.on_backend(m)?,
                Input::Ended(Side::Client) => return Ok(()),
                Input::Ended(Side::Backend) => {
                    if !self.terminated {
                        self.event("terminated", json!({}))?;
                    }
                    return Ok(());
                }
            }
        }
    }

    fn on_client(&mut self, m: Value) -> Result<(), String> {
        match m["type"].as_str() {
            Some("request") => crate::request(self, m),
            Some("response") => {
                // The client's answer to a reverse request (`runInTerminal`).
                let Some(backend_seq) = m["request_seq"].as_i64().and_then(|s| self.reverse.remove(&s)) else {
                    return Ok(());
                };
                let mut m = m;
                m["request_seq"] = json!(backend_seq);
                self.send_backend(m)
            }
            _ => Ok(()),
        }
    }

    fn on_backend(&mut self, mut m: Value) -> Result<(), String> {
        match m["type"].as_str() {
            Some("response") => {
                let origin = m["request_seq"].as_i64().and_then(|s| self.pending.remove(&s));
                if let Some(Origin::Client { seq, command }) = origin {
                    m["request_seq"] = json!(seq);
                    crate::response(self, &command, &mut m)?;
                    self.send_client(m)?;
                }
                Ok(())
            }
            Some("event") => crate::event(self, m),
            Some("request") => {
                let backend_seq = seq_of(&m);
                let client_seq = self.send_client(m)?;
                self.reverse.insert(client_seq, backend_seq);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Writes a message to the client under the next `seq`; gives that `seq`.
    pub(crate) fn send_client(&mut self, mut m: Value) -> Result<i64, String> {
        self.client_seq += 1;
        m["seq"] = json!(self.client_seq);
        if m["type"] == "event" && m["event"] == "terminated" {
            self.terminated = true;
        }
        write_message(&mut self.client, &m)?;
        Ok(self.client_seq)
    }

    /// Writes a message to the backend under the next `seq`; gives that `seq`. A backend that cannot be written
    /// to has ended: its reader thread says so.
    fn send_backend(&mut self, mut m: Value) -> Result<(), String> {
        self.backend_seq += 1;
        m["seq"] = json!(self.backend_seq);
        if let Err(e) = write_message(&mut self.backend, &m) {
            eprintln!("qb64rust dap: to lldb-dap: {e}");
        }
        Ok(())
    }

    fn request_backend(&mut self, command: &str, arguments: Value, origin: Origin) -> Result<i64, String> {
        let mut m = json!({"type": "request", "command": command});
        if !arguments.is_null() {
            m["arguments"] = arguments;
        }
        self.send_backend(m)?;
        self.pending.insert(self.backend_seq, origin);
        Ok(self.backend_seq)
    }

    /// Passes a client's request to the backend as it is; its answer goes back to the client.
    pub(crate) fn forward(&mut self, request: Value) -> Result<(), String> {
        let origin = Origin::Client {
            seq: seq_of(&request),
            command: command_of(&request).to_string(),
        };
        let mut m = request;
        if let Some(o) = m.as_object_mut() {
            o.remove("seq");
        }
        self.send_backend(m)?;
        self.pending.insert(self.backend_seq, origin);
        Ok(())
    }

    /// Sends a request of the proxy's own whose answer is dropped.
    pub(crate) fn send(&mut self, command: &str, arguments: Value) -> Result<(), String> {
        self.request_backend(command, arguments, Origin::Drop).map(|_| ())
    }

    /// Sends a request of the proxy's own and waits for its answer: the body (`null` when it has none), or the
    /// failure's text. Messages that arrive meanwhile are handled after the current one.
    pub(crate) fn call(&mut self, command: &str, arguments: Value) -> Result<Value, String> {
        let seq = self.request_backend(command, arguments, Origin::Call)?;
        loop {
            match self.input.recv() {
                Ok(Input::Message(Side::Backend, m)) if m["type"] == "response" && m["request_seq"] == seq => {
                    self.pending.remove(&seq);
                    return if m["success"] == true {
                        Ok(m["body"].clone())
                    } else {
                        Err(failure(&m))
                    };
                }
                Ok(Input::Ended(Side::Backend)) => {
                    self.queue.push_back(Input::Ended(Side::Backend));
                    return Err("lldb-dap has ended".into());
                }
                Ok(other) => self.queue.push_back(other),
                Err(_) => return Err("lldb-dap has ended".into()),
            }
        }
    }

    /// Answers a client's request.
    pub(crate) fn respond(&mut self, request: &Value, result: Result<Value, String>) -> Result<(), String> {
        self.send_client(response(0, request, result)).map(|_| ())
    }

    /// Sends the client an event.
    pub(crate) fn event(&mut self, name: &str, body: Value) -> Result<(), String> {
        self.send_client(json!({"type": "event", "event": name, "body": body}))
            .map(|_| ())
    }
}
