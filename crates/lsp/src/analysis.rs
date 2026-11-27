//! One parse of a program (design D2): the main document and the files it includes, from the open documents first
//! and the disk second, with what is needed to answer requests about them.

use crate::encoding::{Columns, Encoding};
use crate::uri::{from_path, path_key, to_path};
use lsp_types::{DiagnosticSeverity, Range, Uri};
use qb64rust_base::{Diagnostic, FileId, MAX_SOURCE_LEN, Severity, SourceMap, Span};
use qb64rust_syntax::program::Tree;
use qb64rust_syntax::{LoadError, Loader, ParsedProgram, include_candidates, parse_settled_on};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// An open document as the parser reads it: the bytes of its text in its encoding.
#[derive(Clone, Debug)]
pub struct OpenFile {
    pub uri: Uri,
    pub bytes: Arc<[u8]>,
    pub encoding: Encoding,
}

/// One file of a parse: where it is, and how its byte columns map to the editor's.
pub struct FileInfo {
    pub uri: Uri,
    pub key: String,
    pub columns: Columns,
    /// The encoding the client names for the file when it is an open document; `None` for a file read from disk.
    pub open: Option<Encoding>,
}

/// The parse of one program.
pub struct Analysis {
    pub map: SourceMap,
    pub parsed: ParsedProgram,
    /// Indexed by [`FileId`]: the main file first, then each included file in the order it was loaded.
    pub files: Vec<FileInfo>,
}

impl Analysis {
    pub fn info(&self, file: FileId) -> &FileInfo {
        &self.files[file.0 as usize]
    }

    /// The file of the document `key` in this parse, when the parse covers it.
    pub fn file_of(&self, key: &str) -> Option<FileId> {
        self.files
            .iter()
            .position(|f| f.key == key)
            .map(|i| FileId(qb64rust_base::to_u32(i)))
    }

    /// The first tree of `file`: for the main file its own (tree 0), for an included file its first inclusion.
    pub fn tree(&self, file: FileId) -> Option<&Tree> {
        self.parsed.trees.iter().find(|t| t.file == file)
    }

    /// The editor range of a span.
    pub fn range(&self, span: Span) -> Range {
        let file = self.map.file(span.file);
        let columns = &self.info(span.file).columns;
        Range::new(columns.position(file, span.start), columns.position(file, span.end))
    }

    /// The parser's diagnostics, as [`Analysis::lsp_diagnostics`] publishes them.
    pub fn parse_diagnostics(&self) -> Vec<(FileId, Vec<lsp_types::Diagnostic>)> {
        let list: Vec<Diagnostic> = self
            .parsed
            .trees
            .iter()
            .flat_map(|t| t.diagnostics.list().iter().cloned())
            .collect();
        self.lsp_diagnostics(&list)
    }

    /// The diagnostics `list` (the parser's, or a check's) for every file this parse covered, each file once (an
    /// empty list for a file without any), in [`FileId`] order; within a file in the order of `list`. A diagnostic
    /// given twice (a file included twice) is published once.
    pub fn lsp_diagnostics(&self, list: &[Diagnostic]) -> Vec<(FileId, Vec<lsp_types::Diagnostic>)> {
        let mut per_file: Vec<Vec<lsp_types::Diagnostic>> = self.files.iter().map(|_| Vec::new()).collect();
        let mut seen = std::collections::HashSet::new();
        for d in list {
            if !seen.insert((d.span, d.message.clone())) {
                continue;
            }
            let mark = if d.unsupported { "not supported yet: " } else { "" };
            per_file[d.span.file.0 as usize].push(lsp_types::Diagnostic {
                range: self.range(d.span),
                severity: Some(match d.severity {
                    Severity::Error => DiagnosticSeverity::ERROR,
                    Severity::Warning => DiagnosticSeverity::WARNING,
                }),
                source: Some("qb64rust".to_string()),
                // A lint warning's code (spec `compiler/lints`, "Codes"), which the editor shows beside the message.
                code: d.code.map(|c| lsp_types::NumberOrString::String(c.to_string())),
                // The old compiler's wording on a line of its own, as the CLI's note (`m3-old-wording-notes`).
                message: match d.old_wording() {
                    Some(old) => format!("{mark}{}\nQB64pe: {old}", d.message),
                    None => format!("{mark}{}", d.message),
                },
                ..lsp_types::Diagnostic::default()
            });
        }
        for (i, diags) in per_file.iter_mut().enumerate() {
            if let Some(d) = self.open_as_another_encoding(FileId(qb64rust_base::to_u32(i))) {
                diags.insert(0, d);
            }
        }
        per_file
            .into_iter()
            .enumerate()
            .map(|(i, d)| (FileId(qb64rust_base::to_u32(i)), d))
            .collect()
    }

    /// The warning for a UTF-8 file that the client has open in a single-byte encoding, at its first line (spec
    /// `editor/language-server`, "A UTF-8 file open in another encoding"; design D10 of `utf8-source`). The parse
    /// still reads the bytes that encoding gives, which for `cp437` are the file's own.
    fn open_as_another_encoding(&self, file: FileId) -> Option<lsp_types::Diagnostic> {
        let Some(open @ Encoding::Table(..)) = self.info(file).open else {
            return None;
        };
        let tree = self.tree(file)?;
        let found = qb64rust_syntax::encoding::file_encoding(&self.map, file, tree.encoding);
        if !found.is_utf8() {
            return None;
        }
        let how = match found.by {
            qb64rust_syntax::encoding::DeclaredBy::Line(_) => "by its `'$ENCODING:UTF8` line",
            _ => "by `encoding = \"utf-8\"` in `qb64rust.toml`",
        };
        let (start, end) = self.map.file(file).line_content(0).unwrap_or((0, 0));
        Some(lsp_types::Diagnostic {
            range: self.range(Span::new(file, start, end)),
            severity: Some(DiagnosticSeverity::WARNING),
            source: Some("qb64rust".to_string()),
            message: format!(
                "this file is declared UTF-8 {how}, but it is open as `{}`: reopen it with the encoding UTF-8 \
                 (Reopen with Encoding)",
                open.name()
            ),
            ..lsp_types::Diagnostic::default()
        })
    }
}

/// Parses the program whose main file is `main` (key `key`). Included files are looked up as the compiler does
/// (the including file's folder, then `root`); at each place, an open document of that name is used before the
/// disk. A file read from disk is taken to be in `default`.
pub fn analyze(
    key: &str,
    main: &OpenFile,
    open: &HashMap<String, OpenFile>,
    root: &Path,
    default: Encoding,
    project: crate::ProjectEncoding,
) -> Analysis {
    let mut map = SourceMap::new();
    let path = to_path(&main.uri);
    let name = path
        .as_ref()
        .map_or_else(|| main.uri.as_str().to_string(), |p| p.to_string_lossy().into_owned());
    let id = map.add(name, main.bytes.to_vec());
    // The settings file's encoding, for the files in its folder and below it, as the compiler gives it (design D2
    // of `utf8-source`).
    let project = path
        .as_deref()
        .and_then(project)
        .and_then(|(folder, e)| Some((std::fs::canonicalize(folder).ok()?, e)));
    let mut loader = DocLoader {
        open,
        root,
        default,
        files: Vec::new(),
        dirs: Vec::new(),
        seen: HashMap::new(),
        project,
    };
    loader.set_encoding(&mut map, id, path.as_deref());
    loader.record(
        &map,
        id,
        main.uri.clone(),
        key.to_string(),
        path.as_deref(),
        main.encoding,
    );
    let parsed = parse_settled_on(qb64rust_base::Platform::host(), &mut map, id, &mut loader);
    loader.record_builtins(&map, map.files().count());
    Analysis {
        map,
        parsed,
        files: loader.files,
    }
}

/// The [`Loader`] of a parse (design D2).
struct DocLoader<'a> {
    open: &'a HashMap<String, OpenFile>,
    root: &'a Path,
    default: Encoding,
    files: Vec<FileInfo>,
    /// The folder of each file, by [`FileId`]; `None` for a document that is not a file (`untitled:`).
    dirs: Vec<Option<PathBuf>>,
    /// Key -> file, for files loaded already (the main file is not among them, as in the driver's loader).
    seen: HashMap<String, FileId>,
    /// The settings file's canonical folder and the encoding it gives ([`DocLoader::set_encoding`]).
    project: Option<(PathBuf, qb64rust_base::SourceEncoding)>,
}

impl DocLoader<'_> {
    fn record(&mut self, map: &SourceMap, id: FileId, uri: Uri, key: String, path: Option<&Path>, enc: Encoding) {
        assert_eq!(
            id.0 as usize,
            self.files.len(),
            "files are recorded in the order they are added"
        );
        let columns = enc.columns(map.file(id));
        let open = self.open.get(&key).map(|f| f.encoding);
        self.files.push(FileInfo {
            uri,
            key,
            columns,
            open,
        });
        self.dirs.push(path.and_then(Path::parent).map(Path::to_path_buf));
    }

    /// Records the files the parser added to the map itself, up to the file `end` (QB64pe's built-in files: the
    /// constants of `$COLOR`, `afterlastline.bm`). Each is named under the compiler root, as the CLI names it
    /// relative to it; it is no file of the editor's.
    fn record_builtins(&mut self, map: &SourceMap, end: usize) {
        while self.files.len() < end {
            let id = FileId(qb64rust_base::to_u32(self.files.len()));
            let path = self.root.join(&map.file(id).name);
            self.record(map, id, from_path(&path), path_key(&path), None, Encoding::DEFAULT);
        }
    }
}

impl DocLoader<'_> {
    /// Gives the file `id` at `path` the settings file's encoding when it is in that file's folder or below it.
    fn set_encoding(&self, map: &mut SourceMap, id: FileId, path: Option<&Path>) {
        let Some((folder, encoding)) = &self.project else {
            return;
        };
        let dir = path.and_then(Path::parent).and_then(|d| std::fs::canonicalize(d).ok());
        if dir.is_some_and(|d| d.starts_with(folder)) {
            map.set_project_encoding(id, Some(*encoding));
        }
    }
}

impl Loader for DocLoader<'_> {
    fn load(&mut self, map: &mut SourceMap, from: FileId, path: &[u8]) -> Result<FileId, LoadError> {
        let dir = self.dirs[from.0 as usize].as_deref();
        for (candidate, _) in include_candidates(path, dir.map(|d| (d, d)), self.root) {
            if let Some(id) = self.take(map, &candidate, None)? {
                return Ok(id);
            }
        }
        Err(LoadError::NotFound)
    }

    fn load_root(&mut self, map: &mut SourceMap, path: &Path) -> Result<FileId, LoadError> {
        // Named as the old compiler names a library file: by its path under the compiler root.
        let shown = path.to_string_lossy().into_owned();
        self.take(map, &self.root.join(path), Some(shown))?
            .ok_or(LoadError::NotFound)
    }

    fn library(&mut self, name: &[u8]) -> Result<Vec<u8>, LoadError> {
        let path = self.root.join(qb64rust_syntax::library::descriptor_path(name));
        if let Some(f) = self.open.get(&path_key(&path)) {
            return Ok(f.bytes.to_vec());
        }
        if !path.is_file() {
            return Err(LoadError::NotFound);
        }
        std::fs::read(&path).map_err(|e| LoadError::Unreadable(e.to_string()))
    }
}

impl DocLoader<'_> {
    /// Loads `candidate` from the open documents or the disk into the map, as `shown` (its own path when `None`);
    /// `Ok(None)` when there is no such file.
    fn take(
        &mut self,
        map: &mut SourceMap,
        candidate: &Path,
        shown: Option<String>,
    ) -> Result<Option<FileId>, LoadError> {
        let key = path_key(candidate);
        if let Some(&id) = self.seen.get(&key) {
            return Ok(Some(id));
        }
        let (uri, bytes, enc) = if let Some(f) = self.open.get(&key) {
            (f.uri.clone(), f.bytes.to_vec(), f.encoding)
        } else if candidate.is_file() {
            let bytes = std::fs::read(candidate).map_err(|e| LoadError::Unreadable(e.to_string()))?;
            (from_path(candidate), bytes, self.default)
        } else {
            return Ok(None);
        };
        if bytes.len() > MAX_SOURCE_LEN {
            return Err(LoadError::Unreadable(format!("larger than {MAX_SOURCE_LEN} bytes")));
        }
        self.record_builtins(map, map.files().count());
        let id = map.add(shown.unwrap_or_else(|| candidate.to_string_lossy().into_owned()), bytes);
        self.set_encoding(map, id, Some(candidate));
        self.seen.insert(key.clone(), id);
        self.record(map, id, uri, key, Some(candidate), enc);
        Ok(Some(id))
    }
}
