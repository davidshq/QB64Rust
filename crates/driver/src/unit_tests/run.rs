//! Building and running tests (spec `compiler/unit-tests`, "Running a test"; design D3 and D6 of
//! `basic-unit-tests`). Each test file is built once into a folder of its own under the system temp folder, then
//! each test runs in a process of its own with a timeout. Builds, then runs, go `jobs` at a time.

use super::discover::TestSub;
use super::entry::{NAME_ENV, RESULT_ENV, TEMP_ENV, entry_path, entry_point};
use super::judge::{self, Outcome};
use super::loader::TestLoader;
use crate::compile::{CompileOptions, compile_exe};
use crate::fixes::{CliFix, with_cli};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A test file and the tests of it to run.
pub struct TestFile {
    /// As found: shown in the output and matched by `--filter`.
    pub path: PathBuf,
    pub tests: Vec<TestSub>,
}

/// How tests are built and run.
pub struct RunOptions {
    pub jobs: usize,
    pub timeout: Duration,
    /// The compiler root.
    pub include_root: PathBuf,
    /// `--qb64pe-root`, `--toolchain-root`, `--libqb-root`.
    pub root: Option<PathBuf>,
    pub toolchain_root: Option<PathBuf>,
    pub libqb_root: Option<PathBuf>,
    /// `--fixes` and `--fix`, over each test file's settings file.
    pub fixes: Vec<CliFix>,
}

/// The result of one test.
#[derive(Debug)]
pub struct TestResult {
    /// The test file as found.
    pub file: String,
    pub name: String,
    /// The line of the test SUB.
    pub line: u32,
    pub outcome: Outcome,
    pub message: String,
    /// Where the failure is, when libqb named a line: a file and its line.
    pub location: Option<(String, u32)>,
    /// What the test printed: stdout, then stderr.
    pub output: String,
    pub ms: u128,
    /// The test's folder, kept after a failure.
    pub kept: Option<PathBuf>,
}

/// FNV-1a, 64 bits: the name of a test file's build folder, from its canonical path.
fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// The folder of a test file's executable and its tests' files in this run: `<temp>/qb64rust-test/<fnv1a64 of its
/// path>/<process id>`. Two test files never share one, nor do two runs of one file at once (the editor's and a
/// terminal's), which build it with other filters.
pub fn build_folder(test_file: &Path) -> PathBuf {
    let canonical = std::fs::canonicalize(test_file).unwrap_or_else(|_| test_file.to_path_buf());
    std::env::temp_dir()
        .join("qb64rust-test")
        .join(format!("{:016x}", fnv1a64(canonical.to_string_lossy().as_bytes())))
        .join(std::process::id().to_string())
}

/// The lock of a run's folder: `<folder>.lock` beside it, held for the whole run.
fn lock_path(folder: &Path) -> PathBuf {
    folder.with_extension("lock")
}

/// Marks the folder `folder` of this run as in use: its lock is taken before the folder is made, so another run
/// never takes a folder in use for one left behind. `None` when the lock cannot be had.
fn claim(folder: &Path) -> Option<std::fs::File> {
    std::fs::create_dir_all(folder.parent()?).ok()?;
    let lock = std::fs::File::create(lock_path(folder)).ok()?;
    lock.lock().ok()?;
    Some(lock)
}

/// Removes the folders that earlier runs of the same test file left (kept after a failure, or ended before they
/// could clean up): every sibling of this run's folder `folder` whose lock no run holds.
fn remove_left_behind(folder: &Path) {
    let Some(Ok(entries)) = folder.parent().map(std::fs::read_dir) else {
        return;
    };
    for path in entries.flatten().map(|e| e.path()) {
        if path == folder || !path.is_dir() {
            continue;
        }
        let lock = lock_path(&path);
        let free = std::fs::File::open(&lock).map_or(true, |f| f.try_lock().is_ok());
        if free {
            let _ = std::fs::remove_dir_all(&path);
            let _ = std::fs::remove_file(&lock);
        }
    }
}

/// Removes a test file's folder of this run and its lock (released by the caller), and the folder above it once no
/// run has one there.
fn remove_build_folder(test_file: &Path) {
    let folder = build_folder(test_file);
    let _ = std::fs::remove_dir_all(&folder);
    let _ = std::fs::remove_file(lock_path(&folder));
    if let Some(parent) = folder.parent() {
        let _ = std::fs::remove_dir(parent);
    }
}

/// Runs `work` on each item, `jobs` at a time, on threads with the compiler's stack; `done` gets each result on the
/// calling thread as it comes.
fn pool<T: Send, R: Send>(jobs: usize, items: Vec<T>, work: impl Fn(T) -> R + Sync, mut done: impl FnMut(R)) {
    let queue = Mutex::new(items.into_iter());
    let (tx, rx) = mpsc::channel();
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            let tx = tx.clone();
            let (queue, work) = (&queue, &work);
            std::thread::Builder::new()
                .stack_size(qb64rust_base::STACK_SIZE)
                .spawn_scoped(s, move || {
                    loop {
                        let next = queue.lock().unwrap_or_else(|e| e.into_inner()).next();
                        let Some(item) = next else { break };
                        if tx.send(work(item)).is_err() {
                            break;
                        }
                    }
                })
                .expect("cannot start a test thread");
        }
        drop(tx);
        for r in rx {
            done(r);
        }
    });
}

/// Builds the test file `file` with its generated entry point; its executable, or the compiler's message.
fn build(file: &TestFile, o: &RunOptions) -> Result<PathBuf, String> {
    let path = std::path::absolute(&file.path).map_err(|e| e.to_string())?;
    let folder = build_folder(&path);
    std::fs::create_dir_all(&folder).map_err(|e| format!("creating {}: {e}", folder.display()))?;
    let stem = path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
    let exe = folder.join(format!("{stem}{}", std::env::consts::EXE_SUFFIX));
    let main = entry_path(&path);
    let file_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let (fixes, _) = with_cli(&main, &o.fixes).map_err(|e| e.to_string())?;
    let options = CompileOptions {
        exe: exe.clone(),
        quiet: true,
        root: o.root.clone(),
        toolchain_root: o.toolchain_root.clone(),
        libqb_root: o.libqb_root.clone(),
        include_root: o.include_root.clone(),
        fixes,
        ..CompileOptions::default()
    };
    let mut text = String::new();
    let built = compile_exe(
        &main,
        &main.to_string_lossy(),
        entry_point(&file_name, &file.tests).into_bytes(),
        |id| Box::new(TestLoader::new(&o.include_root, id, &main)),
        &options,
        &mut |t| text.push_str(t),
    );
    match built {
        Ok(true) => Ok(exe),
        Ok(false) => Err(text.trim_end().to_string()),
        Err(e) => Err(format!("{text}{e}").trim_end().to_string()),
    }
}

/// Reads a pipe to its end in a thread of its own, into a buffer the caller can take at any time: a child the test
/// started may hold the pipe open after the test's process ends.
fn drain(mut pipe: impl Read + Send + 'static) -> (Arc<Mutex<Vec<u8>>>, std::thread::JoinHandle<()>) {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let into = Arc::clone(&buf);
    let handle = std::thread::spawn(move || {
        let mut chunk = [0u8; 8192];
        while let Ok(n) = pipe.read(&mut chunk) {
            if n == 0 {
                break;
            }
            into.lock()
                .unwrap_or_else(|e| e.into_inner())
                .extend_from_slice(&chunk[..n]);
        }
    });
    (buf, handle)
}

/// Waits up to `limit` for a reader to reach the end of its pipe; what it read either way.
fn take(reader: (Arc<Mutex<Vec<u8>>>, std::thread::JoinHandle<()>), limit: Duration) -> String {
    let start = Instant::now();
    while !reader.1.is_finished() && start.elapsed() < limit {
        std::thread::sleep(Duration::from_millis(10));
    }
    let bytes = reader.0.lock().unwrap_or_else(|e| e.into_inner()).clone();
    latin1(&bytes)
}

/// A program's bytes as text, one character per byte (as the extension reads a build's output): its CP437 text
/// keeps every byte.
fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| char::from(b)).collect()
}

/// Runs the test `test` of `file` from the executable `exe`, and judges it.
fn run_one(file: &TestFile, test: &TestSub, exe: &Path, o: &RunOptions) -> TestResult {
    let start = Instant::now();
    let mut result = TestResult {
        file: file.path.display().to_string(),
        name: test.name.clone(),
        line: test.line,
        outcome: Outcome::Error,
        message: String::new(),
        location: None,
        output: String::new(),
        ms: 0,
        kept: None,
    };
    let path = std::path::absolute(&file.path).unwrap_or_else(|_| file.path.clone());
    let dir = path.parent().map_or_else(PathBuf::new, Path::to_path_buf);
    let folder = build_folder(&path).join("tests").join(&test.name);
    let temp = folder.join("temp");
    let result_file = folder.join("result.txt");
    let _ = std::fs::remove_dir_all(&folder);
    if let Err(e) = std::fs::create_dir_all(&temp) {
        result.message = format!("creating {}: {e}", temp.display());
        return result;
    }
    let mut temp_env = temp.clone().into_os_string();
    temp_env.push(std::path::MAIN_SEPARATOR_STR);
    let mut cmd = Command::new(exe);
    cmd.current_dir(&dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("QB64PE_NOPROMPT", "y")
        .env(NAME_ENV, &test.name)
        .env(RESULT_ENV, &result_file)
        .env(TEMP_ENV, temp_env);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt as _;
        // A console program started by a program with no console would open a window of its own.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            result.message = format!("starting {}: {e}", exe.display());
            return result;
        }
    };
    let out = child.stdout.take().map(drain);
    let err = child.stderr.take().map(drain);
    // The test's process only: a child it started is not ended with it (design D6).
    let mut timed_out = false;
    let exit = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status.code(),
            Ok(None) if start.elapsed() >= o.timeout => {
                timed_out = true;
                let _ = child.kill();
                break child.wait().ok().and_then(|s| s.code());
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(10)),
            Err(_) => break None,
        }
    };
    let limit = Duration::from_secs(2);
    let stdout = out.map(|r| take(r, limit)).unwrap_or_default();
    let stderr = err.map(|r| take(r, limit)).unwrap_or_default();
    let results = latin1(&std::fs::read(&result_file).unwrap_or_default());
    let verdict = judge::judge(&judge::Run {
        result: &results,
        stdout: &stdout,
        stderr: &stderr,
        exit,
        timed_out,
    });
    result.ms = start.elapsed().as_millis();
    result.outcome = verdict.outcome;
    result.message = if timed_out {
        format!("timed out after {} s", o.timeout.as_secs())
    } else {
        verdict.message
    };
    result.location = verdict
        .location
        .and_then(|(f, line)| located(&f, &path, &file.path).map(|f| (f, line)));
    result.output = stdout + &stderr;
    if result.outcome == Outcome::Pass {
        let _ = std::fs::remove_dir_all(&folder);
    } else {
        result.kept = Some(temp);
    }
    result
}

/// The file of a location libqb named, `None` for the entry point (`main module`): as the test file was found
/// when it is that file, else the path libqb gave, from the test file's folder when relative.
fn located(named: &str, test_file: &Path, shown: &Path) -> Option<String> {
    if named == "main module" {
        return None;
    }
    let dir = test_file.parent().unwrap_or(Path::new(""));
    let p = dir.join(named);
    let same = |a: &Path, b: &Path| match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    };
    Some(if same(&p, test_file) {
        shown.display().to_string()
    } else {
        p.display().to_string()
    })
}

/// Builds every test file, then runs every test of the files that built; `on_result` gets each result as it comes.
/// A file that does not build gives each of its tests a build error with the compiler's message. The folder of a
/// file whose tests all passed is removed at the end; one kept after a failure goes at the file's next run.
pub fn run_all(files: &[TestFile], o: &RunOptions, mut on_result: impl FnMut(TestResult)) {
    let absolute = |f: &TestFile| std::path::absolute(&f.path).unwrap_or_else(|_| f.path.clone());
    let mut claims: Vec<Option<std::fs::File>> = files
        .iter()
        .map(|f| {
            let folder = build_folder(&absolute(f));
            let lock = claim(&folder);
            if lock.is_some() {
                remove_left_behind(&folder);
            }
            lock
        })
        .collect();
    let mut built: Vec<Option<PathBuf>> = vec![None; files.len()];
    pool(
        o.jobs,
        (0..files.len()).collect(),
        |i| (i, build(&files[i], o)),
        |(i, r)| match r {
            Ok(exe) => built[i] = Some(exe),
            Err(message) => {
                for t in &files[i].tests {
                    on_result(TestResult {
                        file: files[i].path.display().to_string(),
                        name: t.name.clone(),
                        line: t.line,
                        outcome: Outcome::BuildError,
                        message: message.clone(),
                        location: None,
                        output: String::new(),
                        ms: 0,
                        // Only a folder the build wrote to: a front end's error leaves it empty.
                        kept: Some(build_folder(&absolute(&files[i])))
                            .filter(|d| std::fs::read_dir(d).is_ok_and(|mut e| e.next().is_some())),
                    });
                }
            }
        },
    );
    let runs: Vec<(usize, usize)> = (0..files.len())
        .filter(|&i| built[i].is_some())
        .flat_map(|i| (0..files[i].tests.len()).map(move |t| (i, t)))
        .collect();
    let built = &built;
    let mut failed = vec![false; files.len()];
    pool(
        o.jobs,
        runs,
        |(i, t)| {
            let exe = built[i].as_deref().unwrap_or(Path::new(""));
            (i, run_one(&files[i], &files[i].tests[t], exe, o))
        },
        |(i, r)| {
            failed[i] |= r.outcome != Outcome::Pass;
            on_result(r);
        },
    );
    // A file whose tests all passed leaves nothing; a failure keeps the folders its results name.
    for (i, f) in files.iter().enumerate() {
        drop(claims[i].take());
        if built[i].is_some() && !failed[i] {
            remove_build_folder(&absolute(f));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a64_of_known_strings() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn two_files_two_folders() {
        assert_ne!(
            build_folder(Path::new("a/x_test.bas")),
            build_folder(Path::new("b/x_test.bas"))
        );
    }

    #[test]
    fn folders_left_behind_go_and_those_in_use_stay() {
        let d = tempfile::tempdir().unwrap();
        let own = d.path().join("hash").join("10");
        let _mine = claim(&own).unwrap();
        std::fs::create_dir_all(&own).unwrap();
        // A run that failed: its folder kept, its lock released.
        let failed = d.path().join("hash").join("20");
        drop(claim(&failed).unwrap());
        std::fs::create_dir_all(failed.join("tests")).unwrap();
        // A run that ended before it took a lock.
        let unlocked = d.path().join("hash").join("30");
        std::fs::create_dir_all(&unlocked).unwrap();
        // A run going on beside this one.
        let busy = d.path().join("hash").join("40");
        let _other = claim(&busy).unwrap();
        std::fs::create_dir_all(&busy).unwrap();
        remove_left_behind(&own);
        assert!(own.is_dir() && busy.is_dir() && lock_path(&busy).is_file());
        assert!(!failed.exists() && !lock_path(&failed).exists() && !unlocked.exists());
    }
}
