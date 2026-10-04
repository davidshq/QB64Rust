//! Parse snapshots for each statement form of the slice, precedence, and recovery (task 3.2).

use qb64rust_base::{FileId, SourceMap};
use qb64rust_syntax::tree::{Node, print};
use qb64rust_syntax::{dump_tree, parse};

fn tree(src: &str) -> String {
    let p = parse(FileId(0), src.as_bytes());
    assert_eq!(print(&p.green, src.as_bytes()), src.as_bytes(), "round trip");
    let mut out = dump_tree(Node::root(&p.green, FileId(0)), src.as_bytes());
    let mut map = SourceMap::new();
    map.add("t.bas", src.as_bytes().to_vec());
    for d in p.diagnostics.list() {
        out.push_str(&d.render(&map));
        out.push('\n');
    }
    out
}

#[test]
fn metacommand_and_comments() {
    insta::assert_snapshot!(tree("$CONSOLE:ONLY\n' c\nREM r\nEND\n"));
}

#[test]
fn print_forms() {
    insta::assert_snapshot!(tree("PRINT \"a\"; x; -3, 1.5;\n? \"[\" \"a\"1; x \"b\" \"]\"\nPRINT\n"));
}

#[test]
fn dim_forms() {
    insta::assert_snapshot!(tree("DIM a AS LONG, b$, c AS _INTEGER64\n"));
}

#[test]
fn assignments() {
    insta::assert_snapshot!(tree("x = 1: LET y% = x + 2\n"));
}

#[test]
fn call_with_optional_argument() {
    insta::assert_snapshot!(tree("PRINT INSTR(3, s$, \"l\"); INSTR(s$, \"l\")\n"));
}

#[test]
fn precedence() {
    insta::assert_snapshot!(tree(
        "x = -2 ^ 2 + 2 ^ -3 * 4 - a MOD b \\ c * d\ny = NOT a = b AND c OR d\n"
    ));
}

#[test]
fn two_errors_reported() {
    // Errors on lines 2 and 5, one each; the statements around them parse.
    insta::assert_snapshot!(tree(
        "x = 1\nFOR i = 1 TO 3: PRINT i\nPRINT x\nPRINT x;\nPRINT (2\nEND\n"
    ));
}

#[test]
fn unsupported_statement_message() {
    let p = parse(FileId(0), b"PRINT 1\n\nFOR i = 1 TO 2\n");
    let d = &p.diagnostics.list()[0];
    assert_eq!(d.message, "`FOR` is not supported yet");
    assert_eq!(d.span.start, 9);
}

#[test]
fn error_cap() {
    let src = "FOR\n".repeat(150);
    let p = parse(FileId(0), src.as_bytes());
    assert_eq!(p.diagnostics.error_count(), 100);
    assert!(p.diagnostics.is_capped());
}
