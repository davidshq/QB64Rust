//! What `evaluate` takes in BASIC terms (design D8 of `m5-dap-proxy`): a variable name with or without its suffix,
//! integer-constant subscripts, member paths, and a display suffix `,h`, `,b` or `,o`. Anything else is not a
//! place: in the Debug Console it goes to `lldb` as typed.

use crate::format::{Kind, Radix, parse_number, parse_string};
use crate::proxy::Proxy;
use crate::stack;
use crate::symbols::Shape;
use crate::vars::{self, Loc, REF_BASE, Val};
use crate::wire::base64_encode;
use serde_json::{Value, json};

/// A place a user named: `name[suffix] [(i, …)] {.member [(i, …)]} [,h|,b|,o]`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Place {
    /// In upper case.
    pub name: String,
    /// The type a suffix names (as the symbol file spells it), if one was written.
    pub suffix_type: Option<&'static str>,
    pub subscripts: Vec<i64>,
    /// Each member in upper case, with its own subscripts.
    pub members: Vec<(String, Vec<i64>)>,
    pub radix: Radix,
}

/// The type a suffix names, as the symbol file spells it. `$` is any string, `STRING` or `STRING * n`.
fn suffix_type(s: &str) -> Option<&'static str> {
    Some(match s {
        "%%" => "_BYTE",
        "~%%" => "_UNSIGNED _BYTE",
        "%" => "INTEGER",
        "~%" => "_UNSIGNED INTEGER",
        "&" => "LONG",
        "~&" => "_UNSIGNED LONG",
        "&&" => "_INTEGER64",
        "~&&" => "_UNSIGNED _INTEGER64",
        "!" => "SINGLE",
        "#" => "DOUBLE",
        "##" => "_FLOAT",
        "%&" => "_OFFSET",
        "~%&" => "_UNSIGNED _OFFSET",
        "$" => "STRING",
        "`" => "_BIT",
        "~`" => "_UNSIGNED _BIT",
        _ => return None,
    })
}

/// Whether a variable's type answers a suffix's.
pub fn suffix_matches(suffix: &str, ty: &str) -> bool {
    match suffix {
        "STRING" => ty == "STRING" || ty.starts_with("STRING *"),
        "_BIT" => ty == "_BIT" || ty.starts_with("_BIT *"),
        "_UNSIGNED _BIT" => ty == "_UNSIGNED _BIT" || ty.starts_with("_UNSIGNED _BIT *"),
        s => s == ty,
    }
}

struct Scan<'a> {
    s: &'a [u8],
    at: usize,
}

impl Scan<'_> {
    fn skip_space(&mut self) {
        while self.s.get(self.at).is_some_and(u8::is_ascii_whitespace) {
            self.at += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.at).copied()
    }

    fn eat(&mut self, b: u8) -> bool {
        self.skip_space();
        if self.peek() == Some(b) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    fn identifier(&mut self) -> Option<String> {
        self.skip_space();
        let start = self.at;
        if !self.peek().is_some_and(|b| b.is_ascii_alphabetic() || b == b'_') {
            return None;
        }
        while self.peek().is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_') {
            self.at += 1;
        }
        Some(
            self.s[start..self.at]
                .iter()
                .map(|&b| char::from(b.to_ascii_uppercase()))
                .collect(),
        )
    }

    fn suffix(&mut self) -> Option<Option<&'static str>> {
        let start = self.at;
        while self.peek().is_some_and(|b| b"~%&!#$`".contains(&b)) {
            self.at += 1;
        }
        if start == self.at {
            return Some(None);
        }
        let text: String = self.s[start..self.at].iter().map(|&b| char::from(b)).collect();
        suffix_type(&text).map(Some)
    }

    fn integer(&mut self) -> Option<i64> {
        self.skip_space();
        let negative = self.eat(b'-');
        self.skip_space();
        let start = self.at;
        while self.peek().is_some_and(|b| b.is_ascii_digit()) {
            self.at += 1;
        }
        let digits = &self.s[start..self.at];
        if digits.is_empty() {
            return None;
        }
        let n = digits
            .iter()
            .try_fold(0i64, |n, &d| n.checked_mul(10)?.checked_add(i64::from(d - b'0')))?;
        Some(if negative { -n } else { n })
    }

    /// `(i, …)` if one follows.
    fn subscripts(&mut self) -> Option<Vec<i64>> {
        if !self.eat(b'(') {
            return Some(Vec::new());
        }
        let mut out = vec![self.integer()?];
        while self.eat(b',') {
            out.push(self.integer()?);
        }
        self.eat(b')').then_some(out)
    }
}

/// Parses `text` as a place; `None` when it is not one.
pub fn parse(text: &str) -> Option<Place> {
    let mut sc = Scan {
        s: text.as_bytes(),
        at: 0,
    };
    let name = sc.identifier()?;
    let suffix_type = sc.suffix()?;
    let subscripts = sc.subscripts()?;
    let mut members = Vec::new();
    while sc.eat(b'.') {
        let m = sc.identifier()?;
        members.push((m, sc.subscripts()?));
    }
    let mut radix = Radix::Decimal;
    if sc.eat(b',') {
        sc.skip_space();
        radix = match sc.peek().map(|b| b.to_ascii_lowercase()) {
            Some(b'h' | b'x') => Radix::Hex,
            Some(b'b') => Radix::Binary,
            Some(b'o') => Radix::Octal,
            _ => return None,
        };
        sc.at += 1;
    }
    sc.skip_space();
    (sc.at == sc.s.len()).then_some(Place {
        name,
        suffix_type,
        subscripts,
        members,
        radix,
    })
}

/// The value a place names, seen from `frame`: the frame's procedure's own variables first, then the main module's.
/// A scalar and an array may share a name: a place with subscripts takes the array, one without the scalar, and
/// either takes the other only when there is no variable of its own shape. `Ok(None)` when no variable has the name.
fn resolve(p: &mut Proxy, frame: &Value, place: &Place) -> Result<Option<Val>, String> {
    let Some(s) = p.state.session.as_ref() else {
        return Ok(None);
    };
    let procedure = frame.as_i64().and_then(|f| s.frames.get(&f).copied()).flatten();
    let array = !place.subscripts.is_empty();
    let named = |v: &crate::symbols::Variable| {
        v.name == place.name && place.suffix_type.is_none_or(|t| suffix_matches(t, &v.ty))
    };
    let find = |shaped: bool| {
        let fits = |v: &&crate::symbols::Variable| named(v) && (!shaped || (v.shape != Shape::Scalar) == array);
        procedure
            .and_then(|pr| s.symbols.variables_of(Some(pr)).map(|(_, v)| v).find(fits))
            .or_else(|| s.symbols.variables_of(None).map(|(_, v)| v).find(fits))
    };
    let Some(var) = find(true).or_else(|| find(false)).cloned() else {
        return Ok(None);
    };
    let mut v = vars::variable_val(p, frame, &var);
    if !place.subscripts.is_empty() {
        v = vars::element_of(p, &v, &place.subscripts)?;
    }
    for (member, subs) in &place.members {
        v = vars::member_of(p, &v, member)?;
        if !subs.is_empty() {
            v = vars::element_of(p, &v, subs)?;
        }
    }
    Ok(Some(v))
}

const NOT_HERE: &str = "not a variable here";

pub fn evaluate(p: &mut Proxy, m: Value) -> Result<(), String> {
    let a = &m["arguments"];
    let text = a["expression"].as_str().unwrap_or("").to_string();
    let repl = a["context"] == "repl";
    let place = parse(&text);
    let found = match &place {
        Some(place) => resolve(p, &a["frameId"].clone(), place),
        None => Ok(None),
    };
    let (place, v) = match (place, found) {
        (Some(place), Ok(Some(v))) => (place, v),
        (_, Ok(None)) | (None, _) if repl => {
            // An `lldb` command can write memory: what this stop has read is read again.
            vars::written(p);
            return p.forward(m);
        }
        (_, Err(e)) => return p.respond(&m, Err(e)),
        _ => return p.respond(&m, Err(NOT_HERE.into())),
    };
    let radix = match place.radix {
        Radix::Decimal if a["format"]["hex"] == true => Radix::Hex,
        r => r,
    };
    let item = vars::item(p, &text, v, radix);
    let body = json!({"result": item["value"], "type": item["type"], "variablesReference": item["variablesReference"]});
    p.respond(&m, Ok(body))
}

/// Writes `text` into the place of `v` (spec "Set value"); gives the answer of `setVariable`.
fn write(p: &mut Proxy, v: &Val, text: &str) -> Result<Value, String> {
    let kind = Kind::of(&v.ty);
    let frame = v.frame.clone();
    let write_bytes = |p: &mut Proxy, at: u64, bytes: &[u8]| {
        if p.state.backend_writes_memory {
            return p
                .call(
                    "writeMemory",
                    json!({"memoryReference": format!("0x{at:x}"), "data": base64_encode(bytes)}),
                )
                .map(|_| ());
        }
        // `lldb-dap` 20 and older: `lldb`'s own command, one byte a value (`State::backend_writes_memory`).
        let values: String = bytes.iter().map(|b| format!(" 0x{b:02x}")).collect();
        let body = p.call(
            "evaluate",
            json!({"expression": format!("`memory write -s 1 0x{at:x}{values}"), "frameId": frame,
                   "context": "repl"}),
        )?;
        match body["result"].as_str() {
            Some(out) if out.contains("error:") => Err(out.trim().to_string()),
            _ => Ok(()),
        }
    };
    let fixed = |n: u64| {
        let mut b = parse_string(text);
        b.resize(usize::try_from(n).unwrap_or(0), b' ');
        b
    };
    match (&v.loc, &kind) {
        (Loc::Unallocated, _) => return Err(format!("{} is not allocated yet", v.path)),
        (Loc::Bytes(a), Kind::Integer { .. } | Kind::Offset { .. } | Kind::Single | Kind::Double | Kind::Float) => {
            let bytes = parse_number(&kind, text)?;
            write_bytes(p, *a, &bytes)?;
        }
        (Loc::Bytes(a), Kind::Fixed(n)) => write_bytes(p, *a, &fixed(*n))?,
        (Loc::Qbs(q), Kind::Fixed(n)) => {
            let (chr, _) = vars::string_at(p, *q)?;
            write_bytes(p, chr, &fixed(*n))?;
        }
        (Loc::Qbs(_) | Loc::QbsPtr(_), Kind::String) => {
            let q = match v.loc {
                Loc::QbsPtr(a) => {
                    let ptr = vars::read(p, a, 8)?;
                    u64::from_le_bytes(ptr.try_into().unwrap_or([0; 8]))
                }
                Loc::Qbs(q) => q,
                _ => 0,
            };
            if q == 0 {
                return Err(format!("{} is not allocated yet", v.path));
            }
            set_string(p, &v.frame, q, &parse_string(text))?;
        }
        _ => return Err(format!("a value of {} cannot be set here", v.ty)),
    }
    vars::written(p);
    let (value, _) = vars::shown(p, v, Radix::Decimal)?;
    Ok(json!({"value": value, "type": v.ty}))
}

/// A `STRING` set through libqb's own assignment (measured, task 3.1): `qbs_set` of the variable's `qbs` and a new
/// temporary one, called through function pointers of the C types, as libqb has no debug information.
fn set_string(p: &mut Proxy, frame: &Value, q: u64, bytes: &[u8]) -> Result<(), String> {
    let literal: String = bytes.iter().map(|b| format!("\\{b:03o}")).collect();
    let expression = format!(
        "((void*(*)(void*,void*))&qbs_set)((void*){q}, ((void*(*)(const char*,int))&qbs_new_txt_len)(\"{literal}\", {}))",
        bytes.len()
    );
    p.call(
        "evaluate",
        json!({"expression": expression, "frameId": frame, "context": "watch"}),
    )
    .map(|_| ())
    .map_err(|e| format!("the STRING could not be set through libqb: {e}"))
}

pub fn set_variable(p: &mut Proxy, m: Value) -> Result<(), String> {
    let a = &m["arguments"];
    let reference = a["variablesReference"].as_i64().unwrap_or(0);
    if reference <= REF_BASE {
        return p.forward(m);
    }
    let name = a["name"].as_str().unwrap_or("").to_string();
    let value = a["value"].as_str().unwrap_or("").to_string();
    let result = vars::listed(p, reference).and_then(|l| {
        let (_, v) = l
            .into_iter()
            .find(|(n, _)| *n == name)
            .ok_or_else(|| format!("no variable {name} here"))?;
        write(p, &v, &value)
    });
    p.respond(&m, result)
}

pub fn set_expression(p: &mut Proxy, m: Value) -> Result<(), String> {
    let a = &m["arguments"];
    let value = a["value"].as_str().unwrap_or("").to_string();
    let text = a["expression"].as_str().unwrap_or("");
    let result = match parse(text) {
        Some(place) => match resolve(p, &a["frameId"].clone(), &place) {
            Ok(Some(v)) => write(p, &v, &value),
            Ok(None) => Err(NOT_HERE.into()),
            Err(e) => Err(e),
        },
        None => Err(NOT_HERE.into()),
    };
    p.respond(&m, result)
}

/// A data breakpoint for a value of at most 8 bytes, in `lldb-dap`'s own form of `dataId` (`<hex address>/<size>`,
/// measured), so that `setDataBreakpoints` passes through.
fn data_info(p: &mut Proxy, v: &Val) -> Result<Value, String> {
    let kind = Kind::of(&v.ty);
    let at = match (&v.loc, &kind) {
        (Loc::Bytes(a), Kind::Integer { .. } | Kind::Offset { .. } | Kind::Single | Kind::Double) => {
            Some((*a, kind.size().unwrap_or(8)))
        }
        (Loc::Bytes(a), Kind::Fixed(n)) if *n <= 8 => Some((*a, *n)),
        (Loc::Qbs(q), Kind::Fixed(n)) if *n <= 8 => Some((vars::string_at(p, *q)?.0, *n)),
        _ => None,
    };
    let why = match (&v.loc, &kind) {
        (Loc::Unallocated, _) => "it is not allocated yet".to_string(),
        (_, Kind::String) => "a STRING's text moves when it changes".into(),
        (_, Kind::Float) => "a _FLOAT takes 10 bytes, more than the 8 a data breakpoint watches".into(),
        (_, Kind::Fixed(n)) => format!("a STRING * {n} takes more than the 8 bytes a data breakpoint watches"),
        (_, Kind::User(_)) => "a user type: watch one of its members".into(),
        _ => "it is not a value of up to 8 bytes".into(),
    };
    Ok(match at {
        Some((a, size)) => json!({
            "dataId": format!("{a:X}/{size}"),
            "description": format!("{} ({size} bytes)", v.path),
            "accessTypes": ["read", "write", "readWrite"],
            "canPersist": false,
        }),
        None => json!({"dataId": null, "description": format!("No data breakpoint for {}: {why}", v.path)}),
    })
}

pub fn data_breakpoint_info(p: &mut Proxy, m: Value) -> Result<(), String> {
    let a = &m["arguments"];
    let reference = a["variablesReference"].as_i64().unwrap_or(0);
    let name = a["name"].as_str().unwrap_or("").to_string();
    let found = if reference > REF_BASE {
        vars::listed(p, reference).map(|l| l.into_iter().find(|(n, _)| *n == name).map(|(_, v)| v))
    } else if reference == 0 {
        match parse(&name) {
            Some(place) => resolve(p, &a["frameId"].clone(), &place),
            None => Ok(None),
        }
    } else {
        return p.forward(m);
    };
    let result = match found {
        Ok(Some(v)) => data_info(p, &v),
        Ok(None) if reference == 0 => return p.forward(m),
        Ok(None) => Err(format!("no variable {name} here")),
        Err(e) => Err(e),
    };
    p.respond(&m, result)
}

/// The top BASIC frame of the program's thread: its id, procedure and file.
fn top_frame(p: &mut Proxy) -> Result<(Value, Option<usize>, usize), String> {
    let thread = json!(p.state.session.as_ref().and_then(|s| s.program_thread));
    let (frames, _) = stack::backend_frames(p, &thread)?;
    let s = p.state.session.as_ref().ok_or("no program is being debugged")?;
    frames
        .iter()
        .find_map(|f| s.basic_frame(f).map(|(pr, file)| (f["id"].clone(), pr, file)))
        .ok_or_else(|| "the program is not stopped in BASIC code".into())
}

/// The lines of the current frame's procedure (spec "Set next line"): between a procedure's header and its `END`,
/// or for the main module every line of the main file outside the procedures.
pub fn goto_targets(p: &mut Proxy, m: Value) -> Result<(), String> {
    let a = &m["arguments"];
    let line = a["line"].as_u64().unwrap_or(0);
    let path = a["source"]["path"].as_str().unwrap_or("").to_string();
    let result = top_frame(p).map(|(_, procedure, _)| {
        let Some(s) = p.state.session.as_ref() else {
            return json!({"targets": []});
        };
        let file = s.file_of(&path);
        let ok = match (procedure, file) {
            (_, None) => false,
            (Some(i), Some(f)) => {
                let pr = &s.symbols.procedures[i];
                pr.file == f && pr.end_file == f && line > pr.line && line <= pr.end_line
            }
            (None, Some(f)) => {
                f == s.symbols.main_file
                    && s.within(f, line)
                    && !s
                        .symbols
                        .procedures
                        .iter()
                        .any(|pr| pr.file == f && line >= pr.line && line <= pr.end_line)
            }
        };
        if ok {
            json!({"targets": [{"id": line, "label": format!("Line {line}"), "line": line}]})
        } else {
            json!({"targets": []})
        }
    });
    if let (Ok(t), Some(s)) = (&result, p.state.session.as_mut())
        && !t["targets"].as_array().is_none_or(Vec::is_empty)
    {
        s.goto_path = Some(path);
    }
    p.respond(&m, result)
}

/// `goto`: `lldb`'s `thread jump` through `lldb-dap`'s command prefix (measured, task 3.1), then a stop there. The
/// program's thread is selected first: a command runs on `lldb`'s selected thread, not the frame's, and on Linux the
/// program runs on a thread of its own, where `thread jump` from the main thread finds the line outside "the current
/// function" and refuses a line of several locations (measured, task 4.2 of `m7-linux`).
pub fn goto(p: &mut Proxy, m: Value) -> Result<(), String> {
    let line = m["arguments"]["targetId"].as_u64().unwrap_or(0);
    let path = p
        .state
        .session
        .as_ref()
        .and_then(|s| s.goto_path.clone())
        .unwrap_or_default();
    let file = std::path::Path::new(&path)
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().to_string());
    let thread = p.state.session.as_ref().and_then(|s| s.program_thread);
    let result = top_frame(p).and_then(|(frame, _, _)| {
        if let Some(t) = thread {
            p.call(
                "evaluate",
                json!({"expression": format!("`thread select -t {t}"), "frameId": frame, "context": "repl"}),
            )?;
        }
        let body = p.call(
            "evaluate",
            json!({"expression": format!("`thread jump --file \"{file}\" --line {line}"), "frameId": frame,
                   "context": "repl"}),
        )?;
        let out = body["result"].as_str().unwrap_or("");
        if out.contains("error:") {
            Err(out.trim().to_string())
        } else {
            Ok(Value::Null)
        }
    });
    let ok = result.is_ok();
    p.respond(&m, result)?;
    if ok {
        let thread = p.state.session.as_mut().map(|s| {
            s.resumed();
            s.program_thread
        });
        p.event(
            "stopped",
            json!({"reason": "goto", "threadId": thread.flatten(), "allThreadsStopped": true}),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_suffixes() {
        let p = parse("total").unwrap();
        assert_eq!((p.name.as_str(), p.suffix_type), ("TOTAL", None));
        assert_eq!(parse("twice&").unwrap().suffix_type, Some("LONG"));
        assert_eq!(parse("u~%").unwrap().suffix_type, Some("_UNSIGNED INTEGER"));
        assert_eq!(parse("s$").unwrap().suffix_type, Some("STRING"));
        assert_eq!(parse("x##").unwrap().suffix_type, Some("_FLOAT"));
        assert!(parse("x&%~").is_none(), "not a suffix");
    }

    #[test]
    fn subscripts_and_members() {
        let p = parse("a(1, -2)").unwrap();
        assert_eq!(p.subscripts, [1, -2]);
        let p = parse("many(1).p.nm").unwrap();
        assert_eq!(p.subscripts, [1]);
        assert_eq!(p.members, [("P".to_string(), vec![]), ("NM".to_string(), vec![])]);
        let p = parse("r.v(2)").unwrap();
        assert_eq!(p.members, [("V".to_string(), vec![2])]);
    }

    #[test]
    fn display_suffixes() {
        assert_eq!(parse("total,h").unwrap().radix, Radix::Hex);
        assert_eq!(parse("total, b").unwrap().radix, Radix::Binary);
        assert_eq!(parse("total,o").unwrap().radix, Radix::Octal);
        assert!(parse("total,q").is_none());
    }

    #[test]
    fn not_places() {
        for t in ["", "1 + 2", "a(i)", "a(1", "x + 1", "frame variable"] {
            assert!(parse(t).is_none(), "{t}");
        }
        // A bare word is a name; when no variable has it, the Debug Console gives it to lldb (`bt`).
        assert_eq!(parse("bt").unwrap().name, "BT");
    }

    #[test]
    fn suffix_types_match() {
        assert!(suffix_matches("STRING", "STRING * 4"));
        assert!(suffix_matches("LONG", "LONG"));
        assert!(!suffix_matches("LONG", "INTEGER"));
    }
}
