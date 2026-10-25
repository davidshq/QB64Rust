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
        self.add_file(map, &real, &shown)
    }

    fn load_root(&mut self, map: &mut SourceMap, path: &Path) -> Result<FileId, LoadError> {
        let real = self.root.join(path);
        if !real.is_file() {
            return Err(LoadError::NotFound);
        }
        // Named as the old compiler names a library file: by its path under its folder, as the descriptor spells it.
        self.add_file(map, &real, path)
    }

    fn library(&mut self, name: &[u8]) -> Result<Vec<u8>, LoadError> {
        let path = self.root.join(qb64rust_syntax::library::descriptor_path(name));
        if !path.is_file() {
            return Err(LoadError::NotFound);
        }
        std::fs::read(&path).map_err(|e| LoadError::Unreadable(e.to_string()))
    }
}

impl FileLoader {
    /// Reads the file at `real` into the map as `shown`, once per file.
    fn add_file(&mut self, map: &mut SourceMap, real: &Path, shown: &Path) -> Result<FileId, LoadError> {
        let canonical = std::fs::canonicalize(real).map_err(|e| LoadError::Unreadable(e.to_string()))?;
        if let Some(&id) = self.seen.get(&canonical) {
            return Ok(id);
        }
        let bytes = std::fs::read(real).map_err(|e| LoadError::Unreadable(e.to_string()))?;
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

/// [`frontend_with`] for the main file on disk, if given (`disk`: its path and the compiler root): then the checks
/// of [`check_program`] that read the disk are made too, as for a compile run.
pub fn frontend_at(
    name: &str,
    bytes: Vec<u8>,
    loader: impl FnOnce(FileId) -> Box<dyn Loader>,
    disk: Option<(&Path, &Path)>,
) -> Frontend {
    let mut map = SourceMap::new();
    let file = map.add(name, bytes);
    let mut loader = loader(file);
    let parsed = parse_settled(&mut map, file, loader.as_mut());
    let (program, diagnostics) = check_program(&map, &parsed, disk);
    Frontend {
        map,
        file,
        parsed,
        program,
        diagnostics,
    }
}

/// The front end after the parse, for the CLI and the language server's check alike (design D2 of
/// `extension-diagnostics`): `sema`, then, when the main file's path and the compiler root are given (`disk`), the
/// checks that read the disk ([`check_exe_icon`], [`check_embedded_files`]). The diagnostics are the parse's first,
/// then the others.
pub fn check_program(map: &SourceMap, parsed: &ParsedProgram, disk: Option<(&Path, &Path)>) -> (Program, Diagnostics) {
    // QB64RUST_NO_FOLD=1 turns integer constant folding off (test use only, design D5).
    let fold = std::env::var_os("QB64RUST_NO_FOLD").is_none_or(|v| v != "1");
    let (program, sema_diags) = qb64rust_sema::check_with(map, parsed, fold);
    let mut diagnostics = parsed.diagnostics();
    diagnostics.extend(sema_diags);
    if let Some((path, root)) = disk {
        check_exe_icon(&program, &mut diagnostics, path, root);
        check_embedded_files(&program, &mut diagnostics, path, root);
    }
    (program, diagnostics)
}

/// [`check_program`] for the language server (`qb64rust_lsp::Check`): the diagnostics alone, for the main file at
/// `main_path` on disk (none for a document that is not a file) and the compiler root `root`.
pub fn check_for_server(map: &SourceMap, parsed: &ParsedProgram, main_path: Option<&Path>, root: &Path) -> Diagnostics {
    check_program(map, parsed, main_path.map(|m| (m, root))).1
}

/// Where the icon `$EXEICON` names lies ([`named_file`]). `None` for a program without one.
pub fn exe_icon_path(fe: &Frontend, main_path: &Path, root: &Path) -> Option<PathBuf> {
    icon_path(&fe.program, main_path, root)
}

fn icon_path(program: &Program, main_path: &Path, root: &Path) -> Option<PathBuf> {
    let icon = program.settings.exe_icon.as_ref()?;
    Some(named_file(&icon.name, main_path, root))
}

/// Where a file `$EXEICON` or `$EMBED` names lies: a name that starts with `./` or `.\` in the folder of the main
/// source file `main_path` (design D4 of `m3-sound`), an absolute name as written, and any other relative name under
/// the compiler root `root`, as the old compiler looks in its own folder (design D3 of `m4-devices-continue-paths`);
/// joined onto the root, a name with one leading slash names the root of the root's drive, the old compiler's
/// reading. The rest of a `./` name is added to the folder as text, as the old compiler does (`qb64pe.bas` 3853),
/// not joined as a path: a rest that starts with a slash (`.//icons/game.ico`) stays inside the folder. (`sema`
/// keeps a drive with no slash after it, `C:icon.ico`, marked.)
fn named_file(name: &str, main_path: &Path, root: &Path) -> PathBuf {
    match qb64rust_sema::ExeIcon::beside_source(name) {
        Some(rest) => beside(main_path, rest),
        None if Path::new(name).is_absolute() => PathBuf::from(name),
        None => root.join(name),
    }
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
pub fn check_exe_icon(program: &Program, diagnostics: &mut Diagnostics, main_path: &Path, root: &Path) {
    let (Some(path), Some(icon)) = (icon_path(program, main_path, root), &program.settings.exe_icon) else {
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

/// Where a file a `$EMBED` line names lies, as for the icon ([`named_file`]; design D3 of `m3-network-embed`).
pub fn embedded_path(file: &qb64rust_sema::EmbeddedFile, main_path: &Path, root: &Path) -> PathBuf {
    named_file(&file.name, main_path, root)
}

/// The check `sema` cannot make: each file a `$EMBED` line names is a file (measured, `verification\v44_x07`,
/// `x27`: a missing file and a folder). An error at the line that names the file as the program writes it, where
/// the old compiler names the folder it looked in (`DIVERGENCES.md` D-015).
pub fn check_embedded_files(program: &Program, diagnostics: &mut Diagnostics, main_path: &Path, root: &Path) {
    for f in &program.settings.embedded {
        if !embedded_path(f, main_path, root).is_file() {
            diagnostics.error(f.span, format!("the file `{}` of `$EMBED` is not found", f.shown));
        }
    }
}

/// The handle and bytes of each file a compiled `_EMBEDDED$` names, in file order, for the build
/// (`build::write_embedded`).
pub fn used_embedded_files(fe: &Frontend, main_path: &Path, root: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    fe.program
        .settings
        .embedded
        .iter()
        .filter(|f| f.used)
        .map(|f| {
            let path = embedded_path(f, main_path, root);
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
    emit_ir_with(fe, ir, resolved, false, None)
}

/// [`emit_ir`], with a counter at each source statement for `--coverage` (design D1 of `cws-whole-game`), and the
/// folder put before a relative file name in a log statement's entry (`log_folder`, the working folder with its
/// separator for a build; design D2 of `m4-handlers-logging-checking`).
pub fn emit_ir_with(
    fe: &Frontend,
    ir: &qb64rust_ir::Program,
    resolved: &Resolved,
    coverage: bool,
    log_folder: Option<&str>,
) -> qb64rust_codegen_cpp::Fragments {
    emit_ir_all(fe, ir, resolved, coverage, log_folder, false).0
}

/// [`emit_ir_with`], and with `debug` the debug symbol file of the same emitter.
fn emit_ir_all(
    fe: &Frontend,
    ir: &qb64rust_ir::Program,
    resolved: &Resolved,
    coverage: bool,
    log_folder: Option<&str>,
    debug: bool,
) -> (qb64rust_codegen_cpp::Fragments, Option<qb64rust_codegen_cpp::DebugInfo>) {
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
        builtins: coverage_builtins(fe),
    });
    let source_name = &fe.map.file(fe.file).name;
    if debug {
        let (fragments, info) = qb64rust_codegen_cpp::emit_debug(
            ir,
            source_name,
            &included,
            resolved,
            coverage.as_ref(),
            log_folder,
            &debug_files(fe),
        );
        return (fragments, Some(info));
    }
    let fragments =
        qb64rust_codegen_cpp::emit_with(ir, source_name, &included, resolved, coverage.as_ref(), log_folder);
    (fragments, None)
}

/// The log folder of [`emit_ir_with`] for a build from `dir`: the folder with a separator after it unless it ends
/// in one (a drive's root, `C:\`), as the old compiler's `_STARTDIR$` gives it (libqb's `func__cwd`). A name that
/// is not ASCII is written as UTF-8, where the old compiler has the bytes of the system's code page
/// (`DIVERGENCES.md`).
pub fn log_folder(dir: &Path) -> String {
    let mut folder = dir.display().to_string();
    if !folder.ends_with(['/', '\\']) {
        folder.push(std::path::MAIN_SEPARATOR);
    }
    folder
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

/// Each name token of the program's trees that stands for a built-in `sema` compiles
/// ([`qb64rust_sema::builtins::coverage_name`]), for the names of a coverage record (design D1 of
/// `m4-coverage-review`). A name after a member's dot (`r.left`) is the member's, and the name of one of the
/// program's own SUBs and FUNCTIONs is the procedure's (`FUNCTION beep`, `tests\corpus\slice\s24_proc_names`).
pub fn coverage_builtins(fe: &Frontend) -> Vec<(qb64rust_base::Span, &'static str)> {
    use qb64rust_syntax::SyntaxKind;
    let own = |name: &str| {
        let bare = name.strip_suffix('$').unwrap_or(name);
        fe.program.procs.iter().any(|p| p.name == bare)
    };
    let mut out = Vec::new();
    for tree in &fe.parsed.trees {
        let bytes = &fe.map.file(tree.file).bytes;
        let mut after_dot = false;
        for tok in tree.root().tokens() {
            if tok.kind == SyntaxKind::Ident && !after_dot {
                let text = &bytes[tok.span.start as usize..tok.span.end as usize];
                if let Some(name) = qb64rust_sema::builtins::coverage_name(text).filter(|n| !own(n)) {
                    out.push((tok.span, name));
                }
            }
            if tok.kind != SyntaxKind::Whitespace {
                after_dot = tok.kind == SyntaxKind::Dot;
            }
        }
    }
    out
}

/// The 64-bit FNV-1a hash.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// `--dump cpp`; with `coverage`, the C++ of a `--coverage` build.
pub fn dump_cpp(fe: &Frontend, resolved: &Resolved, coverage: bool) -> String {
    qb64rust_codegen_cpp::dump(&emit_ir_with(fe, &lower(fe), resolved, coverage, None))
}

/// Each file of the program by its index, for the debug symbol file: its path as the `#line` directives name it (as
/// the front end was given it or found it) and the FNV-1a 64 hash of its bytes, as in a coverage record.
pub fn debug_files(fe: &Frontend) -> Vec<qb64rust_codegen_cpp::DebugFile> {
    fe.map
        .files()
        .map(|(_, f)| qb64rust_codegen_cpp::DebugFile {
            path: f.name.clone(),
            hash: format!("{:016x}", fnv1a64(&f.bytes)),
        })
        .collect()
}

/// [`emit_ir_with`], and the debug symbol file of the program (design D3 of `m5-debug-build-symbols`): its fragments
/// are [`emit_ir_with`]'s.
pub fn emit_ir_debug(
    fe: &Frontend,
    ir: &qb64rust_ir::Program,
    resolved: &Resolved,
    coverage: bool,
    log_folder: Option<&str>,
) -> (qb64rust_codegen_cpp::Fragments, qb64rust_codegen_cpp::DebugInfo) {
    let (fragments, info) = emit_ir_all(fe, ir, resolved, coverage, log_folder, true);
    (fragments, info.expect("asked for"))
}

/// `--dump debug-info`: the debug symbol file of a debug build, with the files' paths as given to the compiler.
pub fn dump_debug_info(fe: &Frontend, resolved: &Resolved) -> String {
    emit_ir_debug(fe, &lower(fe), resolved, false, None).1.to_json()
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

    /// Where a file of `$EXEICON` or `$EMBED` lies by the form of its name (design D3 of
    /// `m4-devices-continue-paths`): beside the source, as written, or under the compiler root; one leading slash
    /// is the root of the compiler root's drive.
    #[test]
    fn named_file_by_the_form_of_the_name() {
        let sep = std::path::MAIN_SEPARATOR_STR;
        let (main, root) = (Path::new("dir/p.bas"), Path::new("C:/root/does/not/exist"));
        assert_eq!(
            named_file("./a.ico", main, root),
            PathBuf::from(format!("dir{sep}a.ico"))
        );
        let elsewhere = "D:/does/not/exist/a.ico";
        assert_eq!(named_file(elsewhere, main, root), PathBuf::from(elsewhere));
        let under = PathBuf::from("C:/root/does/not/exist/icons/a.ico");
        assert_eq!(named_file("icons/a.ico", main, root), under);
        if cfg!(windows) {
            assert_eq!(
                named_file("/does/not/exist/a.ico", main, root),
                PathBuf::from("C:/does/not/exist/a.ico")
            );
            let slashed = r"\does\not\exist\a.ico";
            assert_eq!(named_file(slashed, main, root), PathBuf::from(format!("C:{slashed}")));
        }
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

    /// The log folder takes one separator, none at a drive's root, which already ends in one.
    #[test]
    fn log_folder_ends_in_one_separator() {
        let sep = std::path::MAIN_SEPARATOR;
        assert_eq!(log_folder(Path::new("C:/work")), format!("C:/work{sep}"));
        assert_eq!(log_folder(Path::new("C:\\")), "C:\\");
        assert_eq!(log_folder(Path::new("/")), "/");
    }
}
