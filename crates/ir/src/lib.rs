//! The typed IR (design D6), and its lowering from the typed tree.
//!
//! The IR is ABI-neutral: it names no libqb or `qbx.cpp` symbol, no C type, no `passed` mask and no event loop.
//! Those are the C++ emitter's encoding of the rules stated here:
//!
//! - **Errors are handled per statement.** An operation marked as possibly raising a runtime error is followed by
//!   an implicit check; on error the rest of its statement is skipped. Errors and events are serviced at the
//!   statement boundary: a pending error goes to the active handler ([`Op::SetHandler`]). [`Resume::Retry`]
//!   re-runs the statement that raised, [`Resume::Next`] continues after it, [`Resume::To`] at a label; a
//!   statement in a procedure resumes in that procedure.
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

/// Types, binary operators and conversion kinds are `sema`'s (`study\20` §3.4): integers by width, floats by
/// width (`F80` is extended precision), strings. They name no C type, so the IR stays ABI-neutral.
pub use qb64rust_sema::{BinOp, ConvKind as Conv, Ty};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VarId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProcId(pub u32);

/// A label of the main module's body ([`Program::main`]); procedures have none yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LabelId(pub u32);

/// A position in a body: before statement `at` (at the end when `at` is the number of statements). Labels are
/// positions, not operations, so the IR has no jumps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    /// The BASIC name in upper case.
    pub name: String,
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
    /// Both operands have the value's type; integer overflow wraps.
    Binary {
        op: BinOp,
        lhs: Box<Value>,
        rhs: Box<Value>,
    },
    Neg(Box<Value>),
    Concat(Box<Value>, Box<Value>),
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
            ValueKind::Convert { from, .. } | ValueKind::Neg(from) => from.may_raise(),
            ValueKind::Binary { lhs, rhs, .. } => lhs.may_raise() || rhs.may_raise(),
            ValueKind::Concat(..) | ValueKind::CallBuiltin { .. } | ValueKind::CallProc { .. } => true,
        }
    }

    /// Whether this value involves strings (string results need temporary cleanup at the statement's end).
    pub fn uses_strings(&self) -> bool {
        self.ty == Ty::Str
            || match &self.kind {
                ValueKind::Const(_) | ValueKind::Var(_) => false,
                ValueKind::Convert { from, .. } | ValueKind::Neg(from) => from.uses_strings(),
                ValueKind::Binary { lhs, rhs, .. } | ValueKind::Concat(lhs, rhs) => {
                    lhs.uses_strings() || rhs.uses_strings()
                }
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
    /// In source order; only the main module has labels.
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
