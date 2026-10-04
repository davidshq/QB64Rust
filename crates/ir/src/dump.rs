//! `--dump ir`: the lowering-pair text of design D6.

use crate::{BinOp, Const, Op, PrintItem, Program, Ty, Value, ValueKind};
use qb64rust_base::show_bytes;
use std::fmt::Write as _;

pub fn dump(p: &Program) -> String {
    let mut out = String::new();
    for s in &p.main {
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
                Op::Assign { place, value } => {
                    let v = p.var(*place);
                    writeln!(out, "  Assign {}:{:?} = {}", v.name, v.ty, val(p, value)).unwrap();
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
            }
        }
    }
    out
}

fn ty(t: Ty) -> String {
    format!("{t:?}")
}

fn val(p: &Program, v: &Value) -> String {
    match &v.kind {
        ValueKind::Const(Const::Int(i)) => format!("Const {i}:{}", ty(v.ty)),
        ValueKind::Const(Const::Float(t)) => format!("Const {t}:{}", ty(v.ty)),
        ValueKind::Const(Const::Str(s)) => format!("Const \"{}\"", show_bytes(s)),
        ValueKind::Var(id) => format!("Var {}:{}", p.var(*id).name, ty(v.ty)),
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
    }
}
