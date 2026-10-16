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

/// The icon of the executable (design D5 of `m3-sound`; `qb64pe.bas` 13551–13604): a copy of the file as
/// `<build>/temp/icon.ico` and the resource file `icon.rc` that names it, in the old compiler's one line. With the
/// make flag `DEP_ICON_RC=y` the clone's `Makefile` runs `windres` on that file and links the result. A file that
/// is no icon fails there, as in the old compiler's build. The copy is of the bytes alone: `std::fs::copy` would
/// make the copy of a read-only icon read-only, and the next build into a kept folder could not replace it.
pub fn write_icon(build: &Path, icon: &Path) -> std::io::Result<()> {
    let temp = build.join("temp");
    std::fs::create_dir_all(&temp)?;
    std::fs::write(temp.join(ICON_FILES[0]), std::fs::read(icon)?)?;
    std::fs::write(temp.join(ICON_FILES[1]), "0 ICON \"icon.ico\"\n")
}

/// The files [`write_icon`] writes into `<build>/temp`.
const ICON_FILES: [&str; 2] = ["icon.ico", "icon.rc"];

/// The make flag that links the icon `$EXEICON` names.
const ICON_RC_FLAG: &str = "DEP_ICON_RC=y";

fn slash(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

/// The make flags a program's settings ask for: the old compiler's, in its order (`qb64pe.bas` 13706–13721;
/// compared with its builds of four programs, design D3 of `m2-console-screenhide`). A program without a window
/// links the libqb without one; a console links with `-mconsole`; the icon part is a libqb variant of its own, as
/// is the audio part, and so are the image codec, FreeType and the screen image (compared with six builds, design
/// D2 of `m3-images-fonts-mouse`); `DEP_ICON_RC=y` links the icon `$EXEICON` names ([`write_icon`]).
/// The clone's `Makefile` names each variant's object by the flags, so the variants stand side by side.
pub fn make_flags(s: &qb64rust_ir::Settings) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::ImageCodec) {
        flags.push("DEP_IMAGE_CODEC=y");
    }
    if !s.window {
        flags.push("DEP_CONSOLE_ONLY=y");
    }
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::Printer) {
        flags.push("DEP_PRINTER=y");
    }
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::Icon) {
        flags.push("DEP_ICON=y");
    }
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::ScreenImage) {
        flags.push("DEP_SCREENIMAGE=y");
    }
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::Font) {
        flags.push("DEP_FONT=y");
    }
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::DataProcessing) {
        flags.push("DEP_ZLIB=y");
    }
    if s.console {
        flags.push("DEP_CONSOLE=y");
    }
    if s.exe_icon {
        flags.push(ICON_RC_FLAG);
    }
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::Audio) {
        flags.push("DEP_AUDIO_MINIAUDIO=y");
    }
    flags
}

/// What the `-f:` settings and the program ask of a build.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Settings {
    /// The make flags of the program's settings ([`make_flags`]).
    pub flags: Vec<&'static str>,
    /// `OptimizeCppProgram`: adds `-O2` as `qb64pe` does (`-fwrapv` stays, so overflow still wraps).
    pub optimize: bool,
    /// `ExtraLinkerFlags`: added to the end of the link command (`CXXLIBS_EXTRA`), as `qb64pe` does. The test
    /// runner gives the screen-state file this way (`tools/screen`).
    pub extra_link: String,
    /// `GenerateLicenseFile`: the clone's `Makefile` writes the licences of the parts it links, which go beside the
    /// executable as `<exe>.license.txt`, as `qb64pe` does (design D6 of `m3-asserts-debug-zlib`).
    pub license: bool,
    /// The library files the program's `DECLARE LIBRARY` blocks found (`library.rs`): on the link command after
    /// the extra link flags, as `qb64pe` adds them (`qb64pe.bas` 13726, 13741), and in the build's key.
    pub libraries: Vec<PathBuf>,
    /// The headers they found: in the build's key (the fragments name them by path only). The files a header
    /// itself includes are not (design D4 of `m3-declare-library`).
    pub headers: Vec<PathBuf>,
}

/// The link command's extra part: the extra link flags, then each library file, in quotes if its path has a space.
fn libs_extra(settings: &Settings) -> String {
    let mut parts = vec![settings.extra_link.clone()];
    for lib in &settings.libraries {
        let p = slash(lib);
        parts.push(if p.contains(' ') { format!("\"{p}\"") } else { p });
    }
    parts.retain(|p| !p.is_empty());
    parts.join(" ")
}

/// The licence file of an executable: `<exe>.license.txt`, the name the clone's `Makefile` gives it.
pub fn license_path(exe: &Path) -> PathBuf {
    let mut name = exe.as_os_str().to_owned();
    name.push(".license.txt");
    PathBuf::from(name)
}

/// Builds the executable. The executable is linked inside the build folder (whose name has no spaces, see
/// [`build_dir`]) and then moved to `exe`. On failure the build folder is kept and the make output is returned.
pub fn build(root: &Path, build: &Path, exe: &Path, quiet: bool, settings: &Settings) -> Result<(), String> {
    let cache = std::env::var_os(CACHE_ENV).map(PathBuf::from);
    // A build that writes a licence file first removes the one at the output path, so it is never an earlier
    // build's; a build that writes none leaves it, as `qb64pe` does (measured 2026-10-16).
    let license = license_path(exe);
    if settings.license {
        let _ = std::fs::remove_file(&license);
    }
    if let Some(dir) = &cache {
        let hit = dir.join(format!("{}.exe", build_key(root, build, settings)));
        // With a licence file, an entry without its file is a miss.
        let hit_license = license_path(&hit);
        if hit.is_file()
            && (!settings.license || hit_license.is_file())
            && std::fs::copy(&hit, exe).is_ok()
            && (!settings.license || std::fs::copy(&hit_license, &license).is_ok())
        {
            if !quiet {
                println!("building {} (the same build is in {CACHE_ENV})", exe.display());
            }
            return Ok(());
        }
    }
    build_with_make(root, build, exe, quiet, settings)?;
    if let Some(dir) = &cache {
        // Keyed again after the build: `make` may have built missing libqb objects. Written under a name of its own
        // and renamed, so a parallel run never copies half a file; a failure only loses the cache entry.
        let key = build_key(root, build, settings);
        let entry = dir.join(format!("{key}.exe"));
        // The licence file first, so an entry with the setting is whole once its executable is there.
        let mut kept = std::fs::create_dir_all(dir).is_ok();
        if kept && settings.license {
            let part = dir.join(format!("{key}.{}.license.part", std::process::id()));
            kept = std::fs::copy(&license, &part).is_ok() && std::fs::rename(&part, license_path(&entry)).is_ok();
            let _ = std::fs::remove_file(&part);
        }
        let part = dir.join(format!("{key}.{}.part", std::process::id()));
        if kept && std::fs::copy(exe, &part).is_ok() && std::fs::rename(&part, &entry).is_err() {
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
/// fragments in `<build>/temp`, the clone's `qbx.cpp` and `Makefile`, the settings (with the contents of each word
/// of the extra link flags that names a file, and of each library file and header), and the size and modification
/// time of every file under the clone's `internal/c` (libqb's sources, headers and objects) and of the toolchain's
/// executables. A changed file of the clone or a rebuilt libqb object gives a new key.
fn build_key(root: &Path, build: &Path, settings: &Settings) -> String {
    let mut h = Fnv128::new();
    // The version stands for `build_with_make`'s fixed arguments: change it when they change.
    h.write(b"qb64rust build cache 3\0");
    // The program's make flags are in no fragment: `console=1` is the same text with and without a window.
    for flag in &settings.flags {
        h.write(flag.as_bytes());
        h.write(b"\0");
    }
    h.write(b"\0");
    h.write(&[u8::from(settings.optimize)]);
    h.write(&[u8::from(settings.license)]);
    h.write(settings.extra_link.as_bytes());
    h.write(b"\0");
    for word in settings.extra_link.split_whitespace() {
        // `make` runs in the root (`-C`), so that is where a relative name is looked for.
        let p = root.join(word);
        if p.is_file() {
            h.file(&p, word);
        }
    }
    // The libraries and headers of `DECLARE LIBRARY`, by path and contents (nothing for a program without them, so
    // its key is unchanged).
    for p in settings.libraries.iter().chain(&settings.headers) {
        h.write(b"library\0");
        h.file(p, &p.to_string_lossy());
    }
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
    // The icon of the executable and its resource file, for a program that has one ([`write_icon`]). Only then
    // does `make` read them: a kept build folder may hold those of an earlier build.
    if settings.flags.contains(&ICON_RC_FLAG) {
        for name in ICON_FILES {
            let p = build.join("temp").join(name);
            if p.is_file() {
                h.file(&p, name);
            }
        }
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

fn build_with_make(root: &Path, build: &Path, exe: &Path, quiet: bool, settings: &Settings) -> Result<(), String> {
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
        .args(["OS=win", "BITS=64"])
        .args(&settings.flags)
        .arg("exe")
        .arg(format!("EXE={}", slash(&linked)))
        .arg(format!("QB_QBX_SRC={}/qbx.cpp", slash(&c)))
        .arg(format!("PATH_INTERNAL_TEMP={}", slash(&build.join("temp"))))
        // -fwrapv: LONG and _INTEGER64 overflow wraps (DIVERGENCES.md D-001, D-002).
        .arg(format!(
            "CXXFLAGS_EXTRA={}-fwrapv -I{}/internal/c",
            if settings.optimize { "-O2 " } else { "" },
            slash(root)
        ));
    let libs = libs_extra(settings);
    if !libs.is_empty() {
        cmd.arg(format!("CXXLIBS_EXTRA={libs}"));
    }
    // After the dependencies, as `qb64pe` adds it (`qb64pe.bas` 13772); the `Makefile` names the file after `EXE`.
    let linked_license = license_path(&linked);
    if settings.license {
        let _ = std::fs::remove_file(&linked_license);
        cmd.arg("GENERATE_LICENSE=y");
    }
    if !quiet {
        println!("building {}", exe.display());
    }
    let out = cmd.output().map_err(|e| format!("running {}: {e}", make.display()))?;
    if out.status.success() && linked.is_file() && (!settings.license || linked_license.is_file()) {
        if settings.license {
            let to = license_path(exe);
            std::fs::rename(&linked_license, &to)
                .map_err(|e| format!("moving the licence file to {}: {e}", to.display()))?;
        }
        std::fs::rename(&linked, exe).map_err(|e| {
            let _ = std::fs::remove_file(license_path(exe));
            format!("moving the executable to {}: {e}", exe.display())
        })
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

    /// The key follows the extra link flags and the contents of a file they name (spec `compiler/cli`, scenario
    /// "The extra file changes").
    #[test]
    fn build_key_follows_the_extra_link_file() {
        let d = tempfile::tempdir().unwrap();
        let file = d.path().join("extra.cpp");
        std::fs::write(&file, "int a;").unwrap();
        let key = |extra: &str| {
            let settings = Settings {
                extra_link: extra.to_string(),
                ..Settings::default()
            };
            build_key(d.path(), d.path(), &settings)
        };
        let name = slash(&file);
        let (none, first) = (key(""), key(&name));
        assert_ne!(none, first);
        assert_eq!(first, key(&name));
        std::fs::write(&file, "int b;").unwrap();
        assert_ne!(first, key(&name));
        // A relative name is the root's file, as `make -C <root>` reads it.
        let relative = key("extra.cpp");
        std::fs::write(&file, "int c;").unwrap();
        assert_ne!(relative, key("extra.cpp"));
        assert_ne!(key("-lm"), key("-lz"));
    }

    fn program_settings(console: bool, window: bool, icon: bool) -> qb64rust_ir::Settings {
        qb64rust_ir::Settings {
            console,
            window,
            window_hidden: false,
            asserts: false,
            runtime_parts: icon.then_some(qb64rust_ir::RuntimePart::Icon).into_iter().collect(),
            exe_icon: false,
        }
    }

    /// The flags of the four programs whose old-compiler builds were read (design D3 of `m2-console-screenhide`,
    /// its table): `$CONSOLE:ONLY`, alone or with `$CONSOLE` (one set of settings); `$CONSOLE`; `$CONSOLE` with
    /// `_ICON`.
    #[test]
    fn make_flags_are_the_old_compilers() {
        assert_eq!(
            make_flags(&program_settings(true, false, false)),
            ["DEP_CONSOLE_ONLY=y", "DEP_CONSOLE=y"]
        );
        assert_eq!(make_flags(&program_settings(true, true, false)), ["DEP_CONSOLE=y"]);
        assert_eq!(
            make_flags(&program_settings(true, true, true)),
            ["DEP_ICON=y", "DEP_CONSOLE=y"]
        );
    }

    /// A program with neither `$CONSOLE` nor `$CONSOLE:ONLY`, with or without `$SCREENHIDE`: no flag, so the
    /// clone's `Makefile` links the libqb with the window and `-mwindows` (design D1 of `m3-window-output`, its
    /// two rows; `$SCREENHIDE` is a global of the program, not a flag).
    #[test]
    fn a_program_without_a_console_has_no_make_flag() {
        let shown = program_settings(false, true, false);
        let hidden = qb64rust_ir::Settings {
            window_hidden: true,
            ..program_settings(false, true, false)
        };
        assert!(make_flags(&shown).is_empty());
        assert!(make_flags(&hidden).is_empty());
    }

    /// Two builds that differ only in the program's flags do not share an executable (spec `compiler/pipeline`,
    /// "The build follows the program"): the fragments of a program with and without a window can be equal.
    #[test]
    fn build_key_follows_the_program_flags() {
        let d = tempfile::tempdir().unwrap();
        let key = |s: &qb64rust_ir::Settings| {
            let settings = Settings {
                flags: make_flags(s),
                ..Settings::default()
            };
            build_key(d.path(), d.path(), &settings)
        };
        let only = key(&program_settings(true, false, false));
        let window = key(&program_settings(true, true, false));
        let icon = key(&program_settings(true, true, true));
        assert_ne!(only, window);
        assert_ne!(window, icon);
        assert_eq!(window, key(&program_settings(true, true, false)));
        // A program with sound is built against another libqb than the same program without.
        assert_ne!(only, key(&with_audio(program_settings(true, false, false))));
    }

    /// The icon files of a kept build folder: those of an earlier build are in no key of a program without
    /// `$EXEICON`; a read-only icon is copied without that attribute, so the next build replaces the copy.
    #[test]
    fn icon_files_of_a_kept_build_folder() {
        let d = tempfile::tempdir().unwrap();
        let build = d.path().join("build");
        let key = |flags: Vec<&'static str>| {
            let settings = Settings {
                flags,
                ..Settings::default()
            };
            build_key(d.path(), &build, &settings)
        };
        let (plain, with_icon) = (key(vec![]), key(vec![ICON_RC_FLAG]));
        let icon = d.path().join("game.ico");
        std::fs::write(&icon, "first").unwrap();
        let mut read_only = std::fs::metadata(&icon).unwrap().permissions();
        read_only.set_readonly(true);
        std::fs::set_permissions(&icon, read_only).unwrap();
        write_icon(&build, &icon).unwrap();
        write_icon(&build, &icon).unwrap();
        assert_eq!(std::fs::read(build.join("temp/icon.ico")).unwrap(), b"first");
        assert_eq!(plain, key(vec![]));
        assert_ne!(with_icon, key(vec![ICON_RC_FLAG]));
        // So that the folder can be removed.
        let mut writable = std::fs::metadata(&icon).unwrap().permissions();
        #[expect(clippy::permissions_set_readonly_false, reason = "a scratch file of the test")]
        writable.set_readonly(false);
        std::fs::set_permissions(&icon, writable).unwrap();
    }

    /// `$EXEICON`: the flag of the icon resource follows the console's and comes before the audio part's
    /// (`qb64pe.bas` 13717–13721; design D5 of `m3-sound`). The statement the line becomes is `_ICON`, so the icon
    /// part of the runtime is there too.
    #[test]
    fn make_flags_of_a_program_with_an_exe_icon() {
        let icon = |mut s: qb64rust_ir::Settings| {
            s.exe_icon = true;
            s
        };
        assert_eq!(
            make_flags(&icon(program_settings(true, false, true))),
            ["DEP_CONSOLE_ONLY=y", "DEP_ICON=y", "DEP_CONSOLE=y", "DEP_ICON_RC=y"]
        );
        assert_eq!(
            make_flags(&icon(with_audio(program_settings(false, true, true)))),
            ["DEP_ICON=y", "DEP_ICON_RC=y", "DEP_AUDIO_MINIAUDIO=y"]
        );
    }

    /// The icon's bytes are part of the key: a changed icon is another executable (spec `compiler/pipeline`, "A
    /// changed icon"). That a build without an icon has the key it had before there were icons is
    /// `icon_files_of_a_kept_build_folder`'s to check.
    #[test]
    fn build_key_follows_the_icon() {
        let d = tempfile::tempdir().unwrap();
        let icon = d.path().join("game.ico");
        let with_icon = Settings {
            flags: vec![ICON_RC_FLAG],
            ..Settings::default()
        };
        let key = || build_key(d.path(), d.path(), &with_icon);
        let none = key();
        std::fs::write(&icon, [0, 0, 1, 0, 1]).unwrap();
        write_icon(d.path(), &icon).unwrap();
        let first = key();
        assert_ne!(none, first);
        assert_eq!(first, key());
        std::fs::write(&icon, [0, 0, 1, 0, 2]).unwrap();
        write_icon(d.path(), &icon).unwrap();
        assert_ne!(first, key());
        assert_eq!(
            std::fs::read(d.path().join("temp/icon.rc")).unwrap(),
            b"0 ICON \"icon.ico\"\n"
        );
    }

    /// The bytes of a library file and of a header are part of the key (spec `compiler/pipeline`, "A changed
    /// header"); a program without them keeps its key.
    #[test]
    fn build_key_follows_the_libraries_and_headers() {
        let d = tempfile::tempdir().unwrap();
        let (lib, header) = (d.path().join("v.a"), d.path().join("v.h"));
        std::fs::write(&lib, "a").unwrap();
        std::fs::write(&header, "int f(void);").unwrap();
        let none = build_key(d.path(), d.path(), &Settings::default());
        let with = Settings {
            libraries: vec![lib.clone()],
            headers: vec![header.clone()],
            ..Settings::default()
        };
        let key = || build_key(d.path(), d.path(), &with);
        let first = key();
        assert_ne!(none, first);
        assert_eq!(first, key());
        std::fs::write(&header, "int f(int);").unwrap();
        let second = key();
        assert_ne!(first, second);
        std::fs::write(&lib, "b").unwrap();
        assert_ne!(second, key());
    }

    /// The libraries come after the extra link flags, as `qb64pe` adds them; a path with a space is quoted.
    #[test]
    fn libraries_on_the_link_command() {
        let s = Settings {
            extra_link: "screen_state.o".to_string(),
            libraries: vec![PathBuf::from("libs\\v.a"), PathBuf::from("my libs\\w.o")],
            ..Settings::default()
        };
        assert_eq!(libs_extra(&s), "screen_state.o libs/v.a \"my libs/w.o\"");
        assert_eq!(libs_extra(&Settings::default()), "");
    }

    fn with_audio(mut s: qb64rust_ir::Settings) -> qb64rust_ir::Settings {
        s.runtime_parts.insert(qb64rust_ir::RuntimePart::Audio);
        s
    }

    /// `LPRINT` needs the printer part; its flag stands after `DEP_CONSOLE_ONLY=y` and before `DEP_ICON=y`, as
    /// the old compiler orders them (`qb64pe.bas` 13707–13711; design D8 of `m3-console-files`).
    #[test]
    fn the_printer_flag() {
        let mut s = program_settings(true, false, true);
        s.runtime_parts.insert(qb64rust_ir::RuntimePart::Printer);
        assert_eq!(
            make_flags(&s),
            ["DEP_CONSOLE_ONLY=y", "DEP_PRINTER=y", "DEP_ICON=y", "DEP_CONSOLE=y"]
        );
    }

    /// `SOUND` and `PLAY` need libqb's audio part; its flag is the old compiler's last (`qb64pe.bas` 13721; design
    /// D3 of `m3-sound`). A program with only `BEEP` has no runtime part and so the flags of any program.
    #[test]
    fn make_flags_of_a_program_with_sound() {
        assert_eq!(
            make_flags(&with_audio(program_settings(false, true, false))),
            ["DEP_AUDIO_MINIAUDIO=y"]
        );
        assert_eq!(
            make_flags(&with_audio(program_settings(true, false, false))),
            ["DEP_CONSOLE_ONLY=y", "DEP_CONSOLE=y", "DEP_AUDIO_MINIAUDIO=y"]
        );
        assert_eq!(
            make_flags(&with_audio(program_settings(true, true, true))),
            ["DEP_ICON=y", "DEP_CONSOLE=y", "DEP_AUDIO_MINIAUDIO=y"]
        );
        assert!(make_flags(&program_settings(false, true, false)).is_empty());
    }

    /// The image codec, FreeType and the screen image, in the old compiler's order (`qb64pe.bas` 13707–13713),
    /// as the six builds of task 1.2 of `m3-images-fonts-mouse` gave them: `_LOADIMAGE`, `_LOADFONT`,
    /// `_SCREENIMAGE` (which names only its own part: the clone's `Makefile` adds the codec), and all three with
    /// `$CONSOLE:ONLY`.
    #[test]
    fn make_flags_of_images_and_fonts() {
        use qb64rust_ir::RuntimePart::{Font, ImageCodec, ScreenImage};
        let with = |console: bool, window: bool, parts: &[qb64rust_ir::RuntimePart]| {
            let mut s = program_settings(console, window, false);
            s.runtime_parts.extend(parts.iter().copied());
            make_flags(&s)
        };
        assert_eq!(with(false, true, &[ImageCodec]), ["DEP_IMAGE_CODEC=y"]);
        assert_eq!(with(false, true, &[Font]), ["DEP_FONT=y"]);
        assert_eq!(with(false, true, &[ScreenImage]), ["DEP_SCREENIMAGE=y"]);
        assert_eq!(
            with(true, false, &[Font, ScreenImage, ImageCodec]),
            [
                "DEP_IMAGE_CODEC=y",
                "DEP_CONSOLE_ONLY=y",
                "DEP_SCREENIMAGE=y",
                "DEP_FONT=y",
                "DEP_CONSOLE=y"
            ]
        );
        // Beside the printer and the icon.
        let mut s = program_settings(true, true, true);
        s.runtime_parts
            .extend([ImageCodec, Font, qb64rust_ir::RuntimePart::Printer]);
        assert_eq!(
            make_flags(&s),
            [
                "DEP_IMAGE_CODEC=y",
                "DEP_PRINTER=y",
                "DEP_ICON=y",
                "DEP_FONT=y",
                "DEP_CONSOLE=y"
            ]
        );
    }

    /// The data-processing part stands after `DEP_FONT=y` and before `DEP_CONSOLE=y`, `DEP_ICON_RC=y` and the
    /// audio, as the old compiler orders them (`qb64pe.bas` 13706–13721; design D4 of `m3-asserts-debug-zlib`).
    #[test]
    fn make_flags_of_the_data_processing_part() {
        use qb64rust_ir::RuntimePart::{Audio, DataProcessing, Font};
        let mut s = program_settings(true, false, false);
        s.runtime_parts.insert(DataProcessing);
        assert_eq!(make_flags(&s), ["DEP_CONSOLE_ONLY=y", "DEP_ZLIB=y", "DEP_CONSOLE=y"]);
        let mut s = program_settings(true, true, true);
        s.exe_icon = true;
        s.runtime_parts.extend([DataProcessing, Font, Audio]);
        assert_eq!(
            make_flags(&s),
            [
                "DEP_ICON=y",
                "DEP_FONT=y",
                "DEP_ZLIB=y",
                "DEP_CONSOLE=y",
                "DEP_ICON_RC=y",
                "DEP_AUDIO_MINIAUDIO=y"
            ]
        );
        let mut s = program_settings(false, true, false);
        s.runtime_parts.insert(DataProcessing);
        assert_eq!(make_flags(&s), ["DEP_ZLIB=y"]);
    }
}
