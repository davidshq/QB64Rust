//! One parse of a program (design D2): the main document and the files it includes, from the open documents first
//! and the disk second, with what is needed to answer requests about them.

use crate::encoding::{Columns, Encoding};
use crate::uri::{from_path, path_key, to_path};
use lsp_types::{DiagnosticSeverity, Range, Uri};
use qb64rust_base::{FileId, MAX_SOURCE_LEN, Severity, SourceMap, Span};
use qb64rust_syntax::{LoadError, Loader, ParsedProgram, include_name, parse};
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

    /// The editor range of a span.
    pub fn range(&self, span: Span) -> Range {
        let file = self.map.file(span.file);
        let columns = &self.info(span.file).columns;
        Range::new(columns.position(file, span.start), columns.position(file, span.end))
    }

    /// The parser's diagnostics of every file this parse covered, each file once (an empty list for a file
    /// without errors), in [`FileId`] order. A file included twice reports an error once.
    pub fn diagnostics(&self) -> Vec<(FileId, Vec<lsp_types::Diagnostic>)> {
        let mut per_file: Vec<Vec<lsp_types::Diagnostic>> = self.files.iter().map(|_| Vec::new()).collect();
        let mut seen = std::collections::HashSet::new();
        for tree in &self.parsed.trees {
            for d in tree.diagnostics.list() {
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
                    message: format!("{mark}{}", d.message),
                    ..lsp_types::Diagnostic::default()
                });
            }
        }
        per_file
            .into_iter()
            .enumerate()
            .map(|(i, d)| (FileId(qb64rust_base::to_u32(i)), d))
            .collect()
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
) -> Analysis {
    let mut map = SourceMap::new();
    let path = to_path(&main.uri);
    let name = path
        .as_ref()
        .map_or_else(|| main.uri.as_str().to_string(), |p| p.to_string_lossy().into_owned());
    let id = map.add(name, main.bytes.to_vec());
    let mut loader = DocLoader {
        open,
        root,
        default,
        files: Vec::new(),
        dirs: Vec::new(),
        seen: HashMap::new(),
    };
    loader.record(
        &map,
        id,
        main.uri.clone(),
        key.to_string(),
        path.as_deref(),
        main.encoding,
    );
    let parsed = parse(&mut map, id, &mut loader);
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
}

impl DocLoader<'_> {
    fn record(&mut self, map: &SourceMap, id: FileId, uri: Uri, key: String, path: Option<&Path>, enc: Encoding) {
        assert_eq!(
            id.0 as usize,
            self.files.len(),
            "files are recorded in the order they are added"
        );
        let columns = enc.columns(map.file(id));
        self.files.push(FileInfo { uri, key, columns });
        self.dirs.push(path.and_then(Path::parent).map(Path::to_path_buf));
    }
}

impl Loader for DocLoader<'_> {
    fn load(&mut self, map: &mut SourceMap, from: FileId, path: &[u8]) -> Result<FileId, LoadError> {
        let rel = include_name(path);
        let candidates = if rel.is_absolute() {
            vec![rel]
        } else {
            let from_dir = self.dirs[from.0 as usize].as_ref().map(|d| d.join(&rel));
            from_dir.into_iter().chain([self.root.join(&rel)]).collect()
        };
        for candidate in candidates {
            let key = path_key(&candidate);
            if let Some(&id) = self.seen.get(&key) {
                return Ok(id);
            }
            let (uri, bytes, enc) = if let Some(f) = self.open.get(&key) {
                (f.uri.clone(), f.bytes.to_vec(), f.encoding)
            } else if candidate.is_file() {
                let bytes = std::fs::read(&candidate).map_err(|e| LoadError::Unreadable(e.to_string()))?;
                (from_path(&candidate), bytes, self.default)
            } else {
                continue;
            };
            if bytes.len() > MAX_SOURCE_LEN {
                return Err(LoadError::Unreadable(format!("larger than {MAX_SOURCE_LEN} bytes")));
            }
            let id = map.add(candidate.to_string_lossy().into_owned(), bytes);
            self.seen.insert(key.clone(), id);
            self.record(map, id, uri, key, Some(&candidate), enc);
            return Ok(id);
        }
        Err(LoadError::NotFound)
    }
}
