//! The symbol table (design D12): snapshots of `dump_symbols` per spec scenario, and position lookups.

use qb64rust_base::{FileId, SourceFile};
use qb64rust_sema::{Program, SymbolKind, check, dump_symbols};

fn check_source(text: &[u8]) -> (SourceFile, Program) {
    let file = SourceFile::new("t.bas", text.to_vec());
    let parse = qb64rust_syntax::parse(FileId(0), &file.bytes);
    let root = qb64rust_syntax::tree::Node::root(&parse.green, FileId(0));
    let (program, diags) = check(root, &file, &parse.diagnostics);
    assert!(!diags.has_errors() && !parse.diagnostics.has_errors(), "{diags:?}");
    (file, program)
}

/// Scenario "Definition and references of a variable".
#[test]
fn definition_and_references_of_a_variable() {
    let (file, p) = check_source(b"$CONSOLE:ONLY\nDIM n AS LONG\nn = 1\nPRINT n&\n");
    insta::assert_snapshot!(dump_symbols(&p, &file));
}

/// An implicit variable is defined by its first use in the source, even when the value of its assignment is
/// resolved before the target; suffixed and plain names of other types are other symbols.
#[test]
fn implicit_variables() {
    let src = b"$CONSOLE:ONLY\nx = x + 1\nPRINT x, x%, y$\nx% = 2\n";
    let (file, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &file));
}

/// A statement with an error records nothing; the name's first clean use defines it.
#[test]
fn statement_with_an_error_records_nothing() {
    let file = SourceFile::new("t.bas", b"$CONSOLE:ONLY\nz = \"a\"\nz = 2\n".to_vec());
    let parse = qb64rust_syntax::parse(FileId(0), &file.bytes);
    let root = qb64rust_syntax::tree::Node::root(&parse.green, FileId(0));
    let (p, diags) = check(root, &file, &parse.diagnostics);
    assert_eq!(diags.error_count(), 1);
    insta::assert_snapshot!(dump_symbols(&p, &file), @"Var Z : SINGLE def 3:1");
}

#[test]
fn symbol_at_a_position() {
    // Offsets: `DIM nn AS LONG` starts at 14, `nn` at 18..20; `PRINT nn` starts at 29, `nn` at 35..37.
    let (_, p) = check_source(b"$CONSOLE:ONLY\nDIM nn AS LONG\nPRINT nn\n");
    let id = p.symbols.at(FileId(0), 18).expect("first byte of the definition");
    let SymbolKind::Var(v) = p.symbols.get(id).kind;
    assert_eq!(p.var(v).name, "NN");
    assert_eq!(p.symbols.at(FileId(0), 19), Some(id), "inside the definition");
    assert_eq!(p.symbols.at(FileId(0), 36), Some(id), "inside the reference");
    assert_eq!(p.symbols.at(FileId(0), 17), None, "before the name");
    assert_eq!(p.symbols.at(FileId(0), 20), None, "just after the name");
    assert_eq!(p.symbols.at(FileId(0), 0), None, "start of the file");
    assert_eq!(p.symbols.at(FileId(0), 37), None, "after the last name");
    assert_eq!(p.symbols.at(FileId(1), 18), None, "another file");
}
