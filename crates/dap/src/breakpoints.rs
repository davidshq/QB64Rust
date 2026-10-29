//! The proxy's own function breakpoints, kept apart from the client's (design D9 of `m5-dap-proxy`): the one on the
//! main module that finds the program's thread (D4, D11) and the one on libqb's `error` of the exception filter
//! (D10). `setFunctionBreakpoints` replaces the whole set, so the backend always gets the union; the client sees
//! only its own.

use crate::proxy::Proxy;
use crate::symbols::Symbols;
use crate::{ERROR_FILTER, errors};
use serde_json::{Value, json};

pub struct Breakpoints {
    client: Vec<Value>,
    main_c_name: String,
    /// The breakpoint on the main module is wanted (until its first hit).
    main: bool,
    /// The breakpoint of the exception filter is wanted.
    error: bool,
    main_id: Option<i64>,
    error_id: Option<i64>,
    /// The union has been sent once.
    sent: bool,
}

/// What a stop on the proxy's own breakpoint is.
#[derive(PartialEq, Eq, Debug)]
pub enum Own {
    Main,
    Error,
}

impl Breakpoints {
    pub fn new(symbols: &Symbols) -> Breakpoints {
        Breakpoints {
            client: Vec::new(),
            main_c_name: symbols.main_c_name.clone(),
            main: true,
            error: false,
            main_id: None,
            error_id: None,
            sent: false,
        }
    }

    pub fn is_own(&self, id: Option<i64>) -> bool {
        id.is_some() && (id == self.main_id || id == self.error_id)
    }

    /// Whether a stop whose `hitBreakpointIds` are `hits` is the proxy's: only its own breakpoints were hit.
    pub fn own_stop(&self, hits: &[i64]) -> Option<Own> {
        if hits.is_empty() || !hits.iter().all(|&h| self.is_own(Some(h))) {
            return None;
        }
        if self.error_id.is_some_and(|e| hits.contains(&e)) {
            Some(Own::Error)
        } else {
            Some(Own::Main)
        }
    }

    pub fn drop_main(&mut self) {
        self.main = false;
    }
}

/// Sends the backend the union of the client's function breakpoints and the proxy's own; gives the backend's
/// answers to the client's, in their order.
pub fn sync(p: &mut Proxy) -> Result<Vec<Value>, String> {
    let Some(s) = p.state.session.as_mut() else {
        return Ok(Vec::new());
    };
    let b = &mut s.breakpoints;
    let mut list = b.client.clone();
    let mut own = Vec::new();
    if b.main {
        list.push(json!({"name": b.main_c_name}));
        own.push(Own::Main);
    }
    if b.error {
        list.push(json!({"name": "error", "condition": format!("(int){} != 0", crate::FIRST_ARG)}));
        own.push(Own::Error);
    }
    let n = b.client.len();
    b.sent = true;
    let body = p.call("setFunctionBreakpoints", json!({"breakpoints": list}))?;
    let answers = body["breakpoints"].as_array().cloned().unwrap_or_default();
    let Some(s) = p.state.session.as_mut() else {
        return Ok(Vec::new());
    };
    let b = &mut s.breakpoints;
    b.main_id = None;
    b.error_id = None;
    for (which, answer) in own.iter().zip(answers.iter().skip(n)) {
        let id = answer["id"].as_i64();
        match which {
            Own::Main => b.main_id = id,
            Own::Error => b.error_id = id,
        }
    }
    Ok(answers.into_iter().take(n).collect())
}

pub fn set_function(p: &mut Proxy, m: Value) -> Result<(), String> {
    let Some(s) = p.state.session.as_mut() else {
        return p.forward(m);
    };
    s.breakpoints.client = m["arguments"]["breakpoints"].as_array().cloned().unwrap_or_default();
    let result = sync(p).map(|bps| json!({"breakpoints": bps}));
    p.respond(&m, result)
}

/// The filter "BASIC runtime errors" is the proxy's; `lldb-dap`'s own filters are not offered.
pub fn set_exception(p: &mut Proxy, m: Value) -> Result<(), String> {
    let on = m["arguments"]["filters"]
        .as_array()
        .is_some_and(|f| f.iter().any(|f| f == ERROR_FILTER));
    let Some(s) = p.state.session.as_mut() else {
        return p.respond(&m, Ok(json!({"breakpoints": []})));
    };
    let changed = s.breakpoints.error != on;
    s.breakpoints.error = on;
    let sent = s.breakpoints.sent;
    let result = if changed || !sent { sync(p).map(|_| ()) } else { Ok(()) };
    let answer = result.map(|()| json!({"breakpoints": if on { vec![json!({"verified": true})] } else { vec![] }}));
    p.respond(&m, answer)
}

/// Before the program starts, the proxy's own breakpoints are set, also when the client set none of its own.
pub fn configuration_done(p: &mut Proxy, m: Value) -> Result<(), String> {
    if p.state.session.as_ref().is_some_and(|s| !s.breakpoints.sent)
        && let Err(e) = sync(p)
    {
        return p.respond(&m, Err(e));
    }
    p.forward(m)
}

pub fn exception_info(p: &mut Proxy, m: Value) -> Result<(), String> {
    let info = p.state.session.as_ref().and_then(|s| s.error.clone());
    let result = match info {
        Some((code, text)) => Ok(json!({
            "exceptionId": ERROR_FILTER,
            "description": format!("Error {code}: {text}"),
            "breakMode": "always",
            "details": {"message": text, "typeName": format!("Error {code}")},
        })),
        None => Err("the program is not stopped at a runtime error".into()),
    };
    p.respond(&m, result)
}

/// The text of the error stop of `code`.
pub fn error_text(code: i64) -> (i64, String) {
    (code, errors::text(code).to_string())
}
