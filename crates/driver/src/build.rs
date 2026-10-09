//! Building an executable against the QB64pe reference clone (design D8).
//!
//! The fragments go into `<exe folder>\<exe name>.qb64rust\temp\`, a copy of the clone's `qbx.cpp` into
//! `...\c\` (it includes the fragments as `../temp/*.txt`), and the clone's `Makefile` is run with command-line
//! overrides so that `qbx.o`, the `.sym` file and the executable stay outside the clone. libqb objects are built
//! into the clone's git-ignored folders if missing and reused otherwise, as `qb64pe.exe` does.

use qb64rust_codegen_cpp::Fragments;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const ROOT_ENV: &str = "QB64RUST_QB64PE_ROOT";

/// Finds the reference clone: `--qb64pe-root`, else `QB64RUST_QB64PE_ROOT`, else `..\QB64pe` next to the
/// repository this binary was built from. A candidate counts only if it holds `internal\c\qbx.cpp`.
pub fn find_root(option: Option<&Path>) -> Result<PathBuf, String> {
    let candidates: Vec<PathBuf> = match option {
        Some(p) => vec![p.to_path_buf()],
        None => match std::env::var_os(ROOT_ENV) {
            Some(p) => vec![PathBuf::from(p)],
            None => vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../QB64pe")],
        },
    };
    for c in &candidates {
        if c.join("internal/c/qbx.cpp").is_file() {
            return Ok(std::path::absolute(c).unwrap_or_else(|_| c.clone()));
        }
    }
    Err(format!(
        "QB64pe reference clone not found (looked in {}); give it with --qb64pe-root <dir> or the environment \
         variable {ROOT_ENV}",
        candidates
            .iter()
            .map(|c| c.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

/// The build folder for an executable: `<exe folder>/<exe file name>.qb64rust`, with every byte of the file name
/// other than `A-Z a-z 0-9 . _ -` replaced by `_`. `make` splits paths at spaces and expands `$`, and the shell it
/// runs takes `'`; the compile suite has names such as `<test> - output.exe`, `dollar$sign`, `single'quote'test`.
pub fn build_dir(exe: &Path) -> PathBuf {
    let name: String = exe
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect();
    exe.with_file_name(format!("{name}.qb64rust"))
}

/// Writes the fragments into `<build>/temp/`. Returns that folder.
pub fn write_fragments(build: &Path, fragments: &Fragments) -> std::io::Result<PathBuf> {
    let temp = build.join("temp");
    std::fs::create_dir_all(&temp)?;
    for (name, text) in &fragments.files {
        std::fs::write(temp.join(name), text)?;
    }
    Ok(temp)
}

fn slash(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

/// Builds the executable; `optimize` adds `-O2` as `qb64pe`'s `OptimizeCppProgram` does (`-fwrapv` stays, so
/// overflow still wraps). The executable is linked inside the build folder (whose name has no spaces, see
/// [`build_dir`]) and then moved to `exe`. On failure the build folder is kept and the make output is returned.
pub fn build(root: &Path, build: &Path, exe: &Path, quiet: bool, optimize: bool) -> Result<(), String> {
    let cache = std::env::var_os(CACHE_ENV).map(PathBuf::from);
    if let Some(dir) = &cache {
        let hit = dir.join(format!("{}.exe", build_key(root, build, optimize)));
        if hit.is_file() && std::fs::copy(&hit, exe).is_ok() {
            if !quiet {
                println!("building {} (the same build is in {CACHE_ENV})", exe.display());
            }
            return Ok(());
        }
    }
    build_with_make(root, build, exe, quiet, optimize)?;
    if let Some(dir) = &cache {
        // Keyed again after the build: `make` may have built missing libqb objects. Written under a name of its own
        // and renamed, so a parallel run never copies half a file; a failure only loses the cache entry.
        let key = build_key(root, build, optimize);
        let part = dir.join(format!("{key}.{}.part", std::process::id()));
        if std::fs::create_dir_all(dir).is_ok()
            && std::fs::copy(exe, &part).is_ok()
            && std::fs::rename(&part, dir.join(format!("{key}.exe"))).is_err()
        {
            let _ = std::fs::remove_file(&part);
        }
    }
    Ok(())
}

/// The environment variable naming a folder of built executables to reuse (tier 2 runs, `tools\legacy_tests`): a
/// build whose inputs ([`build_key`]) equal an earlier one's copies that executable instead of running `make`.
/// Unset (the default, and in CI) every build runs `make`.
pub const CACHE_ENV: &str = "QB64RUST_BUILD_CACHE";

/// The key of a build: a 128-bit FNV-1a hash of everything `make` reads that can change between builds — the
/// fragments in `<build>/temp`, the clone's `qbx.cpp` and `Makefile`, the options, and the size and modification
/// time of every file under the clone's `internal/c` (libqb's sources, headers and objects) and of the toolchain's
/// executables. A changed file of the clone or a rebuilt libqb object gives a new key.
fn build_key(root: &Path, build: &Path, optimize: bool) -> String {
    let mut h = Fnv128::new();
    // The version stands for `build_with_make`'s fixed arguments: change it when they change.
    h.write(b"qb64rust build cache 1\0");
    h.write(&[u8::from(optimize)]);
    let mut fragments: Vec<PathBuf> = std::fs::read_dir(build.join("temp"))
        .map(|d| d.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    fragments.retain(|p| p.extension().is_some_and(|e| e == "txt"));
    fragments.sort();
    for p in &fragments {
        h.file(
            p,
            &p.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
    }
    h.file(&root.join("internal/c/qbx.cpp"), "qbx.cpp");
    h.file(&root.join("Makefile"), "Makefile");
    let c = root.join("internal/c");
    let mut stamps = Vec::new();
    stamp_tree(&c, &c, &mut stamps);
    if let Ok(d) = std::fs::read_dir(c.join("c_compiler/bin")) {
        for e in d.flatten() {
            stamp(&c, &e.path(), &mut stamps);
        }
    }
    stamps.sort();
    for s in &stamps {
        h.write(s.as_bytes());
        h.write(b"\0");
    }
    format!("{:032x}", h.0)
}

/// The stamps of the files under `dir`, without the toolchain (`c_compiler`, stamped by its executables) and
/// `temp` folders (`qb64pe.exe`'s scratch space).
fn stamp_tree(base: &Path, dir: &Path, out: &mut Vec<String>) {
    let Ok(d) = std::fs::read_dir(dir) else {
        return;
    };
    for e in d.flatten() {
        let p = e.path();
        let name = e.file_name();
        if p.is_dir() {
            if name != "c_compiler" && name != "temp" {
                stamp_tree(base, &p, out);
            }
        } else {
            stamp(base, &p, out);
        }
    }
}

/// `<path relative to base> <size> <modified, ns since 1970>` of a file.
fn stamp(base: &Path, p: &Path, out: &mut Vec<String>) {
    let Ok(m) = std::fs::metadata(p) else {
        return;
    };
    let modified = m
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    let rel = p.strip_prefix(base).unwrap_or(p).to_string_lossy().replace('\\', "/");
    out.push(format!("{rel} {} {modified}", m.len()));
}

/// FNV-1a with 128 bits (deterministic across Rust versions, unlike `DefaultHasher`).
struct Fnv128(u128);

impl Fnv128 {
    const OFFSET: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
    const PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;

    fn new() -> Self {
        Fnv128(Self::OFFSET)
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= u128::from(b);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    /// A file's name, length and contents (a missing file as "missing").
    fn file(&mut self, p: &Path, name: &str) {
        self.write(name.as_bytes());
        self.write(b"\0");
        match std::fs::read(p) {
            Ok(bytes) => {
                self.write(&(bytes.len() as u64).to_le_bytes());
                self.write(&bytes);
            }
            Err(_) => self.write(b"missing"),
        }
    }
}

fn build_with_make(root: &Path, build: &Path, exe: &Path, quiet: bool, optimize: bool) -> Result<(), String> {
    let c = build.join("c");
    std::fs::create_dir_all(&c).map_err(|e| e.to_string())?;
    std::fs::copy(root.join("internal/c/qbx.cpp"), c.join("qbx.cpp")).map_err(|e| format!("copying qbx.cpp: {e}"))?;
    let _ = std::fs::remove_file(c.join("qbx.o"));
    let _ = std::fs::remove_file(exe);
    let linked = build.join("program.exe");
    let _ = std::fs::remove_file(&linked);
    let make = root.join("internal/c/c_compiler/bin/mingw32-make.exe");
    let mut cmd = Command::new(&make);
    cmd.arg("-C")
        .arg(root)
        .arg("-j3")
        .args(["OS=win", "BITS=64", "DEP_CONSOLE_ONLY=y", "exe"])
        .arg(format!("EXE={}", slash(&linked)))
        .arg(format!("QB_QBX_SRC={}/qbx.cpp", slash(&c)))
        .arg(format!("PATH_INTERNAL_TEMP={}", slash(&build.join("temp"))))
        // -fwrapv: LONG and _INTEGER64 overflow wraps (DIVERGENCES.md D-001, D-002).
        .arg(format!(
            "CXXFLAGS_EXTRA={}-fwrapv -I{}/internal/c",
            if optimize { "-O2 " } else { "" },
            slash(root)
        ));
    if !quiet {
        println!("building {}", exe.display());
    }
    let out = cmd.output().map_err(|e| format!("running {}: {e}", make.display()))?;
    if out.status.success() && linked.is_file() {
        std::fs::rename(&linked, exe).map_err(|e| format!("moving the executable to {}: {e}", exe.display()))
    } else {
        let _ = std::fs::remove_file(exe);
        #[expect(
            clippy::disallowed_methods,
            reason = "the C++ toolchain's messages, shown as they are"
        )]
        let (stdout, stderr) = (
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        Err(format!(
            "C++ build failed (make exit status {:?}); build folder kept: {}\n{stdout}{stderr}",
            out.status.code(),
            build.display(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_folder_names_are_safe_for_make() {
        let dir = build_dir(Path::new("out/single'quote'test-dollar$sign - output.exe"));
        assert_eq!(
            dir,
            Path::new("out/single_quote_test-dollar_sign_-_output.exe.qb64rust")
        );
    }
}
