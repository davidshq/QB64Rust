//! The debug adapter, `qb64rust dap` (OpenSpec change `m5-dap-proxy`, spec `editor/debug-adapter`): a proxy in
//! front of the toolchain's `lldb-dap` that shows a debug build of this compiler in BASIC terms. It reads the
//! debug symbol file of `compiler/debug-info` and rewrites the few requests and events that must speak BASIC;
//! everything else passes through with only its `seq` changed.

mod breakpoints;
pub mod errors;
pub mod eval;
pub mod format;
mod launch;
mod proxy;
mod stack;
mod step;
pub mod symbols;
mod vars;
pub mod wire;

pub use proxy::{Proxy, serve, serve_without_backend};

use proxy::command_of;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use symbols::Symbols;

/// The environment `lldb` runs in (design D1, "Measured"): it debugs through `lldb-server` rather than its own
/// Windows plugin, which under load could lose the stop that ends a step and wait for ever. On Linux `lldb` debugs
/// through `lldb-server` already, and the variable is not set (design D5 of `m7-linux`).
pub const LLDB_ENV: &[(&str, &str)] = if cfg!(windows) {
    &[("LLDB_USE_LLDB_SERVER", "1")]
} else {
    &[]
};

/// The register that holds the first integer argument of a call at the callee's entry: `ecx` in the Windows x64
/// convention, `edi` in the System V one of Linux (measured, task 4.2 of `m7-linux`). libqb's `error` takes the error
/// code there (design D10).
pub const FIRST_ARG: &str = if cfg!(windows) { "$ecx" } else { "$edi" };

/// The exception filter of a stop at a runtime error (design D10).
pub const ERROR_FILTER: &str = "qbError";

/// What the proxy knows: nothing until a `launch` it accepts, then the session.
#[derive(Default)]
pub struct State {
    session: Option<Session>,
    /// The backend offers `writeMemory` (`supportsWriteMemoryRequest` in its `initialize` answer). `lldb-dap` 20 and
    /// older do not, and end when sent one; a value is then written with `lldb`'s `memory write` (measured, task 4.2
    /// of `m7-linux`).
    backend_writes_memory: bool,
}

/// The oldest `lldb-dap` the adapter's features work with: 18 offers no data breakpoints, 19 ends at the first
/// `readMemory` (so no value can be shown), 20 passes the debugger's tests (measured on Ubuntu 24.04, task 4.2 of
/// `m7-linux`, and in a fresh container, task 1.4). An older one known by its name is not started
/// (`qb64rust_driver::toolchain::lldb_dap`).
pub const LLDB_DAP_MINIMUM: u32 = 20;

/// The version a versioned `lldb-dap`'s file name gives (`lldb-dap-19` gives 19, as Debian and Ubuntu name each
/// version's); `None` for any other name (`lldb-dap`, `lldb-dap.exe`).
pub fn lldb_dap_version(path: &std::path::Path) -> Option<u32> {
    path.file_name()?.to_str()?.strip_prefix("lldb-dap-")?.parse().ok()
}

/// A debug session of one program.
pub(crate) struct Session {
    pub symbols: Symbols,
    pub files: Vec<stack::FileInfo>,
    /// Each procedure as the client sees it: `SUB show`, `FUNCTION twice&`.
    pub names: Vec<String>,
    /// The thread that runs the BASIC program, known from the first stop of the proxy's breakpoint on the main
    /// module (design D11).
    pub program_thread: Option<i64>,
    pub stop_on_entry: bool,
    /// The proxy's own `continue` after its breakpoint on the main module was sent, and its `continued` not seen.
    pub starting: bool,
    /// The client asked for a pause that has not stopped yet.
    pub pausing: bool,
    pub breakpoints: breakpoints::Breakpoints,
    pub step: Option<step::Step>,
    pub refs: vars::Refs,
    /// The procedure of each BASIC frame shown since the last stop (`None` for the main module), by frame id.
    pub frames: HashMap<i64, Option<usize>>,
    /// The error of the last error stop: its number and text.
    pub error: Option<(i64, String)>,
    /// The source of the last `gotoTargets` that gave a target, for its `goto`.
    pub goto_path: Option<String>,
    /// The debuggee's memory read since the last stop: the start of each block and its bytes.
    pub memory: Vec<(u64, Vec<u8>)>,
}

impl Session {
    /// Forgets what belongs to one stop: the variable references, the frames, the memory read.
    pub fn resumed(&mut self) {
        self.refs.clear();
        self.frames.clear();
        self.memory.clear();
    }
}

/// A request from the client.
fn request(p: &mut Proxy, m: Value) -> Result<(), String> {
    match command_of(&m) {
        "launch" => launch::launch(p, m),
        "configurationDone" => breakpoints::configuration_done(p, m),
        "setFunctionBreakpoints" => breakpoints::set_function(p, m),
        "setExceptionBreakpoints" => breakpoints::set_exception(p, m),
        "exceptionInfo" => breakpoints::exception_info(p, m),
        _ if p.state.session.is_none() => p.forward(m),
        "threads" => stack::threads(p, m),
        "stackTrace" => stack::stack_trace(p, m),
        "next" | "stepIn" | "stepOut" => step::start(p, m),
        "continue" | "pause" => {
            let mut m = m;
            if let Some(s) = p.state.session.as_mut() {
                // A pause that brought no stop is not left to name a later one.
                s.pausing = command_of(&m) == "pause";
                if !s.pausing {
                    s.resumed();
                    s.step = None;
                }
                if let Some(t) = s.program_thread {
                    m["arguments"]["threadId"] = json!(t);
                }
            }
            p.forward(m)
        }
        "scopes" => vars::scopes(p, m),
        "variables" => vars::variables(p, m),
        "evaluate" => eval::evaluate(p, m),
        "setVariable" => eval::set_variable(p, m),
        "setExpression" => eval::set_expression(p, m),
        "dataBreakpointInfo" => eval::data_breakpoint_info(p, m),
        "gotoTargets" => eval::goto_targets(p, m),
        "goto" => eval::goto(p, m),
        _ => p.forward(m),
    }
}

/// A response of the backend to a client's request, before it goes to the client.
fn response(p: &mut Proxy, command: &str, m: &mut Value) -> Result<(), String> {
    if command == "initialize" && m["success"] == true {
        p.state.backend_writes_memory = m["body"]["supportsWriteMemoryRequest"] == true;
        let body = &mut m["body"];
        if !body.is_object() {
            *body = json!({});
        }
        body["supportsGotoTargetsRequest"] = json!(true);
        body["supportsExceptionInfoRequest"] = json!(true);
        body["supportsSetExpression"] = json!(true);
        body["exceptionBreakpointFilters"] = json!([
            {"filter": ERROR_FILTER, "label": "BASIC runtime errors", "default": false}
        ]);
    }
    Ok(())
}

/// An event of the backend.
fn event(p: &mut Proxy, mut m: Value) -> Result<(), String> {
    let Some(s) = p.state.session.as_mut() else {
        return p.send_client(m).map(|_| ());
    };
    match m["event"].as_str().unwrap_or("") {
        "stopped" => return stopped(p, m),
        // The steps the proxy takes on its own, and the start after its breakpoint on the main module, are not
        // the client's to see.
        "continued" if s.starting => {
            s.starting = false;
            return Ok(());
        }
        "continued" if s.step.is_some() || s.program_thread.is_none() => return Ok(()),
        "continued" => {
            s.resumed();
            if let Some(t) = s.program_thread {
                m["body"]["threadId"] = json!(t);
            }
        }
        "breakpoint" if s.breakpoints.is_own(m["body"]["breakpoint"]["id"].as_i64()) => return Ok(()),
        // libqb's threads are hidden (spec "One thread, the program's").
        "thread" => return Ok(()),
        "exited" | "terminated" => s.step = None,
        _ => {}
    }
    p.send_client(m).map(|_| ())
}

/// A `stopped` event of the backend. When a request the proxy makes while handling it fails (the program ended, or
/// `lldb-dap` did), the client is told why and shown the stop as it came, so that the session is not lost: a
/// backend that has ended is followed by `terminated`.
fn stopped(p: &mut Proxy, m: Value) -> Result<(), String> {
    let thread = m["body"]["threadId"].as_i64().unwrap_or(0);
    let reason = m["body"]["reason"].clone();
    let Err(e) = step::stopped(p, m) else {
        return Ok(());
    };
    // Not known at this stop: `exceptionInfo` does not give an earlier stop's error as this one's.
    if let Some(s) = p.state.session.as_mut() {
        s.error = None;
    }
    p.event(
        "output",
        json!({"category": "console", "output": format!("qb64rust dap: {e}\n")}),
    )?;
    step::report(p, json!({"reason": reason}), thread)
}

/// Runs the adapter over stdin and stdout with the `lldb-dap` at `backend` as its backend, or, when the clone was
/// not found (`Err`) or `lldb-dap` cannot be started, answers every request with that error (spec
/// `editor/debug-adapter`, "Adapter start and backend").
pub fn run_stdio(backend: Result<PathBuf, String>) -> Result<(), String> {
    run(backend, std::io::stdin(), std::io::stdout().lock())
}

/// [`run_stdio`] over any pair of streams.
pub fn run(
    backend: Result<PathBuf, String>,
    client_in: impl Read + Send + 'static,
    client_out: impl Write + 'static,
) -> Result<(), String> {
    let started = backend.and_then(|path| {
        Command::new(&path)
            .envs(LLDB_ENV.iter().copied())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("cannot start lldb-dap at {}: {e}", path.display()))
    });
    let mut child = match started {
        Ok(child) => child,
        Err(message) => return serve_without_backend(client_in, client_out, &message),
    };
    let (Some(backend_in), Some(backend_out)) = (child.stdin.take(), child.stdout.take()) else {
        return Err("lldb-dap started without pipes".into());
    };
    let result = serve(client_in, client_out, backend_in, backend_out);
    // A client that left without `disconnect` leaves the backend, and its program, running.
    if child.try_wait().ok().flatten().is_none() {
        let _ = child.kill();
    }
    let _ = child.wait();
    result
}
