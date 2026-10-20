//! `qb64rust`: the command line (design D9, spec `compiler/cli`).
//!
//! `qb64rust [-x] [-q] [-m] [-w] [-z] [-f:<setting>=<value>]... <file.bas> [-o <exe>] [--dump tokens|tree|typed|ir|cpp]
//! [--qb64pe-root <dir>] [--libqb-root <dir>] [--include-root <dir>] [--keep-build] [--coverage]`
//!
//! `qb64rust --report unsupported <file or folder>... [--include-root <dir>]`: what a set of programs still needs
//! (`report.rs`).
//!
//! `qb64rust lsp`: the language server (crate `qb64rust-lsp`).

use qb64rust_driver::{FileLoader, build, dump_cpp, dump_ir, emit_ir_with, frontend_at, lower, report};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;

const USAGE: &str = "usage: qb64rust [-x] [-q] [-m] [-w] [-z] [-f:<setting>=<value>]... <file.bas> [-o <exe>] \
[--dump tokens|tree|typed|ir|cpp] [--qb64pe-root <dir>] [--libqb-root <dir>] [--include-root <dir>] \
[--keep-build] [--coverage]\n\
       qb64rust --report unsupported <file or folder>... [--include-root <dir>]\n\
       qb64rust lsp    (the language server, over stdin and stdout)";

#[derive(Default)]
struct Options {
    /// One file to compile; with `--report`, any number of files and folders.
    inputs: Vec<PathBuf>,
    /// `--report unsupported`.
    report: bool,
    output: Option<PathBuf>,
    quiet: bool,
    cpp_only: bool,
    dump: Option<String>,
    root: Option<PathBuf>,
    /// `--libqb-root`: the copy of libqb a build uses (design D3 of `libqb-copy`).
    libqb_root: Option<PathBuf>,
    /// `--include-root`: where included files are looked up after the including file's folder.
    include_root: Option<PathBuf>,
    keep_build: bool,
    /// `--coverage`: a counter at each statement, written at exit to the file `QB64RUST_COVERAGE` names (design D1
    /// of `cws-whole-game`). The fragments differ, so the build cache's key does too.
    coverage: bool,
    /// `-w`: print the warnings too (spec `compiler/cli`).
    warnings: bool,
    /// `-f:<setting>=<value>` as given, checked by [`build_settings`].
    settings: Vec<String>,
}

/// The `-f:` settings: `OptimizeCppProgram=true` (as `qb64pe` turns it into `-O2`) and `ExtraLinkerFlags=<value>`
/// (added to the link command). `StripDebugSymbols` is accepted and ignored; any other setting is not supported
/// yet (spec `compiler/cli`).
fn build_settings(settings: &[String]) -> Result<build::Settings, String> {
    let mut out = build::Settings::default();
    for s in settings {
        let (name, value) = s.split_once('=').unwrap_or((s, ""));
        match name {
            "OptimizeCppProgram" => {
                out.optimize = match value.to_ascii_lowercase().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(format!("setting `{name}` needs `true` or `false`, not `{value}`")),
                }
            }
            "GenerateLicenseFile" => {
                out.license = match value.to_ascii_lowercase().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(format!("setting `{name}` needs `true` or `false`, not `{value}`")),
                }
            }
            "ExtraLinkerFlags" => out.extra_link = value.to_string(),
            "StripDebugSymbols" => {}
            _ => return Err(format!("setting `{name}` is not supported yet")),
        }
    }
    Ok(out)
}

fn parse_args() -> Result<Options, String> {
    let mut o = Options::default();
    let mut args = std::env::args_os().skip(1);
    while let Some(a) = args.next() {
        let s = a.to_string_lossy().to_string();
        let mut value = |what: &str| args.next().ok_or_else(|| format!("{what} needs a value"));
        match s.as_str() {
            // Accepted for the corpus runner and the M1 extension: -x (compile only, no IDE) and -m (plain
            // output) change nothing here.
            "-x" | "-m" => {}
            "-w" => o.warnings = true,
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
            "--libqb-root" => o.libqb_root = Some(PathBuf::from(value("--libqb-root")?)),
            "--include-root" => o.include_root = Some(PathBuf::from(value("--include-root")?)),
            "--keep-build" => o.keep_build = true,
            "--coverage" => o.coverage = true,
            "--report" => {
                let r = value("--report")?.to_string_lossy().to_string();
                if r != "unsupported" {
                    return Err(format!("unknown --report `{r}`"));
                }
                o.report = true;
            }
            _ if s.starts_with("-f:") => o.settings.push(s["-f:".len()..].to_string()),
            _ if s.starts_with('-') => return Err(format!("unknown option `{s}`")),
            _ => o.inputs.push(PathBuf::from(a)),
        }
    }
    if o.inputs.len() > 1 && !o.report {
        return Err("more than one input file".into());
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

/// A panic is an internal compiler error (`study\archive\21` item 6, design D11 of `m2-parser-breadth`): one message
/// with its location and the input file, no executable left behind, exit code 3.
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
    // A check runs the CLI's own front end (design D2 of `extension-diagnostics`).
    let check = qb64rust_driver::check_for_server;
    match qb64rust_driver::with_stack(move || qb64rust_lsp::run_stdio(root, check)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("qb64rust lsp: {msg}");
            ExitCode::FAILURE
        }
    }
}

/// The compiler root (spec `compiler/cli`): `--include-root`, resolved against the working directory now, or the
/// folder of this executable.
fn include_root(o: &Options) -> Result<PathBuf, String> {
    match &o.include_root {
        Some(r) => std::path::absolute(r).map_err(|e| e.to_string()),
        None => Ok(std::env::current_exe()
            .map_err(|e| e.to_string())?
            .parent()
            .map_or_else(PathBuf::new, Path::to_path_buf)),
    }
}

/// `--report unsupported`: the table of `report.rs` for the files and folders given, with the error cap lifted so
/// that a large program counts in full. The exit status is 0 whatever the programs hold: the report is no verdict.
fn report_unsupported(o: &Options) -> Result<ExitCode, String> {
    qb64rust_base::lift_error_cap();
    print!("{}", report::unsupported(&o.inputs, &include_root(o)?)?);
    Ok(ExitCode::SUCCESS)
}

fn run() -> Result<ExitCode, String> {
    let o = parse_args().map_err(|e| format!("{e}\n{USAGE}"))?;
    let input = o
        .inputs
        .first()
        .cloned()
        .ok_or_else(|| format!("no input file\n{USAGE}"))?;
    if o.report {
        return report_unsupported(&o);
    }
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
    let include_root = include_root(&o)?;
    let mut fe = frontend_at(
        &name,
        bytes,
        |file| Box::new(FileLoader::new(&include_root, file, &input)),
        Some((&input, &include_root)),
    );
    // The files of the program's `DECLARE LIBRARY` blocks, looked for from the clone as the old compiler's folder,
    // for a build, `-z` and the `cpp` dump; the other dumps need none, nor does a block with no name.
    let resolved = if matches!(o.dump.as_deref(), None | Some("cpp"))
        && !fe.has_errors()
        && fe.program.libraries.iter().any(|l| !l.names.is_empty())
    {
        let places = build::find_root(o.root.as_deref())
            .and_then(|root| qb64rust_driver::library::Places::new(&root, &input).map_err(|e| e.to_string()));
        let places = places.inspect_err(|_| {
            if o.dump.is_none() && !o.cpp_only {
                let _ = std::fs::remove_file(&exe);
            }
        })?;
        qb64rust_driver::library::resolve(&mut fe, &places)
    } else {
        qb64rust_driver::Resolved::default()
    };
    let errors = fe.diagnostics.error_count();
    // Warnings only with -w, never counted in the summary (spec `compiler/cli`).
    let report = |fe: &qb64rust_driver::Frontend| {
        let errors = fe.diagnostics.error_count();
        if o.warnings {
            print!("{}", fe.render_with_warnings());
        } else if errors > 0 {
            print!("{}", fe.render_diagnostics());
        }
        if errors > 0 {
            println!("{}", fe.diagnostics.summary());
        }
    };
    if let Some(stage) = o.dump.as_deref() {
        match stage {
            "tree" => print!("{}", qb64rust_syntax::dump_trees(&fe.parsed, &fe.map)),
            _ if errors > 0 => {}
            "typed" => print!("{}", qb64rust_sema::dump_typed(&fe.program)),
            "ir" => print!("{}", dump_ir(&fe)),
            "cpp" => print!("{}", dump_cpp(&fe, &resolved, o.coverage)),
            _ => unreachable!(),
        }
        report(&fe);
        return Ok(if errors > 0 {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        });
    }

    let mut settings = build_settings(&o.settings).inspect_err(|_| {
        let _ = std::fs::remove_file(&exe);
    })?;
    for lib in &resolved.libraries {
        settings.libraries.extend(lib.files.iter().cloned());
        settings.headers.extend(lib.headers.iter().cloned());
    }
    if fe.diagnostics.error_count() > 0 {
        let _ = std::fs::remove_file(&exe);
        report(&fe);
        return Ok(ExitCode::FAILURE);
    }
    report(&fe);

    let ir = lower(&fe);
    settings.flags = build::make_flags(&ir.settings);
    let fragments = emit_ir_with(&fe, &ir, &resolved, o.coverage);
    let build_dir = build::build_dir(&exe);
    if o.cpp_only {
        let temp = build::write_fragments(&build_dir, &fragments).map_err(|e| e.to_string())?;
        println!("{}", temp.display());
        return Ok(ExitCode::SUCCESS);
    }
    // Before any step that can fail, so a failed build never leaves an older executable behind.
    let _ = std::fs::remove_file(&exe);
    let roots = build::Roots {
        clone: build::find_root(o.root.as_deref())?,
        libqb: build::find_libqb(o.libqb_root.as_deref())?,
    };
    build::write_fragments(&build_dir, &fragments).map_err(|e| e.to_string())?;
    let icon = qb64rust_driver::exe_icon_path(&fe, &input, &include_root);
    let exe_name = exe
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    build::write_resources(
        &build_dir,
        icon.as_deref(),
        ir.settings.version_info.as_ref(),
        &exe_name,
    )?;
    build::write_embedded(
        &build_dir,
        &qb64rust_driver::used_embedded_files(&fe, &input, &include_root)?,
    )?;
    if let Err(e) = build::build(&roots, &build_dir, &exe, o.quiet, &settings) {
        // A failure that names only library functions is the program's (design D4 of `cpp-failure-errors`): its
        // errors as the front end's, with the summary; the build folder goes as after a build that worked.
        if let build::BuildError::Toolchain { output, .. } = &e
            && let Some(errors) = qb64rust_driver::cpp_errors::diagnostics(&fe, output)
        {
            for d in errors {
                fe.diagnostics.push(d);
            }
            print!("{}", fe.render_diagnostics());
            println!("{}", fe.diagnostics.summary());
            if !o.keep_build {
                let _ = std::fs::remove_dir_all(&build_dir);
            }
            return Ok(ExitCode::FAILURE);
        }
        return Err(e.to_string());
    }
    if !o.keep_build {
        let _ = std::fs::remove_dir_all(&build_dir);
    }
    if !o.quiet {
        println!("{}", Path::new(&exe).display());
    }
    Ok(ExitCode::SUCCESS)
}
