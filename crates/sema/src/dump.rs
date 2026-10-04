//! `--dump typed`: one node per line, indented, with its type (FreeBASIC lesson L8: assertions on types).

use crate::{Expr, ExprKind, PrintItem, Program, StmtKind, Ty};
use qb64rust_base::show_bytes;
use std::fmt::Write as _;

pub fn dump_typed(p: &Program) -> String {
    let mut out = String::new();
    for s in &p.stmts {
        match &s.kind {
            StmtKind::ConsoleOnly => writeln!(out, "line {}: ConsoleOnly", s.line).unwrap(),
            StmtKind::End => writeln!(out, "line {}: End", s.line).unwrap(),
            StmtKind::Assign { var, value } => {
                let v = p.var(*var);
                writeln!(out, "line {}: Assign {}:{}", s.line, v.name, ty(v.ty)).unwrap();
                expr(p, value, 1, &mut out);
            }
            StmtKind::Print { items, newline } => {
                writeln!(out, "line {}: Print{}", s.line, if *newline { " newline" } else { "" }).unwrap();
                for i in items {
                    match i {
                        PrintItem::Str(e) => {
                            writeln!(out, "  Str").unwrap();
                            expr(p, e, 2, &mut out);
                        }
                        PrintItem::Num(e) => {
                            writeln!(out, "  Num").unwrap();
                            expr(p, e, 2, &mut out);
                        }
                        PrintItem::Zone => writeln!(out, "  Zone").unwrap(),
                    }
                }
            }
        }
    }
    out
}

fn ty(t: Ty) -> &'static str {
    match t {
        Ty::I16 => "I16",
        Ty::I32 => "I32",
        Ty::I64 => "I64",
        Ty::F32 => "F32",
        Ty::F64 => "F64",
        Ty::F80 => "F80",
        Ty::Str => "Str",
    }
}

fn expr(p: &Program, e: &Expr, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let types = if e.qb == e.ty {
        ty(e.ty).to_string()
    } else {
        format!("{} (qb {})", ty(e.ty), ty(e.qb))
    };
    match &e.kind {
        ExprKind::Int(v) => writeln!(out, "{pad}Int {v} : {types}").unwrap(),
        ExprKind::Float(t) => writeln!(out, "{pad}Float {t} : {types}").unwrap(),
        ExprKind::Str(s) => writeln!(out, "{pad}Str \"{}\" : {types}", show_bytes(s)).unwrap(),
        ExprKind::Var(id) => writeln!(out, "{pad}Var {} : {types}", p.var(*id).name).unwrap(),
        ExprKind::Convert { how, from } => {
            writeln!(out, "{pad}Convert {how:?} : {types}").unwrap();
            expr(p, from, depth + 1, out);
        }
        ExprKind::Binary { op, lhs, rhs } => {
            writeln!(out, "{pad}Binary {op:?} : {types}").unwrap();
            expr(p, lhs, depth + 1, out);
            expr(p, rhs, depth + 1, out);
        }
        ExprKind::Neg(x) => {
            writeln!(out, "{pad}Neg : {types}").unwrap();
            expr(p, x, depth + 1, out);
        }
        ExprKind::Concat(a, b) => {
            writeln!(out, "{pad}Concat : {types}").unwrap();
            expr(p, a, depth + 1, out);
            expr(p, b, depth + 1, out);
        }
        ExprKind::Call { builtin, args } => {
            writeln!(out, "{pad}Call {} : {types}", builtin.get().name.to_ascii_uppercase()).unwrap();
            for a in args {
                match a {
                    Some(a) => expr(p, a, depth + 1, out),
                    None => writeln!(out, "{pad}  (absent)").unwrap(),
                }
            }
        }
    }
}
