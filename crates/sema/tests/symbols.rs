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

/// Scenario "Same name, different symbols": main's `x` and the SUB's implicit `x`.
#[test]
fn same_name_different_symbols() {
    let src = b"$CONSOLE:ONLY\nx = 1\nshow\nPRINT x\nSUB show\n    x = 2\n    PRINT x\nEND SUB\n";
    let (file, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &file));
    // `x` in the SUB starts at 46 (line 6); main's `x` at 14.
    let in_sub = p.symbols.at(FileId(0), 46).unwrap();
    let in_main = p.symbols.at(FileId(0), 14).unwrap();
    assert_ne!(in_sub, in_main);
}

/// Scenario "Call before definition": the call is a reference of the procedure, defined by the header's name;
/// the function's own name assigned in its body is the result variable, not the procedure.
#[test]
fn call_before_definition() {
    let src =
        b"$CONSOLE:ONLY\nn& = 3\nPRINT twice&(n&)\nFUNCTION twice& (a AS LONG)\n    twice& = a * 2\nEND FUNCTION\n";
    let (file, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &file));
}

/// A `SHARED` line that creates the main-module variable is its definition; main's later uses refer to it.
#[test]
fn shared_line_creates_the_main_variable() {
    let src = b"$CONSOLE:ONLY\nSUB s\n    SHARED h AS LONG\n    h = 1\nEND SUB\nh = 2\nPRINT h&\n";
    let (file, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &file));
}

/// A label is defined where it stands, even when `ON ERROR GOTO` names it first; `ON ERROR GOTO` in a SUB and
/// `RESUME` refer to the same main-module label.
#[test]
fn label_with_handler_and_resume_references() {
    let src =
        b"$CONSOLE:ONLY\nON ERROR GOTO h\ns\nback: SYSTEM\nh:\nRESUME back\nSUB s\n    ON ERROR GOTO H\nEND SUB\n";
    let (file, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &file));
    // `h` in `ON ERROR GOTO h` (offset 28) is the label defined at `h:` (offset 45).
    let id = p.symbols.at(FileId(0), 28).unwrap();
    assert_eq!(p.symbols.at(FileId(0), 45), Some(id));
    assert!(matches!(p.symbols.get(id).kind, SymbolKind::Label(_)));
}

#[test]
fn symbol_at_a_position() {
    // Offsets: `DIM nn AS LONG` starts at 14, `nn` at 18..20; `PRINT nn` starts at 29, `nn` at 35..37.
    let (_, p) = check_source(b"$CONSOLE:ONLY\nDIM nn AS LONG\nPRINT nn\n");
    let id = p.symbols.at(FileId(0), 18).expect("first byte of the definition");
    let SymbolKind::Var(v) = p.symbols.get(id).kind else {
        panic!("not a variable")
    };
    assert_eq!(p.var(v).name, "NN");
    assert_eq!(p.symbols.at(FileId(0), 19), Some(id), "inside the definition");
    assert_eq!(p.symbols.at(FileId(0), 36), Some(id), "inside the reference");
    assert_eq!(p.symbols.at(FileId(0), 17), None, "before the name");
    assert_eq!(p.symbols.at(FileId(0), 20), None, "just after the name");
    assert_eq!(p.symbols.at(FileId(0), 0), None, "start of the file");
    assert_eq!(p.symbols.at(FileId(0), 37), None, "after the last name");
    assert_eq!(p.symbols.at(FileId(1), 18), None, "another file");
}
