//! Tier 1 (design D7): the walks of the language server over every corpus and upstream program, with their included
//! files: no panic, every range inside its file, every symbol's name inside its range and every child inside its
//! parent, every folding range at least two lines; go to definition at every name of the smaller programs.

use lsp_types::{DocumentSymbol, Position, Range};
use qb64rust_lsp::analysis::{Analysis, OpenFile, analyze};
use qb64rust_lsp::encoding::Encoding;
use qb64rust_lsp::uri::{from_path, path_key};
use qb64rust_lsp::{definition, folding, symbols};
use qb64rust_syntax::SyntaxKind;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn programs(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            programs(&p, out);
        } else if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bas")) {
            out.push(p);
        }
    }
}

fn inside(inner: Range, outer: Range) -> bool {
    let le = |a: Position, b: Position| (a.line, a.character) <= (b.line, b.character);
    le(outer.start, inner.start) && le(inner.end, outer.end) && le(inner.start, inner.end)
}

fn check_symbols(name: &str, lines: u32, symbols: &[DocumentSymbol], parent: Option<Range>) {
    for s in symbols {
        assert!(s.range.end.line < lines, "{name}: {} past the end", s.name);
        assert!(
            inside(s.selection_range, s.range),
            "{name}: name of {} outside its range",
            s.name
        );
        if let Some(p) = parent {
            assert!(inside(s.range, p), "{name}: {} outside its parent", s.name);
        }
        check_symbols(name, lines, s.children.as_deref().unwrap_or_default(), Some(s.range));
    }
}

/// Go to definition at every name of the main file; a result must be a name token of some file.
fn check_definitions(name: &str, a: &Analysis) {
    let main = a.parsed.main();
    for t in main.root().tokens() {
        if t.kind != SyntaxKind::Ident {
            continue;
        }
        if let Some(span) = definition::definition(a, t.span.start + 1) {
            let text = a.map.text(span);
            assert!(!text.is_empty(), "{name}: empty definition");
            let file = a.map.file(span.file);
            let pos = a.info(span.file).columns.position(file, span.start);
            assert!(pos.line < file.line_count(), "{name}: definition past the end");
        }
    }
}

#[test]
fn walks_over_every_program() {
    let root = repo().join("tests/upstream/root");
    let mut files = Vec::new();
    programs(&repo().join("tests/corpus"), &mut files);
    programs(&repo().join("tests/upstream/compile_tests"), &mut files);
    assert!(files.len() > 600, "found only {} programs", files.len());
    let none = HashMap::new();
    let (mut symbol_count, mut fold_count) = (0, 0);
    for path in &files {
        let name = path.strip_prefix(repo()).unwrap_or(path).display().to_string();
        let bytes = std::fs::read(path).unwrap();
        let main = OpenFile {
            uri: from_path(path),
            bytes: Arc::from(bytes),
            encoding: Encoding::DEFAULT,
        };
        let a = analyze(&path_key(path), &main, &none, &root, Encoding::DEFAULT);
        let lines = a.map.file(a.parsed.main().file).line_count();

        let s = symbols::document_symbols(&a);
        check_symbols(&name, lines, &s, None);
        symbol_count += s.len();

        let f = folding::folding_ranges(&a);
        for r in &f {
            assert!(
                r.start_line < r.end_line,
                "{name}: folding {}-{}",
                r.start_line,
                r.end_line
            );
            assert!(r.end_line < lines, "{name}: folding past the end");
        }
        fold_count += f.len();

        if a.map.file(a.parsed.main().file).bytes.len() < 50_000 {
            check_definitions(&name, &a);
        }
    }
    eprintln!(
        "{} programs, {symbol_count} top-level symbols, {fold_count} folding ranges",
        files.len()
    );
    assert!(symbol_count > 100 && fold_count > 500);
}
