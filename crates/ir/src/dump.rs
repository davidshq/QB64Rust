//! `--dump ir`: the lowering-pair text of design D6.

use crate::{
    Arg, ArrayRef, BinOp, Body, Expr, ExprKind, LabelId, MemValue, OnError, Op, Place, PrintItem, PrintTo, ProcKind,
    Program, Reference, Resume, RuntimePart, Settings, Shape, Source, StmtArg, Storage, Ty, UsingValue, VarId, When,
};

use qb64rust_base::{show_bytes, to_u32};
use std::fmt::Write as _;

/// The program's settings on one line: `console, window hidden at the start, runtime parts: icon`.
fn settings(s: &Settings) -> String {
    let mut parts = Vec::new();
    if s.console {
        parts.push("console".to_string());
    }
    parts.push(if s.window { "window" } else { "no window" }.to_string());
    if s.window_hidden {
        parts.push("hidden at the start".to_string());
    }
    if s.asserts {
        parts.push("asserts".to_string());
    }
    if !s.runtime_parts.is_empty() {
        let names: Vec<&str> = s
            .runtime_parts
            .iter()
            .map(|p| match p {
                RuntimePart::Icon => "icon",
                RuntimePart::Audio => "audio",
                RuntimePart::Printer => "printer",
                RuntimePart::ImageCodec => "image codec",
                RuntimePart::Font => "font",
                RuntimePart::ScreenImage => "screen image",
                RuntimePart::DataProcessing => "data processing",
            })
            .collect();
        parts.push(format!("runtime parts: {}", names.join(", ")));
    }
    if s.exe_icon {
        // Not the name: an absolute one would put a folder of the machine into a dump.
        parts.push("an icon for the executable".to_string());
    }
    parts.join(", ")
}

/// The program's settings, the main module's statements, then each procedure: its header, its variables by storage
/// class, its statements.
pub fn dump(p: &Program) -> String {
    let mut out = String::new();
    writeln!(out, "Settings: {}", settings(&p.settings)).unwrap();
    // The program's data, when there is any: one item per line, a quoted one in quotes.
    for (i, item) in p.data.iter().enumerate() {
        let q = if item.quoted { "\"" } else { "" };
        writeln!(out, "Data {i}: {q}{}{q}", show_bytes(&item.text)).unwrap();
    }
    // The member arrays, when there are any: each with its bounds, which are part of its user type.
    for t in &p.types {
        for m in t.members.iter().filter(|m| m.is_array()) {
            let dims: Vec<String> = m.bounds.iter().map(|(l, u)| format!("{l} To {u}")).collect();
            // A dynamic member array's bounds are those it starts with (design D5 of `m3-typefields`).
            let dynamic = if m.dynamic { " dynamic" } else { "" };
            writeln!(
                out,
                "Member array {}.{}:{} ({}){dynamic}",
                t.name,
                m.name,
                ty(p, m.ty),
                dims.join(", ")
            )
            .unwrap();
        }
    }
    // The arrays, when there are any: a static one with its bounds, a dynamic one with its number of dimensions.
    for (i, v) in p.vars.iter().enumerate() {
        let name = var(p, VarId(to_u32(i)));
        let memory = if v.conventional { ", in conventional memory" } else { "" };
        match &v.shape {
            Shape::Scalar => {}
            Shape::Static { bounds } => {
                let dims: Vec<String> = bounds.iter().map(|(l, u)| format!("{l} To {u}")).collect();
                writeln!(out, "Array {name}:{} static ({}){memory}", ty(p, v.ty), dims.join(", ")).unwrap();
            }
            Shape::Dynamic { dims, at_start } => {
                let dims = dims.map_or("?".to_string(), |n| n.to_string());
                let start = at_start.as_ref().map_or(String::new(), |bounds| {
                    let dims: Vec<String> = bounds.iter().map(|(l, u)| format!("{l} To {u}")).collect();
                    format!(", allocated at the start ({})", dims.join(", "))
                });
                // An array parameter stands for the array passed, static or dynamic.
                let kind = if matches!(v.storage, Storage::Param(_)) {
                    "passed"
                } else {
                    "dynamic"
                };
                writeln!(
                    out,
                    "Array {name}:{} {kind}, dimensions: {dims}{start}{memory}",
                    ty(p, v.ty)
                )
                .unwrap();
            }
        }
    }
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
            let parens = if class == "param" && v.is_array() { "()" } else { "" };
            writeln!(out, "  {class} {}{parens}:{}", v.name, ty(p, v.ty)).unwrap();
        }
        body(p, &proc.body, &mut out);
    }
    out
}

/// Where a print goes: `console`, `file <number>` or `printer`.
fn print_to(p: &Program, to: &PrintTo) -> String {
    match to {
        PrintTo::Screen => "console".to_string(),
        PrintTo::File(n) => format!("file {}", val(p, n)),
        PrintTo::Printer => "printer".to_string(),
    }
}

fn print_items(p: &Program, items: &[PrintItem], out: &mut String) {
    for i in items {
        match i {
            PrintItem::Str(v) => writeln!(out, "    Str {}", val(p, v)).unwrap(),
            PrintItem::Num(v) => writeln!(out, "    Num {}", val(p, v)).unwrap(),
            PrintItem::Zone => writeln!(out, "    Zone").unwrap(),
        }
    }
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
                Op::MemGet { block, offset, target } => writeln!(
                    out,
                    "  MemGet {} at {} into {}",
                    place_text(p, block),
                    val(p, offset),
                    reference_text(p, target)
                )
                .unwrap(),
                Op::MemPut { block, offset, value } => writeln!(
                    out,
                    "  MemPut {} at {} = {}",
                    place_text(p, block),
                    val(p, offset),
                    mem_value(p, value)
                )
                .unwrap(),
                Op::MemFill {
                    block,
                    offset,
                    bytes,
                    value,
                } => writeln!(
                    out,
                    "  MemFill {} at {} for {} with {}",
                    place_text(p, block),
                    val(p, offset),
                    val(p, bytes),
                    mem_value(p, value)
                )
                .unwrap(),
                Op::End(None) => writeln!(out, "  End").unwrap(),
                Op::System(None) => writeln!(out, "  System").unwrap(),
                Op::End(Some(code)) => writeln!(out, "  End {}", val(p, code)).unwrap(),
                Op::System(Some(code)) => writeln!(out, "  System {}", val(p, code)).unwrap(),
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
                Op::CopyWhole { from, to } => {
                    let t = ty(p, p.place_ty(to));
                    writeln!(out, "  CopyWhole {}:{t} <- {}", place_text(p, to), place_text(p, from)).unwrap();
                }
                Op::Allocate { array, bounds, mode } => {
                    writeln!(
                        out,
                        "  Allocate {mode:?} {}:{}",
                        array_text(p, array),
                        ty(p, p.array_ty(array))
                    )
                    .unwrap();
                    for (lower, upper) in bounds {
                        writeln!(out, "    {} To {}", val(p, lower), val(p, upper)).unwrap();
                    }
                }
                Op::Clear { array } => {
                    writeln!(out, "  Clear {}:{}", array_text(p, array), ty(p, p.array_ty(array))).unwrap();
                }
                Op::Release { array } => {
                    writeln!(out, "  Release {}:{}", array_text(p, array), ty(p, p.array_ty(array))).unwrap();
                }
                Op::EraseParam { array } => {
                    writeln!(out, "  EraseParam {}:{}", var(p, *array), ty(p, p.var(*array).ty)).unwrap();
                }
                Op::CopyArray { from, to } => {
                    let t = ty(p, p.array_ty(to));
                    let (from, to) = (array_text(p, from), array_text(p, to));
                    writeln!(out, "  CopyArray {from} -> {to}:{t}").unwrap();
                }
                Op::Print { to, items, newline } => {
                    let to = print_to(p, to);
                    writeln!(out, "  Print {to}{}", if *newline { " newline" } else { "" }).unwrap();
                    print_items(p, items, out);
                }
                Op::PrintUsing {
                    to,
                    before,
                    format,
                    values,
                    newline,
                } => {
                    let to = print_to(p, to);
                    writeln!(out, "  PrintUsing {to}{}", if *newline { " newline" } else { "" }).unwrap();
                    print_items(p, before, out);
                    writeln!(out, "    Format {}", val(p, format)).unwrap();
                    for v in values {
                        match v {
                            UsingValue::Str(e) => writeln!(out, "    Str {}", val(p, e)).unwrap(),
                            UsingValue::Num { value, as_ } => {
                                writeln!(out, "    Num as {}: {}", ty(p, *as_), val(p, value)).unwrap();
                            }
                            UsingValue::Blanks(e) => writeln!(out, "    Blanks {}", val(p, e)).unwrap(),
                        }
                    }
                }
                Op::Field { file, parts } => {
                    writeln!(out, "  Field file {}", val(p, file)).unwrap();
                    for (place, width) in parts {
                        let t = ty(p, p.place_ty(place));
                        writeln!(out, "    {}:{t} width {}", place_text(p, place), val(p, width)).unwrap();
                    }
                }
                Op::Lset { right, target, value } => {
                    let word = if *right { "Rset" } else { "Lset" };
                    let t = ty(p, p.place_ty(target));
                    writeln!(out, "  {word} {}:{t} = {}", place_text(p, target), val(p, value)).unwrap();
                }
                Op::Write { to, items, newline } => {
                    let to = match to {
                        None => "console".to_string(),
                        Some(n) => format!("file {}", val(p, n)),
                    };
                    writeln!(out, "  Write {to}{}", if *newline { " newline" } else { "" }).unwrap();
                    print_items(p, items, out);
                }
                Op::Input { from, line, targets } => {
                    let word = if *line { "LineInput" } else { "Input" };
                    let from = match from {
                        Source::File(n) => format!("file {}", val(p, n)),
                        Source::Console { prompt, question, stay } => {
                            let prompt = match prompt {
                                Some(text) => format!(" prompt \"{}\"", show_bytes(text)),
                                None => String::new(),
                            };
                            let question = if *question { " question" } else { "" };
                            let stay = if *stay { " stay" } else { "" };
                            format!("console{prompt}{question}{stay}")
                        }
                    };
                    writeln!(out, "  {word} {from}").unwrap();
                    for place in targets {
                        writeln!(out, "    Target {}:{}", place_text(p, place), ty(p, p.place_ty(place))).unwrap();
                    }
                }
                Op::Read(targets) => {
                    writeln!(out, "  Read").unwrap();
                    for place in targets {
                        writeln!(out, "    Target {}:{}", place_text(p, place), ty(p, p.place_ty(place))).unwrap();
                    }
                }
                Op::Restore { at, label } => match label {
                    Some(l) => writeln!(out, "  Restore item {at} ({l})").unwrap(),
                    None => writeln!(out, "  Restore item {at}").unwrap(),
                },
                Op::Call { proc, args: a } => {
                    writeln!(out, "  Call {} [{}]", p.proc(*proc).name, args(p, a)).unwrap();
                }
                Op::Builtin { id, args: a } => {
                    let slots: Vec<String> = a
                        .iter()
                        .map(|arg| match arg {
                            StmtArg::Value(v) => format!("Value({})", val(p, v)),
                            StmtArg::Place(place) => {
                                format!("Place {}:{}", place_text(p, place), ty(p, p.place_ty(place)))
                            }
                            StmtArg::Block { array, index } if index.is_empty() => format!("Bytes {}", var(p, *array)),
                            StmtArg::Block { array, index } => {
                                let i: Vec<String> = index.iter().map(|i| val(p, i)).collect();
                                format!("Bytes {}({})", var(p, *array), i.join(", "))
                            }
                            StmtArg::Bytes(place) => {
                                format!("BytesOf {}:{}", place_text(p, place), ty(p, p.place_ty(place)))
                            }
                            StmtArg::Word(w) => format!("Word {w}"),
                            StmtArg::Absent => "Absent".to_string(),
                        })
                        .collect();
                    // The entry's place among the forms of its name (`OPEN` has two), when there are several.
                    let b = id.get();
                    let forms = qb64rust_builtins::find_statements(b.name.as_bytes(), b.musthave == Some("$"));
                    let form = match forms.iter().position(|f| f == id) {
                        Some(k) if forms.len() > 1 => format!(" form {}", k + 1),
                        Some(_) | None => String::new(),
                    };
                    let name = b.name.to_ascii_uppercase();
                    writeln!(out, "  Builtin {name}{form} [{}]", slots.join(", ")).unwrap();
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
        Ty::FixedStr(n) => format!("Str*{n}"),
    }
}

/// A place: a variable as [`var`] shows it, `X(i, …)` for an element, `P.M` for a member.
fn place_text(p: &Program, place: &Place) -> String {
    match place {
        Place::Var(v) => var(p, *v),
        Place::Element { array, index } => {
            let i: Vec<String> = index.iter().map(|v| val(p, v)).collect();
            format!("{}({})", array_text(p, array), i.join(", "))
        }
        Place::Member { base, member } => {
            let m = &p.member(p.place_ty(base), *member).name;
            format!("{}.{m}", place_text(p, base))
        }
    }
}

/// An array: a variable as [`var`] shows it, `P.M` for a member array.
fn array_text(p: &Program, array: &ArrayRef) -> String {
    match array {
        ArrayRef::Var(v) => var(p, *v),
        ArrayRef::Member { base, member } => {
            let m = &p.member_of(base, *member).name;
            format!("{}.{m}", place_text(p, base))
        }
    }
}

/// What `_MEMPUT` writes or `_MEMFILL` fills with.
fn mem_value(p: &Program, value: &MemValue) -> String {
    match value {
        MemValue::Typed { value, ty: t } => format!("{} as {}", val(p, value), ty(p, *t)),
        MemValue::Bytes(of) => format!("bytes of {}", reference_text(p, of)),
        MemValue::Str(s) => format!("string {}", val(p, s)),
    }
}

/// What `_MEM`, `_MEMELEMENT` and `_OFFSET` name: a place, or an array as a whole with `()`.
fn reference_text(p: &Program, of: &Reference) -> String {
    match of {
        Reference::Place(place) => place_text(p, place),
        Reference::Array(array) => format!("{}()", array_text(p, array)),
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
            Arg::Array(a) => format!("Array {}:{}", array_text(p, a), ty(p, p.array_ty(a))),
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
            format!("{word} {}({d}):{}", array_text(p, array), ty(p, v.ty))
        }
        ExprKind::VarPtr { array, index } => {
            let i: Vec<String> = index.iter().map(|i| val(p, i)).collect();
            format!("VarPtr {}({}):{}", var(p, *array), i.join(", "), ty(p, v.ty))
        }
        ExprKind::VarSeg { array } => format!("VarSeg {}:{}", var(p, *array), ty(p, v.ty)),
        ExprKind::MemOf { of, element } => {
            let word = if *element { "MemElement" } else { "Mem" };
            format!("{word} {}:{}", reference_text(p, of), ty(p, v.ty))
        }
        ExprKind::AddressOf(of) => format!("AddressOf {}:{}", reference_text(p, of), ty(p, v.ty)),
        ExprKind::MemRead { block, offset, of } => format!(
            "MemRead {} at {} as {}:{}",
            place_text(p, block),
            val(p, offset),
            ty(p, *of),
            ty(p, v.ty)
        ),
        ExprKind::MemAt { offset, size } => format!("MemAt({}, {}):{}", val(p, offset), val(p, size), ty(p, v.ty)),
        ExprKind::Block { array, index } if index.is_empty() => format!("Bytes {}", var(p, *array)),
        ExprKind::Block { array, index } => {
            let i: Vec<String> = index.iter().map(|i| val(p, i)).collect();
            format!("Bytes {}({})", var(p, *array), i.join(", "))
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
        ExprKind::Choice { condition, then, else_ } => format!(
            "Choice:{}({} ? {} : {})",
            ty(p, v.ty),
            val(p, condition),
            val(p, then),
            val(p, else_)
        ),
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
