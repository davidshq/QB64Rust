//! The compiler pipeline as a library, so that tests can run each stage; `main.rs` is the command line.

pub mod build;

use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_sema::Program;
use qb64rust_syntax::{LoadError, Loader, NoLoader, ParsedProgram, include_name, parse};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub use qb64rust_base::STACK_SIZE;

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

    /// Each diagnostic rendered as `<file>:<line>:<col>: error: <message>`, one per line, in source order (the
    /// main file first, then the included files in the order they were loaded).
    pub fn render_diagnostics(&self) -> String {
        let mut list: Vec<_> = self.diagnostics.list().to_vec();
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

/// [`frontend`] with included files found by the loader `loader` makes for the main file's id.
pub fn frontend_with(name: &str, bytes: Vec<u8>, loader: impl FnOnce(FileId) -> Box<dyn Loader>) -> Frontend {
    let mut map = SourceMap::new();
    let file = map.add(name, bytes);
    let mut loader = loader(file);
    let parsed = parse(&mut map, file, loader.as_mut());
    // QB64RUST_NO_FOLD=1 turns integer constant folding off (test use only, design D5).
    let fold = std::env::var_os("QB64RUST_NO_FOLD").is_none_or(|v| v != "1");
    let (program, sema_diags) = qb64rust_sema::check_with(&map, &parsed, fold);
    let mut diagnostics = parsed.diagnostics();
    diagnostics.extend(sema_diags);
    Frontend {
        map,
        file,
        parsed,
        program,
        diagnostics,
    }
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
/// found it.
pub fn emit(fe: &Frontend) -> qb64rust_codegen_cpp::Fragments {
    let included = fe
        .map
        .files()
        .filter(|&(id, _)| id != fe.file)
        .map(|(id, f)| (id, f.name.clone()))
        .collect();
    qb64rust_codegen_cpp::emit(&lower(fe), &fe.map.file(fe.file).name, &included)
}

/// `--dump cpp`.
pub fn dump_cpp(fe: &Frontend) -> String {
    qb64rust_codegen_cpp::dump(&emit(fe))
}
