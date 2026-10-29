//! Stepping by BASIC line (design D6 of `m5-dap-proxy`, amended by "Measured"): a client's step is answered at once
//! and sent on; each stop of the backend's is looked at and stepped on until it is on a line of BASIC code. Every
//! `stopped` event of the backend comes here first.
//!
//! | Where the stop is | Then |
//! |---|---|
//! | a function that is no BASIC code (libqb, `is_error_pending`, `evnt`) | `stepOut` |
//! | the main module or a procedure, on a line of no listed file or past a file's end (`qbx.cpp`, `maindata.txt`, `data1.txt`) | `next` |
//! | a procedure's header line | `next` (on to the first statement) |
//! | the caller after a return, or a line after a `stepOut` of libqb's, at a column other than 1 (the rest of a `CALL`, of the line) | `next`; `stepIn` for a client's `stepIn` |
//! | on entry, a line with no statement (blank, a comment, a metacommand) | `next` |
//! | anything else | the stop is reported |

use crate::breakpoints::{self, Own, error_text};
use crate::proxy::Proxy;
use crate::stack;
use serde_json::{Value, json};

/// The bound on the steps the proxy takes for one step of the client's: a bound on a rule's mistake, not on a long
/// statement (each step on is a short `lldb` step).
pub const MAX_STEPS: u32 = 200;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Next,
    In,
    Out,
    /// From the proxy's breakpoint on the main module to the first BASIC line.
    Entry,
}

pub struct Step {
    kind: Kind,
    /// The number of frames of the program's thread when the step began.
    depth: i64,
    count: u32,
    /// The last step on was a `stepOut` of code that is no BASIC code, so the stop after it may be in the middle of
    /// the line it was called from.
    returning: bool,
    /// The line (file, line) of the last stop in the middle of a line that was stepped on from: a stop on another
    /// line is at its start, whatever its column. `clang` on Windows gives each BASIC line one entry, at column 1;
    /// `g++` on Linux one per C++ statement, at its own column (`steps.bas:4:29`; measured, task 4.2 of `m7-linux`).
    mid_line: Option<(usize, u64)>,
}

/// A client's `next`, `stepIn` or `stepOut`.
pub fn start(p: &mut Proxy, m: Value) -> Result<(), String> {
    let Some(thread) = p.state.session.as_ref().and_then(|s| s.program_thread) else {
        return p.forward(m);
    };
    let kind = match m["command"].as_str() {
        Some("stepIn") => Kind::In,
        Some("stepOut") => Kind::Out,
        _ => Kind::Next,
    };
    let depth = match stack::backend_frames(p, &json!(thread)) {
        Ok((_, total)) => total,
        Err(e) => return p.respond(&m, Err(e)),
    };
    p.respond(&m, Ok(Value::Null))?;
    if let Some(s) = p.state.session.as_mut() {
        s.resumed();
        s.pausing = false;
        s.step = Some(Step {
            kind,
            depth,
            count: 0,
            returning: false,
            mid_line: None,
        });
    }
    let command = m["command"].as_str().unwrap_or("next").to_string();
    p.send(&command, json!({"threadId": thread}))
}

/// Starts the steps from the proxy's breakpoint on the main module to the first BASIC line.
fn start_entry(p: &mut Proxy, thread: i64) -> Result<(), String> {
    if let Some(s) = p.state.session.as_mut() {
        s.step = Some(Step {
            kind: Kind::Entry,
            depth: 0,
            count: 0,
            returning: false,
            mid_line: None,
        });
    }
    p.send("next", json!({"threadId": thread}))
}

/// What to do at a stop during a step.
#[derive(PartialEq, Eq, Debug)]
enum Then {
    StepOut,
    Next,
    StepIn,
    Report,
}

/// Whether a line's text holds no statement: blank, a comment or a metacommand.
fn no_statement(text: &[u8]) -> bool {
    let t = text.trim_ascii();
    t.is_empty()
        || t.starts_with(b"'")
        || t.starts_with(b"$")
        || t.get(..3).is_some_and(|w| w.eq_ignore_ascii_case(b"REM"))
}

fn decide(s: &crate::Session, step: &Step, frame: &Value, total: i64) -> Then {
    let Some(procedure) = s.basic_function(frame) else {
        return Then::StepOut;
    };
    let Some((_, file)) = s.basic_frame(frame) else {
        return Then::Next;
    };
    let line = frame["line"].as_u64().unwrap_or(0);
    let column = frame["column"].as_u64().unwrap_or(1);
    if let Some(pr) = procedure.map(|i| &s.symbols.procedures[i])
        && pr.file == file
        && pr.line == line
    {
        return Then::Next;
    }
    let new_line = step.mid_line.is_some_and(|at| at != (file, line));
    match step.kind {
        Kind::Entry if s.line_text(file, line).is_some_and(no_statement) => Then::Next,
        Kind::Entry => Then::Report,
        _ if column == 1 || new_line || !(total < step.depth || step.returning) => Then::Report,
        Kind::In => Then::StepIn,
        _ => Then::Next,
    }
}

/// The BASIC line of a stop that [`decide`] steps on from in the middle of a line, for [`Step::mid_line`].
fn mid_line_of(s: &crate::Session, frame: &Value) -> Option<(usize, u64)> {
    let (_, file) = s.basic_frame(frame)?;
    Some((file, frame["line"].as_u64()?))
}

/// A `stopped` event of the backend.
pub fn stopped(p: &mut Proxy, m: Value) -> Result<(), String> {
    let body = &m["body"];
    let reason = body["reason"].as_str().unwrap_or("").to_string();
    let thread = body["threadId"].as_i64().unwrap_or(0);
    let hits: Vec<i64> = body["hitBreakpointIds"]
        .as_array()
        .map(|h| h.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default();
    let Some(s) = p.state.session.as_mut() else {
        return p.send_client(m).map(|_| ());
    };
    match s.breakpoints.own_stop(&hits) {
        Some(Own::Main) => {
            s.program_thread = Some(thread);
            s.breakpoints.drop_main();
            let entry = s.stop_on_entry;
            s.starting = !entry;
            // The program goes on without the breakpoint's removal rather than stop where the client set nothing.
            if let Err(e) = breakpoints::sync(p) {
                p.event(
                    "output",
                    json!({"category": "console", "output": format!("qb64rust dap: {e}\n")}),
                )?;
            }
            return if entry {
                start_entry(p, thread)
            } else {
                p.send("continue", json!({"threadId": thread}))
            };
        }
        Some(Own::Error) => return error_stop(p, thread),
        None => {}
    }
    if reason == "step" && s.step.is_some() {
        return step_on(p, thread);
    }
    s.step = None;
    let is_program = s.program_thread.is_none_or(|t| t == thread);
    let pausing = std::mem::take(&mut s.pausing);
    // A pause stops the process on a thread of its own (`DbgUiRemoteBreakin`), reported as an exception. Any other
    // exception on another thread (a fault in a thread of libqb's) is reported as it came.
    // `lldb-dap` 19 reports a stop at a data breakpoint as `breakpoint`, described `watchpoint <n>`; later ones as
    // `data breakpoint` (measured, task 4.2 of `m7-linux`).
    let watchpoint = reason == "breakpoint"
        && body["description"]
            .as_str()
            .is_some_and(|d| d.starts_with("watchpoint "));
    let reason = if reason == "exception" && !is_program && pausing {
        "pause".to_string()
    } else if watchpoint {
        "data breakpoint".to_string()
    } else {
        reason
    };
    let client_hits: Vec<i64> = hits.into_iter().filter(|&h| !s.breakpoints.is_own(Some(h))).collect();
    let mut out = json!({"reason": reason, "allThreadsStopped": true});
    for key in ["description", "text"] {
        if !body[key].is_null() && reason != "pause" {
            out[key] = body[key].clone();
        }
    }
    if !client_hits.is_empty() {
        out["hitBreakpointIds"] = json!(client_hits);
    }
    report(p, out, thread)
}

/// Sends the client a stop on the program's thread.
pub(crate) fn report(p: &mut Proxy, mut body: Value, thread: i64) -> Result<(), String> {
    let program = match p.state.session.as_mut() {
        Some(s) => {
            s.step = None;
            s.starting = false;
            s.resumed();
            s.program_thread.unwrap_or(thread)
        }
        None => thread,
    };
    body["threadId"] = json!(program);
    body["allThreadsStopped"] = json!(true);
    p.event("stopped", body)
}

fn step_on(p: &mut Proxy, thread: i64) -> Result<(), String> {
    let (frames, total) = stack::backend_frames(p, &json!(thread))?;
    let Some(s) = p.state.session.as_mut() else {
        return Ok(());
    };
    let Some(step) = s.step.as_mut() else {
        return Ok(());
    };
    step.count += 1;
    let kind = step.kind;
    let reason = if kind == Kind::Entry { "entry" } else { "step" };
    if step.count > MAX_STEPS {
        p.event(
            "output",
            json!({"category": "console", "output": format!(
                "qb64rust dap: the step was cut short after {MAX_STEPS} steps of lldb's\n")}),
        )?;
        return report(p, json!({"reason": reason}), thread);
    }
    let then = match (frames.first(), s.step.as_ref()) {
        (Some(top), Some(step)) => decide(s, step, top, total),
        _ => Then::Report,
    };
    // A step on with `next` or `stepIn` from a BASIC line that was not reported is from the middle of that line.
    let mid = match (&then, frames.first()) {
        (Then::Next | Then::StepIn, Some(top)) => mid_line_of(s, top),
        _ => None,
    };
    if let Some(step) = s.step.as_mut() {
        step.returning = then == Then::StepOut;
        if mid.is_some() {
            step.mid_line = mid;
        }
    }
    match then {
        Then::Report => report(p, json!({"reason": reason}), thread),
        Then::Next => p.send("next", json!({"threadId": thread})),
        Then::StepIn => p.send("stepIn", json!({"threadId": thread})),
        Then::StepOut => p.send("stepOut", json!({"threadId": thread})),
    }
}

/// A stop on the proxy's breakpoint on libqb's `error` (design D10): the code is the first argument, in
/// [`crate::FIRST_ARG`].
fn error_stop(p: &mut Proxy, thread: i64) -> Result<(), String> {
    let (frames, _) = stack::backend_frames(p, &json!(thread))?;
    let frame = frames.first().map_or(Value::Null, |f| f["id"].clone());
    let code = p
        .call(
            "evaluate",
            json!({"expression": format!("(int){}", crate::FIRST_ARG), "frameId": frame, "context": "watch"}),
        )
        .ok()
        .and_then(|b| b["result"].as_str().and_then(|r| r.trim().parse::<i64>().ok()))
        .unwrap_or(0);
    let (code, text) = error_text(code);
    let description = format!("Error {code}: {text}");
    if let Some(s) = p.state.session.as_mut() {
        s.error = Some((code, text));
    }
    report(
        p,
        json!({"reason": "exception", "description": description, "text": format!("Error {code}")}),
        thread,
    )
}

#[cfg(test)]
mod tests {
    use super::no_statement;

    #[test]
    fn lines_without_a_statement() {
        assert!(no_statement(b"$CONSOLE:ONLY\r"));
        assert!(no_statement(b"  ' a comment"));
        assert!(no_statement(b"REM x"));
        assert!(no_statement(b""));
        assert!(!no_statement(b"x = 1"));
    }
}
