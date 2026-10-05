//! Parse snapshots for each statement form of the slice, precedence, and recovery (task 3.2).

use qb64rust_base::SourceMap;
use qb64rust_syntax::tree::print;
use qb64rust_syntax::{NoLoader, ParsedProgram, dump_tree, parse};

fn parse_one(src: &[u8]) -> ParsedProgram {
    let mut map = SourceMap::new();
    let file = map.add("t.bas", src.to_vec());
    parse(&mut map, file, &mut NoLoader)
}

fn tree(src: &str) -> String {
    let mut map = SourceMap::new();
    let file = map.add("t.bas", src.as_bytes().to_vec());
    let program = parse(&mut map, file, &mut NoLoader);
    let p = program.main();
    assert_eq!(print(&p.green, src.as_bytes()), src.as_bytes(), "round trip");
    let mut out = dump_tree(p.root(), src.as_bytes());
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
/// (built-in statements such as `LOCATE , 5`); with `CALL` they are an error. In parentheses an argument may be
/// left out (`CALL s(1,)`); `sema` reports that.
#[test]
fn call_arguments_the_parser_cannot_read() {
    insta::assert_snapshot!(tree("LOCATE , 5\nCOLOR 4,\ns (1, 2)\nCALL s(1,)\nCALL s(1 2)\n"));
}

/// Member access after an index or a member (`.` is a `Dot` token there), with blanks around the dot; a dotted
/// name (`a.b`) stays one name.
#[test]
fn member_access() {
    insta::assert_snapshot!(tree("PRINT a(1).b.c(2), a(2) .b, a(2). b, a.b\na(i).x(j).y = q(1).r\n"));
}

#[test]
fn exit_declare_shared_static() {
    insta::assert_snapshot!(tree(
        "DECLARE SUB s (a AS LONG)\nDECLARE FUNCTION f$ ()\nDIM SHARED g AS LONG, h$\nSUB s (a AS LONG)\n  SHARED k AS LONG, m\n  STATIC c AS LONG\n  EXIT SUB\nEND SUB\n"
    ));
}

/// `DATA` holds one `DataText` token; `READ` targets are expressions; `RESTORE` takes a label or a number. Text
/// after a closing quote is an error, as in the old compiler.
#[test]
fn data_read_restore() {
    insta::assert_snapshot!(tree(
        "DATA 1, \"a, b\": READ x, a$(2), t(1).f\nRESTORE\nRESTORE lab\nRESTORE 100\nDATA\nDATA \"a\" b, 2\n"
    ));
}

/// Line numbers (measured M3): at the start of a line, also alone, before `:`, a comment, a label, glued to the
/// statement, with a decimal point or a suffix, and before `END SUB`. After a label or a `:` a number is an error.
/// `GOTO`/`GOSUB`/`RETURN` take a label or a number. Before a `SUB` header a number is allowed, and inside a `SUB`
/// such a header is still the nested-procedure error (`verification\v16_m3_sub_header`, `v16_m3_nested_sub`).
#[test]
fn line_numbers_and_jumps() {
    insta::assert_snapshot!(tree(
        "10 PRINT 1\n20 :\n30\n  40 ' c\n50 lab: GOTO 10\n60PRINT 2\n10.5 GOSUB lab\n70& RETURN\nRETURN 30\nSUB s\n80 END SUB\nlab2: 90 PRINT\nPRINT: 100\nGOTO\n110 SUB u\n120 SUB v\nEND SUB\n"
    ));
}

/// An array element assignment is not a call: `a(1) = 2` is an `AssignStmt` (`sema` says arrays are not
/// supported). With a `,` outside parentheses it is a call: `s (5 / 2) = 2, 0` is a `CallStmt`.
#[test]
fn array_assignment_is_not_a_call() {
    insta::assert_snapshot!(tree("a(1) = 2\nEXIT FOR\nDECLARE LIBRARY\ns (5 / 2) = 2, 0\n"));
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
    let p = parse_one(b"PRINT 1\n\nFOR i = 1 TO 2\n");
    let diags = p.diagnostics();
    let d = &diags.list()[0];
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
    let p = parse_one(src.as_bytes());
    assert_eq!(p.diagnostics().error_count(), 100);
    assert!(p.diagnostics().is_capped());
}
