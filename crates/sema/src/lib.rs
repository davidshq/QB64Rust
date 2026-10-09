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

pub mod builtins;
mod check;
pub mod consteval;
mod dump;
pub mod literal;
mod symbols;

pub use check::{check, check_with};
pub use dump::dump_typed;
pub use symbols::{Symbol, SymbolId, SymbolKind, Symbols, dump_symbols};

use qb64rust_base::{FileId, Span};
use qb64rust_builtins::BuiltinId;

/// A type (design D3 of `m2-numeric-types`). It has no order: what a rule needs to know about a type it asks
/// through [`Ty::int_bits`], [`Ty::is_signed`], [`Ty::is_unsigned`], [`Ty::float_rank`] and [`Ty::storage`].
///
/// The variants `I8`, `U8`, `U16`, `U32`, `U64`, `Off`, `UOff`, `Bit` and `FixedStr` are produced nowhere yet (their
/// declarations are "not supported yet"); a `match` lists them and says why they cannot reach it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    /// `_BYTE`.
    I8,
    /// `_UNSIGNED _BYTE`.
    U8,
    /// `INTEGER`.
    I16,
    /// `_UNSIGNED INTEGER`.
    U16,
    /// `LONG`.
    I32,
    /// `_UNSIGNED LONG`.
    U32,
    /// `_INTEGER64`.
    I64,
    /// `_UNSIGNED _INTEGER64`.
    U64,
    /// `_OFFSET`: its own variant, not `I64`, because its operator rules differ (design D4).
    Off,
    /// `_UNSIGNED _OFFSET`.
    UOff,
    /// `_BIT * width` (1 to 64), `_UNSIGNED` when not `signed`.
    Bit {
        width: u8,
        signed: bool,
    },
    /// `SINGLE`.
    F32,
    /// `DOUBLE`.
    F64,
    /// `_FLOAT`.
    F80,
    Str,
    /// `STRING * n`: the type of a place only; loading it gives a `Str` value (design D3).
    FixedStr(u32),
    /// A `TYPE` of the program ([`Program::types`]), design D3 of `m2-arrays-and-types`.
    User(TypeId),
}

impl Ty {
    pub fn is_int(self) -> bool {
        self.int_bits().is_some()
    }

    pub fn is_float(self) -> bool {
        self.float_rank().is_some()
    }

    pub fn is_numeric(self) -> bool {
        self.is_int() || self.is_float()
    }

    /// The width in bits of an integer type, 8 to 64: `_OFFSET` 64 (the targets are 64-bit), `_BIT * n` that of its
    /// [`Ty::storage`] (32 or 64); `None` for a float, a string or a user type.
    pub fn int_bits(self) -> Option<u32> {
        match self {
            Ty::I8 | Ty::U8 => Some(8),
            Ty::I16 | Ty::U16 => Some(16),
            Ty::I32 | Ty::U32 => Some(32),
            Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => Some(64),
            Ty::Bit { width, .. } => Some(if width <= 32 { 32 } else { 64 }),
            Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str | Ty::FixedStr(_) | Ty::User(_) => None,
        }
    }

    /// A signed integer or a float.
    pub fn is_signed(self) -> bool {
        self.is_numeric() && !self.is_unsigned()
    }

    /// An `_UNSIGNED` integer type.
    pub fn is_unsigned(self) -> bool {
        match self {
            Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::UOff => true,
            Ty::Bit { signed, .. } => !signed,
            Ty::I8
            | Ty::I16
            | Ty::I32
            | Ty::I64
            | Ty::Off
            | Ty::F32
            | Ty::F64
            | Ty::F80
            | Ty::Str
            | Ty::FixedStr(_)
            | Ty::User(_) => false,
        }
    }

    /// The rank of a float type, SINGLE 1 < DOUBLE 2 < `_FLOAT` 3; `None` for any other type.
    pub fn float_rank(self) -> Option<u32> {
        match self {
            Ty::F32 => Some(1),
            Ty::F64 => Some(2),
            Ty::F80 => Some(3),
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
            | Ty::Bit { .. }
            | Ty::Str
            | Ty::FixedStr(_)
            | Ty::User(_) => None,
        }
    }

    /// Whether this numeric type is wider than `other`: a float is wider than an integer, a float of higher
    /// [`Ty::float_rank`] or an integer of more [`Ty::int_bits`] wider than another. Of two integer types of one
    /// width neither is wider (which one an operator computes in is design D4's `held`, task 5.1).
    pub fn is_wider_than(self, other: Ty) -> bool {
        match (self.float_rank(), other.float_rank()) {
            (Some(x), Some(y)) => x > y,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => {
                let bits = |t: Ty| {
                    t.int_bits()
                        .unwrap_or_else(|| unreachable!("{t:?} is not a numeric type"))
                };
                bits(self) > bits(other)
            }
        }
    }

    /// The type the old compiler stores a value of this type in (`study\02` §3.2): `_BIT * n` in a 32-bit integer up to
    /// 32 bits, a 64-bit one above, of its signedness; every other type in itself (`_OFFSET` is `ptrszint`).
    pub fn storage(self) -> Ty {
        match self {
            Ty::Bit { width, signed } => match (width <= 32, signed) {
                (true, true) => Ty::I32,
                (true, false) => Ty::U32,
                (false, true) => Ty::I64,
                (false, false) => Ty::U64,
            },
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
            | Ty::Str
            | Ty::FixedStr(_)
            | Ty::User(_) => self,
        }
    }

    /// The QB type name (`INTEGER`, `_FLOAT`...); `TYPE` for a user type, whose name is [`Program::type_name`]'s.
    pub fn qb_name(self) -> &'static str {
        match self {
            Ty::I8 => "_BYTE",
            Ty::U8 => "_UNSIGNED _BYTE",
            Ty::I16 => "INTEGER",
            Ty::U16 => "_UNSIGNED INTEGER",
            Ty::I32 => "LONG",
            Ty::U32 => "_UNSIGNED LONG",
            Ty::I64 => "_INTEGER64",
            Ty::U64 => "_UNSIGNED _INTEGER64",
            Ty::Off => "_OFFSET",
            Ty::UOff => "_UNSIGNED _OFFSET",
            Ty::Bit { signed: true, .. } => "_BIT",
            Ty::Bit { signed: false, .. } => "_UNSIGNED _BIT",
            Ty::F32 => "SINGLE",
            Ty::F64 => "DOUBLE",
            Ty::F80 => "_FLOAT",
            Ty::Str | Ty::FixedStr(_) => "STRING",
            Ty::User(_) => "TYPE",
        }
    }
}

/// The reason in an `unreachable!` arm for a type of [`unproduced_types`].
pub const NEW_TYPE_UNREACHABLE: &str =
    "the numeric types of m2-numeric-types and `STRING * n` are produced nowhere yet";

/// A pattern of the [`Ty`] variants that nothing produces yet (design D3 of `m2-numeric-types`), for the arm
/// `unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}")`. A variant leaves it when its declaration is
/// supported, so every `match` must then decide what it does with that type.
#[macro_export]
macro_rules! unproduced_types {
    () => {
        $crate::Ty::I8
            | $crate::Ty::U8
            | $crate::Ty::U16
            | $crate::Ty::U32
            | $crate::Ty::U64
            | $crate::Ty::Off
            | $crate::Ty::UOff
            | $crate::Ty::Bit { .. }
            | $crate::Ty::FixedStr(_)
    };
}

/// The size of a value of a numeric or user type in memory: what `LEN` of a place gives and what the layout of a
/// `TYPE` is made of. A `_FLOAT` takes 32 bytes as in the old compiler (`study\02` §1.7); a user type is its members'
/// sizes added up, in order, without padding (measured, `verification\v18_h_type_members`, `v20_g_len`).
pub fn size_of(types: &[UserType], t: Ty) -> u32 {
    match t {
        Ty::I16 => 2,
        Ty::I32 | Ty::F32 => 4,
        Ty::I64 | Ty::F64 => 8,
        Ty::F80 => 32,
        Ty::User(id) => types[id.0 as usize].members.iter().map(|m| size_of(types, m.ty)).sum(),
        Ty::Str => unreachable!("a string has no fixed size"),
        unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(pub u32);

/// A member of a user type: its index in [`UserType::members`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemberId(pub u32);

/// A `TYPE … END TYPE` of the main module. Its layout (sizes, offsets) is the emitter's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserType {
    /// The name in upper case.
    pub name: String,
    /// In declaration order.
    pub members: Vec<Member>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Member {
    /// The name in upper case, without suffix.
    pub name: String,
    /// A numeric type or a user type defined before this one.
    pub ty: Ty,
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
    /// 1-based source line of the label, in its file.
    pub line: u32,
    /// The file the label stands in (an included file's labels report errors with its name).
    pub file: FileId,
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

/// A variable: a name plus a type (design D5), and its storage. An array is a variable with dimensions; its `ty`
/// is the type of its elements. Arrays and scalars of the same name and type are different variables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
    /// The name without suffix, in upper case.
    pub name: String,
    pub ty: Ty,
    pub storage: Storage,
    /// The lower and upper bound of each dimension, first dimension first; empty for a scalar. Only static arrays
    /// exist so far: the bounds are known when compiling (design D4 of `m2-arrays-and-types`).
    pub dims: Vec<(i64, i64)>,
}

impl Var {
    pub fn is_array(&self) -> bool {
        !self.dims.is_empty()
    }
}

/// Where a value is read from and stored to (design D5 of `m2-arrays-and-types`).
#[derive(Clone, Debug, PartialEq)]
pub enum Place {
    /// A scalar variable.
    Var(VarId),
    /// An element of an array: one index per dimension, each already converted to `_INTEGER64`. An index outside
    /// its dimension's bounds raises error 9.
    Element { array: VarId, index: Vec<Expr> },
    /// A member of a place whose type is a user type.
    Member { base: Box<Place>, member: MemberId },
}

impl Place {
    /// Whether finding the place may raise an error: an element's indexes are checked, a member's base may be an
    /// element.
    pub fn may_raise(&self) -> bool {
        match self {
            Place::Var(_) => false,
            Place::Element { .. } => true,
            Place::Member { base, .. } => base.may_raise(),
        }
    }

    /// The variable the place is part of.
    pub fn root(&self) -> VarId {
        match self {
            Place::Var(v) | Place::Element { array: v, .. } => *v,
            Place::Member { base, .. } => base.root(),
        }
    }

    /// Whether an element lies on the way to the place (`a(i).m`, `a(i)`).
    pub fn has_element(&self) -> bool {
        match self {
            Place::Var(_) => false,
            Place::Element { .. } => true,
            Place::Member { base, .. } => base.has_element(),
        }
    }

    /// The indexes on the way to the place, outermost element first.
    pub fn indexes(&self) -> Vec<&Expr> {
        match self {
            Place::Var(_) => Vec::new(),
            Place::Element { index, .. } => index.iter().collect(),
            Place::Member { base, .. } => base.indexes(),
        }
    }
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
    /// The place itself (a variable, an element, a member): the procedure's assignments to the parameter change it.
    Ref(Place),
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
    /// The value of a place; never of a user type (a whole `TYPE` value is not a value, measured).
    Load(Place),
    /// `LBOUND` (`upper` false) or `UBOUND` of an array: a dimension's bound, typed `_INTEGER64` (measured). `dim`
    /// is LONG; absent for the first dimension. A dimension outside 1 to the number of dimensions raises error 9.
    Bound {
        upper: bool,
        array: VarId,
        dim: Option<Box<Expr>>,
    },
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
    /// A store; the value has the place's type (never a user type). The store rule depends on the place (design D6
    /// of `m2-arrays-and-types`).
    Assign {
        place: Place,
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
    /// `ON n GOTO l1, l2, …` (`gosub` false) or `ON n GOSUB …`: to the n-th label of the same body; 0 and values past
    /// the count (also above 255, `DIVERGENCES-QB45.md` Q-001) continue with the next statement, a negative value
    /// raises error 5. `value` is LONG (design D8 of `m2-core-builtins`, measured `verification\v20_i_on_goto`).
    OnJump {
        value: Expr,
        gosub: bool,
        targets: Vec<LabelId>,
    },
    /// `SELECT CASE` or `SELECT EVERYCASE` (design D7 of `m2-core-builtins`, measured `verification\v20_h_select`).
    Select {
        /// What the cases are compared with. A plain scalar variable is read at each test (`copied` false); anything
        /// else is evaluated once, at the `SELECT`, into a hidden variable of this expression's type, which is the
        /// old compiler's: a string, `_INTEGER64`, LONG for every narrower integer, or the float type it believes.
        selector: Expr,
        copied: bool,
        /// `EVERYCASE`: every matching `CASE` runs, and `CASE ELSE` only when none did.
        every: bool,
        cases: Vec<Case>,
        else_: Option<Vec<Stmt>>,
        /// 1-based source line of the `END SELECT`.
        end_line: u32,
    },
}

/// A `CASE` with its items (any of them matching runs the body) and its statements.
#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    pub items: Vec<CaseItem>,
    pub body: Vec<Stmt>,
    /// 1-based source line of the `CASE`.
    pub line: u32,
}

/// An item of a `CASE`, compared with the selector. Every value is already converted to the type the comparison is
/// computed in (the old compiler's: the item converted to the selector's type, a float item for an integer selector
/// rounded half to even; then C's usual conversions); the selector is converted to it where it is narrower.
#[derive(Clone, Debug, PartialEq)]
pub enum CaseItem {
    /// `value` (as `IS = value`) or `IS op value`; `op` is one of the six comparisons.
    Is(BinOp, Expr),
    /// `low TO high`: from `low` to `high` inclusive (`9 TO 1` matches nothing).
    Range(Expr, Expr),
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
    /// The user types, in source order.
    pub types: Vec<UserType>,
    /// Where each variable and procedure is defined and used (design D12).
    pub symbols: Symbols,
}

impl Program {
    pub fn var(&self, id: VarId) -> &Var {
        &self.vars[id.0 as usize]
    }

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

    /// The type of a place: a variable's, an array's element type, a member's.
    pub fn place_ty(&self, p: &Place) -> Ty {
        match p {
            Place::Var(v) | Place::Element { array: v, .. } => self.var(*v).ty,
            Place::Member { base, member } => self.member(self.place_ty(base), *member).ty,
        }
    }

    /// The QB name of a type, a user type's own name for one.
    pub fn type_name(&self, t: Ty) -> String {
        match t {
            Ty::User(id) => self.user_type(id).name.clone(),
            Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str => t.qb_name().to_string(),
            unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
        }
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

#[cfg(test)]
mod tests {
    use super::Ty;

    const fn bit(width: u8, signed: bool) -> Ty {
        Ty::Bit { width, signed }
    }

    #[test]
    fn int_bits() {
        let cases = [
            (Ty::I8, 8),
            (Ty::U8, 8),
            (Ty::I16, 16),
            (Ty::U16, 16),
            (Ty::I32, 32),
            (Ty::U32, 32),
            (Ty::I64, 64),
            (Ty::U64, 64),
            (Ty::Off, 64),
            (Ty::UOff, 64),
            (bit(1, true), 32),
            (bit(32, false), 32),
            (bit(33, true), 64),
            (bit(64, false), 64),
        ];
        for (t, bits) in cases {
            assert_eq!(t.int_bits(), Some(bits), "{t:?}");
            assert!(t.is_int() && t.is_numeric() && !t.is_float(), "{t:?}");
        }
        for t in [
            Ty::F32,
            Ty::F64,
            Ty::F80,
            Ty::Str,
            Ty::FixedStr(3),
            Ty::User(super::TypeId(0)),
        ] {
            assert_eq!(t.int_bits(), None, "{t:?}");
        }
    }

    #[test]
    fn float_rank() {
        assert_eq!(Ty::F32.float_rank(), Some(1));
        assert_eq!(Ty::F64.float_rank(), Some(2));
        assert_eq!(Ty::F80.float_rank(), Some(3));
        for t in [Ty::I16, Ty::U64, Ty::Off, bit(7, false), Ty::Str, Ty::FixedStr(3)] {
            assert_eq!(t.float_rank(), None, "{t:?}");
        }
        assert!(Ty::F80.is_float() && Ty::F80.is_numeric() && !Ty::F80.is_int());
        assert!(!Ty::Str.is_numeric() && !Ty::FixedStr(3).is_numeric());
    }

    #[test]
    fn signedness() {
        for t in [Ty::U8, Ty::U16, Ty::U32, Ty::U64, Ty::UOff, bit(7, false)] {
            assert!(t.is_unsigned() && !t.is_signed(), "{t:?}");
        }
        for t in [
            Ty::I8,
            Ty::I16,
            Ty::I32,
            Ty::I64,
            Ty::Off,
            bit(1, true),
            Ty::F32,
            Ty::F64,
            Ty::F80,
        ] {
            assert!(t.is_signed() && !t.is_unsigned(), "{t:?}");
        }
        for t in [Ty::Str, Ty::FixedStr(3), Ty::User(super::TypeId(0))] {
            assert!(!t.is_signed() && !t.is_unsigned(), "{t:?}");
        }
    }

    #[test]
    fn is_wider_than() {
        // Narrowest first; each is wider than every one before it.
        let order = [Ty::I8, Ty::I16, Ty::I32, Ty::I64, Ty::F32, Ty::F64, Ty::F80];
        for (i, &a) in order.iter().enumerate() {
            for (j, &b) in order.iter().enumerate() {
                assert_eq!(a.is_wider_than(b), i > j, "{a:?} {b:?}");
            }
        }
        // One width: neither is wider.
        for (a, b) in [
            (Ty::I32, Ty::U32),
            (Ty::I64, Ty::Off),
            (Ty::U64, Ty::UOff),
            (Ty::I32, bit(5, true)),
        ] {
            assert!(!a.is_wider_than(b) && !b.is_wider_than(a), "{a:?} {b:?}");
        }
        assert!(!Ty::U8.is_wider_than(Ty::I8) && Ty::U16.is_wider_than(Ty::I8));
    }

    #[test]
    fn storage() {
        assert_eq!(bit(1, true).storage(), Ty::I32);
        assert_eq!(bit(32, false).storage(), Ty::U32);
        assert_eq!(bit(33, true).storage(), Ty::I64);
        assert_eq!(bit(64, false).storage(), Ty::U64);
        for t in [Ty::I8, Ty::U16, Ty::Off, Ty::UOff, Ty::F80, Ty::Str, Ty::FixedStr(3)] {
            assert_eq!(t.storage(), t, "{t:?}");
        }
    }
}
