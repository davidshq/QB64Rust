//! `qb64rust`: the command line (design D9, spec `compiler/cli`).
//!
//! `qb64rust [-x] [-q] [-m] [-w] [-z|--print-build-inputs] [-f:<setting>=<value>]... <file.bas> [-o <exe>]
//! [--dump tokens|tree|typed|ir|cpp|debug-info]
//! [--toolchain-root <dir>] [--qb64pe-root <dir>] [--libqb-root <dir>] [--include-root <dir>] [--keep-build]
//! [--coverage] [--trace-values] [--debug] [--fixes on|off] [--fix <name>=on|off]...`
//!
//! `qb64rust --list-fixes`: the fixes, one per line (spec `compiler/fixes`, `fixes.rs`).
//!
//! `qb64rust -y [-q] [-m] [-w] [-f:<layout setting>=<value>]... <file.bas> -o <out.bas> [--include-root <dir>]`: the
//! file laid out as `qb64pe -y` lays it out (spec `compiler/format`).
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
use qb64rust_base::layout::Settings;
use qb64rust_driver::fixes::CliFix;
use qb64rust_driver::{FileLoader, compile, fixes, frontend_explicit, report, toolchain};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;

const USAGE: &str = "usage: qb64rust [-x|-c] [-q] [-m] [-w] [-e] [-p] [-z|--print-build-inputs] [-f:<setting>=<value>]... <file.bas> [-o <exe>] \
[--dump tokens|tree|typed|ir|cpp|debug-info] [--toolchain-root <dir>] [--qb64pe-root <dir>] [--libqb-root <dir>] \
[--include-root <dir>] [--keep-build] [--coverage] [--trace-values] [--debug] [--fixes on|off] \
[--fix <name>=on|off]...\n\
       qb64rust -y [-q] [-m] [-w] [-f:<layout setting>=<value>]... <file.bas> -o <out.bas> [--include-root <dir>]\n\
       qb64rust --list-fixes    (the fixes --fix and qb64rust.toml name)\n\
       qb64rust --report unsupported|warnings <file or folder>... [--include-root <dir>]\n\
       qb64rust lsp    (the language server, over stdin and stdout)\n\
       qb64rust dap [--toolchain-root <dir>] [--qb64pe-root <dir>]    (the debug adapter, over stdin and stdout)\n\
       qb64rust toolchain install [--from-zip <zip>]    (downloads and installs the C++ toolchain, or installs \
it from the release's zip; on Linux names the packages)\n\
       qb64rust toolchain path [--toolchain-root <dir>] [--qb64pe-root <dir>]    (the toolchain a build would use)\n\
       qb64rust test [<file or folder>...] [--list] [--filter <text>]... [--jobs <n>] [--timeout <seconds>] \
[--format text|json] [--toolchain-root <dir>] [--qb64pe-root <dir>] [--libqb-root <dir>] [--include-root <dir>] \
[--fixes on|off] [--fix <name>=on|off]... [--coverage]    (runs the test SUBs of *_test.bas files)\n\
       qb64rust --version | --help";

/// The usage of `qb64rust toolchain` (spec `compiler/cli`, "Toolchain subcommand").
const TOOLCHAIN_USAGE: &str = "usage: qb64rust toolchain install [--from-zip <zip>]    (downloads and installs the \
C++ toolchain, or installs it from the release's zip; on Linux names the packages)\n\
       qb64rust toolchain path [--toolchain-root <dir>] [--qb64pe-root <dir>]    (the toolchain a build would use)";

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
}

/// The layout settings of `-y` (spec `compiler/format`); a build setting is checked as a build checks it and
/// changes nothing, as with `qb64pe`.
fn layout_settings(settings: &[String]) -> Result<Settings, String> {
    let mut layout = Settings::default();
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
            "--fixes" => {
                let v = value("--fixes")?.to_string_lossy().to_string();
                o.fixes.push(fixes::parse_fixes_option(&v)?);
            }
            "--fix" => {
                let v = value("--fix")?.to_string_lossy().to_string();
                o.fixes.push(fixes::parse_fix_option(&v)?);
            }
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

/// `qb64rust toolchain install` and `qb64rust toolchain path` (spec `compiler/toolchain`). `path` prints the folder,
/// then where it came from (a word, a colon, what it means); with none found, the message of "No toolchain found".
fn toolchain_command(args: &[std::ffi::OsString]) -> ExitCode {
    let fail = |msg: &str| {
        println!("qb64rust: {msg}");
        ExitCode::FAILURE
    };
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

/// The debug adapter over stdin and stdout (spec `editor/debug-adapter`, design D12 of `m5-dap-proxy`), with the
/// toolchain's `lldb-dap` as its backend. Its options are `--toolchain-root` and `--qb64pe-root`; the toolchain is
/// found as a build finds it (design D4 of `m6-toolchain`). When none is found the adapter still starts and answers
/// `initialize` with the error, so that the editor shows it. Stdout carries the protocol, as for `lsp`.
fn dap(args: &[std::ffi::OsString]) -> ExitCode {
    let Some((root, qb64pe)) = toolchain_options(args) else {
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
    let mut args = args.into_iter();
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
            "--coverage" => o.coverage = true,
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
        coverage: o.coverage,
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

/// The fixes of the program `input` (spec `compiler/fixes`): `--fixes` replaces the settings file's, so the file is
/// read only without it; then each `--fix`. With `-w`, the file used is named. A settings file that cannot be used is
/// printed as `<file>:<line>: <message>`, and the caller exits with status 1.
fn program_fixes(o: &Options, input: &Path) -> Result<Fixes, ExitCode> {
    match fixes::with_cli(input, &o.fixes) {
        Ok((set, file)) => {
            if let Some(file) = file.filter(|_| o.warnings) {
                println!("fixes from {}", file.display());
            }
            Ok(set)
        }
        Err(e) => {
            println!("{e}");
            Err(ExitCode::FAILURE)
        }
    }
}

/// `-y`: the main file laid out into `-o` (spec `compiler/format`). Nothing is built, and nothing is written when
/// the front end reports an error.
fn format(o: &Options, input: &Path, name: &str, bytes: Vec<u8>) -> Result<ExitCode, String> {
    let Some(out) = &o.output else {
        return Err("formatting requires an output file: give -o <file>".into());
    };
    let settings = layout_settings(&o.settings)?;
    let include_root = include_root(o)?;
    let fixes = match program_fixes(o, input) {
        Ok(f) => f,
        Err(code) => return Ok(code),
    };
    // `-e` holds here too: `qb64pe -y -e` rejects a variable not declared (measured, 2026-11-04).
    let fe = frontend_explicit(
        Platform::host(),
        o.explicit,
        name,
        bytes,
        |file| Box::new(FileLoader::new(&include_root, file, input)),
        Some((input, &include_root)),
        fixes,
    );
    let errors = fe.diagnostics.error_count();
    if o.warnings {
        print!("{}", fe.render_with_warnings());
    } else if errors > 0 {
        print!("{}", fe.render_diagnostics());
    }
    if errors > 0 {
        println!("{}", fe.diagnostics.summary());
        return Ok(ExitCode::FAILURE);
    }
    let text = qb64rust_driver::format_program(&fe, &settings);
    std::fs::write(out, text).map_err(|e| format!("writing {}: {e}", out.display()))?;
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
    let exe = match &o.output {
        Some(p) => p.clone(),
        None => input.with_extension(Platform::host().exe_extension()),
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
    let fixes = match program_fixes(&o, &input) {
        Ok(f) => f,
        Err(code) => {
            if o.dump.is_none() && !o.cpp_only {
                compile::remove_old(&exe);
            }
            return Ok(code);
        }
    };
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
        debug: o.debug,
        warnings: o.warnings,
        explicit: o.explicit,
        settings: o.settings.clone(),
        fixes,
    };
    let built = compile::compile_exe(
        &input,
        &name,
        bytes,
        |file| Box::new(FileLoader::new(&include_root, file, &input)),
        &options,
        &mut |text| print!("{text}"),
    )?;
    Ok(if built { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}
