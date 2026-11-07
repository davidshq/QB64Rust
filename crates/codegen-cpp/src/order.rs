//! The order in which the parts of one expression, call or statement are evaluated (OpenSpec change
//! `eval-order-left-to-right`; spec `language/evaluation-order`).
//!
//! C++ leaves open the order of the arguments of one call and of the two operands of `+`, `-`, `==` and the other
//! arithmetic and comparison operators. Clang, which QB64pe's Windows build uses, evaluates them left to right;
//! `g++` on Linux often evaluates call arguments right to left. So the same C++ can give another result on Linux.
//! This module fixes the order to the Windows one on every platform:
//!
//! - A **group** is the parts of one IR node that the emitter writes as the arguments of one C++ call or the
//!   operands of one such operator, in the order of the C++ text ([`expr_group`], [`op_group`], design D1). Each
//!   `match` has no wildcard arm, so a new kind does not compile until it is placed.
//! - Each part has a [`Class`]: whether it changes state, raises an error, or reads anything (design D2).
//! - Two parts **conflict** when another order could change what the program does ([`conflict`], design D3). A
//!   part is **held** when it conflicts with a later part of its group: it is evaluated into a temporary `seqN`
//!   first, in the group's order. The last part is never held. A group with no conflict keeps its C++ as it was.
//!
//! A held part is written by [`Emitter::hold`] and found again by the writers through two maps: `value` checks
//! [`Emitter::held_exprs`] first, `load_place`, `place_ref` and `bytes_of` check [`Emitter::held_places`]. A hold
//! inside an expression is a C++ comma expression at the node itself, `(seq1=…,node)`, so a node inside one branch
//! of `_IIF` or on the right of `_ANDALSO` holds its parts only when that branch runs (design D4). A statement's
//! holds are `seqN=…;` lines just before its one call.

use crate::Emitter;
use crate::decl::is_qbs;
use crate::names::c_type;
use qb64rust_builtins::BuiltinId;
use qb64rust_builtins::passing::plan;
use qb64rust_ir::builtins::{Rule, Slot, StmtRule, find, slots, stmt_find};
use qb64rust_ir::{
    AddressWord, Arg, ArrayRef, BinOp, Event, EventHandler, Expr, ExprKind, KeyForm, MemValue, Op, Place, Program,
    Reference, StmtArg, Ty,
};

/// Whether a part can raise an error, and which.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Raises {
    #[default]
    No,
    /// Only error 9, from the checks of indexes (and the guard of a whole array or a dynamic member array, and
    /// `LBOUND` and `UBOUND`). Two such parts give the same error and element 0 in either order (user, Q1,
    /// 2026-11-07).
    Index,
    /// Any other error.
    Other,
}

/// What can be seen of evaluating a part (design D2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Class {
    /// It calls a user FUNCTION, a library procedure, or a built-in of [`CHANGES_STATE`].
    pub(crate) state: bool,
    pub(crate) raises: Raises,
    /// It is not a constant: it reads a variable, an element, a descriptor or the runtime's state.
    pub(crate) reads: bool,
}

impl Class {
    /// A number or a string literal, or a place found without evaluating anything.
    pub(crate) const CONSTANT: Class = Class {
        state: false,
        raises: Raises::No,
        reads: false,
    };
    pub(crate) const READS: Class = Class {
        state: false,
        raises: Raises::No,
        reads: true,
    };
    pub(crate) const STATE: Class = Class {
        state: true,
        raises: Raises::Other,
        reads: true,
    };
    pub(crate) const RAISES: Class = Class {
        state: false,
        raises: Raises::Other,
        reads: false,
    };
    pub(crate) const INDEX: Class = Class {
        state: false,
        raises: Raises::Index,
        reads: false,
    };

    pub(crate) fn join(self, other: Class) -> Class {
        Class {
            state: self.state || other.state,
            raises: self.raises.max(other.raises),
            reads: self.reads || other.reads,
        }
    }
}

/// Whether evaluating `a` and `b` in the other order could change what the program does (design D3): one
/// changes state and the other is not a constant, or both raise, unless both raise only error 9 from index checks.
pub(crate) fn conflict(a: Class, b: Class) -> bool {
    let state = (a.state && b.reads) || (b.state && a.reads);
    let raises =
        a.raises != Raises::No && b.raises != Raises::No && !(a.raises == Raises::Index && b.raises == Raises::Index);
    state || raises
}

/// Which parts of a group are held: each one that conflicts with a later part (design D3). The last is never held.
pub(crate) fn held(classes: &[Class]) -> Vec<bool> {
    (0..classes.len())
        .map(|i| classes[i + 1..].iter().any(|&later| conflict(classes[i], later)))
        .collect()
}

/// The built-in functions that change program state, read against libqb's sources (user, Q2, 2026-11-07). Each
/// name is compared without regard to case or a `$`. A built-in not here counts as only reading.
const CHANGES_STATE: &[&str] = &[
    // The seed (`libqb.cpp`, `func_rnd`).
    "RND",
    // A key taken from the keyboard buffer (`libqb\src\keyboard.cpp`, `qbs_inkey`, `func__keyhit`).
    "INKEY$",
    "_KEYHIT",
    // Characters read from a file or the keyboard (`libqb.cpp`, `func_input`).
    "INPUT$",
    // A port read; port &H3C9 steps the palette index (`libqb.cpp`, `func_inp`).
    "INP",
    // An event taken from the queue (`libqb\src\mouse.cpp`, `func__mouseinput`; `qbx.cpp`, `func__deviceinput`).
    "_MOUSEINPUT",
    "_DEVICEINPUT",
    // A key or an event read from the console (`libqb\src\console.cpp`, `func__cinp`, `func__getconsoleinput`).
    "_CINP",
    "_CONSOLEINPUT",
    // A new image or font handle (`libqb.cpp`, `func__newimage`, `func__copyimage`, `func__loadfont`,
    // `func__screenimage`; `parts\video\image\image.cpp`, `func__loadimage`; `parts\os\clipboard\clipboard.cpp`,
    // `func__clipboardimage`).
    "_NEWIMAGE",
    "_COPYIMAGE",
    "_LOADFONT",
    "_SCREENIMAGE",
    "_LOADIMAGE",
    "_CLIPBOARDIMAGE",
    // A new memory block and its lock (`libqb.cpp`, `func__memimage`; `libqb\src\memblock.cpp`, `func__memnew`;
    // `parts\audio\audio.cpp`, `func__memsound`).
    "_MEMIMAGE",
    "_MEMNEW",
    "_MEMSOUND",
    // A new sound handle (`parts\audio\audio.cpp`).
    "_SNDOPEN",
    "_SNDOPENRAW",
    "_SNDCOPY",
    "_SNDNEW",
    // A new connection handle (`libqb.cpp`, `func__openhost`, `func__openclient`, `func__openconnection`).
    "_OPENHOST",
    "_OPENCLIENT",
    "_OPENCONNECTION",
    // A timer number taken from the free list (`qbx.cpp`, `func__freetimer`).
    "_FREETIMER",
    // A program run (`libqb\src\shell.cpp`, `func_shell`, `func__shellhide`).
    "SHELL",
    "_SHELLHIDE",
    // The next name of a directory listing (`libqb\src\filesystem.cpp`, `func__files`).
    "_FILES$",
    // The next dropped file (`libqb\src\window-gui.cpp`, `func__droppedfile`).
    "_DROPPEDFILE",
    // A request or an event cleared once read (`libqb.cpp`, `func__exit`; `libqb\src\window-gui.cpp`,
    // `func__resize`).
    "_EXIT",
    "_RESIZE",
    // A dialog the user answers (`parts\gui\gui.cpp`).
    "_INPUTBOX$",
    "_MESSAGEBOX",
    "_OPENFILEDIALOG$",
    "_SAVEFILEDIALOG$",
    "_SELECTFOLDERDIALOG$",
    "_COLORCHOOSERDIALOG",
];

/// The built-in functions that never raise an error: their libqb entry calls no `error()` (read with a scan of
/// each entry's body), or the emitter writes them inline with nothing that raises. Every other built-in counts as
/// raising (design D2). The bit functions are not here: the fix `bit-number-check` makes their bit number raise.
const CANNOT_RAISE: &[&str] = &[
    // Inline (`builtins.rs`): `->len`, `std::floor`, `std::min` and the others, the rotations, the bytes of a
    // number, and C++'s own mathematics.
    "LEN",
    "INT",
    "FIX",
    "_MIN",
    "_MAX",
    "_CLAMP",
    "_ROL",
    "_ROR",
    "MKI$",
    "MKL$",
    "MKS$",
    "MKD$",
    "_MK$",
    "SIN",
    "COS",
    "TAN",
    "ATN",
    "_ACOS",
    "_ASIN",
    "_ATAN2",
    "_HYPOT",
    "_CEIL",
    "_SINH",
    "_COSH",
    "_TANH",
    "_ASINH",
    "_ACOSH",
    "_ATANH",
    // The string functions (`libqb\src\qbs.cpp`, `libqb\src\string_functions.cpp`, `qbs_str.cpp`).
    "LEFT$",
    "RIGHT$",
    "MID$",
    "LCASE$",
    "UCASE$",
    "LTRIM$",
    "RTRIM$",
    "_TRIM$",
    "STR$",
    "INSTR",
    "_INSTRREV",
    "SPACE$",
    "STRING$",
    "_STRCMP",
    "_STRICMP",
    // Numbers (`libqb\include\qbmath.h`, `extended_math.h`).
    "ABS",
    "SGN",
    "_PI",
    "_D2R",
    "_D2G",
    "_R2D",
    "_R2G",
    "_G2D",
    "_G2R",
    "_SHL",
    "_SHR",
    // Colors (`libqb\include\graphics.h`).
    "_RGB32",
    "_RGBA32",
    "_RED32",
    "_GREEN32",
    "_BLUE32",
    "_ALPHA32",
    // The cursor, the error and the program (`libqb.cpp`, `libqb\src\error_handle.cpp`, `command.cpp`).
    "CSRLIN",
    "POS",
    "ERR",
    "ERL",
    "_ERRORLINE",
    "_INCLERRORLINE",
    "_INCLERRORFILE$",
    "_ERRORMESSAGE$",
    "COMMAND$",
    "_COMMANDCOUNT",
    "_OS$",
    "_STARTDIR$",
    "_DEST",
    "_SOURCE",
    "_MOUSEX",
    "_MOUSEY",
    // State changed, nothing raised (`func_rnd`, `qbs_inkey`, `func__keyhit`).
    "RND",
    "INKEY$",
    "_KEYHIT",
];

/// Whether `id`, a built-in function, is named in `list`.
fn named(id: BuiltinId, list: &[&str]) -> bool {
    let name = id.get().name;
    list.iter()
        .any(|n| n.trim_end_matches('$').eq_ignore_ascii_case(name.trim_end_matches('$')))
}

/// The class of a built-in function's own call, without its arguments.
fn builtin_class(id: BuiltinId) -> Class {
    let mut c = Class::READS;
    if named(id, CHANGES_STATE) {
        c = c.join(Class::STATE);
        c.raises = Raises::No;
    }
    if !named(id, CANNOT_RAISE) {
        c = c.join(Class::RAISES);
    }
    c
}

/// What the order rule needs to know about a program and the statement being written.
#[derive(Clone, Copy)]
pub(crate) struct Ctx<'p> {
    pub(crate) p: &'p Program,
    /// The statement is checked: an index raises error 9 outside its bounds ([`Emitter::checking`]).
    pub(crate) checking: bool,
}

impl Ctx<'_> {
    fn index(self) -> Class {
        if self.checking { Class::INDEX } else { Class::CONSTANT }
    }

    pub(crate) fn expr(self, e: &Expr) -> Class {
        let all = |es: &mut dyn Iterator<Item = &Expr>| es.fold(Class::CONSTANT, |c, e| c.join(self.expr(e)));
        match &e.kind {
            ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) => Class::CONSTANT,
            ExprKind::Load(place) => self.place(place).join(Class::READS),
            // A dimension outside the array's, or an array with no storage, raises error 9 and gives 0, whatever is
            // evaluated before or after it: as an index check does.
            ExprKind::Bound { array, dim, .. } => {
                let dim = dim
                    .as_deref()
                    .map_or(Class::CONSTANT, |d| self.expr(d).join(Class::INDEX));
                match array {
                    ArrayRef::Var(_) => dim.join(Class::READS).join(Class::INDEX),
                    ArrayRef::Member { base, member } if self.p.member_of(base, *member).dynamic => {
                        dim.join(self.place(base)).join(Class::READS).join(Class::INDEX)
                    }
                    // The member's bound, a constant: the place is not found.
                    ArrayRef::Member { .. } => dim,
                }
            }
            ExprKind::Address { word, of } => match word {
                AddressWord::VarSeg => Class::CONSTANT,
                AddressWord::VarPtr => self.place(of),
                AddressWord::Sadd | AddressWord::VarPtrStr => self.place(of).join(Class::READS),
            },
            ExprKind::Block { index, .. } => {
                let found = if index.is_empty() {
                    Class::CONSTANT
                } else {
                    self.index()
                };
                all(&mut index.iter()).join(found).join(Class::READS)
            }
            ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => self.expr(from),
            ExprKind::Binary { op, lhs, rhs } => {
                let own = match op {
                    BinOp::IDiv | BinOp::Mod | BinOp::Pow => Class::RAISES,
                    BinOp::Add
                    | BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::Eq
                    | BinOp::Ne
                    | BinOp::Lt
                    | BinOp::Gt
                    | BinOp::Le
                    | BinOp::Ge
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::Xor
                    | BinOp::Eqv
                    | BinOp::Imp
                    | BinOp::AndAlso
                    | BinOp::OrElse => Class::CONSTANT,
                };
                self.expr(lhs).join(self.expr(rhs)).join(own)
            }
            ExprKind::Concat(lhs, rhs) | ExprKind::StrCompare { lhs, rhs, .. } => self.expr(lhs).join(self.expr(rhs)),
            ExprKind::Choice { condition, then, else_ } => {
                self.expr(condition).join(self.expr(then)).join(self.expr(else_))
            }
            // `CHR$` of a constant from 0 to 255 is a constant: it raises nothing (`func_chr` in libqb's `qbs.cpp`),
            // and programs join many of them (`CHR$(27) + CHR$(91)`).
            ExprKind::Call { builtin, args }
                if builtin.get().name.eq_ignore_ascii_case("chr")
                    && matches!(
                        &args[..],
                        [Some(Expr {
                            kind: ExprKind::Int(0..=255),
                            ..
                        })]
                    ) =>
            {
                Class::CONSTANT
            }
            ExprKind::Call { builtin, args } => all(&mut args.iter().flatten()).join(builtin_class(*builtin)),
            ExprKind::CallProc { args, .. } => args.iter().fold(Class::STATE, |c, a| c.join(self.arg(a))),
            ExprKind::MemOf { of, .. } | ExprKind::AddressOf(of) => self.reference(of).join(Class::READS),
            ExprKind::MemAt { offset, size } => self.expr(offset).join(self.expr(size)).join(Class::RAISES),
            ExprKind::MemRead { block, offset, .. } => self
                .place(block)
                .join(self.expr(offset))
                .join(Class::READS)
                .join(Class::RAISES),
            ExprKind::ArrayBytes(_) => Class::READS,
        }
    }

    /// Finding a place, without reading its value: a variable is found without evaluating anything.
    pub(crate) fn place(self, place: &Place) -> Class {
        match place {
            Place::Var(_) => Class::CONSTANT,
            Place::Element { array, index } => index
                .iter()
                .fold(self.array(array), |c, i| c.join(self.expr(i)))
                .join(self.index())
                .join(Class::READS),
            Place::Member { base, .. } => self.place(base),
        }
    }

    fn array(self, array: &ArrayRef) -> Class {
        match array {
            ArrayRef::Var(_) => Class::CONSTANT,
            // A dynamic member array's descriptor is read and guarded (error 9 while it has no storage).
            ArrayRef::Member { base, member } if self.p.member_of(base, *member).dynamic => {
                self.place(base).join(self.index()).join(Class::READS)
            }
            ArrayRef::Member { base, .. } => self.place(base),
        }
    }

    fn reference(self, of: &Reference) -> Class {
        match of {
            Reference::Place(place) => self.place(place),
            Reference::Array(array) => self.array(array),
        }
    }

    fn arg(self, a: &Arg) -> Class {
        match a {
            Arg::Ref(place) => self.place(place),
            Arg::Temp(v, _) | Arg::Value(v) => self.expr(v),
            Arg::Array(array) => self.array(array),
            Arg::Null => Class::CONSTANT,
        }
    }

    fn stmt_arg(self, a: &StmtArg) -> Class {
        match a {
            StmtArg::Value(v) => self.expr(v),
            StmtArg::Place(place) | StmtArg::Bytes(place) | StmtArg::Rest(place) => self.place(place),
            StmtArg::Block { index, .. } => {
                let found = if index.is_empty() {
                    Class::CONSTANT
                } else {
                    self.index()
                };
                index.iter().fold(found.join(Class::READS), |c, i| c.join(self.expr(i)))
            }
            StmtArg::Word(_) | StmtArg::Absent => Class::CONSTANT,
        }
    }
}

/// One part of a group: what the writer passes to `value` (an expression), or to `load_place`, `place_ref` or
/// `bytes_of` (a place), so that the writer finds it held. `Unheld` is a part the emitter cannot hold, whose class
/// still counts for the parts before it: the bytes of an array (`ExprKind::Block`, `StmtArg::Block`,
/// `StmtArg::Rest`), whose index reads only (`sema`), and which is written as a `byte_element` call.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Part<'a> {
    Expr(&'a Expr),
    Place(&'a Place),
    Unheld(Class),
}

impl Part<'_> {
    pub(crate) fn class(self, cx: Ctx<'_>) -> Class {
        match self {
            Part::Expr(e) => cx.expr(e),
            Part::Place(place) => cx.place(place),
            Part::Unheld(c) => c,
        }
    }
}

/// An expression as a part: one the emitter cannot hold is [`Part::Unheld`].
fn expr_part<'a>(cx: Ctx<'_>, e: &'a Expr) -> Part<'a> {
    match (&e.kind, e.ty) {
        (ExprKind::Block { .. }, _) | (_, Ty::User(_)) => Part::Unheld(cx.expr(e)),
        _ => Part::Expr(e),
    }
}

/// The parts of the arguments of a call of a procedure, as [`Emitter::arg`] and `library_args` write them.
fn arg_parts<'a>(cx: Ctx<'_>, args: &'a [Arg], parts: &mut Vec<Part<'a>>) {
    for a in args {
        match a {
            Arg::Ref(place) => parts.push(Part::Place(place)),
            Arg::Temp(v, _) => parts.push(expr_part(cx, v)),
            Arg::Value(v) => parts.push(expr_part(cx, crate::library::byval_value(v))),
            // A member array is found through its base place's bytes (`member_array_arg`, `dyn_slot`).
            Arg::Array(ArrayRef::Member { base, .. }) => parts.push(Part::Place(base)),
            Arg::Array(ArrayRef::Var(_)) | Arg::Null => {}
        }
    }
}

/// The group of an expression node: its parts in the order of the C++ text, empty when the node writes them in an
/// order C++ fixes or has fewer than two (design D1).
pub(crate) fn expr_group<'a>(cx: Ctx<'_>, e: &'a Expr) -> Vec<Part<'a>> {
    let mut parts = Vec::new();
    match &e.kind {
        // One part or none.
        ExprKind::Int(_)
        | ExprKind::Float(_)
        | ExprKind::Str(_)
        | ExprKind::Load(_)
        | ExprKind::Address { .. }
        | ExprKind::Block { .. }
        | ExprKind::Convert { .. }
        | ExprKind::Unary { .. }
        | ExprKind::MemOf { .. }
        | ExprKind::AddressOf(_)
        | ExprKind::ArrayBytes(_) => {}
        // `func_lbound(desc,dim,n)`: the dimension is the one part evaluated. A member array's bound is a constant,
        // or its dimension in a comma expression.
        ExprKind::Bound {
            array: ArrayRef::Member { base, member },
            dim: Some(dim),
            ..
        } if cx.p.member_of(base, *member).dynamic => {
            parts.push(Part::Place(base));
            parts.push(expr_part(cx, dim));
        }
        ExprKind::Bound { .. } => {}
        // `&&` and `||` are ordered, and the right side runs only when needed.
        ExprKind::Binary {
            op: BinOp::AndAlso | BinOp::OrElse,
            ..
        } => {}
        // The operators and `qb_safe_idiv`, `qb_safe_mod`, `pow2`; `qbs_add` and `qbs_equal` and their kin.
        ExprKind::Binary { lhs, rhs, .. } | ExprKind::Concat(lhs, rhs) | ExprKind::StrCompare { lhs, rhs, .. } => {
            parts.push(expr_part(cx, lhs));
            parts.push(expr_part(cx, rhs));
        }
        // `?:` is ordered, and only one branch runs.
        ExprKind::Choice { .. } => {}
        ExprKind::Call { builtin, args } => call_parts(cx, *builtin, args, &mut parts),
        ExprKind::CallProc { args, .. } => arg_parts(cx, args, &mut parts),
        ExprKind::MemAt { offset, size } => {
            parts.push(expr_part(cx, offset));
            parts.push(expr_part(cx, size));
        }
        // The block is written only where the statement is checked (`checked_block`).
        ExprKind::MemRead { block, offset, .. } => {
            if cx.checking {
                parts.push(Part::Place(block));
            }
            parts.push(expr_part(cx, offset));
        }
    }
    if parts.len() < 2 { Vec::new() } else { parts }
}

/// The parts of a built-in function's call, as `Emitter::call` writes its rule.
fn call_parts<'a>(cx: Ctx<'_>, id: BuiltinId, args: &'a [Option<Expr>], parts: &mut Vec<Part<'a>>) {
    let table = |slots: &[Slot], parts: &mut Vec<Part<'a>>| {
        for (i, a) in args.iter().flatten().enumerate() {
            match (slots.get(i), &a.kind) {
                // A block: the address of its place (`mem_ref`).
                (Some(Slot::Mem), ExprKind::Load(place)) => parts.push(Part::Place(place)),
                _ => parts.push(expr_part(cx, a)),
            }
        }
    };
    let Some(s) = find(id) else {
        return table(&[], parts);
    };
    match s.rule {
        Rule::Plain
        | Rule::ResultOfArg
        | Rule::FloatByArg
        | Rule::Fixed(_)
        | Rule::StrOrIndex
        | Rule::Embedded
        | Rule::GivenArgs(_) => {
            let slots: Vec<Slot> = slots(s).into_iter().map(|(slot, _)| slot).collect();
            if let Rule::GivenArgs(_) = s.rule {
                // Only the arguments given, each by its slot from the first.
                return table(&slots, parts);
            }
            // Each slot by its position, the absent ones included.
            for (i, a) in args.iter().enumerate() {
                let Some(a) = a else { continue };
                match (slots.get(i), &a.kind) {
                    (Some(Slot::Mem), ExprKind::Load(place)) => parts.push(Part::Place(place)),
                    _ => parts.push(expr_part(cx, a)),
                }
            }
        }
        // Every argument given, in order.
        Rule::Asc | Rule::StringFill | Rule::Widest | Rule::Rotate => {
            parts.extend(args.iter().flatten().map(|a| expr_part(cx, a)));
        }
        // One argument evaluated.
        Rule::IntFix
        | Rule::Exp
        | Rule::Convert(_)
        | Rule::Len
        | Rule::Val
        | Rule::Radix(_)
        | Rule::Mk(_)
        | Rule::Cv(_) => {}
        Rule::Choice | Rule::Own | Rule::Cast => {}
    }
}

/// How a statement's parts are ordered.
pub(crate) enum OpGroup<'a> {
    /// Each part, or each step, is a C++ statement of its own, or it has fewer than two parts: nothing to do.
    None,
    /// One C++ call: its holds are written as lines before the operation.
    Call(Vec<Part<'a>>),
    /// Several calls, each with its own group, which the writer holds itself ([`field_group`]).
    PerCall,
}

/// The group of a statement (design D1): its parts in the order of the C++ text of its one call.
pub(crate) fn op_group<'a>(cx: Ctx<'_>, op: &'a Op) -> OpGroup<'a> {
    let mut parts = Vec::new();
    match op {
        Op::SelectConsole
        | Op::End(_)
        | Op::System(_)
        | Op::Stop
        | Op::Exit
        | Op::SetHandler(_)
        | Op::NewHandler(_)
        | Op::LastHandler
        | Op::Run(_)
        | Op::PaletteUsing { .. }
        | Op::Raise(_)
        | Op::Resume(_)
        | Op::Jump(_)
        | Op::Branch { .. }
        | Op::Gosub(_)
        | Op::Return(_)
        | Op::Restore { .. }
        // The file, the procedure and the line are constants; the message is the one part evaluated.
        | Op::Log { .. }
        // Separate C++ statements: each store, each copy, each bound, each item, each target, each argument's
        // offset before the call.
        | Op::AssignAll(_)
        | Op::CopyWhole { .. }
        | Op::Allocate { .. }
        | Op::Clear { .. }
        | Op::Release { .. }
        | Op::EraseParam { .. }
        | Op::CopyArray { .. }
        | Op::ArrayCopy { .. }
        | Op::Print { .. }
        | Op::PrintUsing { .. }
        | Op::Write { .. }
        | Op::Input { .. }
        | Op::Read { .. }
        | Op::CallAbsolute { .. }
        // The offset into `tmp_long` first, then the copy; unchecked, an assignment, whose value C++ evaluates first.
        | Op::MemGet { .. }
        | Op::MemPut { .. } => {}
        // Into an element, a number or a string: the index is a statement of its own, written first
        // (`verification\v69_j_assign`). Through a member with an index on the way: a string runs the index first
        // too; a number is one C++ assignment, whose value C++20 evaluates first on both platforms, and the indexes
        // in the target are a group of their own (`flat_index`). A string into a variable or a member of one:
        // `qbs_set(target,value)`.
        Op::Assign { place, value } => {
            if is_qbs(value.ty) && !place.has_element() {
                parts.push(Part::Place(place));
                parts.push(expr_part(cx, value));
            }
        }
        Op::Call { args, .. } => arg_parts(cx, args, &mut parts),
        Op::Builtin { id, args } => builtin_stmt_parts(cx, *id, args, &mut parts),
        Op::Field { .. } => return OpGroup::PerCall,
        Op::Lset { target, value, .. } => {
            parts.push(Part::Place(target));
            parts.push(expr_part(cx, value));
        }
        // `ontimer_setup(number,seconds,id,arg)` and its kin.
        Op::OnEvent { event, handler } => {
            match event {
                Event::Timer { number, seconds } => {
                    parts.extend(number.iter().map(|n| expr_part(cx, n)));
                    parts.push(expr_part(cx, seconds));
                }
                Event::Key { key } => parts.push(expr_part(cx, key)),
                Event::Strig { button, controller } => {
                    parts.extend(controller.iter().map(|c| expr_part(cx, c)));
                    parts.insert(0, expr_part(cx, button));
                }
            }
            match handler {
                EventHandler::Sub { arg: Some(a), .. } => parts.push(expr_part(cx, a)),
                EventHandler::Sub { arg: None, .. } | EventHandler::Label(_) => {}
            }
        }
        Op::Key(form) => match form {
            KeyForm::Assign { number, text } => {
                parts.push(expr_part(cx, number));
                parts.push(expr_part(cx, text));
            }
            KeyForm::On | KeyForm::Off | KeyForm::List => {}
        },
        // `sub__memfill(block,offset,count,value)`: a string value is held in `tmp_long` before it.
        Op::MemFill {
            block,
            offset,
            bytes,
            value,
        } => {
            if cx.checking {
                parts.push(Part::Place(block));
            }
            parts.push(expr_part(cx, offset));
            parts.push(expr_part(cx, bytes));
            match value {
                MemValue::Typed { value, .. } => parts.push(expr_part(cx, crate::mem::stored_expr(value))),
                MemValue::Bytes(of) => parts.push(Part::Unheld(cx.reference(of))),
                MemValue::Str(_) => {}
            }
        }
    }
    if parts.len() < 2 {
        OpGroup::None
    } else {
        OpGroup::Call(parts)
    }
}

/// The parts of a built-in statement's call, by its rule.
fn builtin_stmt_parts<'a>(cx: Ctx<'_>, id: BuiltinId, args: &'a [StmtArg], parts: &mut Vec<Part<'a>>) {
    let stmt_part = |a: &'a StmtArg| match a {
        StmtArg::Value(v) => Some(expr_part(cx, v)),
        StmtArg::Place(place) | StmtArg::Bytes(place) => Some(Part::Place(place)),
        StmtArg::Block { .. } | StmtArg::Rest(_) => Some(Part::Unheld(cx.stmt_arg(a))),
        StmtArg::Word(_) | StmtArg::Absent => None,
    };
    let Some(form) = stmt_find(id) else { return };
    match form.rule {
        // Only the parts the template passes are written (`plain_stmt`).
        StmtRule::Plain | StmtRule::Graphics => {
            let template = plan(&id.get().template());
            for (part, a) in template.parts.iter().zip(args) {
                if part.passed {
                    parts.extend(stmt_part(a));
                }
            }
        }
        // The file number, the position and the variable (`file_get_put`).
        StmtRule::File => parts.extend(args.iter().filter_map(stmt_part)),
        // `swap_32(&a,&b)`; a whole `TYPE` passes the second place first.
        StmtRule::Swap => {
            if let [StmtArg::Place(a), StmtArg::Place(b)] = args {
                if let Ty::User(_) = cx.p.place_ty(a) {
                    parts.extend([Part::Place(b), Part::Place(a)]);
                } else {
                    parts.extend([Part::Place(a), Part::Place(b)]);
                }
            }
        }
        // `sub_mid(target,start,length,value,passed)`.
        StmtRule::MidAssign => parts.extend(args.iter().filter_map(stmt_part)),
        // One `sub_close` per number; the `ASC` statement's steps each a statement of their own.
        StmtRule::Close | StmtRule::AscAssign | StmtRule::Log(_) => {}
    }
}

/// The group of one part of `FIELD`: `field_add(string,width)`. The file number is a call of its own before them.
pub(crate) fn field_group<'a>(cx: Ctx<'_>, place: &'a Place, width: &'a Expr) -> Vec<Part<'a>> {
    vec![Part::Place(place), expr_part(cx, width)]
}

/// A held place: an element of an array variable by its flat position, anything else by the address of its bytes
/// (design D5).
#[derive(Clone, Debug)]
pub(crate) enum HeldPlace {
    Flat(String),
    Bytes(String),
}

/// A hold, with whether a writer found it.
#[derive(Clone, Debug)]
pub(crate) struct Hold<T> {
    pub(crate) what: T,
    used: bool,
}

impl<'a> Emitter<'a> {
    pub(crate) fn order_ctx(&self) -> Ctx<'a> {
        Ctx {
            p: self.p,
            checking: self.checking,
        }
    }

    /// A new `seqN` temporary of C type `ty`, declared with the body's other temporaries.
    fn seq_temp(&mut self, ty: &str) -> String {
        self.pass += 1;
        let name = format!("seq{}", self.pass);
        self.pass_decls.push(format!("{ty} {name};\n"));
        name
    }

    /// Plans the holds of a group and writes each held part into its temporary, in the group's order. Gives the
    /// assignments (`seqN=…`); the writer then finds the temporaries through the maps. [`Self::release_holds`] after it.
    pub(crate) fn hold(&mut self, parts: &[Part<'_>]) -> Vec<String> {
        if parts.len() < 2 {
            return Vec::new();
        }
        let cx = self.order_ctx();
        let classes: Vec<Class> = parts.iter().map(|part| part.class(cx)).collect();
        let mut out = Vec::new();
        for (part, held) in parts.iter().zip(held(&classes)) {
            if !held {
                continue;
            }
            match *part {
                Part::Expr(e) => {
                    let text = self.value(e);
                    // A string of either kind is its `qbs*`.
                    let ty = if is_qbs(e.ty) { "qbs*" } else { c_type(e.ty) };
                    let name = self.seq_temp(ty);
                    out.push(format!("{name}={text}"));
                    self.held_exprs.insert(
                        std::ptr::from_ref(e),
                        Hold {
                            what: name,
                            used: false,
                        },
                    );
                }
                Part::Place(place) => {
                    let held = match place {
                        Place::Element {
                            array: ArrayRef::Var(array),
                            index,
                        } => {
                            let flat = self.flat_index(*array, index);
                            let name = self.seq_temp("int64");
                            out.push(format!("{name}={flat}"));
                            HeldPlace::Flat(name)
                        }
                        Place::Var(_) => unreachable!("a variable is found without evaluating anything"),
                        Place::Element { .. } | Place::Member { .. } => {
                            let bytes = self.bytes_of(place);
                            let name = self.seq_temp("char*");
                            out.push(format!("{name}={bytes}"));
                            HeldPlace::Bytes(name)
                        }
                    };
                    self.held_places.insert(
                        std::ptr::from_ref(place),
                        Hold {
                            what: held,
                            used: false,
                        },
                    );
                }
                // Not held (see `Part`): it keeps the C++ compiler's order.
                Part::Unheld(_) => {}
            }
        }
        out
    }

    /// Removes the holds of a group after its writer ran. A hold no writer found would have evaluated its part
    /// twice: that is a mistake in a group's parts, and stops the compiler.
    pub(crate) fn release_holds(&mut self, parts: &[Part<'_>]) {
        for part in parts {
            let used = match *part {
                Part::Expr(e) => self.held_exprs.remove(&std::ptr::from_ref(e)).map(|h| h.used),
                Part::Place(place) => self.held_places.remove(&std::ptr::from_ref(place)).map(|h| h.used),
                Part::Unheld(_) => None,
            };
            assert!(used != Some(false), "a held part was not found by its writer: {part:?}");
        }
    }

    /// The temporary that holds `e`, when it is held.
    pub(crate) fn held_expr(&mut self, e: &Expr) -> Option<String> {
        let h = self.held_exprs.get_mut(&std::ptr::from_ref(e))?;
        h.used = true;
        Some(h.what.clone())
    }

    /// The position that holds `place`, when it is held.
    pub(crate) fn held_place(&mut self, place: &Place) -> Option<HeldPlace> {
        let h = self.held_places.get_mut(&std::ptr::from_ref(place))?;
        h.used = true;
        Some(h.what.clone())
    }

    /// `write`'s text with the holds of `parts` before it, as a comma expression at the node.
    pub(crate) fn with_holds(&mut self, parts: &[Part<'_>], write: impl FnOnce(&mut Self) -> String) -> String {
        let holds = self.hold(parts);
        let text = write(self);
        self.release_holds(parts);
        if holds.is_empty() {
            text
        } else {
            format!("({},{text})", holds.join(","))
        }
    }

    /// Texts written as parts of one unordered C++ construct (the terms of an element's flat position), each with
    /// its class and the C type of its temporary: the held ones are put into temporaries. Gives the assignments,
    /// and the text to use for each part.
    pub(crate) fn hold_texts(&mut self, parts: Vec<(Class, String, &str)>) -> (Vec<String>, Vec<String>) {
        let classes: Vec<Class> = parts.iter().map(|(c, ..)| *c).collect();
        let mut holds = Vec::new();
        let mut texts = Vec::new();
        for ((_, text, ty), held) in parts.into_iter().zip(held(&classes)) {
            if held {
                let name = self.seq_temp(ty);
                holds.push(format!("{name}={text}"));
                texts.push(name);
            } else {
                texts.push(text);
            }
        }
        (holds, texts)
    }
}

/// `text` with `holds` before it, as a comma expression.
pub(crate) fn sequenced(holds: &[String], text: String) -> String {
    if holds.is_empty() {
        text
    } else {
        format!("({},{text})", holds.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qb64rust_base::{FileId, Span};
    use qb64rust_ir::{Body, Proc, ProcId, ProcKind, Shape, Storage, Var, VarId};

    fn e(ty: Ty, kind: ExprKind) -> Expr {
        Expr {
            span: Span::new(FileId(0), 0, 0),
            ty,
            qb: ty,
            kind,
        }
    }

    fn int(i: i64) -> Expr {
        e(Ty::I64, ExprKind::Int(i))
    }

    fn load(v: u32) -> Expr {
        e(Ty::I32, ExprKind::Load(Place::Var(VarId(v))))
    }

    /// `F(n)` of the program below.
    fn call_f(n: i64) -> Expr {
        e(
            Ty::I32,
            ExprKind::CallProc {
                proc: ProcId(0),
                args: vec![Arg::Temp(e(Ty::I32, ExprKind::Int(n)), 0)],
            },
        )
    }

    fn element(array: u32, index: Expr) -> Place {
        Place::Element {
            array: ArrayRef::Var(VarId(array)),
            index: vec![index],
        }
    }

    fn bin(op: BinOp, a: Expr, b: Expr) -> Expr {
        e(
            Ty::I32,
            ExprKind::Binary {
                op,
                lhs: Box::new(a),
                rhs: Box::new(b),
            },
        )
    }

    fn builtin(name: &str, args: Vec<Option<Expr>>) -> Expr {
        let id = qb64rust_builtins::find_function(name.as_bytes()).expect("a built-in function");
        e(Ty::I32, ExprKind::Call { builtin: id, args })
    }

    /// `X`, `A(0 TO 9)`, `B(0 TO 9)`, and `FUNCTION F(N)`.
    fn program() -> Program {
        let var = |name: &str, shape: Shape, storage: Storage| Var {
            name: name.into(),
            ty: Ty::I32,
            storage,
            shape,
            conventional: false,
        };
        let array = || Shape::Static { bounds: vec![(0, 9)] };
        Program {
            vars: vec![
                var("X", Shape::Scalar, Storage::Global),
                var("A", array(), Storage::Global),
                var("B", array(), Storage::Global),
                var("N", Shape::Scalar, Storage::Param(ProcId(0))),
            ],
            procs: vec![Proc {
                name: "F".into(),
                kind: ProcKind::Function(Ty::I32),
                params: vec![VarId(3)],
                result: None,
                body: Body::default(),
                file: FileId(0),
                line: 1,
                end_file: FileId(0),
                end_line: 2,
                external: None,
            }],
            ..Program::default()
        }
    }

    fn holds(cx: Ctx<'_>, parts: &[Part<'_>]) -> Vec<bool> {
        let classes: Vec<Class> = parts.iter().map(|p| p.class(cx)).collect();
        held(&classes)
    }

    #[test]
    fn two_function_calls_hold_the_first() {
        let p = program();
        let cx = Ctx { p: &p, checking: true };
        let x = bin(BinOp::Sub, call_f(1), call_f(2));
        let group = expr_group(cx, &x);
        assert_eq!(group.len(), 2);
        assert_eq!(holds(cx, &group), [true, false]);
    }

    #[test]
    fn a_variable_before_a_function_is_held() {
        let p = program();
        let cx = Ctx { p: &p, checking: true };
        let x = bin(BinOp::Add, load(0), call_f(1));
        assert_eq!(holds(cx, &expr_group(cx, &x)), [true, false]);
        // After it, it is not: the last part is never held, and a constant does not conflict.
        let y = bin(BinOp::Add, call_f(1), load(0));
        assert_eq!(holds(cx, &expr_group(cx, &y)), [true, false]);
        let z = bin(BinOp::Add, int(1), call_f(1));
        assert_eq!(holds(cx, &expr_group(cx, &z)), [false, false]);
    }

    #[test]
    fn two_index_checks_do_not_conflict() {
        let p = program();
        let cx = Ctx { p: &p, checking: true };
        let a = e(Ty::I32, ExprKind::Load(element(1, load(0))));
        let b = e(Ty::I32, ExprKind::Load(element(2, load(0))));
        let x = bin(BinOp::Add, a.clone(), b.clone());
        assert_eq!(holds(cx, &expr_group(cx, &x)), [false, false]);
        // An index check and another error do.
        let y = bin(BinOp::Add, a, bin(BinOp::IDiv, int(1), load(0)));
        assert_eq!(holds(cx, &expr_group(cx, &y)), [true, false]);
        // Below `$CHECKING:OFF` an index raises nothing.
        let cx = Ctx { p: &p, checking: false };
        let z = bin(
            BinOp::Add,
            e(Ty::I32, ExprKind::Load(element(1, load(0)))),
            bin(BinOp::IDiv, int(1), load(0)),
        );
        assert_eq!(holds(cx, &expr_group(cx, &z)), [false, false]);
    }

    #[test]
    fn ordered_constructs_give_no_group() {
        let p = program();
        let cx = Ctx { p: &p, checking: true };
        let also = bin(BinOp::AndAlso, call_f(1), call_f(2));
        assert!(expr_group(cx, &also).is_empty());
        let or_else = bin(BinOp::OrElse, call_f(1), call_f(2));
        assert!(expr_group(cx, &or_else).is_empty());
        let choice = e(
            Ty::I32,
            ExprKind::Choice {
                condition: Box::new(load(0)),
                then: Box::new(call_f(1)),
                else_: Box::new(call_f(2)),
            },
        );
        assert!(expr_group(cx, &choice).is_empty());
    }

    #[test]
    fn group_kinds() {
        let p = program();
        let cx = Ctx { p: &p, checking: true };
        // A FUNCTION's arguments: a place by reference and a copy.
        let f = e(
            Ty::I32,
            ExprKind::CallProc {
                proc: ProcId(0),
                args: vec![Arg::Ref(element(1, call_f(1))), Arg::Temp(call_f(2), 0)],
            },
        );
        let group = expr_group(cx, &f);
        assert!(matches!(group[..], [Part::Place(_), Part::Expr(_)]));
        assert_eq!(holds(cx, &group), [true, false]);
        // A built-in's arguments, in the order of its slots.
        let mid = builtin(
            "MID",
            vec![
                Some(e(Ty::Str, ExprKind::Str(b"abc".to_vec()))),
                Some(call_f(1)),
                Some(call_f(2)),
            ],
        );
        assert_eq!(holds(cx, &expr_group(cx, &mid)), [false, true, false]);
        // A built-in that changes state against a variable.
        let rnd = builtin("RND", vec![None]);
        let x = bin(BinOp::Add, load(0), rnd);
        assert_eq!(holds(cx, &expr_group(cx, &x)), [true, false]);
        // Two string functions that cannot raise do not conflict; two that can, do.
        let s = |name: &str| {
            builtin(
                name,
                vec![Some(e(Ty::Str, ExprKind::Load(Place::Var(VarId(0))))), Some(load(0))],
            )
        };
        let cat = e(Ty::Str, ExprKind::Concat(Box::new(s("LEFT")), Box::new(s("RIGHT"))));
        assert_eq!(holds(cx, &expr_group(cx, &cat)), [false, false]);
        let chr = |v| builtin("CHR", vec![Some(v)]);
        let cat = e(
            Ty::Str,
            ExprKind::Concat(Box::new(chr(load(0))), Box::new(chr(load(0)))),
        );
        assert_eq!(holds(cx, &expr_group(cx, &cat)), [true, false]);
        // A SUB's arguments.
        let call = Op::Call {
            proc: ProcId(0),
            args: vec![
                Arg::Ref(element(1, int(9))),
                Arg::Temp(bin(BinOp::Mod, int(1), load(0)), 0),
            ],
        };
        let OpGroup::Call(group) = op_group(cx, &call) else {
            panic!("a SUB call is one call");
        };
        assert_eq!(holds(cx, &group), [true, false]);
        // A numeric store into an element: the index is a statement of its own, first; no group.
        let store = Op::Assign {
            place: element(1, call_f(1)),
            value: call_f(2),
        };
        assert!(matches!(op_group(cx, &store), OpGroup::None));
    }

    #[test]
    fn listed_names_are_built_in_functions() {
        for name in CHANGES_STATE.iter().chain(CANNOT_RAISE) {
            let bare = name.trim_end_matches('$');
            assert!(
                qb64rust_builtins::find_function(bare.as_bytes()).is_some(),
                "{name} is no built-in function of builtins.json"
            );
        }
    }
}
