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
//!   placeholder value; the statement goes on. A store or a call made with that value still happens
//!   ([`Op::Assign`], [`Op::AssignAll`], [`Op::Call`]; measured: `x = ASC("")` leaves 0 in `x`). Only these
//!   points check for a pending error:
//!   - each item of an [`Op::Print`]: a raising item skips the rest of the statement, line end included;
//!   - a procedure's entry: a procedure entered while an error is pending returns at once;
//!   - [`Op::Jump`] and [`Op::Gosub`]: not taken while an error is pending;
//!   - [`Op::Branch`] with [`OnError::Skip`]: not taken while an error is pending.
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
//! - **Optional arguments are present or absent** ([`Value::CallBuiltin`] slots are `Option`s in table order).
//! - **Every conversion is explicit** ([`ValueKind::Convert`]); every operation states the type it computes in.
//! - **Integer overflow wraps** in two's complement (`DIVERGENCES.md` D-001, D-002).

// A new type or operator must be handled everywhere, not fall into a `_ =>` arm (study\21).
#![warn(clippy::wildcard_enum_match_arm)]

mod dump;
mod lower;

pub use dump::dump;
pub use lower::lower;

use qb64rust_base::Span;
use qb64rust_builtins::BuiltinId;

/// Types, operators and conversion kinds are `sema`'s (`study\20` §3.4): integers by width, floats by width (`F80`
/// is extended precision), strings. They name no C type, so the IR stays ABI-neutral.
pub use qb64rust_sema::{BinOp, ConvKind as Conv, Ty, UnOp};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VarId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProcId(pub u32);

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
/// types or their storage differ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
    pub name: String,
    pub ty: Ty,
    pub storage: Storage,
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

/// How an argument reaches a parameter (design D4 of `m2-procedures-and-errors`).
#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    /// The variable itself: assignments to the parameter change it.
    Ref(VarId),
    /// A fresh copy of the value, which already has the parameter's type; changes to it are lost (no copy-back).
    Temp(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Int(i64),
    /// Decimal text (`1.5E+2`); the value is the nearest value of the constant's type. (A SINGLE literal is held
    /// as `F64`, the nearest double, as in the old compiler.)
    Float(String),
    Str(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub ty: Ty,
    pub kind: ValueKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueKind {
    Const(Const),
    Var(VarId),
    Convert {
        how: Conv,
        from: Box<Value>,
    },
    /// Both operands have the same type: the value's, except for comparisons, `_ANDALSO` and `_ORELSE`, whose
    /// operands share a type of their own and whose value is LONG. Integer overflow wraps. `_ANDALSO` and `_ORELSE`
    /// evaluate the right operand only when the left one does not decide.
    Binary {
        op: BinOp,
        lhs: Box<Value>,
        rhs: Box<Value>,
    },
    /// The operand has the value's type, except for `_NEGATE`, whose value is LONG.
    Unary {
        op: UnOp,
        operand: Box<Value>,
    },
    Concat(Box<Value>, Box<Value>),
    /// A comparison of two strings, byte by byte (`op` is one of the six comparisons): -1 or 0, LONG.
    StrCompare {
        op: BinOp,
        lhs: Box<Value>,
        rhs: Box<Value>,
    },
    /// `None` = optional argument absent.
    CallBuiltin {
        id: BuiltinId,
        args: Vec<Option<Value>>,
    },
    /// A FUNCTION call; one argument per parameter. Always may raise.
    CallProc {
        proc: ProcId,
        args: Vec<Arg>,
    },
}

impl Value {
    /// Whether evaluating this value may raise a runtime error. Built-in calls and string operations always may,
    /// until built-ins carry a "cannot raise" flag (M3, `study\16` §8).
    pub fn may_raise(&self) -> bool {
        match &self.kind {
            ValueKind::Const(_) | ValueKind::Var(_) => false,
            ValueKind::Convert { from, .. } | ValueKind::Unary { operand: from, .. } => from.may_raise(),
            ValueKind::Binary { op, lhs, rhs } => op_may_raise(*op) || lhs.may_raise() || rhs.may_raise(),
            ValueKind::Concat(..)
            | ValueKind::StrCompare { .. }
            | ValueKind::CallBuiltin { .. }
            | ValueKind::CallProc { .. } => true,
        }
    }

    /// Whether this value involves strings (string results need temporary cleanup at the statement's end).
    pub fn uses_strings(&self) -> bool {
        self.ty == Ty::Str
            || match &self.kind {
                ValueKind::Const(_) | ValueKind::Var(_) => false,
                ValueKind::Convert { from, .. } | ValueKind::Unary { operand: from, .. } => from.uses_strings(),
                ValueKind::Binary { lhs, rhs, .. }
                | ValueKind::Concat(lhs, rhs)
                | ValueKind::StrCompare { lhs, rhs, .. } => lhs.uses_strings() || rhs.uses_strings(),
                ValueKind::CallBuiltin { args, .. } => args.iter().flatten().any(Value::uses_strings),
                ValueKind::CallProc { args, .. } => args.iter().any(Arg::uses_strings),
            }
    }
}

impl Arg {
    /// Whether passing this argument involves strings: a string copy (a temporary), or a string variable.
    pub fn uses_strings(&self) -> bool {
        match self {
            Arg::Ref(_) => false,
            Arg::Temp(v) => v.uses_strings(),
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
    Str(Value),
    /// Printed as `STR$` of the value plus one space.
    Num(Value),
    /// Move to the next print zone.
    Zone,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// Send output to (and read input from) the console instead of a window.
    SelectConsole,
    Assign {
        place: VarId,
        value: Value,
    },
    /// Items in order; a raising item skips the rest, including the line end.
    Print {
        items: Vec<PrintItem>,
        newline: bool,
    },
    End,
    /// End the program at once (`SYSTEM`).
    System,
    /// A SUB call; one argument per parameter. Always may raise.
    Call {
        proc: ProcId,
        args: Vec<Arg>,
    },
    /// Leave the procedure (`EXIT SUB`, `EXIT FUNCTION`); a FUNCTION returns its result variable's value.
    Exit,
    /// Make the statements from a label the program's error handler, or (`None`) remove it. One handler for the
    /// whole program, also when set inside a procedure.
    SetHandler(Option<LabelId>),
    /// Raise the runtime error with this number (an `I32`).
    Raise(Value),
    /// End the running handler and continue as stated; outside a handler it raises error 20.
    Resume(Resume),
    /// Continue at a label of this body; not taken while an error is pending.
    Jump(LabelId),
    /// Continue at a label of this body when `cond` (a number) is zero or non-zero as `when` says; otherwise go on
    /// with the next operation. `on_error` says what a pending error does.
    Branch {
        cond: Value,
        when: When,
        to: LabelId,
        on_error: OnError,
    },
    /// Evaluate and store each value in order, every store made also after a raising value; then the statement
    /// rule as for [`Op::Assign`]. (A `FOR` header: all its limits are stored before an error is serviced.)
    AssignAll(Vec<(VarId, Value)>),
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
            Op::SelectConsole | Op::End | Op::System | Op::Exit | Op::SetHandler(_) | Op::Jump(_) | Op::Gosub(_) => {
                false
            }
            // `RESUME` outside a handler raises error 20, `RETURN` with no `GOSUB` pending error 3.
            Op::Call { .. } | Op::Raise(_) | Op::Resume(_) | Op::Return(_) => true,
            Op::Assign { value, .. } => value.may_raise(),
            Op::AssignAll(stores) => stores.iter().any(|(_, v)| v.may_raise()),
            Op::Branch { cond, .. } => cond.may_raise(),
            Op::Print { items, .. } => items.iter().any(|i| match i {
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
}

impl Program {
    pub fn var(&self, id: VarId) -> &Var {
        &self.vars[id.0 as usize]
    }

    pub fn proc(&self, id: ProcId) -> &Proc {
        &self.procs[id.0 as usize]
    }
}
