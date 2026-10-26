//! The layout rules one by one (task 4.3 of `m5b-formatter`): each test names the part of QB64pe's sources it follows,
//! and each expected text is what the clone's `qb64pe -y` wrote for the same input (2026-10-25, the five defaults
//! given explicitly, the other settings as the test says). The oracle (`crates\driver\tests\format_oracle.rs`) holds
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
    let out = qb64rust_format::format_file(&map, &parsed, &program, file, settings);
    let text: String = out.iter().map(|&b| char::from(b)).collect();
    text.replace("\r\n", "\n")
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
