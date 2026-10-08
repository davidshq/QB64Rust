//! The built-in checker (design D2, D3 of `m2-core-builtins`): how each slot type converts an argument (measured,
//! `verification\v20_a_slots`), the arity check, and the kind check.

use qb64rust_base::{Diagnostics, SourceMap};
use qb64rust_sema::{ConvKind, Expr, ExprKind, PrintItem, Program, StmtKind, Ty, check};
use qb64rust_syntax::{NoLoader, parse};

fn check_any(text: &str) -> (Program, Diagnostics) {
    let mut map = SourceMap::new();
    let file = map.add("t.bas", format!("$CONSOLE:ONLY\n{text}\n").into_bytes());
    let parsed = parse(&mut map, file, &mut NoLoader);
    assert!(!parsed.diagnostics().has_errors(), "parse errors in {text}");
    check(&map, &parsed)
}

/// The expression of the first item of the last `PRINT` in `text`, which must check without errors.
fn printed(text: &str) -> Expr {
    let (p, diags) = check_any(text);
    assert!(!diags.has_errors(), "{text}: {:?}", diags.list());
    let Some(StmtKind::Print { items, .. }) = p
        .stmts
        .iter()
        .rev()
        .map(|s| &s.kind)
        .find(|k| matches!(k, StmtKind::Print { .. }))
    else {
        panic!("no PRINT in {text}");
    };
    match &items[0] {
        PrintItem::Str(e) => e.clone(),
        // A number is printed in its believed type: look through that conversion.
        PrintItem::Num(e) => match &e.kind {
            ExprKind::Convert { from, .. } => (**from).clone(),
            _ => e.clone(),
        },
        PrintItem::Zone => panic!("a zone first"),
    }
}

/// The argument slots of a call.
fn slots(e: &Expr) -> &[Option<Expr>] {
    let ExprKind::Call { args, .. } = &e.kind else {
        panic!("not a built-in call: {e:?}");
    };
    args
}

/// The messages of the check's errors.
fn errors(text: &str) -> Vec<String> {
    check_any(text).1.list().iter().map(|d| d.message.clone()).collect()
}

/// A LONG slot converts as a store into a LONG: a float rounded half to even to `_INTEGER64`, then truncated.
#[test]
fn long_slot_rounds_then_truncates() {
    let call = printed("PRINT CHR$(65.5)");
    let arg = slots(&call)[0].as_ref().unwrap();
    assert_eq!(arg.ty, Ty::I32);
    let ExprKind::Convert {
        how: ConvKind::Truncate,
        from,
    } = &arg.kind
    else {
        panic!("{arg:?}");
    };
    assert!(
        matches!(
            from.kind,
            ExprKind::Convert {
                how: ConvKind::RoundEven,
                ..
            }
        ) && from.ty == Ty::I64
    );
    // An integer argument is converted exactly; a constant is folded.
    let call = printed("PRINT CHR$(65&&)");
    assert_eq!(slots(&call)[0].as_ref().unwrap().kind, ExprKind::Int(65));
}

/// A STRING slot takes the string as it is; an absent optional slot is `None`.
#[test]
fn string_slots_and_optional_slots() {
    let call = printed("PRINT INSTR(\"abc\", \"b\")");
    assert_eq!(call.ty, Ty::I32);
    let s = slots(&call);
    assert_eq!(s.len(), 3);
    assert!(s[0].is_none());
    assert!(matches!(s[1].as_ref().unwrap().kind, ExprKind::Str(_)));
    let call = printed("PRINT INSTR(2.5, \"abc\", \"b\")");
    assert_eq!(slots(&call)[0].as_ref().unwrap().ty, Ty::I32);
}

/// A DOUBLE slot converts exactly (`_PI`); a `_FLOAT` slot takes the argument in its own type (`SQR`, `_HYPOT`);
/// an any-numeric slot casts it to its believed type (`STR$`, `ABS`); `INT` takes it as it is.
#[test]
fn double_float_and_any_numeric_slots() {
    let arg = |text: &str| slots(&printed(text))[0].clone().unwrap();
    let a = arg("PRINT _PI(2%)");
    assert!(
        matches!(
            a.kind,
            ExprKind::Convert {
                how: ConvKind::Nearest,
                ..
            }
        ) && a.ty == Ty::F64
    );
    let a = arg("DIM i AS INTEGER\nPRINT SQR(i)");
    assert!(matches!(a.kind, ExprKind::Load(_)) && a.ty == Ty::I16);
    let a = arg("PRINT STR$(2.5)");
    assert!(matches!(
        a.kind,
        ExprKind::Convert {
            how: ConvKind::Nearest,
            ..
        }
    ));
    assert_eq!(
        (a.ty, a.qb),
        (Ty::F32, Ty::F32),
        "a SINGLE literal is held as a DOUBLE, cast to SINGLE"
    );
    let a = arg("DIM i AS INTEGER\nPRINT ABS(i + 1)");
    assert_eq!((a.ty, a.qb), (Ty::I64, Ty::I64), "`i + 1` is believed _INTEGER64");
    let a = arg("PRINT INT(2.5)");
    assert_eq!((a.ty, a.qb), (Ty::F64, Ty::F32), "as it is");
}

/// Held and believed result types (design D2, measured from the C++).
#[test]
fn result_types() {
    let types = |text: &str| {
        let e = printed(text);
        (e.ty, e.qb)
    };
    let decl = "DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE\n";
    assert_eq!(
        types(&format!("{decl}PRINT SQR(i)")),
        (Ty::F64, Ty::F32),
        "func_sqr returns a double"
    );
    assert_eq!(
        types(&format!("{decl}PRINT SIN(f)")),
        (Ty::F32, Ty::F32),
        "std::sin(float)"
    );
    assert_eq!(types(&format!("{decl}PRINT SIN(q)")), (Ty::F64, Ty::F80));
    assert_eq!(types(&format!("{decl}PRINT EXP(l)")), (Ty::F80, Ty::F80));
    assert_eq!(types(&format!("{decl}PRINT EXP(i)")), (Ty::F64, Ty::F32));
    assert_eq!(types(&format!("{decl}PRINT ABS(i)")), (Ty::I16, Ty::I16));
    assert_eq!(
        types(&format!("{decl}PRINT CLNG(d)")),
        (Ty::I32, Ty::I32),
        "CLNG is LONG, not the table's INTEGER"
    );
    assert_eq!(
        types(&format!("{decl}PRINT CSNG(l)")),
        (Ty::F64, Ty::F32),
        "not narrowed"
    );
    assert_eq!(
        types(&format!("{decl}PRINT CINT(d)")),
        (Ty::I32, Ty::I16),
        "func_cint_double returns int32"
    );
    assert_eq!(types(&format!("{decl}PRINT VAL(\"1\", INTEGER)")), (Ty::I64, Ty::I64));
    assert_eq!(types(&format!("{decl}PRINT VAL(\"1\", SINGLE)")), (Ty::F32, Ty::F32));
    assert_eq!(types(&format!("{decl}PRINT _HYPOT(i, l)")), (Ty::F64, Ty::F80));
    assert_eq!(types(&format!("{decl}PRINT SGN(d)")), (Ty::I32, Ty::I32));
}

#[test]
fn arity() {
    assert_eq!(errors("PRINT CHR$(1, 2)"), ["`CHR$` takes 1 argument"]);
    assert_eq!(errors("PRINT INSTR(\"a\")"), ["`INSTR` takes 2 or 3 arguments"]);
    assert_eq!(
        errors("PRINT INSTR(1, \"a\", \"b\", \"c\")"),
        ["`INSTR` takes 2 or 3 arguments"]
    );
    assert_eq!(errors("PRINT CHR$()"), ["`CHR$` with empty parentheses"]);
}

#[test]
fn argument_kinds() {
    assert_eq!(errors("PRINT CHR$(\"a\")"), ["`CHR$` needs a number"]);
    assert_eq!(
        errors("PRINT INSTR(\"a\", 1)"),
        ["argument 3 of `INSTR` needs a string"]
    );
    assert_eq!(
        errors("PRINT INSTR(\"a\", \"b\", \"c\")"),
        ["argument 1 of `INSTR` needs a number"]
    );
}

/// A built-in needing no argument is called by its bare name; one written with another suffix is no built-in.
#[test]
fn bare_names_and_suffixes() {
    let call = printed("PRINT ERR");
    assert_eq!((call.ty, call.qb), (Ty::I32, Ty::I32));
    assert!(slots(&call).is_empty());
    let call = printed("PRINT ERL");
    assert_eq!(call.ty, Ty::F64);
    // `CHR` without `$` is a free name: an implicit array here, not supported yet.
    let (_, diags) = check_any("PRINT CHR(65)");
    assert!(!diags.has_real_errors());
}
