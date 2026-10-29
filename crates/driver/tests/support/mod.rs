//! The child processes of the CLI tests, each with a deadline (`study\archive\31` item 1). `Command::output` waits for
//! ever, and a generated program whose runtime error opens libqb's dialog waits for a person: on 2026-10-16 a test
//! waited 2 h 15 min on one. Here every child runs with stdin closed (or fed from a buffer) and a deadline; past it,
//! its whole process tree is killed and the test fails, naming the phase and showing what the child had printed. Each
//! run writes a line `cli-time <test> <phase> <seconds>` to the test's captured stderr (shown with `--nocapture`).

pub mod dap;

use std::fmt;
use std::io::{Read, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

/// The deadline of a run of the compiler, or of a tool of the clone's toolchain: the test runner's
/// (`COMPILE_TIMEOUT`). A build takes seconds; the first after the copy's libqb objects are gone builds them too,
/// about 3 minutes more (`tools\legacy_tests\README.md`).
pub const COMPILER_LIMIT: Duration = Duration::from_secs(900);
/// The deadline of a generated program: the runner's for a corpus program (`CORPUS_RUN_TIMEOUT`). The tests' programs
/// end in well under a second, a screen program's announced waits in a few.
pub const PROGRAM_LIMIT: Duration = Duration::from_secs(60);
/// How long the output pipes may stay open after the child has ended or been killed: a process the child started
/// that outlived it may hold them.
const PIPE_GRACE: Duration = Duration::from_secs(10);

/// A child that did not end in time, killed with its tree, or one whose output pipes stayed open after it ended;
/// with what it had printed.
pub struct Timeout {
    pub program: String,
    pub limit: Duration,
    pub reason: &'static str,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl fmt::Display for Timeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} (limit {} s)\nstdout so far: {}\nstderr so far: {}",
            self.program,
            self.reason,
            self.limit.as_secs(),
            self.stdout.escape_ascii(),
            self.stderr.escape_ascii()
        )
    }
}

/// Reads a pipe to its end on a thread of its own, so that a child writing much to both pipes never blocks. The
/// buffer holds what has come so far; the receiver gets a message when the pipe is at its end.
fn read_pipe(mut pipe: impl Read + Send + 'static) -> (Arc<Mutex<Vec<u8>>>, mpsc::Receiver<()>) {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let (done, ended) = mpsc::channel();
    let to = Arc::clone(&buffer);
    thread::spawn(move || {
        let mut chunk = [0u8; 8192];
        while let Ok(n @ 1..) = pipe.read(&mut chunk) {
            to.lock().unwrap().extend_from_slice(&chunk[..n]);
        }
        let _ = done.send(());
    });
    (buffer, ended)
}

/// Kills the child and every process it started, then reaps it. `Child::kill` ends only the child itself: a
/// compiler's `make` and `g++`, or a program's own children, would live on and hold the build folder and the pipes.
/// `taskkill` is the runner's way too (`kill_tree` in `run_legacy_tests.py`); it is a system tool that does not wait
/// on what it kills, so it gets no deadline. The child is not reaped before it, so its process id is not reused.
/// Elsewhere the child leads a process group of its own ([`own_group`]) and `kill` ends the group: without it, a
/// `qb64rust dap` killed after a hang left `lldb-dap` and the program it debugged running (Linux CI, 2026-10-29).
fn kill_tree(child: &mut Child) {
    #[cfg(windows)]
    let mut cmd = Command::new("taskkill");
    #[cfg(windows)]
    cmd.args(["/T", "/F", "/PID", &child.id().to_string()]);
    #[cfg(not(windows))]
    let mut cmd = Command::new("kill");
    #[cfg(not(windows))]
    cmd.args(["-KILL", "--", &format!("-{}", child.id())]);
    let _ = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let _ = child.kill();
    let _ = child.wait();
}

/// Makes the child of `cmd` lead a process group of its own, for [`kill_tree`]; on Windows `taskkill /T` needs none.
/// A Ctrl+C at the terminal then reaches only the test binary, not the children in their groups.
fn own_group(cmd: &mut Command) {
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(cmd, 0);
    #[cfg(not(unix))]
    let _ = cmd;
}

/// Runs `cmd` with stdin closed, or fed `input` and then closed, and both outputs read whole. Past `limit` the
/// process tree is killed and the result is a [`Timeout`] holding what it had printed.
pub fn try_run(cmd: &mut Command, input: Option<&[u8]>, limit: Duration) -> Result<Output, Timeout> {
    let program = cmd.get_program().to_string_lossy().into_owned();
    cmd.stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    own_group(cmd);
    let mut child = cmd.spawn().unwrap_or_else(|e| panic!("cannot start {program}: {e}"));
    if let Some(input) = input {
        let mut stdin = child.stdin.take().unwrap();
        let input = input.to_vec();
        // A child that stops reading would block this write: on its own thread, it cannot hold up the deadline.
        thread::spawn(move || {
            let _ = stdin.write_all(&input);
        });
    }
    let (stdout, stdout_ended) = read_pipe(child.stdout.take().unwrap());
    let (stderr, stderr_ended) = read_pipe(child.stderr.take().unwrap());
    let start = Instant::now();
    let mut pause = Duration::from_millis(1);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break Some(status);
        }
        if start.elapsed() >= limit {
            kill_tree(&mut child);
            break None;
        }
        thread::sleep(pause);
        pause = (pause * 2).min(Duration::from_millis(50));
    };
    let grace = Instant::now() + PIPE_GRACE;
    let pipes_ended = [stdout_ended, stderr_ended].iter().all(|ended| {
        ended
            .recv_timeout(grace.saturating_duration_since(Instant::now()))
            .is_ok()
    });
    let take = |buffer: &Arc<Mutex<Vec<u8>>>| std::mem::take(&mut *buffer.lock().unwrap());
    let (stdout, stderr) = (take(&stdout), take(&stderr));
    // The limit named is the wait that ran out: the run's, or the pipes' after the child had ended.
    let (reason, limit) = match status {
        Some(status) if pipes_ended => return Ok(Output { status, stdout, stderr }),
        // Its tree cannot be found once the child has ended: `taskkill /T` follows the parent's process id.
        Some(_) => (
            "ended, but a process it started kept its output open (that process may still run)",
            PIPE_GRACE,
        ),
        None => ("did not end in time; its process tree was killed", limit),
    };
    Err(Timeout {
        program,
        limit,
        reason,
        stdout,
        stderr,
    })
}

/// [`try_run`] for a test: a [`Timeout`] fails the test, naming `phase` (`compiler`, `program`, `toolchain`, …).
pub fn run(cmd: &mut Command, input: Option<&[u8]>, limit: Duration, phase: &str) -> Output {
    let start = Instant::now();
    let result = try_run(cmd, input, limit);
    let test = thread::current().name().unwrap_or("?").to_owned();
    eprintln!("cli-time {test} {phase} {:.2}", start.elapsed().as_secs_f64());
    result.unwrap_or_else(|timeout| panic!("{phase}: {timeout}"))
}

/// A generated program, to be run unattended: a runtime error is written to stderr and ends the program, with no
/// dialog that waits for a person (`QB64PE_NOPROMPT=y`). A test of another mode sets the variable again.
pub fn unattended(exe: &Path) -> Command {
    let mut cmd = Command::new(exe);
    cmd.env("QB64PE_NOPROMPT", "y");
    cmd
}

/// A tool of the toolchain a build uses (`nm.exe`, `lldb.exe`), by its Windows file name (off Windows without
/// `.exe`, and of a system toolchain the first on `PATH`); `""`: its `bin` folder.
pub fn toolchain(name: &str) -> PathBuf {
    use qb64rust_driver::toolchain::{Source, find};
    let found = find(qb64rust_base::Platform::host(), None, None).expect("a C++ toolchain");
    let name = if cfg!(windows) {
        name
    } else {
        name.trim_end_matches(".exe")
    };
    if found.source == Source::System {
        if name.is_empty() {
            return found.folder;
        }
        if let Some(p) = found.path.iter().map(|d| d.join(name)).find(|p| p.is_file()) {
            return p;
        }
        return found.folder.join(name);
    }
    found.folder.join("bin").join(name)
}

/// The folder of the toolchain a build uses, as `--toolchain-root` takes it: of a system toolchain the folder above
/// the `bin` of its `g++` (`/usr`).
pub fn toolchain_folder() -> PathBuf {
    use qb64rust_driver::toolchain::{Source, find};
    let found = find(qb64rust_base::Platform::host(), None, None).expect("a C++ toolchain");
    if found.source == Source::System {
        return found.folder.parent().map(Path::to_path_buf).unwrap_or(found.folder);
    }
    found.folder
}

/// The deadline of a run of `lldb` in batch mode: it starts the program, runs the commands and exits, killing it;
/// a few seconds each.
pub const LLDB_LIMIT: Duration = Duration::from_secs(120);

/// The variable that names the file of [`lldb`]'s log.
pub const LLDB_LOG_ENV: &str = "QB64RUST_LLDB_LOG";

/// The lock that lets one debugger session of a test binary run at a time: a run of [`lldb`], or a session of
/// [`dap::Dap`] (`lldb-dap`, through the proxy or alone). `lldb-server`'s race after a single step ends a session
/// more often beside other sessions (design "Measured" of `m5-dap-proxy`).
pub fn debugger_alone() -> std::sync::MutexGuard<'static, ()> {
    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner())
}

/// The `lldb` of the toolchain's `lldb-dap` ([`qb64rust_driver::toolchain::lldb_dap`]): the same name with `lldb`
/// for `lldb-dap` (`lldb-dap.exe` gives `lldb.exe`, `lldb-dap-20` gives `lldb-20`).
pub fn lldb_beside_lldb_dap() -> PathBuf {
    use qb64rust_driver::toolchain;
    let found = toolchain::find(qb64rust_base::Platform::host(), None, None).expect("a C++ toolchain");
    let dap = toolchain::lldb_dap(&found, None).expect("lldb-dap");
    let name = dap.file_name().and_then(|n| n.to_str()).expect("lldb-dap's name");
    dap.with_file_name(name.replacen("lldb-dap", "lldb", 1))
}

/// Runs the clone's `lldb` in batch mode (`-b`) on `exe`, one `-o` per command, with `QB64PE_NOPROMPT=y` (the
/// program's runtime errors open no dialog) and [`LLDB_LIMIT`]; gives what it printed, stdout then stderr. After
/// the last command `lldb` exits and ends the program (design D4 of `m5-debug-build-symbols`). One run at a time in
/// a test binary: in a full `--ignored` run, beside the other tests' builds, one of several `lldb`s started together
/// once stopped after `breakpoint set` until its deadline (2026-10-25); a run takes about a second. When
/// [`LLDB_LOG_ENV`] names a file, `lldb` writes its log of stepping, threads and the process there first (task 1.1
/// of `m5-dap-proxy`, the hang of a `next`). The `lldb` is the one beside the `lldb-dap` the adapter starts: on
/// Linux `lldb-20` beside `lldb-dap-20`, as Ubuntu's versioned packages install them (`m7-linux` task 4.2).
#[expect(clippy::disallowed_methods, reason = "lldb's own messages")]
pub fn lldb(exe: &Path, commands: &[&str]) -> String {
    let _alone = debugger_alone();
    let mut cmd = Command::new(lldb_beside_lldb_dap());
    cmd.arg("-b").env("QB64PE_NOPROMPT", "y");
    for (name, value) in qb64rust_dap::LLDB_ENV {
        cmd.env(name, value);
    }
    if let Some(log) = std::env::var_os(LLDB_LOG_ENV) {
        cmd.arg("-o").arg(format!(
            "log enable -T -f \"{}\" lldb step thread process",
            Path::new(&log).display()
        ));
    }
    for c in commands {
        cmd.arg("-o").arg(c);
    }
    cmd.arg(exe);
    if let Some(dir) = exe.parent() {
        cmd.current_dir(dir);
    }
    let o = run(&mut cmd, None, LLDB_LIMIT, "lldb");
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}

/// The folder of the clone's locks (`tools\legacy_tests\clone_lock.py`, `CLONE_LOCK_DIR`): under the main checkout's
/// `target`, also when the tests run in a git worktree, so that its runs and the main checkout's see each other.
fn clone_lock_dir() -> PathBuf {
    main_checkout(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")).join("target/qb64pe-clone")
}

/// The checkout whose `target` holds the locks: `repo`, or for a git worktree (its `.git` a file `gitdir: <main>/.git/
/// worktrees/<name>`, whose `commondir` file names the main `.git`) the main checkout (`clone_lock.py`,
/// `main_checkout`).
#[expect(clippy::disallowed_methods, reason = "git's files of paths, not BASIC source")]
fn main_checkout(repo: &Path) -> PathBuf {
    let worktree = || -> Option<PathBuf> {
        let text = std::fs::read_to_string(repo.join(".git")).ok()?;
        let gitdir = repo.join(text.trim().strip_prefix("gitdir:")?.trim());
        let common = std::fs::read_to_string(gitdir.join("commondir")).ok()?;
        let dot_git = std::fs::canonicalize(gitdir.join(common.trim())).ok()?;
        Some(dot_git.parent()?.to_path_buf())
    };
    worktree().unwrap_or_else(|| repo.to_path_buf())
}

/// The lock the driver takes around each `make` when [`OBJECTS_LOCK_ENV`] names it: shared while the copy's libqb
/// objects are up to date, alone while they are built (`clone_lock.py`, `OBJECTS_LOCK`). Every compiler a test
/// starts is given it, so the tests' builds and a test runner's run of the new compiler build side by side.
pub fn objects_lock() -> PathBuf {
    let dir = clone_lock_dir();
    // A fresh checkout (a copy synced into WSL) has no runner's folder yet.
    let _ = std::fs::create_dir_all(&dir);
    dir.join("objects.lock")
}

/// The variable that names [`objects_lock`] to the driver (`OBJECTS_LOCK_ENV` of `crates\driver\src\build.rs`).
pub const OBJECTS_LOCK_ENV: &str = "QB64RUST_OBJECTS_LOCK";
