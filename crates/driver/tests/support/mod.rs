//! The child processes of the CLI tests, each with a deadline (`study\archive\31` item 1). `Command::output` waits for
//! ever, and a generated program whose runtime error opens libqb's dialog waits for a person: on 2026-10-16 a test
//! waited 2 h 15 min on one. Here every child runs with stdin closed (or fed from a buffer) and a deadline; past it,
//! its whole process tree is killed and the test fails, naming the phase and showing what the child had printed. Each
//! run writes a line `cli-time <test> <phase> <seconds>` to the test's captured stderr (shown with `--nocapture`).

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
fn kill_tree(child: &mut Child) {
    #[cfg(windows)]
    let _ = Command::new("taskkill")
        .args(["/T", "/F", "/PID", &child.id().to_string()])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let _ = child.kill();
    let _ = child.wait();
}

/// Runs `cmd` with stdin closed, or fed `input` and then closed, and both outputs read whole. Past `limit` the
/// process tree is killed and the result is a [`Timeout`] holding what it had printed.
pub fn try_run(cmd: &mut Command, input: Option<&[u8]>, limit: Duration) -> Result<Output, Timeout> {
    let program = cmd.get_program().to_string_lossy().into_owned();
    cmd.stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
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
    clone_lock_dir().join("objects.lock")
}

/// The variable that names [`objects_lock`] to the driver (`OBJECTS_LOCK_ENV` of `crates\driver\src\build.rs`).
pub const OBJECTS_LOCK_ENV: &str = "QB64RUST_OBJECTS_LOCK";
