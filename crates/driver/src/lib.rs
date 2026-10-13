//! The compiler pipeline as a library, so that tests can run each stage; `main.rs` is the command line.

pub mod build;
pub mod report;

use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_sema::Program;
use qb64rust_syntax::{LoadError, Loader, NoLoader, ParsedProgram, include_name, parse_settled};
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

/// [`frontend`] with included files found by the loader `loader` makes for the main file's id.
pub fn frontend_with(name: &str, bytes: Vec<u8>, loader: impl FnOnce(FileId) -> Box<dyn Loader>) -> Frontend {
    let mut map = SourceMap::new();
    let file = map.add(name, bytes);
    let mut loader = loader(file);
    let parsed = parse_settled(&mut map, file, loader.as_mut());
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

/// Where the icon `$EXEICON` names lies: a name that starts with `./` or `.\` in the folder of the main source
/// file `main_path`, an absolute name as written (design D4 of `m3-sound`). `None` for a program without one.
/// The rest of a `./` name is added to the folder as text, as the old compiler does (`qb64pe.bas` 3853), not
/// joined as a path: a rest that starts with a slash (`.//icons/game.ico`) stays inside the folder.
pub fn exe_icon_path(fe: &Frontend, main_path: &Path) -> Option<PathBuf> {
    let icon = fe.program.settings.exe_icon.as_ref()?;
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
pub fn check_exe_icon(fe: &mut Frontend, main_path: &Path) {
    let Some(path) = exe_icon_path(fe, main_path) else {
        return;
    };
    if !path.is_file()
        && let Some(icon) = &fe.program.settings.exe_icon
    {
        let msg = format!("the file `{}` of `$EXEICON` is not found", icon.shown);
        fe.diagnostics.error(icon.span, msg);
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
    emit_ir(fe, &lower(fe))
}

/// The C++ fragments of `ir`, the IR of `fe`'s program.
pub fn emit_ir(fe: &Frontend, ir: &qb64rust_ir::Program) -> qb64rust_codegen_cpp::Fragments {
    let included = fe
        .map
        .files()
        .filter(|&(id, _)| id != fe.file)
        .map(|(id, f)| (id, f.name.clone()))
        .collect();
    qb64rust_codegen_cpp::emit(ir, &fe.map.file(fe.file).name, &included)
}

/// `--dump cpp`.
pub fn dump_cpp(fe: &Frontend) -> String {
    qb64rust_codegen_cpp::dump(&emit(fe))
}

#[cfg(test)]
mod tests {
    use super::*;

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
