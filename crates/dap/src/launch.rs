//! `launch` (design D4 of `m5-dap-proxy`): the executable of a debug build and its symbol file, read once; the
//! backend's `launch` built from the client's. Stop on entry is the proxy's own: `lldb-dap`'s stops in the loader.
//! The proxy's breakpoint on the main module, set for every session, finds the program's thread; with
//! `stopOnEntry` the program is then stepped to its first BASIC line (D6).

use crate::proxy::Proxy;
use crate::symbols::Symbols;
use crate::{Session, breakpoints, stack};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn launch(p: &mut Proxy, m: Value) -> Result<(), String> {
    let a = &m["arguments"];
    let Some(program) = a["program"].as_str().map(PathBuf::from) else {
        return p.respond(
            &m,
            Err("launch needs `program`: the executable of a debug build".into()),
        );
    };
    let mut symbol_file = program.clone().into_os_string();
    symbol_file.push(".debug.json");
    let symbol_file = PathBuf::from(symbol_file);
    if !program.is_file() {
        return p.respond(&m, Err(format!("{} is not a file", program.display())));
    }
    if !symbol_file.is_file() {
        return p.respond(
            &m,
            Err(format!(
                "{} has no debug symbol file {}: build it with --debug",
                program.display(),
                symbol_file.display()
            )),
        );
    }
    // A debug build writes the symbol file after the executable is in place, so an executable newer than its symbol
    // file was written again by another build, whose symbols these are not: QB64pe's leaves the symbol file (a
    // `qb64rust` build without `--debug` removes it).
    let modified = |f: &Path| std::fs::metadata(f).and_then(|m| m.modified()).ok();
    if let (Some(exe), Some(symbols)) = (modified(&program), modified(&symbol_file))
        && exe > symbols
    {
        return p.respond(
            &m,
            Err(format!(
                "{} is newer than its debug symbol file {}: it was built again without --debug; build it with --debug",
                program.display(),
                symbol_file.display()
            )),
        );
    }
    let symbols = match Symbols::read(&symbol_file) {
        Ok(s) => s,
        Err(e) => return p.respond(&m, Err(format!("{e}: build the program again with --debug"))),
    };
    let exe_dir = program.parent().map_or_else(PathBuf::new, Path::to_path_buf);
    let files = stack::file_infos(&symbols, &exe_dir);
    let names = stack::procedure_names(&symbols, &files);
    let mut env = match &a["env"] {
        Value::Object(o) => o.clone(),
        Value::Array(list) => list
            .iter()
            .filter_map(|e| e.as_str()?.split_once('='))
            .map(|(k, v)| (k.to_string(), json!(v)))
            .collect(),
        _ => serde_json::Map::new(),
    };
    if a["noPrompt"] == true {
        env.insert("QB64PE_NOPROMPT".into(), json!("y"));
    }
    let mut backend = json!({
        "program": program,
        "cwd": a["cwd"].as_str().map_or(exe_dir.clone(), PathBuf::from),
        "env": env,
        "stopOnEntry": false,
    });
    if let Some(init) = a.get("initCommands") {
        backend["initCommands"] = init.clone();
    }
    if let Some(args) = a.get("args") {
        backend["args"] = args.clone();
    }
    if let Some(console) = a["console"].as_str() {
        backend["console"] = json!(console);
        if console == "integratedTerminal" || console == "externalTerminal" {
            backend["runInTerminal"] = json!(true);
        }
    }
    p.state.session = Some(Session {
        breakpoints: breakpoints::Breakpoints::new(&symbols),
        symbols,
        files,
        names,
        program_thread: None,
        stop_on_entry: a["stopOnEntry"] == true,
        starting: false,
        pausing: false,
        step: None,
        refs: crate::vars::Refs::default(),
        frames: HashMap::new(),
        error: None,
        goto_path: None,
        memory: Vec::new(),
    });
    let mut m = m;
    m["arguments"] = backend;
    p.forward(m)
}
