//! Resolution, typing and constant folding (design D5). Produces the typed tree: every expression has a type,
//! every implicit conversion is an explicit [`ExprKind::Convert`] node, every operator carries the type it is
//! computed in.
//!
//! Each expression has two types. `ty` is the type its value is computed and held in (what the generated code
//! actually does); `qb` is the type the old compiler believes it has, which decides how a `PRINT` formats it.
//! They differ in three measured cases (`study\02` §1.4): an integer operation is computed in 32 bits unless an
//! operand is `_INTEGER64` but believed `_INTEGER64`; a SINGLE literal is held as a DOUBLE; and a float operation
//! with a SINGLE literal operand is computed in DOUBLE but believed SINGLE.

mod check;
mod dump;
pub mod literal;

pub use check::{check, check_with};
pub use dump::dump_typed;

use qb64rust_base::Span;
use qb64rust_builtins::BuiltinId;

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

impl Ty {
    pub fn is_int(self) -> bool {
        matches!(self, Ty::I16 | Ty::I32 | Ty::I64)
    }

    pub fn is_float(self) -> bool {
        matches!(self, Ty::F32 | Ty::F64 | Ty::F80)
    }

    pub fn is_numeric(self) -> bool {
        self != Ty::Str
    }

    /// The QB type name (`INTEGER`, `_FLOAT`...).
    pub fn qb_name(self) -> &'static str {
        match self {
            Ty::I16 => "INTEGER",
            Ty::I32 => "LONG",
            Ty::I64 => "_INTEGER64",
            Ty::F32 => "SINGLE",
            Ty::F64 => "DOUBLE",
            Ty::F80 => "_FLOAT",
            Ty::Str => "STRING",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VarId(pub u32);

/// A main-module variable: a name plus a type (design D5).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
    /// The name without suffix, in upper case.
    pub name: String,
    pub ty: Ty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvKind {
    /// Exact: integer to a wider integer, float to a wider float.
    Widen,
    /// Integer to a narrower integer: keeps the low bits, no error.
    Truncate,
    /// Float to integer, half to even. To LONG (32 bits) only from SINGLE; otherwise to `_INTEGER64`.
    RoundEven,
    /// To the nearest value of a float type: integer to float, or float to a narrower float.
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
pub struct Expr {
    pub span: Span,
    /// The type the value is computed and held in.
    pub ty: Ty,
    /// The type the old compiler believes the value has.
    pub qb: Ty,
    pub kind: ExprKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Int(i64),
    /// Decimal text in C form, see [`literal::NumLit::Float`].
    Float(String),
    Str(Vec<u8>),
    Var(VarId),
    Convert {
        how: ConvKind,
        from: Box<Expr>,
    },
    /// Both operands have the node's `ty`. Integer overflow wraps.
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// The operand has the node's `ty`.
    Neg(Box<Expr>),
    Concat(Box<Expr>, Box<Expr>),
    /// A built-in function call; one slot per table argument, `None` for an absent optional argument. Each
    /// present argument is already converted to the slot's type.
    Call {
        builtin: BuiltinId,
        args: Vec<Option<Expr>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrintItem {
    /// A string expression.
    Str(Expr),
    /// A numeric expression, converted to its believed type: printed as `STR$` and one space.
    Num(Expr),
    /// `,`: move to the next print zone.
    Zone,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    /// `$CONSOLE:ONLY`: output and input go to the console.
    ConsoleOnly,
    Assign {
        var: VarId,
        value: Expr,
    },
    Print {
        items: Vec<PrintItem>,
        newline: bool,
    },
    End,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub span: Span,
    /// 1-based source line of the statement's first token.
    pub line: u32,
    pub kind: StmtKind,
}

/// The typed main module.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program {
    pub vars: Vec<Var>,
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn var(&self, id: VarId) -> &Var {
        &self.vars[id.0 as usize]
    }
}
