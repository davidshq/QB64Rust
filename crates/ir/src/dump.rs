//! `--dump ir`: the lowering-pair text of design D6.

use crate::{Arg, BinOp, Body, Const, Op, PrintItem, ProcKind, Program, Storage, Ty, Value, ValueKind, VarId};
use qb64rust_base::show_bytes;
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
                ty(t),
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
                // Variables of other procedures and global ones.
                Storage::Global | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => {
                    continue;
                }
            };
            writeln!(out, "  {class} {}:{}", v.name, ty(v.ty)).unwrap();
        }
        body(p, &proc.body, &mut out);
    }
    out
}

fn body(p: &Program, b: &Body, out: &mut String) {
    for s in &b.stmts {
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
                Op::Assign { place, value } => {
                    let v = p.var(*place);
                    writeln!(out, "  Assign {}:{:?} = {}", var(p, *place), v.ty, val(p, value)).unwrap();
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
}

fn ty(t: Ty) -> String {
    format!("{t:?}")
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
    };
    format!("{}({class})", v.name)
}

fn args(p: &Program, args: &[Arg]) -> String {
    args.iter()
        .map(|a| match a {
            Arg::Ref(v) => format!("Ref {}:{}", var(p, *v), ty(p.var(*v).ty)),
            Arg::Temp(v) => format!("Temp({})", val(p, v)),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn val(p: &Program, v: &Value) -> String {
    match &v.kind {
        ValueKind::Const(Const::Int(i)) => format!("Const {i}:{}", ty(v.ty)),
        ValueKind::Const(Const::Float(t)) => format!("Const {t}:{}", ty(v.ty)),
        ValueKind::Const(Const::Str(s)) => format!("Const \"{}\"", show_bytes(s)),
        ValueKind::Var(id) => format!("Var {}:{}", var(p, *id), ty(v.ty)),
        ValueKind::Convert { how, from } => format!("({} -> Convert {} {how:?})", val(p, from), ty(v.ty)),
        ValueKind::Binary { op, lhs, rhs } => {
            let o = match op {
                BinOp::Add => "Add",
                BinOp::Sub => "Sub",
                BinOp::Mul => "Mul",
                BinOp::Div => "Div",
            };
            let wrap = if v.ty <= Ty::I64 { " wrap" } else { "" };
            format!("{o}:{}({}, {}){wrap}", ty(v.ty), val(p, lhs), val(p, rhs))
        }
        ValueKind::Neg(x) => format!("Neg:{}({})", ty(v.ty), val(p, x)),
        ValueKind::Concat(a, b) => format!("Concat({}, {})", val(p, a), val(p, b)),
        ValueKind::CallBuiltin { id, args } => {
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
                ty(v.ty),
                args.join(", ")
            )
        }
        ValueKind::CallProc { proc, args: a } => {
            format!("CallProc {}:{} [{}]", p.proc(*proc).name, ty(v.ty), args(p, a))
        }
    }
}
