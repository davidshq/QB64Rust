//! The C++ toolchain a build and the debug adapter use (spec `compiler/toolchain`, design D1 to D4 of
//! `m6-toolchain`): where it is looked for ([`find`]), and its installation by `qb64rust toolchain install`
//! ([`install`]): the pinned upstream release of llvm-mingw, downloaded only when that command runs.
//!
//! On Windows a toolchain folder has llvm-mingw's layout (`bin\c++.exe`, `bin\mingw32-make.exe`, `bin\lldb-dap.exe`,
//! …); QB64pe's `internal\c\c_compiler` is one. On Linux the toolchain is the system's (design D1 of `m7-linux`):
//! `g++`, `make` and `objcopy` found on `PATH`, or a folder with `bin/g++` and `bin/make` given by the option or the
//! variable; nothing is downloaded, and what is missing is named with the distribution's packages
//! ([`packages_message`]).

use crate::build;
use qb64rust_base::Platform;
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
    /// Linux: the system's programs, found on `PATH` (design D1 of `m7-linux`).
    System,
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
            Source::System => "system: the system's g++, make and objcopy, found on PATH",
        }
    }
}

/// A program of the toolchain, named per platform by [`Tool::file_name`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Cxx,
    Cc,
    Ar,
    Objcopy,
    Make,
    Windres,
    Nm,
    LldbDap,
}

impl Tool {
    /// The tools a build runs, stamped for the build cache and the copy's objects (`build.rs`).
    pub const BUILD: [Tool; 5] = [Tool::Ar, Tool::Cc, Tool::Cxx, Tool::Objcopy, Tool::Make];

    pub fn file_name(self, platform: Platform) -> &'static str {
        match platform {
            Platform::Windows => match self {
                Tool::Cxx => "c++.exe",
                Tool::Cc => "gcc.exe",
                Tool::Ar => "ar.exe",
                Tool::Objcopy => "objcopy.exe",
                Tool::Make => "mingw32-make.exe",
                Tool::Windres => "windres.exe",
                Tool::Nm => "nm.exe",
                Tool::LldbDap => "lldb-dap.exe",
            },
            Platform::Linux => match self {
                Tool::Cxx => "g++",
                Tool::Cc => "gcc",
                Tool::Ar => "ar",
                Tool::Objcopy => "objcopy",
                Tool::Make => "make",
                Tool::Windres => "windres",
                Tool::Nm => "nm",
                Tool::LldbDap => "lldb-dap",
            },
        }
    }
}

/// A toolchain folder and where it came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    /// The toolchain folder; for [`Source::System`] the folder of the `g++` found (`/usr/bin`).
    pub folder: PathBuf,
    pub source: Source,
    pub platform: Platform,
    /// Linux: the folders of `PATH`, where a system toolchain's programs and `lldb-dap` are looked for.
    pub path: Vec<PathBuf>,
}

impl Found {
    /// A toolchain folder on a platform, with no `PATH` (the tests, and every Windows source).
    pub fn in_folder(folder: PathBuf, source: Source, platform: Platform) -> Found {
        Found {
            folder,
            source,
            platform,
            path: Vec::new(),
        }
    }

    /// A program of the toolchain: of a system toolchain the first on `PATH` (`<folder>/<name>` when none is), else
    /// `<folder>/bin/<name>`.
    pub fn tool(&self, tool: Tool) -> PathBuf {
        let name = tool.file_name(self.platform);
        if self.source == Source::System {
            return on_path(&self.path, name).unwrap_or_else(|| self.folder.join(name));
        }
        self.folder.join("bin").join(name)
    }
}

/// The first `<dir>/<name>` of `path` that is a file.
fn on_path(path: &[PathBuf], name: &str) -> Option<PathBuf> {
    path.iter().map(|d| d.join(name)).find(|p| p.is_file())
}

/// Whether `dir` holds the programs a build runs on `platform`.
fn is_toolchain(dir: &Path, platform: Platform) -> bool {
    [Tool::Cxx, Tool::Make]
        .iter()
        .all(|t| dir.join("bin").join(t.file_name(platform)).is_file())
}

/// The message of spec `compiler/toolchain`, "No toolchain found", for Windows.
pub fn missing_message() -> String {
    format!(
        "no C++ toolchain found: install it with `qb64rust toolchain install` (llvm-mingw {} from \
         github.com/mstorsjo/llvm-mingw, a {} MB download, about {DISK_MB} MB on disk), or give a toolchain folder \
         with --toolchain-root <dir> or the environment variable {ROOT_ENV}",
        release_tag(),
        SIZE / 1_000_000
    )
}

/// The programs a system toolchain must have on `PATH` (spec `compiler/toolchain`, "Toolchain lookup").
const SYSTEM_TOOLS: [Tool; 3] = [Tool::Cxx, Tool::Make, Tool::Objcopy];

/// The message of "No toolchain found" on Linux: the programs `missing` from `PATH` and the packages of the
/// distribution `os_release` names.
pub fn linux_missing_message(missing: &[&str], os_release: Option<&[u8]>) -> String {
    let names = match missing {
        [] => String::new(),
        [one] => format!("{one} is"),
        [first @ .., last] => format!("{} and {last} are", first.join(", ")),
    };
    format!(
        "no C++ toolchain found: {names} not on PATH. {} Or give a toolchain folder (with bin/g++ and bin/make) with \
         --toolchain-root <dir> or the environment variable {ROOT_ENV}",
        packages_message(os_release)
    )
}

/// A distribution's packages for a build (spec `compiler/toolchain`, "The system packages"; design D1 of
/// `m7-linux`): the `ID`s and `ID_LIKE`s of `/etc/os-release` it covers, the install command and the packages.
/// QB64pe's `setup_lnx.sh` lists, less those no libqb build reads (measured, design "Measured"). `MANUAL.md` holds
/// the same lines; `vscode/scripts/package_download.py` (`manual_problems`) compares them.
pub const PACKAGES: [(&[&str], &str, &[&str]); 3] = [
    (
        &["debian", "ubuntu"],
        "sudo apt-get install",
        &[
            "build-essential",
            "libglu1-mesa-dev",
            "libasound2-dev",
            "libpng-dev",
            "libcurl4-openssl-dev",
            "libx11-dev",
            "libxcursor-dev",
            "libxrandr-dev",
            "libxinerama-dev",
            "libxi-dev",
        ],
    ),
    (
        &["fedora", "rhel", "centos"],
        "sudo dnf install",
        &[
            "gcc-c++",
            "make",
            "mesa-libGLU-devel",
            "alsa-lib-devel",
            "libpng-devel",
            "libcurl-devel",
            "libX11-devel",
            "libXcursor-devel",
            "libXrandr-devel",
            "libXinerama-devel",
            "libXi-devel",
        ],
    ),
    (
        &["arch"],
        "sudo pacman -S",
        &[
            "gcc",
            "make",
            "zlib",
            "curl",
            "glu",
            "alsa-lib",
            "libx11",
            "libxcursor",
            "libxrandr",
            "libxinerama",
            "libxi",
        ],
    ),
];

/// What a build needs where the distribution is none of [`PACKAGES`].
const GENERIC_PACKAGES: &str = "the C++ compiler (g++), make, and the development files of OpenGL, GLU, X11 (Xcursor, \
                                Xrandr, Xinerama, Xi), ALSA, libpng and libcurl";

/// The values of `ID` and `ID_LIKE` in the text of `/etc/os-release`, unquoted, in that order.
fn distribution_ids(os_release: &[u8]) -> Vec<String> {
    let mut ids = Vec::new();
    for key in ["ID=", "ID_LIKE="] {
        for line in os_release.split(|&b| b == b'\n') {
            if let Some(value) = line.strip_prefix(key.as_bytes()) {
                let value: Vec<u8> = value
                    .iter()
                    .copied()
                    .filter(|&b| b != b'"' && b != b'\'' && b != b'\r')
                    .collect();
                for word in value.split(|&b| b == b' ').filter(|w| !w.is_empty()) {
                    ids.push(word.iter().map(|&b| char::from(b.to_ascii_lowercase())).collect());
                }
            }
        }
    }
    ids
}

/// The install line of the distribution `os_release` names (the text of `/etc/os-release`): `ID` first, then each
/// word of `ID_LIKE`. `None` for a distribution none of [`PACKAGES`] covers.
pub fn install_command(os_release: Option<&[u8]>) -> Option<String> {
    distribution(os_release).map(|(_, command, packages)| format!("{command} {}", packages.join(" ")))
}

/// The row of [`PACKAGES`] for the distribution `os_release` names.
fn distribution(
    os_release: Option<&[u8]>,
) -> Option<&'static (&'static [&'static str], &'static str, &'static [&'static str])> {
    let ids = distribution_ids(os_release?);
    ids.iter()
        .find_map(|id| PACKAGES.iter().find(|(names, _, _)| names.contains(&id.as_str())))
}

/// The sentence that names the packages to install (design D1 of `m7-linux`): the install line of the distribution
/// detected, else what is needed.
pub fn packages_message(os_release: Option<&[u8]>) -> String {
    match install_command(os_release) {
        Some(line) => format!("Install the packages with `{line}`."),
        None => format!("Install {GENERIC_PACKAGES}."),
    }
}

/// The text of `/etc/os-release` (else `/usr/lib/os-release`, its fallback), if there is one.
pub fn os_release() -> Option<Vec<u8>> {
    std::fs::read("/etc/os-release")
        .or_else(|_| std::fs::read("/usr/lib/os-release"))
        .ok()
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
    /// Linux: the folders of `PATH`.
    path: Vec<PathBuf>,
    /// Linux: the text of `/etc/os-release`.
    os_release: Option<Vec<u8>>,
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
    (folder.join(MARKER).is_file() && is_toolchain(&folder, Platform::Windows)).then_some(folder)
}

/// The folders of `PATH`, in order.
fn path_folders() -> Vec<PathBuf> {
    std::env::var_os("PATH")
        .map(|p| {
            std::env::split_paths(&p)
                .filter(|d| !d.as_os_str().is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Finds the toolchain for `platform` in the order of spec `compiler/toolchain` (design D4 of `m6-toolchain`, D1 of
/// `m7-linux`): `--toolchain-root` (`toolchain_option`), `QB64RUST_TOOLCHAIN_ROOT`, then on Windows the
/// `internal\c\c_compiler` of a QB64pe folder given by `--qb64pe-root` (`qb64pe_option`) or `QB64RUST_QB64PE_ROOT`,
/// the installed toolchain, the default clone's, and on Linux the system's programs on `PATH`. A folder named by the
/// option or the variable must be a toolchain; the others count only when they are one. A QB64pe folder given must be
/// one, as for a build ([`build::find_root`]), so that `toolchain path` and `dap` agree with it.
pub fn find(
    platform: Platform,
    toolchain_option: Option<&Path>,
    qb64pe_option: Option<&Path>,
) -> Result<Found, String> {
    let qb64pe_var = std::env::var_os(build::ROOT_ENV);
    let qb64pe_given = qb64pe_option.is_some() || qb64pe_var.as_ref().is_some_and(|v| !v.is_empty());
    let qb64pe = if qb64pe_given {
        build::find_root(qb64pe_option)?
    } else {
        None
    };
    let places = match platform {
        Platform::Windows => Lookup {
            option: toolchain_option.map(Path::to_path_buf),
            variable: var(ROOT_ENV),
            qb64pe,
            home: home(),
            clone: (qb64pe_option.is_none() && qb64pe_var.is_none()).then(build::default_clone),
            ..Lookup::default()
        },
        Platform::Linux => Lookup {
            option: toolchain_option.map(Path::to_path_buf),
            variable: var(ROOT_ENV),
            path: path_folders(),
            os_release: os_release(),
            ..Lookup::default()
        },
    };
    find_in(platform, &places)
}

fn find_in(platform: Platform, p: &Lookup) -> Result<Found, String> {
    let found = |folder: &Path, source| Found {
        folder: std::path::absolute(folder).unwrap_or_else(|_| folder.to_path_buf()),
        source,
        platform,
        path: p.path.clone(),
    };
    for (given, source, how) in [
        (&p.option, Source::Option, "--toolchain-root"),
        (&p.variable, Source::Variable, ROOT_ENV),
    ] {
        if let Some(dir) = given {
            if is_toolchain(dir, platform) {
                return Ok(found(dir, source));
            }
            let (cxx, make) = match platform {
                Platform::Windows => ("bin\\c++.exe", "bin\\mingw32-make.exe"),
                Platform::Linux => ("bin/g++", "bin/make"),
            };
            return Err(format!(
                "{} (given by {how}) is not a C++ toolchain folder: it has no {cxx} or {make}",
                dir.display()
            ));
        }
    }
    if platform == Platform::Linux {
        let missing: Vec<&str> = SYSTEM_TOOLS
            .iter()
            .map(|t| t.file_name(platform))
            .filter(|name| on_path(&p.path, name).is_none())
            .collect();
        if !missing.is_empty() {
            return Err(linux_missing_message(&missing, p.os_release.as_deref()));
        }
        let cxx = on_path(&p.path, Tool::Cxx.file_name(platform)).unwrap_or_default();
        let folder = cxx.parent().map(Path::to_path_buf).unwrap_or_default();
        return Ok(found(&folder, Source::System));
    }
    let compiler = |q: &PathBuf| q.join("internal").join("c").join("c_compiler");
    let is_windows_toolchain = |d: &PathBuf| is_toolchain(d, platform);
    if let Some(dir) = p.qb64pe.as_ref().map(compiler).filter(is_windows_toolchain) {
        return Ok(found(&dir, Source::Qb64pe));
    }
    if let Some(dir) = p.home.as_deref().and_then(installed) {
        return Ok(found(&dir, Source::Installed));
    }
    if let Some(dir) = p.clone.as_ref().map(compiler).filter(is_windows_toolchain) {
        return Ok(found(&dir, Source::Clone));
    }
    Err(missing_message())
}

/// The `lldb-dap` the debug adapter starts (spec `editor/debug-adapter`, "Adapter start and backend"; design D5 of
/// `m7-linux`). On Windows `bin\lldb-dap.exe` of the toolchain, whether it exists or not (the adapter names the path
/// when it does not). On Linux `bin/lldb-dap` of a toolchain folder given by the option or the variable, else the
/// `lldb-dap-<n>` on `PATH` with the highest `<n>` of at least [`LLDB_DAP_MINIMUM`] (Ubuntu's `lldb-20` installs
/// `lldb-dap-20`), else `lldb-dap` on `PATH` (of a version not known by its name). A versioned one of the minimum comes
/// before the plain name, which on Ubuntu 24.04 may be the distribution's older default (18). With none, the message
/// names the package and [`LLDB_DAP_MINIMUM`]; with only older versioned ones, it names the newest of them too: one is
/// not started, since 19's ends at the first value it reads (the review of the staged files, 2026-10-29).
pub fn lldb_dap(found: &Found, os_release: Option<&[u8]>) -> Result<PathBuf, String> {
    if found.platform == Platform::Windows {
        return Ok(found.tool(Tool::LldbDap));
    }
    let name = Tool::LldbDap.file_name(found.platform);
    if matches!(found.source, Source::Option | Source::Variable) {
        let p = found.folder.join("bin").join(name);
        if p.is_file() {
            return Ok(p);
        }
    }
    let mut best: Option<(u32, PathBuf)> = None;
    for dir in &found.path {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for e in entries.flatten() {
            if let Some(v) = qb64rust_dap::lldb_dap_version(&e.path())
                && e.path().is_file()
                && best.as_ref().is_none_or(|(b, _)| v > *b)
            {
                best = Some((v, e.path()));
            }
        }
    }
    match best {
        Some((v, p)) if v >= LLDB_DAP_MINIMUM => Ok(p),
        older => on_path(&found.path, name).ok_or_else(|| lldb_dap_missing(older.as_ref(), os_release)),
    }
}

/// `qb64rust toolchain install` on Linux (spec `compiler/toolchain`, "Toolchain install"): downloads nothing; the
/// system toolchain found and the headers of the packages ([`crate::cpp_errors::SYSTEM_HEADERS`]) looked for in
/// `/usr/include` and `/usr/local/include`. The line to print when all are there, else the message of "No toolchain
/// found".
pub fn check_system() -> Result<String, String> {
    let found = find(Platform::Linux, None, None)?;
    let include = [Path::new("/usr/include"), Path::new("/usr/local/include")];
    let missing: Vec<&str> = crate::cpp_errors::SYSTEM_HEADERS
        .iter()
        .copied()
        .filter(|h| !include.iter().any(|d| d.join(h).is_file()))
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "the system toolchain is in {}, but the headers {} are missing. {}",
            found.folder.display(),
            missing.join(", "),
            packages_message(os_release().as_deref())
        ));
    }
    Ok(format!(
        "the system toolchain is used, nothing to install: {}",
        found.folder.display()
    ))
}

/// The oldest `lldb-dap` the adapter's features work with (the adapter's [`qb64rust_dap::LLDB_DAP_MINIMUM`]).
pub const LLDB_DAP_MINIMUM: u32 = qb64rust_dap::LLDB_DAP_MINIMUM;

/// The message of spec `compiler/toolchain`, "The debugger's package". Debian and Ubuntu package each version apart
/// (`lldb-20`), and their plain `lldb` may be older than [`LLDB_DAP_MINIMUM`] (18 on Ubuntu 24.04). `older`: the newest
/// `lldb-dap-<n>` found below the minimum, which is named and not started.
fn lldb_dap_missing(older: Option<&(u32, PathBuf)>, os_release: Option<&[u8]>) -> String {
    let n = LLDB_DAP_MINIMUM;
    let package = match distribution(os_release) {
        Some((_, command, _)) if command.starts_with("sudo apt-get") => format!(
            "install lldb {n} or later with `{command} lldb-{n}` (where the distribution has none, from apt.llvm.org)"
        ),
        Some((_, command, _)) => format!("install lldb {n} or later with `{command} lldb`"),
        None => format!("install lldb {n} or later (its package is lldb on most distributions)"),
    };
    match older {
        Some((v, p)) => format!(
            "lldb-dap too old: {} is lldb-dap {v}, and the debugger needs {n} or later; {package}",
            p.display()
        ),
        None => format!("lldb-dap not found: no lldb-dap or lldb-dap-<n> on PATH; {package}"),
    }
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
    if !out.status.success() || !is_toolchain(&folder, Platform::Windows) {
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
            ..Lookup::default()
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
            assert_eq!(
                find_in(Platform::Windows, &p).unwrap(),
                Found::in_folder(folder, source, Platform::Windows)
            );
            // Take away the place that won.
            match source {
                Source::Option => p.option = None,
                Source::Variable => p.variable = None,
                Source::Qb64pe => p.qb64pe = None,
                Source::Installed => p.home = None,
                Source::Clone => p.clone = None,
                Source::System => unreachable!(),
            }
        }
        let err = find_in(Platform::Windows, &p).unwrap_err();
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
        let err = find_in(
            Platform::Windows,
            &Lookup {
                option: Some(empty.clone()),
                ..Lookup::default()
            },
        )
        .unwrap_err();
        assert!(
            err.contains("--toolchain-root") && err.contains("not a C++ toolchain"),
            "{err}"
        );
        let err = find_in(
            Platform::Windows,
            &Lookup {
                variable: Some(empty.clone()),
                ..Lookup::default()
            },
        )
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
        assert_eq!(find_in(Platform::Windows, &p).unwrap().source, Source::Clone);
    }

    /// Empty files named `names` in `dir`.
    fn files(dir: &Path, names: &[&str]) -> PathBuf {
        std::fs::create_dir_all(dir).unwrap();
        for n in names {
            std::fs::write(dir.join(n), "").unwrap();
        }
        dir.to_path_buf()
    }

    const UBUNTU: &str = "PRETTY_NAME=\"Ubuntu 24.04.4 LTS\"\nNAME=\"Ubuntu\"\nVERSION_ID=\"24.04\"\nID=ubuntu\n\
                          ID_LIKE=debian\n";

    /// The Linux order of spec `compiler/toolchain` (design D1 of `m7-linux`): the option, the variable, then the
    /// system's programs on `PATH`, the first folder that has each; a QB64pe folder, an install and the clone are
    /// Windows' places only.
    #[test]
    fn linux_lookup_order() {
        let d = tempfile::tempdir().unwrap();
        let folder = |n: &str| {
            files(&d.path().join(n).join("bin"), &["g++", "make"])
                .parent()
                .unwrap()
                .to_path_buf()
        };
        let (option, variable) = (folder("o"), folder("v"));
        let first = files(&d.path().join("p1"), &["make"]);
        let second = files(&d.path().join("p2"), &["g++", "make", "objcopy", "ar", "gcc"]);
        let home = d.path().join("home");
        marked(&home);
        let mut p = Lookup {
            option: Some(option.clone()),
            variable: Some(variable.clone()),
            qb64pe: Some(qb64pe(&d.path().join("q"))),
            home: Some(home),
            clone: Some(qb64pe(&d.path().join("c"))),
            path: vec![first.clone(), second.clone()],
            os_release: Some(UBUNTU.as_bytes().to_vec()),
        };
        let got = find_in(Platform::Linux, &p).unwrap();
        assert_eq!((got.folder.clone(), got.source), (abs(&option), Source::Option));
        assert_eq!(got.tool(Tool::Cxx), abs(&option).join("bin/g++"));
        p.option = None;
        assert_eq!(find_in(Platform::Linux, &p).unwrap().source, Source::Variable);
        p.variable = None;
        let got = find_in(Platform::Linux, &p).unwrap();
        assert_eq!((got.folder.clone(), got.source), (abs(&second), Source::System));
        // Each program from the first folder of `PATH` that has it.
        assert_eq!(got.tool(Tool::Make), first.join("make"));
        assert_eq!(got.tool(Tool::Cxx), second.join("g++"));
        assert_eq!(got.tool(Tool::Objcopy), second.join("objcopy"));
        assert_eq!(Source::System.describe().split(':').next(), Some("system"));
    }

    /// No `g++` on Ubuntu: the message names it and the `apt-get` line (spec `compiler/toolchain`, "No g++ on
    /// Ubuntu"); a folder given that is not a toolchain names Linux's files.
    #[test]
    fn linux_no_toolchain() {
        let d = tempfile::tempdir().unwrap();
        let p = Lookup {
            path: vec![files(&d.path().join("p"), &["make"])],
            os_release: Some(UBUNTU.as_bytes().to_vec()),
            ..Lookup::default()
        };
        let err = find_in(Platform::Linux, &p).unwrap_err();
        assert!(err.contains("g++ and objcopy are not on PATH"), "{err}");
        assert!(err.contains("`sudo apt-get install build-essential "), "{err}");
        assert!(err.contains("--toolchain-root") && err.contains(ROOT_ENV), "{err}");
        assert!(!err.contains("qb64rust toolchain install"), "{err}");
        let empty = files(&d.path().join("empty"), &[]);
        let err = find_in(
            Platform::Linux,
            &Lookup {
                option: Some(empty),
                ..Lookup::default()
            },
        )
        .unwrap_err();
        assert!(err.contains("has no bin/g++ or bin/make"), "{err}");
    }

    /// The install line by `/etc/os-release` (design D1 of `m7-linux`): `ID`, then `ID_LIKE`.
    #[test]
    fn packages_by_distribution() {
        let line = |text: &str| install_command(Some(text.as_bytes()));
        let apt = line(UBUNTU).unwrap();
        assert!(apt.starts_with("sudo apt-get install build-essential "), "{apt}");
        assert!(
            apt.contains(" libglu1-mesa-dev ") && apt.contains(" libasound2-dev "),
            "{apt}"
        );
        assert_eq!(line("ID=debian\nVERSION_ID=\"12\"\n"), Some(apt.clone()));
        // Linux Mint names Ubuntu in `ID_LIKE`.
        assert_eq!(line("ID=linuxmint\nID_LIKE=\"ubuntu debian\"\n"), Some(apt.clone()));
        let dnf = line("NAME=\"Fedora Linux\"\nID=fedora\n").unwrap();
        assert!(
            dnf.starts_with("sudo dnf install gcc-c++ make mesa-libGLU-devel "),
            "{dnf}"
        );
        assert_eq!(line("ID=\"rocky\"\nID_LIKE=\"rhel centos fedora\"\n"), Some(dnf));
        let pacman = line("NAME=\"Arch Linux\"\nID=arch\n").unwrap();
        assert!(pacman.starts_with("sudo pacman -S gcc make "), "{pacman}");
        assert_eq!(line("ID=manjaro\nID_LIKE=arch\n"), Some(pacman));
        assert_eq!(line("ID=alpine\n"), None);
        assert_eq!(install_command(None), None);
        let generic = packages_message(Some(b"ID=nixos\n"));
        assert!(generic.contains("g++") && generic.contains("ALSA"), "{generic}");
        assert!(packages_message(Some(UBUNTU.as_bytes())).contains(&apt));
    }

    /// The `lldb-dap` of design D5 of `m7-linux`: a given folder's, else the highest `lldb-dap-<n>` of the minimum,
    /// else `lldb-dap` on `PATH`, else the message with the package (naming the newest older `lldb-dap-<n>`, which is
    /// not started).
    #[test]
    fn linux_lldb_dap() {
        let d = tempfile::tempdir().unwrap();
        let tools = files(&d.path().join("usr"), &["g++", "make", "objcopy"]);
        let versions = files(
            &d.path().join("v"),
            &["lldb-dap-18", "lldb-dap-19", "lldb-dap-9", "lldb-dap-x"],
        );
        let found = |path: Vec<PathBuf>| Found {
            folder: tools.clone(),
            source: Source::System,
            platform: Platform::Linux,
            path,
        };
        let ubuntu = Some(UBUNTU.as_bytes());
        // Only older ones: refused, the newest named with the package.
        let err = lldb_dap(&found(vec![tools.clone(), versions.clone()]), ubuntu).unwrap_err();
        let newest_older = versions.join("lldb-dap-19").display().to_string();
        assert!(
            err.contains("lldb-dap too old")
                && err.contains(&format!("{newest_older} is lldb-dap 19"))
                && err.contains("`sudo apt-get install lldb-20`"),
            "{err}"
        );
        // The plain name, of a version not known, before an older versioned one.
        let plain = files(&d.path().join("plain"), &["lldb-dap"]);
        assert_eq!(
            lldb_dap(&found(vec![versions.clone(), plain.clone()]), ubuntu).unwrap(),
            plain.join("lldb-dap")
        );
        // A versioned one of the minimum before the plain name (Ubuntu 24.04's default `lldb-dap` is 18), the highest.
        let new = files(&d.path().join("new"), &["lldb-dap-20", "lldb-dap-21"]);
        let chosen = lldb_dap(&found(vec![plain.clone(), versions.clone(), new.clone()]), ubuntu).unwrap();
        assert_eq!(chosen, new.join("lldb-dap-21"));
        let err = lldb_dap(&found(vec![tools.clone()]), ubuntu).unwrap_err();
        assert!(
            err.contains("lldb-dap") && err.contains("lldb 20 or later with `sudo apt-get install lldb-20`"),
            "{err}"
        );
        // Elsewhere the plain package, with the same minimum.
        let fedora = Some(b"ID=fedora\n".as_slice());
        let err = lldb_dap(&found(vec![tools.clone()]), fedora).unwrap_err();
        assert!(
            err.contains("install lldb 20 or later with `sudo dnf install lldb`"),
            "{err}"
        );
        // A folder given by the option: its own, else `PATH`'s.
        let given = files(&d.path().join("given/bin"), &["g++", "make", "lldb-dap"]);
        let mut by_option = found(vec![plain.clone()]);
        by_option.source = Source::Option;
        by_option.folder = given.parent().unwrap().to_path_buf();
        assert_eq!(lldb_dap(&by_option, ubuntu).unwrap(), given.join("lldb-dap"));
        std::fs::remove_file(given.join("lldb-dap")).unwrap();
        assert_eq!(lldb_dap(&by_option, ubuntu).unwrap(), plain.join("lldb-dap"));
        // Windows: the toolchain's, as before.
        let win = Found::in_folder(tools.clone(), Source::Installed, Platform::Windows);
        assert_eq!(lldb_dap(&win, None).unwrap(), tools.join("bin").join("lldb-dap.exe"));
    }

    // The install, in process (design D7), from a zip of a fake toolchain given by a `file:///` URL: no test
    // reaches the network.

    /// A zip as the release's: one top folder, the release's name, with a toolchain and one of the folders the
    /// install removes. Made by Windows' `tar.exe`. Gives its `file:///` URL, SHA-256 and size.
    #[cfg(windows)]
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
    #[cfg(windows)]
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
    #[cfg(windows)]
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
    #[cfg(windows)]
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
    #[cfg(windows)]
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
    #[cfg(windows)]
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
    #[cfg(windows)]
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
