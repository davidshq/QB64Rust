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
fn system_with_and_without_exit_code() {
    insta::assert_snapshot!(tree("SYSTEM\nsystem: END\nSYSTEM 3\n"));
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
fn sub_and_function_definitions() {
    insta::assert_snapshot!(tree(
        "SUB s (a AS LONG, b$, c)\n  PRINT a\nEND SUB\nFUNCTION f& (x AS _UNSIGNED LONG)\n  f& = x\nEND FUNCTION\nSUB t\nEND SUB\nPRINT f&(1)\n"
    ));
}

#[test]
fn calls() {
    insta::assert_snapshot!(tree("CALL s(n, 2)\nCALL t\ns n, \"x\"\ns (n)\nt\nbump n + 1: t\n"));
}

/// Without `CALL`, arguments that are not an expression list are kept in an `Error` node with no diagnostic
/// (built-in statements such as `LOCATE , 5`); with `CALL` they are an error.
#[test]
fn call_arguments_the_parser_cannot_read() {
    insta::assert_snapshot!(tree("LOCATE , 5\nCOLOR 4,\ns (1, 2)\nCALL s(1,)\n"));
}

#[test]
fn exit_declare_shared_static() {
    insta::assert_snapshot!(tree(
        "DECLARE SUB s (a AS LONG)\nDECLARE FUNCTION f$ ()\nDIM SHARED g AS LONG, h$\nSUB s (a AS LONG)\n  SHARED k AS LONG, m\n  STATIC c AS LONG\n  EXIT SUB\nEND SUB\n"
    ));
}

/// An array element assignment is not a call: `a(1) = 2` still says arrays are not supported.
#[test]
fn array_assignment_is_not_a_call() {
    insta::assert_snapshot!(tree("a(1) = 2\nEXIT FOR\nDECLARE LIBRARY\n"));
}

#[test]
fn recovery_nested_sub() {
    insta::assert_snapshot!(tree("SUB a\n  PRINT 1\nSUB b\n  PRINT 2\nEND SUB\nPRINT 3\n"));
}

#[test]
fn recovery_missing_end_sub() {
    insta::assert_snapshot!(tree("PRINT 0\nSUB a\n  PRINT 1\n"));
}

#[test]
fn recovery_stray_end_sub() {
    insta::assert_snapshot!(tree("PRINT 0\nEND SUB\nPRINT 1\n"));
}

#[test]
fn recovery_wrong_end_kind() {
    insta::assert_snapshot!(tree("SUB a\n  PRINT 1\nEND FUNCTION\nPRINT 2\n"));
}

/// A file that ends inside a SUB with no line end after the last statement.
#[test]
fn file_ends_inside_a_sub() {
    insta::assert_snapshot!(tree("SUB a\nPRINT 1"));
}

/// A header with an error still parses its body; `FUNCTION f AS LONG` is rejected (the old compiler does too).
#[test]
fn header_errors() {
    insta::assert_snapshot!(tree(
        "FUNCTION f (a) AS LONG\nf = 1\nEND FUNCTION\nSUB\nEND SUB\nSUB s (BYVAL x)\nEND SUB\n"
    ));
}

#[test]
fn labels() {
    // A label on its own line, before a statement, two in a row; `a$:` and keywords are not labels; `END:` and
    // `SYSTEM:` are the statements.
    insta::assert_snapshot!(tree(
        "handler:\nback: PRINT 1\na: b.c: PRINT 2\nx$: PRINT 3\nEND: SYSTEM:\n"
    ));
}

#[test]
fn error_handling_statements() {
    insta::assert_snapshot!(tree(
        "ON ERROR GOTO h\nON ERROR GOTO 0\nRESUME\nRESUME 0\nRESUME NEXT\nRESUME back\nERROR 5\nERROR k + 1\n"
    ));
}

#[test]
fn error_handling_statement_errors() {
    insta::assert_snapshot!(tree(
        "ON ERROR GOTO\nON ERROR RESUME NEXT\nON TIMER(1) GOSUB t\nRESUME NEXT x\nERROR\nON ERROR GOTO h 1\nON\nON ERROR\n"
    ));
}

#[test]
fn unsupported_statement_message() {
    let p = parse(FileId(0), b"PRINT 1\n\nFOR i = 1 TO 2\n");
    let d = &p.diagnostics.list()[0];
    assert_eq!(d.message, "statement `FOR`");
    assert!(d.unsupported);
    assert_eq!(d.span.start, 9);
}

/// Errors at a BASIC word or operator the parser does not handle there are marked; a genuine syntax error is not.
#[test]
fn marked_parse_errors() {
    insta::assert_snapshot!(tree("x = a MOD b\nIF a < b THEN\nPRINT #1, x\nx = 5 TO 6\nx = 5 6\n"));
}

#[test]
fn error_cap() {
    let src = "FOR\n".repeat(150);
    let p = parse(FileId(0), src.as_bytes());
    assert_eq!(p.diagnostics.error_count(), 100);
    assert!(p.diagnostics.is_capped());
}
