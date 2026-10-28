//! `DECLARE LIBRARY`: the files a block names and the linkage of its declarations (design D2 of
//! `m3-declare-library`; `qb64pe.bas` 4447–5318 and 13776–13933 for Windows; measured, `verification\v37_*`,
//! `study\00` §5). `sema` reads no file; this runs after it and before the C++, and gives `codegen-cpp` a
//! [`Resolved`]. Its errors are diagnostics of the program, at the block's line.

use crate::{Frontend, Linkage, Resolved, ResolvedLibrary};
use qb64rust_base::Span;
use qb64rust_sema::{LibraryKind, ProcId};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Where the lookup looks.
#[derive(Clone, Debug)]
pub struct Places {
    /// The old compiler's folder, which a written path is taken from: the QB64pe folder, else the include root
    /// (design D5 of `m6-toolchain`).
    pub root: PathBuf,
    /// The folder of the main source file: what `./` names, and a DLL is also looked for there.
    pub source_dir: PathBuf,
    /// `%SYSTEMROOT%\System32`, where a DLL is also looked for.
    pub system32: Option<PathBuf>,
    /// The toolchain's `nm.exe`, which reads a library without a header; or why there is none (no toolchain found).
    pub nm: Result<PathBuf, String>,
}

impl Places {
    /// The places of a compile of `main_path` with `root` as the old compiler's folder.
    pub fn new(root: &Path, main_path: &Path, nm: Result<PathBuf, String>) -> std::io::Result<Places> {
        Ok(Places {
            root: root.to_path_buf(),
            source_dir: folder_of(main_path)?,
            system32: std::env::var_os("SYSTEMROOT").map(|r| PathBuf::from(r).join("System32")),
            nm,
        })
    }
}

/// The absolute folder of the file `path` (the working directory for a name without a folder).
fn folder_of(path: &Path) -> std::io::Result<PathBuf> {
    let dir = path.parent().unwrap_or(Path::new(""));
    std::path::absolute(if dir.as_os_str().is_empty() {
        Path::new(".")
    } else {
        dir
    })
}

/// What one name of a block found.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Found {
    /// A library file for the link command (`.lib`, `.a`, `.o`).
    pub file: Option<PathBuf>,
    pub header: Option<PathBuf>,
    /// A `DYNAMIC` library: the name `LoadLibrary` is given.
    pub dll: Option<String>,
    /// A `DYNAMIC` library: the file the lookup found, which tells two names of one DLL apart from two DLLs.
    pub dll_file: Option<PathBuf>,
}

/// A written name taken apart as the old compiler does (`qb64pe.bas` 4510–4590): `\\` made `\`, the version after
/// a `.` or `:` dropped, `/` made `\`; then the folder, up to and with the last `\`, and the name.
fn split(written: &str) -> (String, String) {
    let mut x = written.to_string();
    while x.contains("\\\\") {
        x = x.replacen("\\\\", "\\", 1);
    }
    let x = qb64rust_codegen_cpp::strip_version(&x).replace('/', "\\");
    match x.rfind('\\') {
        Some(at) => (x[..=at].to_string(), x[at + 1..].to_string()),
        None => (String::new(), x),
    }
}

/// The first of `candidates` that is a file.
fn first_file(candidates: impl IntoIterator<Item = PathBuf>) -> Option<PathBuf> {
    candidates.into_iter().find(|p| p.is_file())
}

/// Looks for one name of a block of kind `kind` in the old compiler's order on Windows (`qb64pe.bas` 4590–5204):
/// `file_dir` is the folder of the file the block stands in. The written folder is taken from the clone, except
/// exactly `./`, which is the source's folder (`x18`, `x51`).
/// - Not `DYNAMIC`: `<folder>name.lib`, `.a`, `.o`, then `name.lib`, `.a`, `.o` from the clone; and a header
///   `<folder>name.h`, `.hpp`, `name.h`, `.hpp`, then from `file_dir` joined with the written folder (`v37_i`).
///   Neither: "LIBRARY not found" (`x01`, `x18`, `x52`).
/// - `DYNAMIC`: `<folder>name.dll`, the same in System32, `name.dll` from the clone or the source's folder, then
///   System32; no header. Nothing: "DYNAMIC LIBRARY not found" (`x02`, `x53`).
///
/// A name that is only a folder finds nothing and is no error, as for the old compiler.
pub fn find(kind: LibraryKind, written: &str, file_dir: &Path, places: &Places) -> Result<Found, &'static str> {
    let (folder, x) = split(written);
    if x.is_empty() {
        return Ok(Found::default());
    }
    let beside_source = folder == ".\\";
    let base = if beside_source {
        places.source_dir.clone()
    } else {
        places.root.join(&folder)
    };
    let named = |dir: &Path, ext: &str| dir.join(format!("{x}{ext}"));
    if kind == LibraryKind::Dynamic {
        let dll = format!("{x}.dll");
        // `LoadLibrary` is given the folder as the lookup holds it: the source's folder for `./`, else as written.
        let with_folder = || {
            if beside_source {
                named(&places.source_dir, ".dll").to_string_lossy().into_owned()
            } else {
                format!("{folder}{dll}")
            }
        };
        let system32 = places.system32.as_deref();
        let by_folder = first_file(
            std::iter::once(named(&base, ".dll"))
                .chain(system32.filter(|_| !beside_source).map(|s| s.join(&folder).join(&dll))),
        );
        let found = match by_folder {
            Some(file) => Some((with_folder(), file)),
            None => first_file(
                [places.root.as_path(), places.source_dir.as_path()]
                    .into_iter()
                    .chain(system32)
                    .map(|d| d.join(&dll)),
            )
            .map(|file| (dll, file)),
        };
        return match found {
            Some((dll, file)) => Ok(Found {
                dll: Some(dll),
                dll_file: Some(file),
                ..Found::default()
            }),
            None => Err("DYNAMIC LIBRARY not found"),
        };
    }
    let exts = [".lib", ".a", ".o"];
    let file = first_file(
        exts.iter()
            .map(|e| named(&base, e))
            .chain(exts.iter().map(|e| named(&places.root, e))),
    );
    let fallback = file_dir.join(&folder);
    let header = first_file(
        [&base, &places.root, &fallback]
            .into_iter()
            .flat_map(|d| [named(d, ".h"), named(d, ".hpp")]),
    );
    if file.is_none() && header.is_none() {
        return Err("LIBRARY not found");
    }
    Ok(Found {
        file,
        header,
        ..Found::default()
    })
}

/// Whether a header has an include guard: a `#pragma once`, or an `#ifndef X` (or `#if !defined(X)`) whose `X` a
/// later `#define` defines, wherever they stand. `#ifdef __cplusplus` is none. It errs towards accepting, as
/// [`defines`] does: `#ifndef NULL` with `#define NULL 0` passes for a guard too.
fn has_guard(text: &[u8]) -> bool {
    let ident = |s: &[u8]| -> Vec<u8> {
        s.iter()
            .take_while(|b| b.is_ascii_alphanumeric() || **b == b'_')
            .copied()
            .collect()
    };
    let mut tested: Vec<Vec<u8>> = Vec::new();
    for line in text.split(|&b| b == b'\n') {
        let Some(rest) = line.trim_ascii_start().strip_prefix(b"#") else {
            continue;
        };
        let rest = rest.trim_ascii_start();
        let word = rest.iter().take_while(|b| b.is_ascii_alphabetic()).count();
        let arg = rest[word..].trim_ascii();
        match &rest[..word] {
            b"pragma" if arg.starts_with(b"once") => return true,
            b"ifndef" => tested.push(ident(arg)),
            b"if" => {
                let Some(arg) = arg.strip_prefix(b"!") else {
                    continue;
                };
                let Some(arg) = arg.trim_ascii_start().strip_prefix(b"defined") else {
                    continue;
                };
                let arg = arg.trim_ascii_start();
                tested.push(ident(arg.strip_prefix(b"(").unwrap_or(arg).trim_ascii_start()));
            }
            b"define" => {
                let name = ident(arg);
                if !name.is_empty() && tested.contains(&name) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Whether a header defines something, so that two copies of it fail in C++: outside comments, literals and
/// preprocessor lines, a `{` that opens neither an `extern "C"` block nor a namespace (a function's body, a
/// `struct`, an initializer). A header that only declares may be included twice (measured, `verification\v37_x68`).
/// It errs towards accepting, leaving the header to the C++ compiler (whose error the old compiler gives too): a
/// definition without a brace (`int x;`), a `{` after a C++14 digit separator (`1'000` reads as a character literal
/// to the end of the line), a directive after a comment on its line (`/* x */ #define`) read as code.
fn defines(text: &[u8]) -> bool {
    // The code since the last `;`, `{` or `}`, comments and preprocessor lines left out.
    let mut code: Vec<u8> = Vec::new();
    let mut line_start = true;
    let mut i = 0;
    while i < text.len() {
        let b = text[i];
        if b == b'\n' {
            line_start = true;
            code.push(b' ');
            i += 1;
            continue;
        }
        if b.is_ascii_whitespace() {
            code.push(b' ');
            i += 1;
            continue;
        }
        let was_line_start = std::mem::replace(&mut line_start, false);
        match b {
            b'/' if text.get(i + 1) == Some(&b'/') => {
                while i < text.len() && text[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'/' if text.get(i + 1) == Some(&b'*') => {
                i = text[i + 2..]
                    .windows(2)
                    .position(|w| w == b"*/")
                    .map_or(text.len(), |at| i + 2 + at + 2);
                code.push(b' ');
                continue;
            }
            b'#' if was_line_start => {
                // To the end of the line, and of each line a `\` at its end continues.
                while i < text.len() && text[i] != b'\n' {
                    if text[i] == b'\\' {
                        i += 1;
                        if text.get(i) == Some(&b'\r') {
                            i += 1;
                        }
                    }
                    i += 1;
                }
                continue;
            }
            b'"' | b'\'' => {
                let start = i;
                i += 1;
                while i < text.len() && text[i] != b && text[i] != b'\n' {
                    if text[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
                i = (i + 1).min(text.len());
                code.extend_from_slice(&text[start..i]);
                continue;
            }
            b'{' => {
                let words: Vec<&[u8]> = code.split(|&c| c == b' ').filter(|w| !w.is_empty()).collect();
                // By the last words, so that a macro without a `;` before them (`API_BEGIN extern "C" {`) is
                // no definition: `extern "C"`, `extern "C++"`, `namespace`, `namespace n`, `namespace a::b`.
                let from_end = |n: usize, word: &[u8]| words.len().checked_sub(n).is_some_and(|at| words[at] == word);
                let opens_scope = (from_end(2, b"extern") && (from_end(1, b"\"C\"") || from_end(1, b"\"C++\"")))
                    || from_end(1, b"namespace")
                    || from_end(2, b"namespace");
                if !opens_scope {
                    return true;
                }
                code.clear();
            }
            b';' | b'}' => code.clear(),
            _ => code.push(b),
        }
        i += 1;
    }
    false
}

/// Why `nm` gave no linkage for a name.
#[derive(Debug, PartialEq, Eq)]
pub enum NmError {
    NotFound,
    /// More than one C++ symbol of the name.
    Multiple,
    /// `nm` could not be run.
    Run(String),
}

/// The lines of `nm`'s output, without a CR.
fn lines(out: &[u8]) -> impl Iterator<Item = &[u8]> {
    out.split(|&b| b == b'\n').map(|l| l.strip_suffix(b"\r").unwrap_or(l))
}

/// The linkage of the C name `name` from `nm --demangle -g` (`g`) of a library, and if need be `nm -D --demangle
/// -g` (`dynamic`), as the old compiler decides it (`qb64pe.bas` 13805–13933): a line holding ` name(` is a C++
/// symbol, with its text from the name to the first `)`; two such lines are an error; else a line ending in
/// ` name` is a C symbol, first in `g`, then in `dynamic`.
pub fn linkage_in(g: &[u8], dynamic: impl FnOnce() -> Result<Vec<u8>, String>, name: &str) -> Result<Linkage, NmError> {
    let cpp = format!(" {name}(");
    let c = format!(" {name}");
    let mut found = Vec::new();
    for line in lines(g) {
        if let Some(at) = line.windows(cpp.len()).position(|w| w == cpp.as_bytes()) {
            let text = &line[at + 1..];
            let end = text.iter().position(|&b| b == b')').map_or(text.len(), |e| e + 1);
            #[expect(
                clippy::disallowed_methods,
                reason = "a C++ symbol shown by nm, written into the C++"
            )]
            found.push(String::from_utf8_lossy(&text[..end]).into_owned());
        }
    }
    match found.len() {
        0 => {}
        1 => return Ok(Linkage::Cpp(found.remove(0))),
        _ => return Err(NmError::Multiple),
    }
    let ends = |out: &[u8]| lines(out).any(|l| l.ends_with(c.as_bytes()));
    if ends(g) || ends(&dynamic().map_err(NmError::Run)?) {
        return Ok(Linkage::C);
    }
    Err(NmError::NotFound)
}

/// The toolchain's `nm`, run once per file and option in a compile.
struct Nm {
    exe: Result<PathBuf, String>,
    runs: HashMap<(PathBuf, bool), Result<Vec<u8>, String>>,
}

impl Nm {
    fn new(places: &Places) -> Nm {
        Nm {
            exe: places.nm.clone(),
            runs: HashMap::new(),
        }
    }

    /// `nm <file> [-D] --demangle -g`: its standard output, whatever its exit status (on an archive `-D` finds
    /// nothing), as the old compiler reads it.
    fn symbols(&mut self, file: &Path, dynamic: bool) -> Result<Vec<u8>, String> {
        self.runs
            .entry((file.to_path_buf(), dynamic))
            .or_insert_with(|| {
                let exe = self.exe.as_ref().map_err(|e| format!("cannot read the library: {e}"))?;
                let mut cmd = Command::new(exe);
                cmd.arg(file);
                if dynamic {
                    cmd.arg("-D");
                }
                cmd.args(["--demangle", "-g"])
                    .output()
                    .map(|o| o.stdout)
                    .map_err(|e| format!("cannot run `{}`: {e}", exe.display()))
            })
            .clone()
    }
}

/// Finds the files of every block of `fe`'s program and the linkage of each declaration of a block without a
/// header, adding an error for each block whose library is not found or whose declaration its library does not
/// hold. The old compiler's way is kept where it rejects a program (`DECISIONS.md` 2026-10-16): a declaration is
/// looked for in the block's last library only (`x24`). Where its build would fail, the program is rejected here,
/// before the C++: a header without a guard that defines something and that two names find (`x16`; one that only
/// declares builds, `x68`), two `DYNAMIC` blocks whose handles have one name (`x67`), a block without a header
/// whose last name finds no library file (`x69`), and a library file whose path holds a `$`, which `make` expands
/// (`x70`: `lib$dir\v37c.a` reaches the link as `libir\v37c.a`).
pub fn resolve(fe: &mut Frontend, places: &Places) -> Resolved {
    let mut resolved = Resolved::default();
    let mut nm = Nm::new(places);
    // Each header found so far, by its canonical path.
    let mut headers: Vec<PathBuf> = Vec::new();
    // Each `DYNAMIC` handle so far: its name, the DLL's file by its canonical path, and the block's name as written.
    let mut handles: Vec<(String, Option<PathBuf>, &str)> = Vec::new();
    let program = &fe.program;
    let diags = &mut fe.diagnostics;
    for (i, lib) in program.libraries.iter().enumerate() {
        let mut found = ResolvedLibrary::default();
        resolved.libraries.push(ResolvedLibrary::default());
        let file_dir = folder_of(Path::new(&fe.map.file(lib.span.file).name)).unwrap_or_default();
        // The library file of the block's last name, where its declarations are looked for, and that name.
        let mut last = None;
        let mut dll_file = None;
        let mut failed = false;
        for name in &lib.names {
            match find(lib.kind, &name.text, &file_dir, places) {
                Err(msg) => {
                    diags.error(name.span, msg);
                    failed = true;
                    break;
                }
                Ok(f) => {
                    if let Some(h) = f.header {
                        check_twice(&h, &mut headers, &name.text, name.span, diags);
                        found.headers.push(h);
                    }
                    if let Some(file) = &f.file {
                        if file.to_string_lossy().contains('$') {
                            diags.error(
                                name.span,
                                format!(
                                    "the path of the library file of `{}` holds a `$`, which `make` expands, so the \
                                     link fails, as the old compiler's does; move the file to a folder without one",
                                    name.text
                                ),
                            );
                        }
                        found.files.push(file.clone());
                    }
                    last = f.file.map(|file| (file, &name.text));
                    found.dll = f.dll.or(found.dll);
                    dll_file = f.dll_file.or(dll_file);
                }
            }
        }
        if failed {
            continue;
        }
        if lib.kind == LibraryKind::Dynamic
            && let Some(name) = lib.names.first()
        {
            let handle = qb64rust_codegen_cpp::dll_handle(&name.text);
            // One DLL by the file found: `./v37dll` and `v37dll` beside the source are one, `a/x` and `b/x` two.
            let file = dll_file.map(|f| std::fs::canonicalize(&f).unwrap_or(f));
            match handles.iter().find(|(h, ..)| *h == handle) {
                Some((_, earlier_file, earlier)) => {
                    let advice = if *earlier_file == file {
                        "declare the functions in one block".to_string()
                    } else {
                        format!(
                            "`{earlier}` and `{}` are two DLLs whose names give one handle; rename one of them",
                            name.text
                        )
                    };
                    diags.error(
                        lib.span,
                        format!(
                            "an earlier `DECLARE DYNAMIC LIBRARY` block has the DLL handle `{handle}` too: the old \
                             compiler declares it once per block and its C++ fails; {advice}"
                        ),
                    );
                }
                None => handles.push((handle, file, &name.text)),
            }
        }
        // Linkage by `nm` for a named block without a header (a `DYNAMIC` block has none and needs none), from the
        // file of its last name.
        if lib.kind != LibraryKind::Dynamic
            && found.headers.is_empty()
            && let Some(last_name) = lib.names.last()
        {
            let decls: Vec<_> = program
                .procs
                .iter()
                .enumerate()
                .filter_map(|(p, q)| {
                    q.external
                        .as_ref()
                        .filter(|e| e.library.0 as usize == i)
                        .map(|e| (p, e))
                })
                .collect();
            match last {
                // The old compiler writes no prototype then, and its C++ fails at the first call (`x69`).
                None if !decls.is_empty() => diags.error(
                    lib.span,
                    format!(
                        "the last name of the block, `{}`, finds no library file, where the old compiler looks for \
                         the block's functions, and no header declares them: its C++ fails",
                        last_name.text
                    ),
                ),
                None => {}
                Some((file, written)) => {
                    let shown = shown_file(written, &file);
                    for (p, e) in decls {
                        let g = nm.symbols(&file, false);
                        let linkage = g
                            .map_err(NmError::Run)
                            .and_then(|g| linkage_in(&g, || nm.symbols(&file, true), &e.c_name));
                        match linkage {
                            Ok(l) => {
                                resolved.linkage.insert(ProcId(qb64rust_base::to_u32(p)), l);
                            }
                            Err(NmError::NotFound) => diags.error(
                                lib.span,
                                format!("Could not find sub/function '{}' in '{shown}'", e.c_name),
                            ),
                            Err(NmError::Multiple) => diags.error(
                                lib.span,
                                format!(
                                    "Unable to resolve multiple instances of sub/function '{}' in '{shown}'",
                                    e.c_name
                                ),
                            ),
                            // Once for the block: every declaration would give the same.
                            Err(NmError::Run(msg)) => {
                                diags.error(lib.span, msg);
                                break;
                            }
                        }
                    }
                }
            }
        }
        resolved.libraries[i] = found;
    }
    resolved
}

/// A header found again: an error if it has no guard and defines something, since it is then included twice and
/// its C++ fails, as the old compiler's does (measured, `verification\v37_x16`; `DECISIONS.md` 2026-10-16). One
/// that only declares builds (`x68`). A header that cannot be read is left to the C++ compiler.
fn check_twice(
    header: &Path,
    seen: &mut Vec<PathBuf>,
    written: &str,
    span: Span,
    diags: &mut qb64rust_base::Diagnostics,
) {
    let canonical = std::fs::canonicalize(header).unwrap_or_else(|_| header.to_path_buf());
    if !seen.contains(&canonical) {
        seen.push(canonical);
        return;
    }
    if std::fs::read(header).is_ok_and(|text| !has_guard(&text) && defines(&text)) {
        diags.error(
            span,
            format!(
                "the header of `{written}` is named by an earlier `DECLARE LIBRARY` too, defines what it declares \
                 and has no include guard (`#pragma once` or `#ifndef`): the old compiler includes it twice and its \
                 C++ fails"
            ),
        );
    }
}

/// A library file as an error names it: the written name without its version, with the file's extension. The old
/// compiler names the full path; no diagnostic here holds a folder of the machine (as for `$EXEICON`, D-015).
fn shown_file(written: &str, file: &Path) -> String {
    let ext = file.extension().map(|e| e.to_string_lossy()).unwrap_or_default();
    format!("{}.{ext}", qb64rust_codegen_cpp::strip_version(written))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A temporary layout: `root` as the clone, `src` as the source's folder, `src\inc` as an included file's,
    /// `sys` as System32.
    struct Layout {
        _dir: tempfile::TempDir,
        places: Places,
        inc: PathBuf,
    }

    impl Layout {
        fn new(files: &[&str]) -> Layout {
            let dir = tempfile::tempdir().unwrap();
            let base = dir.path().to_path_buf();
            for f in files {
                let p = base.join(f);
                std::fs::create_dir_all(p.parent().unwrap()).unwrap();
                std::fs::write(&p, "x").unwrap();
            }
            for d in ["root", "src/inc", "sys"] {
                std::fs::create_dir_all(base.join(d)).unwrap();
            }
            Layout {
                places: Places {
                    root: base.join("root"),
                    source_dir: base.join("src"),
                    system32: Some(base.join("sys")),
                    nm: Err("no nm in these tests".to_string()),
                },
                inc: base.join("src/inc"),
                _dir: dir,
            }
        }

        fn find(&self, kind: LibraryKind, written: &str) -> Result<Found, &'static str> {
            find(kind, written, &self.places.source_dir, &self.places)
        }

        fn path(&self, rel: &str) -> PathBuf {
            self.places.root.parent().unwrap().join(rel)
        }
    }

    fn file(p: PathBuf) -> Found {
        Found {
            file: Some(p),
            ..Found::default()
        }
    }

    /// Each step of the order, for the forms that are not `DYNAMIC`.
    #[test]
    fn library_files_and_headers_in_the_old_compilers_order() {
        let l = Layout::new(&[
            "src/v37c.a",
            "src/both.a",
            "src/both.o",
            "root/lib/rl.lib",
            "root/lib/rl.a",
            "root/plain.o",
            "root/sub/hd.h",
            "src/sub/fb.hpp",
            "src/inc/up/ih.h",
            "src/v.a",
            "root/hp.hpp",
        ]);
        let plain = LibraryKind::Plain;
        // `./` is the source's folder; a name without it is not (`x18`).
        assert_eq!(l.find(plain, "./v37c"), Ok(file(l.path("src/v37c.a"))));
        assert_eq!(l.find(plain, ".\\v37c"), Ok(file(l.path("src/v37c.a"))));
        assert_eq!(l.find(plain, "v37c"), Err("LIBRARY not found"));
        // `.lib` before `.a` before `.o`; from the clone by the written folder, then by the name alone.
        assert_eq!(l.find(plain, "./both"), Ok(file(l.path("src/both.a"))));
        assert_eq!(l.find(plain, "lib/rl"), Ok(file(l.path("root/lib/rl.lib"))));
        assert_eq!(l.find(plain, "nowhere/plain"), Ok(file(l.path("root/plain.o"))));
        // A version is dropped (`v37_i`).
        assert_eq!(l.find(plain, "./v.1.0"), Ok(file(l.path("src/v.a"))));
        assert_eq!(l.find(LibraryKind::Static, "./v:2"), Ok(file(l.path("src/v.a"))));
        // A header from the clone, `.hpp`, then from the source's folder joined with the written one.
        let header = |p: &str| Found {
            header: Some(l.path(p)),
            ..Found::default()
        };
        assert_eq!(l.find(plain, "sub/hd"), Ok(header("root/sub/hd.h")));
        assert_eq!(l.find(plain, "hp"), Ok(header("root/hp.hpp")));
        assert_eq!(l.find(plain, "sub/fb"), Ok(header("src/sub/fb.hpp")));
        // From an included file's folder (`v37_i`: `"../v37_lib/v37h"` from `v37_inc\`).
        let from_inc = find(plain, "up/ih", &l.inc, &l.places);
        assert_eq!(from_inc, Ok(header("src/inc/up/ih.h")));
        assert_eq!(l.find(plain, "up/ih"), Err("LIBRARY not found"));
        // A DLL is no library for these forms (`x52`); a folder alone finds nothing and is no error.
        assert_eq!(l.find(LibraryKind::Customtype, "./nothing"), Err("LIBRARY not found"));
        assert_eq!(l.find(plain, "sub/"), Ok(Found::default()));
    }

    /// Each step of the order for `DYNAMIC`, and the name `LoadLibrary` is given.
    #[test]
    fn dlls_in_the_old_compilers_order() {
        let l = Layout::new(&[
            "src/own.dll",
            "root/d/rd.dll",
            "sys/d/sd.dll",
            "src/near.dll",
            "sys/kernel.dll",
            "root/only.h",
        ]);
        let dynamic = |w: &str| l.find(LibraryKind::Dynamic, w).map(|f| f.dll.unwrap());
        let own = l.path("src").join("own.dll").to_string_lossy().into_owned();
        assert_eq!(dynamic("./own"), Ok(own));
        // The file found, whatever `LoadLibrary` is given.
        let found = |w: &str| l.find(LibraryKind::Dynamic, w).map(|f| f.dll_file.unwrap());
        assert_eq!(found("./own"), Ok(l.path("src/own.dll")));
        assert_eq!(found("d/sd"), Ok(l.path("sys/d/sd.dll")));
        assert_eq!(found("x/near"), Ok(l.path("src/near.dll")));
        assert_eq!(found("kernel"), Ok(l.path("sys/kernel.dll")));
        assert_eq!(dynamic("d/rd"), Ok("d\\rd.dll".to_string()));
        assert_eq!(dynamic("d/sd"), Ok("d\\sd.dll".to_string()));
        // By the name alone: beside the source, then in System32 (`v37_g`, `kernel32`).
        assert_eq!(dynamic("x/near"), Ok("near.dll".to_string()));
        assert_eq!(dynamic("kernel"), Ok("kernel.dll".to_string()));
        assert_eq!(dynamic("./kernel"), Ok("kernel.dll".to_string()));
        // A header is no DLL (`x53`).
        assert_eq!(dynamic("only"), Err("DYNAMIC LIBRARY not found"));
    }

    #[test]
    fn guards() {
        assert!(has_guard(b"#pragma once\nint f(void);\n"));
        assert!(has_guard(b"  #  pragma   once\r\n"));
        assert!(has_guard(b"#ifndef H\r\n#define H\r\n#endif\r\n"));
        assert!(has_guard(b"// first\n#if !defined(H_)\n# define H_ 1\n#endif\n"));
        assert!(has_guard(b"#if ! defined H\n#define H\n"));
        assert!(!has_guard(b"// no guard\nint g_one(void) { return 1; }\n"));
        assert!(!has_guard(b"#include <stdint.h>\n#pragma pack(1)\n"));
        // A test that defines nothing it tested is no guard: `__cplusplus`, `#ifndef` of another name.
        assert!(!has_guard(b"#ifdef __cplusplus\nextern \"C\" {\n#endif\n"));
        assert!(!has_guard(b"#ifndef A\n#define B\n#endif\n#if !defined(C)\n#endif\n"));
    }

    /// What two copies of fail in C++ (`v37_x16`), and what may be included twice (`v37_x68`).
    #[test]
    fn definitions_in_a_header() {
        // Declarations only, in `extern "C"` and a namespace, with braces in comments, strings and macros.
        let declares = b"#include <stdint.h>\n#ifdef __cplusplus\nextern \"C\" {\n#endif\n\
                         int32_t c_add(int32_t a, int32_t b); // { not here\n/* nor { here */\n\
                         #define INIT { 0, \\\n 1 }\nextern const char *s;\n#ifdef __cplusplus\n}\n#endif\n\
                         namespace n { int f(); }\ninline namespace m { int g(); }\nextern \"C++\"{ int h(); }\n\
                         int q(const char *t = \"{\");\nchar c = '{';\n";
        assert!(!defines(declares));
        assert!(!defines(b"extern \"C\" int32_t c_add(int32_t a, int32_t b);\r\n"));
        // A macro without a `;` before the scope, and namespaces by their last words.
        assert!(!defines(b"API_BEGIN extern \"C\" { int f(void); }\n"));
        assert!(!defines(
            b"namespace { int f(); }\nnamespace a::b { int g(); }\nAPI namespace c { }\n"
        ));
        // A function's body, a struct, an initializer, a body with `extern "C"` before it.
        assert!(defines(b"int g_one(void) { return 1; }\n"));
        assert!(defines(b"struct P { int x; };\n"));
        assert!(defines(b"int a[] = {1, 2};\n"));
        assert!(defines(b"extern \"C\" int f(void) { return 0; }\n"));
        assert!(defines(b"namespace n { int f() { return 1; } }\n"));
    }

    /// The old compiler's reading of `nm`'s lines (`qb64pe.bas` 13805–13933).
    #[test]
    fn linkage_from_nm() {
        let none = || Ok(Vec::new());
        // As `nm` writes an archive's symbols, with CRLF.
        let lines = [
            "",
            "v37cpp.o:",
            "0000000000000000 T cpp_add(int, int)",
            "0000000000000010 T c_add",
            "",
        ];
        let g = lines.join("\r\n");
        let g = g.as_bytes();
        assert_eq!(
            linkage_in(g, none, "cpp_add"),
            Ok(Linkage::Cpp("cpp_add(int, int)".to_string()))
        );
        assert_eq!(linkage_in(g, none, "c_add"), Ok(Linkage::C));
        // A name that only ends another is not it; nor one that only starts another.
        assert_eq!(linkage_in(g, none, "add"), Err(NmError::NotFound));
        assert_eq!(linkage_in(g, none, "c_ad"), Err(NmError::NotFound));
        let two = b"0 T f(int)\n0 T f(double)\n";
        assert_eq!(linkage_in(two, none, "f"), Err(NmError::Multiple));
        // `-D` is read for a C name only when `-g` has none.
        let dynamic = || Ok(b"0 T d_add\n".to_vec());
        assert_eq!(linkage_in(b"", dynamic, "d_add"), Ok(Linkage::C));
        let fails = || Err("no nm".to_string());
        assert_eq!(linkage_in(b"", fails, "x"), Err(NmError::Run("no nm".to_string())));
    }

    #[test]
    fn written_names_taken_apart() {
        assert_eq!(split("./v37c"), (".\\".into(), "v37c".into()));
        assert_eq!(split("v37_lib/v37h.2"), ("v37_lib\\".into(), "v37h".into()));
        assert_eq!(split("a\\\\b"), ("a\\".into(), "b".into()));
        assert_eq!(split("kernel32"), (String::new(), "kernel32".into()));
        assert_eq!(shown_file("./v37c", Path::new("x/v37c.a")), "./v37c.a");
    }
}
