//! `TestLoader` (design D3 of `basic-unit-tests`): the loader of a test build. The library's two files come from
//! memory under their reserved names; every other include is found as [`FileLoader`] finds it, from the virtual main
//! file's folder, which is the test file's.

use super::entry::{LIBRARY_BI, LIBRARY_BI_TEXT, LIBRARY_BM, LIBRARY_BM_TEXT};
use crate::FileLoader;
use qb64rust_base::{FileId, SourceMap};
use qb64rust_syntax::{LoadError, Loader, include_name};
use std::collections::HashMap;
use std::path::Path;

pub struct TestLoader {
    files: FileLoader,
    /// The library's files added to the map already, by name.
    library: HashMap<&'static str, FileId>,
}

impl TestLoader {
    /// As [`FileLoader::new`]: `main` the virtual main file, `main_path` its path beside the test file.
    pub fn new(root: &Path, main: FileId, main_path: &Path) -> TestLoader {
        TestLoader {
            files: FileLoader::new(root, main, main_path),
            library: HashMap::new(),
        }
    }
}

/// The library file an include names, if it names one: its name and text.
fn library_file(path: &[u8]) -> Option<(&'static str, &'static str)> {
    let name = include_name(path).to_string_lossy().replace('\\', "/");
    [(LIBRARY_BI, LIBRARY_BI_TEXT), (LIBRARY_BM, LIBRARY_BM_TEXT)]
        .into_iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(&name))
}

impl Loader for TestLoader {
    fn load(&mut self, map: &mut SourceMap, from: FileId, path: &[u8]) -> Result<FileId, LoadError> {
        let Some((name, text)) = library_file(path) else {
            return self.files.load(map, from, path);
        };
        Ok(*self
            .library
            .entry(name)
            .or_insert_with(|| map.add(name, text.as_bytes().to_vec())))
    }

    fn load_root(&mut self, map: &mut SourceMap, path: &Path) -> Result<FileId, LoadError> {
        self.files.load_root(map, path)
    }

    fn library(&mut self, name: &[u8]) -> Result<Vec<u8>, LoadError> {
        self.files.library(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_library_comes_from_memory_and_the_rest_from_disk() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("math.bm"), "SUB Helper\nEND SUB\n").unwrap();
        let main_path = d.path().join("math_test.bas.test-entry.bas");
        let mut map = SourceMap::new();
        let main = map.add(main_path.to_string_lossy(), Vec::new());
        let mut loader = TestLoader::new(d.path(), main, &main_path);
        let bi = loader.load(&mut map, main, b"qb64rust-test/qbtest.bi").unwrap();
        assert_eq!(&*map.file(bi).bytes, LIBRARY_BI_TEXT.as_bytes());
        // The same file again, whatever the separator or case.
        assert_eq!(loader.load(&mut map, main, b"QB64RUST-TEST\\QBTEST.BI").unwrap(), bi);
        let bm = loader.load(&mut map, main, b"qb64rust-test/qbtest.bm").unwrap();
        assert_eq!(&*map.file(bm).bytes, LIBRARY_BM_TEXT.as_bytes());
        let helper = loader.load(&mut map, main, b"math.bm").unwrap();
        assert_eq!(&*map.file(helper).bytes, b"SUB Helper\nEND SUB\n");
        assert!(loader.load(&mut map, main, b"none.bm").is_err());
    }
}
