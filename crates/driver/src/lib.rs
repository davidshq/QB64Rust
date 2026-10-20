//! The compiler pipeline as a library, so that tests can run each stage; `main.rs` is the command line.

pub mod build;
pub mod cpp_errors;
pub mod library;
pub mod report;

use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_sema::Program;
use qb64rust_syntax::{LoadError, Loader, NoLoader, ParsedProgram, include_name, parse_settled};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub use qb64rust_base::STACK_SIZE;
pub use qb64rust_codegen_cpp::{Linkage, Resolved, ResolvedLibrary};

/// Runs `f` on a new thread with [`STACK_SIZE`] of stack and returns its result. A panic in `f` is passed on.
pub fn with_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let thread = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(f)
        .expect("cannot start the compiler thread");
    match thread.join() {
        Ok(v) => v,
        Err(e) => std::panic::resume_unwind(e),
    }
}

/// The result of the front end for one program: its trees, and the typed program if parsing and checking gave no
/// errors.
pub struct Frontend {
    pub map: SourceMap,
    /// The main file.
    pub file: FileId,
    pub parsed: ParsedProgram,
    pub program: Program,
    pub diagnostics: Diagnostics,
}

impl Frontend {
    pub fn has_errors(&self) -> bool {
        self.diagnostics.has_errors()
    }

    /// Each error rendered as `<file>:<line>:<col>: error: <message>`, one per line, in source order (the main file
    /// first, then the included files in the order they were loaded). Warnings are left out.
    pub fn render_diagnostics(&self) -> String {
        self.render(false)
    }

    /// The errors and the warnings, in source order (`-w`, spec `compiler/cli`).
    pub fn render_with_warnings(&self) -> String {
        self.render(true)
    }

    fn render(&self, warnings: bool) -> String {
        let mut list: Vec<_> = if warnings {
            self.diagnostics.list().to_vec()
        } else {
            self.diagnostics.errors().cloned().collect()
        };
        list.sort_by_key(|d| (d.span.file.0, d.span.start));
        let mut out = String::new();
        for d in list {
            out.push_str(&d.render(&self.map));
            out.push('\n');
        }
        out
    }
}

/// Finds included files on disk (design D9, measured M7): a path as written (a leading `.\` or `./` dropped) in
/// the including file's folder, then relative to the compiler root; an absolute path as written. Never relative
/// to the working directory. A file found twice (by its canonical path) is the same [`FileId`], so that
/// `$INCLUDEONCE` can recognise it.
pub struct FileLoader {
    root: PathBuf,
    /// The real folder of each file loaded, the main file's first.
    dirs: HashMap<FileId, PathBuf>,
    /// Canonical path -> file, for files loaded already.
    seen: HashMap<PathBuf, FileId>,
}

impl FileLoader {
    /// `main` is the main file as added to the map, `main_path` where it is on disk, `root` the compiler root.
    pub fn new(root: &Path, main: FileId, main_path: &Path) -> FileLoader {
        let dir = main_path.parent().map_or_else(PathBuf::new, Path::to_path_buf);
        FileLoader {
            root: root.to_path_buf(),
            dirs: HashMap::from([(main, dir)]),
            seen: HashMap::new(),
        }
    }
}

impl Loader for FileLoader {
    fn load(&mut self, map: &mut SourceMap, from: FileId, path: &[u8]) -> Result<FileId, LoadError> {
        let rel = &include_name(path);
        let from_dir = self.dirs.get(&from).cloned().unwrap_or_default();
        let shown_dir = Path::new(&map.file(from).name)
            .parent()
            .map_or_else(PathBuf::new, Path::to_path_buf);
        let candidates = if rel.is_absolute() {
            vec![(rel.to_path_buf(), rel.to_path_buf())]
        } else {
            vec![
                (from_dir.join(rel), shown_dir.join(rel)),
                (self.root.join(rel), self.root.join(rel)),
            ]
        };
        let Some((real, shown)) = candidates.into_iter().find(|(p, _)| p.is_file()) else {
            return Err(LoadError::NotFound);
        };
        let canonical = std::fs::canonicalize(&real).map_err(|e| LoadError::Unreadable(e.to_string()))?;
        if let Some(&id) = self.seen.get(&canonical) {
            return Ok(id);
        }
        let bytes = std::fs::read(&real).map_err(|e| LoadError::Unreadable(e.to_string()))?;
        if bytes.len() > qb64rust_base::MAX_SOURCE_LEN {
            return Err(LoadError::Unreadable(format!(
                "larger than {} bytes",
                qb64rust_base::MAX_SOURCE_LEN
            )));
        }
        let id = map.add(shown.to_string_lossy().into_owned(), bytes);
        self.seen.insert(canonical, id);
        self.dirs
            .insert(id, real.parent().map_or_else(PathBuf::new, Path::to_path_buf));
        Ok(id)
    }
}

/// Lexes, parses and checks a file that includes no other (`$INCLUDE` finds no file). `name` is how the file is
/// shown in diagnostics and `#line` directives.
pub fn frontend(name: &str, bytes: Vec<u8>) -> Frontend {
    frontend_with(name, bytes, |_| Box::new(NoLoader))
}

/// [`frontend`] with included files found by the loader `loader` makes for the main file's id. The checks that
/// read the disk are left out: [`frontend_at`] makes them.
pub fn frontend_with(name: &str, bytes: Vec<u8>, loader: impl FnOnce(FileId) -> Box<dyn Loader>) -> Frontend {
    frontend_at(name, bytes, loader, None)
}

/// [`frontend_with`] for the main file at `main_path` on disk, if given: then the checks of [`check_program`] that
/// read the disk are made too, as for a compile run.
pub fn frontend_at(
    name: &str,
    bytes: Vec<u8>,
    loader: impl FnOnce(FileId) -> Box<dyn Loader>,
    main_path: Option<&Path>,
) -> Frontend {
    let mut map = SourceMap::new();
    let file = map.add(name, bytes);
    let mut loader = loader(file);
    let parsed = parse_settled(&mut map, file, loader.as_mut());
    let (program, diagnostics) = check_program(&map, &parsed, main_path);
    Frontend {
        map,
        file,
        parsed,
        program,
        diagnostics,
    }
}

/// The front end after the parse, for the CLI and the language server's check alike (design D2 of
/// `extension-diagnostics`): `sema`, then, when the main file's path `main_path` is given, the checks that read the
/// disk ([`check_exe_icon`], [`check_embedded_files`]). The diagnostics are the parse's first, then the others.
pub fn check_program(map: &SourceMap, parsed: &ParsedProgram, main_path: Option<&Path>) -> (Program, Diagnostics) {
    // QB64RUST_NO_FOLD=1 turns integer constant folding off (test use only, design D5).
    let fold = std::env::var_os("QB64RUST_NO_FOLD").is_none_or(|v| v != "1");
    let (program, sema_diags) = qb64rust_sema::check_with(map, parsed, fold);
    let mut diagnostics = parsed.diagnostics();
    diagnostics.extend(sema_diags);
    if let Some(path) = main_path {
        check_exe_icon(&program, &mut diagnostics, path);
        check_embedded_files(&program, &mut diagnostics, path);
    }
    (program, diagnostics)
}

/// [`check_program`] for the language server (`qb64rust_lsp::Check`): the diagnostics alone.
pub fn check_for_server(map: &SourceMap, parsed: &ParsedProgram, main_path: Option<&Path>) -> Diagnostics {
    check_program(map, parsed, main_path).1
}

/// Where the icon `$EXEICON` names lies: a name that starts with `./` or `.\` in the folder of the main source
/// file `main_path`, an absolute name as written (design D4 of `m3-sound`). `None` for a program without one.
/// The rest of a `./` name is added to the folder as text, as the old compiler does (`qb64pe.bas` 3853), not
/// joined as a path: a rest that starts with a slash (`.//icons/game.ico`) stays inside the folder.
pub fn exe_icon_path(fe: &Frontend, main_path: &Path) -> Option<PathBuf> {
    icon_path(&fe.program, main_path)
}

fn icon_path(program: &Program, main_path: &Path) -> Option<PathBuf> {
    let icon = program.settings.exe_icon.as_ref()?;
    Some(match qb64rust_sema::ExeIcon::beside_source(&icon.name) {
        Some(rest) => beside(main_path, rest),
        None => PathBuf::from(&icon.name),
    })
}

/// `rest` added as text to the folder of the file `main_path`.
fn beside(main_path: &Path, rest: &str) -> PathBuf {
    let mut path = main_path
        .parent()
        .map_or_else(std::ffi::OsString::new, |p| p.as_os_str().to_os_string());
    // A source file named without a folder lies in the working directory.
    if path.is_empty() {
        path.push(".");
    }
    // Not after a drive alone (`C:game.bas`): a separator there would name the drive's root.
    if !path.to_string_lossy().ends_with(['/', '\\', ':']) {
        path.push(std::path::MAIN_SEPARATOR_STR);
    }
    path.push(rest);
    PathBuf::from(path)
}

/// The check `sema` cannot make, since it reads no file: the icon `$EXEICON` names is there. A missing file is an
/// error at the metacommand's line, counted with the others. It names the file as the program writes it, where
/// the old compiler names the folder it looked in (`DIVERGENCES.md`): no diagnostic holds a folder of the machine.
/// A file that is there but is no icon ([`is_icon`]) is an error too, where both compilers' C++ build fails
/// (`verification\v29_e16_not_an_icon`; design D1 of `cpp-failure-errors`). A file that cannot be read is left to
/// the build.
pub fn check_exe_icon(program: &Program, diagnostics: &mut Diagnostics, main_path: &Path) {
    let (Some(path), Some(icon)) = (icon_path(program, main_path), &program.settings.exe_icon) else {
        return;
    };
    if !path.is_file() {
        let msg = format!("the file `{}` of `$EXEICON` is not found", icon.shown);
        diagnostics.error(icon.span, msg);
    } else if std::fs::read(&path).is_ok_and(|bytes| !is_icon(&bytes)) {
        let msg = format!("the file `{}` of `$EXEICON` is not an icon file", icon.shown);
        diagnostics.error(icon.span, msg);
    }
}

/// The bytes start with an icon directory that `llvm-rc` can read the images of: reserved word 0, type 1, at least
/// one image, each image's 16-byte entry inside the file and its bytes (size at entry offset 8, offset at 12)
/// too. What the images hold is not looked at.
pub fn is_icon(bytes: &[u8]) -> bool {
    let word = |at: usize| bytes.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let dword = |at: usize| {
        bytes
            .get(at..at + 4)
            .map(|b| u64::from(u32::from_le_bytes([b[0], b[1], b[2], b[3]])))
    };
    let (Some(0), Some(1), Some(count)) = (word(0), word(2), word(4)) else {
        return false;
    };
    count > 0
        && (0..usize::from(count)).all(|i| {
            let entry = 6 + 16 * i;
            matches!((dword(entry + 8), dword(entry + 12)),
                (Some(size), Some(offset)) if offset + size <= bytes.len() as u64)
        })
}

/// Where a file a `$EMBED` line names lies, as for the icon ([`exe_icon_path`]; design D3 of `m3-network-embed`).
pub fn embedded_path(file: &qb64rust_sema::EmbeddedFile, main_path: &Path) -> PathBuf {
    match qb64rust_sema::ExeIcon::beside_source(&file.name) {
        Some(rest) => beside(main_path, rest),
        None => PathBuf::from(&file.name),
    }
}

/// The check `sema` cannot make: each file a `$EMBED` line names is a file (measured, `verification\v44_x07`,
/// `x27`: a missing file and a folder). An error at the line that names the file as the program writes it, where
/// the old compiler names the folder it looked in (`DIVERGENCES.md` D-015).
pub fn check_embedded_files(program: &Program, diagnostics: &mut Diagnostics, main_path: &Path) {
    for f in &program.settings.embedded {
        if !embedded_path(f, main_path).is_file() {
            diagnostics.error(f.span, format!("the file `{}` of `$EMBED` is not found", f.shown));
        }
    }
}

/// The handle and bytes of each file a compiled `_EMBEDDED$` names, in file order, for the build
/// (`build::write_embedded`).
pub fn used_embedded_files(fe: &Frontend, main_path: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    fe.program
        .settings
        .embedded
        .iter()
        .filter(|f| f.used)
        .map(|f| {
            let path = embedded_path(f, main_path);
            let bytes =
                std::fs::read(&path).map_err(|e| format!("reading the embedded file {}: {e}", path.display()))?;
            Ok((f.handle.clone(), bytes))
        })
        .collect()
}

/// The IR of a checked program (one without errors). Everything the front end accepts lowers and is emitted.
pub fn lower(fe: &Frontend) -> qb64rust_ir::Program {
    qb64rust_ir::lower(&fe.program)
}

/// `--dump ir`.
pub fn dump_ir(fe: &Frontend) -> String {
    qb64rust_ir::dump(&lower(fe))
}

/// The C++ fragments of a checked program; `#line` directives name each file as the front end was given it or
/// found it. `resolved`: the files of its `DECLARE LIBRARY` blocks.
pub fn emit(fe: &Frontend, resolved: &Resolved) -> qb64rust_codegen_cpp::Fragments {
    emit_ir(fe, &lower(fe), resolved)
}

/// The C++ fragments of `ir`, the IR of `fe`'s program.
pub fn emit_ir(fe: &Frontend, ir: &qb64rust_ir::Program, resolved: &Resolved) -> qb64rust_codegen_cpp::Fragments {
    emit_ir_with(fe, ir, resolved, false)
}

/// [`emit_ir`], with a counter at each source statement for `--coverage` (design D1 of `cws-whole-game`).
pub fn emit_ir_with(
    fe: &Frontend,
    ir: &qb64rust_ir::Program,
    resolved: &Resolved,
    coverage: bool,
) -> qb64rust_codegen_cpp::Fragments {
    let included = fe
        .map
        .files()
        .filter(|&(id, _)| id != fe.file)
        .map(|(id, f)| (id, f.name.clone()))
        .collect();
    let position = |span: qb64rust_base::Span| fe.map.file(span.file).line_col(span.start);
    let coverage = coverage.then(|| qb64rust_codegen_cpp::Coverage {
        files: coverage_files(fe),
        position: &position,
    });
    qb64rust_codegen_cpp::emit_with(ir, &fe.map.file(fe.file).name, &included, resolved, coverage.as_ref())
}

/// Each file of the program by its index, for a coverage record: its path relative to the main file's folder (an
/// included file found elsewhere, in the compiler root, as it was found), with `/` between folders, and the FNV-1a
/// 64 hash of its bytes as 16 hex digits, which `tools\coverage\coverage.py` checks against the file it reads.
pub fn coverage_files(fe: &Frontend) -> Vec<(String, String)> {
    let main_dir = Path::new(&fe.map.file(fe.file).name)
        .parent()
        .map_or_else(PathBuf::new, Path::to_path_buf);
    fe.map
        .files()
        .map(|(_, f)| {
            let name = Path::new(&f.name);
            let rel = name.strip_prefix(&main_dir).unwrap_or(name);
            (
                rel.to_string_lossy().replace('\\', "/"),
                format!("{:016x}", fnv1a64(&f.bytes)),
            )
        })
        .collect()
}

/// The 64-bit FNV-1a hash.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// `--dump cpp`; with `coverage`, the C++ of a `--coverage` build.
pub fn dump_cpp(fe: &Frontend, resolved: &Resolved, coverage: bool) -> String {
    qb64rust_codegen_cpp::dump(&emit_ir_with(fe, &lower(fe), resolved, coverage))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An icon directory with `count` entries, each of `size` bytes at `offset`, padded with zeros to `len` bytes.
    fn icon(kind: u16, count: u16, size: u32, offset: u32, len: usize) -> Vec<u8> {
        let mut b = [0u16.to_le_bytes(), kind.to_le_bytes(), count.to_le_bytes()].concat();
        for _ in 0..count {
            b.extend([0u8; 8]);
            b.extend(size.to_le_bytes());
            b.extend(offset.to_le_bytes());
        }
        b.resize(len.max(b.len()), 0);
        b
    }

    /// What an icon file is for `$EXEICON` (design D1 of `cpp-failure-errors`).
    #[test]
    fn icon_files() {
        assert!(is_icon(&icon(1, 1, 40, 22, 62)));
        assert!(is_icon(&icon(1, 5, 40, 86, 126)));
        assert!(!is_icon(b"this file is text, not an icon\r\n"));
        assert!(!is_icon(&[0, 0, 1, 0]));
        assert!(!is_icon(&icon(2, 1, 40, 22, 62)), "a cursor");
        assert!(!is_icon(&icon(1, 0, 0, 0, 6)), "no image");
        assert!(!is_icon(&icon(1, 1, 40, 22, 61)), "an image past the end");
        // Two entries end at byte 38; cut at 30, the first (an empty image) is inside, the second is not.
        let mut cut = icon(1, 2, 0, 0, 0);
        cut.truncate(30);
        assert!(!is_icon(&cut), "an entry past the end");
    }

    /// The repo's icons are icon files.
    #[test]
    fn repo_icons_are_icons() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for f in [
            "tests/programs/civil-war-strategy/cws.ico",
            "tests/frontend/inc/exeicon.ico",
            "verification/v29_inc/icon.ico",
        ] {
            assert!(is_icon(&std::fs::read(repo.join(f)).unwrap()), "{f}");
        }
        assert!(!is_icon(
            &std::fs::read(repo.join("verification/v29_inc/not_an_icon.ico")).unwrap()
        ));
    }

    /// Where a `./` name of `$EXEICON` lies, by the form of the source file's name.
    #[test]
    fn beside_adds_the_rest_as_text() {
        let sep = std::path::MAIN_SEPARATOR_STR;
        for (main, rest, expected) in [
            ("p.bas", "a.ico", format!(".{sep}a.ico")),
            ("dir/p.bas", "a.ico", format!("dir{sep}a.ico")),
            ("dir/p.bas", "/sub/a.ico", format!("dir{sep}/sub/a.ico")),
            ("C:p.bas", "a.ico", "C:a.ico".to_string()),
            ("C:/p.bas", "a.ico", "C:/a.ico".to_string()),
        ] {
            assert_eq!(beside(Path::new(main), rest), PathBuf::from(expected), "{main} {rest}");
        }
    }
}
