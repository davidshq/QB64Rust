//! The layout rules one by one (task 4.3 of `m5b-formatter`): each test names the part of QB64pe's sources it follows,
//! and each expected text is what the clone's `qb64pe -y` wrote for the same input (2026-10-25, the five defaults
//! given explicitly, the other settings as the test says). The oracle (`crates\driver\tests\it\format_oracle.rs`) holds
//! the rules to whole programs.

use qb64rust_base::layout::{KeywordStyle, Settings};
use qb64rust_base::{FileId, SourceMap};
use qb64rust_syntax::{LoadError, Loader, parse_settled};

/// Included files by name, for the test of names declared in an include.
struct Files(Vec<(&'static str, &'static str)>);

impl Loader for Files {
    fn load(&mut self, map: &mut SourceMap, _from: FileId, path: &[u8]) -> Result<FileId, LoadError> {
        let (name, text) = self
            .0
            .iter()
            .find(|(n, _)| n.as_bytes() == path)
            .ok_or(LoadError::NotFound)?;
        Ok(map.add(*name, text.as_bytes().to_vec()))
    }
}

/// The program `src` (LF line ends, each made CR LF) formatted with `settings`, as text with LF line ends.
fn format_with(src: &str, settings: &Settings, files: Vec<(&'static str, &'static str)>) -> String {
    let mut map = SourceMap::new();
    let file = map.add("p.bas", src.replace('\n', "\r\n").into_bytes());
    let parsed = parse_settled(&mut map, file, &mut Files(files));
    let (program, diags) = qb64rust_sema::check(&map, &parsed);
    assert!(
        !diags.has_errors() && !parsed.diagnostics().has_errors(),
        "{:?}",
        diags.list()
    );
    let out = qb64rust_format::format_file(&map, &parsed, &program, file, settings, true);
    let text: String = out.iter().map(|&b| char::from(b)).collect();
    text.replace("\r\n", "\n")
}

/// The file `name` holding `src` (LF line ends) formatted as an incomplete program (spec `compiler/format`, "Names
/// in an incomplete program"): the check may report errors; the parse may not.
fn format_incomplete(name: &str, src: &str) -> String {
    format_incomplete_with(name, src, &Settings::default())
}

/// [`format_incomplete`] with `settings`.
fn format_incomplete_with(name: &str, src: &str, settings: &Settings) -> String {
    let mut map = SourceMap::new();
    let file = map.add(name, src.replace('\n', "\r\n").into_bytes());
    let parsed = parse_settled(&mut map, file, &mut Files(Vec::new()));
    assert!(!parsed.diagnostics().has_errors(), "{:?}", parsed.diagnostics().list());
    let (program, _) = qb64rust_sema::check(&map, &parsed);
    let out = qb64rust_format::format_file(&map, &parsed, &program, file, settings, false);
    let text: String = out.iter().map(|&b| char::from(b)).collect();
    text.replace("\r\n", "\n")
}

/// Scenario "A variable declared elsewhere": a variable no statement of the file declares is written as read at
/// each use.
#[test]
fn incomplete_variable_declared_elsewhere() {
    let got = format_incomplete("lib.bm", "SUB a\natkfac = 1\nATKFAC = 2\nEND SUB\n");
    assert_eq!(got, "Sub a\n    atkfac = 1\n    ATKFAC = 2\nEnd Sub\n");
}

/// Scenario "A name the file declares": `DIM` and a SUB header give their spelling; the name of a SUB that no file
/// defines stays as read.
#[test]
fn incomplete_name_the_file_declares() {
    let src = "SUB Bump\nDIM Total AS LONG\nTOTAL = 1\nEND SUB\nSUB b\nBUMP\nCALL other\nEND SUB\n";
    let got = format_incomplete("lib.bm", src);
    let want = "Sub Bump\n    Dim Total As Long\n    Total = 1\nEnd Sub\nSub b\n    Bump\n    Call other\nEnd Sub\n";
    assert_eq!(got, want);
}

/// Scenario "A complete program is not affected": a `.bas` program without errors spells an implicit variable as
/// its first use, as `qb64pe -y` does.
#[test]
fn complete_program_spells_implicit_variables() {
    assert_eq!(format("total = 1\nTOTAL = 2\n"), "total = 1\ntotal = 2\n");
}

/// The program of three files for the agreement test (task 2.5 of `include-file-program`): `defs.bi` declares the
/// shared names in capitals, `lib.bm` uses them in lower case and calls a SUB of `main.bas`.
const AGREE_MAIN: &str =
    "'$INCLUDE: 'defs.bi'\nATKFAC = 2\nDrawMap\nReport\n'$INCLUDE: 'lib.bm'\nSUB DrawMap\nEND SUB\n";
const AGREE_DEFS: &str = "COMMON SHARED ATKFAC AS INTEGER, TOTAL AS LONG\n";
const AGREE_LIB: &str = "sub Report\natkfac = atkfac + 1\ntotal = 3\ndrawmap\nprint atkfac; total\nend sub\n";

/// `lib.bm` of [`AGREE_MAIN`]'s program formatted inside it: a complete program, every name in its declaration's
/// spelling.
fn format_inside_program() -> String {
    let mut map = SourceMap::new();
    let main = map.add("main.bas", AGREE_MAIN.replace('\n', "\r\n").into_bytes());
    let files = vec![("defs.bi", AGREE_DEFS), ("lib.bm", AGREE_LIB)];
    let parsed = parse_settled(&mut map, main, &mut Files(files));
    let (program, diags) = qb64rust_sema::check(&map, &parsed);
    assert!(
        !diags.has_errors() && !parsed.diagnostics().has_errors(),
        "{:?}",
        diags.list()
    );
    let lib = map.files().find(|(_, f)| f.name == "lib.bm").map(|(id, _)| id).unwrap();
    let out = qb64rust_format::format_file(&map, &parsed, &program, lib, &Settings::default(), true);
    let text: String = out.iter().map(|&b| char::from(b)).collect();
    text.replace("\r\n", "\n")
}

/// Scenario "Alone and inside its program": the include file formatted alone and inside its program are equal but
/// for the letter case of names, and every name whose case the first changed has the same case in the second.
#[test]
fn include_file_alone_and_inside_its_program_agree() {
    let alone = format_incomplete("lib.bm", AGREE_LIB);
    let inside = format_inside_program();
    assert_eq!(
        alone,
        "Sub Report\n    atkfac = atkfac + 1\n    total = 3\n    drawmap\n    Print atkfac; total\nEnd Sub\n"
    );
    assert_eq!(
        inside,
        "Sub Report\n    ATKFAC = ATKFAC + 1\n    TOTAL = 3\n    DrawMap\n    Print ATKFAC; TOTAL\nEnd Sub\n"
    );
    assert_eq!(alone.to_ascii_lowercase(), inside.to_ascii_lowercase());
    let words = |s: &str| -> Vec<String> {
        s.split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .filter(|w| !w.is_empty())
            .map(str::to_string)
            .collect()
    };
    let typed = words(AGREE_LIB);
    for (a, b) in words(&alone).iter().zip(words(&inside).iter()) {
        // A word the two spell differently is one that formatting alone left as typed.
        if a != b {
            assert!(typed.contains(a), "`{a}` alone, `{b}` inside: alone changed its case");
        }
    }
}

/// Scenario "A statement the check rejects is not changed": the layout of `KILL` with two values in parentheses
/// would read as another statement, so the line keeps its text but for letter case.
#[test]
fn incomplete_rejected_statement_is_not_changed() {
    assert_eq!(
        format_incomplete("prog.bas", "kill (\"a\", \"b\")\n"),
        "Kill (\"a\", \"b\")\n"
    );
}

/// A program with an error in a later stage is incomplete even when its file is a `.bas` file: the implicit
/// variable is written as read, the declared one takes its spelling, keywords and built-ins as always.
#[test]
fn incomplete_program_with_an_error() {
    let src = "DIM Total AS LONG\ntotal = len(\"a\")\nzz = 1\nZZ = 2\ncall   missing\n";
    let got = format_incomplete("prog.bas", src);
    assert_eq!(
        got,
        "Dim Total As Long\nTotal = Len(\"a\")\nzz = 1\nZZ = 2\nCall missing\n"
    );
}

fn format(src: &str) -> String {
    format_with(src, &Settings::default(), Vec::new())
}

fn setting(name: &str, value: &str) -> Settings {
    let mut s = Settings::default();
    assert!(s.set(name, value).unwrap());
    s
}

/// The `IF` handler (`qb64pe.bas` 6680): `If`, the condition's layout, `Then`; one space between elements
/// (`fixoperationorder$` part I); the body one level in.
#[test]
fn if_block_spacing_and_indent() {
    assert_eq!(
        format("if x   =   1 then\nprint   x\nend if\n"),
        "If x = 1 Then\n    Print x\nEnd If\n"
    );
}

/// `SELECT CASE` and `CASE` (`qb64pe.bas` 6790-7210): `Select Case` one level, each `Case` closes the one before it
/// (`lhscontrollevel - 1`), `Is` and `To`, and the SUB a level of its own (`IndentSubs`, 5410).
#[test]
fn select_case_in_a_sub() {
    let src = "sub s(x)\nselect case x\ncase 1\nprint 1\ncase is > 2, 5 to 7\nprint 2\ncase else\nprint 3\nend select\nend sub\n";
    let want = "Sub s (x)\n    Select Case x\n        Case 1\n            Print 1\n        Case Is > 2, 5 To 7\n            Print 2\n        Case Else\n            Print 3\n    End Select\nEnd Sub\n";
    assert_eq!(format(src), want);
}

/// `IndentSubs` off (`qb64pe.bas` 5410: the SUB adds no level).
#[test]
fn indent_subs_off() {
    let got = format_with(
        "sub s\nif 1 then\nprint 1\nend if\nend sub\n",
        &setting("IndentSubs", "0"),
        Vec::new(),
    );
    assert_eq!(got, "Sub s\nIf 1 Then\n    Print 1\nEnd If\nEnd Sub\n");
}

/// `ELSEIF` and `ELSE` stand one level out (`qb64pe.bas` 6633, 6677: `lhscontrollevel - 1`).
#[test]
fn else_and_elseif() {
    assert_eq!(
        format("if a then\nprint 1\nelseif b then\nprint 2\nelse\nprint 3\nend if\n"),
        "If a Then\n    Print 1\nElseIf b Then\n    Print 2\nElse\n    Print 3\nEnd If\n"
    );
}

/// A `TYPE` block (`qb64pe.bas` 4378-4440, 12650: `definingtype`): the fields one level in, the name as written, the
/// members respelled as declared at each use.
#[test]
fn type_block_and_members() {
    assert_eq!(
        format("type pt\nxp as long\nyp as integer\nend type\ndim p as pt\np.XP = 1\n"),
        "Type pt\n    xp As Long\n    yp As Integer\nEnd Type\nDim p As pt\np.xp = 1\n"
    );
}

/// Labels and line numbers at the level of the statements around them (`qb64pe.bas` 3945-4070); a label used
/// elsewhere as the label table spells it.
#[test]
fn labels_and_line_numbers() {
    assert_eq!(
        format("for i = 1 to 2\nlab:\n10 print i\nnext\ngoto Lab\n"),
        "For i = 1 To 2\n    lab:\n    10 Print i\nNext\nGoTo lab\n"
    );
}

/// `:` glued to the statement before it (`layout$ + sp2 + ":"`, `qb64pe.bas` 4114), the comment after one space and
/// a `REM` with its text as written (`lineformat$`, 25200-25440).
#[test]
fn colons_and_comments() {
    assert_eq!(
        format("x = 1:print   :   y = 2   ' note\nrem   kept\n"),
        "x = 1: Print: y = 2 ' note\nRem   kept\n"
    );
}

/// A metacommand in a comment in the keyword style (`lineformat$`, 25454-25480), indented as a line of its block.
#[test]
fn metacommand_in_a_comment() {
    let got = format_with(
        "if 1 then\n        '$include:'inc.bi'\nend if\n",
        &Settings::default(),
        vec![("inc.bi", "' empty\r\n")],
    );
    assert_eq!(got, "If 1 Then\n    '$Include:'inc.bi'\nEnd If\n");
}

/// Names as their first declaration spells them (`id.cn`, `fixoperationorder$` part H): a variable after its `DIM`,
/// a function called before its header, the function's result in its body.
#[test]
fn names_spelled_as_declared() {
    assert_eq!(
        format("dim myVar as long\nMYVAR = 1\nx = myfunc(2)\nfunction myFunc (n)\nmyfunc = n\nend function\n"),
        "Dim myVar As Long\nmyVar = 1\nx = myFunc(2)\nFunction myFunc (n)\n    myFunc = n\nEnd Function\n"
    );
}

/// A name declared in an included file respells the main file, which alone is written.
#[test]
fn names_from_an_include() {
    let got = format_with(
        "'$include:'inc.bi'\nTOTAL = 1\n",
        &Settings::default(),
        vec![("inc.bi", "DIM SHARED total AS LONG\r\n")],
    );
    assert_eq!(got, "'$Include:'inc.bi'\ntotal = 1\n");
}

/// Built-ins with the old table's spelling (`SCase$(RTRIM$(id.cn))`).
#[test]
fn built_in_spelling() {
    assert_eq!(
        format("x = len(\"a\") + _rgb32(1, 2, 3) + instr(\"ab\", \"b\")\n"),
        "x = Len(\"a\") + _RGB32(1, 2, 3) + InStr(\"ab\", \"b\")\n"
    );
}

/// `?` laid out as `Print` (`xprint`: `l$ = SCase$("Print")`), and a string's closing quote added (`lineformat$`).
#[test]
fn question_mark_and_closing_quote() {
    assert_eq!(
        format("dim myVar\nprint x;:?MYVAR\n"),
        "Dim myVar\nPrint x;: Print myVar\n"
    );
    assert_eq!(format("print \"abc\n"), "Print \"abc\"\n");
}

/// A line joined by `_` and the line it joins are written as read; so is a metacommand line after them, which does
/// not go through `lineformat$` and keeps its `linecontinuation` (`qb64pe.bas` 12684).
#[test]
fn continued_lines_as_read() {
    assert_eq!(
        format("print   1; _\n2\n$console\nprint 3\n"),
        "print   1; _\n2\n$console\nPrint 3\n"
    );
}

/// `'$FORMAT:OFF` turns `IDEAutoIndent` and `IDEAutoLayout` off (`lineformat$` 25454): the lines keep their indent and
/// spacing, the letter case is still carried over (`apply_layout_indent$`).
#[test]
fn format_off() {
    assert_eq!(
        format("'$format:off\n  for i=1 to 2:next\n'$format:on\nfor i=1 to 2:next\n"),
        "'$Format:Off\n  For i=1 To 2:Next\n'$Format:On\nFor i = 1 To 2: Next\n"
    );
}

/// `AutoLayout` off: the user's spacing with the layout's letter case (`apply_layout_indent$`, `format.bas` 53-97).
#[test]
fn layout_off_keeps_spacing() {
    let got = format_with(
        "if x   =   1 then\nprint   \"a\"\nend if\n",
        &setting("AutoLayout", "false"),
        Vec::new(),
    );
    assert_eq!(got, "If x   =   1 Then\n    Print   \"a\"\nEnd If\n");
}

/// The keyword styles (`SCase$`, `SCase2$`): capitals and lower case, names as declared in both.
#[test]
fn keyword_styles() {
    let upper = setting("KeywordCapitals", "true");
    assert_eq!(upper.keywords, KeywordStyle::Upper);
    assert_eq!(
        format_with("print 1: if x then y = len(\"a\")\n", &upper, Vec::new()),
        "PRINT 1: IF x THEN y = LEN(\"a\")\n"
    );
    let lower = setting("KeywordLowercase", "true");
    assert_eq!(
        format_with("PRINT 1: IF X THEN Y = LEN(\"A\")\n", &lower, Vec::new()),
        "print 1: if X then Y = len(\"A\")\n"
    );
}

/// Numbers as `lineformat$` keeps them: a plain integer without its leading zeros, any other in capitals as typed.
#[test]
fn numbers() {
    assert_eq!(
        format("x = 007 + 1.50 + 1e3 + &hff + 3#\n"),
        "x = 7 + 1.50 + 1E3 + &HFF + 3#\n"
    );
}

/// Signs (`fixoperationorder$` parts B and C): `-+` is `-`, `++` is `+`, an operator's `- -` is gone, a negation is
/// glued to its operand, `- -` at the start stays.
#[test]
fn signs() {
    assert_eq!(
        format("print -+x; 2 + +x; 2 * - -x; - -x; 2 - - -x\n"),
        "Print -x; 2 + x; 2 * x; --x; 2 - x\n"
    );
}

/// `PRINT` items with no separator around a string get `;` (`qb64pe.bas` 11270).
#[test]
fn print_semicolons_added() {
    assert_eq!(format("print \"a\" \"b\" x \"c\"\n"), "Print \"a\"; \"b\"; x; \"c\"\n");
}

/// Built-in statements by their templates (`seperateargs`, the SUB-call layout of `qb64pe.bas` 11640-12415).
#[test]
fn template_statements() {
    assert_eq!(
        format("line (0,0)-step(9,9),,bf\nlocate ,5\nopen \"f\" for input as #1\nclose #1\npset step(1,2)\n"),
        "Line (0, 0)-Step(9, 9), , BF\nLocate , 5\nOpen \"f\" For Input As #1\nClose #1\nPSet Step(1, 2)\n"
    );
}

/// `ON TIMER(n) GOSUB` (`qb64pe.bas` 7731) and `TIMER ON` with the spacing of `sub_timer` (11640).
#[test]
fn timer_events() {
    assert_eq!(
        format("on timer(1) gosub t\ntimer on\nend\nt: return\n"),
        "On Timer(1) GoSub t\nTimer On\nEnd\nt: Return\n"
    );
}

/// `CALL` of a built-in, with parentheses and without (`usecall`, `qb64pe.bas` 11603).
#[test]
fn call_of_a_built_in() {
    assert_eq!(
        format("call _printstring((1, 2), \"x\")\ncall cls\n"),
        "Call _PrintString((1, 2), \"x\")\nCall Cls\n"
    );
}

/// `CALL ABSOLUTE(…)` is spelled from its layout (`qb64pe.bas` 10688) only with parentheses (`IF n > 2`, 10622);
/// without them `ABSOLUTE` is the SUB's name, spelled as declared.
#[test]
fn call_absolute_only_with_parentheses() {
    assert_eq!(
        format("sub absolute\nprint 1\nend sub\ndim a as long\ncall absolute\ncall absolute(a)\n"),
        "Sub absolute\n    Print 1\nEnd Sub\nDim a As Long\nCall absolute\nCall Absolute(a)\n"
    );
}

/// `ON n GOTO|GOSUB` keeps the word in capitals (`xongotogosub`: `l$ + sp + e2$`).
#[test]
fn on_goto_word_in_capitals() {
    assert_eq!(
        format("on n goto a, b\na:\nb:\non n gosub a\n"),
        "On n GOTO a, b\na:\nb:\nOn n GOSUB a\n"
    );
}

/// `DIM a()` lays out no parentheses, `COMMON` lays out `()` whatever its bounds (`qb64pe.bas` 9043, `allocarray`).
#[test]
fn empty_and_common_bounds() {
    assert_eq!(
        format("dim a() as long\nredim a(1 to 3) as long\ncommon shared c(3) as long\n"),
        "Dim a As Long\nReDim a(1 To 3) As Long\nCommon Shared c() As Long\n"
    );
}

/// `DATA` items without the blanks around them (`lineformat$`'s data scan).
#[test]
fn data_items() {
    assert_eq!(format("data 0,   0, \"a, b\"  ,x y\n"), "Data 0,0,\"a, b\",x y\n");
}

/// `_DEFINE` and `DEFLNG` (`qb64pe.bas` 6110-6160): letters in capitals, `_DEFINE`'s type in capitals with a `sp`
/// after it, so a `:` after it has a space before it.
#[test]
fn define_and_deftype() {
    assert_eq!(
        format("_define a as integer: x = 1\ndeflng b-c, d\n"),
        "_Define A As INTEGER : x = 1\nDefLng B-C, D\n"
    );
}

/// `DECLARE SUB` sets no layout (`qb64pe.bas` 4448): its line as read, trimmed.
#[test]
fn declare_sub_as_read() {
    assert_eq!(
        format("declare sub   s (x)\nsub s(x)\nend sub\n"),
        "declare sub   s (x)\nSub s (x)\nEnd Sub\n"
    );
}

/// `OPTION BASE` writes the number element alone (`qb64pe.bas` 11514).
#[test]
fn option_base_number_alone() {
    assert_eq!(format("option base 0 + 1\n"), "Option Base 0\n");
}

/// A numeric array named alone in a slot that takes one gets `()` (`targettyp = -3`, `qb64pe.bas` 11766).
#[test]
fn array_argument_gets_parentheses() {
    assert_eq!(
        format("dim a(10)\nget (0, 0)-(1, 1), a\nput (0, 0), a, pset\n"),
        "Dim a(10)\nGet (0, 0)-(1, 1), a()\nPut (0, 0), a(), PSet\n"
    );
}

/// `CLS :` is the statement, not a label (`validlabel`).
#[test]
fn built_in_before_a_colon() {
    assert_eq!(format("cls : print 1\n"), "Cls: Print 1\n");
}

/// D-038: a number in `_ARRAYCOPY`'s subscripts as written, where `-y` writes its internal element
/// (`1.5E+0,1.5`) and so another program.
#[test]
fn arraycopy_number_as_written() {
    assert_eq!(
        format("dim g(9), d(9)\n_arraycopy g(1.5 to 3.5) to d(0)\n"),
        "Dim g(9), d(9)\n_ArrayCopy g(1.5 To 3.5) To d(0)\n"
    );
}

// The rewrites `OneStatementPerLine` and `BlockIf` (OpenSpec change `format-statement-lines`): one test for each
// scenario of the delta of spec `compiler/format`. QB64pe has neither setting; what its compiler makes of the
// rewritten forms is measured by `verification\v72_*`.

/// The settings with the two rewrites as given.
fn rewrites(one_statement_per_line: bool, block_if: bool) -> Settings {
    let mut s = Settings::default();
    s.one_statement_per_line = one_statement_per_line;
    s.block_if = block_if;
    s
}

/// The typed tree of a complete program, the number of each line left out.
fn typed(src: &str) -> String {
    let mut map = SourceMap::new();
    let file = map.add("p.bas", src.replace('\n', "\r\n").into_bytes());
    let parsed = parse_settled(&mut map, file, &mut Files(Vec::new()));
    let (program, diags) = qb64rust_sema::check(&map, &parsed);
    assert!(
        !diags.has_errors() && !parsed.diagnostics().has_errors(),
        "{src}: {:?}",
        diags.list()
    );
    let dump = qb64rust_sema::dump_typed(&program);
    let mut out = String::with_capacity(dump.len());
    let mut rest = dump.as_str();
    while let Some(at) = rest.find("line ") {
        out.push_str(&rest[..at + 5]);
        rest = rest[at + 5..].trim_start_matches(|c: char| c.is_ascii_digit());
        out.push('#');
    }
    out.push_str(rest);
    out
}

/// `src` formatted with the two rewrites as given, held to spec `compiler/format`, "A rewritten program is the
/// same program": the same typed tree as `src` but for line numbers, and the same text when formatted again.
fn rewrite(src: &str, one_statement_per_line: bool, block_if: bool) -> String {
    let s = rewrites(one_statement_per_line, block_if);
    let out = format_with(src, &s, Vec::new());
    assert_eq!(typed(&out), typed(src), "the typed tree of\n{out}");
    assert_eq!(format_with(&out, &s, Vec::new()), out, "formatted again");
    out
}

/// `src` with `OneStatementPerLine` alone.
fn split(src: &str) -> String {
    rewrite(src, true, false)
}

/// `src` with `BlockIf` alone.
fn block_if(src: &str) -> String {
    rewrite(src, false, true)
}

/// `src` with both rewrites.
fn both(src: &str) -> String {
    rewrite(src, true, true)
}

/// Scenarios "Off by default" and "On the command line".
#[test]
fn rewrite_settings_off_by_default() {
    assert_eq!(format("x = 0: y = 0\n"), "x = 0: y = 0\n");
    assert_eq!(
        format_with("x = 0: y = 0\n", &setting("OneStatementPerLine", "true"), Vec::new()),
        "x = 0\ny = 0\n"
    );
}

/// Scenario "Two assignments".
#[test]
fn split_two_assignments() {
    assert_eq!(split("x = 0: y = 0\n"), "x = 0\ny = 0\n");
}

/// Scenario "A loop on one line".
#[test]
fn split_a_loop_on_one_line() {
    assert_eq!(
        split("for i = 1 to 3: print i: next\n"),
        "For i = 1 To 3\n    Print i\nNext\n"
    );
}

/// Scenario "SELECT CASE on one line".
#[test]
fn split_select_case_on_one_line() {
    assert_eq!(
        split("x = 1\nSELECT CASE x: CASE 1: b = 5: CASE ELSE: b = 6: END SELECT\n"),
        "x = 1\nSelect Case x\n    Case 1\n        b = 5\n    Case Else\n        b = 6\nEnd Select\n"
    );
}

/// Scenario "A statement after ELSEIF and after ELSE".
#[test]
fn split_after_elseif_and_else() {
    let src = "IF a THEN\nx = 1\nELSEIF b THEN x = 2: b = 3\nELSE x = 3\nEND IF\n";
    let want = "If a Then\n    x = 1\nElseIf b Then\n    x = 2\n    b = 3\nElse\n    x = 3\nEnd If\n";
    assert_eq!(split(src), want);
    assert_eq!(both(src), want);
}

/// Scenario "One statement and a colon": no cut, so the colon stays.
#[test]
fn split_one_statement_and_a_colon() {
    assert_eq!(split("x = 1:\n"), "x = 1:\n");
    assert_eq!(split(": x = 1\n"), ": x = 1\n");
}

/// Scenario "Colons inside the IF stay".
#[test]
fn split_keeps_a_single_line_if_whole() {
    assert_eq!(split("IF a THEN b = 1: x = 2\n"), "If a Then b = 1: x = 2\n");
    // A colon of a `FOR` inside a branch is inside the `IF` too.
    assert_eq!(
        split("IF 1 THEN FOR i = 1 TO 3: PRINT i;: NEXT: PRINT\n"),
        "If 1 Then For i = 1 To 3: Print i;: Next: Print\n"
    );
}

/// Scenario "A statement before the IF".
#[test]
fn split_a_statement_before_the_if() {
    assert_eq!(
        split("x = 1: IF a THEN b = 1: x = 2\n"),
        "x = 1\nIf a Then b = 1: x = 2\n"
    );
}

/// Scenario "A label before two statements".
#[test]
fn split_a_label_stays_with_the_first_statement() {
    assert_eq!(split("top: x = 9: b = 9\nGOTO top\n"), "top: x = 9\nb = 9\nGoTo top\n");
    assert_eq!(split("10 x = 9: b = 9\n"), "10 x = 9\nb = 9\n");
    assert_eq!(split("10 : x = 9: b = 9\n"), "10: x = 9\nb = 9\n");
}

/// Scenario "A SUB's name and a colon".
#[test]
fn split_a_sub_name_and_a_colon() {
    let src = "DrawBoard: PRINT \"Ready\": x = 1\nSUB DrawBoard\nEND SUB\n";
    assert_eq!(
        split(src),
        "DrawBoard: Print \"Ready\"\nx = 1\nSub DrawBoard\nEnd Sub\n"
    );
}

/// Scenario "A built-in statement's name and a colon".
#[test]
fn split_a_built_in_statement_and_a_colon() {
    assert_eq!(
        split("CLS: LOCATE 1, 1: PRINT \"Ready\"\n"),
        "Cls\nLocate 1, 1\nPrint \"Ready\"\n"
    );
    assert_eq!(split("x = 1: CLS:\n"), "x = 1\nCls\n");
    // Alone it is not cut, and keeps its colon.
    assert_eq!(split("CLS:\n"), "Cls:\n");
}

/// Scenarios "A continued line", "Format off" and "A metacommand in the comment".
#[test]
fn rewrites_leave_lines_alone() {
    assert_eq!(both("x = 0: y = _\n1\n"), "x = 0: y = _\n1\n");
    assert_eq!(
        both("'$FORMAT:OFF\nx = 0: y = 0\n'$FORMAT:ON\nx = 0: y = 0\n"),
        "'$Format:Off\nx = 0: y = 0\n'$Format:On\nx = 0\ny = 0\n"
    );
    assert_eq!(split("DIM a(5): x = 1 '$DYNAMIC\n"), "Dim a(5): x = 1 '$Dynamic\n");
    assert_eq!(
        block_if("$IF WIN THEN\nIF a THEN x = 1\n$END IF\n"),
        "$If WIN Then\n    If a Then\n        x = 1\n    End If\n$End If\n"
    );
}

/// A line of a `$IF` branch that is not taken is written as read.
#[test]
fn rewrites_leave_an_inactive_branch_alone() {
    assert_eq!(
        both("$IF X THEN\nx = 0: y = 0\n$END IF\n"),
        "$If X Then\nx = 0: y = 0\n$End If\n"
    );
}

/// Scenario "AutoLayout off", and the same for `AutoIndent`.
#[test]
fn rewrites_need_layout_and_indent() {
    let mut s = rewrites(true, true);
    s.auto_layout = false;
    assert_eq!(format_with("x = 0:  y = 0\n", &s, Vec::new()), "x = 0:  y = 0\n");
    let mut s = rewrites(true, true);
    s.auto_indent = false;
    assert_eq!(format_with("  x = 0: y = 0\n", &s, Vec::new()), "  x = 0: y = 0\n");
}

/// Scenarios "A SUB that is not defined" and "An include file alone".
#[test]
fn rewrites_need_a_program_without_errors() {
    let s = rewrites(true, true);
    assert_eq!(
        format_incomplete_with("p.bas", "call missing: x = 1\n", &s),
        "Call missing: x = 1\n"
    );
    assert_eq!(
        format_incomplete_with("lib.bm", "SUB a\nx = 0: y = 0\nEND SUB\n", &s),
        "Sub a\n    x = 0: y = 0\nEnd Sub\n"
    );
}

/// Scenarios "A comment after two statements" and "One statement keeps its comment"; a comment above a line inside a
/// block is at the block's indent.
#[test]
fn comment_of_a_rewritten_line() {
    assert_eq!(
        split("x = 0: y = 0 ' reset the cursor\n"),
        "' reset the cursor\nx = 0\ny = 0\n"
    );
    assert_eq!(both("x = 0 ' reset\n"), "x = 0 ' reset\n");
    assert_eq!(
        split("FOR i = 1 TO 2\nx = i: y = i ' both\nNEXT\n"),
        "For i = 1 To 2\n    ' both\n    x = i\n    y = i\nNext\n"
    );
    assert_eq!(split("x = 0: y = 0 REM reset\n"), "Rem reset\nx = 0\ny = 0\n");
}

/// Scenario "A REM comment after a single-line IF".
#[test]
fn rem_comment_of_a_single_line_if() {
    let want = "Rem first turn\nIf a Then\n    x = 1\nEnd If\n";
    assert_eq!(block_if("IF a THEN x = 1: REM first turn\n"), want);
    assert_eq!(both("IF a THEN x = 1: REM first turn\n"), want);
}

/// Scenario "Both branches".
#[test]
fn block_if_both_branches() {
    let want = "If a Then\n    x = 1\nElse\n    x = 2\nEnd If\n";
    assert_eq!(block_if("IF a THEN x = 1 ELSE x = 2\n"), want);
    assert_eq!(both("IF a THEN x = 1 ELSE x = 2\n"), want);
}

/// Scenarios "Several statements in a branch, both settings" and "…, BlockIf alone".
#[test]
fn block_if_several_statements_in_a_branch() {
    let src = "IF hp <= 0 THEN PRINT \"Dead\": lives = lives - 1\n";
    assert_eq!(
        both(src),
        "If hp <= 0 Then\n    Print \"Dead\"\n    lives = lives - 1\nEnd If\n"
    );
    assert_eq!(
        block_if(src),
        "If hp <= 0 Then\n    Print \"Dead\": lives = lives - 1\nEnd If\n"
    );
}

/// Scenario "An IF inside a branch".
#[test]
fn block_if_inside_a_branch() {
    let want = "If a Then\n    If b Then\n        x = 1\n    Else\n        x = 2\n    End If\nEnd If\n";
    assert_eq!(block_if("IF a THEN IF b THEN x = 1 ELSE x = 2\n"), want);
    assert_eq!(both("IF a THEN IF b THEN x = 1 ELSE x = 2\n"), want);
}

/// Scenario "An empty first branch".
#[test]
fn block_if_an_empty_first_branch() {
    let want = "If a Then\nElse\n    x = 4\nEnd If\n";
    assert_eq!(block_if("IF a THEN ELSE x = 4\n"), want);
    assert_eq!(both("IF a THEN ELSE x = 4\n"), want);
}

/// Scenario "A colon right after THEN or ELSE".
#[test]
fn block_if_a_colon_after_then_or_else() {
    for f in [block_if, both] {
        assert_eq!(f("IF x THEN: x = 4\n"), "If x Then\n    x = 4\nEnd If\n");
        assert_eq!(f("IF x THEN ELSE: x = 3\n"), "If x Then\nElse\n    x = 3\nEnd If\n");
    }
}

/// Scenario "A statement before the IF, BlockIf alone" (measured with `qb64pe.exe`: `v72_f_colon_then_if_b`).
#[test]
fn block_if_a_statement_before_the_if() {
    assert_eq!(
        block_if("x = 1: IF a THEN b = 1\n"),
        "x = 1: If a Then\n    b = 1\nEnd If\n"
    );
    assert_eq!(
        both("x = 1: IF a THEN b = 1\n"),
        "x = 1\nIf a Then\n    b = 1\nEnd If\n"
    );
}

/// Scenario "THEN and a line number".
#[test]
fn block_if_then_and_a_line_number() {
    let src = "IF x THEN 100 ELSE 200\n100 PRINT 1\n200 PRINT 2\n";
    let want = "If x Then\n    GoTo 100\nElse\n    GoTo 200\nEnd If\n100 Print 1\n200 Print 2\n";
    assert_eq!(block_if(src), want);
    assert_eq!(both(src), want);
}

/// Scenario "IF and GOTO without THEN".
#[test]
fn block_if_goto_without_then() {
    let src = "IF y GOTO 100\n100 PRINT 1\n";
    assert_eq!(block_if(src), "If y Then\n    GoTo 100\nEnd If\n100 Print 1\n");
    assert_eq!(both(src), "If y Then\n    GoTo 100\nEnd If\n100 Print 1\n");
}

/// Scenario "A line number before a single-line IF".
#[test]
fn block_if_after_a_line_number() {
    let src = "10 IF x THEN 20\n20 PRINT 1\n";
    assert_eq!(block_if(src), "10 If x Then\n    GoTo 20\nEnd If\n20 Print 1\n");
    assert_eq!(both(src), "10 If x Then\n    GoTo 20\nEnd If\n20 Print 1\n");
}

/// Scenario "Rewrites for the project", as settings: a branch of two statements with both.
#[test]
fn block_if_with_both_settings() {
    assert_eq!(
        both("IF a THEN x = 1: y = 2\n"),
        "If a Then\n    x = 1\n    y = 2\nEnd If\n"
    );
}

/// Task 4.2 of `format-statement-lines`: a single-line `IF` three deep, one inside a `SELECT CASE` on one line, one
/// with a `REM` comment inside a `FOR`, and one inside `'$FORMAT:OFF`; each output's typed tree is its source's.
#[test]
fn block_if_deeper_forms() {
    assert_eq!(
        both("IF a THEN IF b THEN IF c THEN x = 1 ELSE x = 2\n"),
        "If a Then\n    If b Then\n        If c Then\n            x = 1\n        Else\n            x = 2\n        End If\n    \
         End If\nEnd If\n"
    );
    assert_eq!(
        both("SELECT CASE a: CASE 1: IF b THEN x = 1 ELSE x = 2\nEND SELECT\n"),
        "Select Case a\n    Case 1\n        If b Then\n            x = 1\n        Else\n            x = 2\n        End If\n\
         End Select\n"
    );
    assert_eq!(
        block_if("FOR i = 1 TO 2\nIF i = 2 THEN x = i: REM last\nNEXT\n"),
        "For i = 1 To 2\n    Rem last\n    If i = 2 Then\n        x = i\n    End If\nNext\n"
    );
    assert_eq!(
        both("'$FORMAT:OFF\nIF a THEN x = 1\n'$FORMAT:ON\n"),
        "'$Format:Off\nIf a Then x = 1\n'$Format:On\n"
    );
}
