//! `qb64rust`: the command line (design D9, spec `compiler/cli`).
//!
//! `qb64rust [-x] [-q] [-m] [-w] [-z] <file.bas> [-o <exe>] [--dump tokens|tree|typed|ir|cpp]
//! [--qb64pe-root <dir>] [--keep-build]`

use qb64rust_driver::{build, dump_cpp, dump_ir, emit, frontend};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage: qb64rust [-x] [-q] [-m] [-w] [-z] <file.bas> [-o <exe>] \
[--dump tokens|tree|typed|ir|cpp] [--qb64pe-root <dir>] [--keep-build]";

#[derive(Default)]
struct Options {
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    quiet: bool,
    cpp_only: bool,
    dump: Option<String>,
    root: Option<PathBuf>,
    keep_build: bool,
}

fn parse_args() -> Result<Options, String> {
    let mut o = Options::default();
    let mut args = std::env::args_os().skip(1);
    while let Some(a) = args.next() {
        let s = a.to_string_lossy().to_string();
        let mut value = |what: &str| args.next().ok_or_else(|| format!("{what} needs a value"));
        match s.as_str() {
            // Accepted for the corpus runner and the M1 extension: -x (compile only, no IDE) and -m (plain
            // output) change nothing here; -w (show warnings) has nothing to show yet.
            "-x" | "-m" | "-w" => {}
            "-q" => o.quiet = true,
            "-z" => o.cpp_only = true,
            "-o" => o.output = Some(PathBuf::from(value("-o")?)),
            "--dump" => {
                let d = value("--dump")?.to_string_lossy().to_string();
                if !["tokens", "tree", "typed", "ir", "cpp"].contains(&d.as_str()) {
                    return Err(format!("unknown --dump stage `{d}`"));
                }
                o.dump = Some(d);
            }
            "--qb64pe-root" => o.root = Some(PathBuf::from(value("--qb64pe-root")?)),
            "--keep-build" => o.keep_build = true,
            _ if s.starts_with('-') => return Err(format!("unknown option `{s}`")),
            _ if o.input.is_some() => return Err("more than one input file".into()),
            _ => o.input = Some(PathBuf::from(a)),
        }
    }
    Ok(o)
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(msg) => {
            println!("qb64rust: {msg}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let o = parse_args().map_err(|e| format!("{e}\n{USAGE}"))?;
    let input = o.input.clone().ok_or_else(|| format!("no input file\n{USAGE}"))?;
    let bytes = std::fs::read(&input).map_err(|e| format!("cannot read {}: {e}", input.display()))?;
    if bytes.len() > qb64rust_base::MAX_SOURCE_LEN {
        let max = qb64rust_base::MAX_SOURCE_LEN;
        return Err(format!(
            "{} is too large: a source file has at most {max} bytes",
            input.display()
        ));
    }
    let name = input.to_string_lossy().to_string();

    if o.dump.as_deref() == Some("tokens") {
        print!("{}", qb64rust_syntax::dump_tokens(&bytes));
        return Ok(ExitCode::SUCCESS);
    }
    let fe = frontend(&name, bytes);
    let errors = fe.diagnostics.error_count();
    let report = |fe: &qb64rust_driver::Frontend| {
        if errors > 0 {
            print!("{}", fe.render_diagnostics());
            println!("{errors} error{}", if errors == 1 { "" } else { "s" });
        }
    };
    if let Some(stage) = o.dump.as_deref() {
        match stage {
            "tree" => print!("{}", qb64rust_syntax::dump_tree(fe.root(), fe.bytes())),
            _ if errors > 0 => {}
            "typed" => print!("{}", qb64rust_sema::dump_typed(&fe.program)),
            "ir" => print!("{}", dump_ir(&fe)),
            "cpp" => print!("{}", dump_cpp(&fe)),
            _ => unreachable!(),
        }
        report(&fe);
        return Ok(if errors > 0 {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        });
    }

    let exe = match &o.output {
        Some(p) => p.clone(),
        None => input.with_extension("exe"),
    };
    let exe = std::path::absolute(&exe).map_err(|e| e.to_string())?;
    if errors > 0 {
        let _ = std::fs::remove_file(&exe);
        report(&fe);
        return Ok(ExitCode::FAILURE);
    }

    let fragments = emit(&fe);
    let build_dir = build::build_dir(&exe);
    if o.cpp_only {
        let temp = build::write_fragments(&build_dir, &fragments).map_err(|e| e.to_string())?;
        println!("{}", temp.display());
        return Ok(ExitCode::SUCCESS);
    }
    // Before any step that can fail, so a failed build never leaves an older executable behind.
    let _ = std::fs::remove_file(&exe);
    let root = build::find_root(o.root.as_deref())?;
    build::write_fragments(&build_dir, &fragments).map_err(|e| e.to_string())?;
    build::build(&root, &build_dir, &exe, o.quiet)?;
    if !o.keep_build {
        let _ = std::fs::remove_dir_all(&build_dir);
    }
    if !o.quiet {
        println!("{}", Path::new(&exe).display());
    }
    Ok(ExitCode::SUCCESS)
}
