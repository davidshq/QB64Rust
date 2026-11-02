//! The features on parsed programs, without the protocol: symbols, folding ranges and go to definition (design D4,
//! D7). The fixture `fixtures\outline.bas` has every kind of symbol, block and jump.

use lsp_types::{DocumentSymbol, Position, Uri};
use qb64rust_lsp::analysis::{Analysis, OpenFile, analyze};
use qb64rust_lsp::encoding::Encoding;
use qb64rust_lsp::uri::{from_path, path_key};
use qb64rust_lsp::{definition, folding, symbols};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;
use std::sync::Arc;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

fn open(uri: Uri, text: &[u8]) -> OpenFile {
    OpenFile {
        uri,
        bytes: Arc::from(text),
        encoding: Encoding::DEFAULT,
    }
}

/// Parses `main` with the open documents `others` and the include root `root`.
fn parse_with(main: &Path, text: &[u8], others: &[(&Path, &[u8])], root: &Path) -> Analysis {
    let open_files: HashMap<String, OpenFile> = others
        .iter()
        .map(|(p, t)| (path_key(p), open(from_path(p), t)))
        .collect();
    analyze(
        &path_key(main),
        &open(from_path(main), text),
        &open_files,
        root,
        Encoding::DEFAULT,
    )
}

fn parse_text(text: &str) -> Analysis {
    let main = fixture("virtual.bas");
    parse_with(&main, text.as_bytes(), &[], &fixture(""))
}

fn outline() -> Analysis {
    let path = fixture("outline.bas");
    let text = std::fs::read(&path).unwrap();
    parse_with(&path, &text, &[], &fixture(""))
}

fn show_symbols(symbols: &[DocumentSymbol], depth: usize, out: &mut String) {
    for s in symbols {
        let r = s.range;
        let sel = s.selection_range;
        writeln!(
            out,
            "{:indent$}{:?} {} {} {}:{}-{}:{} name {}:{}-{}:{}",
            "",
            s.kind,
            s.name,
            s.detail.as_deref().unwrap_or("-"),
            r.start.line,
            r.start.character,
            r.end.line,
            r.end.character,
            sel.start.line,
            sel.start.character,
            sel.end.line,
            sel.end.character,
            indent = depth * 2
        )
        .unwrap();
        show_symbols(s.children.as_deref().unwrap_or_default(), depth + 1, out);
    }
}

#[test]
fn symbols_of_every_kind() {
    let mut out = String::new();
    show_symbols(&symbols::document_symbols(&outline()), 0, &mut out);
    insta::assert_snapshot!(out);
}

#[test]
fn folding_of_every_kind() {
    let mut ranges = folding::folding_ranges(&outline());
    ranges.sort_by_key(|r| (r.start_line, r.end_line));
    let mut out = String::new();
    for r in ranges {
        writeln!(out, "{}-{} {:?}", r.start_line, r.end_line, r.kind).unwrap();
    }
    insta::assert_snapshot!(out);
}

#[test]
fn do_loop_folds_to_the_line_before_loop() {
    // Spec scenario: `DO` on line 2 and `LOOP` on line 5 fold lines 2 to 4 (0-based 1 to 3).
    let a = parse_text("x = 1\nDO\n  x = x + 1\n  PRINT x\nLOOP\n");
    let ranges = folding::folding_ranges(&a);
    assert_eq!(ranges.len(), 1);
    assert_eq!((ranges[0].start_line, ranges[0].end_line), (1, 3));
}

#[test]
fn outline_of_spec_scenario() {
    // `SUB a`, a label `top:` in the main module and `again:` inside `a`: `a` (with child `again`) and `top`.
    let a = parse_text("top:\nPRINT 1\nSUB a\nagain:\nEND SUB\n");
    let s = symbols::document_symbols(&a);
    let names: Vec<_> = s.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, ["top", "a"]);
    let children: Vec<_> = s[1]
        .children
        .as_ref()
        .unwrap()
        .iter()
        .map(|c| c.name.as_str())
        .collect();
    assert_eq!(children, ["again"]);
}

/// The definition of the name at `needle`'s first occurrence on line `line` (0-based), as `line:column` (0-based)
/// and the file's name.
fn def_at(a: &Analysis, line: u32, needle: &str) -> Option<(String, u32, u32)> {
    let main = a.parsed.main().file;
    let file = a.map.file(main);
    let (start, end) = file.line_content(line).unwrap();
    let text = &file.bytes[start as usize..end as usize];
    let col = text
        .windows(needle.len())
        .position(|w| w == needle.as_bytes())
        .expect("needle on the line");
    // Inside the name, not at its start.
    let pos = Position::new(line, u32::try_from(col + 1).unwrap());
    let offset = a.info(main).columns.offset(file, pos);
    let span = definition::definition(a, offset)?;
    let r = a.range(span);
    let name = Path::new(&a.map.file(span.file).name)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    Some((name, r.start.line, r.start.character))
}

#[test]
fn definition_of_each_form() {
    let a = outline();
    let at = |line, col| Some(("outline.bas".to_string(), line, col));
    // A call without `CALL` before the definition, on line 12 (1-based); `SUB bump` is on line 48.
    assert_eq!(def_at(&a, 11, "bump"), at(47, 4));
    // `DECLARE` points to the definition; so does the header itself.
    assert_eq!(def_at(&a, 4, "bump"), at(47, 4));
    assert_eq!(def_at(&a, 47, "bump"), at(47, 4));
    // `CALL bump(n)` inside the procedure.
    assert_eq!(def_at(&a, 51, "bump"), at(47, 4));
    // A function call in an expression, the suffix written.
    assert_eq!(def_at(&a, 12, "twice"), at(54, 9));
    // `GOTO again` in the main module and inside `bump`: each its own body's label.
    assert_eq!(def_at(&a, 39, "again"), at(40, 0));
    assert_eq!(def_at(&a, 50, "again"), at(48, 4));
    // `GOSUB 10`: the line number; `ON ERROR GOTO handler`: the main module's label.
    assert_eq!(def_at(&a, 42, "10"), at(41, 0));
    assert_eq!(def_at(&a, 38, "handler"), at(44, 0));
    // A variable, a name with no definition, a keyword: nothing.
    assert_eq!(def_at(&a, 12, "total"), None);
    assert_eq!(def_at(&a, 45, "NEXT"), None);
    assert_eq!(def_at(&a, 9, "DIM"), None);
}

#[test]
fn definition_at_the_end_of_a_name() {
    let a = parse_text("GOTO done\ndone:\n");
    let main = a.parsed.main().file;
    let offset = a.info(main).columns.offset(a.map.file(main), Position::new(0, 9));
    let span = definition::definition(&a, offset).unwrap();
    assert_eq!(a.range(span).start, Position::new(1, 0));
}

#[test]
fn on_error_in_a_procedure_names_the_main_module_label() {
    let a = parse_text("hnd:\nRESUME NEXT\nSUB s\nhnd:\nON ERROR GOTO hnd\nEND SUB\n");
    assert_eq!(def_at(&a, 4, "hnd"), Some(("virtual.bas".into(), 0, 0)));
}

#[test]
fn definition_across_included_files() {
    let dir = tempfile::tempdir().unwrap();
    let lib = dir.path().join("lib.bm");
    std::fs::write(&lib, "SUB hello\nPRINT \"hi\"\nEND SUB\n").unwrap();
    let main = dir.path().join("main.bas");
    let a = parse_with(&main, b"hello\n'$INCLUDE: 'lib.bm'\n", &[], dir.path());
    assert_eq!(def_at(&a, 0, "hello"), Some(("lib.bm".into(), 0, 4)));
}
