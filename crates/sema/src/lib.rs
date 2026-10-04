//! Resolution, typing and constant folding (design D5). Produces the typed tree: every expression has a type,
//! every implicit conversion is an explicit [`ExprKind::Convert`] node, every operator carries the type it is
//! computed in.
//!
//! Each expression has two types. `ty` is the type its value is computed and held in (what the generated code
//! actually does); `qb` is the type the old compiler believes it has, which decides how a `PRINT` formats it.
//! They differ in three measured cases (`study\02` §1.4): an integer operation is computed in 32 bits unless an
//! operand is `_INTEGER64` but believed `_INTEGER64`; a SINGLE literal is held as a DOUBLE; and a float operation
//! with a SINGLE literal operand is computed in DOUBLE but believed SINGLE.

// A new type or operator must be handled everywhere, not fall into a `_ =>` arm (study\21).
#![warn(clippy::wildcard_enum_match_arm)]

mod check;
mod dump;
pub mod literal;
mod symbols;

pub use check::{check, check_with};
pub use dump::dump_typed;
pub use symbols::{Symbol, SymbolId, SymbolKind, Symbols, dump_symbols};

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
        match self {
            Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 => true,
            Ty::Str => false,
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProcId(pub u32);

/// Where a variable lives (design D2, D6 of `m2-procedures-and-errors`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Storage {
    /// A main-module variable (also when a procedure names it with `SHARED`, or sees it through `DIM SHARED`).
    Main,
    /// `STATIC` in a procedure: one for the program, keeps its value between calls.
    Static(ProcId),
    /// `DIM` or implicit in a procedure: new on every call.
    Local(ProcId),
    /// A parameter of the procedure.
    Param(ProcId),
    /// The result of a FUNCTION.
    Result(ProcId),
}

/// A variable: a name plus a type (design D5), and its storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
    /// The name without suffix, in upper case.
    pub name: String,
    pub ty: Ty,
    pub storage: Storage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcKind {
    Sub,
    /// A FUNCTION with its result type (its suffix, or SINGLE).
    Function(Ty),
}

/// A SUB or FUNCTION.
#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
    /// The name without suffix, in upper case.
    pub name: String,
    pub kind: ProcKind,
    /// One variable per parameter, in order.
    pub params: Vec<VarId>,
    /// The FUNCTION's result variable.
    pub result: Option<VarId>,
    pub stmts: Vec<Stmt>,
}

/// How an argument is passed to a procedure (design D4).
#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    /// The variable itself: the procedure's assignments to the parameter change it.
    Ref(VarId),
    /// A fresh copy of a value, already converted to the parameter's type; changes to it are lost.
    Temp(Expr),
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
    /// A FUNCTION call; one argument per parameter. Its `ty` and `qb` are the function's type (measured: printed
    /// with the function's type, not as an integer operation).
    CallProc {
        proc: ProcId,
        args: Vec<Arg>,
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
    /// A SUB call; one argument per parameter.
    Call {
        proc: ProcId,
        args: Vec<Arg>,
    },
    /// `EXIT SUB` / `EXIT FUNCTION`: leaves the procedure (either word leaves either kind, measured).
    Exit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub span: Span,
    /// 1-based source line of the statement's first token.
    pub line: u32,
    pub kind: StmtKind,
}

/// The typed program.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program {
    /// Every variable of the program, of every storage class.
    pub vars: Vec<Var>,
    /// Procedures in definition order.
    pub procs: Vec<Proc>,
    /// The main module's statements.
    pub stmts: Vec<Stmt>,
    /// Where each variable and procedure is defined and used (design D12).
    pub symbols: Symbols,
}

impl Program {
    pub fn var(&self, id: VarId) -> &Var {
        &self.vars[id.0 as usize]
    }

    pub fn proc(&self, id: ProcId) -> &Proc {
        &self.procs[id.0 as usize]
    }
}
