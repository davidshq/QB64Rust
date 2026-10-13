//! `--dump typed`: one node per line, indented, with its type (FreeBASIC lesson L8: assertions on types).

use crate::{
    Arg, ArrayRef, CaseItem, Expr, ExprKind, InputSource, Place, PrintItem, PrintTo, ProcKind, Program, Resume, Shape,
    Stmt, StmtArg, StmtKind, Storage, Ty, UsingValue, VarId,
};

use qb64rust_base::show_bytes;
use std::fmt::Write as _;

/// The main module's statements and labels, then each procedure: its header, its variables by storage class, its
/// statements. Variables other than main-module ones are shown with their storage (`A (param)`).
pub fn dump_typed(p: &Program) -> String {
    let mut out = String::new();
    // The user types and the arrays first, when there are any.
    for t in &p.types {
        writeln!(out, "TYPE {}", t.name).unwrap();
        for m in &t.members {
            // A member array with its bounds.
            let bounds = if m.is_array() {
                let dims: Vec<String> = m.bounds.iter().map(|(l, u)| format!("{l} TO {u}")).collect();
                format!("({})", dims.join(", "))
            } else {
                String::new()
            };
            writeln!(out, "  {}:{}{bounds}", m.name, ty(p, m.ty)).unwrap();
        }
    }
    for (i, v) in p.vars.iter().enumerate() {
        let name = var_name(p, VarId(qb64rust_base::to_u32(i)));
        match &v.shape {
            Shape::Scalar => {}
            Shape::Static { bounds } => {
                let dims: Vec<String> = bounds.iter().map(|(l, u)| format!("{l} TO {u}")).collect();
                writeln!(out, "ARRAY {name}:{}({})", ty(p, v.ty), dims.join(", ")).unwrap();
            }
            Shape::Dynamic { dims, at_start } => {
                let dims = match dims {
                    Some(1) => "1 dimension".to_string(),
                    Some(n) => format!("{n} dimensions"),
                    None => "dimensions not known".to_string(),
                };
                let start = at_start.as_ref().map_or(String::new(), |bounds| {
                    let dims: Vec<String> = bounds.iter().map(|(l, u)| format!("{l} TO {u}")).collect();
                    format!(", allocated at the start ({})", dims.join(", "))
                });
                // An array parameter stands for the array passed, static or dynamic.
                let kind = if matches!(v.storage, Storage::Param(_)) {
                    "passed"
                } else {
                    "dynamic"
                };
                writeln!(out, "ARRAY {name}:{} {kind}, {dims}{start}", ty(p, v.ty)).unwrap();
            }
        }
    }
    // The program's data, when there is any: one item per line, a quoted one in quotes.
    for (i, item) in p.data.iter().enumerate() {
        let q = if item.quoted { "\"" } else { "" };
        writeln!(out, "DATA {i}: {q}{}{q}", show_bytes(&item.text)).unwrap();
    }
    stmts(p, &p.stmts, 0, &mut out);
    for (i, proc) in p.procs.iter().enumerate() {
        match proc.kind {
            ProcKind::Sub => writeln!(out, "SUB {}", proc.name).unwrap(),
            ProcKind::Function(t) => writeln!(out, "FUNCTION {} : {}", proc.name, ty(p, t)).unwrap(),
        }
        for v in &p.vars {
            let class = match v.storage {
                Storage::Param(q) if q.0 as usize == i => "param",
                Storage::Result(q) if q.0 as usize == i => "result",
                Storage::Static(q) if q.0 as usize == i => "static",
                Storage::Local(q) if q.0 as usize == i => "local",
                // Variables of other procedures and of the main module.
                Storage::Main | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => {
                    continue;
                }
            };
            let parens = if class == "param" && v.is_array() { "()" } else { "" };
            writeln!(out, "  {class} {}{parens}:{}", v.name, ty(p, v.ty)).unwrap();
        }
        stmts(p, &proc.stmts, 0, &mut out);
    }
    out
}

/// A variable's name, with its storage class unless it is a main-module variable.
fn var_name(p: &Program, id: VarId) -> String {
    let v = p.var(id);
    let class = match v.storage {
        Storage::Main => return v.name.clone(),
        Storage::Static(_) => "static",
        Storage::Local(_) => "local",
        Storage::Param(_) => "param",
        Storage::Result(_) => "result",
    };
    format!("{} ({class})", v.name)
}

/// A place as text: `X`, `X(…)` for an element (its indexes are printed by [`indexes`]), `P.M` for a member.
fn place_text(p: &Program, place: &Place) -> String {
    match place {
        Place::Var(v) => var_name(p, *v),
        Place::Element { array, .. } => format!("{}(…)", array_text(p, array)),
        Place::Member { base, member } => {
            let m = &p.member(p.place_ty(base), *member).name;
            format!("{}.{m}", place_text(p, base))
        }
    }
}

/// An array as text: `X`, or `P.M` for a member array.
fn array_text(p: &Program, array: &ArrayRef) -> String {
    match array {
        ArrayRef::Var(v) => var_name(p, *v),
        ArrayRef::Member { base, member } => {
            let m = &p.member(p.place_ty(base), *member).name;
            format!("{}.{m}", place_text(p, base))
        }
    }
}

/// The indexes of the elements on the way to a place, outermost first, each at `depth`.
fn indexes(p: &Program, place: &Place, depth: usize, out: &mut String) {
    for e in place.indexes() {
        expr(p, e, depth, out);
    }
}

fn args(p: &Program, args: &[Arg], depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    for a in args {
        match a {
            Arg::Ref(place) => {
                let t = ty(p, p.place_ty(place));
                writeln!(out, "{pad}ref {} : {t}", place_text(p, place)).unwrap();
                indexes(p, place, depth + 1, out);
            }
            Arg::Temp(e) => {
                writeln!(out, "{pad}temp").unwrap();
                expr(p, e, depth + 1, out);
            }
            Arg::Array(a) => {
                let t = ty(p, p.array_ty(a));
                writeln!(out, "{pad}array {}() : {t}", array_text(p, a)).unwrap();
                for e in a.indexes() {
                    expr(p, e, depth + 1, out);
                }
            }
        }
    }
}

/// Where a `PRINT` goes, as the words after the statement's name.
fn print_to(to: &PrintTo) -> &'static str {
    match to {
        PrintTo::Screen => "",
        PrintTo::File(_) => " to a file",
        PrintTo::Printer => " to the printer",
    }
}

fn ending(newline: bool) -> &'static str {
    if newline { " newline" } else { "" }
}

/// The items of a `PRINT` or `WRITE` at nesting depth `d`, each under its heading.
fn print_items(p: &Program, items: &[PrintItem], d: usize, out: &mut String) {
    let pad = "  ".repeat(d);
    for i in items {
        match i {
            PrintItem::Str(e) => {
                writeln!(out, "{pad}  Str").unwrap();
                expr(p, e, d + 2, out);
            }
            PrintItem::Num(e) => {
                writeln!(out, "{pad}  Num").unwrap();
                expr(p, e, d + 2, out);
            }
            PrintItem::Zone => writeln!(out, "{pad}  Zone").unwrap(),
        }
    }
}

/// The statements at nesting depth `d`: each is `line N: …` indented by `d` steps, its parts one step deeper, and
/// the statements of a block's bodies at `d + 1` under a heading (`Then`, `ElseIf`, `Else`, `Body`).
fn stmts(p: &Program, list: &[Stmt], d: usize, out: &mut String) {
    let pad = "  ".repeat(d);
    for s in list {
        let line = |out: &mut String, text: &str| writeln!(out, "{pad}line {}: {text}", s.line).unwrap();
        let heading = |out: &mut String, text: &str| writeln!(out, "{pad}  {text}").unwrap();
        match &s.kind {
            StmtKind::Label(l) => line(out, &format!("Label {}", p.label(*l).name)),
            StmtKind::Goto(l) => line(out, &format!("Goto {}", p.label(*l).name)),
            StmtKind::Gosub(l) => line(out, &format!("Gosub {}", p.label(*l).name)),
            StmtKind::Return(None) => line(out, "Return"),
            StmtKind::Return(Some(l)) => line(out, &format!("Return {}", p.label(*l).name)),
            StmtKind::ConsoleOnly => line(out, "ConsoleOnly"),
            StmtKind::End => line(out, "End"),
            StmtKind::System => line(out, "System"),
            StmtKind::Exit => line(out, "Exit"),
            StmtKind::ExitLoop(k) => line(out, &format!("Exit {k:?}")),
            StmtKind::ExitSelect { case } => line(out, if *case { "Exit Case" } else { "Exit Select" }),
            StmtKind::OnError(Some(l)) => line(out, &format!("OnError {}", p.label(*l).name)),
            StmtKind::OnError(None) => line(out, "OnError 0"),
            StmtKind::Resume(r) => {
                let to = match r {
                    Resume::Retry => "Retry".to_string(),
                    Resume::Next => "Next".to_string(),
                    Resume::To(l) => format!("To {}", p.label(*l).name),
                };
                line(out, &format!("Resume {to}"));
            }
            StmtKind::Error(code) => {
                line(out, "Error");
                expr(p, code, d + 1, out);
            }
            StmtKind::Call { proc, args: a } => {
                line(out, &format!("Call {}", p.proc(*proc).name));
                args(p, a, d + 1, out);
            }
            StmtKind::Builtin { id, args: a } => {
                line(out, &format!("Builtin {}", id.get().name.to_ascii_uppercase()));
                for arg in a {
                    match arg {
                        StmtArg::Value(e) => expr(p, e, d + 1, out),
                        StmtArg::Place(place) => {
                            let t = ty(p, p.place_ty(place));
                            heading(out, &format!("place {} : {t}", place_text(p, place)));
                            indexes(p, place, d + 2, out);
                        }
                        StmtArg::Block { array, index } => {
                            let from = if index.is_empty() { "whole" } else { "from an element" };
                            heading(out, &format!("bytes of {} ({from})", var_name(p, *array)));
                            for i in index {
                                expr(p, i, d + 2, out);
                            }
                        }
                        StmtArg::Bytes(place) => {
                            let t = ty(p, p.place_ty(place));
                            heading(out, &format!("bytes of place {} : {t}", place_text(p, place)));
                            indexes(p, place, d + 2, out);
                        }
                        StmtArg::Word(w) => heading(out, &format!("word {w}")),
                        StmtArg::Absent => heading(out, "(absent)"),
                    }
                }
            }
            StmtKind::CopyWhole { from, to } => {
                let t = ty(p, p.place_ty(to));
                let text = format!("CopyWhole {}:{t} <- {}", place_text(p, to), place_text(p, from));
                line(out, &text);
                for (side, place) in [("To", to), ("From", from)] {
                    if place.has_element() {
                        heading(out, &format!("{side} index"));
                        indexes(p, place, d + 2, out);
                    }
                }
            }
            StmtKind::Assign { place, value } => {
                let t = ty(p, p.place_ty(place));
                line(out, &format!("Assign {}:{t}", place_text(p, place)));
                if place.has_element() {
                    heading(out, "Index");
                    indexes(p, place, d + 2, out);
                    heading(out, "Value");
                    expr(p, value, d + 2, out);
                } else {
                    expr(p, value, d + 1, out);
                }
            }
            StmtKind::Allocate {
                array,
                bounds,
                mode,
                joined,
            } => {
                let t = ty(p, p.var(*array).ty);
                let joined = if *joined { ", with the next statement" } else { "" };
                line(out, &format!("Allocate {}:{t} ({mode:?}{joined})", var_name(p, *array)));
                for (lower, upper) in bounds {
                    heading(out, "Lower");
                    expr(p, lower, d + 2, out);
                    heading(out, "Upper");
                    expr(p, upper, d + 2, out);
                }
            }
            StmtKind::Clear { array } => {
                let t = ty(p, p.array_ty(array));
                line(out, &format!("Clear {}:{t}", array_text(p, array)));
                for e in array.indexes() {
                    expr(p, e, d + 1, out);
                }
            }
            StmtKind::Release { array } => {
                let t = ty(p, p.var(*array).ty);
                line(out, &format!("Release {}:{t}", var_name(p, *array)));
            }
            StmtKind::EraseParam { array } => {
                let t = ty(p, p.var(*array).ty);
                line(out, &format!("EraseParam {}:{t}", var_name(p, *array)));
            }
            StmtKind::Print { to, items, newline } => {
                line(out, &format!("Print{}{}", print_to(to), ending(*newline)));
                if let PrintTo::File(n) = to {
                    heading(out, "File");
                    expr(p, n, d + 2, out);
                }
                print_items(p, items, d, out);
            }
            StmtKind::PrintUsing {
                to,
                before,
                format,
                values,
                newline,
            } => {
                line(out, &format!("PrintUsing{}{}", print_to(to), ending(*newline)));
                if let PrintTo::File(n) = to {
                    heading(out, "File");
                    expr(p, n, d + 2, out);
                }
                print_items(p, before, d, out);
                heading(out, "Format");
                expr(p, format, d + 2, out);
                for v in values {
                    match v {
                        UsingValue::Str(e) => {
                            heading(out, "Str");
                            expr(p, e, d + 2, out);
                        }
                        UsingValue::Num { value, as_ } => {
                            heading(out, &format!("Num as {}", ty(p, *as_)));
                            expr(p, value, d + 2, out);
                        }
                        UsingValue::Blanks(e) => {
                            heading(out, "Blanks");
                            expr(p, e, d + 2, out);
                        }
                    }
                }
            }
            StmtKind::Field { file, parts } => {
                line(out, "Field");
                heading(out, "File");
                expr(p, file, d + 2, out);
                for (place, width) in parts {
                    let t = ty(p, p.place_ty(place));
                    heading(out, &format!("place {} : {t}", place_text(p, place)));
                    indexes(p, place, d + 2, out);
                    heading(out, "Width");
                    expr(p, width, d + 2, out);
                }
            }
            StmtKind::Lset { right, target, value } => {
                let t = ty(p, p.place_ty(target));
                let word = if *right { "Rset" } else { "Lset" };
                line(out, &format!("{word} {} : {t}", place_text(p, target)));
                indexes(p, target, d + 1, out);
                expr(p, value, d + 1, out);
            }
            StmtKind::Write { to, items, newline } => {
                let file = if to.is_some() { " to a file" } else { "" };
                line(out, &format!("Write{file}{}", ending(*newline)));
                if let Some(n) = to {
                    heading(out, "File");
                    expr(p, n, d + 2, out);
                }
                print_items(p, items, d, out);
            }
            StmtKind::Input {
                from,
                line: whole,
                targets,
            } => {
                let word = if *whole { "LineInput" } else { "Input" };
                match from {
                    InputSource::File(n) => {
                        line(out, &format!("{word} from a file"));
                        heading(out, "File");
                        expr(p, n, d + 2, out);
                    }
                    InputSource::Console { prompt, question, stay } => {
                        let prompt = match prompt {
                            Some(text) => format!(" prompt \"{}\"", show_bytes(text)),
                            None => String::new(),
                        };
                        let question = if *question { " question" } else { "" };
                        let stay = if *stay { " stay" } else { "" };
                        line(out, &format!("{word} from the console{prompt}{question}{stay}"));
                    }
                }
                for place in targets {
                    let t = ty(p, p.place_ty(place));
                    heading(out, &format!("target {} : {t}", place_text(p, place)));
                    indexes(p, place, d + 2, out);
                }
            }
            StmtKind::Read(targets) => {
                line(out, "Read");
                for place in targets {
                    let t = ty(p, p.place_ty(place));
                    heading(out, &format!("target {} : {t}", place_text(p, place)));
                    indexes(p, place, d + 2, out);
                }
            }
            StmtKind::Restore(None) => line(out, "Restore"),
            StmtKind::Restore(Some(l)) => {
                let l = p.label(*l);
                line(out, &format!("Restore {} (item {})", l.name, l.data_at));
            }
            StmtKind::If {
                branches,
                else_,
                end_line,
            } => {
                line(out, &format!("If (END IF line {end_line})"));
                for (i, b) in branches.iter().enumerate() {
                    heading(
                        out,
                        &format!("{} line {}", if i == 0 { "If" } else { "ElseIf" }, b.line),
                    );
                    expr(p, &b.cond, d + 2, out);
                    stmts(p, &b.body, d + 2, out);
                }
                if let Some(body) = else_ {
                    heading(out, "Else");
                    stmts(p, body, d + 2, out);
                }
            }
            StmtKind::For {
                var,
                temp,
                start,
                end,
                step,
                body,
                end_line,
            } => {
                let v = p.var(*var);
                line(
                    out,
                    &format!(
                        "For {}:{} counting in {} (NEXT line {end_line})",
                        var_name(p, *var),
                        ty(p, v.ty),
                        ty(p, *temp)
                    ),
                );
                heading(out, "From");
                expr(p, start, d + 2, out);
                heading(out, "To");
                expr(p, end, d + 2, out);
                if let Some(step) = step {
                    heading(out, "Step");
                    expr(p, step, d + 2, out);
                }
                heading(out, "Body");
                stmts(p, body, d + 2, out);
            }
            StmtKind::Do { test, body, end_line } => {
                line(out, &format!("Do (LOOP line {end_line})"));
                if let Some(t) = test {
                    let word = if t.until { "Until" } else { "While" };
                    heading(out, &format!("{word} at {:?}", t.at));
                    expr(p, &t.cond, d + 2, out);
                }
                heading(out, "Body");
                stmts(p, body, d + 2, out);
            }
            StmtKind::While { cond, body, end_line } => {
                line(out, &format!("While (WEND line {end_line})"));
                heading(out, "Cond");
                expr(p, cond, d + 2, out);
                heading(out, "Body");
                stmts(p, body, d + 2, out);
            }
            StmtKind::OnJump { value, gosub, targets } => {
                let names: Vec<&str> = targets.iter().map(|l| p.label(*l).name.as_str()).collect();
                let kw = if *gosub { "Gosub" } else { "Goto" };
                line(out, &format!("On{kw} {}", names.join(", ")));
                expr(p, value, d + 1, out);
            }
            StmtKind::Select {
                selector,
                copied,
                every,
                cases,
                else_,
                end_line,
            } => {
                let kind = if *every { "SelectEveryCase" } else { "SelectCase" };
                let read = if *copied { "copied once" } else { "read at each test" };
                line(out, &format!("{kind}, selector {read} (END SELECT line {end_line})"));
                expr(p, selector, d + 2, out);
                for c in cases {
                    heading(out, &format!("Case line {}", c.line));
                    for item in &c.items {
                        match item {
                            CaseItem::Is(op, v) => {
                                writeln!(out, "{pad}    Is {op:?}").unwrap();
                                expr(p, v, d + 3, out);
                            }
                            CaseItem::Range(low, high) => {
                                writeln!(out, "{pad}    Range").unwrap();
                                expr(p, low, d + 3, out);
                                expr(p, high, d + 3, out);
                            }
                        }
                    }
                    writeln!(out, "{pad}    Body").unwrap();
                    stmts(p, &c.body, d + 3, out);
                }
                if let Some(body) = else_ {
                    heading(out, "Else");
                    stmts(p, body, d + 2, out);
                }
            }
        }
    }
}

/// A type's short name; a user type by its name (`T:PT`).
fn ty(p: &Program, t: Ty) -> String {
    match t {
        Ty::I8 => "I8".into(),
        Ty::U8 => "U8".into(),
        Ty::I16 => "I16".into(),
        Ty::U16 => "U16".into(),
        Ty::I32 => "I32".into(),
        Ty::U32 => "U32".into(),
        Ty::I64 => "I64".into(),
        Ty::U64 => "U64".into(),
        Ty::Off => "Off".into(),
        Ty::UOff => "UOff".into(),
        Ty::Bit { width, signed: true } => format!("Bit{width}"),
        Ty::Bit { width, signed: false } => format!("UBit{width}"),
        Ty::F32 => "F32".into(),
        Ty::F64 => "F64".into(),
        Ty::F80 => "F80".into(),
        Ty::Str => "Str".into(),
        Ty::FixedStr(n) => format!("Str*{n}"),
        Ty::User(id) => format!("T:{}", p.user_type(id).name),
    }
}

fn expr(p: &Program, e: &Expr, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let types = if e.qb == e.ty {
        ty(p, e.ty)
    } else {
        format!("{} (qb {})", ty(p, e.ty), ty(p, e.qb))
    };
    match &e.kind {
        ExprKind::Int(v) => writeln!(out, "{pad}Int {v} : {types}").unwrap(),
        ExprKind::Float(t) => writeln!(out, "{pad}Float {t} : {types}").unwrap(),
        ExprKind::Str(s) => writeln!(out, "{pad}Str \"{}\" : {types}", show_bytes(s)).unwrap(),
        ExprKind::Load(Place::Var(id)) => writeln!(out, "{pad}Var {} : {types}", var_name(p, *id)).unwrap(),
        ExprKind::Load(place @ Place::Element { .. }) => {
            writeln!(out, "{pad}Element {} : {types}", place_text(p, place)).unwrap();
            indexes(p, place, depth + 1, out);
        }
        ExprKind::Load(place @ Place::Member { .. }) => {
            writeln!(out, "{pad}Member {} : {types}", place_text(p, place)).unwrap();
            indexes(p, place, depth + 1, out);
        }
        ExprKind::Bound { upper, array, dim } => {
            let word = if *upper { "UBound" } else { "LBound" };
            // The indexes on the way to a member array are not evaluated, and not shown.
            writeln!(out, "{pad}{word} {} : {types}", array_text(p, array)).unwrap();
            if let Some(d) = dim {
                expr(p, d, depth + 1, out);
            }
        }
        ExprKind::VarPtr { array, index } => {
            writeln!(out, "{pad}VarPtr {} : {types}", var_name(p, *array)).unwrap();
            for i in index {
                expr(p, i, depth + 1, out);
            }
        }
        ExprKind::VarSeg { array } => writeln!(out, "{pad}VarSeg {} : {types}", var_name(p, *array)).unwrap(),
        ExprKind::Convert { how, from } => {
            writeln!(out, "{pad}Convert {how:?} : {types}").unwrap();
            expr(p, from, depth + 1, out);
        }
        ExprKind::Binary { op, lhs, rhs } => {
            writeln!(out, "{pad}Binary {op:?} : {types}").unwrap();
            expr(p, lhs, depth + 1, out);
            expr(p, rhs, depth + 1, out);
        }
        ExprKind::Unary { op, operand } => {
            writeln!(out, "{pad}{op:?} : {types}").unwrap();
            expr(p, operand, depth + 1, out);
        }
        ExprKind::Concat(a, b) => {
            writeln!(out, "{pad}Concat : {types}").unwrap();
            expr(p, a, depth + 1, out);
            expr(p, b, depth + 1, out);
        }
        ExprKind::StrCompare { op, lhs, rhs } => {
            writeln!(out, "{pad}StrCompare {op:?} : {types}").unwrap();
            expr(p, lhs, depth + 1, out);
            expr(p, rhs, depth + 1, out);
        }
        ExprKind::Choice { condition, then, else_ } => {
            writeln!(out, "{pad}Choice : {types}").unwrap();
            expr(p, condition, depth + 1, out);
            expr(p, then, depth + 1, out);
            expr(p, else_, depth + 1, out);
        }
        ExprKind::Call { builtin, args: slots } => {
            writeln!(out, "{pad}Call {} : {types}", builtin.get().name.to_ascii_uppercase()).unwrap();
            for a in slots {
                match a {
                    Some(a) => expr(p, a, depth + 1, out),
                    None => writeln!(out, "{pad}  (absent)").unwrap(),
                }
            }
        }
        ExprKind::CallProc { proc, args: a } => {
            writeln!(out, "{pad}CallProc {} : {types}", p.proc(*proc).name).unwrap();
            args(p, a, depth + 1, out);
        }
    }
}
