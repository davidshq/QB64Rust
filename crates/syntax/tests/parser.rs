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

/// Array bounds in `DIM`, `DIM SHARED`, `STATIC` and `SHARED` (design D2 of `m2-arrays-and-types`).
#[test]
fn dim_array_forms() {
    insta::assert_snapshot!(tree(
        "DIM a(5) AS LONG, b(1 TO 3, -2 TO n), c$(2)\nDIM SHARED d(0)\nSTATIC e(4) AS t\nSHARED f() AS LONG, g\n"
    ));
}

/// A DOS end-of-file byte (0x1A) ending a line is whitespace, as the old compiler's line reader drops it
/// (`qb64pe.bas` 28060); one inside a line is still an error.
#[test]
fn eof_byte_at_line_end() {
    for src in [&b"PRINT 1\n\x1a"[..], b"PRINT 1\x1a\nPRINT 2\n", b"PRINT 1\r\n\x1a\r\n"] {
        let p = parse_one(src);
        assert!(p.main().diagnostics.list().is_empty(), "{src:?}");
        assert_eq!(print(&p.main().green, src), src);
    }
    assert!(!parse_one(b"PRINT \x1a 1\n").main().diagnostics.list().is_empty());
}

#[test]
fn dim_bounds_accessor() {
    use qb64rust_syntax::ast::{DimStmt, SourceFile};
    let src = b"DIM a(5) AS LONG, b(1 TO 3, 2), c\n";
    let program = parse_one(src);
    let root = SourceFile::cast(program.main().root()).unwrap();
    let dim = root.statements().find_map(DimStmt::cast).unwrap();
    let shapes: Vec<Vec<(bool, bool)>> = dim
        .items()
        .map(|i| {
            i.bounds()
                .map(|b| b.ranges().iter().map(|(l, u)| (l.is_some(), u.is_some())).collect())
                .unwrap_or_default()
        })
        .collect();
    assert_eq!(shapes, [vec![(false, true)], vec![(true, true), (false, true)], vec![]]);
    assert!(dim.items().all(|i| i.name().is_some()));
    assert!(dim.items().next().unwrap().as_clause().is_some());
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
    insta::assert_snapshot!(tree("x = 1\nREDIM c(3): PRINT i\nPRINT x\nPRINT x;\nPRINT (2\nEND\n"));
}

#[test]
fn const_forms() {
    insta::assert_snapshot!(tree(
        "CONST a = 1\nCONST b% = 2.5, c$ = \"x\" + \"y\", d# = -a ^ 2\nIF 1 THEN CONST e = 4\nSUB s\n  CONST f& = a * 2\nEND SUB\n"
    ));
}

#[test]
fn const_errors() {
    // One error per statement; the statements after each parse. `ROOT` is an operator only in a `CONST` value
    // (marked "not supported yet" there), elsewhere a plain syntax error.
    insta::assert_snapshot!(tree(
        "CONST\nCONST a\nCONST a = 1,\nCONST 5 = 1\nCONST a = 1 2\nCONST r = 20 ROOT 3\nCONST q = (8 ROOT 3) + 1\nx = 20 ROOT 3\nPRINT a\n"
    ));
}

#[test]
fn option_forms() {
    insta::assert_snapshot!(tree(
        "OPTION BASE 1\nOPTION _EXPLICIT: DIM x\nOPTION _EXPLICITARRAY\nOPTION EXPLICIT\nSUB s\n  OPTION _EXPLICIT\nEND SUB\n"
    ));
}

#[test]
fn option_errors() {
    // `option` alone or before `=` is a name (not a reserved word).
    insta::assert_snapshot!(tree(
        "OPTION BASE\nOPTION FOO\nOPTION _EXPLICIT 1\noption = 3\nOPTION\n"
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

/// Without `CALL`, arguments that are not an expression list are kept in an `Error` node with no diagnostic (`t ,
/// 5`: a built-in statement without a template, or a SUB); with `CALL` they are an error. In parentheses an
/// argument may be left out (`CALL s(1,)`); `sema` reports that. Built-in statements with a template (`LOCATE , 5`,
/// `COLOR 4,`) are read by it (task 7.5).
#[test]
fn call_arguments_the_parser_cannot_read() {
    insta::assert_snapshot!(tree("LOCATE , 5\nCOLOR 4,\ns (1, 2)\nCALL s(1,)\nCALL s(1 2)\nt , 5\n"));
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

/// Declarations (design D5, task 7.2): `DEFxxx` and `_DEFINE` letter ranges, `DIM AS type` lists, fixed-length
/// strings, `REDIM` with `_PRESERVE`/`SHARED` and a member array, `COMMON` with a block name, `ERASE` of a plain
/// array and a member array (also with blanks around the dot), `STATIC AS`, array parameters and `STATIC` after a
/// header, a type name as an argument.
#[test]
fn declarations() {
    insta::assert_snapshot!(tree(
        "DEFINT A-Z, i\n_DEFINE m AS _UNSIGNED LONG\nDIM AS LONG a, b(3)\nDIM s AS STRING * 8\nREDIM SHARED _PRESERVE r(1 TO 2)\nREDIM w(0).v(3)\nCOMMON SHARED /blk/ c()\nERASE r, w(0).v, a . s\nSTATIC AS LONG st\nSUB p (x(), y( ,) AS LONG) STATIC\nEND SUB\nv = VAL(\"1\", _UNSIGNED _INTEGER64)\n"
    ));
}

/// The preprocessor (design D8): every `$…` line is a `MetaStmt`; a branch not taken is one `InactiveCode` node
/// (here holding a nested `$IF`, an unclosed `FOR` and garbage), up to the `$ELSE` of its level; a `$IF` inside a
/// `FOR` body; the `$IF` and block entries nest.
#[test]
fn preprocessor() {
    insta::assert_snapshot!(tree(
        "$IF LINUX THEN\n$IF WIN THEN\nFOR i = 1 TO\n$END IF\n(((\n$ELSE\nPRINT 1\n$END IF\nFOR i = 1 TO 2\n  $IF WIN THEN\n  PRINT i\n  $END IF\nNEXT\n$LET A = 1\n"
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
    insta::assert_snapshot!(tree("a(1) = 2\ns (5 / 2) = 2, 0\n"));
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
    // A label on its own line, before a statement; after a `:` a name and a colon are a call (`b.c`, measured
    // `verification\v16_m3_call_after_colon`); `a$:` and keywords are not labels; `END:` and `SYSTEM:` are the
    // statements.
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
    let p = parse_one(b"PRINT 1\n\nLIST 1\n");
    let diags = p.diagnostics();
    let d = &diags.list()[0];
    assert_eq!(d.message, "statement `LIST`");
    assert!(d.unsupported);
    assert_eq!(d.span.start, 9);
}

/// The expression depth limit counts tree levels the same way for every kind of node: an expression exactly 1,000
/// levels deep parses, one a level deeper is one "not supported yet" error. Runs on a large stack, as the compiler
/// does (`driver::with_stack`); a test thread's 2 MiB would not hold the parser at this depth in a debug build.
#[test]
fn expression_depth_limit() {
    // (name, statement at 1,000 levels, the same at 1,001 levels).
    let cases = [
        // A left-associative chain: one `BinExpr` per operator over the first literal.
        (
            "chain",
            format!("x = 1{}", " + 1".repeat(999)),
            format!("x = 1{}", " + 1".repeat(1000)),
        ),
        (
            "parens",
            format!("x = {}1{}", "(".repeat(999), ")".repeat(999)),
            format!("x = {}1{}", "(".repeat(1000), ")".repeat(1000)),
        ),
        (
            "negations",
            format!("x = {}1", "- ".repeat(999)),
            format!("x = {}1", "- ".repeat(1000)),
        ),
        // Each call is two levels (`CallExpr`, `ArgList`); `(1)` adds two more.
        (
            "calls",
            format!("x = {}(1){}", "f(".repeat(499), ")".repeat(499)),
            format!("x = {}((1)){}", "f(".repeat(499), ")".repeat(499)),
        ),
        // `a(1)` is three levels (`CallExpr`, `ArgList`, `Literal`); each member one more.
        (
            "members",
            format!("x = a(1){}", ".b".repeat(997)),
            format!("x = a(1){}", ".b".repeat(998)),
        ),
        (
            "target",
            format!("a(1){} = 1", ".b".repeat(997)),
            format!("a(1){} = 1", ".b".repeat(998)),
        ),
    ];
    let run = move || {
        for (name, fits, too_deep) in cases {
            let p = parse_one(format!("{fits}\n").as_bytes());
            assert!(p.diagnostics().list().is_empty(), "{name}: 1,000 levels must parse");
            let p = parse_one(format!("{too_deep}\n").as_bytes());
            let diags = p.diagnostics();
            assert_eq!(diags.list().len(), 1, "{name}: 1,001 levels");
            let d = &diags.list()[0];
            assert_eq!(d.message, "an expression nested more than 1000 levels deep", "{name}");
            assert!(d.unsupported, "{name}");
        }
    };
    let thread = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(run)
        .unwrap();
    if let Err(e) = thread.join() {
        std::panic::resume_unwind(e);
    }
}

/// Errors at a BASIC word or operator the parser does not handle there are marked; a genuine syntax error is not.
#[test]
fn marked_parse_errors() {
    insta::assert_snapshot!(tree("x = a MOD b\nREDIM a(5)\nPRINT #1, x\nx = 5 TO 6\nx = 5 6\n"));
}

/// Multi-line `IF` (design D4 of `m2-parser-breadth`): `ELSEIF … THEN` and `ELSE` with a statement on the same
/// line, `ENDIF`, `ELSE IF` as an `ELSE` holding a new block, a `'` comment after `THEN`.
#[test]
fn if_blocks() {
    insta::assert_snapshot!(tree(
        "IF a THEN ' c\n  x = 1\nELSEIF b THEN y = 2\nELSE z = 3: w = 4\nEND IF\nIF a THEN\nELSE IF b THEN\n  x = 1\n  END IF\nENDIF\n"
    ));
}

/// Single-line `IF`: each `ELSE` belongs to the innermost `IF`, `THEN 10 ELSE 20`, `IF c GOTO`, `THEN :`, a whole
/// `FOR` block on the line, `THEN REM` (a single-line `IF`), empty branches, the comma scan stopping at `ELSE`, and
/// statements after a jump and `:` staying in the branch (`THEN 10: PRINT 5`, `GOTO 30: PRINT 6`).
#[test]
fn single_line_if() {
    insta::assert_snapshot!(tree(
        "IF a THEN IF b THEN x ELSE y ELSE z\nIF a THEN 10 ELSE 20\nIF a GOTO 30 ELSE PRINT 1\nIF a THEN 10: PRINT 5\nIF a GOTO 30: PRINT 6\nIF a THEN : FOR i = 1 TO 2: PRINT i: NEXT: PRINT 2\nIF a THEN REM c\nIF a THEN ELSE PRINT 3\nIF a THEN PRINT 4 ELSE\nIF a THEN q(1) = 2 ELSE s 1, 2\n"
    ));
}

/// `FOR`/`NEXT` (`STEP`, `NEXT j, i` closing two blocks), `DO` with a condition at either end, `WHILE`/`WEND`, and
/// the `EXIT` forms inside them.
#[test]
fn loops() {
    insta::assert_snapshot!(tree(
        "FOR i = 1 TO 9 STEP 2\n  FOR j% = i TO 1 STEP -1\n    EXIT FOR\nNEXT j%, i\nDO WHILE a: EXIT DO: LOOP\nDO\nLOOP UNTIL b\nWHILE c\n  IF c THEN EXIT WHILE\nWEND\n"
    ));
}

/// Spec `compiler/pipeline`, "Statement not compiled yet": a `DoBlock` holding a `SwapStmt` (`sema` marks the `SWAP`,
/// `tests\frontend\blocks_not_compiled_yet.bas`).
#[test]
fn swap_in_a_do() {
    insta::assert_snapshot!(tree("DO: SWAP a, b: LOOP UNTIL a > 0\n"));
}

/// `SELECT CASE` and `SELECT EVERYCASE`: `IS`, ranges, lists, `CASE ELSE`, a comment before the first `CASE`,
/// `EXIT SELECT` and `EXIT CASE`.
#[test]
fn select_case() {
    insta::assert_snapshot!(tree(
        "SELECT CASE x\n  ' c\n  CASE IS < 1: PRINT 1\n  CASE 1, 2 TO 3, IS = 6\n    EXIT SELECT\n  CASE ELSE\nEND SELECT\nSELECT EVERYCASE y\n  CASE 1: EXIT CASE\nEND SELECT\n"
    ));
}

/// `TYPE` fields: `name AS type`, `AS type name, ...`, element arrays with bounds, `_DYNAMIC`/`_STATIC` before and
/// after the name, fixed-length strings, `_UNSIGNED`.
#[test]
fn type_block() {
    insta::assert_snapshot!(tree(
        "TYPE t\n  a AS LONG\n  AS INTEGER b, c(1 TO 2)\n  s AS STRING * 8\n  AS STRING * 4 u, v\n  w AS _UNSIGNED _BYTE\n\n  e(-1 TO 1, 3) _DYNAMIC AS t2\n  _STATIC f(9) AS DOUBLE\nEND TYPE\n"
    ));
}

/// `DECLARE LIBRARY` blocks: the header forms, `ALIAS` as a string or a name, `BYVAL` parameters, a FUNCTION
/// without parentheses, and a comment.
#[test]
fn declare_library() {
    insta::assert_snapshot!(tree(
        "DECLARE DYNAMIC LIBRARY \"a\", \"b\"\n  FUNCTION f& ALIAS \"g\" (BYVAL x AS LONG, y AS _OFFSET)\n  ' c\n  SUB s ALIAS t\n  FUNCTION h~&\nEND DECLARE\n"
    ));
}

/// `DEF FN` in both forms (the old compiler rejects them; `sema` says so); `DEF SEG` is another statement.
#[test]
fn def_fn() {
    insta::assert_snapshot!(tree(
        "DEF FNa (x) = x * 2\nDEF FNb\n  EXIT DEF\n  FNb = 1\nEND DEF\nDEF SEG = 0\n"
    ));
}

/// Block recovery (design D4): a closer of an outer block ends the inner ones with one error; stray closers;
/// missing closers at the end of the file and at a SUB header; a closer in a single-line `IF`; a block left open
/// in one; a statement before the first `CASE`; a statement inside `TYPE`; `NEXT` naming too many variables;
/// `EXIT` outside its block; a second `ELSE`.
#[test]
fn block_recovery() {
    insta::assert_snapshot!(tree(
        "FOR i = 1 TO 2\n  IF a THEN\nNEXT\nEND IF\nWEND\nFOR k = 1 TO 2: IF a THEN NEXT\nNEXT k\nIF a THEN DO\nSELECT CASE x\n  PRINT 1\nEND SELECT\nTYPE t\n  PRINT 2\nEND TYPE\nFOR j = 1 TO 2: NEXT j, i\nEXIT DO\nIF a THEN\nELSE\nELSE\nEND IF\nIF a THEN x ELSE y ELSE z\nWHILE b\nSUB s\nEND SUB\nDO\n"
    ));
}

/// Spec `compiler/pipeline`, "Missing END IF": one error at the header's line; the statements after it are parsed.
#[test]
fn spec_missing_end_if() {
    let p = parse_one(b"x = 1\nIF a THEN\nPRINT 2\ny = 3\n");
    let diags = p.diagnostics();
    assert_eq!(diags.list().len(), 1);
    assert_eq!(diags.list()[0].span.start, 6);
    let dump = dump_tree(p.main().root(), b"x = 1\nIF a THEN\nPRINT 2\ny = 3\n");
    assert!(
        dump.contains("PrintStmt 16..23") && dump.contains("AssignStmt 24..29"),
        "{dump}"
    );
}

/// Spec `compiler/pipeline`, "One NEXT closes two loops".
#[test]
fn spec_one_next_closes_two_loops() {
    let p = parse_one(b"FOR i = 1 TO 2\nFOR j = 1 TO 2\nNEXT j, i\nPRINT 1\n");
    assert!(p.diagnostics().list().is_empty());
}

/// Labels stand only at the start of a line, after an optional line number (measured, group 6 review).
#[test]
fn labels_only_at_line_start() {
    insta::assert_snapshot!(tree("10 lab: PRINT 1\nWHILE a: s: WEND\n"));
}

#[test]
fn error_cap() {
    let src = "FOR\n".repeat(150);
    let p = parse_one(src.as_bytes());
    assert_eq!(p.diagnostics().error_count(), 100);
    assert!(p.diagnostics().is_capped());
}
