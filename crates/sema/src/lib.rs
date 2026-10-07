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
pub mod consteval;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LabelId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstId(pub u32);

/// A `CONST` (design D6 of `m2-control-flow-slice`). Its uses are literal nodes in the typed tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Const {
    /// The name without suffix, in upper case.
    pub name: String,
    pub ty: Ty,
    pub value: consteval::Value,
    /// The procedure the constant belongs to; `None` for the main module's.
    pub proc: Option<ProcId>,
}

/// A label (design D5 of `m2-control-flow-slice`). It belongs to one body, the main module or a procedure, also
/// when it stands inside a block; where it stands is its [`StmtKind::Label`] statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    /// The name in upper case.
    pub name: String,
    /// 1-based source line of the label.
    pub line: u32,
    /// The procedure whose body holds the label; `None` for the main module.
    pub proc: Option<ProcId>,
}

/// Where `RESUME` continues (spec `language/error-handling`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    /// `RESUME` / `RESUME 0`: run the statement that raised the error again.
    Retry,
    /// `RESUME NEXT`: continue after the statement that raised the error.
    Next,
    /// `RESUME label`.
    To(LabelId),
}

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
    /// 1-based source lines of the header and of the closing `END SUB`/`END FUNCTION`.
    pub line: u32,
    pub end_line: u32,
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
    /// `=`, `<>`, `<`, `>`, `<=`, `>=`: -1 or 0, typed LONG.
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    /// Bit by bit on integers.
    And,
    Or,
    Xor,
    Eqv,
    Imp,
    /// `_ANDALSO`, `_ORELSE`: -1 or 0; the right operand is evaluated only when the left one does not decide.
    AndAlso,
    OrElse,
    /// `\`: integer division truncating toward zero; a divisor of 0 raises error 11.
    IDiv,
    /// `MOD`: remainder with the sign of the dividend; a divisor of 0 raises error 11.
    Mod,
    /// `^`: computed in `_FLOAT`; a negative base with a non-integer exponent raises error 5.
    Pow,
}

impl BinOp {
    /// The six comparisons.
    pub fn is_comparison(self) -> bool {
        match self {
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => true,
            BinOp::Add
            | BinOp::Sub
            | BinOp::Mul
            | BinOp::Div
            | BinOp::And
            | BinOp::Or
            | BinOp::Xor
            | BinOp::Eqv
            | BinOp::Imp
            | BinOp::AndAlso
            | BinOp::OrElse
            | BinOp::IDiv
            | BinOp::Mod
            | BinOp::Pow => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    /// Unary minus.
    Neg,
    /// `NOT`: bit by bit.
    Not,
    /// `_NEGATE`: -1 for 0, else 0; typed LONG.
    Negate,
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
    /// Both operands have the same type: the node's `ty`, except for comparisons, `_ANDALSO` and `_ORELSE`, whose
    /// operands share a type of their own and whose node is LONG. Integer overflow wraps.
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// The operand has the node's `ty`, except for `_NEGATE`, whose node is LONG.
    Unary {
        op: UnOp,
        operand: Box<Expr>,
    },
    Concat(Box<Expr>, Box<Expr>),
    /// A comparison (`op` is one of the six) of two strings, byte by byte: -1 or 0, typed LONG.
    StrCompare {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
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
    /// `SYSTEM`: ends the program at once, without the "press any key" prompt of `END`.
    System,
    /// A SUB call; one argument per parameter.
    Call {
        proc: ProcId,
        args: Vec<Arg>,
    },
    /// `EXIT SUB` / `EXIT FUNCTION`: leaves the procedure (either word leaves either kind, measured).
    Exit,
    /// `ON ERROR GOTO label` (`Some`) or `ON ERROR GOTO 0` (`None`): sets or removes the program's error handler.
    OnError(Option<LabelId>),
    /// `RESUME ...`: ends the running error handler.
    Resume(Resume),
    /// `ERROR n`: raises error `n`, already converted to LONG (rounded half to even).
    Error(Expr),
    /// Where a label stands: before the next statement of its body.
    Label(LabelId),
    /// `GOTO label`: a label of the same body.
    Goto(LabelId),
    /// `GOSUB label`: a label of the same body; `RETURN` comes back after this statement.
    Gosub(LabelId),
    /// `RETURN` (`None`): back to after the last `GOSUB`, also one made in another body (one stack for the program,
    /// measured). `RETURN label` (main module only): forget the last `GOSUB` and continue at the label. Either
    /// raises error 3 when no `GOSUB` is pending.
    Return(Option<LabelId>),
    /// `IF` (block or single line, which the old compiler also turns into a block): the first branch is the `IF`,
    /// the others are `ELSEIF`s, tried in order; `else_` runs when no condition is true. Conditions are numeric;
    /// true is non-zero.
    If {
        branches: Vec<Branch>,
        else_: Option<Vec<Stmt>>,
        /// 1-based source line of the `END IF`; the statement's own line for a single-line `IF`.
        end_line: u32,
    },
    /// `FOR var = start TO end [STEP step]` … `NEXT`. `start`, `end` and `step` are converted to `temp`, the type
    /// the loop counts in (wider than `var`'s, `study\02` §6.5, measured); each pass stores the count into `var`.
    For {
        var: VarId,
        temp: Ty,
        start: Expr,
        end: Expr,
        /// `None` without `STEP` (a step of 1).
        step: Option<Expr>,
        body: Vec<Stmt>,
        /// 1-based source line of the `NEXT` that closes the loop.
        end_line: u32,
    },
    /// `DO` … `LOOP`, with at most one condition, at the top or at the bottom.
    Do {
        test: Option<LoopTest>,
        body: Vec<Stmt>,
        /// 1-based source line of the `LOOP`.
        end_line: u32,
    },
    /// `WHILE cond` … `WEND`.
    While {
        cond: Expr,
        body: Vec<Stmt>,
        /// 1-based source line of the `WEND`.
        end_line: u32,
    },
    /// `EXIT FOR`, `EXIT DO`, `EXIT WHILE`: leaves the innermost loop of that kind (the parser checked there is
    /// one).
    ExitLoop(LoopKind),
}

/// One branch of an [`StmtKind::If`]: its condition and its statements.
#[derive(Clone, Debug, PartialEq)]
pub struct Branch {
    pub cond: Expr,
    pub body: Vec<Stmt>,
    /// 1-based source line of the `IF` or `ELSEIF`.
    pub line: u32,
}

/// The condition of a `DO` loop.
#[derive(Clone, Debug, PartialEq)]
pub struct LoopTest {
    /// At `DO` (checked before each pass) or at `LOOP` (after each pass).
    pub at: TestAt,
    /// `UNTIL` (leave when true) rather than `WHILE` (leave when false).
    pub until: bool,
    pub cond: Expr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestAt {
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopKind {
    For,
    Do,
    While,
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
    /// The labels of every body, in source order.
    pub labels: Vec<Label>,
    /// Every `CONST`, in source order.
    pub consts: Vec<Const>,
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

    pub fn label(&self, id: LabelId) -> &Label {
        &self.labels[id.0 as usize]
    }

    pub fn constant(&self, id: ConstId) -> &Const {
        &self.consts[id.0 as usize]
    }
}
