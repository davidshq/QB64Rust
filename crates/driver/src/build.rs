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
