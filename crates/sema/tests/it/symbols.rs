//! The symbol table (design D12): snapshots of `dump_symbols` per spec scenario, and position lookups.

use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_sema::{Program, SymbolKind, check, dump_symbols};
use qb64rust_syntax::{NoLoader, parse_settled};

/// Parses and checks one file; the parse errors and the check's errors.
fn check_any(text: &[u8]) -> (SourceMap, Program, Diagnostics, Diagnostics) {
    let mut map = SourceMap::new();
    let file = map.add("t.bas", text.to_vec());
    let parsed = parse_settled(&mut map, file, &mut NoLoader);
    let (program, diags) = check(&map, &parsed);
    (map, program, parsed.diagnostics(), diags)
}

fn check_source(text: &[u8]) -> (SourceMap, Program) {
    let (map, program, parse_diags, diags) = check_any(text);
    assert!(!diags.has_errors() && !parse_diags.has_errors(), "{diags:?}");
    (map, program)
}

/// Scenario "Definition and references of a variable".
#[test]
fn definition_and_references_of_a_variable() {
    let (map, p) = check_source(b"$CONSOLE:ONLY\nDIM n AS LONG\nn = 1\nPRINT n&\n");
    insta::assert_snapshot!(dump_symbols(&p, &map));
}

/// An implicit variable is defined by its first use in the source, even when the value of its assignment is
/// resolved before the target; suffixed and plain names of other types are other symbols.
#[test]
fn implicit_variables() {
    let src = b"$CONSOLE:ONLY\nx = x + 1\nPRINT x, x%, y$\nx% = 2\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map));
}

/// An array and a `TYPE` variable (m2-arrays-and-types task 4.2): an element's or a member's use is a reference
/// of its variable, at the name; a dotted plain name is a variable of its own.
#[test]
fn arrays_and_members() {
    let src =
        b"$CONSOLE:ONLY\nTYPE t\nm AS LONG\nEND TYPE\nDIM a(3) AS t, p AS t\na(1).m = p.m\nPRINT LBOUND(a)\nq.r = 1\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map), @r"
    Type T def 2:6 refs 5:13 5:21
    Member T.M : LONG def 3:1 refs 6:6 6:12
    Var A() : T def 5:5 refs 6:1 7:14
    Var P : T def 5:16 refs 6:10
    Var Q.R : SINGLE def 8:1
    ");
}

/// User types and their members (design D3 of `m5b-formatter`): a type is defined by its `TYPE` line and referenced
/// by each `AS` that names it, a member by its name in the block and each access: the part after the dot of a
/// dotted name, with blanks around the dot too, and the name after `.` of an element.
#[test]
fn types_and_members() {
    let src = b"$CONSOLE:ONLY\nTYPE pt\nxp AS LONG\nEND TYPE\nTYPE box\nc AS PT\nEND TYPE\nDIM a(3) AS Pt, p AS PT, b AS box\na(1).XP = p.Xp + p . xP\nb.c.xp = 2\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map), @r"
    Type PT def 2:6 refs 6:6 8:13 8:22
    Member PT.XP : LONG def 3:1 refs 9:6 9:13 9:22 10:5
    Type BOX def 5:6 refs 8:31
    Member BOX.C : PT def 6:1 refs 10:3
    Var A() : PT def 8:5 refs 9:1
    Var P : PT def 8:17 refs 9:11 9:18
    Var B : BOX def 8:26 refs 10:1
    ");
    // The member `xP` of `p . xP` (line 9, column 22) is the member defined at `xp` (line 3).
    let line9 = src.split(|&c| c == b'\n').take(8).map(|l| l.len() + 1).sum::<usize>();
    let at = |col: usize| u32::try_from(line9 + col - 1).unwrap();
    let member = p.symbols.at(FileId(0), at(22)).unwrap();
    assert!(matches!(p.symbols.get(member).kind, SymbolKind::Member(..)));
    assert_eq!(p.symbols.at(FileId(0), at(6)), Some(member));
}

/// A statement with an error records nothing; the name's first clean use defines it.
#[test]
fn statement_with_an_error_records_nothing() {
    let (map, p, _, diags) = check_any(b"$CONSOLE:ONLY\nz = \"a\"\nz = 2\n");
    assert_eq!(diags.error_count(), 1);
    insta::assert_snapshot!(dump_symbols(&p, &map), @"Var Z : SINGLE def 3:1");
}

/// Scenario "Same name, different symbols": main's `x` and the SUB's implicit `x`.
#[test]
fn same_name_different_symbols() {
    let src = b"$CONSOLE:ONLY\nx = 1\nshow\nPRINT x\nSUB show\n    x = 2\n    PRINT x\nEND SUB\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map));
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
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map));
}

/// A `SHARED` line that creates the main-module variable is its definition; main's later uses refer to it.
#[test]
fn shared_line_creates_the_main_variable() {
    let src = b"$CONSOLE:ONLY\nSUB s\n    SHARED h AS LONG\n    h = 1\nEND SUB\nh = 2\nPRINT h&\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map));
}

/// A label is defined where it stands, even when `ON ERROR GOTO` names it first; `ON ERROR GOTO` in a SUB and
/// `RESUME` refer to the same main-module label.
#[test]
fn label_with_handler_and_resume_references() {
    let src =
        b"$CONSOLE:ONLY\nON ERROR GOTO h\ns\nback: SYSTEM\nh:\nRESUME back\nSUB s\n    ON ERROR GOTO H\nEND SUB\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map));
    // `h` in `ON ERROR GOTO h` (offset 28) is the label defined at `h:` (offset 45).
    let id = p.symbols.at(FileId(0), 28).unwrap();
    assert_eq!(p.symbols.at(FileId(0), 45), Some(id));
    assert!(matches!(p.symbols.get(id).kind, SymbolKind::Label(_)));
}

/// Labels belong to a body (`m2-control-flow-slice` task 5.1): `a` in main and `a` in a SUB are two symbols, each
/// referenced by the `GOTO` of its own body; the SUB's label is listed with its SUB.
#[test]
fn label_in_a_sub() {
    let src = b"$CONSOLE:ONLY\nGOTO a\na: s\nSYSTEM\nSUB s\n    GOTO a\n    a: PRINT 1\n    GOSUB b\n    EXIT SUB\n    b: RETURN\nEND SUB\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map));
    // `a` of the SUB's `GOTO a` (offset 48) is the SUB's label at offset 54, not main's at offset 21.
    let sub_label = p.symbols.at(FileId(0), 54).unwrap();
    assert_eq!(p.symbols.at(FileId(0), 48), Some(sub_label));
    assert_ne!(p.symbols.at(FileId(0), 21), Some(sub_label));
}

/// Constants are their own kind of symbol (`m2-control-flow-slice` task 4.2): a main constant used in main, in a
/// later SUB and in another `CONST`; a SUB constant with the same name is another symbol; a suffixed use refers to
/// the constant; the same `CONST` again is a reference.
#[test]
fn constants_are_their_own_kind() {
    let src = b"$CONSOLE:ONLY\nCONST k = 1, s = \"a\" + \"b\"\nCONST k2# = k * 2\nPRINT k; k2#; s; k%\nCONST k = 1\nx = k\ns1\nSUB s1\n    CONST k = 2.5\n    PRINT k\nEND SUB\n";
    let (map, p) = check_source(src);
    insta::assert_snapshot!(dump_symbols(&p, &map));
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

/// Which symbols a statement declares (design D2 of `include-file-program`): every kind that a statement makes,
/// and a variable made by `DIM`, `REDIM`, `COMMON`, `SHARED`, `STATIC`, a parameter or a FUNCTION's header; not a
/// scalar or an array made by its first use. `sh`, made by its first use in main, is declared by `SHARED`.
#[test]
fn declared_symbols() {
    let src = b"$CONSOLE:ONLY\nCOMMON SHARED cm AS LONG\nTYPE pt\nxp AS LONG\nEND TYPE\nCONST k = 1\nDIM d AS pt\nREDIM r(3) AS LONG\nim = 1\nia(2) = 1\nsh = 2\ns 1\nPRINT f%(2)\nlbl: PRINT d.xp, r(1), cm, im, ia(2), k\nSUB s (p AS LONG)\nSHARED sh\nSTATIC st AS LONG\nlv = p + st + sh\nEND SUB\nFUNCTION f% (q AS INTEGER)\nf% = q\nEND FUNCTION\n";
    let (_, p) = check_source(src);
    let mut got: Vec<String> = p
        .symbols
        .iter()
        .map(|(id, s)| {
            let name = match s.kind {
                SymbolKind::Var(v) => format!("Var {}", p.var(v).name),
                SymbolKind::Proc(q) => format!("Proc {}", p.proc(q).name),
                SymbolKind::Label(l) => format!("Label {}", p.label(l).name),
                SymbolKind::Const(c) => format!("Const {}", p.constant(c).name),
                SymbolKind::Type(t) => format!("Type {}", p.user_type(t).name),
                SymbolKind::Member(t, m) => format!("Member {}", p.user_type(t).members[m.0 as usize].name),
            };
            let how = if p.symbols.is_declared(id) {
                "declared"
            } else {
                "implicit"
            };
            format!("{name} {how}")
        })
        .collect();
    got.sort();
    let want = [
        "Const K declared",
        "Label LBL declared",
        "Member XP declared",
        "Proc F declared",
        "Proc S declared",
        "Type PT declared",
        "Var CM declared",
        "Var D declared",
        "Var F declared",
        "Var IA implicit",
        "Var IM implicit",
        "Var LV implicit",
        "Var P declared",
        "Var Q declared",
        "Var R declared",
        "Var SH declared",
        "Var ST declared",
    ];
    assert_eq!(got, want);
}

/// An OpenGL routine is made by its first call, not declared (design D2 of `include-file-program`).
#[test]
fn opengl_routine_is_not_declared() {
    let src = b"SUB _GL\n_glClear _GL_COLOR_BUFFER_BIT\nEND SUB\n";
    let (_, p) = check_source(src);
    let gl: Vec<bool> = p
        .symbols
        .iter()
        .filter(|(_, s)| matches!(s.kind, SymbolKind::Proc(q) if p.proc(q).name != "_GL"))
        .map(|(id, _)| p.symbols.is_declared(id))
        .collect();
    assert_eq!(gl, [false]);
}
