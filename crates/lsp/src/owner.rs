//! The program an open document belongs to (design D7 of `include-file-program`; spec `editor/language-server`,
//! "Program of an included file"): the `main` entry of the settings file, else the open `.bas` document whose
//! program includes it, else a `.bas` file on disk whose program does. The lookup runs on the worker, since it
//! reads the disk and parses.
//!
//! The file's extension does not decide whether a file is a program: a `.bas` file may be included by another. It
//! decides only that a program found among the open documents or on disk is a `.bas` file.

use crate::analysis::{OpenFile, analyze};
use crate::encoding::Encoding;
use crate::server::SETTINGS_FILE;
use crate::uri::{from_path, path_key, to_path};
use lsp_types::Uri;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// How many files the disk search reads at most, over all its folders.
pub const MAX_FILES_READ: usize = 500;

/// How many programs found on disk the search parses at most.
pub const MAX_ANALYZED: usize = 20;

/// Where a document's program was found (the `source` of `qb64rust/program`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// The `main` entry of the `[project]` table of the settings file.
    Settings,
    /// An open `.bas` document.
    Open,
    /// A `.bas` file on disk.
    Disk,
    /// None: the document is its own program.
    None,
}

impl Source {
    /// The name the notification gives it.
    pub fn name(self) -> &'static str {
        match self {
            Source::Settings => "settings",
            Source::Open => "open",
            Source::Disk => "disk",
            Source::None => "none",
        }
    }
}

/// The program of a document whose program is another file: its main file's key and URI, and where it was found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Owner {
    pub main: String,
    pub uri: Uri,
    pub source: Source,
}

/// What a lookup reads besides the disk.
pub struct Lookup<'a> {
    /// The document's key and URI.
    pub doc: &'a str,
    pub uri: &'a Uri,
    /// The open documents, by key.
    pub open: &'a HashMap<String, OpenFile>,
    /// The open `.bas` documents that are their own program, the one opened first first.
    pub programs: &'a [String],
    /// The settings file's `main` for the document, when it has one that exists.
    pub settings_main: Option<PathBuf>,
    /// The client's workspace folders.
    pub folders: &'a [PathBuf],
    /// The compiler root, where included files are looked up after the including file's folder.
    pub root: &'a Path,
    /// The encoding of files read from disk.
    pub default: Encoding,
}

/// The program of the document, `None` when it is its own program.
pub fn find_owner(l: &Lookup) -> Option<Owner> {
    // 1. The settings file. A `main` that names the document makes it its own program.
    if let Some(main) = &l.settings_main {
        let key = path_key(main);
        if key == l.doc {
            return None;
        }
        if let Some(file) = program_file(l, main, &key)
            && covers(l, &key, &file)
        {
            return Some(Owner {
                main: key,
                uri: file.uri,
                source: Source::Settings,
            });
        }
    }
    // 2. The open `.bas` documents. Of those whose program includes the document, one that another of them
    // includes is passed over, so that a chain of includes gives its top file.
    let mut found: Vec<(&String, Vec<String>)> = Vec::new();
    for key in l.programs.iter().filter(|k| k.as_str() != l.doc) {
        let Some(file) = l.open.get(key) else {
            continue;
        };
        let files = program_files(l, key, file);
        if files.iter().any(|f| f == l.doc) {
            found.push((key, files));
        }
    }
    let top = found
        .iter()
        .find(|(k, _)| !found.iter().any(|(other, files)| other != k && files.contains(k)))
        .or(found.first());
    if let Some((key, _)) = top {
        return Some(Owner {
            main: (*key).clone(),
            uri: l.open[*key].uri.clone(),
            source: Source::Open,
        });
    }
    // 3. The disk.
    let path = to_path(l.uri)?;
    on_disk(l, &path)
}

/// One file of a folder searched on disk: its path, its name and bytes in lower case, and its bytes.
struct Found {
    path: PathBuf,
    name: Vec<u8>,
    lower: Vec<u8>,
    bytes: Vec<u8>,
}

/// Step 3 of the lookup: folder by folder from the document's upward, up to [`upper_limit`].
fn on_disk(l: &Lookup, path: &Path) -> Option<Owner> {
    let dir = path.parent()?;
    let limit = upper_limit(dir, l.folders);
    let mut names: Vec<Vec<u8>> = vec![lower_name(path)?];
    let (mut read, mut analyzed) = (0, 0);
    for folder in dir.ancestors() {
        let mut files = Vec::new();
        for p in basic_files(folder) {
            if path_key(&p) == l.doc {
                continue;
            }
            if read == MAX_FILES_READ {
                eprintln!(
                    "qb64rust: the search for the program of {} stopped after {read} files",
                    path.display()
                );
                return None;
            }
            read += 1;
            let (Ok(bytes), Some(name)) = (std::fs::read(&p), lower_name(&p)) else {
                continue;
            };
            files.push(Found {
                lower: bytes.to_ascii_lowercase(),
                path: p,
                name,
                bytes,
            });
        }
        // A file is kept when it names the document or a file kept before: an include of an include is followed
        // up to its top file. The search is a filter only; the parse decides.
        let mut kept = vec![false; files.len()];
        loop {
            let mut grew = false;
            for (i, f) in files.iter().enumerate() {
                if !kept[i] && names.iter().any(|n| contains(&f.lower, n)) {
                    kept[i] = true;
                    names.push(f.name.clone());
                    grew = true;
                }
            }
            if !grew {
                break;
            }
        }
        let mut candidates: Vec<&Found> = files
            .iter()
            .zip(&kept)
            .filter(|(f, k)| **k && is_bas(&f.path) && !l.open.contains_key(&path_key(&f.path)))
            .map(|(f, _)| f)
            .collect();
        // Spec "Order of programs found in one folder": one that no other names first, then one that is not a
        // `_test.bas` file, then by name.
        let named_by_other = |f: &Found| {
            candidates
                .iter()
                .any(|o| !std::ptr::eq(*o, f) && contains(&o.lower, &f.name))
        };
        let mut order: Vec<(bool, bool, Vec<u8>, &Found)> = candidates
            .iter()
            .map(|f| (named_by_other(f), f.name.ends_with(b"_test.bas"), f.name.clone(), *f))
            .collect();
        order.sort_by(|a, b| (a.0, a.1, &a.2).cmp(&(b.0, b.1, &b.2)));
        candidates = order.into_iter().map(|(_, _, _, f)| f).collect();
        for f in candidates {
            if analyzed == MAX_ANALYZED {
                eprintln!(
                    "qb64rust: the search for the program of {} stopped after {analyzed} programs",
                    path.display()
                );
                return None;
            }
            analyzed += 1;
            let key = path_key(&f.path);
            let file = OpenFile {
                uri: from_path(&f.path),
                bytes: f.bytes.clone().into(),
                encoding: l.default,
            };
            if covers(l, &key, &file) {
                return Some(Owner {
                    main: key,
                    uri: file.uri,
                    source: Source::Disk,
                });
            }
        }
        if folder == limit {
            break;
        }
    }
    None
}

/// The folder the disk search stops at: the folder of the nearest `qb64rust.toml` at or above `dir`, else the
/// workspace folder that holds `dir`, else `dir`. A settings file above the workspace folder is not looked for.
pub fn upper_limit(dir: &Path, folders: &[PathBuf]) -> PathBuf {
    let dir_key = path_key(dir);
    let within = |f: &PathBuf| {
        let k = path_key(f);
        dir_key == k
            || dir_key
                .strip_prefix(k.trim_end_matches('/'))
                .is_some_and(|r| r.starts_with('/'))
    };
    let workspace = folders.iter().filter(|f| within(f)).max_by_key(|f| path_key(f).len());
    for a in dir.ancestors() {
        if a.join(SETTINGS_FILE).is_file() {
            return a.to_path_buf();
        }
        if workspace.is_some_and(|w| path_key(w) == path_key(a)) {
            return a.to_path_buf();
        }
    }
    dir.to_path_buf()
}

/// The files of a folder with a BASIC extension (`.bas`, `.bi`, `.bm`, any letter case), by name.
fn basic_files(folder: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .is_some_and(|e| ["bas", "bi", "bm"].iter().any(|x| e.eq_ignore_ascii_case(x)))
                && p.is_file()
        })
        .collect();
    out.sort_by_key(|p| lower_name(p));
    out
}

/// Whether the path names a `.bas` file, any letter case.
pub fn is_bas(path: &Path) -> bool {
    path.extension().is_some_and(|e| e.eq_ignore_ascii_case("bas"))
}

/// A file's name in lower case.
fn lower_name(path: &Path) -> Option<Vec<u8>> {
    Some(path.file_name()?.to_string_lossy().to_ascii_lowercase().into_bytes())
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && hay.windows(needle.len()).any(|w| w == needle)
}

/// The main file at `path` (key `key`): its open document, else its bytes on disk.
fn program_file(l: &Lookup, path: &Path, key: &str) -> Option<OpenFile> {
    if let Some(f) = l.open.get(key) {
        return Some(f.clone());
    }
    let bytes = std::fs::read(path).ok()?;
    Some(OpenFile {
        uri: from_path(path),
        bytes: bytes.into(),
        encoding: l.default,
    })
}

/// The keys of the files the program of `main` covers, its main file first.
fn program_files(l: &Lookup, key: &str, main: &OpenFile) -> Vec<String> {
    analyze(key, main, l.open, l.root, l.default)
        .files
        .into_iter()
        .map(|f| f.key)
        .collect()
}

/// Whether the program of `main` includes the document.
fn covers(l: &Lookup, key: &str, main: &OpenFile) -> bool {
    program_files(l, key, main).iter().skip(1).any(|f| f == l.doc)
}
