//! The typed IR (design D6), and its lowering from the typed tree.
//!
//! The IR is ABI-neutral: it names no libqb or `qbx.cpp` symbol, no C type, no `passed` mask and no event loop.
//! Those are the C++ emitter's encoding of the rules stated here:
//!
//! - **A body is flat** (design D8 of `m2-control-flow-slice`): a list of statements, with labels as positions
//!   in it ([`Label`]), the program's own and those the lowering makes. Control moves between them only by
//!   [`Op::Jump`], [`Op::Branch`], [`Op::Gosub`] and [`Op::Return`], always within one body. Blocks (`IF`,
//!   `FOR`, `DO`, `WHILE`) exist only in the typed tree; the lowering turns each into statements and jumps.
//! - **Errors are pending, and handled per statement.** A raising operation records a pending error and yields a
//!   placeholder value; the statement goes on. The first error of a statement is the one serviced. A store or a
//!   call made with that value still happens ([`Op::Assign`], [`Op::AssignAll`], [`Op::Call`]; measured: `x =
//!   ASC("")` leaves 0 in `x`). A built-in statement ([`Op::Builtin`]) evaluates its arguments in order and makes
//!   its call also after a raising argument (measured, `verification\v22_a_calls`: the handler runs once, with the
//!   argument's error); the runtime's entry does nothing while an error is pending, or raises another, which
//!   changes nothing. Only these points check for a pending error:
//!   - each item of an [`Op::Print`] or an [`Op::Write`]: a raising item skips the rest of the statement, line end
//!     included; with a file number, also after the number is evaluated (a raising number writes nothing). What was
//!     written before the raising item stays written (measured, `verification\v22_b_print`);
//!   - each target of an [`Op::Input`], and after its file number: a raising read or target skips the later
//!     targets, whose fields stay unread. Each target is stored by the rule of its place; a read that raises stores
//!     zero or an empty string (measured, `verification\v22_b_input`);
//!
//!     An [`Op::Read`] has no such point (measured, `verification\v22_c_errors`): every target is read and stored
//!     in order, by the rule of its place. While an error is pending a read takes no item and gives zero, which is
//!     stored; a string target is left as it is. A read that raises (no number: error 2; outside the target's
//!     range: error 6) does not take its item either, so the next `READ` meets it again; one that finds the data
//!     used up raises error 4 and gives zero or an empty string;
//!   - a procedure's entry: a procedure entered while an error is pending returns at once;
//!   - [`Op::Jump`] and [`Op::Gosub`]: not taken while an error is pending;
//!   - [`Op::Branch`] with [`OnError::Skip`]: not taken while an error is pending;
//!   - a store into a [`Place::Element`]: its indexes are evaluated first, left to right; while an error is pending
//!     after them the value is not evaluated and nothing is stored.
//!
//!   A store into a [`Place::Member`] with an element on the way (`a(i).m`) evaluates the value first, then the
//!   indexes, and is skipped when an index is out of range or when the indexes raised an error while none was
//!   pending before them (`DIVERGENCES.md` D-004; the old compiler writes the first element). A store into a
//!   variable or into a member of one does not check. An index out of range raises error 9, and a read with it
//!   gives the value at the array's first position (measured, `verification\v18_*`).
//!
//!   [`Op::Branch`] with [`OnError::UseValue`] does not check: it tests the placeholder value (measured for
//!   `ELSEIF`).
//! - **Errors and events are serviced at the statement boundary**: a pending error goes to the active handler
//!   ([`Op::SetHandler`]). [`Resume::Retry`] re-runs the statement where the error is serviced,
//!   [`Resume::Next`] continues after it, [`Resume::To`] at a label; a statement in a procedure resumes in that
//!   procedure. A jump or branch taken leaves its statement before the boundary, so an error still pending then
//!   (only possible after a `UseValue` branch) is serviced at the boundary of the next statement that runs, and
//!   retry and resume refer to that statement (measured, `verification\v17_b_elseif`).
//!
//! With the lowering's layout ([`lower`]), this rule gives the measured behaviour of a raising block header: the
//! branch of an `IF`, `WHILE` or `DO WHILE` condition is not taken, so the body runs; the backward branch of a
//! `LOOP UNTIL` is not taken, so the loop is left; a `FOR` header stores its three limits from the placeholder
//! values ([`Op::AssignAll`]), does not take its jump to the loop's entry, and runs the body with the variable
//! unassigned.
//! - **Optional arguments are present or absent** ([`ExprKind::Call`] slots are `Option`s in table order).
//! - **Every conversion is explicit** ([`ExprKind::Convert`]); every operation states the type it computes in.
//! - **Integer overflow wraps** in two's complement (`DIVERGENCES.md` D-001, D-002).

// A new type or operator must be handled everywhere, not fall into a `_ =>` arm (study\21).
#![warn(clippy::wildcard_enum_match_arm)]

mod dump;
mod lower;
mod validate;

pub use dump::dump;
pub use lower::lower;
pub use validate::validate;

use qb64rust_base::{FileId, Span};

/// The [`Ty`] variants nothing produces yet, and those no value of reaches the IR yet, as patterns, and the reasons
/// their arms give.
pub use qb64rust_sema::{BIT_VALUE_UNREACHABLE, PLACE_ONLY_UNREACHABLE, place_only_types};
/// Types, operators and conversion kinds are `sema`'s (`study\20` §3.4): integers by width, floats by width (`F80`
/// is extended precision), strings, user types. They name no C type, so the IR stays ABI-neutral.
pub use qb64rust_sema::{BinOp, ConvKind as Conv, Member, MemberId, Ty, TypeId, UnOp, UserType};

/// An item of the program's data: its text, and whether it was written in quotes (the runtime reads a quoted
/// item's text as it is, and takes an unquoted one as a number where a number is read).
pub use qb64rust_sema::DataItem;
/// Which built-in functions are compiled and the rule each follows; the emitter writes a call by its rule.
pub use qb64rust_sema::builtins;
/// The size of a numeric or user type in memory (the layout of a `TYPE`, `LEN` of a place).
pub use qb64rust_sema::size_of;
/// Values, places and arguments are `sema`'s typed tree too (design D10 of `m2-arrays-and-types`: the lowering did
/// no work on them, only a variant-for-variant copy). The emitter ignores an [`Expr`]'s `span` and `qb`. Variable and
/// procedure ids are `sema`'s; the IR's variable list is `sema`'s with the lowering's temporaries appended
/// ([`Program::vars`]).
pub use qb64rust_sema::{Arg, Expr, ExprKind, InputSource as Source, Place, ProcId, StmtArg, VarId};

/// The table entry of a built-in: its identity in the IR (never a libqb name).
pub use qb64rust_builtins::BuiltinId;

/// A label of one body: its index in that body's [`Body::labels`]. [`Resume::To`] and [`Op::SetHandler`] name
/// labels of the main module ([`Program::main`]); the jumps name labels of their own body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LabelId(pub u32);

/// A position in a body: before statement `at` (at the end when `at` is the number of statements). Labels are
/// positions, not operations; the jumps move between them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    /// The BASIC name in upper case; `None` for a label the lowering made.
    pub name: Option<String>,
    pub line: u32,
    /// The file of a BASIC label (`None` for one the lowering made).
    pub file: Option<FileId>,
    pub at: usize,
}

/// Where a resume continues.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    /// Run the statement that raised the error again, from its start.
    Retry,
    /// Continue after the statement that raised the error.
    Next,
    /// Continue at a label of the main module.
    To(LabelId),
}

/// Where a variable lives and how long.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Storage {
    /// One for the whole program, seen by the main module (and by procedures through `SHARED`).
    Global,
    /// One for the whole program, seen by one procedure only.
    Static(ProcId),
    /// New on every call of the procedure, zero or empty at the start of the call.
    Local(ProcId),
    /// A parameter: the caller's variable or a copy made for the call ([`Arg`]). Its position is its index in
    /// [`Proc::params`].
    Param(ProcId),
    /// A FUNCTION's result: new on every call, zero or empty at the start; its value at the end is returned.
    Result(ProcId),
    /// A hidden variable the lowering made (the limits of a `FOR`), of the main module (`None`) or of a procedure.
    /// Zero at the start: once for the program in the main module, on every call in a procedure.
    Temp(Option<ProcId>),
}

/// A variable. `name` is the BASIC name in upper case without suffix; two variables may share a name when their
/// types or their storage differ. An array has `dims`, the lower and upper bound of each dimension (first
/// dimension first), known when compiling; `ty` is its element type. Only the main module has arrays so far.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
    pub name: String,
    pub ty: Ty,
    pub storage: Storage,
    /// Empty for a scalar.
    pub dims: Vec<(i64, i64)>,
}

impl Var {
    pub fn is_array(&self) -> bool {
        !self.dims.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcKind {
    Sub,
    /// A FUNCTION returning a value of this type.
    Function(Ty),
}

/// A SUB or FUNCTION.
#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
    /// The BASIC name in upper case without suffix.
    pub name: String,
    pub kind: ProcKind,
    /// One variable per parameter, in order.
    pub params: Vec<VarId>,
    /// The FUNCTION's result variable.
    pub result: Option<VarId>,
    pub body: Body,
    /// Source lines of the header and of the closing `END SUB`/`END FUNCTION`.
    pub line: u32,
    pub end_line: u32,
}

/// What the error rule and the emitter ask of a value, a place or an argument of the shared tree.
pub trait Facts {
    /// Whether evaluating it may raise a runtime error. Built-in calls and string operations always may, until
    /// built-ins carry a "cannot raise" flag (M3, `study\16` §8); an element's index check may.
    fn may_raise(&self) -> bool;
    /// Whether it involves strings (string results need temporary cleanup at the statement's end). `p` gives the
    /// types of places passed by reference.
    fn uses_strings(&self, p: &Program) -> bool;
}

impl Facts for Expr {
    fn may_raise(&self) -> bool {
        match &self.kind {
            ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) => false,
            ExprKind::Load(place) => Facts::may_raise(place),
            ExprKind::Bound { .. } => true,
            ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => from.may_raise(),
            ExprKind::Binary { op, lhs, rhs } => op_may_raise(*op) || lhs.may_raise() || rhs.may_raise(),
            ExprKind::Concat(..) | ExprKind::StrCompare { .. } | ExprKind::Call { .. } | ExprKind::CallProc { .. } => {
                true
            }
        }
    }

    fn uses_strings(&self, p: &Program) -> bool {
        self.ty == Ty::Str
            || match &self.kind {
                ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) => false,
                ExprKind::Load(place) => place.uses_strings(p),
                ExprKind::Bound { dim, .. } => dim.as_deref().is_some_and(|d| d.uses_strings(p)),
                ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => from.uses_strings(p),
                ExprKind::Binary { lhs, rhs, .. }
                | ExprKind::Concat(lhs, rhs)
                | ExprKind::StrCompare { lhs, rhs, .. } => lhs.uses_strings(p) || rhs.uses_strings(p),
                ExprKind::Call { args, .. } => args.iter().flatten().any(|a| a.uses_strings(p)),
                ExprKind::CallProc { args, .. } => args.iter().any(|a| a.uses_strings(p)),
            }
    }
}

impl Facts for Place {
    /// An element's index may raise error 9; a member's base may be an element.
    fn may_raise(&self) -> bool {
        Place::may_raise(self)
    }

    fn uses_strings(&self, p: &Program) -> bool {
        self.indexes().iter().any(|v| v.uses_strings(p))
    }
}

impl Facts for Arg {
    /// Finding a place passed by reference may raise (its index); a copy as its value does.
    fn may_raise(&self) -> bool {
        match self {
            Arg::Ref(place) => Facts::may_raise(place),
            Arg::Temp(v) => v.may_raise(),
        }
    }

    /// A string copy (a temporary), a place whose index involves strings, or a `STRING * n` element or member,
    /// which is passed as a temporary fixed `qbs` over its bytes (`qbs_new_fixed(…,n,1)`, design D6).
    fn uses_strings(&self, p: &Program) -> bool {
        match self {
            Arg::Ref(place) => {
                place.uses_strings(p)
                    || (!matches!(place, Place::Var(_)) && matches!(p.place_ty(place), Ty::FixedStr(_)))
            }
            Arg::Temp(v) => v.uses_strings(p),
        }
    }
}

impl Facts for StmtArg {
    fn may_raise(&self) -> bool {
        match self {
            StmtArg::Value(v) => v.may_raise(),
            StmtArg::Place(place) => Facts::may_raise(place),
            StmtArg::Word(_) | StmtArg::Absent => false,
        }
    }

    /// A string value, or a place that is a string or whose index involves strings.
    fn uses_strings(&self, p: &Program) -> bool {
        match self {
            StmtArg::Value(v) => v.uses_strings(p),
            StmtArg::Place(place) => place.uses_strings(p) || p.place_ty(place).is_string(),
            StmtArg::Word(_) | StmtArg::Absent => false,
        }
    }
}

/// Whether the operator itself may raise: `\` and `MOD` by 0 (error 11), `^` with a negative base and a non-integer
/// exponent (error 5).
fn op_may_raise(op: BinOp) -> bool {
    match op {
        BinOp::IDiv | BinOp::Mod | BinOp::Pow => true,
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
        | BinOp::OrElse => false,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrintItem {
    Str(Expr),
    /// Printed as `STR$` of the value plus one space.
    Num(Expr),
    /// Move to the next print zone.
    Zone,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// Send output to (and read input from) the console instead of a window.
    SelectConsole,
    /// A store, by the rule of its place (see the crate documentation).
    Assign {
        place: Place,
        value: Expr,
    },
    /// Items in order, to the console (`to` is `None`) or to the file with this number (an `I32`); a raising item
    /// skips the rest, including the line end. In a file a [`PrintItem::Zone`] is 14 columns wide, on the console
    /// 10 (the runtime's).
    Print {
        to: Option<Expr>,
        items: Vec<PrintItem>,
        newline: bool,
    },
    /// The items ([`PrintItem::Str`] in double quotes, [`PrintItem::Num`] without blanks) separated by commas, to
    /// the console or to a file, then a line end if `newline`; without it the last item is followed by a comma
    /// too. A raising item skips the rest.
    Write {
        to: Option<Expr>,
        items: Vec<PrintItem>,
        newline: bool,
    },
    /// Read one field per target, or (`line`) one whole line into the single string target, from a file or the
    /// console. A numeric target takes the field's value in its own type; a value outside the type's range raises
    /// error 6. The targets are stored in order, each by the rule of its place; see the crate documentation for the
    /// pending-error checks. At least one target; none is of a user type or a member of an element.
    Input {
        from: Source,
        line: bool,
        targets: Vec<Place>,
    },
    /// Read the next item of the program's data ([`Program::data`]) into each target in turn, each stored by the
    /// rule of its place. A numeric target takes the item's value in its own type. No point of it checks for a
    /// pending error: see the crate documentation. At least one target; none is of a user type or a member of an
    /// element.
    Read(Vec<Place>),
    /// Make the next [`Op::Read`] start at item `at` of [`Program::data`] (at its end: nothing is left to read).
    /// `label` is the BASIC label, in upper case, that gave the position; `None` for the start of the data (`at`
    /// is 0). The emitter may name the position after it.
    Restore {
        at: usize,
        label: Option<String>,
    },
    End,
    /// End the program at once (`SYSTEM`).
    System,
    /// A SUB call; one argument per parameter. Always may raise.
    Call {
        proc: ProcId,
        args: Vec<Arg>,
    },
    /// A built-in statement that is one call of the runtime (design D2 of `m2-builtin-statements`): the built-in by
    /// its table entry, which also says which form was written, and one slot per argument or choice of its template
    /// in template order ([`builtins::stmt_slots`]). The values are evaluated in order; the call is made also after
    /// a raising one. Always may raise.
    Builtin {
        id: BuiltinId,
        args: Vec<StmtArg>,
    },
    /// Leave the procedure (`EXIT SUB`, `EXIT FUNCTION`); a FUNCTION returns its result variable's value.
    Exit,
    /// Make the statements from a label the program's error handler, or (`None`) remove it. One handler for the
    /// whole program, also when set inside a procedure.
    SetHandler(Option<LabelId>),
    /// Raise the runtime error with this number (an `I32`).
    Raise(Expr),
    /// End the running handler and continue as stated; outside a handler it raises error 20.
    Resume(Resume),
    /// Continue at a label of this body; not taken while an error is pending.
    Jump(LabelId),
    /// Continue at a label of this body when `cond` (a number) is zero or non-zero as `when` says; otherwise go on
    /// with the next operation. `on_error` says what a pending error does.
    Branch {
        cond: Expr,
        when: When,
        to: LabelId,
        on_error: OnError,
    },
    /// Evaluate and store each value in order, every store made also after a raising value; then the statement
    /// rule as for [`Op::Assign`]. (A `FOR` header: all its limits are stored before an error is serviced.)
    AssignAll(Vec<(VarId, Expr)>),
    /// Continue at a label of this body; a [`Op::Return`] without label comes back after this statement. One
    /// stack of pending `GOSUB`s for the whole program. Not taken while an error is pending.
    Gosub(LabelId),
    /// Back to after the last pending [`Op::Gosub`], also one made in another body (`None`), or forget it and
    /// continue at a label of the main module (`Some`). Raises error 3 when no `GOSUB` is pending.
    Return(Option<LabelId>),
}

impl Op {
    /// Whether this operation may raise a runtime error.
    pub fn may_raise(&self) -> bool {
        match self {
            Op::SelectConsole
            | Op::End
            | Op::System
            | Op::Exit
            | Op::SetHandler(_)
            | Op::Jump(_)
            | Op::Gosub(_)
            | Op::Restore { .. } => false,
            // `RESUME` outside a handler raises error 20, `RETURN` with no `GOSUB` pending error 3.
            Op::Call { .. } | Op::Builtin { .. } | Op::Raise(_) | Op::Resume(_) | Op::Return(_) => true,
            Op::Assign { place, value } => place.may_raise() || value.may_raise(),
            Op::AssignAll(stores) => stores.iter().any(|(_, v)| v.may_raise()),
            Op::Branch { cond, .. } => cond.may_raise(),
            // A file may not be open, a console input may meet the end of the input, the data may be used up.
            Op::Print { to: Some(_), .. } | Op::Write { to: Some(_), .. } | Op::Input { .. } | Op::Read(_) => true,
            Op::Print { to: None, items, .. } | Op::Write { to: None, items, .. } => items.iter().any(|i| match i {
                PrintItem::Str(v) | PrintItem::Num(v) => v.may_raise(),
                PrintItem::Zone => false,
            }),
        }
    }
}

/// When an [`Op::Branch`] is taken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum When {
    Zero,
    NonZero,
}

/// What a pending error does to an [`Op::Branch`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnError {
    /// Not taken while an error is pending (`IF`, `WHILE`, `DO`, `LOOP` conditions).
    Skip,
    /// Taken or not by the value, which after a raising condition is the placeholder (`ELSEIF`, measured).
    UseValue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub span: Span,
    pub line: u32,
    pub ops: Vec<Op>,
    pub may_raise: bool,
}

/// A sequence of statements: the main module's, or a procedure's.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Body {
    /// The body's own labels in source order, then the labels the lowering made.
    pub labels: Vec<Label>,
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program {
    /// Every variable, of every storage class.
    pub vars: Vec<Var>,
    /// Procedures in definition order.
    pub procs: Vec<Proc>,
    pub main: Body,
    /// The user types; a type's layout is the emitter's.
    pub types: Vec<UserType>,
    /// The program's data: the items of every `DATA` statement, in the order [`Op::Read`] takes them. How they are
    /// stored in the executable is the emitter's.
    pub data: Vec<DataItem>,
}

impl Program {
    pub fn user_type(&self, id: TypeId) -> &UserType {
        &self.types[id.0 as usize]
    }

    /// The member of a place of user type `base`.
    pub fn member(&self, base: Ty, m: MemberId) -> &Member {
        let Ty::User(t) = base else {
            panic!("a member of a place of type {base:?}");
        };
        &self.user_type(t).members[m.0 as usize]
    }

    /// The type of a place.
    pub fn place_ty(&self, p: &Place) -> Ty {
        match p {
            Place::Var(v) | Place::Element { array: v, .. } => self.var(*v).ty,
            Place::Member { base, member } => self.member(self.place_ty(base), *member).ty,
        }
    }

    pub fn var(&self, id: VarId) -> &Var {
        &self.vars[id.0 as usize]
    }

    pub fn proc(&self, id: ProcId) -> &Proc {
        &self.procs[id.0 as usize]
    }
}
