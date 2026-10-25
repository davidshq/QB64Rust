//! Which frames are BASIC code, the stack in BASIC terms, and the one thread (designs D5 and D11 of `m5-dap-proxy`).
//! A frame is BASIC code when its function is the main module's or a procedure's, its source is a file the symbol
//! file lists, and its line is within that file. The line count is not in the symbol file: the proxy reads each file
//! once.

use crate::Session;
use crate::proxy::Proxy;
use crate::symbols::{Kind, Symbols};
use serde_json::{Value, json};
use std::path::Path;

/// A file of the symbol file as the proxy matches frames against it.
pub struct FileInfo {
    /// The path as `#line` names it, in lower case with `\` separators.
    key: String,
    relative: bool,
    /// The file's lines, when it could be read.
    pub lines: Option<Vec<Vec<u8>>>,
}

fn normalise(path: &str) -> String {
    path.replace('/', "\\").to_ascii_lowercase()
}

/// The files of `symbols`, read from beside the executable when their paths are relative.
pub fn file_infos(symbols: &Symbols, exe_dir: &Path) -> Vec<FileInfo> {
    symbols
        .files
        .iter()
        .map(|f| {
            let path = Path::new(&f.path);
            let relative = path.is_relative();
            let on_disk = if relative {
                exe_dir.join(path)
            } else {
                path.to_path_buf()
            };
            let lines = std::fs::read(on_disk).ok().map(|bytes| {
                let mut lines: Vec<Vec<u8>> = bytes.split(|&b| b == b'\n').map(<[u8]>::to_vec).collect();
                if lines.last().is_some_and(Vec::is_empty) {
                    lines.pop();
                }
                lines
            });
            FileInfo {
                key: normalise(&f.path),
                relative,
                lines,
            }
        })
        .collect()
}

/// Each procedure's name as declared (`SUB show`, `FUNCTION twice&`), read from its header line; the symbol file's
/// name in upper case when the line cannot be read.
pub fn procedure_names(symbols: &Symbols, files: &[FileInfo]) -> Vec<String> {
    symbols
        .procedures
        .iter()
        .map(|pr| {
            let keyword = match pr.kind {
                Kind::Sub => "SUB",
                Kind::Function => "FUNCTION",
            };
            let declared = files
                .get(pr.file)
                .and_then(|f| f.lines.as_ref())
                .and_then(|lines| lines.get(usize::try_from(pr.line).ok()?.checked_sub(1)?))
                .and_then(|line| {
                    let text = qb64rust_base::show_bytes(line);
                    let rest = text.trim_start();
                    let word = rest.get(..keyword.len())?;
                    if !word.eq_ignore_ascii_case(keyword) {
                        return None;
                    }
                    let name: String = rest[keyword.len()..]
                        .trim_start()
                        .chars()
                        .take_while(|c| !c.is_whitespace() && *c != '(' && *c != '\'')
                        .collect();
                    (!name.is_empty()).then_some(name)
                });
            format!("{keyword} {}", declared.unwrap_or_else(|| pr.name.clone()))
        })
        .collect()
}

impl Session {
    /// The index of the listed file a frame's source path names.
    pub fn file_of(&self, path: &str) -> Option<usize> {
        let path = normalise(path);
        self.files
            .iter()
            .position(|f| path == f.key || (f.relative && path.ends_with(&format!("\\{}", f.key))))
    }

    /// Whether `line` is within file `file`'s lines (also when the file could not be read).
    pub fn within(&self, file: usize, line: u64) -> bool {
        match self.files.get(file).and_then(|f| f.lines.as_ref()) {
            Some(lines) => line >= 1 && line <= lines.len() as u64,
            None => true,
        }
    }

    /// The text of a line of a listed file.
    pub fn line_text(&self, file: usize, line: u64) -> Option<&[u8]> {
        let lines = self.files.get(file)?.lines.as_ref()?;
        lines
            .get(usize::try_from(line).ok()?.checked_sub(1)?)
            .map(Vec::as_slice)
    }

    /// Whether a frame's function is the main module's or a procedure's: `Some` with the procedure (`None` for the
    /// main module).
    pub fn basic_function(&self, frame: &Value) -> Option<Option<usize>> {
        let name = frame["name"].as_str()?;
        if self.symbols.is_main(name) {
            Some(None)
        } else {
            self.symbols.procedure_of_function(name).map(Some)
        }
    }

    /// Whether a frame is BASIC code (design D5): `Some` with its procedure, and its file.
    pub fn basic_frame(&self, frame: &Value) -> Option<(Option<usize>, usize)> {
        let procedure = self.basic_function(frame)?;
        let file = self.file_of(frame["source"]["path"].as_str()?)?;
        self.within(file, frame["line"].as_u64()?).then_some((procedure, file))
    }

    /// The name a BASIC frame is shown by.
    pub fn frame_name(&self, procedure: Option<usize>) -> String {
        match procedure {
            Some(p) => self.names[p].clone(),
            None => "Main module".into(),
        }
    }
}

/// The program's thread, else the thread the client named.
pub fn thread(p: &Proxy, m: &Value) -> Value {
    match p.state.session.as_ref().and_then(|s| s.program_thread) {
        Some(t) => json!(t),
        None => m["arguments"]["threadId"].clone(),
    }
}

/// Every frame of `thread` as the backend gives them.
pub fn backend_frames(p: &mut Proxy, thread: &Value) -> Result<(Vec<Value>, i64), String> {
    let body = p.call(
        "stackTrace",
        json!({"threadId": thread, "startFrame": 0, "levels": 1000}),
    )?;
    let total = body["totalFrames"].as_i64().unwrap_or(0);
    Ok((body["stackFrames"].as_array().cloned().unwrap_or_default(), total))
}

pub fn stack_trace(p: &mut Proxy, m: Value) -> Result<(), String> {
    let thread = thread(p, &m);
    let frames = match backend_frames(p, &thread) {
        Ok((f, _)) => f,
        Err(e) => return p.respond(&m, Err(e)),
    };
    let Some(s) = p.state.session.as_mut() else {
        return p.forward(m);
    };
    let mut shown = Vec::new();
    for f in &frames {
        let Some((procedure, _)) = s.basic_frame(f) else {
            continue;
        };
        let Some(id) = f["id"].as_i64() else {
            continue;
        };
        s.frames.insert(id, procedure);
        let path = f["source"]["path"].as_str().unwrap_or("");
        let name = Path::new(path)
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_string());
        shown.push(json!({
            "id": id,
            "name": s.frame_name(procedure),
            "line": f["line"],
            "column": f["column"],
            "source": {"name": name, "path": path},
            "instructionPointerReference": f["instructionPointerReference"],
        }));
    }
    let total = shown.len();
    let start = usize::try_from(m["arguments"]["startFrame"].as_u64().unwrap_or(0)).unwrap_or(0);
    let levels = usize::try_from(m["arguments"]["levels"].as_u64().unwrap_or(0)).unwrap_or(0);
    let page: Vec<Value> = shown
        .into_iter()
        .skip(start)
        .take(if levels == 0 { usize::MAX } else { levels })
        .collect();
    p.respond(&m, Ok(json!({"stackFrames": page, "totalFrames": total})))
}

pub fn threads(p: &mut Proxy, m: Value) -> Result<(), String> {
    match p.state.session.as_ref().and_then(|s| s.program_thread) {
        Some(t) => p.respond(&m, Ok(json!({"threads": [{"id": t, "name": "Program"}]}))),
        None => p.forward(m),
    }
}
