//! `qb64rust`: the command line (design D9, spec `compiler/cli`).
//!
//! `qb64rust [-x] [-q] [-m] [-w] [-z|--print-build-inputs] [-f:<setting>=<value>]... <file.bas> [-o <exe>]
//! [--dump tokens|tree|typed|ir|cpp|debug-info]
//! [--toolchain-root <dir>] [--qb64pe-root <dir>] [--libqb-root <dir>] [--include-root <dir>] [--keep-build]
//! [--coverage] [--trace-values] [--debug] [--call-trace|--no-call-trace] [--fixes on|off] [--fix <name>=on|off]...
//! [--target host|wasm]`
//!
//! `--target wasm` (experimental) builds a `$CONSOLE:ONLY` program to `<name>.js` and `<name>.wasm` for Node (spec
//! `compiler/cli`, "The target option"; OpenSpec change `wasm-console`), with the Emscripten SDK of `--emsdk-root
//! <dir>` or `QB64RUST_EMSDK_ROOT`. `qb64rust toolchain path --target wasm [--emsdk-root <dir>]` names that SDK.
//!
//! Without `-o` the program goes beside its source, unless `qb64rust.toml`'s `[build]` names an output folder or
//! has a `copy` list (spec `compiler/cli`, "The output folder"). `qb64rust --print-paths <file.bas> [--target
//! host|wasm] [--no-settings]` prints that path and the folder the program runs in, for the editor.
//!
//! `qb64rust init [<folder>] [--yes] [--plan] ...`: writes a folder's `qb64rust.toml` from a few questions
//! (`init.rs`, spec `compiler/project-init`).
//!
//! `qb64rust --list-fixes`: the fixes, one per line (spec `compiler/fixes`, `fixes.rs`).
//!
//! `qb64rust -y [-q] [-m] [-w] [-f:<layout setting>=<value>]... <file.bas> -o <out.bas> [--include-root <dir>]
//! [--program <main.bas>]`: the file laid out as `qb64pe -y` lays it out, alone or as a file of the program
//! `--program` names (spec `compiler/format`).
//!
//! `qb64rust --report unsupported|warnings <file or folder>... [--include-root <dir>]`: what a set of programs still
//! needs, or its lint warnings counted by code (`report.rs`).
//!
//! `qb64rust lsp`: the language server (crate `qb64rust-lsp`).
//!
//! `qb64rust dap [--toolchain-root <dir>] [--qb64pe-root <dir>]`: the debug adapter (crate `qb64rust-dap`).
//!
//! `qb64rust toolchain install [--from-zip <zip>]` and `qb64rust toolchain path [--toolchain-root <dir>]
//! [--qb64pe-root <dir>]`: the C++ toolchain (`toolchain.rs`, spec `compiler/toolchain`).
//!
//! `qb64rust --old-wording-words` and `qb64rust --old-wording-keys`: for `tools\diagnostics\old_wording.py` only,
//! not in the usage (spec `compiler/cli`, "Old-wording table tooling").

use qb64rust_base::Platform;
use qb64rust_base::fixes::{FIXES, Fixes};
use qb64rust_base::layout::{Overrides, Settings};
use qb64rust_driver::fixes::CliFix;
use qb64rust_driver::lints::{CliLint, Lints};
use qb64rust_driver::{FileLoader, compile, fixes, frontend_explicit, lints, output, report, settings, toolchain};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;

const USAGE: &str = "usage: qb64rust [-x|-c] [-q] [-m] [-w] [-e] [-p] [-z|--print-build-inputs] [-f:<setting>=<value>]... <file.bas> [-o <exe>] \
[--dump tokens|tree|typed|ir|cpp|debug-info] [--toolchain-root <dir>] [--qb64pe-root <dir>] [--libqb-root <dir>] \
[--include-root <dir>] [--keep-build] [--coverage] [--trace-values] [--debug] [--call-trace|--no-call-trace] \
[--fixes on|off] [--fix <name>=on|off]... [--lints on|off] [--lint <code>=on|off]... [--no-settings] [--target host|wasm] \
[--emsdk-root <dir>]\n\
       (--target wasm is experimental: a $CONSOLE:ONLY program built to <name>.js and <name>.wasm, run by node)\n\
       qb64rust -y [-q] [-m] [-w] [-f:<layout setting>=<value>]... [--format-default <layout setting>=<value>]... \
<file.bas> -o <out.bas> [--include-root <dir>] [--fixes on|off] [--fix <name>=on|off]... [--lints on|off] \
[--lint <code>=on|off]... [--no-settings] [--program <main.bas>]\n\
       qb64rust --print-paths <file.bas> [--target host|wasm] [--no-settings]    (the program's path and the folder \
it runs in, as JSON)\n\
       qb64rust --list-fixes    (the fixes --fix and qb64rust.toml name)\n\
       qb64rust --report unsupported|warnings <file or folder>... [--include-root <dir>]\n\
       qb64rust lsp    (the language server, over stdin and stdout)\n\
       qb64rust dap [--toolchain-root <dir>] [--qb64pe-root <dir>]    (the debug adapter, over stdin and stdout)\n\
       qb64rust toolchain install [--from-zip <zip>]    (downloads and installs the C++ toolchain, or installs \
it from the release's zip; on Linux names the packages)\n\
       qb64rust toolchain path [--toolchain-root <dir>] [--qb64pe-root <dir>]    (the toolchain a build would use)\n\
       qb64rust toolchain path --target wasm [--emsdk-root <dir>]    (the Emscripten SDK a wasm build would use)\n\
       qb64rust toolchain install --target wasm    (downloads and installs the Emscripten SDK; on Linux names \
the emsdk commands)\n\
       qb64rust test [<file or folder>...] [--list] [--filter <text>]... [--jobs <n>] [--timeout <seconds>] \
[--format text|json] [--toolchain-root <dir>] [--qb64pe-root <dir>] [--libqb-root <dir>] [--include-root <dir>] \
[--fixes on|off] [--fix <name>=on|off]... [--no-settings] [--coverage] [--call-trace|--no-call-trace]    (runs the \
test SUBs of *_test.bas files)\n\
       qb64rust init [<folder>] [--yes] [--output <folder>] [--copy <path>]... [--plan] ...    (writes the folder's \
qb64rust.toml from a few questions; qb64rust init --help lists the options)\n\
       qb64rust convert --to utf8|cp437 <file>...    (rewrites source files from code page 437 to UTF-8, or back)\n\
       qb64rust serve [<folder>|<file.bas>] [--port <n>] [--open] [--no-headers]    (serves a web folder of a \
--target wasm build on 127.0.0.1, as a web host must serve it)\n\
       qb64rust --version | --help";

/// The usage of `qb64rust serve` (spec `compiler/cli`, "Serve subcommand").
const SERVE_USAGE: &str = "usage: qb64rust serve [<folder>|<file.bas>] [--port <n>] [--open] [--no-headers]    (serves \
the web folder of a --target wasm build on 127.0.0.1 with the two headers a page needs for threads; without an \
argument build/web of the current folder; --open opens the browser; --no-headers serves as a host that cannot send \
the headers)";

/// The usage of `qb64rust toolchain` (spec `compiler/cli`, "Toolchain subcommand").
const TOOLCHAIN_USAGE: &str = "usage: qb64rust toolchain install [--from-zip <zip>]    (downloads and installs the \
C++ toolchain, or installs it from the release's zip; on Linux names the packages)\n\
       qb64rust toolchain path [--toolchain-root <dir>] [--qb64pe-root <dir>]    (the toolchain a build would use)\n\
       qb64rust toolchain path --target wasm [--emsdk-root <dir>]    (the Emscripten SDK a wasm build would use)\n\
       qb64rust toolchain install --target wasm    (downloads and installs the Emscripten SDK; on Linux names \
the emsdk commands)";

#[derive(Default)]
struct Options {
    /// One file to compile; with `--report`, any number of files and folders.
    inputs: Vec<PathBuf>,
    /// `--report unsupported` or `--report warnings`: the report asked for.
    report: Option<String>,
    /// `-y`: lay the file out instead of compiling it (spec `compiler/format`).
    format: bool,
    output: Option<PathBuf>,
    quiet: bool,
    cpp_only: bool,
    /// `--print-build-inputs`: `-z`, then the program's half of the build's key (design D1 of `testing-reset`).
    print_build_inputs: bool,
    dump: Option<String>,
    /// `--qb64pe-root`.
    root: Option<PathBuf>,
    /// `--toolchain-root` (spec `compiler/toolchain`).
    toolchain_root: Option<PathBuf>,
    /// `--libqb-root`: the copy of libqb a build uses (design D3 of `libqb-copy`).
    libqb_root: Option<PathBuf>,
    /// `--include-root`: where included files are looked up after the including file's folder.
    include_root: Option<PathBuf>,
    keep_build: bool,
    /// `--coverage`: a counter at each statement, written at exit to the file `QB64RUST_COVERAGE` names (design D1
    /// of `cws-whole-game`). The fragments differ, so the build cache's key does too.
    coverage: bool,
    /// `--trace-values`: a record call at each store into a variable; at exit the values are written to the file
    /// `QB64RUST_TRACE` names (OpenSpec change `value-trace`). The fragments differ, so the build cache's key does
    /// too.
    trace: bool,
    /// `--debug`: a build for a debugger (design D1 of `m5-debug-build-symbols`): `-g`, unstripped, never optimised,
    /// and the debug symbol file beside the executable.
    debug: bool,
    /// `--call-trace` (`Some(true)`) or `--no-call-trace` (`Some(false)`), the last one given: a call stack in a
    /// runtime error's message (OpenSpec change `call-trace`). Over `[build]` `call_trace` and `--debug`.
    call_trace: Option<bool>,
    /// `-w`: print the warnings too (spec `compiler/cli`).
    warnings: bool,
    /// `-e`: `OPTION _EXPLICIT` forced, as `qb64pe -e` does (spec `compiler/cli`).
    explicit: bool,
    /// `-p`: the libqb objects removed first, as `qb64pe -p` does (spec `compiler/cli`).
    purge: bool,
    /// `-f:<setting>=<value>` as given, checked by [`compile::build_settings`].
    settings: Vec<String>,
    /// `--fixes` and `--fix`, in the order given (spec `compiler/fixes`).
    fixes: Vec<CliFix>,
    /// `--lints` and `--lint`, in the order given (spec `compiler/lints`).
    lints: Vec<CliLint>,
    /// `--no-settings`: no `qb64rust.toml` is read (spec `compiler/cli`).
    no_settings: bool,
    /// `--format-default <setting>=<value>` as given: layout settings below the settings file's (spec
    /// `compiler/format`).
    format_defaults: Vec<String>,
    /// `--program <file.bas>`: with `-y`, the program the input file is formatted as a file of (spec
    /// `compiler/format`, "An include file inside its program").
    program: Option<PathBuf>,
    /// `--target wasm`: the platform a build, `-z` and a dump are made for; `None` (also `--target host`) for the
    /// host (spec `compiler/cli`, "The target option").
    target: Option<Platform>,
    /// `--emsdk-root`: the Emscripten SDK of a wasm build (spec `compiler/toolchain`, "The Emscripten SDK").
    emsdk_root: Option<PathBuf>,
    /// `--print-paths`: the program's path and the folder it runs in, as JSON, and nothing built (spec
    /// `compiler/cli`, "The paths of a program").
    print_paths: bool,
}

/// The value of `--target` (spec `compiler/cli`, "The target option"): `host`, the default, or `wasm`.
fn parse_target(name: &str) -> Result<Option<Platform>, String> {
    match name {
        "host" => Ok(None),
        "wasm" => Ok(Some(Platform::Wasm)),
        _ => Err(format!(
            "unknown --target `{name}`: the targets are `host` (the default) and `wasm` (experimental)"
        )),
    }
}

/// The arguments without a `--target <name>` pair, and the target it named, for `dap` and `test`, which build for
/// the host only: `--target wasm` is refused with `refused`, the message that names what the target does not have
/// (spec `compiler/cli`, "Options that a wasm build does not take").
fn without_target(args: &[std::ffi::OsString], refused: &str) -> Result<Vec<std::ffi::OsString>, String> {
    let mut rest = Vec::new();
    let mut args = args.iter();
    while let Some(a) = args.next() {
        if a == "--target" {
            let name = args.next().ok_or("--target needs a value")?;
            if parse_target(&name.to_string_lossy())?.is_some() {
                return Err(format!("--target wasm: {refused} not available on the wasm target"));
            }
        } else {
            rest.push(a.clone());
        }
    }
    Ok(rest)
}

/// The options a wasm build does not take, each named with what it builds (spec `compiler/cli`, "Options that a
/// wasm build does not take").
fn check_wasm_options(o: &Options) -> Result<(), String> {
    if o.target != Some(Platform::Wasm) {
        return Ok(());
    }
    for (given, option, what) in [
        (o.debug, "--debug", "debug builds are"),
        (o.coverage, "--coverage", "coverage builds are"),
        (o.trace, "--trace-values", "value-trace builds are"),
    ] {
        if given {
            return Err(format!("{option}: {what} not available on the wasm target"));
        }
    }
    if let Some(out) = o.output.as_ref().filter(|_| !o.format)
        && !out.extension().is_some_and(|e| e.eq_ignore_ascii_case("js"))
    {
        return Err(format!(
            "-o {}: the output of a wasm build ends in `.js` (its `.wasm` file is written beside it)",
            out.display()
        ));
    }
    Ok(())
}

/// The layout settings of `-y` (spec `compiler/format`, "Order of layout settings"; design D2 of
/// `project-settings`): QB64pe's defaults, each `--format-default`, the settings file's `[format]` table `file`,
/// then each `-f:` layout setting. A build setting given with `-f:` is checked as a build checks it and changes
/// nothing, as with `qb64pe`.
fn layout_settings(settings: &[String], defaults: &[String], file: &Overrides) -> Result<Settings, String> {
    let mut layout = Settings::default();
    for s in defaults {
        let (name, value) = s.split_once('=').unwrap_or((s, ""));
        if !layout.set(name, value)? {
            return Err(format!(
                "--format-default takes a layout setting, and `{name}` is not one"
            ));
        }
    }
    layout.apply(file);
    let mut rest = Vec::new();
    for s in settings {
        let (name, value) = s.split_once('=').unwrap_or((s, ""));
        if !layout.set(name, value)? {
            rest.push(s.clone());
        }
    }
    compile::build_settings(&rest)?;
    Ok(layout)
}

fn parse_args() -> Result<Options, String> {
    let mut o = Options::default();
    let mut args = std::env::args_os().skip(1);
    while let Some(a) = args.next() {
        let s = a.to_string_lossy().to_string();
        let mut value = |what: &str| args.next().ok_or_else(|| format!("{what} needs a value"));
        match s.as_str() {
            // Accepted for the corpus runner and the M1 extension: -x (compile only, no IDE) and -m (plain
            // output) change nothing here. -c is qb64pe's compile in its own window; with no IDE it is -x.
            "-x" | "-c" | "-m" => {}
            // qb64pe's -s shows or changes a stored setting of the IDE (-s, -s:<setting>[=<value>]). There is no
            // store here: a setting is given for each build.
            _ if s == "-s" || s.starts_with("-s:") => {
                return Err(format!(
                    "`{s}` is not supported: there are no stored settings; give a setting for this build with \
                     `-f:<setting>=<value>`"
                ));
            }
            "-w" => o.warnings = true,
            "-e" => o.explicit = true,
            "-p" => o.purge = true,
            "-q" => o.quiet = true,
            "-z" => o.cpp_only = true,
            "--print-build-inputs" => {
                o.cpp_only = true;
                o.print_build_inputs = true;
            }
            "-y" => o.format = true,
            "-o" => o.output = Some(PathBuf::from(value("-o")?)),
            "--dump" => {
                let d = value("--dump")?.to_string_lossy().to_string();
                if !["tokens", "tree", "typed", "ir", "cpp", "debug-info"].contains(&d.as_str()) {
                    return Err(format!("unknown --dump stage `{d}`"));
                }
                o.dump = Some(d);
            }
            "--qb64pe-root" => o.root = Some(PathBuf::from(value("--qb64pe-root")?)),
            "--toolchain-root" => o.toolchain_root = Some(PathBuf::from(value("--toolchain-root")?)),
            "--libqb-root" => o.libqb_root = Some(PathBuf::from(value("--libqb-root")?)),
            "--include-root" => o.include_root = Some(PathBuf::from(value("--include-root")?)),
            "--keep-build" => o.keep_build = true,
            "--coverage" => o.coverage = true,
            "--trace-values" => o.trace = true,
            "--debug" => o.debug = true,
            "--call-trace" => o.call_trace = Some(true),
            "--no-call-trace" => o.call_trace = Some(false),
            "--fixes" => {
                let v = value("--fixes")?.to_string_lossy().to_string();
                o.fixes.push(fixes::parse_fixes_option(&v)?);
            }
            "--fix" => {
                let v = value("--fix")?.to_string_lossy().to_string();
                o.fixes.push(fixes::parse_fix_option(&v)?);
            }
            "--lints" => {
                let v = value("--lints")?.to_string_lossy().to_string();
                o.lints.push(lints::parse_lints_option(&v)?);
            }
            "--lint" => {
                let v = value("--lint")?.to_string_lossy().to_string();
                o.lints.push(lints::parse_lint_option(&v)?);
            }
            "--no-settings" => o.no_settings = true,
            "--format-default" => {
                let v = value("--format-default")?.to_string_lossy().to_string();
                o.format_defaults.push(v);
            }
            "--program" => o.program = Some(PathBuf::from(value("--program")?)),
            "--target" => o.target = parse_target(&value("--target")?.to_string_lossy())?,
            "--emsdk-root" => o.emsdk_root = Some(PathBuf::from(value("--emsdk-root")?)),
            "--print-paths" => o.print_paths = true,
            "--report" => {
                let r = value("--report")?.to_string_lossy().to_string();
                if r != "unsupported" && r != "warnings" {
                    return Err(format!("unknown --report `{r}`"));
                }
                o.report = Some(r);
            }
            _ if s.starts_with("-f:") => o.settings.push(s["-f:".len()..].to_string()),
            _ if s.starts_with('-') => return Err(format!("unknown option `{s}`")),
            _ => o.inputs.push(PathBuf::from(a)),
        }
    }
    if o.inputs.len() > 1 && o.report.is_none() {
        return Err("more than one input file".into());
    }
    if !o.format_defaults.is_empty() && !o.format {
        return Err("--format-default is for -y only".into());
    }
    if o.program.is_some() && !o.format {
        return Err("--program is for -y only".into());
    }
    // Only the options that change the paths go with `--print-paths`; any other would be ignored without a word.
    if o.print_paths
        && (o.output.is_some()
            || o.format
            || o.report.is_some()
            || o.cpp_only
            || o.dump.is_some()
            || o.debug
            || o.coverage
            || o.trace
            || o.call_trace.is_some()
            || o.purge)
    {
        return Err(
            "--print-paths prints the paths of a build without -o; it takes only --target and \
                    --no-settings, not -o, -y, -z, -p, --dump, --report, --debug, --coverage, --trace-values or \
                    --call-trace"
                .into(),
        );
    }
    check_wasm_options(&o)?;
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
    // `qb64rust dap`: the same rule; `dap.bas` is a file to compile.
    if args.first().is_some_and(|a| a == "dap") {
        return dap(&args[1..]);
    }
    // `qb64rust toolchain`: the same rule; `toolchain.bas` is a file to compile.
    if args.first().is_some_and(|a| a == "toolchain") {
        return toolchain_command(&args[1..]);
    }
    // `qb64rust convert`: the same rule; `convert.bas` is a file to compile.
    if args.first().is_some_and(|a| a == "convert") {
        return convert_command(&args[1..]);
    }
    // `qb64rust serve`: the same rule; `serve.bas` is a file to compile.
    if args.first().is_some_and(|a| a == "serve") {
        return serve_command(&args[1..]);
    }
    // `qb64rust init`: the same rule; `init.bas` is a file to compile.
    if args.first().is_some_and(|a| a == "init") {
        let rest: Vec<String> = args[1..].iter().map(|a| a.to_string_lossy().into_owned()).collect();
        return qb64rust_driver::init::command(&rest);
    }
    // `qb64rust test`: the same rule; `test.bas` is a file to compile.
    if args.first().is_some_and(|a| a == "test") {
        return match qb64rust_driver::with_stack(|| test_command(std::env::args_os().skip(2).collect())) {
            Ok(code) => code,
            Err(msg) => {
                println!("qb64rust: {msg}\n{USAGE}");
                ExitCode::FAILURE
            }
        };
    }
    // `--version`, `--help`, `-h`: the only argument (spec `compiler/cli`, "Version and help"; design D4 of
    // `m6-compiler-download`).
    if let Some(word) = args
        .first()
        .and_then(|a| a.to_str())
        .filter(|a| matches!(*a, "--version" | "--help" | "-h"))
    {
        if args.len() > 1 {
            eprintln!("qb64rust: `{word}` takes no other argument\n{USAGE}");
            return ExitCode::FAILURE;
        }
        if word == "--version" {
            println!("qb64rust {}", env!("CARGO_PKG_VERSION"));
        } else {
            println!("{USAGE}");
        }
        return ExitCode::SUCCESS;
    }
    // `--list-fixes`: the only argument, as `--version` (spec `compiler/fixes`, "Listing the fixes").
    if args.first().is_some_and(|a| a == "--list-fixes") {
        if args.len() > 1 {
            eprintln!("qb64rust: `--list-fixes` takes no other argument\n{USAGE}");
            return ExitCode::FAILURE;
        }
        let width = FIXES.iter().map(|f| f.1.len()).max().unwrap_or(0);
        for (_, name, description) in FIXES {
            println!("{name:width$}  {description}");
        }
        return ExitCode::SUCCESS;
    }
    // `--old-wording-words`, `--old-wording-keys`: the only argument, for the old-wording table's script.
    if let Some(word) = args
        .first()
        .and_then(|a| a.to_str())
        .filter(|a| matches!(*a, "--old-wording-words" | "--old-wording-keys"))
    {
        if args.len() > 1 {
            eprintln!("qb64rust: `{word}` takes no other argument");
            return ExitCode::FAILURE;
        }
        return if word == "--old-wording-words" {
            old_wording_words()
        } else {
            old_wording_keys()
        };
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

/// The words a backtick span of a message may hold and stay in its key: the parser's reserved words and the
/// built-in table's names, in capitals, sorted. `old_wording_words.txt` is this list, written by the script.
fn old_wording_word_set() -> std::collections::BTreeSet<String> {
    let mut words: std::collections::BTreeSet<String> =
        qb64rust_syntax::keywords().iter().map(|w| (*w).to_string()).collect();
    words.extend(qb64rust_builtins::BUILTINS.iter().map(|b| b.name.to_ascii_uppercase()));
    words
}

/// `--old-wording-words`: the word list, one word per line.
fn old_wording_words() -> ExitCode {
    let mut out = std::io::stdout().lock();
    for word in old_wording_word_set() {
        if writeln!(out, "{word}").is_err() {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

/// `--old-wording-keys`: for each line of stdin (a message of this compiler), one JSON line on stdout:
/// `{"key": <its key>, "values": [<per placeholder, [code, text] for each form of OLD_WORDING_CODES, in order>]}`.
/// The key is made with the word list of [`old_wording_word_set`], the list the script writes for the compiler.
#[expect(
    clippy::disallowed_methods,
    reason = "the compiler's own messages, UTF-8, not BASIC source"
)]
fn old_wording_keys() -> ExitCode {
    use std::io::BufRead as _;
    let words = old_wording_word_set();
    let is_word = |w: &str| words.contains(w);
    let mut out = std::io::stdout().lock();
    for line in std::io::stdin().lock().lines() {
        let Ok(message) = line else {
            return ExitCode::FAILURE;
        };
        let (key, values) = qb64rust_base::old_wording_key(message.trim_end_matches('\r'), &is_word);
        let values: Vec<Vec<(&str, String)>> = values
            .iter()
            .map(|v| {
                qb64rust_base::OLD_WORDING_CODES
                    .into_iter()
                    .zip(qb64rust_base::old_wording_forms(v))
                    .collect()
            })
            .collect();
        let json = serde_json::json!({ "key": key, "values": values });
        if writeln!(out, "{json}").is_err() {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
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
    // A check and a formatting request run the CLI's own front end (design D2 of `extension-diagnostics`, D5 of
    // `m5b-formatter`).
    let hooks = qb64rust_driver::server_hooks();
    match qb64rust_driver::with_stack(move || qb64rust_lsp::run_stdio(root, hooks)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("qb64rust lsp: {msg}");
            ExitCode::FAILURE
        }
    }
}

/// `--toolchain-root <dir>` and `--qb64pe-root <dir>`, each at most once, and nothing else: the options of `dap` and
/// `toolchain path`, which choose the toolchain as a build does. `None` for any other argument.
fn toolchain_options(args: &[std::ffi::OsString]) -> Option<(Option<PathBuf>, Option<PathBuf>)> {
    let (mut toolchain, mut qb64pe) = (None, None);
    let mut args = args.iter();
    while let Some(a) = args.next() {
        let slot = match a.to_str() {
            Some("--toolchain-root") => &mut toolchain,
            Some("--qb64pe-root") => &mut qb64pe,
            _ => return None,
        };
        if slot.is_some() {
            return None;
        }
        *slot = Some(PathBuf::from(args.next()?));
    }
    Some((toolchain, qb64pe))
}

/// The usage of `qb64rust convert` (OpenSpec change `utf8-source`, design D9).
const CONVERT_USAGE: &str = "usage: qb64rust convert --to utf8|cp437 <file>...    (rewrites source files from code \
page 437 to UTF-8, or back, in place; changes no file when one cannot be converted)";

/// `qb64rust serve [<folder>|<file.bas>] [--port <n>] [--open] [--no-headers]` (spec `compiler/web-serve`): the web
/// folder named, or the one a wasm build of the program writes, or `build/web` of the current folder, served on
/// `127.0.0.1` until the process is interrupted.
fn serve_command(args: &[std::ffi::OsString]) -> ExitCode {
    use qb64rust_driver::{build, serve};
    let fail = |msg: &str| {
        println!("qb64rust: {msg}");
        ExitCode::FAILURE
    };
    let usage = |msg: &str| fail(&format!("{msg}\n{SERVE_USAGE}"));
    let (mut named, mut port, mut open, mut isolation) = (None, None, false, true);
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].to_string_lossy();
        match arg.as_ref() {
            "--port" => {
                let Some(n) = args
                    .get(i + 1)
                    .and_then(|n| n.to_str())
                    .and_then(|n| n.parse::<u16>().ok())
                    .filter(|n| *n > 0)
                else {
                    return usage("--port needs a port number from 1 to 65535");
                };
                port = Some(n);
                i += 1;
            }
            "--open" => open = true,
            "--no-headers" => isolation = false,
            _ if arg.starts_with('-') => return usage(&format!("serve does not know the option `{arg}`")),
            _ if named.is_some() => return usage("serve takes one folder or one program"),
            _ => named = Some(PathBuf::from(&args[i])),
        }
        i += 1;
    }
    // A program: the web folder its build writes, read from its settings file as a build reads it.
    let is_source = |p: &Path| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bas"));
    let (folder, build_command) = match &named {
        Some(source) if is_source(source) => {
            if !source.is_file() {
                return fail(&format!("cannot read {}", source.display()));
            }
            let source_folder = std::path::absolute(source).unwrap_or_else(|_| source.clone());
            let source_folder = source_folder.parent().unwrap_or(Path::new(""));
            let output = match settings::load(source, false) {
                Ok((_, build, _)) => build.output_for(source_folder, Platform::Wasm),
                Err(e) => return fail(&e.to_string()),
            };
            (
                build::web_folder(&output.join("x.js")),
                format!("qb64rust -x --target wasm {}", source.display()),
            )
        }
        Some(folder) => (folder.clone(), "qb64rust -x --target wasm <file.bas>".to_string()),
        None => (
            PathBuf::from("build").join("web"),
            "qb64rust -x --target wasm <file.bas>".to_string(),
        ),
    };
    let folder = std::path::absolute(&folder).unwrap_or(folder);
    if !folder.join(build::WASM_INDEX).is_file() {
        let what = if folder.is_dir() {
            format!("{} has no index.html", folder.display())
        } else {
            format!("{} is not a folder", folder.display())
        };
        return fail(&format!(
            "{what}: a window program built with `{build_command}` writes its web folder"
        ));
    }
    let (listener, used) = match serve::bind(port) {
        Ok(bound) => bound,
        Err(msg) => return fail(&msg),
    };
    let url = serve::address(used);
    // The address first, alone on its line: the editor reads it from there.
    println!("{url}");
    println!("serving {} until interrupted (Ctrl+C)", folder.display());
    if port.is_none() && used != serve::DEFAULT_PORT {
        println!(
            "port {} is taken. A browser keeps a page's saved games for its address, so games saved at another \
             port are not found at this one",
            serve::DEFAULT_PORT
        );
    }
    if !isolation {
        println!("without the two headers of threads, as a host that cannot send them");
    }
    let _ = std::io::stdout().flush();
    if open && let Err(msg) = serve::open_browser(&url) {
        println!("qb64rust: {msg}");
    }
    serve::serve(&listener, &folder, isolation);
    ExitCode::SUCCESS
}

/// `qb64rust convert --to utf8|cp437 <file>...` (spec `compiler/source-encoding`, "The convert command").
fn convert_command(args: &[std::ffi::OsString]) -> ExitCode {
    use qb64rust_driver::convert::{To, run};
    let fail = |msg: &str| {
        println!("qb64rust: {msg}\n{CONVERT_USAGE}");
        ExitCode::FAILURE
    };
    let mut to = None;
    let mut files = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--to" {
            let Some(name) = args.get(i + 1) else {
                return fail("--to needs utf8 or cp437");
            };
            let Some(t) = To::from_name(&name.to_string_lossy()) else {
                return fail(&format!("--to takes utf8 or cp437, not `{}`", name.to_string_lossy()));
            };
            to = Some(t);
            i += 2;
        } else if args[i].to_string_lossy().starts_with('-') {
            // An option it does not know, as `qb64rust toolchain` refuses one (`toolchain_options`).
            return fail(&format!(
                "convert does not know the option `{}`",
                args[i].to_string_lossy()
            ));
        } else {
            files.push(PathBuf::from(&args[i]));
            i += 1;
        }
    }
    let Some(to) = to else {
        return fail("convert needs --to utf8 or --to cp437");
    };
    if files.is_empty() {
        return fail("convert needs at least one file");
    }
    match run(&files, to) {
        Ok(done) => {
            for f in &done.written {
                println!("converted {}", f.display());
            }
            for f in &done.left {
                println!("left {} as it is: it holds no byte above 127", f.display());
            }
            ExitCode::SUCCESS
        }
        Err(failed) => {
            for p in failed.problems {
                println!("{p}");
            }
            if failed.written.is_empty() {
                println!("no file was changed");
            } else {
                for f in &failed.written {
                    println!("converted {} before the failure", f.display());
                }
            }
            ExitCode::FAILURE
        }
    }
}

/// `qb64rust toolchain install` and `qb64rust toolchain path` (spec `compiler/toolchain`). `path` prints the folder,
/// then where it came from (a word, a colon, what it means); with none found, the message of "No toolchain found".
fn toolchain_command(args: &[std::ffi::OsString]) -> ExitCode {
    let fail = |msg: &str| {
        println!("qb64rust: {msg}");
        ExitCode::FAILURE
    };
    // `--target wasm` (spec `compiler/toolchain`, "The wasm path of the toolchain subcommand"); `--target host` is
    // the subcommand without it.
    let mut args = args.to_vec();
    if let Some(at) = args.iter().position(|a| a == "--target") {
        let Some(name) = args.get(at + 1) else {
            return fail(TOOLCHAIN_USAGE);
        };
        let target = match parse_target(&name.to_string_lossy()) {
            Ok(target) => target,
            Err(msg) => return fail(&msg),
        };
        args.drain(at..at + 2);
        if target == Some(Platform::Wasm) {
            return wasm_toolchain_command(&args);
        }
    }
    let args = &args[..];
    match args.first().and_then(|a| a.to_str()) {
        Some("install") if args.len() == 1 && Platform::host() == Platform::Linux => match toolchain::check_system() {
            Ok(line) => {
                println!("{line}");
                ExitCode::SUCCESS
            }
            Err(msg) => fail(&msg),
        },
        Some("install") if args.len() == 1 => match toolchain::install() {
            Ok(_) => ExitCode::SUCCESS,
            Err(msg) => fail(&msg),
        },
        Some("install") if args.len() == 3 && args[1] == "--from-zip" => {
            if Platform::host() == Platform::Linux {
                return fail("--from-zip is for Windows: on Linux the system toolchain is used, nothing is installed");
            }
            match toolchain::install_zip(Path::new(&args[2])) {
                Ok(_) => ExitCode::SUCCESS,
                Err(msg) => fail(&msg),
            }
        }
        Some("path") => {
            let Some((root, qb64pe)) = toolchain_options(&args[1..]) else {
                return fail(TOOLCHAIN_USAGE);
            };
            match toolchain::find(Platform::host(), root.as_deref(), qb64pe.as_deref()) {
                Ok(found) => {
                    println!("{}", found.folder.display());
                    println!("{}", found.source.describe());
                    ExitCode::SUCCESS
                }
                Err(msg) => fail(&msg),
            }
        }
        _ => fail(TOOLCHAIN_USAGE),
    }
}

/// `qb64rust toolchain path --target wasm [--emsdk-root <dir>]`: the Emscripten SDK folder and where it came from;
/// `qb64rust toolchain install --target wasm`: the SDK downloaded and installed on Windows (`toolchain::install_emsdk`).
fn wasm_toolchain_command(args: &[std::ffi::OsString]) -> ExitCode {
    let fail = |msg: &str| {
        println!("qb64rust: {msg}");
        ExitCode::FAILURE
    };
    let words: Vec<&str> = args.iter().map(|a| a.to_str().unwrap_or("")).collect();
    let option = match words.as_slice() {
        ["install"] => {
            return match toolchain::install_emsdk() {
                Ok(_) => ExitCode::SUCCESS,
                Err(msg) => fail(&msg),
            };
        }
        ["path"] => None,
        ["path", "--emsdk-root", dir] => Some(PathBuf::from(dir)),
        _ => return fail(TOOLCHAIN_USAGE),
    };
    match toolchain::find_emsdk(option.as_deref()) {
        Ok(sdk) => {
            println!("{}", sdk.root.display());
            println!("{}", sdk.source.describe());
            ExitCode::SUCCESS
        }
        Err(msg) => fail(&msg),
    }
}

/// The debug adapter over stdin and stdout (spec `editor/debug-adapter`, design D12 of `m5-dap-proxy`), with the
/// toolchain's `lldb-dap` as its backend. Its options are `--toolchain-root` and `--qb64pe-root`; the toolchain is
/// found as a build finds it (design D4 of `m6-toolchain`). When none is found the adapter still starts and answers
/// `initialize` with the error, so that the editor shows it. Stdout carries the protocol, as for `lsp`.
fn dap(args: &[std::ffi::OsString]) -> ExitCode {
    let args = match without_target(args, "the debugger is") {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("qb64rust: {msg}");
            return ExitCode::FAILURE;
        }
    };
    let Some((root, qb64pe)) = toolchain_options(&args) else {
        eprintln!("qb64rust: `dap` takes only --toolchain-root <dir> and --qb64pe-root <dir>\n{USAGE}");
        return ExitCode::FAILURE;
    };
    let backend = toolchain::find(Platform::host(), root.as_deref(), qb64pe.as_deref())
        .and_then(|found| toolchain::lldb_dap(&found, toolchain::os_release().as_deref()));
    match qb64rust_driver::with_stack(move || qb64rust_dap::run_stdio(backend)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("qb64rust dap: {msg}");
            ExitCode::FAILURE
        }
    }
}

/// `qb64rust test` (spec `compiler/unit-tests`; `unit_tests\`): finds the tests of the paths given, then lists them
/// (`--list`) or builds and runs them. Exit status 0 when every test passed, 1 otherwise; `Err` for a usage error.
fn test_command(args: Vec<std::ffi::OsString>) -> Result<ExitCode, String> {
    use qb64rust_driver::unit_tests::{discover, report as out, run};
    let mut o = Options::default();
    let (mut list, mut json, mut filters) = (false, false, Vec::new());
    let mut jobs = std::thread::available_parallelism().map_or(1, usize::from);
    let mut timeout = 30u64;
    let mut paths = Vec::new();
    let mut args = without_target(&args, "unit-test runs are")?.into_iter();
    while let Some(a) = args.next() {
        let s = a.to_string_lossy().to_string();
        let mut value = |what: &str| {
            args.next()
                .map(|v| v.to_string_lossy().to_string())
                .ok_or_else(|| format!("{what} needs a value"))
        };
        match s.as_str() {
            "--list" => list = true,
            "--filter" => filters.push(value("--filter")?.to_lowercase().replace('\\', "/")),
            "--jobs" => {
                jobs = value("--jobs")?
                    .parse()
                    .ok()
                    .filter(|&n| n > 0)
                    .ok_or("--jobs takes a number above 0")?;
            }
            "--timeout" => {
                timeout = value("--timeout")?
                    .parse()
                    .ok()
                    .filter(|&n| n > 0)
                    .ok_or("--timeout takes a number of seconds above 0")?;
            }
            "--format" => {
                json = match value("--format")?.as_str() {
                    "text" => false,
                    "json" => true,
                    f => return Err(format!("--format takes text or json, not `{f}`")),
                }
            }
            "--qb64pe-root" => o.root = Some(PathBuf::from(value("--qb64pe-root")?)),
            "--toolchain-root" => o.toolchain_root = Some(PathBuf::from(value("--toolchain-root")?)),
            "--libqb-root" => o.libqb_root = Some(PathBuf::from(value("--libqb-root")?)),
            "--include-root" => o.include_root = Some(PathBuf::from(value("--include-root")?)),
            "--fixes" => o.fixes.push(fixes::parse_fixes_option(&value("--fixes")?)?),
            "--fix" => o.fixes.push(fixes::parse_fix_option(&value("--fix")?)?),
            "--no-settings" => o.no_settings = true,
            "--coverage" => o.coverage = true,
            "--call-trace" => o.call_trace = Some(true),
            "--no-call-trace" => o.call_trace = Some(false),
            _ if s.starts_with('-') => return Err(format!("`test` does not take `{s}`")),
            _ => paths.push(PathBuf::from(a)),
        }
    }
    let say = |text: &str| {
        if json {
            println!(
                "{{\"event\":\"error\",\"message\":{}}}",
                qb64rust_base::json_string(text)
            );
        } else {
            println!("qb64rust test: {text}");
        }
    };
    let files = match discover::test_files(&paths) {
        Ok(f) => f,
        Err(e) => {
            say(&e);
            return Ok(ExitCode::FAILURE);
        }
    };
    // The tests of each file that the filters keep; a SUB that takes parameters is noted in a text list.
    let mut found = Vec::new();
    for path in files {
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                say(&format!("cannot read {}: {e}", path.display()));
                return Ok(ExitCode::FAILURE);
            }
        };
        let shown = path.display().to_string();
        let f = discover::tests_of(&shown, bytes);
        if list && !json {
            for (line, note) in &f.notes {
                println!("{shown}:{line}: {note}");
            }
        }
        let key = |t: &discover::TestSub| format!("{shown}:{}", t.name).to_lowercase().replace('\\', "/");
        let tests: Vec<_> = (f.tests.into_iter())
            .filter(|t| filters.is_empty() || filters.iter().any(|f| key(t).contains(f.as_str())))
            .collect();
        if !tests.is_empty() {
            found.push(run::TestFile { path, tests });
        }
    }
    if found.is_empty() {
        say("no test found");
        return Ok(ExitCode::FAILURE);
    }
    // The tests found: a list's lines, and the first JSON lines of a run; a run's text starts with its results.
    for f in found.iter().filter(|_| list || json) {
        for t in &f.tests {
            let shown = f.path.display().to_string();
            println!(
                "{}",
                if json {
                    out::test_json(&shown, t)
                } else {
                    out::test_text(&shown, t)
                }
            );
        }
    }
    if list {
        return Ok(ExitCode::SUCCESS);
    }
    let options = run::RunOptions {
        jobs,
        timeout: std::time::Duration::from_secs(timeout),
        include_root: include_root(&o)?,
        root: o.root.clone(),
        toolchain_root: o.toolchain_root.clone(),
        libqb_root: o.libqb_root.clone(),
        fixes: o.fixes.clone(),
        no_settings: o.no_settings,
        coverage: o.coverage,
        call_trace: o.call_trace,
        coverage_file: std::env::var_os(run::COVERAGE_ENV)
            .filter(|v| !v.is_empty())
            .map(|v| std::path::absolute(PathBuf::from(&v)).unwrap_or_else(|_| PathBuf::from(v))),
    };
    let mut counts = out::Counts::default();
    let mut failures = Vec::new();
    run::run_all(&found, &options, |r| {
        counts.add(r.outcome);
        println!(
            "{}",
            if json {
                out::result_json(&r)
            } else {
                out::result_text(&r)
            }
        );
        let _ = std::io::stdout().flush();
        if r.outcome != qb64rust_driver::unit_tests::judge::Outcome::Pass {
            failures.push(r);
        }
    });
    if json {
        println!("{}", out::summary_json(&counts));
    } else {
        for r in &failures {
            print!("\n{}", out::details_text(r));
        }
        println!("\n{}", out::summary_text(&counts));
    }
    Ok(if counts.all_passed() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
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

/// `--report unsupported` or `--report warnings`: the table of `report.rs` for the files and folders given, with the
/// error cap lifted so that a large program counts in full. The exit status is 0 whatever the programs hold: the
/// report is no verdict.
fn run_report(o: &Options) -> Result<ExitCode, String> {
    qb64rust_base::lift_error_cap();
    let root = include_root(o)?;
    let text = match o.report.as_deref() {
        Some("warnings") => report::warnings(&o.inputs, &root)?,
        _ => report::unsupported(&o.inputs, &root)?,
    };
    print!("{text}");
    Ok(ExitCode::SUCCESS)
}

/// The fixes, the lints and the `[format]` table of the program `input`: its settings file's, unless
/// `--no-settings`; then `--fixes` and `--fix` over the fixes (spec `compiler/fixes`), `--lints` and `--lint` over
/// the lints (spec `compiler/lints`). With `-w`, the file used is named. A settings file that cannot be used is
/// printed as `<file>:<line>: <message>`, and the caller exits with status 1. The `[build]` table comes with them.
fn program_settings(o: &Options, input: &Path) -> Result<(Fixes, Lints, Overrides, settings::Build), ExitCode> {
    match settings::load(input, o.no_settings) {
        Ok((project, build, file)) => {
            // Not with `--print-paths`, whose output is one line of JSON.
            if let Some(file) = file.filter(|_| o.warnings && !o.print_paths) {
                println!("settings from {}", file.display());
            }
            Ok((
                fixes::apply_cli(project.fixes, &o.fixes),
                lints::apply_cli(project.lints, &o.lints),
                project.format,
                build,
            ))
        }
        Err(e) => {
            println!("{e}");
            Err(ExitCode::FAILURE)
        }
    }
}

/// `-y`: the main file laid out into `-o` (spec `compiler/format`). Nothing is built, and nothing is written when
/// the parser reports an error. An error of a later stage is printed, and the file is still written (design D1 of
/// `include-file-program`). With `--program`, the program is that file, `input` is one of its files, and the
/// settings file is the program's, as in the editor.
fn format(o: &Options, input: &Path, name: &str, bytes: Vec<u8>) -> Result<ExitCode, String> {
    let Some(out) = &o.output else {
        return Err("formatting requires an output file: give -o <file>".into());
    };
    let include_root = include_root(o)?;
    let (main, name, bytes) = match &o.program {
        Some(p) => {
            let bytes = std::fs::read(p).map_err(|e| format!("cannot read {}: {e}", p.display()))?;
            (p.as_path(), p.to_string_lossy().to_string(), bytes)
        }
        None => (input, name.to_string(), bytes),
    };
    let input_name = input.display();
    let main_name = main.display();
    // A settings file that cannot be used stops `-y` before anything is written (design D5 of `project-settings`).
    let (fixes, lints, file_format, build) = match program_settings(o, main) {
        Ok(s) => s,
        Err(code) => return Ok(code),
    };
    let settings = layout_settings(&o.settings, &o.format_defaults, &file_format)?;
    // A rewrite that the other settings make do nothing is said once; the file is still written (design D6 of
    // `format-statement-lines`).
    if let Some(w) = settings.rewrite_warning() {
        eprintln!("warning: {w}");
    }
    // `-e` holds here too: `qb64pe -y -e` rejects a variable not declared (measured, 2026-11-04). Here the error is
    // printed and the file still written (D-064).
    let mut fe = frontend_explicit(
        Platform::host(),
        o.explicit,
        &name,
        bytes,
        |file| Box::new(FileLoader::new(&include_root, file, main).with_encoding(&build.root, build.encoding)),
        Some((main, &include_root)),
        fixes,
    );
    lints.drop_from(&mut fe.diagnostics);
    let errors = fe.diagnostics.error_count();
    if o.warnings {
        print!("{}", fe.render_with_warnings());
    } else if errors > 0 {
        print!("{}", fe.render_diagnostics());
    }
    if errors > 0 {
        println!("{}", fe.diagnostics.summary());
    }
    if fe.parsed.diagnostics().has_errors() {
        return Ok(ExitCode::FAILURE);
    }
    // Looked for after the parse, which loads the included files.
    let file = match o.program {
        Some(_) => qb64rust_driver::program_file(&fe, input),
        None => Some(fe.file),
    };
    let Some(file) = file else {
        return Err(format!("{input_name} is not a file of the program {main_name}"));
    };
    let complete = qb64rust_driver::format_complete(&fe.diagnostics, Some(main));
    let text = qb64rust_driver::format_program_file(&fe, file, &settings, complete);
    std::fs::write(out, text).map_err(|e| format!("writing {}: {e}", out.display()))?;
    Ok(ExitCode::SUCCESS)
}

/// The program a build of `input` with no `-o` writes, and the folder it runs in (design D1 and D2 of
/// `project-init`): the source's name with the platform's ending in [`settings::Build::output_for`], and that
/// folder, since a program runs in its own folder. Both absolute. `-x` and `--print-paths` both take them from
/// here, so that they cannot disagree.
fn default_paths(input: &Path, platform: Platform, build: &settings::Build) -> Result<(PathBuf, PathBuf), String> {
    let input = std::path::absolute(input).map_err(|e| e.to_string())?;
    let name = input
        .file_name()
        .ok_or_else(|| format!("{} is not a file", input.display()))?;
    let folder = build.output_for(input.parent().unwrap_or(Path::new("")), platform);
    let exe = folder.join(name).with_extension(platform.exe_extension());
    Ok((exe, folder))
}

/// `--print-paths` (spec `compiler/cli`, "The paths of a program"): [`default_paths`] as one line of JSON. Nothing
/// is built or copied. A settings file that cannot be used is printed as a build prints it, with status 1.
fn print_paths(o: &Options, input: &Path) -> Result<ExitCode, String> {
    let build = match program_settings(o, input) {
        Ok((.., build)) => build,
        Err(code) => return Ok(code),
    };
    let platform = o.target.unwrap_or_else(Platform::host);
    let (exe, run_in) = default_paths(input, platform, &build)?;
    let mut json = serde_json::json!({
        "exe": exe.to_string_lossy(),
        "run_in": run_in.to_string_lossy(),
    });
    // The page a window program's build would write; `exe` is then a `$CONSOLE:ONLY` program's script.
    if platform == Platform::Wasm {
        let page = qb64rust_driver::build::web_folder(&exe).join(qb64rust_driver::build::WASM_INDEX);
        json["page"] = page.to_string_lossy().into();
    }
    println!("{json}");
    Ok(ExitCode::SUCCESS)
}

fn run() -> Result<ExitCode, String> {
    let o = parse_args().map_err(|e| format!("{e}\n{USAGE}"))?;
    if o.purge {
        // As `qb64pe -p`, which runs libqb's `make clean` and then goes on with the rest of its command line; alone it
        // opens the IDE there, and ends here.
        let libqb = qb64rust_driver::build::find_libqb(o.libqb_root.as_deref())?;
        qb64rust_driver::build::purge_objects(&libqb)?;
        if !o.quiet {
            println!("removed the built libqb objects of {}", libqb.display());
        }
        if o.inputs.is_empty() {
            return Ok(ExitCode::SUCCESS);
        }
    }
    let input = o
        .inputs
        .first()
        .cloned()
        .ok_or_else(|| format!("no input file\n{USAGE}"))?;
    if o.report.is_some() {
        return run_report(&o);
    }
    if o.print_paths {
        return print_paths(&o, &input);
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
    if o.format {
        return format(&o, &input, &name, bytes);
    }
    let platform = o.target.unwrap_or_else(Platform::host);
    PANIC_CONTEXT.lock().unwrap_or_else(|e| e.into_inner()).input = Some(name.clone());
    if o.dump.as_deref() == Some("tokens") {
        print!("{}", qb64rust_syntax::dump_tokens(&bytes));
        return Ok(ExitCode::SUCCESS);
    }
    let include_root = include_root(&o)?;
    let builds = o.dump.is_none() && !o.cpp_only;
    // The settings first, since they say where the program goes. A settings file that cannot be used removes only
    // a program named by `-o`: without it the output folder is not known (design D2 of `build-output-folder`).
    let (fixes, lints, _, build) = match program_settings(&o, &input) {
        Ok(s) => s,
        Err(code) => {
            if let Some(out) = o.output.as_ref().filter(|_| builds) {
                compile::remove_old(&std::path::absolute(out).map_err(|e| e.to_string())?, platform);
            }
            return Ok(code);
        }
    };
    let exe = match &o.output {
        Some(p) => std::path::absolute(p).map_err(|e| e.to_string())?,
        None => default_paths(&input, platform, &build)?.0,
    };
    // On Linux the program of `maps.bas` is `maps`, which may be a data folder beside it (design, "Risks", of
    // `project-init`). Said before the compile, so that the message does not depend on the linker's.
    if builds && exe.is_dir() {
        println!(
            "{} is a folder, so the program cannot be written there: name another file with -o, or an output \
             folder with `output` in the `[build]` table of qb64rust.toml",
            exe.display()
        );
        return Ok(ExitCode::FAILURE);
    }
    PANIC_CONTEXT.lock().unwrap_or_else(|e| e.into_inner()).exe = builds.then(|| exe.clone());
    // For the CLI test of the panic hook only.
    if std::env::var_os("QB64RUST_TEST_PANIC").is_some_and(|v| v == "1") {
        panic!("QB64RUST_TEST_PANIC=1 is set");
    }
    // The data files of `[build]` `copy`, before the compile (design D3 of `build-output-folder`); a build with `-o`
    // copies nothing. A failure stops the command before anything is compiled.
    if o.output.is_none()
        && builds
        && let Err(e) = output::prepare(&build, exe.parent().unwrap_or(Path::new("")))
    {
        compile::remove_old(&exe, platform);
        println!("{e}");
        return Ok(ExitCode::FAILURE);
    }
    let options = compile::CompileOptions {
        exe,
        quiet: o.quiet,
        cpp_only: o.cpp_only,
        print_build_inputs: o.print_build_inputs,
        dump: o.dump.clone(),
        root: o.root.clone(),
        toolchain_root: o.toolchain_root.clone(),
        libqb_root: o.libqb_root.clone(),
        include_root: include_root.clone(),
        keep_build: o.keep_build,
        coverage: o.coverage,
        trace: o.trace,
        call_trace: build.call_trace(o.call_trace, o.debug),
        debug: o.debug,
        warnings: o.warnings,
        explicit: o.explicit,
        settings: o.settings.clone(),
        fixes,
        lints,
        target: o.target,
        emsdk_root: o.emsdk_root.clone(),
        // What `[build]` `copy` copied, which a page packs (design D11 of `wasm-window`); a build with `-o` copies
        // nothing.
        page_files: if o.output.is_none() {
            output::copied_files(&build)
        } else {
            Vec::new()
        },
        web: o.output.is_none(),
    };
    let built = compile::compile_exe(
        &input,
        &name,
        bytes,
        |file| Box::new(FileLoader::new(&include_root, file, &input).with_encoding(&build.root, build.encoding)),
        &options,
        &mut |text| print!("{text}"),
    )?;
    Ok(if built { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}
