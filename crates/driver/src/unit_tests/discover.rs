//! Test files and test SUBs (spec `compiler/unit-tests`, "Test files and test SUBs"; design D5 of
//! `basic-unit-tests`).

use qb64rust_base::{Platform, SourceMap};
use qb64rust_syntax::ast::{ProcDef, ProcHeader};
use qb64rust_syntax::tree::Node;
use qb64rust_syntax::{NoLoader, SyntaxKind, parse_settled_on};
use std::path::{Path, PathBuf};

/// A test SUB: its name as written and the line of its header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestSub {
    pub name: String,
    pub line: u32,
}

/// What a test file holds: its tests in source order, and a note for each SUB that is a test by its name but takes
/// parameters (`TestWith takes parameters: not a test`), with its line.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Found {
    pub tests: Vec<TestSub>,
    pub notes: Vec<(u32, String)>,
}

/// The tests of the test file `bytes` (shown as `name`), from its own parse: its includes are not read, so a SUB of
/// an included file is never one. A test is a `SUB` whose name starts with `test` in any case and that has no
/// parameter list or an empty one. Comments and strings are tokens of their own, so they never match.
pub fn tests_of(name: &str, bytes: Vec<u8>) -> Found {
    let mut map = SourceMap::new();
    let file = map.add(name, bytes);
    let parsed = parse_settled_on(Platform::host(), &mut map, file, &mut NoLoader);
    let mut found = Found::default();
    for tree in parsed.trees.iter().filter(|t| t.file == file) {
        walk(tree.root(), &map, &mut found);
    }
    found
}

fn walk(node: Node, map: &SourceMap, found: &mut Found) {
    if node.kind() == SyntaxKind::ProcDef {
        if let Some(header) = ProcDef::cast(node).and_then(ProcDef::header) {
            take(header, map, found);
        }
        return;
    }
    for child in node.child_nodes() {
        walk(child, map, found);
    }
}

fn take(header: ProcHeader, map: &SourceMap, found: &mut Found) {
    let (Some(keyword), Some(name)) = (header.keyword(), header.name()) else {
        return;
    };
    if !map.text(keyword.span).eq_ignore_ascii_case(b"SUB") {
        return;
    }
    // A name is letters, digits, `_` and `.`: ASCII, so each byte is its character.
    let name: String = map.text(name.span).iter().map(|&b| char::from(b)).collect();
    if !name.to_ascii_lowercase().starts_with("test") {
        return;
    }
    let line = map.file(keyword.span.file).line_col(keyword.span.start).0;
    if header.params().next().is_some() {
        found.notes.push((line, format!("{name} takes parameters: not a test")));
    } else {
        found.tests.push(TestSub { name, line });
    }
}

/// Whether a file's name makes it a test file: it ends in `_test.bas`, in any case.
pub fn is_test_file(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|n| n.to_string_lossy().to_ascii_lowercase().ends_with("_test.bas"))
}

/// The test files of the paths given (with none, the working folder): a file named is taken whatever its name; a
/// folder gives every test file in it or below it, sorted. A path that does not exist is an error.
pub fn test_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    let default = [PathBuf::from(".")];
    let paths = if paths.is_empty() { &default[..] } else { paths };
    let mut out = Vec::new();
    for p in paths {
        if p.is_file() {
            out.push(p.clone());
        } else if p.is_dir() {
            let mut found = Vec::new();
            collect(p, &mut found).map_err(|e| format!("reading {}: {e}", p.display()))?;
            found.sort();
            out.extend(found);
        } else {
            return Err(format!("{}: no such file or folder", p.display()));
        }
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|p| seen.insert(std::fs::canonicalize(p).unwrap_or_else(|_| p.clone())));
    Ok(out)
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, out)?;
        } else if is_test_file(&path) {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(text: &str) -> Found {
        tests_of("math_test.bas", text.as_bytes().to_vec())
    }

    fn names(text: &str) -> Vec<String> {
        found(text).tests.into_iter().map(|t| t.name).collect()
    }

    #[test]
    fn found_in_a_file() {
        // Spec "Found in a folder": `TestWith` takes parameters and `TestF` is a FUNCTION.
        let text = "SUB TestAdd\nEND SUB\n\nsub test_sub ()\nend sub\nSUB TestWith (x AS LONG)\nEND SUB\n\
                    FUNCTION TestF\nTestF = 1\nEND FUNCTION\nSUB Helper\nEND SUB\nSUB TESTUPPER\nEND SUB\n";
        let f = found(text);
        assert_eq!(
            f.tests,
            [
                TestSub {
                    name: "TestAdd".into(),
                    line: 1
                },
                TestSub {
                    name: "test_sub".into(),
                    line: 4
                },
                TestSub {
                    name: "TESTUPPER".into(),
                    line: 13
                },
            ]
        );
        assert_eq!(f.notes, [(6, "TestWith takes parameters: not a test".to_string())]);
    }

    #[test]
    fn a_sub_of_an_included_file_is_not_a_test() {
        assert_eq!(names("'$INCLUDE:'math.bm'\nSUB TestOwn\nEND SUB\n"), ["TestOwn"]);
    }

    #[test]
    fn a_comment_or_a_string_is_not_a_test() {
        assert_eq!(
            names("' SUB TestOld\nREM SUB TestRem\nPRINT \"SUB TestStr\"\nx$ = \"SUB TestX\"\n"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_files_of_a_folder() {
        let d = tempfile::tempdir().unwrap();
        let lib = d.path().join("lib");
        std::fs::create_dir_all(lib.join("deep")).unwrap();
        for f in [
            "a_test.bas",
            "B_TEST.BAS",
            "helpers.bas",
            "deep/c_test.bas",
            "x_test.bi",
        ] {
            std::fs::write(lib.join(f), "").unwrap();
        }
        let got = test_files(std::slice::from_ref(&lib)).unwrap();
        let mut names: Vec<String> = got
            .iter()
            .map(|p| p.strip_prefix(&lib).unwrap().to_string_lossy().replace('\\', "/"))
            .collect();
        names.sort();
        assert_eq!(names, ["B_TEST.BAS", "a_test.bas", "deep/c_test.bas"]);
        // A file named is taken whatever its name.
        assert_eq!(
            test_files(&[lib.join("helpers.bas")]).unwrap(),
            [lib.join("helpers.bas")]
        );
        assert!(
            test_files(&[lib.join("none")])
                .unwrap_err()
                .contains("no such file or folder")
        );
    }
}
