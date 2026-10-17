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

/// The resources of the executable (design D5 of `m3-sound`, D4 of `m3-versioninfo`; `qb64pe.bas` 13551–13640):
/// for an icon a copy of the file as `<build>/temp/icon.ico`, and the files of [`version_files`]. With the make flag
/// `DEP_ICON_RC=y` the clone's `Makefile` runs `windres` on `icon.rc` and links the result. A file that is no icon
/// fails there, as in the old compiler's build. The copy is of the bytes alone: `std::fs::copy` would make the copy
/// of a read-only icon read-only, and the next build into a kept folder could not replace it. `exe_name` is the
/// executable's file name (`game.exe`), which the manifest's file name follows. The resource files an earlier build
/// left in a kept folder are removed first, so the cache key ([`build_key`]) hashes this build's alone.
pub fn write_resources(
    build: &Path,
    icon: Option<&Path>,
    version: Option<&qb64rust_ir::VersionInfo>,
    exe_name: &str,
) -> Result<(), String> {
    if icon.is_none() && version.is_none() {
        return Ok(());
    }
    let temp = build.join("temp");
    let writing = |e: std::io::Error| format!("writing the resources into {}: {e}", temp.display());
    std::fs::create_dir_all(&temp).map_err(writing)?;
    for entry in std::fs::read_dir(&temp).map_err(writing)?.flatten() {
        if is_resource_file(&entry.file_name().to_string_lossy()) {
            std::fs::remove_file(entry.path()).map_err(writing)?;
        }
    }
    if let Some(icon) = icon {
        let bytes = std::fs::read(icon).map_err(|e| format!("copying {}: {e}", icon.display()))?;
        std::fs::write(temp.join("icon.ico"), bytes).map_err(|e| format!("copying {}: {e}", icon.display()))?;
    }
    for (name, bytes) in version_files(icon.is_some(), version, exe_name) {
        std::fs::write(temp.join(name), bytes).map_err(writing)?;
    }
    Ok(())
}

/// The resource script `icon.rc` and, with a version resource, the header and the manifest it includes, as the
/// old compiler writes them (`qb64pe.bas` 13561–13640; measured, `verification\v40_rc\`): with no version
/// resource the icon line alone, with a line end of its own (`windres` reads it as the old compiler's); with one
/// CRLF lines, the icon line first when there is an icon, every value's bytes as written. The manifest is named
/// `<exe_name>.manifest`.
pub fn version_files(icon: bool, version: Option<&qb64rust_ir::VersionInfo>, exe_name: &str) -> Vec<(String, Vec<u8>)> {
    let Some(v) = version else {
        return if icon {
            vec![("icon.rc".to_string(), b"0 ICON \"icon.ico\"\n".to_vec())]
        } else {
            Vec::new()
        };
    };
    fn lines(parts: &[&[u8]]) -> Vec<u8> {
        let mut out = Vec::new();
        for p in parts {
            out.extend_from_slice(p);
            out.extend_from_slice(b"\r\n");
        }
        out
    }
    let manifest_name = format!("{exe_name}.manifest");
    let mut rc: Vec<Vec<u8>> = Vec::new();
    if icon {
        rc.push(b"0 ICON \"icon.ico\"".to_vec());
    }
    rc.push(Vec::new());
    rc.push(b"#include \"manifest.h\"".to_vec());
    rc.push(Vec::new());
    rc.push(format!("CREATEPROCESS_MANIFEST_RESOURCE_ID RT_MANIFEST \"{manifest_name}\"").into_bytes());
    rc.push(Vec::new());
    rc.push(b"1 VERSIONINFO".to_vec());
    if !v.file_version.is_empty() {
        rc.push([b"FILEVERSION     ".as_slice(), &v.file_version].concat());
    }
    if !v.product_version.is_empty() {
        rc.push([b"PRODUCTVERSION  ".as_slice(), &v.product_version].concat());
    }
    for line in [
        "BEGIN",
        "    BLOCK \"StringFileInfo\"",
        "    BEGIN",
        "        BLOCK \"040904E4\"",
        "        BEGIN",
    ] {
        rc.push(line.as_bytes().to_vec());
    }
    for (key, value) in qb64rust_ir::VersionInfo::STRING_KEYS.iter().zip(&v.strings) {
        let head = format!("            VALUE \"{key}\",\"");
        rc.push([head.as_bytes(), value, b"\\0\""].concat());
    }
    for line in [
        "        END",
        "    END",
        "    BLOCK \"VarFileInfo\"",
        "    BEGIN",
        "            VALUE \"Translation\", 0x409, 0x04E4",
        "    END",
        "END",
    ] {
        rc.push(line.as_bytes().to_vec());
    }
    let rc: Vec<&[u8]> = rc.iter().map(Vec::as_slice).collect();

    let s = |i: usize| v.strings[i].as_slice();
    use qb64rust_ir::VersionInfo as V;
    let identity = [s(V::COMPANY_NAME), b".", s(V::PRODUCT_NAME), b".", s(V::PRODUCT_NAME)].concat();
    let name_line = [b"    name=\"".as_slice(), &identity, b"\""].concat();
    let description = [b"<description>".as_slice(), s(V::FILE_DESCRIPTION), b"</description>"].concat();
    let manifest: [&[u8]; 22] = [
        b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        b"<assembly xmlns=\"urn:schemas-microsoft-com:asm.v1\" manifestVersion=\"1.0\">",
        b"<assemblyIdentity",
        b"    version=\"1.0.0.0\"",
        b"    processorArchitecture=\"*\"",
        &name_line,
        b"    type=\"win32\"",
        b"/>",
        &description,
        b"<dependency>",
        b"    <dependentAssembly>",
        b"        <assemblyIdentity",
        b"            type=\"win32\"",
        b"            name=\"Microsoft.Windows.Common-Controls\"",
        b"            version=\"6.0.0.0\"",
        b"            processorArchitecture=\"*\"",
        b"            publicKeyToken=\"6595b64144ccf1df\"",
        b"            language=\"*\"",
        b"        />",
        b"    </dependentAssembly>",
        b"</dependency>",
        b"</assembly>",
    ];
    let header: [&[u8]; 11] = [
        b"#ifndef RESOURCE_H",
        b"#define   RESOURCE_H",
        b"#ifdef    __cplusplus",
        b"extern \"C\" {",
        b"#endif",
        b"#ifdef    __cplusplus",
        b"}",
        b"#endif",
        b"#endif    /* RESOURCE_H */",
        b"#define CREATEPROCESS_MANIFEST_RESOURCE_ID 1 /*Defined manifest file*/",
        b"#define RT_MANIFEST                       24",
    ];
    vec![
        (manifest_name, lines(&manifest)),
        ("manifest.h".to_string(), lines(&header)),
        ("icon.rc".to_string(), lines(&rc)),
    ]
}

/// The files of [`write_resources`] in `<build>/temp` that the build reads: the icon, the resource script, the
/// header and the manifest (named after the executable).
fn is_resource_file(name: &str) -> bool {
    matches!(name, "icon.ico" | "icon.rc" | "manifest.h") || name.ends_with(".manifest")
}

/// The make flag that links the resource script of `$EXEICON` and `$VERSIONINFO`.
const ICON_RC_FLAG: &str = "DEP_ICON_RC=y";

/// The make flag that compiles and links `<build>/temp/embedded.cpp` ([`write_embedded`]).
const EMBED_FLAG: &str = "DEP_EMBED=y";

/// The file of [`write_embedded`] in `<build>/temp`.
const EMBEDDED_CPP: &str = "embedded.cpp";

/// The files `$EMBED` builds into the executable (design D3 and D4 of `m3-network-embed`; `qb64pe.bas`
/// 13650–13690, 28096–28180): `<build>/temp/embedded.cpp` holds each used file (`files`: its handle and its bytes)
/// as an array of its bytes, stored as they are, and `func__embedded`, which `qbx.cpp` declares: a new string of the
/// bytes of the file whose handle equals its argument, or an empty one, as the old compiler's function ends.
/// Without a used file the file an earlier build left in a kept folder is removed, and nothing is written.
pub fn write_embedded(build: &Path, files: &[(String, Vec<u8>)]) -> Result<(), String> {
    let temp = build.join("temp");
    let path = temp.join(EMBEDDED_CPP);
    let writing = |e: std::io::Error| format!("writing {}: {e}", path.display());
    if files.is_empty() {
        return match std::fs::remove_file(&path) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(writing(e)),
            _ => Ok(()),
        };
    }
    std::fs::create_dir_all(&temp).map_err(writing)?;
    std::fs::write(&path, embedded_cpp(files)).map_err(writing)
}

/// The text of `embedded.cpp` ([`write_embedded`]). A handle is letters and digits (`sema` checked it), so it names
/// a C++ array; two handles that differ in letter case are two arrays.
pub fn embedded_cpp(files: &[(String, Vec<u8>)]) -> String {
    use std::fmt::Write as _;
    let mut out = String::from("#include <stdint.h>\n#include <string.h>\n#include \"common.h\"\n\n");
    for (handle, bytes) in files.iter().filter(|(_, b)| !b.is_empty()) {
        writeln!(out, "static const uint8_t embed_{handle}[] = {{").unwrap();
        for line in bytes.chunks(16) {
            let hex: Vec<String> = line.iter().map(|b| format!("0x{b:02X}")).collect();
            writeln!(out, "    {},", hex.join(",")).unwrap();
        }
        out.push_str("};\n\n");
    }
    out.push_str("qbs *func__embedded(qbs *handle)\n{\n");
    for (handle, bytes) in files {
        let len = bytes.len();
        let copy = if len == 0 {
            String::new()
        } else {
            format!("memcpy(data->chr, embed_{handle}, {len}); ")
        };
        writeln!(
            out,
            "    if (qbs_equal(handle, qbs_new_txt(\"{handle}\"))) {{qbs *data = qbs_new({len}, 1); {copy}return data;}}"
        )
        .unwrap();
    }
    out.push_str("    return qbs_new_txt(\"\");\n}\n");
    out
}

fn slash(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

/// The make flags a program's settings ask for: the old compiler's, in its order (`qb64pe.bas` 13706–13721;
/// compared with its builds of four programs, design D3 of `m2-console-screenhide`). A program without a window
/// links the libqb without one; a console links with `-mconsole`; the icon part is a libqb variant of its own, as
/// is the audio part, and so are the image codec, FreeType and the screen image (compared with six builds, design
/// D2 of `m3-images-fonts-mouse`); `DEP_ICON_RC=y` links the resources of `$EXEICON` and `$VERSIONINFO`
/// ([`write_resources`]).
/// The clone's `Makefile` names each variant's object by the flags, so the variants stand side by side.
pub fn make_flags(s: &qb64rust_ir::Settings) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::ImageCodec) {
        flags.push("DEP_IMAGE_CODEC=y");
    }
    if !s.window {
        flags.push("DEP_CONSOLE_ONLY=y");
    }
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::Sockets) {
        flags.extend(["DEP_SOCKETS=y", "DEP_HTTP=y"]);
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
    if s.runtime_parts.contains(&qb64rust_ir::RuntimePart::Embed) {
        flags.push(EMBED_FLAG);
    }
    if s.console {
        flags.push("DEP_CONSOLE=y");
    }
    if s.exe_icon || s.version_info.is_some() {
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
/// time of every file under the clone's `internal/c` that no build writes (libqb's sources and headers,
/// [`written_by_builds`]) and of the toolchain's executables. A changed source or header of the clone gives a new
/// key; a libqb object rebuilt from them, or the `qbx.o` a `qb64pe.exe` compile leaves in the clone, does not
/// (`study\34` §2.2): an object is made from the sources and the make flags, both in the key, and from the
/// lasting compiler settings of `qb64pe.exe` when it builds the object ([`compiler_settings`]), also in the key.
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
    // The resource files of the executable, for a program that has them ([`write_resources`], which removes those
    // an earlier build left first). Only then does `make` read them: a kept build folder of a program without them
    // may hold an earlier build's. A value of the version resource is in `icon.rc` or the manifest, so a changed
    // one changes the key.
    if settings.flags.contains(&ICON_RC_FLAG) {
        let mut names: Vec<String> = std::fs::read_dir(build.join("temp"))
            .map(|d| {
                d.flatten()
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .filter(|n| is_resource_file(n))
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        for name in &names {
            let p = build.join("temp").join(name);
            if p.is_file() {
                h.file(&p, name);
            }
        }
    }
    // The embedded files ([`write_embedded`]), which `make` reads only with the flag; a changed file changes it.
    if settings.flags.contains(&EMBED_FLAG) {
        let p = build.join("temp").join(EMBEDDED_CPP);
        if p.is_file() {
            h.file(&p, EMBEDDED_CPP);
        }
    }
    h.file(&root.join("internal/c/qbx.cpp"), "qbx.cpp");
    h.file(&root.join("Makefile"), "Makefile");
    h.write(b"config\0");
    h.write(&compiler_settings(&root.join("settings/config.ini")));
    h.write(b"\0");
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

/// The stamps of the files under `dir`, without the toolchain (`c_compiler`, stamped by its executables), the
/// `temp` folders (`qb64pe.exe`'s scratch space) and the files a build writes ([`written_by_builds`]).
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
        } else if !written_by_builds(dir == base, &name.to_string_lossy()) {
            stamp(base, &p, out);
        }
    }
}

/// True for a file of the clone's `internal/c` outside `c_compiler` and `temp` that a build of either compiler
/// writes: the libqb objects and archives (`.o`, `.a`), make's `.tmp` files, and in `internal/c` itself the
/// `qbx<N>.cpp` and `qbx*.o` of the program `qb64pe.exe` builds. `top`: the file is in `internal/c` itself. The
/// same files as `written_by_builds` of `tools\legacy_tests\run_legacy_tests.py`, so the two stamps agree.
fn written_by_builds(top: bool, name: &str) -> bool {
    if name.ends_with(".o") || name.ends_with(".a") || name.ends_with(".tmp") {
        return true;
    }
    top && name.starts_with("qbx") && name != "qbx.cpp"
}

/// The `[COMPILER SETTINGS]` section of the clone's `settings/config.ini` (empty when there is none): a libqb object
/// that `qb64pe.exe` builds takes its `ExtraCppFlags` and `OptimizeCppProgram`, and the objects are not in the key
/// ([`written_by_builds`]). Not the same options given for one compile (`-f:`), which never reach the file
/// (`openspec\changes\test-while-building\design.md` D1). Only that section, as `qb64pe.exe` rewrites the file at
/// each run and the IDE's settings in it change nothing `make` builds. Lines are compared without the whitespace
/// around them, so a line ending changes nothing.
fn compiler_settings(config: &Path) -> Vec<u8> {
    let Ok(bytes) = std::fs::read(config) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut inside = false;
    for line in bytes.split(|&b| b == b'\n') {
        let line = line.trim_ascii();
        if line.starts_with(b"[") {
            inside = line.eq_ignore_ascii_case(b"[COMPILER SETTINGS]");
        } else if inside && !line.is_empty() {
            out.extend_from_slice(line);
            out.push(b'\n');
        }
    }
    out
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

/// Names a lock file for the libqb objects of one clone, which every build of a program writes when one it links is
/// missing or older than its source, and reads otherwise. Set by the test runner and the CLI tests
/// (`tools\legacy_tests\clone_lock.py`), so that their builds can run side by side; without it nothing is locked.
pub const OBJECTS_LOCK_ENV: &str = "QB64RUST_OBJECTS_LOCK";

/// The goal of the makefile [`lock_objects`] adds: everything `exe` takes from the clone (`QBLIB` and `EXE_LIBS`,
/// the `Makefile`'s own names) and nothing of the program's own (`qbx.o`, the icon's object, the executable).
const OBJECTS_MAKEFILE: &str = "# Written by qb64rust: what a build with these flags takes from the clone.\n\
                                qb64rust-objects: $(QBLIB) $(EXE_LIBS)\n";

/// With [`OBJECTS_LOCK_ENV`] set, takes its lock: shared when `make -q` says the clone's objects for these flags
/// are up to date, so the build only reads them, and otherwise alone, so that one build at a time writes them.
/// The returned file holds the lock until it is dropped. `make` is `cmd` before its goal.
fn lock_objects(make: &Command, build: &Path) -> Result<Option<std::fs::File>, String> {
    let Some(path) = std::env::var_os(OBJECTS_LOCK_ENV) else {
        return Ok(None);
    };
    let path = PathBuf::from(path);
    let err = |e: std::io::Error| format!("locking the libqb objects ({}): {e}", path.display());
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&path)
        .map_err(err)?;
    file.lock_shared().map_err(err)?;
    let makefile = build.join("objects.mk");
    std::fs::write(&makefile, OBJECTS_MAKEFILE).map_err(|e| format!("writing {}: {e}", makefile.display()))?;
    let mut check = Command::new(make.get_program());
    check
        .args(make.get_args())
        .args(["-q", "-f", "Makefile", "-f"])
        .arg(slash(&makefile))
        .arg("qb64rust-objects");
    // 0: up to date. 1 (to be built) or 2 (make could not tell): built alone, by the build itself.
    let fresh = check.output().is_ok_and(|o| o.status.success());
    if !fresh {
        file.unlock().map_err(err)?;
        file.lock().map_err(err)?;
    }
    Ok(Some(file))
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
    // Held until `make` has ended: shared while the clone's objects are only read, alone while they are built.
    let _objects = lock_objects(&cmd, build)?;
    cmd.arg("exe");
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

    /// The key follows libqb's sources and headers, and not what a build writes in the clone: the `qbx.o` and
    /// `qbx<N>.cpp` of a `qb64pe.exe` compile, a libqb object or archive (`study\34` §2.2). Rewritten with another
    /// length, so the stamp changes whatever the clock's resolution.
    #[test]
    fn build_key_leaves_out_what_builds_write() {
        let d = tempfile::tempdir().unwrap();
        let c = d.path().join("internal/c");
        let libqb = c.join("libqb/src");
        std::fs::create_dir_all(&libqb).unwrap();
        let written = [
            c.join("qbx.o"),
            c.join("qbx2.cpp"),
            libqb.join("qbs.o"),
            c.join("libqb_make_000.o"),
            c.join("parts/video/font/temp/a.a"),
        ];
        let sources = [c.join("qbx.cpp"), libqb.join("qbs.cpp"), c.join("libqb.h")];
        for p in written.iter().chain(&sources) {
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, "1").unwrap();
        }
        let key = || build_key(d.path(), d.path(), &Settings::default());
        let first = key();
        for p in &written {
            std::fs::write(p, "22").unwrap();
            assert_eq!(first, key(), "{}", p.display());
        }
        std::fs::remove_file(&written[0]).unwrap();
        assert_eq!(first, key());
        for p in &sources {
            let before = key();
            std::fs::write(p, "333").unwrap();
            assert_ne!(before, key(), "{}", p.display());
        }
        // A new source file is a new input too.
        let before = key();
        std::fs::write(libqb.join("new.cpp"), "1").unwrap();
        assert_ne!(before, key());
    }

    /// The key follows the compiler settings of `qb64pe.exe`, with which it builds libqb objects the key leaves
    /// out, and not the rest of its settings file, which it rewrites at each run.
    #[test]
    fn build_key_follows_the_compiler_settings_only() {
        let d = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(d.path().join("settings")).unwrap();
        let config = d.path().join("settings/config.ini");
        let write = |ide: &str, flags: &str| {
            let text = format!(
                "[GENERAL SETTINGS]\r\nBackupSize={ide}\r\n\r\n[COMPILER SETTINGS]\r\nExtraCppFlags={flags}\r\n\r\n[IDE WINDOW 1]\r\nIDE_Width={ide}\r\n"
            );
            std::fs::write(&config, text).unwrap();
        };
        let key = || build_key(d.path(), d.path(), &Settings::default());
        write("500", "");
        let first = key();
        write("600", "");
        assert_eq!(first, key());
        write("600", "-O2");
        assert_ne!(first, key());
        std::fs::remove_file(&config).unwrap();
        assert_ne!(first, key());
    }

    /// The same rule as the runner's `written_by_builds` (`tools\legacy_tests\test_run_legacy_tests.py`,
    /// `test_what_a_copy_of_the_clone_leaves_out`), less the folders `stamp_tree` does not enter.
    #[test]
    fn written_by_builds_is_the_runners() {
        assert!(written_by_builds(false, "qbs.o"));
        assert!(written_by_builds(true, "qbx2.cpp"));
        assert!(written_by_builds(true, "qbx.o"));
        assert!(written_by_builds(false, "libqb.a"));
        assert!(!written_by_builds(true, "qbx.cpp"));
        assert!(!written_by_builds(true, "libqb.cpp"));
        assert!(!written_by_builds(false, "qbx2.cpp"));
    }

    fn program_settings(console: bool, window: bool, icon: bool) -> qb64rust_ir::Settings {
        qb64rust_ir::Settings {
            console,
            window,
            window_hidden: false,
            asserts: false,
            runtime_parts: icon.then_some(qb64rust_ir::RuntimePart::Icon).into_iter().collect(),
            exe_icon: false,
            version_info: None,
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
        write_resources(&build, Some(&icon), None, "game.exe").unwrap();
        write_resources(&build, Some(&icon), None, "game.exe").unwrap();
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
        write_resources(d.path(), Some(&icon), None, "game.exe").unwrap();
        let first = key();
        assert_ne!(none, first);
        assert_eq!(first, key());
        std::fs::write(&icon, [0, 0, 1, 0, 2]).unwrap();
        write_resources(d.path(), Some(&icon), None, "game.exe").unwrap();
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

    /// A value of the version resource is part of the key: a changed one is another executable (spec
    /// `compiler/pipeline`, "A changed value"); `$VERSIONINFO` alone sets the flag of the resource script.
    #[test]
    fn build_key_follows_the_version_info() {
        let d = tempfile::tempdir().unwrap();
        let mut s = program_settings(true, false, false);
        s.version_info = Some(qb64rust_ir::VersionInfo::default());
        assert_eq!(make_flags(&s), ["DEP_CONSOLE_ONLY=y", "DEP_CONSOLE=y", "DEP_ICON_RC=y"]);
        let settings = Settings {
            flags: make_flags(&s),
            ..Settings::default()
        };
        let key = |product: &[u8]| {
            let mut v = qb64rust_ir::VersionInfo::default();
            v.strings[qb64rust_ir::VersionInfo::PRODUCT_NAME] = product.to_vec();
            write_resources(d.path(), None, Some(&v), "game.exe").unwrap();
            build_key(d.path(), d.path(), &settings)
        };
        let first = key(b"first");
        assert_eq!(first, key(b"first"));
        assert_ne!(first, key(b"second"));
        // An icon alone into the same kept folder: the earlier build's header and manifest are gone, so the key is
        // that of a fresh folder.
        let icon = d.path().join("game.ico");
        std::fs::write(&icon, [0, 0, 1, 0, 1]).unwrap();
        let icon_only = |build: &Path| {
            write_resources(build, Some(&icon), None, "game.exe").unwrap();
            build_key(d.path(), build, &settings)
        };
        let kept = icon_only(d.path());
        assert!(!d.path().join("temp/manifest.h").exists());
        assert!(!d.path().join("temp/game.exe.manifest").exists());
        let fresh = d.path().join("fresh");
        assert_eq!(kept, icon_only(&fresh));
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

    /// The embed part stands after `DEP_ZLIB=y` and before `DEP_CONSOLE=y`, as the old compiler orders them
    /// (`qb64pe.bas` 13715–13717; design D3 of `m3-network-embed`).
    #[test]
    fn make_flags_of_the_embed_part() {
        use qb64rust_ir::RuntimePart::{DataProcessing, Embed};
        let mut s = program_settings(true, false, false);
        s.runtime_parts.insert(Embed);
        assert_eq!(make_flags(&s), ["DEP_CONSOLE_ONLY=y", "DEP_EMBED=y", "DEP_CONSOLE=y"]);
        s.runtime_parts.insert(DataProcessing);
        assert_eq!(
            make_flags(&s),
            ["DEP_CONSOLE_ONLY=y", "DEP_ZLIB=y", "DEP_EMBED=y", "DEP_CONSOLE=y"]
        );
    }

    /// `embedded.cpp` holds each used file's bytes as they are: an empty file has no array and gives an empty
    /// string, every byte value is written, a file of 33 bytes ends with a short line; a program without a used file
    /// gets no `embedded.cpp`, and one an earlier build left is removed (design D3 and D4 of `m3-network-embed`).
    #[test]
    fn embedded_files_in_the_build() {
        let all: Vec<u8> = (0..=255).collect();
        let odd: Vec<u8> = (b'A'..=b'Z').chain(b'a'..=b'g').collect();
        let files = [
            ("Empty".to_string(), Vec::new()),
            ("all256".to_string(), all),
            ("odd".to_string(), odd.clone()),
        ];
        let text = embedded_cpp(&files);
        assert!(text.starts_with("#include <stdint.h>\n#include <string.h>\n#include \"common.h\"\n"));
        assert!(!text.contains("embed_Empty"));
        assert!(
            text.contains("if (qbs_equal(handle, qbs_new_txt(\"Empty\"))) {qbs *data = qbs_new(0, 1); return data;}")
        );
        assert!(text.contains("static const uint8_t embed_all256[] = {\n    0x00,0x01,"));
        assert!(text.contains("0xFE,0xFF,\n};"));
        assert!(text.contains("memcpy(data->chr, embed_all256, 256); return data;}"));
        // 33 bytes: two lines of 16 and one of 1.
        assert!(text.contains("    0x67,\n};"));
        assert!(text.contains("memcpy(data->chr, embed_odd, 33);"));
        assert!(text.ends_with("    return qbs_new_txt(\"\");\n}\n"));
        assert_eq!(odd.len(), 33);

        let d = tempfile::tempdir().unwrap();
        write_embedded(d.path(), &files).unwrap();
        let path = d.path().join("temp/embedded.cpp");
        assert_eq!(std::fs::read(&path).unwrap(), text.as_bytes());
        write_embedded(d.path(), &[]).unwrap();
        assert!(!path.exists());
        write_embedded(d.path(), &[]).unwrap();
    }

    /// The sockets part is two flags after `DEP_CONSOLE_ONLY=y` and before `DEP_PRINTER=y`, as the old compiler
    /// orders them (`qb64pe.bas` 13709; design D5 of `m3-network-embed`).
    #[test]
    fn make_flags_of_the_sockets_part() {
        use qb64rust_ir::RuntimePart::{DataProcessing, ImageCodec, Printer, Sockets};
        let mut s = program_settings(true, false, false);
        s.runtime_parts.insert(Sockets);
        assert_eq!(
            make_flags(&s),
            ["DEP_CONSOLE_ONLY=y", "DEP_SOCKETS=y", "DEP_HTTP=y", "DEP_CONSOLE=y"]
        );
        let mut s = program_settings(false, true, false);
        s.runtime_parts.extend([Sockets, ImageCodec, Printer, DataProcessing]);
        assert_eq!(
            make_flags(&s),
            [
                "DEP_IMAGE_CODEC=y",
                "DEP_SOCKETS=y",
                "DEP_HTTP=y",
                "DEP_PRINTER=y",
                "DEP_ZLIB=y"
            ]
        );
    }
}
