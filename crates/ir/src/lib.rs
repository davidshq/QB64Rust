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
//!   - a store into an element of an array variable ([`Place::Element`] of an [`ArrayRef::Var`]): its indexes are
//!     evaluated first, left to right; while an error is pending after them the value is not evaluated and nothing
//!     is stored.
//!
//!   A store into a place reached through a member with an index on the way ([`Place::through_member`]: a member
//!   of an element, `a(i).m`, or an element of a member array, `t.v(i)`, `a(i).v(j)`) **evaluates a number before
//!   the indexes and a string after them**, the indexes from left to right, outermost first (measured,
//!   `verification\v18_h_member_index_order`, `v24_h_member_store`, `v24_x313`, `x316`). It is skipped when an
//!   index is out of range or when the indexes raised an error while none was pending before them
//!   (`DIVERGENCES.md` D-004; the old compiler writes element 0 of the dimension whose index was outside); an
//!   error the string value raises after the indexes does not skip it. A store into a variable or into a member
//!   of one does not check. An index out of range raises error 9, and a read with it gives the value at position
//!   0 of its dimension (measured, `verification\v18_*`, `v24_h_member_store`).
//!
//!   A target of [`Op::Read`] or of an [`Op::Input`] from a file reached this way is stored by the same rule: a
//!   number is read before the indexes (an index outside skips the store, the item is taken), a string after them
//!   (an index outside raises error 9 first, so the read takes nothing). A target of an [`Op::Input`] from the
//!   console has its indexes evaluated, left to right and target by target, after the prompt and before the line
//!   is read; with an error pending then nothing is read or stored (measured, `verification\v35_x62`).
//!
//!   The bytes of a member array lie in the value they are a member of (design D8 of
//!   `m2-defxxx-common-arrays`): its bounds are its member's and never change, so nothing allocates or releases
//!   it. The IR names no offset and no size in bytes for it.
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
//!   Three operations take numbers of any type and convert them themselves, each to the type the node states:
//!   the two value parts of an [`ExprKind::Choice`] (to the node's type) and the arguments of the built-ins of
//!   [`builtins::Rule::Widest`] and [`builtins::Rule::Rotate`] (to the node's type; the count of a rotation to
//!   an integer of any width).
//! - **A value is one expression**: nothing of it is evaluated before or apart from it. So an
//!   [`ExprKind::Choice`] evaluates its condition and then only the part chosen, and the right operand of
//!   `_ANDALSO` and `_ORELSE` only when the left one does not decide.
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
/// The fields of the executable's version resource (`$VERSIONINFO`), which the driver writes as they are.
pub use qb64rust_sema::VersionInfo;
/// Which built-in functions are compiled and the rule each follows; the emitter writes a call by its rule.
pub use qb64rust_sema::builtins;
/// What a variable is, a scalar, a static array or a dynamic one, is `sema`'s decision and type (design D3 of
/// `m2-defxxx-common-arrays`), as is what an allocation does with an array that is allocated already.
pub use qb64rust_sema::{AllocMode, Shape};
/// Values, places and arguments are `sema`'s typed tree too (design D10 of `m2-arrays-and-types`: the lowering did
/// no work on them, only a variant-for-variant copy). The emitter ignores an [`Expr`]'s `span` and `qb`. Variable and
/// procedure ids are `sema`'s; the IR's variable list is `sema`'s with the lowering's temporaries appended
/// ([`Program::vars`]).
pub use qb64rust_sema::{
    Arg, ArrayRef, Expr, ExprKind, InputSource as Source, MemValue, Place, ProcId, Reference, StmtArg, VarId,
};
/// The event of an [`Op::OnEvent`] and the forms of [`Op::Key`] are `sema`'s (design D3 of `m3-events`).
pub use qb64rust_sema::{Event, KeyForm};
/// The `DECLARE LIBRARY` blocks and the C function behind a library procedure are `sema`'s (design D1 of
/// `m3-declare-library`): the names as written. Finding the files they name is the driver's.
pub use qb64rust_sema::{External, Library, LibraryId, LibraryKind, LibraryName};
/// Where a `PRINT` goes and the values of a `PRINT USING` are `sema`'s too.
pub use qb64rust_sema::{PrintTo, UsingValue};
/// The size of a numeric or user type in memory (the layout of a `TYPE`, `LEN` of a place), and whether a type
/// holds a `STRING` of its own, which has no such size, or owns dynamic member arrays.
pub use qb64rust_sema::{holds_strings, owns_desc, size_of};

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
    /// In an included file, the line of the main file that `_ERRORLINE` gives there ([`Stmt::include_line`]).
    pub include_line: Option<u32>,
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
/// types or their storage differ. An array has a [`Shape`] other than `Scalar`: static, with the lower and upper
/// bound of each dimension (first dimension first) known when compiling, or dynamic, with a number of dimensions
/// and no bounds (design D3 of `m2-defxxx-common-arrays`); `ty` is its element type. A static array exists for
/// as long as its storage does; a dynamic one is not allocated until an [`Op::Allocate`] runs, and until then
/// (and after an [`Op::Release`]) an element of it, and its bounds, raise error 9. The IR says what an array is,
/// never how it is held: no descriptor, no size in bytes.
///
/// An array parameter (storage [`Storage::Param`]) is written as a dynamic array and stands, in each call, for
/// the array passed as [`Arg::Array`], which may be static or dynamic: an [`Op::Allocate`] of it raises error 10
/// for a static one, and its `ERASE` is [`Op::EraseParam`]. Its number of dimensions is that of every array
/// passed to it; it has none when nothing in the program says one, and then no array is ever passed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
    pub name: String,
    pub ty: Ty,
    pub storage: Storage,
    pub shape: Shape,
    /// An array in conventional memory: one a `VARPTR` or `VARSEG` of the program names. A backend gives its
    /// elements a place there, where `DEF SEG` with `PEEK`, `POKE`, `BLOAD` and `BSAVE` reach them; everything
    /// else about the array is as for any array of its shape.
    pub conventional: bool,
}

impl Var {
    pub fn is_array(&self) -> bool {
        !matches!(self.shape, Shape::Scalar)
    }

    /// The number of dimensions of an array, where it is known; `None` for a scalar too.
    pub fn dim_count(&self) -> Option<usize> {
        match &self.shape {
            Shape::Scalar => None,
            Shape::Static { bounds } => Some(bounds.len()),
            Shape::Dynamic { dims, .. } => *dims,
        }
    }

    /// The bounds of a static array; empty for anything else.
    pub fn bounds(&self) -> &[(i64, i64)] {
        match &self.shape {
            Shape::Static { bounds } => bounds,
            Shape::Scalar | Shape::Dynamic { .. } => &[],
        }
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
    /// A procedure of a `DECLARE LIBRARY` block (design D1 of `m3-declare-library`): a function of the library,
    /// called with its parameters, its body empty. Its parameters and result are variables only for the calls'
    /// sake: nothing stores into them. A `BYVAL` parameter takes an [`Arg::Value`] or an [`Arg::Null`], another
    /// one an [`Arg::Ref`] or an [`Arg::Temp`]; only a library procedure takes those two.
    pub external: Option<External>,
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
            // The index check of the element; a segment is the array's and reads no element.
            ExprKind::VarPtr { .. } => true,
            ExprKind::VarSeg { .. } => false,
            // An element given is checked against the bounds.
            ExprKind::Block { index, .. } => !index.is_empty(),
            ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => from.may_raise(),
            ExprKind::Binary { op, lhs, rhs } => op_may_raise(*op) || lhs.may_raise() || rhs.may_raise(),
            ExprKind::Choice { condition, then, else_ } => {
                condition.may_raise() || then.may_raise() || else_.may_raise()
            }
            ExprKind::Concat(..) | ExprKind::StrCompare { .. } | ExprKind::Call { .. } | ExprKind::CallProc { .. } => {
                true
            }
            // The index checks on the way to the reference; a negative size raises error 301.
            ExprKind::MemOf { of, .. } | ExprKind::AddressOf(of) => !of.indexes().is_empty(),
            ExprKind::MemAt { .. } => true,
            // A block never set, freed, or a region beyond it.
            ExprKind::MemRead { .. } => true,
        }
    }

    fn uses_strings(&self, p: &Program) -> bool {
        self.ty == Ty::Str
            || match &self.kind {
                ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) => false,
                ExprKind::Load(place) => place.uses_strings(p),
                ExprKind::Bound { dim, .. } => dim.as_deref().is_some_and(|d| d.uses_strings(p)),
                ExprKind::VarPtr { index, .. } | ExprKind::Block { index, .. } => {
                    index.iter().any(|i| i.uses_strings(p))
                }
                ExprKind::VarSeg { .. } => false,
                ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => from.uses_strings(p),
                ExprKind::Binary { lhs, rhs, .. }
                | ExprKind::Concat(lhs, rhs)
                | ExprKind::StrCompare { lhs, rhs, .. } => lhs.uses_strings(p) || rhs.uses_strings(p),
                ExprKind::Choice { condition, then, else_ } => {
                    condition.uses_strings(p) || then.uses_strings(p) || else_.uses_strings(p)
                }
                ExprKind::Call { args, .. } => args.iter().flatten().any(|a| a.uses_strings(p)),
                ExprKind::CallProc { args, .. } => args.iter().any(|a| a.uses_strings(p)),
                // An element or a whole array of `STRING * n` is reached through a temporary fixed `qbs`.
                ExprKind::MemOf { of, .. } | ExprKind::AddressOf(of) => {
                    of.indexes().iter().any(|i| i.uses_strings(p))
                        || matches!(of, Reference::Place(Place::Element { array, .. }) | Reference::Array(array)
                            if matches!(p.array_ty(array), Ty::FixedStr(_)))
                }
                ExprKind::MemAt { offset, size } => offset.uses_strings(p) || size.uses_strings(p),
                ExprKind::MemRead { block, offset, .. } => block.uses_strings(p) || offset.uses_strings(p),
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
    /// Finding a place passed by reference may raise (its index); a copy as its value does; finding a member
    /// array may (an index on the way to it).
    fn may_raise(&self) -> bool {
        match self {
            Arg::Ref(place) => Facts::may_raise(place),
            Arg::Temp(v) | Arg::Value(v) => v.may_raise(),
            Arg::Array(array) => array.may_raise(),
            Arg::Null => false,
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
            Arg::Temp(v) | Arg::Value(v) => v.uses_strings(p),
            Arg::Array(array) => array.indexes().iter().any(|v| v.uses_strings(p)),
            Arg::Null => false,
        }
    }
}

impl Facts for StmtArg {
    fn may_raise(&self) -> bool {
        match self {
            StmtArg::Value(v) => v.may_raise(),
            StmtArg::Place(place) => Facts::may_raise(place),
            // The index check of the element the bytes start at.
            StmtArg::Block { index, .. } => !index.is_empty(),
            StmtArg::Bytes(place) => Facts::may_raise(place),
            StmtArg::Word(_) | StmtArg::Absent => false,
        }
    }

    /// A string value, or a place that is a string or whose index involves strings.
    fn uses_strings(&self, p: &Program) -> bool {
        match self {
            StmtArg::Value(v) => v.uses_strings(p),
            StmtArg::Place(place) | StmtArg::Bytes(place) => place.uses_strings(p) || p.place_ty(place).is_string(),
            StmtArg::Block { index, .. } => index.iter().any(|i| i.uses_strings(p)),
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
    Assign { place: Place, value: Expr },
    /// Copy the whole value of one place of a user type into another of the same type (design D9 of
    /// `m2-defxxx-common-arrays`): every member, a `STRING` member as a string of its own; a place copied to
    /// itself keeps its value. **The copy is not guarded** (measured, `verification\v24_h_whole`, `v24_x210`):
    /// an index outside its bounds raises error 9 and the copy is made with the array's first element in the
    /// place of the element named, on either side; any other error an index raises leaves its placeholder index.
    ///
    /// `to` is found first, then `from`. For a type without a `STRING` in it each is found once. **For a type
    /// that holds strings ([`holds_strings`]) both are found again for every part of the value**, in
    /// declaration order: a `STRING` member is a part, a member of a user type that holds strings gives its own
    /// parts in its place, any other member is one part (measured, `v24_x306`, `x307`: a FUNCTION in an index is
    /// called once per part; the old compiler's behaviour, listed for review in `SOMEDAY.md`). A member array
    /// goes element by element where its elements are parts: each element of a `STRING` member array is a
    /// part, each element of one of a user type that holds strings gives its own parts, and any other member
    /// array is one part, all its elements together (measured, `v24_x310`).
    CopyWhole { from: Place, to: Place },
    /// Give an array bounds and elements, zero or empty (design D4 of `m2-defxxx-common-arrays`): one lower and
    /// one upper bound per dimension, each an `I64`. In this order (measured, `study\00` §5):
    ///
    /// 1. a static array raises error 10, and nothing else happens: the bounds are not evaluated;
    /// 2. with [`AllocMode::Dim`], an array that is allocated raises error 10 unless an error is pending, and
    ///    nothing else happens;
    /// 3. the bounds are evaluated in order, lower before upper, first dimension first. **A raising bound does not
    ///    stop the operation**: it goes on with the placeholder value;
    /// 4. a dimension with no element (its upper bound one below its lower bound) raises error 5, and the
    ///    operation goes on: the array is then allocated with these bounds and no element;
    /// 5. a dimension whose upper bound is lower still, and a size the machine cannot hold, end the program ("Out
    ///    of memory"; no handler runs);
    /// 6. the array has the new bounds. Its elements are new; with [`AllocMode::Preserve`] the first elements by
    ///    flat position (first dimension fastest) keep their values, as many as the smaller of the old and the
    ///    new number of elements; with [`AllocMode::Retain`] each element whose indexes lie within both the old
    ///    and the new bounds of every dimension keeps its value at the same indexes. "Old" is the array as it is
    ///    after step 3: a FUNCTION in a bound may have given it other bounds or erased it (`DIVERGENCES.md`
    ///    D-018; the old compiler takes it before step 3 and frees a block twice).
    ///
    /// The array may be static or dynamic. Always may raise.
    ///
    /// The array may be a dynamic member array ([`ArrayRef::Member`] of a [`Member`] that is `dynamic`; design D5
    /// of `m3-typefields`): the place it is part of is found once, its indexes evaluated before the bounds, and
    /// not guarded: an index outside its bounds on the way raises error 9 and the member of the element at
    /// position 0 of that dimension stands in. Step 1 does not apply (it is never static); its number of
    /// dimensions may differ from the one it had, which gives it new elements whatever the mode.
    Allocate {
        array: ArrayRef,
        bounds: Vec<(Expr, Expr)>,
        mode: AllocMode,
    },
    /// Set every element of a static array to zero, an empty string or NUL bytes; its bounds stay.
    ///
    /// Of a member array ([`ArrayRef::Member`]): the place it is part of is found once, then its elements are
    /// cleared. **Not guarded** (measured, `verification\v24_x312`): an index outside its bounds on the way
    /// raises error 9 and the member array of the element at position 0 of that dimension is cleared, as it is
    /// after any other error an index raises.
    Clear { array: ArrayRef },
    /// Release the elements of a dynamic array: it is not allocated afterwards (every use raises error 9) until an
    /// [`Op::Allocate`] runs. Of an array that is not allocated it does nothing. Of a dynamic member array the
    /// place is found as for [`Op::Allocate`].
    Release { array: ArrayRef },
    /// `ERASE` of an array parameter: [`Op::Clear`] when the array passed in this call is a static one,
    /// [`Op::Release`] when it is dynamic.
    EraseParam { array: VarId },
    /// Copy every element of the array `from` into the array `to` (`to() = from()`; design D3 of
    /// `m3-whole-array-assign`; measured, `study\00` §5, the `v36` paragraph). Both have one element type and
    /// number of dimensions; each is an array variable or a member array. In this order:
    ///
    /// 1. `to`'s place is found, then `from`'s, each once (an [`ArrayRef::Member`] with indexes on the way: an
    ///    index outside its bounds raises error 9, and the member of the element at position 0 stands in);
    /// 2. **if an error was raised so far, nothing is copied** (`DIVERGENCES.md` D-004: the old compiler copies a
    ///    `STRING` member array through the one of element 0);
    /// 3. both arrays must be allocated (a static array always is, also after `ERASE`) and have, in every
    ///    dimension, the same lower bound and number of elements; else error 9, and nothing is copied, but for
    ///    two array variables of an owner type, which copy the target's number of elements after the error when
    ///    the source has that many, as the old compiler does (`DIVERGENCES.md` D-022: it also does when the source
    ///    has fewer, reading past it);
    /// 4. the same block on both sides (`a() = a()`) copies nothing;
    /// 5. every element is copied deep, by kind: a `STRING` as a string of its own, a user type as
    ///    [`Op::CopyWhole`] copies a value, an owner value with the bounds and elements of each of its dynamic
    ///    member arrays.
    ///
    /// The target keeps its bounds. Always may raise.
    CopyArray { from: ArrayRef, to: ArrayRef },
    /// Copy elements of the array `from` into the array `to` (`_ARRAYCOPY`; design D4 of `m3-arraycopy`; measured,
    /// `study\00` §5, the `v41` paragraph). Both have one element type: a number type, `_OFFSET`, `STRING * n` or
    /// `STRING`; each is an array variable or a member array. `ranges` is the source's lower and upper bound per
    /// dimension, none for the whole array; `starts` the destination's first indexes, one per dimension, none for
    /// the whole array; each an `I64`. In this order:
    ///
    /// 1. `from`'s place is found, then `to`'s, each once (an [`ArrayRef::Member`] with indexes on the way: an index
    ///    outside its bounds raises error 9, and the member of the element at position 0 stands in; a dynamic
    ///    member that a `REDIM` gave another number of dimensions than declared raises error 9 and is not copied);
    /// 2. every lower bound, then every upper bound, then every start is evaluated, first dimension first. **A
    ///    raising bound or start does not stop the copy**, which goes on with its value;
    /// 3. **if finding a place raised, nothing is copied** (`DIVERGENCES.md` D-004: the old compiler copies
    ///    through the member of element 0);
    /// 4. both arrays must be allocated (a static array always is, also after `ERASE`); every range must lie in
    ///    the source with its lower bound not above its upper; the elements must fit in the destination: from the
    ///    starts, or, with no starts, at the indexes the range gives them (with no ranges either: one dimension,
    ///    the source's own indexes; several, the same bounds on both sides). Else error 9 and nothing is copied;
    /// 5. the elements are copied as if all were read before any is written (the two may overlap or be one
    ///    array): a range's elements to the starts, each coordinate mapped by itself; a whole source in its order
    ///    of elements from the starts; a `STRING` as a string of its own.
    ///
    /// Always may raise.
    ArrayCopy {
        from: ArrayRef,
        to: ArrayRef,
        ranges: Vec<(Expr, Expr)>,
        starts: Vec<Expr>,
    },
    /// Items in order, to the program's destination of text, to the file with a number (an `I32`) or to the
    /// printer ([`PrintTo`]); a raising item skips the rest, including the line end. In a file a
    /// [`PrintItem::Zone`] is 14 columns wide, on the console 10 (the runtime's).
    Print {
        to: PrintTo,
        items: Vec<PrintItem>,
        newline: bool,
    },
    /// Formatted output (design D1 of `m3-console-files`; measured, `study\00` §5, the `v30` paragraph), in this
    /// order:
    ///
    /// 1. to a file: the number is evaluated; an error pending after it ends the statement;
    /// 2. `before` is printed item by item as an [`Op::Print`] with no line end prints it; a raising item ends
    ///    the statement;
    /// 3. `format` (a string) is evaluated, once. An error pending after it ends the statement, with nothing
    ///    more printed to a file and with step 5 on the screen and the printer;
    /// 4. each value in turn: a [`UsingValue::Str`] or [`UsingValue::Num`] is formatted by the next field of the
    ///    format and added to the statement's text (the format starts again when values are left; a field of
    ///    the other kind raises error 13, a format with no field error 5); a [`UsingValue::Blanks`] prints the
    ///    text so far, then its blanks, and empties the text. An error pending after a value goes to step 5;
    /// 5. the text is printed: with a line end if `newline` and no error is pending, else with none. An error
    ///    stays pending.
    ///
    /// **The statement is one unit**: no other rule of the IR for a pending error applies inside it. At least
    /// one value. Always may raise.
    PrintUsing {
        to: PrintTo,
        before: Vec<PrintItem>,
        format: Expr,
        values: Vec<UsingValue>,
        newline: bool,
    },
    /// Name parts of the record buffer of the random file with this number (an `I32`) by strings, in order from
    /// the buffer's first byte (design D3 of `m3-console-files`; measured, `study\00` §5): each part a place of
    /// type `Str` and a width (an `I32`). The file number is evaluated first; then, part by part, the indexes of
    /// the place before the width, and the part is made a field. **Not guarded:** a raising value does not stop
    /// the operation; the runtime makes no part a field while an error is pending, and a part that goes beyond
    /// the record raises error 50 and ends the naming there (the parts before it are fields). A file that is
    /// not open raises error 52, one that is not random error 54, a negative width error 5; an index outside
    /// its bounds error 9. Always may raise.
    Field { file: Expr, parts: Vec<(Place, Expr)> },
    /// Copy bytes from a memory block into a variable (`_MEMGET block, offset, variable`; design D4 of `m3-mem`).
    /// The block is a place of type `_MEM`, the offset an `_OFFSET`, the target a variable, element, member, whole
    /// array or whole member array of any type but `_BIT` (a variable-length string: its characters, as many as it
    /// holds). In order: the offset is evaluated; a block never set raises error 309; a freed one (its lock's id
    /// changed) 308, a region not within the block 300, and nothing is copied; else the target's bytes are copied.
    /// These errors end the program: no handler runs (measured, `verification\v32_d*`). Always may raise.
    MemGet {
        block: Place,
        offset: Expr,
        target: Reference,
    },
    /// Copy bytes into a memory block (`_MEMPUT`): a [`MemValue::Str`] is evaluated first, then the offset, the
    /// checks of [`Op::MemGet`], and the value's bytes are written (a typed value is evaluated there). Always may
    /// raise.
    MemPut {
        block: Place,
        offset: Expr,
        value: MemValue,
    },
    /// Fill bytes of a memory block with a value's bytes, repeated (`_MEMFILL`): a [`MemValue::Str`] is evaluated
    /// first, then the offset and the count (both `_OFFSET`s); a typed value is an argument of the runtime's call
    /// beside them, in the order C++ gives arguments (none fixed), as the old compiler writes it. The runtime
    /// raises error 309 for a block never set, 308 for a freed one, 301 for a negative count or a value of no
    /// bytes, 300 for a region not within the block. Always may raise.
    MemFill {
        block: Place,
        offset: Expr,
        bytes: Expr,
        value: MemValue,
    },
    /// Put a string's characters into the string at a place, at its left or (`right`) at its right, without
    /// changing its length: padded with blanks, or cut at its end. The place is of type `Str` or `FixedStr`
    /// and is found before the value is evaluated; with an error pending after the value nothing is stored.
    /// When the string is a field of a file's record buffer, its part of the buffer is set too, and every
    /// field string of that file is read from the buffer again.
    Lset { right: bool, target: Place, value: Expr },
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
    /// pending-error checks. At least one target; none is of a user type.
    Input {
        from: Source,
        line: bool,
        targets: Vec<Place>,
    },
    /// Read the next item of the program's data ([`Program::data`]) into each target in turn, each stored by the
    /// rule of its place. A numeric target takes the item's value in its own type. No point of it checks for a
    /// pending error: see the crate documentation. At least one target; none is of a user type.
    Read(Vec<Place>),
    /// Make the next [`Op::Read`] start at item `at` of [`Program::data`] (at its end: nothing is left to read).
    /// `label` is the BASIC label, in upper case, that gave the position; `None` for the start of the data (`at`
    /// is 0). The emitter may name the position after it.
    Restore { at: usize, label: Option<String> },
    /// End the program (`END`), with the "press any key" prompt of a program with a console. With a code (a number,
    /// `END n`), a pending error is serviced first, then the code is the process's exit code (its low 32 bits).
    End(Option<Expr>),
    /// End the program at once (`SYSTEM`); the code as for [`Op::End`].
    System(Option<Expr>),
    /// A SUB call; one argument per parameter. Always may raise.
    Call { proc: ProcId, args: Vec<Arg> },
    /// A built-in statement that is one call of the runtime (design D2 of `m2-builtin-statements`): the built-in by
    /// its table entry, which also says which form was written, and one slot per argument or choice of its template
    /// in template order ([`builtins::stmt_slots`]). The values are evaluated in order; the call is made also after
    /// a raising one. Always may raise.
    Builtin { id: BuiltinId, args: Vec<StmtArg> },
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
    /// Set the handler of an event (design D3 of `m3-events`): the operands are evaluated in order, then the event's
    /// handler is replaced. The handler runs when the runtime services the event at a statement boundary (not
    /// while an error handler runs, not while the same event's handler runs): a label of the main module as a
    /// `GOSUB` from there, whatever body set it up, or a SUB with the argument evaluated here. A timer number that
    /// is not allocated, or a key outside 1 to 31, raises error 5. Always may raise.
    OnEvent { event: Event, handler: EventHandler },
    /// `KEY ON`, `KEY OFF`, `KEY LIST` or `KEY n, text$`; a soft key's number outside its range raises error 5.
    /// Always may raise.
    Key(KeyForm),
}

/// The handler of an [`Op::OnEvent`].
#[derive(Clone, Debug, PartialEq)]
pub enum EventHandler {
    /// A label of the main module ([`Body::labels`] of [`Program::main`]).
    Label(LabelId),
    /// A SUB with no parameter or one of an integer type, and its argument (an `I64`) for it.
    Sub { proc: ProcId, arg: Option<Expr> },
}

impl Op {
    /// Whether this operation may raise a runtime error.
    pub fn may_raise(&self) -> bool {
        match self {
            Op::SelectConsole
            | Op::End(None)
            | Op::System(None)
            | Op::Exit
            | Op::SetHandler(_)
            | Op::Jump(_)
            | Op::Gosub(_)
            | Op::Restore { .. }
            | Op::EraseParam { .. } => false,
            // The place a member array is part of may be an element.
            Op::Clear { array } | Op::Release { array } => array.may_raise(),
            // `RESUME` outside a handler raises error 20, `RETURN` with no `GOSUB` pending error 3.
            Op::Call { .. }
            | Op::Builtin { .. }
            | Op::OnEvent { .. }
            | Op::Key(_)
            | Op::Raise(_)
            | Op::Resume(_)
            | Op::Return(_)
            | Op::Allocate { .. }
            | Op::CopyArray { .. }
            | Op::ArrayCopy { .. }
            | Op::MemGet { .. }
            | Op::MemPut { .. }
            | Op::MemFill { .. } => true,
            Op::Assign { place, value } => place.may_raise() || value.may_raise(),
            Op::CopyWhole { from, to } => from.may_raise() || to.may_raise(),
            Op::AssignAll(stores) => stores.iter().any(|(_, v)| v.may_raise()),
            Op::Branch { cond, .. } => cond.may_raise(),
            Op::End(Some(code)) | Op::System(Some(code)) => code.may_raise(),
            // A file may not be open, a console input may meet the end of the input, the data may be used up; a
            // format may have no field for a value; a printer may not be there.
            Op::Print {
                to: PrintTo::File(_) | PrintTo::Printer,
                ..
            }
            | Op::PrintUsing { .. }
            | Op::Field { .. }
            | Op::Lset { .. }
            | Op::Write { to: Some(_), .. }
            | Op::Input { .. }
            | Op::Read(_) => true,
            Op::Print {
                to: PrintTo::Screen,
                items,
                ..
            }
            | Op::Write { to: None, items, .. } => items.iter().any(|i| match i {
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
    /// In an included file, the line of the main file that `_ERRORLINE` gives there: that of the `$INCLUDE` that
    /// brought the outermost included file in; `None` in the main file.
    pub include_line: Option<u32>,
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
    /// The `DECLARE LIBRARY` blocks in file order; [`External::library`] indexes it. A `DYNAMIC` library is loaded,
    /// and each of its functions found, when the program starts: a library that cannot be loaded raises error
    /// 259, a function it does not hold error 260, before the first statement (measured, `verification\v37_j`).
    pub libraries: Vec<Library>,
    pub settings: Settings,
}

/// What a program needs around its statements (design D3 of `m2-console-screenhide`): where its text goes at the
/// start, and the optional parts of the runtime its built-ins use. How a backend provides them (start-up variables,
/// build flags) is the backend's.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    /// The program has a console: [`Op::SelectConsole`] may be used, and text sent to the console reaches
    /// standard output.
    pub console: bool,
    /// The program has a window. Without one, the console is the only place for its text.
    pub window: bool,
    /// The window is hidden when the program starts. Recorded for a program without a window too, as the old
    /// compiler does.
    pub window_hidden: bool,
    /// Assertions are checked (`$ASSERTS`): an `_ASSERT` whose condition is 0 raises an error.
    pub asserts: bool,
    /// The optional parts of the runtime the program's built-ins need, in order.
    pub runtime_parts: std::collections::BTreeSet<RuntimePart>,
    /// The executable carries an icon (`$EXEICON`). Its file is the driver's to find, from the checked program.
    pub exe_icon: bool,
    /// The executable carries a version resource and a manifest (`$VERSIONINFO`).
    pub version_info: Option<VersionInfo>,
}

/// An optional part of the runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimePart {
    /// Setting the window's icon from an image.
    Icon,
    /// libqb's audio.
    Audio,
    /// The printer.
    Printer,
    /// The image decoders and encoders.
    ImageCodec,
    /// FreeType, for fonts loaded from a file.
    Font,
    /// Images of the desktop and the clipboard.
    ScreenImage,
    /// Checksums, compression and base64.
    DataProcessing,
}

impl Program {
    pub fn user_type(&self, id: TypeId) -> &UserType {
        &self.types[id.0 as usize]
    }

    /// Whether `t` is the predefined type `_MEM`, a memory block (design D1 of `m3-mem`): the runtime's block,
    /// 56 bytes, whose members are its address, size, lock, type value, element size, image and sound.
    pub fn is_mem(&self, t: Ty) -> bool {
        matches!(t, Ty::User(id) if self.types.get(id.0 as usize).is_some_and(|u| u.name == qb64rust_sema::MEM_TYPE))
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
            Place::Var(v) => self.var(*v).ty,
            Place::Element { array, .. } => self.array_ty(array),
            Place::Member { base, member } => self.member(self.place_ty(base), *member).ty,
        }
    }

    /// The type of an array's elements.
    pub fn array_ty(&self, a: &ArrayRef) -> Ty {
        match a {
            ArrayRef::Var(v) => self.var(*v).ty,
            ArrayRef::Member { base, member } => self.member(self.place_ty(base), *member).ty,
        }
    }

    /// The member an [`ArrayRef::Member`] names: `member` of a place `base`.
    pub fn member_of(&self, base: &Place, member: MemberId) -> &Member {
        self.member(self.place_ty(base), member)
    }

    pub fn var(&self, id: VarId) -> &Var {
        &self.vars[id.0 as usize]
    }

    pub fn proc(&self, id: ProcId) -> &Proc {
        &self.procs[id.0 as usize]
    }

    pub fn library(&self, id: LibraryId) -> &Library {
        &self.libraries[id.0 as usize]
    }
}
