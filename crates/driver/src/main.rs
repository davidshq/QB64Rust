//! `qb64rust`: the command line (design D9, spec `compiler/cli`).
//!
//! `qb64rust [-x] [-q] [-m] [-w] [-z] [-f:<setting>=<value>]... <file.bas> [-o <exe>] [--dump tokens|tree|typed|ir|cpp]
//! [--qb64pe-root <dir>] [--include-root <dir>] [--keep-build]`
//!
//! `qb64rust lsp`: the language server (crate `qb64rust-lsp`).

use qb64rust_driver::{FileLoader, build, dump_cpp, dump_ir, emit, frontend_with};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;

const USAGE: &str = "usage: qb64rust [-x] [-q] [-m] [-w] [-z] [-f:<setting>=<value>]... <file.bas> [-o <exe>] \
[--dump tokens|tree|typed|ir|cpp] [--qb64pe-root <dir>] [--include-root <dir>] [--keep-build]\n\
       qb64rust lsp     (the language server, over stdin and stdout)";

#[derive(Default)]
struct Options {
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    quiet: bool,
    cpp_only: bool,
    dump: Option<String>,
    root: Option<PathBuf>,
    /// `--include-root`: where included files are looked up after the including file's folder.
    include_root: Option<PathBuf>,
    keep_build: bool,
    /// `-f:<setting>=<value>` as given, checked by [`optimize_setting`].
    settings: Vec<String>,
}

/// Whether to optimise the C++ (`-f:OptimizeCppProgram=true`, as `qb64pe` turns it into `-O2`).
/// `-f:StripDebugSymbols` is accepted and ignored; any other setting is not supported yet (spec `compiler/cli`).
fn optimize_setting(settings: &[String]) -> Result<bool, String> {
    let mut optimize = false;
    for s in settings {
        let (name, value) = s.split_once('=').unwrap_or((s, ""));
        match name {
            "OptimizeCppProgram" => {
                optimize = match value.to_ascii_lowercase().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(format!("setting `{name}` needs `true` or `false`, not `{value}`")),
                }
            }
            "StripDebugSymbols" => {}
            _ => return Err(format!("setting `{name}` is not supported yet")),
        }
    }
    Ok(optimize)
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
            "--include-root" => o.include_root = Some(PathBuf::from(value("--include-root")?)),
            "--keep-build" => o.keep_build = true,
            _ if s.starts_with("-f:") => o.settings.push(s["-f:".len()..].to_string()),
            _ if s.starts_with('-') => return Err(format!("unknown option `{s}`")),
            _ if o.input.is_some() => return Err("more than one input file".into()),
            _ => o.input = Some(PathBuf::from(a)),
        }
    }
    Ok(o)
}

/// What the panic hook needs to know about the run: the input file, and the executable to remove (none in `--dump`
/// and `-z` runs, which never touch it).
struct PanicContext {
    input: Option<String>,
    exe: Option<PathBuf>,
}

static PANIC_CONTEXT: Mutex<PanicContext> = Mutex::new(PanicContext { input: None, exe: None });

/// Exit code of an internal compiler error; every other failure exits with 1 (spec `compiler/cli`).
const ICE_EXIT_CODE: i32 = 3;

/// A panic is an internal compiler error (`study\21` item 6, design D11 of `m2-parser-breadth`): one message with
/// its location and the input file, no executable left behind, exit code 3.
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let msg = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
            .unwrap_or("(no message)");
        let at = info
            .location()
            .map_or(String::new(), |l| format!(" at {}:{}", l.file(), l.line()));
        // A panic while the lock was held must not hide the report.
        let ctx = PANIC_CONTEXT.lock().unwrap_or_else(|e| e.into_inner());
        println!("qb64rust: internal compiler error: {msg}{at}");
        if let Some(input) = &ctx.input {
            println!("qb64rust: while compiling {input}");
        }
        println!("qb64rust: this is a bug in qb64rust, not in the program");
        if let Some(exe) = &ctx.exe {
            let _ = std::fs::remove_file(exe);
        }
        let _ = std::io::stdout().flush();
        std::process::exit(ICE_EXIT_CODE);
    }));
}

fn main() -> ExitCode {
    // `qb64rust lsp`: the exact word as the only argument (spec `compiler/cli`); `lsp.bas` is a file to compile.
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.first().is_some_and(|a| a == "lsp") {
        return lsp(args.len());
    }
    install_panic_hook();
    match qb64rust_driver::with_stack(run) {
        Ok(code) => code,
        Err(msg) => {
            println!("qb64rust: {msg}");
            ExitCode::FAILURE
        }
    }
}

/// The language server over stdin and stdout (spec `editor/language-server`). Stdout carries the protocol, so the
/// compile run's panic hook (which prints there) is not installed: a panic is reported on stderr, which the editor
/// shows in the server's output.
fn lsp(arg_count: usize) -> ExitCode {
    if arg_count > 1 {
        eprintln!("qb64rust: `lsp` takes no other argument\n{USAGE}");
        return ExitCode::FAILURE;
    }
    // The include root when the client names none: the folder of this executable, as for a compile.
    let root = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(Path::to_path_buf))
        .unwrap_or_default();
    match qb64rust_driver::with_stack(move || qb64rust_lsp::run_stdio(root)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("qb64rust lsp: {msg}");
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
    let exe = match &o.output {
        Some(p) => p.clone(),
        None => input.with_extension("exe"),
    };
    let exe = std::path::absolute(&exe).map_err(|e| e.to_string())?;
    {
        let mut ctx = PANIC_CONTEXT.lock().unwrap_or_else(|e| e.into_inner());
        ctx.input = Some(name.clone());
        ctx.exe = (o.dump.is_none() && !o.cpp_only).then(|| exe.clone());
    }
    // For the CLI test of the panic hook only.
    if std::env::var_os("QB64RUST_TEST_PANIC").is_some_and(|v| v == "1") {
        panic!("QB64RUST_TEST_PANIC=1 is set");
    }

    if o.dump.as_deref() == Some("tokens") {
        print!("{}", qb64rust_syntax::dump_tokens(&bytes));
        return Ok(ExitCode::SUCCESS);
    }
    // The compiler root (spec `compiler/cli`): `--include-root`, resolved against the working directory now, or the
    // folder of this executable.
    let include_root = match &o.include_root {
        Some(r) => std::path::absolute(r).map_err(|e| e.to_string())?,
        None => std::env::current_exe()
            .map_err(|e| e.to_string())?
            .parent()
            .map_or_else(PathBuf::new, Path::to_path_buf),
    };
    let fe = frontend_with(&name, bytes, |file| {
        Box::new(FileLoader::new(&include_root, file, &input))
    });
    let errors = fe.diagnostics.error_count();
    let report = |fe: &qb64rust_driver::Frontend| {
        if errors > 0 {
            print!("{}", fe.render_diagnostics());
            println!("{}", fe.diagnostics.summary());
        }
    };
    if let Some(stage) = o.dump.as_deref() {
        match stage {
            "tree" => print!("{}", qb64rust_syntax::dump_trees(&fe.parsed, &fe.map)),
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

    let optimize = optimize_setting(&o.settings).inspect_err(|_| {
        let _ = std::fs::remove_file(&exe);
    })?;
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
    build::build(&root, &build_dir, &exe, o.quiet, optimize)?;
    if !o.keep_build {
        let _ = std::fs::remove_dir_all(&build_dir);
    }
    if !o.quiet {
        println!("{}", Path::new(&exe).display());
    }
    Ok(ExitCode::SUCCESS)
}
