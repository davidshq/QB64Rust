//! `--dump ir`: the lowering-pair text of design D6.

use crate::{
    Arg, BinOp, Body, Expr, ExprKind, LabelId, OnError, Op, Place, PrintItem, ProcKind, Program, Resume, Storage, Ty,
    VarId, When,
};
use qb64rust_base::{show_bytes, to_u32};
use std::fmt::Write as _;

/// The main module's statements, then each procedure: its header, its variables by storage class, its statements.
pub fn dump(p: &Program) -> String {
    let mut out = String::new();
    body(p, &p.main, &mut out);
    for (i, proc) in p.procs.iter().enumerate() {
        match proc.kind {
            ProcKind::Sub => writeln!(out, "Sub {} lines {}-{}", proc.name, proc.line, proc.end_line).unwrap(),
            ProcKind::Function(t) => writeln!(
                out,
                "Function {}:{} lines {}-{}",
                proc.name,
                ty(p, t),
                proc.line,
                proc.end_line
            )
            .unwrap(),
        }
        for v in &p.vars {
            let class = match v.storage {
                Storage::Param(q) if q.0 as usize == i => "param",
                Storage::Result(q) if q.0 as usize == i => "result",
                Storage::Static(q) if q.0 as usize == i => "static",
                Storage::Local(q) if q.0 as usize == i => "local",
                Storage::Temp(Some(q)) if q.0 as usize == i => "temp",
                // Variables of other procedures and global ones.
                Storage::Global
                | Storage::Static(_)
                | Storage::Local(_)
                | Storage::Param(_)
                | Storage::Result(_)
                | Storage::Temp(_) => {
                    continue;
                }
            };
            writeln!(out, "  {class} {}:{}", v.name, ty(p, v.ty)).unwrap();
        }
        body(p, &proc.body, &mut out);
    }
    out
}

fn body(p: &Program, b: &Body, out: &mut String) {
    let labels = |at: usize, out: &mut String| {
        for (i, l) in b.labels.iter().enumerate().filter(|(_, l)| l.at == at) {
            writeln!(out, "Label {} line {}", label(b, LabelId(to_u32(i))), l.line).unwrap();
        }
    };
    for (i, s) in b.stmts.iter().enumerate() {
        labels(i, out);
        writeln!(
            out,
            "Stmt line {}{}",
            s.line,
            if s.may_raise { " may_raise" } else { "" }
        )
        .unwrap();
        for op in &s.ops {
            match op {
                Op::SelectConsole => writeln!(out, "  SelectConsole").unwrap(),
                Op::End => writeln!(out, "  End").unwrap(),
                Op::System => writeln!(out, "  System").unwrap(),
                Op::Exit => writeln!(out, "  Exit").unwrap(),
                Op::SetHandler(Some(l)) => writeln!(out, "  SetHandler {}", label(&p.main, *l)).unwrap(),
                Op::SetHandler(None) => writeln!(out, "  SetHandler none").unwrap(),
                Op::Raise(v) => writeln!(out, "  Raise {}", val(p, v)).unwrap(),
                Op::Resume(Resume::Retry) => writeln!(out, "  Resume Retry").unwrap(),
                Op::Resume(Resume::Next) => writeln!(out, "  Resume Next").unwrap(),
                Op::Resume(Resume::To(l)) => writeln!(out, "  Resume To {}", label(&p.main, *l)).unwrap(),
                Op::Jump(l) => writeln!(out, "  Jump {}", label(b, *l)).unwrap(),
                Op::Gosub(l) => writeln!(out, "  Gosub {}", label(b, *l)).unwrap(),
                Op::Return(None) => writeln!(out, "  Return").unwrap(),
                Op::Return(Some(l)) => writeln!(out, "  Return To {}", label(&p.main, *l)).unwrap(),
                Op::Branch {
                    cond,
                    when,
                    to,
                    on_error,
                } => {
                    let when = match when {
                        When::Zero => "Zero",
                        When::NonZero => "NonZero",
                    };
                    let on_error = match on_error {
                        OnError::Skip => "Skip",
                        OnError::UseValue => "UseValue",
                    };
                    writeln!(out, "  Branch {when} -> {} {on_error}: {}", label(b, *to), val(p, cond)).unwrap();
                }
                Op::AssignAll(stores) => {
                    writeln!(out, "  AssignAll").unwrap();
                    for (place, value) in stores {
                        let t = ty(p, p.var(*place).ty);
                        writeln!(out, "    {}:{t} = {}", var(p, *place), val(p, value)).unwrap();
                    }
                }
                Op::Assign { place, value } => {
                    let t = ty(p, p.place_ty(place));
                    writeln!(out, "  Assign {}:{t} = {}", place_text(p, place), val(p, value)).unwrap();
                }
                Op::Print { items, newline } => {
                    writeln!(out, "  Print console{}", if *newline { " newline" } else { "" }).unwrap();
                    for i in items {
                        match i {
                            PrintItem::Str(v) => writeln!(out, "    Str {}", val(p, v)).unwrap(),
                            PrintItem::Num(v) => writeln!(out, "    Num {}", val(p, v)).unwrap(),
                            PrintItem::Zone => writeln!(out, "    Zone").unwrap(),
                        }
                    }
                }
                Op::Call { proc, args: a } => {
                    writeln!(out, "  Call {} [{}]", p.proc(*proc).name, args(p, a)).unwrap();
                }
            }
        }
    }
    labels(b.stmts.len(), out);
}

/// A label of body `b`: its BASIC name, or `@n` (its index) for a label the lowering made.
fn label(b: &Body, l: LabelId) -> String {
    match &b.labels[l.0 as usize].name {
        Some(name) => name.clone(),
        None => format!("@{}", l.0),
    }
}

/// A type as `Ty` prints it; a user type by its name (`T:PT`).
fn ty(p: &Program, t: Ty) -> String {
    match t {
        Ty::User(id) => format!("T:{}", p.user_type(id).name),
        Ty::Bit { width, signed: true } => format!("Bit{width}"),
        Ty::Bit { width, signed: false } => format!("UBit{width}"),
        Ty::I8
        | Ty::U8
        | Ty::I16
        | Ty::U16
        | Ty::I32
        | Ty::U32
        | Ty::I64
        | Ty::U64
        | Ty::Off
        | Ty::UOff
        | Ty::F32
        | Ty::F64
        | Ty::F80
        | Ty::Str => format!("{t:?}"),
        qb64rust_sema::unproduced_types!() => unreachable!("{}", qb64rust_sema::NEW_TYPE_UNREACHABLE),
    }
}

/// A place: a variable as [`var`] shows it, `X(i, …)` for an element, `P.M` for a member.
fn place_text(p: &Program, place: &Place) -> String {
    match place {
        Place::Var(v) => var(p, *v),
        Place::Element { array, index } => {
            let i: Vec<String> = index.iter().map(|v| val(p, v)).collect();
            format!("{}({})", var(p, *array), i.join(", "))
        }
        Place::Member { base, member } => {
            let m = &p.member(p.place_ty(base), *member).name;
            format!("{}.{m}", place_text(p, base))
        }
    }
}

/// A variable's name, with its storage class unless it is global (`X(param)`).
fn var(p: &Program, id: VarId) -> String {
    let v = p.var(id);
    let class = match v.storage {
        Storage::Global => return v.name.clone(),
        Storage::Static(_) => "static",
        Storage::Local(_) => "local",
        Storage::Param(_) => "param",
        Storage::Result(_) => "result",
        Storage::Temp(_) => "temp",
    };
    format!("{}({class})", v.name)
}

fn args(p: &Program, args: &[Arg]) -> String {
    args.iter()
        .map(|a| match a {
            Arg::Ref(place) => format!("Ref {}:{}", place_text(p, place), ty(p, p.place_ty(place))),
            Arg::Temp(v) => format!("Temp({})", val(p, v)),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn val(p: &Program, v: &Expr) -> String {
    match &v.kind {
        ExprKind::Int(i) => format!("Const {i}:{}", ty(p, v.ty)),
        ExprKind::Float(t) => format!("Const {t}:{}", ty(p, v.ty)),
        ExprKind::Str(s) => format!("Const \"{}\"", show_bytes(s)),
        ExprKind::Load(Place::Var(id)) => format!("Var {}:{}", var(p, *id), ty(p, v.ty)),
        ExprKind::Load(place) => format!("Load {}:{}", place_text(p, place), ty(p, v.ty)),
        ExprKind::Bound { upper, array, dim } => {
            let word = if *upper { "UBound" } else { "LBound" };
            let d = dim.as_deref().map_or("1".to_string(), |d| val(p, d));
            format!("{word} {}({d}):{}", var(p, *array), ty(p, v.ty))
        }
        ExprKind::Convert { how, from } => format!("({} -> Convert {} {how:?})", val(p, from), ty(p, v.ty)),
        ExprKind::Binary { op, lhs, rhs } => {
            // Only `+`, `-` and `*` can overflow an integer type (and `\` of the smallest value by -1, unspecified).
            let wraps = matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul) && v.ty.is_int();
            let wrap = if wraps { " wrap" } else { "" };
            format!("{op:?}:{}({}, {}){wrap}", ty(p, v.ty), val(p, lhs), val(p, rhs))
        }
        ExprKind::Unary { op, operand } => format!("{op:?}:{}({})", ty(p, v.ty), val(p, operand)),
        ExprKind::Concat(a, b) => format!("Concat({}, {})", val(p, a), val(p, b)),
        ExprKind::StrCompare { op, lhs, rhs } => format!("StrCompare {op:?}({}, {})", val(p, lhs), val(p, rhs)),
        ExprKind::Call { builtin: id, args } => {
            let args: Vec<String> = args
                .iter()
                .map(|a| match a {
                    Some(a) => format!("Some({})", val(p, a)),
                    None => "None".to_string(),
                })
                .collect();
            format!(
                "CallBuiltin {}:{} [{}]",
                id.get().name.to_ascii_uppercase(),
                ty(p, v.ty),
                args.join(", ")
            )
        }
        ExprKind::CallProc { proc, args: a } => {
            format!("CallProc {}:{} [{}]", p.proc(*proc).name, ty(p, v.ty), args(p, a))
        }
    }
}
