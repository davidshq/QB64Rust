//! The typed IR (design D6), and its lowering from the typed tree.
//!
//! The IR is ABI-neutral: it names no libqb or `qbx.cpp` symbol, no C type, no `passed` mask and no event loop.
//! Those are the C++ emitter's encoding of the rules stated here:
//!
//! - **Errors are handled per statement.** An operation marked as possibly raising a runtime error is followed by
//!   an implicit check; on error the rest of its statement is skipped. Errors and events are serviced at the
//!   statement boundary (where `RESUME` re-runs and `RESUME NEXT` continues after the statement).
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

/// The IR's own types: integers by width, floats by width (`F80` is extended precision), strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    I16,
    I32,
    I64,
    F32,
    F64,
    F80,
    Str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VarId(pub u32);

/// A main-module variable. `name` is the BASIC name in upper case without suffix; two variables may share a name
/// when their types differ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
    pub name: String,
    pub ty: Ty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conv {
    /// Exact widening (integer or float).
    Widen,
    /// Integer narrowing: keep the low bits.
    Truncate,
    /// Float to integer, half to even.
    RoundEven,
    /// To the nearest float value.
    Nearest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
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
}

impl Value {
    /// Whether evaluating this value may raise a runtime error. Built-in calls and string operations always may,
    /// until built-ins carry a "cannot raise" flag (M3, `study\16` §8).
    pub fn may_raise(&self) -> bool {
        match &self.kind {
            ValueKind::Const(_) | ValueKind::Var(_) => false,
            ValueKind::Convert { from, .. } | ValueKind::Neg(from) => from.may_raise(),
            ValueKind::Binary { lhs, rhs, .. } => lhs.may_raise() || rhs.may_raise(),
            ValueKind::Concat(..) | ValueKind::CallBuiltin { .. } => true,
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub span: Span,
    pub line: u32,
    pub ops: Vec<Op>,
    pub may_raise: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program {
    pub vars: Vec<Var>,
    pub main: Vec<Stmt>,
}

impl Program {
    pub fn var(&self, id: VarId) -> &Var {
        &self.vars[id.0 as usize]
    }
}
