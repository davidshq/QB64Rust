//! The C++ toolchain a build and the debug adapter use (spec `compiler/toolchain`, design D1 to D4 of
//! `m6-toolchain`): where it is looked for ([`find`]), and its installation by `qb64rust toolchain install`
//! ([`install`]): the pinned upstream release of llvm-mingw, downloaded only when that command runs.
//!
//! A toolchain folder has llvm-mingw's layout (`bin\c++.exe`, `bin\mingw32-make.exe`, `bin\lldb-dap.exe`, …);
//! QB64pe's `internal\c\c_compiler` is one.

use crate::build;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

/// The pinned release (design D1): byte for byte the reference clone's toolchain (design "Measured"). Changing it is
/// a change of its own, judged by the full test set.
pub const RELEASE: &str = "llvm-mingw-20260922-ucrt-x86_64";
pub const URL: &str =
    "https://github.com/mstorsjo/llvm-mingw/releases/download/20260922/llvm-mingw-20260922-ucrt-x86_64.zip";
pub const SIZE: u64 = 190_725_905;
pub const SHA256: &str = "e3ad77d117a4bea19a7a3b333341824d79a5a371004a10e25b8504e7b3047666";
/// The disk space the installed folder takes, in MB (design "Measured"), for the messages.
pub const DISK_MB: u64 = 460;

/// The release's tag, `20260922` of [`RELEASE`], as the messages name it.
pub fn release_tag() -> &'static str {
    RELEASE.split('-').nth(2).unwrap_or(RELEASE)
}

/// `--toolchain-root` as a variable.
pub const ROOT_ENV: &str = "QB64RUST_TOOLCHAIN_ROOT";
/// The folder the install goes into instead of `%LOCALAPPDATA%\qb64rust\toolchain` (design D3).
pub const HOME_ENV: &str = "QB64RUST_TOOLCHAIN_HOME";
/// The file an install writes into the toolchain folder last; a folder without it is not installed (design D3).
pub const MARKER: &str = "qb64rust-toolchain.txt";

/// What the install leaves out: the other three targets' folders and the Linux runtime libraries, none of which an
/// x86_64 Windows build reads (design "Measured").
const NOT_KEPT: [&str; 4] = [
    "aarch64-w64-mingw32",
    "armv7-w64-mingw32",
    "i686-w64-mingw32",
    "lib/clang/23/lib/linux",
];

/// Where the toolchain a lookup found came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// `--toolchain-root`.
    Option,
    /// `QB64RUST_TOOLCHAIN_ROOT`.
    Variable,
    /// The `internal\c\c_compiler` of a QB64pe folder given by `--qb64pe-root` or `QB64RUST_QB64PE_ROOT`.
    Qb64pe,
    /// The one `qb64rust toolchain install` put in place.
    Installed,
    /// The `internal\c\c_compiler` of the default clone, `..\QB64pe` next to the repository.
    Clone,
}

impl Source {
    /// The line `qb64rust toolchain path` prints after the folder: a word, a colon, what it means. The extension
    /// reads the word.
    pub fn describe(self) -> &'static str {
        match self {
            Source::Option => "option: given by --toolchain-root",
            Source::Variable => "variable: given by QB64RUST_TOOLCHAIN_ROOT",
            Source::Qb64pe => "qb64pe: the toolchain of the QB64pe folder given",
            Source::Installed => "installed: the toolchain `qb64rust toolchain install` put in place",
            Source::Clone => "clone: the toolchain of the QB64pe clone next to the repository",
        }
    }
}

/// A toolchain folder and where it came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    pub folder: PathBuf,
    pub source: Source,
}

impl Found {
    /// A program of the toolchain, by file name (`c++.exe`, `nm.exe`, `lldb-dap.exe`).
    pub fn bin(&self, name: &str) -> PathBuf {
        self.folder.join("bin").join(name)
    }
}

/// Whether `dir` holds the programs a build runs.
fn is_toolchain(dir: &Path) -> bool {
    dir.join("bin/c++.exe").is_file() && dir.join("bin/mingw32-make.exe").is_file()
}

/// The message of spec `compiler/toolchain`, "No toolchain found", built here alone.
pub fn missing_message() -> String {
    format!(
        "no C++ toolchain found: install it with `qb64rust toolchain install` (llvm-mingw {} from \
         github.com/mstorsjo/llvm-mingw, a {} MB download, about {DISK_MB} MB on disk), or give a toolchain folder \
         with --toolchain-root <dir> or the environment variable {ROOT_ENV}",
        release_tag(),
        SIZE / 1_000_000
    )
}

/// What a lookup reads, gathered so that the tests can give it without the process's environment.
#[derive(Default)]
struct Lookup {
    option: Option<PathBuf>,
    variable: Option<PathBuf>,
    /// The QB64pe folder given explicitly.
    qb64pe: Option<PathBuf>,
    /// The install root ([`home`]).
    home: Option<PathBuf>,
    /// The default clone, unless a QB64pe folder was given or `QB64RUST_QB64PE_ROOT` is set (empty: none).
    clone: Option<PathBuf>,
}

/// A variable's value, with an empty one as unset.
fn var(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from)
}

/// The install root: `QB64RUST_TOOLCHAIN_HOME`, else `%LOCALAPPDATA%\qb64rust\toolchain`.
pub fn home() -> Option<PathBuf> {
    var(HOME_ENV).or_else(|| var("LOCALAPPDATA").map(|d| d.join("qb64rust").join("toolchain")))
}

/// The installed toolchain under `home`: its folder, when it holds the marker an install writes last and the
/// programs a build runs.
pub fn installed(home: &Path) -> Option<PathBuf> {
    let folder = home.join(RELEASE);
    (folder.join(MARKER).is_file() && is_toolchain(&folder)).then_some(folder)
}

/// Finds the toolchain in the order of spec `compiler/toolchain` (design D4): `--toolchain-root`
/// (`toolchain_option`), `QB64RUST_TOOLCHAIN_ROOT`, the `internal\c\c_compiler` of a QB64pe folder given by
/// `--qb64pe-root` (`qb64pe_option`) or `QB64RUST_QB64PE_ROOT`, the installed toolchain, the default clone's. A folder
/// named by the option or the variable must be a toolchain; the others count only when they are one. A QB64pe folder
/// given must be one, as for a build ([`build::find_root`]), so that `toolchain path` and `dap` agree with it.
pub fn find(toolchain_option: Option<&Path>, qb64pe_option: Option<&Path>) -> Result<Found, String> {
    let qb64pe_var = std::env::var_os(build::ROOT_ENV);
    let qb64pe_given = qb64pe_option.is_some() || qb64pe_var.as_ref().is_some_and(|v| !v.is_empty());
    let places = Lookup {
        option: toolchain_option.map(Path::to_path_buf),
        variable: var(ROOT_ENV),
        qb64pe: if qb64pe_given {
            build::find_root(qb64pe_option)?
        } else {
            None
        },
        home: home(),
        clone: (qb64pe_option.is_none() && qb64pe_var.is_none()).then(build::default_clone),
    };
    find_in(&places)
}

fn find_in(p: &Lookup) -> Result<Found, String> {
    let found = |folder: &Path, source| Found {
        folder: std::path::absolute(folder).unwrap_or_else(|_| folder.to_path_buf()),
        source,
    };
    for (given, source, how) in [
        (&p.option, Source::Option, "--toolchain-root"),
        (&p.variable, Source::Variable, ROOT_ENV),
    ] {
        if let Some(dir) = given {
            if is_toolchain(dir) {
                return Ok(found(dir, source));
            }
            return Err(format!(
                "{} (given by {how}) is not a C++ toolchain folder: it has no bin\\c++.exe or bin\\mingw32-make.exe",
                dir.display()
            ));
        }
    }
    let compiler = |q: &PathBuf| q.join("internal").join("c").join("c_compiler");
    if let Some(dir) = p.qb64pe.as_ref().map(compiler).filter(|d| is_toolchain(d)) {
        return Ok(found(&dir, Source::Qb64pe));
    }
    if let Some(dir) = p.home.as_deref().and_then(installed) {
        return Ok(found(&dir, Source::Installed));
    }
    if let Some(dir) = p.clone.as_ref().map(compiler).filter(|d| is_toolchain(d)) {
        return Ok(found(&dir, Source::Clone));
    }
    Err(missing_message())
}

/// `qb64rust toolchain install` (design D2, D3): the pinned release into [`home`].
pub fn install() -> Result<PathBuf, String> {
    let home = home().ok_or_else(|| format!("no folder to install into: set LOCALAPPDATA or {HOME_ENV}"))?;
    install_from(URL, SHA256, SIZE, &home)
}

/// `%SystemRoot%\System32\<name>`, by its full path: Git for Windows puts a GNU `tar` that cannot read a zip first
/// on many a `PATH` (design D2).
fn system_tool(name: &str) -> Result<PathBuf, String> {
    let root = var("SystemRoot").unwrap_or_else(|| PathBuf::from("C:\\Windows"));
    let p = root.join("System32").join(name);
    if p.is_file() {
        Ok(p)
    } else {
        Err(format!("{} not found", p.display()))
    }
}

/// What to do by hand when the install cannot: fetch the zip, unpack it, name the folder.
fn by_hand(url: &str) -> String {
    format!(
        "fetch {url} by hand, unpack it, and give its folder with --toolchain-root <dir> or the environment variable \
         {ROOT_ENV}"
    )
}

/// Installs the release at `url`, whose SHA-256 is `sha256` and whose size is `size` (for the progress lines; 0:
/// unknown), into `home\<RELEASE>` (design D3): under the lock `home\install.lock`; nothing done when the folder is
/// installed already; an unmarked folder (an interrupted install) deleted first; the zip downloaded into
/// `home\download`, checked, unpacked into `home`, the folders of [`NOT_KEPT`] removed, the zip deleted, and the
/// marker written last. Progress goes to standard output. Folders of other releases are left alone. The released
/// binary calls it only with the pinned constants ([`install`]); the tests give a `file:///` URL.
pub fn install_from(url: &str, sha256: &str, size: u64, home: &Path) -> Result<PathBuf, String> {
    let io = |what: &str, p: &Path, e: std::io::Error| format!("{what} {}: {e}", p.display());
    std::fs::create_dir_all(home).map_err(|e| io("creating", home, e))?;
    let lock_path = home.join("install.lock");
    let lock = std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|e| io("opening", &lock_path, e))?;
    lock.lock().map_err(|e| io("locking", &lock_path, e))?;

    let folder = home.join(RELEASE);
    if installed(home).is_some() {
        println!("installed already: {}", folder.display());
        return Ok(folder);
    }
    if folder.exists() {
        println!("removing an interrupted install: {}", folder.display());
        std::fs::remove_dir_all(&folder).map_err(|e| io("removing", &folder, e))?;
    }
    let curl = system_tool("curl.exe").map_err(|e| format!("cannot download the toolchain: {e}; {}", by_hand(url)))?;
    let tar = system_tool("tar.exe").map_err(|e| format!("cannot unpack the toolchain: {e}; {}", by_hand(url)))?;

    let download = home.join("download");
    std::fs::create_dir_all(&download).map_err(|e| io("creating", &download, e))?;
    let zip = download.join(url.rsplit('/').next().unwrap_or("toolchain.zip"));
    let _ = std::fs::remove_file(&zip);
    // Whatever goes wrong from here, nothing is left behind: not the zip, not a half-unpacked folder.
    let clean = || {
        let _ = std::fs::remove_file(&zip);
        let _ = std::fs::remove_dir(&download);
        let _ = std::fs::remove_dir_all(&folder);
    };

    println!("downloading {url}");
    let mut child = Command::new(&curl)
        .args(["-fL", "--retry", "3", "-sS", "-o"])
        .arg(&zip)
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            clean();
            format!("cannot run {}: {e}; {}", curl.display(), by_hand(url))
        })?;
    let mut shown = None;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {}
            Err(e) => {
                let _ = child.kill();
                clean();
                return Err(format!("waiting for {}: {e}", curl.display()));
            }
        }
        let have = std::fs::metadata(&zip).map_or(0, |m| m.len());
        if let Some(percent) = have.saturating_mul(100).checked_div(size).map(|p| p.min(100)) {
            // Every 5 %, so that the extension's progress moves without a flood of lines.
            if shown.is_none_or(|s| percent >= s + 5) {
                println!("downloading {percent}%");
                shown = Some(percent - percent % 5);
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    };
    if !status.success() {
        let mut why = Vec::new();
        if let Some(mut err) = child.stderr.take() {
            let _ = err.read_to_end(&mut why);
        }
        clean();
        #[expect(clippy::disallowed_methods, reason = "curl's own message")]
        let why = String::from_utf8_lossy(&why).trim().to_string();
        return Err(format!(
            "downloading the toolchain failed ({why}, curl exit status {}); {}",
            status.code().map_or_else(|| "none".to_string(), |c| c.to_string()),
            by_hand(url)
        ));
    }
    println!("downloading 100%");

    println!("checking");
    let got = sha256_of(&zip).inspect_err(|_| clean())?;
    if !got.eq_ignore_ascii_case(sha256) {
        clean();
        return Err(format!(
            "the downloaded toolchain's SHA-256 is {got}, not {sha256}: the file is deleted and nothing is installed"
        ));
    }

    println!("unpacking");
    let out = Command::new(&tar)
        .arg("-xf")
        .arg(&zip)
        .arg("-C")
        .arg(home)
        .output()
        .map_err(|e| {
            clean();
            format!("cannot run {}: {e}; {}", tar.display(), by_hand(url))
        })?;
    if !out.status.success() || !is_toolchain(&folder) {
        #[expect(clippy::disallowed_methods, reason = "tar's own message")]
        let why = String::from_utf8_lossy(&out.stderr).trim().to_string();
        clean();
        return Err(format!(
            "unpacking the toolchain failed (tar exit status {:?}{}{}); {}",
            out.status.code(),
            if why.is_empty() { "" } else { ": " },
            why,
            by_hand(url)
        ));
    }
    for part in NOT_KEPT {
        let p = folder.join(part);
        if p.exists() {
            std::fs::remove_dir_all(&p).map_err(|e| {
                clean();
                io("removing", &p, e)
            })?;
        }
    }
    let _ = std::fs::remove_file(&zip);
    let _ = std::fs::remove_dir(&download);
    let marker = folder.join(MARKER);
    std::fs::write(&marker, format!("{RELEASE}\n{sha256}\n")).map_err(|e| {
        clean();
        io("writing", &marker, e)
    })?;
    println!("installed: {}", folder.display());
    Ok(folder)
}

/// The SHA-256 of a file, in lower-case hex.
fn sha256_of(p: &Path) -> Result<String, String> {
    use sha2::Digest as _;
    let mut file = std::fs::File::open(p).map_err(|e| format!("reading {}: {e}", p.display()))?;
    let mut hasher = sha2::Sha256::new();
    let mut buf = vec![0; 1 << 20];
    loop {
        let n = file
            .read(&mut buf)
            .map_err(|e| format!("reading {}: {e}", p.display()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fake toolchain folder: the two programs a lookup asks for, empty.
    fn fake(dir: &Path) -> PathBuf {
        std::fs::create_dir_all(dir.join("bin")).unwrap();
        std::fs::write(dir.join("bin/c++.exe"), "").unwrap();
        std::fs::write(dir.join("bin/mingw32-make.exe"), "").unwrap();
        dir.to_path_buf()
    }

    fn qb64pe(dir: &Path) -> PathBuf {
        fake(&dir.join("internal/c/c_compiler"));
        dir.to_path_buf()
    }

    fn marked(home: &Path) {
        fake(&home.join(RELEASE));
        std::fs::write(home.join(RELEASE).join(MARKER), "").unwrap();
    }

    fn abs(p: &Path) -> PathBuf {
        std::path::absolute(p).unwrap()
    }

    /// The order of spec `compiler/toolchain`, "Toolchain lookup": each place wins over those after it.
    #[test]
    fn toolchain_lookup_order() {
        let d = tempfile::tempdir().unwrap();
        let (option, variable) = (fake(&d.path().join("o")), fake(&d.path().join("v")));
        let (q, c) = (qb64pe(&d.path().join("q")), qb64pe(&d.path().join("c")));
        let home = d.path().join("home");
        marked(&home);
        let mut p = Lookup {
            option: Some(option.clone()),
            variable: Some(variable.clone()),
            qb64pe: Some(q.clone()),
            home: Some(home.clone()),
            clone: Some(c.clone()),
        };
        let compiler = |q: &Path| abs(&q.join("internal/c/c_compiler"));
        let expect = [
            (abs(&option), Source::Option),
            (abs(&variable), Source::Variable),
            (compiler(&q), Source::Qb64pe),
            (abs(&home.join(RELEASE)), Source::Installed),
            (compiler(&c), Source::Clone),
        ];
        for (folder, source) in expect {
            assert_eq!(find_in(&p).unwrap(), Found { folder, source });
            // Take away the place that won.
            match source {
                Source::Option => p.option = None,
                Source::Variable => p.variable = None,
                Source::Qb64pe => p.qb64pe = None,
                Source::Installed => p.home = None,
                Source::Clone => p.clone = None,
            }
        }
        let err = find_in(&p).unwrap_err();
        assert_eq!(err, missing_message());
        assert!(
            err.contains("qb64rust toolchain install") && err.contains("190 MB"),
            "{err}"
        );
        assert!(err.contains("--toolchain-root") && err.contains(ROOT_ENV), "{err}");
    }

    /// A folder named by the option or the variable must be a toolchain; a QB64pe folder without one, an unmarked
    /// install and a clone without one are passed over.
    #[test]
    fn toolchain_lookup_skips_what_is_not_one() {
        let d = tempfile::tempdir().unwrap();
        let empty = d.path().join("empty");
        std::fs::create_dir_all(&empty).unwrap();
        let err = find_in(&Lookup {
            option: Some(empty.clone()),
            ..Lookup::default()
        })
        .unwrap_err();
        assert!(
            err.contains("--toolchain-root") && err.contains("not a C++ toolchain"),
            "{err}"
        );
        let err = find_in(&Lookup {
            variable: Some(empty.clone()),
            ..Lookup::default()
        })
        .unwrap_err();
        assert!(err.contains(ROOT_ENV), "{err}");
        // An unmarked folder (an interrupted install) is not installed.
        let home = d.path().join("home");
        fake(&home.join(RELEASE));
        assert_eq!(installed(&home), None);
        let c = qb64pe(&d.path().join("c"));
        let p = Lookup {
            qb64pe: Some(empty),
            home: Some(home),
            clone: Some(c.clone()),
            ..Lookup::default()
        };
        assert_eq!(find_in(&p).unwrap().source, Source::Clone);
    }

    // The install, in process (design D7), from a zip of a fake toolchain given by a `file:///` URL: no test
    // reaches the network.

    /// A zip as the release's: one top folder, the release's name, with a toolchain and one of the folders the
    /// install removes. Made by Windows' `tar.exe`. Gives its `file:///` URL, SHA-256 and size.
    fn release_zip(dir: &Path) -> (String, String, u64) {
        let src = dir.join("src");
        let top = fake(&src.join(RELEASE));
        std::fs::write(top.join("bin/lldb-dap.exe"), "x").unwrap();
        std::fs::create_dir_all(top.join("i686-w64-mingw32/lib")).unwrap();
        std::fs::write(top.join("i686-w64-mingw32/lib/a.a"), "x").unwrap();
        std::fs::create_dir_all(top.join("lib/clang/23/lib/linux")).unwrap();
        std::fs::write(top.join("lib/clang/23/lib/linux/b.a"), "x").unwrap();
        std::fs::create_dir_all(top.join("lib/clang/23/lib/windows")).unwrap();
        std::fs::write(top.join("lib/clang/23/lib/windows/c.a"), "x").unwrap();
        let zip = dir.join(format!("{RELEASE}.zip"));
        let o = Command::new(system_tool("tar.exe").unwrap())
            .arg("-a")
            .arg("-cf")
            .arg(&zip)
            .arg("-C")
            .arg(&src)
            .arg(RELEASE)
            .output()
            .unwrap();
        assert!(o.status.success(), "{o:?}");
        let url = format!("file:///{}", zip.to_string_lossy().replace('\\', "/"));
        let size = std::fs::metadata(&zip).unwrap().len();
        (url, sha256_of(&zip).unwrap(), size)
    }

    /// Nothing but the lock file (and, after an install, the toolchain folder) is left in the install root.
    fn left(home: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(home)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    /// Install, then already installed: the second downloads nothing (the zip is gone before it runs).
    #[test]
    fn toolchain_install_then_installed_already() {
        let d = tempfile::tempdir().unwrap();
        let (url, sum, size) = release_zip(d.path());
        let home = d.path().join("home");
        let folder = install_from(&url, &sum, size, &home).unwrap();
        assert_eq!(folder, home.join(RELEASE));
        assert_eq!(installed(&home), Some(folder.clone()));
        assert_eq!(
            std::fs::read(folder.join(MARKER)).unwrap(),
            format!("{RELEASE}\n{sum}\n").into_bytes()
        );
        assert!(folder.join("bin/lldb-dap.exe").is_file());
        assert!(!folder.join("i686-w64-mingw32").exists());
        assert!(!folder.join("lib/clang/23/lib/linux").exists());
        assert!(folder.join("lib/clang/23/lib/windows/c.a").is_file());
        assert_eq!(left(&home), ["install.lock", RELEASE]);
        std::fs::remove_file(d.path().join(format!("{RELEASE}.zip"))).unwrap();
        assert_eq!(install_from(&url, &sum, size, &home).unwrap(), folder);
    }

    /// A wrong checksum: both sums in the message, nothing left.
    #[test]
    fn toolchain_install_wrong_checksum() {
        let d = tempfile::tempdir().unwrap();
        let (url, sum, size) = release_zip(d.path());
        let home = d.path().join("home");
        let wrong = "0".repeat(64);
        let err = install_from(&url, &wrong, size, &home).unwrap_err();
        assert!(err.contains(&sum) && err.contains(&wrong), "{err}");
        assert_eq!(left(&home), ["install.lock"]);
    }

    /// An unmarked folder, left by an interrupted install, is deleted and installed whole.
    #[test]
    fn toolchain_install_replaces_an_interrupted_one() {
        let d = tempfile::tempdir().unwrap();
        let (url, sum, size) = release_zip(d.path());
        let home = d.path().join("home");
        let partial = fake(&home.join(RELEASE));
        std::fs::write(partial.join("half.txt"), "x").unwrap();
        install_from(&url, &sum, size, &home).unwrap();
        assert!(!partial.join("half.txt").exists());
        assert!(installed(&home).is_some());
    }

    /// Two installs at once: one installs, the other waits on the lock and finds it installed.
    #[test]
    fn toolchain_install_twice_at_once() {
        let d = tempfile::tempdir().unwrap();
        let (url, sum, size) = release_zip(d.path());
        let home = d.path().join("home");
        let runs: Vec<_> = (0..2)
            .map(|_| {
                let (url, sum, home) = (url.clone(), sum.clone(), home.clone());
                std::thread::spawn(move || install_from(&url, &sum, size, &home))
            })
            .collect();
        for r in runs {
            assert_eq!(r.join().unwrap().unwrap(), home.join(RELEASE));
        }
        assert_eq!(left(&home), ["install.lock", RELEASE]);
    }

    /// A URL that fails: the URL and `--toolchain-root` named, nothing left.
    #[test]
    fn toolchain_install_url_fails() {
        let d = tempfile::tempdir().unwrap();
        let url = format!("file:///{}/no-such.zip", d.path().to_string_lossy().replace('\\', "/"));
        let home = d.path().join("home");
        let err = install_from(&url, SHA256, 0, &home).unwrap_err();
        assert!(err.contains(&url) && err.contains("--toolchain-root"), "{err}");
        assert_eq!(left(&home), ["install.lock"]);
    }
}
