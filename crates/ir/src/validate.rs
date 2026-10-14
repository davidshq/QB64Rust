//! Structural checks of a lowered program (design D8 of `m2-arrays-and-types`): what the emitter relies on and the
//! types cannot say. Tier 1 runs it on every accepted input; the lowering runs it under `debug_assert!`.

use crate::builtins::{Slot, StmtRule, StmtSlot, stmt_find, stmt_slots};
use crate::{
    Arg, ArrayRef, Body, BuiltinId, Expr, ExprKind, LabelId, MemValue, Member, MemberId, Op, Place, PrintItem, PrintTo,
    ProcId, Program, Reference, Resume, Shape, Source, StmtArg, Storage, Ty, UsingValue, Var, VarId,
};

/// Every problem found, each as `<body>: <where>: <what>`; `Ok` when there is none.
pub fn validate(p: &Program) -> Result<(), Vec<String>> {
    let mut v = Validator {
        p,
        problems: Vec::new(),
    };
    // Its text has to go somewhere.
    if !p.settings.console && !p.settings.window {
        v.problem("settings: neither a console nor a window".to_string());
    }
    for (i, proc) in p.procs.iter().enumerate() {
        let id = ProcId(qb64rust_base::to_u32(i));
        for (k, param) in proc.params.iter().enumerate() {
            if v.var_exists(*param, &format!("{}: parameter {}", proc.name, k + 1))
                && p.var(*param).storage != Storage::Param(id)
            {
                v.problem(format!(
                    "{}: parameter {} is not a parameter of this procedure",
                    proc.name,
                    k + 1
                ));
            }
        }
        if let Some(r) = proc.result
            && v.var_exists(r, &format!("{}: result", proc.name))
            && p.var(r).storage != Storage::Result(id)
        {
            v.problem(format!("{}: the result is not a result of this procedure", proc.name));
        }
    }
    // An array allocated at the start lives for the program, and has one pair of bounds per dimension.
    for var in &p.vars {
        if let Shape::Dynamic {
            dims,
            at_start: Some(bounds),
        } = &var.shape
        {
            if !matches!(var.storage, Storage::Global | Storage::Static(_)) {
                v.problem(format!(
                    "{}: allocated at the start, but it does not live for the program",
                    var.name
                ));
            }
            if *dims != Some(bounds.len()) {
                v.problem(format!(
                    "{}: {} pairs of bounds at the start for {dims:?} dimensions",
                    var.name,
                    bounds.len()
                ));
            }
        }
        // Only a listed array (`STATIC a()`) may be without its number of dimensions; one made on each call
        // has it from the statement that made it. The emitter makes a descriptor of one dimension for none.
        if matches!(var.shape, Shape::Dynamic { dims: None, .. }) && matches!(var.storage, Storage::Local(_)) {
            v.problem(format!(
                "{}: an array made on each call without its number of dimensions",
                var.name
            ));
        }
    }
    v.body(&p.main, None);
    for (i, proc) in p.procs.iter().enumerate() {
        v.body(&proc.body, Some(ProcId(qb64rust_base::to_u32(i))));
    }
    if v.problems.is_empty() { Ok(()) } else { Err(v.problems) }
}

/// The type of a place, `None` where the place names something that does not exist.
fn place_ty_checked(p: &Program, place: &Place) -> Option<Ty> {
    match place {
        Place::Var(v)
        | Place::Element {
            array: ArrayRef::Var(v),
            ..
        } => p.vars.get(v.0 as usize).map(|v| v.ty),
        Place::Member { base, member }
        | Place::Element {
            array: ArrayRef::Member { base, member },
            ..
        } => member_checked(p, base, *member).map(|m| m.ty),
    }
}

/// The member `member` of the place `base`, `None` where the place is of no user type or the type has no such
/// member.
fn member_checked<'p>(p: &'p Program, base: &Place, member: MemberId) -> Option<&'p Member> {
    match place_ty_checked(p, base)? {
        Ty::User(t) => p.types.get(t.0 as usize)?.members.get(member.0 as usize),
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
        | Ty::F32
        | Ty::F64
        | Ty::F80
        | Ty::Str
        | Ty::FixedStr(_) => None,
    }
}

struct Validator<'p> {
    p: &'p Program,
    problems: Vec<String>,
}

/// Where a check is made: the body's name and the statement's index and line.
struct At<'a> {
    body: &'a str,
    stmt: usize,
    line: u32,
}

impl std::fmt::Display for At<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: statement {} (line {})", self.body, self.stmt, self.line)
    }
}

impl Validator<'_> {
    fn problem(&mut self, s: String) {
        self.problems.push(s);
    }

    fn var_exists(&mut self, id: VarId, at: &str) -> bool {
        let ok = (id.0 as usize) < self.p.vars.len();
        if !ok {
            self.problem(format!("{at}: variable {} does not exist", id.0));
        }
        ok
    }

    fn body(&mut self, b: &Body, owner: Option<ProcId>) {
        let name = match owner {
            None => "main".to_string(),
            Some(q) => self.p.proc(q).name.clone(),
        };
        for (i, l) in b.labels.iter().enumerate() {
            if l.at > b.stmts.len() {
                let shown = l.name.as_deref().unwrap_or("(made)");
                self.problem(format!("{name}: label {i} {shown} (line {}) is never placed", l.line));
            }
        }
        for (k, s) in b.stmts.iter().enumerate() {
            let at = At {
                body: &name,
                stmt: k,
                line: s.line,
            };
            for op in &s.ops {
                self.op(op, b, owner, &at);
            }
        }
    }

    fn own_label(&mut self, l: LabelId, b: &Body, at: &At, what: &str) {
        if l.0 as usize >= b.labels.len() {
            self.problem(format!("{at}: {what} to label {}, which this body does not have", l.0));
        }
    }

    fn main_label(&mut self, l: LabelId, at: &At, what: &str) {
        if l.0 as usize >= self.p.main.labels.len() {
            self.problem(format!(
                "{at}: {what} to label {}, which the main module does not have",
                l.0
            ));
        }
    }

    fn op(&mut self, op: &Op, b: &Body, owner: Option<ProcId>, at: &At) {
        match op {
            Op::SelectConsole => {
                if !self.p.settings.console {
                    self.problem(format!("{at}: the console is selected in a program without one"));
                }
            }
            // A block, an `_OFFSET` offset (and count), and a variable or a value.
            Op::MemGet { block, offset, target } => {
                self.mem_at(block, offset, owner, at);
                self.mem_bytes(target, owner, at);
            }
            Op::MemPut { block, offset, value } => {
                self.mem_at(block, offset, owner, at);
                self.mem_value(value, owner, at);
            }
            Op::MemFill {
                block,
                offset,
                bytes,
                value,
            } => {
                self.mem_at(block, offset, owner, at);
                if bytes.ty != Ty::Off {
                    self.problem(format!("{at}: a count of bytes of type {:?}", bytes.ty));
                }
                self.value(bytes, owner, at);
                self.mem_value(value, owner, at);
            }
            Op::End | Op::System | Op::Exit | Op::SetHandler(None) | Op::Return(None) => {}
            Op::Resume(Resume::Retry | Resume::Next) => {}
            Op::SetHandler(Some(l)) => self.main_label(*l, at, "handler"),
            Op::Resume(Resume::To(l)) => self.main_label(*l, at, "resume"),
            Op::Return(Some(l)) => self.main_label(*l, at, "return"),
            Op::Jump(l) => self.own_label(*l, b, at, "jump"),
            Op::Gosub(l) => self.own_label(*l, b, at, "gosub"),
            Op::Branch { cond, to, .. } => {
                self.own_label(*to, b, at, "branch");
                self.value(cond, owner, at);
            }
            Op::Assign { place, value } => {
                self.place(place, owner, at);
                self.value(value, owner, at);
                // A value of a user type only from a built-in: a block, or what `_MEMGET` reads; into a place of
                // its type (two places of one are a `CopyWhole`).
                if let Ty::User(_) = value.ty {
                    let built_in = match &value.kind {
                        ExprKind::MemOf { .. } | ExprKind::MemAt { .. } | ExprKind::MemRead { .. } => true,
                        ExprKind::Call { builtin, .. } => builtin.get().ret == Some("_MEM"),
                        ExprKind::Int(_)
                        | ExprKind::Float(_)
                        | ExprKind::Str(_)
                        | ExprKind::Load(_)
                        | ExprKind::Bound { .. }
                        | ExprKind::VarPtr { .. }
                        | ExprKind::VarSeg { .. }
                        | ExprKind::Block { .. }
                        | ExprKind::Convert { .. }
                        | ExprKind::Binary { .. }
                        | ExprKind::Unary { .. }
                        | ExprKind::Concat(..)
                        | ExprKind::StrCompare { .. }
                        | ExprKind::Choice { .. }
                        | ExprKind::CallProc { .. }
                        | ExprKind::AddressOf(_) => false,
                    };
                    let to = place_ty_checked(self.p, place);
                    if !built_in || to.is_some_and(|t| t != value.ty) {
                        self.problem(format!("{at}: a store of a {:?} value into a {to:?}", value.ty));
                    }
                }
            }
            Op::AssignAll(stores) => {
                for (place, value) in stores {
                    self.place(&Place::Var(*place), owner, at);
                    self.value(value, owner, at);
                }
            }
            // Two places of one user type.
            Op::CopyWhole { from, to } => {
                self.place(to, owner, at);
                self.place(from, owner, at);
                if let (Some(tt), Some(tf)) = (place_ty_checked(self.p, to), place_ty_checked(self.p, from))
                    && !(matches!(tt, Ty::User(_)) && tt == tf)
                {
                    self.problem(format!("{at}: a CopyWhole from a {tf:?} to a {tt:?}"));
                }
            }
            Op::Raise(v) => self.value(v, owner, at),
            // An allocation names an array of either kind (of a static one it raises an error when it runs), with
            // one pair of `I64` bounds per dimension.
            Op::Allocate { array, bounds, .. } => {
                if self.array_use(*array, owner, at, "Allocate")
                    && let Some(n) = self.p.var(*array).dim_count()
                    && n != bounds.len()
                {
                    let name = &self.p.var(*array).name;
                    self.problem(format!(
                        "{at}: {} pairs of bounds for {name} of {n} dimensions",
                        bounds.len()
                    ));
                }
                for bound in bounds.iter().flat_map(|(lower, upper)| [lower, upper]) {
                    if bound.ty != Ty::I64 {
                        self.problem(format!("{at}: a bound of type {:?}", bound.ty));
                    }
                    self.value(bound, owner, at);
                }
            }
            // A static array, or a member array (whose bounds never change: no Allocate, Release or EraseParam
            // names one, their arrays being variables).
            Op::Clear { array } => match array {
                ArrayRef::Var(array) => {
                    if self.array_use(*array, owner, at, "Clear")
                        && !matches!(self.p.var(*array).shape, Shape::Static { .. })
                    {
                        let name = &self.p.var(*array).name;
                        self.problem(format!("{at}: Clear of {name}, which is no static array"));
                    }
                }
                ArrayRef::Member { .. } => {
                    self.array_ref(array, owner, at, "Clear");
                }
            },
            Op::Release { array } => {
                if self.array_use(*array, owner, at, "Release") {
                    let v = self.p.var(*array);
                    if !matches!(v.shape, Shape::Dynamic { .. }) {
                        self.problem(format!("{at}: Release of {}, which is no dynamic array", v.name));
                    } else if matches!(v.storage, Storage::Param(_)) {
                        // The array passed may be a static one.
                        self.problem(format!("{at}: Release of the array parameter {}", v.name));
                    }
                }
            }
            Op::EraseParam { array } => {
                if self.array_use(*array, owner, at, "EraseParam")
                    && !matches!(self.p.var(*array).storage, Storage::Param(_))
                {
                    let name = &self.p.var(*array).name;
                    self.problem(format!("{at}: EraseParam of {name}, which is no parameter"));
                }
            }
            Op::Print { to, items, .. } => {
                if let PrintTo::File(n) = to {
                    self.file_number(n, owner, at);
                }
                self.print_items(items, false, owner, at);
            }
            // One unit with its own error checks (the crate documentation's rules for a pending error do not
            // apply inside it): a string format, at least one value, each value of the kind its variant says.
            Op::PrintUsing {
                to,
                before,
                format,
                values,
                ..
            } => {
                if let PrintTo::File(n) = to {
                    self.file_number(n, owner, at);
                }
                self.print_items(before, false, owner, at);
                if format.ty != Ty::Str {
                    self.problem(format!("{at}: the format of a PrintUsing is a {:?}", format.ty));
                }
                self.value(format, owner, at);
                if values.is_empty() {
                    self.problem(format!("{at}: a PrintUsing without a value"));
                }
                for v in values {
                    match v {
                        UsingValue::Str(e) | UsingValue::Blanks(e) => {
                            if e.ty != Ty::Str {
                                self.problem(format!("{at}: a string value of a PrintUsing is a {:?}", e.ty));
                            }
                            self.value(e, owner, at);
                        }
                        UsingValue::Num { value, as_ } => {
                            let entry = matches!(as_, Ty::F32 | Ty::F64 | Ty::F80 | Ty::I64 | Ty::U64);
                            if !entry || !value.ty.is_numeric() || value.ty.is_float() != as_.is_float() {
                                self.problem(format!(
                                    "{at}: a number of a PrintUsing, a {:?}, formatted as {as_:?}",
                                    value.ty
                                ));
                            }
                            self.value(value, owner, at);
                        }
                    }
                }
            }
            Op::Write { to, items, .. } => {
                if let Some(n) = to {
                    self.file_number(n, owner, at);
                }
                self.print_items(items, true, owner, at);
            }
            // A file number, and at least one part: a place of a variable-length string and a width of type I32.
            Op::Field { file, parts } => {
                self.file_number(file, owner, at);
                if parts.is_empty() {
                    self.problem(format!("{at}: a Field without a part"));
                }
                for (place, width) in parts {
                    self.place(place, owner, at);
                    if place_ty_checked(self.p, place).is_some_and(|t| t != Ty::Str) {
                        self.problem(format!("{at}: a part of a Field is no variable-length string"));
                    }
                    if width.ty != Ty::I32 {
                        self.problem(format!("{at}: a width of a Field is a {:?}", width.ty));
                    }
                    self.value(width, owner, at);
                }
            }
            // A string place of either kind and a string value.
            Op::Lset { target, value, .. } => {
                self.place(target, owner, at);
                if place_ty_checked(self.p, target).is_some_and(|t| !t.is_string()) {
                    self.problem(format!("{at}: the target of an Lset is no string"));
                }
                if value.ty != Ty::Str {
                    self.problem(format!("{at}: an Lset stores a value of type {:?}", value.ty));
                }
                self.value(value, owner, at);
            }
            Op::Input { from, line, targets } => {
                match from {
                    Source::File(n) => self.file_number(n, owner, at),
                    Source::Console { .. } => {}
                }
                if targets.is_empty() {
                    self.problem(format!("{at}: an Input without a target"));
                }
                if *line && targets.len() != 1 {
                    self.problem(format!("{at}: a LineInput with {} targets", targets.len()));
                }
                for place in targets {
                    self.place(place, owner, at);
                    match place_ty_checked(self.p, place) {
                        Some(Ty::User(_)) => self.problem(format!("{at}: an Input target of a user type")),
                        Some(t) if *line && !t.is_string() => {
                            self.problem(format!("{at}: a LineInput target of type {t:?}"));
                        }
                        Some(_) | None => {}
                    }
                    if place.through_member() {
                        self.problem(format!(
                            "{at}: an Input target reached through a member with an index on the way"
                        ));
                    }
                }
            }
            Op::Read(targets) => {
                if targets.is_empty() {
                    self.problem(format!("{at}: a Read without a target"));
                }
                for place in targets {
                    self.place(place, owner, at);
                    if let Some(Ty::User(_)) = place_ty_checked(self.p, place) {
                        self.problem(format!("{at}: a Read target of a user type"));
                    }
                    if place.through_member() {
                        self.problem(format!(
                            "{at}: a Read target reached through a member with an index on the way"
                        ));
                    }
                }
            }
            Op::Restore { at: item, label } => {
                if *item > self.p.data.len() {
                    self.problem(format!("{at}: a Restore to item {item} of {}", self.p.data.len()));
                }
                if label.is_none() && *item != 0 {
                    self.problem(format!("{at}: a Restore to item {item} without a label"));
                }
            }
            Op::Call { proc, args } => self.call(*proc, args, owner, at),
            Op::Builtin { id, args } => self.builtin(*id, args, owner, at),
        }
    }

    /// The items of a print or (`write`) a write, which has no zones.
    fn print_items(&mut self, items: &[PrintItem], write: bool, owner: Option<ProcId>, at: &At) {
        for i in items {
            match i {
                PrintItem::Str(v) | PrintItem::Num(v) => self.value(v, owner, at),
                PrintItem::Zone => {
                    if write {
                        self.problem(format!("{at}: a zone in a Write"));
                    }
                }
            }
        }
    }

    /// A file number: a value of type `I32`.
    fn file_number(&mut self, n: &Expr, owner: Option<ProcId>, at: &At) {
        if n.ty != Ty::I32 {
            self.problem(format!("{at}: a file number of type {:?}", n.ty));
        }
        self.value(n, owner, at);
    }

    /// A built-in statement: a form `sema` compiles, with one slot per argument or choice of its template, each
    /// filled as its rule allows (a value for an argument, a word the choice has, absent only where optional).
    fn builtin(&mut self, id: BuiltinId, args: &[StmtArg], owner: Option<ProcId>, at: &At) {
        let name = id.get().name;
        let Some(form) = stmt_find(id) else {
            self.problem(format!("{at}: {name} is no built-in statement that is compiled"));
            return;
        };
        match form.rule {
            // `File`: the template's slots, the third the bytes of a place or of a whole array, or nothing.
            StmtRule::Plain | StmtRule::Graphics | StmtRule::File => {}
            // Any number of file numbers.
            StmtRule::Close => {
                for (k, arg) in args.iter().enumerate() {
                    match arg {
                        StmtArg::Value(v) => self.file_number(v, owner, at),
                        StmtArg::Place(_)
                        | StmtArg::Block { .. }
                        | StmtArg::Bytes(_)
                        | StmtArg::Word(_)
                        | StmtArg::Absent => {
                            self.problem(format!("{at}: slot {} of {name} is no value", k + 1));
                        }
                    }
                }
                return;
            }
            // Two places of one kind: strings, one user type, or numbers of one width (the emitter picks the
            // runtime's entry by the first place's type).
            StmtRule::Swap => {
                let [StmtArg::Place(a), StmtArg::Place(b)] = args else {
                    self.problem(format!("{at}: {name} without its two places"));
                    return;
                };
                self.place(a, owner, at);
                self.place(b, owner, at);
                if let (Some(ta), Some(tb)) = (place_ty_checked(self.p, a), place_ty_checked(self.p, b)) {
                    let fits = match (ta, tb) {
                        (Ty::Bit { .. }, _) | (_, Ty::Bit { .. }) => false,
                        (Ty::User(x), Ty::User(y)) => x == y,
                        (Ty::User(_), _) | (_, Ty::User(_)) => false,
                        (x, y) if x.is_string() || y.is_string() => x.is_string() && y.is_string(),
                        (x, y) => crate::size_of(&self.p.types, x) == crate::size_of(&self.p.types, y),
                    };
                    if !fits {
                        self.problem(format!("{at}: {name} of a {ta:?} and a {tb:?}"));
                    }
                }
                return;
            }
            // A string place, a start, a length or nothing, a string value.
            StmtRule::MidAssign => {
                let [StmtArg::Place(target), StmtArg::Value(start), length, StmtArg::Value(v)] = args else {
                    self.problem(format!(
                        "{at}: {name} without a place, a start, a length slot and a value"
                    ));
                    return;
                };
                self.place(target, owner, at);
                if place_ty_checked(self.p, target).is_some_and(|t| !t.is_string()) {
                    self.problem(format!("{at}: the target of {name} is no string"));
                }
                self.value(start, owner, at);
                match length {
                    StmtArg::Value(l) => self.value(l, owner, at),
                    StmtArg::Absent => {}
                    StmtArg::Place(_) | StmtArg::Block { .. } | StmtArg::Bytes(_) | StmtArg::Word(_) => {
                        self.problem(format!("{at}: slot 3 of {name} is no value"));
                    }
                }
                if v.ty != Ty::Str {
                    self.problem(format!("{at}: {name} stores a value of type {:?}", v.ty));
                }
                self.value(v, owner, at);
                return;
            }
        }
        let slots = stmt_slots(id);
        if slots.len() != args.len() {
            let msg = format!("{at}: {name} with {} slots for {}", args.len(), slots.len());
            self.problem(msg);
            return;
        }
        for (k, (slot, arg)) in slots.iter().zip(args).enumerate() {
            let k = k + 1;
            match (slot, arg) {
                (StmtSlot::Arg { slot, .. }, StmtArg::Value(v)) => {
                    let string = match slot {
                        Slot::Str => true,
                        Slot::Long
                        | Slot::Int64
                        | Slot::Double
                        | Slot::Float
                        | Slot::AnyNumeric
                        | Slot::UInt64
                        | Slot::ULong
                        | Slot::Offset => false,
                        // A number or a string: whichever the value is.
                        Slot::ColorOrTile => v.ty == Ty::Str,
                        Slot::Block | Slot::Bytes => {
                            self.problem(format!("{at}: slot {k} of {name} takes bytes, not a value"));
                            v.ty == Ty::Str
                        }
                        Slot::Mem => {
                            self.problem(format!("{at}: slot {k} of {name} takes a block, not a value"));
                            v.ty == Ty::Str
                        }
                    };
                    if string != (v.ty == Ty::Str) {
                        self.problem(format!("{at}: slot {k} of {name} holds a value of type {:?}", v.ty));
                    }
                    self.value(v, owner, at);
                }
                // A block: a place of type `_MEM`.
                (StmtSlot::Arg { slot: Slot::Mem, .. }, StmtArg::Place(place)) => self.block_place(place, owner, at),
                (StmtSlot::Arg { .. }, StmtArg::Place(_)) => {
                    self.problem(format!("{at}: slot {k} of {name} holds a place"));
                }
                // The bytes of one place, for a file: of any type with a fixed size, or a variable-length string.
                (StmtSlot::Arg { slot, .. }, StmtArg::Bytes(place)) => {
                    if *slot != Slot::Bytes {
                        self.problem(format!("{at}: slot {k} of {name} holds a place's bytes"));
                    }
                    self.place(place, owner, at);
                    if let Some(t) = place_ty_checked(self.p, place)
                        && (matches!(t, Ty::Bit { .. }) || crate::holds_strings(&self.p.types, t))
                    {
                        self.problem(format!("{at}: the bytes of a {t:?}, which has no fixed size in bytes"));
                    }
                }
                // The bytes of an array, whole or from an element with an index per dimension: of numbers for
                // graphics; for a file the whole array, of anything with a fixed size.
                (StmtSlot::Arg { slot, .. }, StmtArg::Block { array, index }) => {
                    if !matches!(slot, Slot::Block | Slot::Bytes) {
                        self.problem(format!("{at}: slot {k} of {name} holds an array's bytes"));
                    }
                    self.array_ref(&ArrayRef::Var(*array), owner, at, "bytes");
                    let a = self.p.var(*array);
                    let fits = if *slot == Slot::Bytes {
                        index.is_empty()
                            && a.ty != Ty::Str
                            && !matches!(a.ty, Ty::Bit { .. })
                            && !crate::holds_strings(&self.p.types, a.ty)
                    } else {
                        a.ty.is_int() || a.ty.is_float()
                    };
                    if !fits {
                        self.problem(format!("{at}: the bytes of {}, an array of {:?}", a.name, a.ty));
                    }
                    if !index.is_empty() && a.dim_count().is_some_and(|n| n != index.len()) {
                        self.problem(format!("{at}: {} indexes for {}", index.len(), a.name));
                    }
                    for i in index {
                        self.value(i, owner, at);
                    }
                }
                (StmtSlot::Choice { alts, .. }, StmtArg::Word(w)) => {
                    if w >= alts {
                        self.problem(format!("{at}: slot {k} of {name} names word {w} of {alts}"));
                    }
                }
                (StmtSlot::Arg { optional, .. } | StmtSlot::Choice { optional, .. }, StmtArg::Absent) => {
                    if !optional {
                        self.problem(format!("{at}: slot {k} of {name} is required"));
                    }
                }
                (StmtSlot::Arg { .. }, StmtArg::Word(_)) => {
                    self.problem(format!("{at}: slot {k} of {name} is an argument, not a choice"));
                }
                (
                    StmtSlot::Choice { .. },
                    StmtArg::Value(_) | StmtArg::Place(_) | StmtArg::Block { .. } | StmtArg::Bytes(_),
                ) => {
                    self.problem(format!("{at}: slot {k} of {name} is a choice, not an argument"));
                }
            }
        }
    }

    fn call(&mut self, proc: ProcId, args: &[Arg], owner: Option<ProcId>, at: &At) {
        match self.p.procs.get(proc.0 as usize) {
            None => self.problem(format!("{at}: call of procedure {}, which does not exist", proc.0)),
            Some(q) if q.params.len() != args.len() => self.problem(format!(
                "{at}: call of {} with {} arguments for {} parameters",
                q.name,
                args.len(),
                q.params.len()
            )),
            Some(_) => {}
        }
        let params = self.p.procs.get(proc.0 as usize).map_or(&[][..], |q| &q.params[..]);
        for (k, a) in args.iter().enumerate() {
            // An array parameter takes a whole array and nothing else does; the two have one number of
            // dimensions where both have one.
            let param = params.get(k).and_then(|&v| self.p.vars.get(v.0 as usize));
            let to_array = param.is_some_and(Var::is_array);
            match a {
                Arg::Ref(place) => self.place(place, owner, at),
                Arg::Temp(v) => self.value(v, owner, at),
                Arg::Array(array) => {
                    if let Some((name, n)) = self.array_ref(array, owner, at, "an array argument")
                        && let Some(param) = param.filter(|_| to_array)
                        && let (Some(n), Some(m)) = (n, param.dim_count())
                        && n != m
                    {
                        self.problem(format!(
                            "{at}: {name} of {n} dimensions passed to {} of {m}",
                            param.name
                        ));
                    }
                }
            }
            // A parameter of a user type takes a place of exactly that type, by reference.
            if let Some(param) = param.filter(|v| !to_array && matches!(v.ty, Ty::User(_)))
                && !matches!(a, Arg::Ref(place) if place_ty_checked(self.p, place) == Some(param.ty))
            {
                self.problem(format!(
                    "{at}: argument {} is no place of the user type of {}",
                    k + 1,
                    param.name
                ));
            }
            if param.is_some() && to_array != matches!(a, Arg::Array(_)) {
                self.problem(format!(
                    "{at}: argument {} is {} for a parameter that is {}",
                    k + 1,
                    if to_array { "no whole array" } else { "a whole array" },
                    if to_array { "an array" } else { "no array" }
                ));
            }
        }
    }

    /// A place: a scalar variable, an element of an array with one `I64` index per dimension, or a member that the
    /// base's user type has and that is no member array.
    fn place(&mut self, place: &Place, owner: Option<ProcId>, at: &At) {
        match place {
            Place::Var(v) => {
                if self.var_use(*v, owner, at) && self.p.var(*v).is_array() {
                    let name = &self.p.var(*v).name;
                    self.problem(format!("{at}: the array {name} used as a scalar"));
                }
            }
            Place::Element { array, index } => {
                match self.array_ref(array, owner, at, "an element") {
                    Some((_, Some(n))) if n == index.len() => {}
                    Some((name, Some(n))) => {
                        let msg = format!("{at}: {} indexes for {name} of {n} dimensions", index.len());
                        self.problem(msg);
                    }
                    Some((name, None)) => {
                        self.problem(format!("{at}: an element of {name}, whose dimensions are not known"));
                    }
                    // Reported by `array_ref`.
                    None => {}
                }
                for i in index {
                    if i.ty != Ty::I64 {
                        self.problem(format!("{at}: an index of type {:?}", i.ty));
                    }
                    self.value(i, owner, at);
                }
            }
            Place::Member { base, member } => {
                self.place(base, owner, at);
                match member_checked(self.p, base, *member) {
                    Some(m) if m.is_array() => {
                        self.problem(format!("{at}: the member array {} used as a single value", m.name));
                    }
                    Some(_) => {}
                    None => self.problem(format!("{at}: member {} of a place without it", member.0)),
                }
            }
        }
    }

    /// The array an operation `what` names: an array variable of this body, or a member array of a place of a
    /// user type. Gives its name and, where it is known, its number of dimensions; `None` after a problem.
    fn array_ref(
        &mut self,
        array: &ArrayRef,
        owner: Option<ProcId>,
        at: &At,
        what: &str,
    ) -> Option<(String, Option<usize>)> {
        match array {
            ArrayRef::Var(v) => {
                if !self.array_use(*v, owner, at, what) {
                    return None;
                }
                let var = self.p.var(*v);
                Some((var.name.clone(), var.dim_count()))
            }
            ArrayRef::Member { base, member } => {
                self.place(base, owner, at);
                match member_checked(self.p, base, *member) {
                    Some(m) if m.is_array() => Some((m.name.clone(), Some(m.bounds.len()))),
                    Some(m) => {
                        self.problem(format!("{at}: {what} of the member {}, which is no array", m.name));
                        None
                    }
                    None => {
                        self.problem(format!("{at}: member {} of a place without it", member.0));
                        None
                    }
                }
            }
        }
    }

    /// The array an operation `what` names: a variable of this body that is an array. Returns whether it is one.
    fn array_use(&mut self, id: VarId, owner: Option<ProcId>, at: &At, what: &str) -> bool {
        if !self.var_use(id, owner, at) {
            return false;
        }
        let v = self.p.var(id);
        if !v.is_array() {
            self.problem(format!("{at}: {what} of {}, which is no array", v.name));
        }
        v.is_array()
    }

    /// A variable used in a body must exist and belong to it: a procedure's own variables (and the lowering's
    /// temporaries) only in that procedure, global ones anywhere. Returns whether it exists.
    fn var_use(&mut self, id: VarId, owner: Option<ProcId>, at: &At) -> bool {
        if !self.var_exists(id, &at.to_string()) {
            return false;
        }
        let v = self.p.var(id);
        let home = match v.storage {
            Storage::Global => return true,
            Storage::Temp(q) => q,
            Storage::Static(q) | Storage::Local(q) | Storage::Param(q) | Storage::Result(q) => Some(q),
        };
        if home != owner {
            let home_name = home.map_or("main", |q| self.p.proc(q).name.as_str());
            self.problem(format!("{at}: uses {} of {home_name}", v.name));
        }
        true
    }

    fn value(&mut self, v: &Expr, owner: Option<ProcId>, at: &At) {
        match &v.kind {
            ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) => {}
            ExprKind::Load(place) => self.place(place, owner, at),
            // A block of a reference, typed `_MEM`: never of a variable-length string or a `_BIT`.
            ExprKind::MemOf { of, .. } => {
                if !self.p.is_mem(v.ty) {
                    self.problem(format!("{at}: a block typed {:?}", v.ty));
                }
                self.reference(of, owner, at, false);
            }
            // An address, typed `_UNSIGNED _OFFSET`: never of a `_BIT`.
            ExprKind::AddressOf(of) => {
                if v.ty != Ty::UOff {
                    self.problem(format!("{at}: an address typed {:?}", v.ty));
                }
                self.reference(of, owner, at, true);
            }
            // A value read from a block: of a number (not `_BIT`), a fixed-length string (a `STRING` value) or a
            // user type, at an `_OFFSET`.
            ExprKind::MemRead { block, offset, of } => {
                let held = if let Ty::FixedStr(_) = of { Ty::Str } else { *of };
                if matches!(of, Ty::Bit { .. } | Ty::Str) || v.ty != held {
                    self.problem(format!("{at}: a value read as {of:?} typed {:?}", v.ty));
                }
                self.mem_at(block, offset, owner, at);
            }
            // A block at an address: an offset and a size, both `_OFFSET` values.
            ExprKind::MemAt { offset, size } => {
                if !self.p.is_mem(v.ty) || !matches!(offset.ty, Ty::Off | Ty::UOff) || size.ty != Ty::Off {
                    self.problem(format!(
                        "{at}: a block at an address typed {:?}, from a {:?} and a {:?}",
                        v.ty, offset.ty, size.ty
                    ));
                }
                self.value(offset, owner, at);
                self.value(size, owner, at);
            }
            ExprKind::Bound { array, dim, .. } => {
                self.array_ref(array, owner, at, "bound");
                if let Some(d) = dim {
                    self.value(d, owner, at);
                }
            }
            // An offset or a segment is of an array in conventional memory; an offset has an index per dimension.
            ExprKind::VarPtr { array, .. } | ExprKind::VarSeg { array } => {
                self.array_ref(&ArrayRef::Var(*array), owner, at, "address");
                let a = self.p.var(*array);
                if !a.conventional {
                    self.problem(format!(
                        "{at}: an address of {}, which is not in conventional memory",
                        a.name
                    ));
                }
                if let ExprKind::VarPtr { index, .. } = &v.kind {
                    if a.dim_count().is_some_and(|n| n != index.len()) {
                        self.problem(format!("{at}: {} indexes for {}", index.len(), a.name));
                    }
                    for i in index {
                        self.value(i, owner, at);
                    }
                }
            }
            // An array's bytes have no value: they stand only in a function's `Slot::Block` ([`Validator::bytes`]).
            ExprKind::Block { array, .. } => {
                let name = &self.p.var(*array).name;
                self.problem(format!("{at}: the bytes of {name} as a value"));
            }
            ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => self.value(from, owner, at),
            ExprKind::Binary { lhs, rhs, .. } | ExprKind::Concat(lhs, rhs) | ExprKind::StrCompare { lhs, rhs, .. } => {
                self.value(lhs, owner, at);
                self.value(rhs, owner, at);
            }
            // The condition an integer; the parts both strings, as the node is, or both numbers, as the node is.
            ExprKind::Choice { condition, then, else_ } => {
                if !condition.ty.is_int() {
                    self.problem(format!("{at}: the condition of a choice is a {:?}", condition.ty));
                }
                let string = v.ty == Ty::Str;
                for part in [then, else_] {
                    if (part.ty == Ty::Str) != string || !(string || part.ty.is_numeric() && v.ty.is_numeric()) {
                        self.problem(format!("{at}: a part of a choice typed {:?} is a {:?}", v.ty, part.ty));
                    }
                }
                for part in [condition, then, else_] {
                    self.value(part, owner, at);
                }
            }
            ExprKind::Call { builtin: id, args } => {
                // The table's slots, or as many as the built-in's rule gives it (`ASC` has two).
                let slots = qb64rust_sema::builtins::slot_count(*id);
                if args.len() != slots {
                    self.problem(format!(
                        "{at}: {} with {} argument slots for {slots}",
                        id.get().name,
                        args.len()
                    ));
                }
                // A `Slot::Block` holds an array's bytes and nothing else; every other slot a value.
                let kinds: Vec<Slot> = qb64rust_sema::builtins::find(*id)
                    .map(|s| {
                        qb64rust_sema::builtins::slots(s)
                            .into_iter()
                            .map(|(slot, _)| slot)
                            .collect()
                    })
                    .unwrap_or_default();
                for (k, a) in args.iter().enumerate() {
                    let Some(a) = a else { continue };
                    match (&a.kind, kinds.get(k)) {
                        (ExprKind::Block { array, index }, Some(Slot::Block)) => {
                            self.bytes(a, *array, index, owner, at);
                        }
                        (_, Some(Slot::Block)) => self.problem(format!(
                            "{at}: argument {} of {} takes an array's bytes, not a value",
                            k + 1,
                            id.get().name
                        )),
                        // A block: a place of type `_MEM`, loaded as itself.
                        (ExprKind::Load(place), Some(Slot::Mem)) => self.block_place(place, owner, at),
                        (_, Some(Slot::Mem)) => self.problem(format!(
                            "{at}: argument {} of {} takes a block, not a value",
                            k + 1,
                            id.get().name
                        )),
                        (_, _) => self.value(a, owner, at),
                    }
                }
            }
            ExprKind::CallProc { proc, args } => self.call(*proc, args, owner, at),
        }
    }

    /// What `_MEM`, `_MEMELEMENT` (`address` false) or `_OFFSET` names: a place or a whole array of this body,
    /// never of `_BIT`; for a block, never of variable-length strings either.
    fn reference(&mut self, of: &Reference, owner: Option<ProcId>, at: &At, address: bool) {
        let ty = match of {
            Reference::Place(place) => {
                self.place(place, owner, at);
                place_ty_checked(self.p, place)
            }
            Reference::Array(array) => {
                self.array_ref(array, owner, at, "reference");
                Some(self.p.array_ty(array))
            }
        };
        if let Some(t) = ty
            && (matches!(t, Ty::Bit { .. }) || (!address && t == Ty::Str))
        {
            self.problem(format!("{at}: a reference to a {t:?}"));
        }
    }

    /// The block and the offset of `_MEMGET`, `_MEMPUT` and `_MEMFILL`: a place of type `_MEM` and an `_OFFSET`.
    fn mem_at(&mut self, block: &Place, offset: &Expr, owner: Option<ProcId>, at: &At) {
        self.block_place(block, owner, at);
        if offset.ty != Ty::Off {
            self.problem(format!("{at}: an offset of type {:?}", offset.ty));
        }
        self.value(offset, owner, at);
    }

    /// The variable `_MEMGET` reads into, or `_MEMPUT` and `_MEMFILL` take the bytes of: never a `_BIT`, and a
    /// variable-length string only as a variable.
    fn mem_bytes(&mut self, of: &Reference, owner: Option<ProcId>, at: &At) {
        self.reference(of, owner, at, true);
        let string = match of {
            Reference::Place(place) => {
                place_ty_checked(self.p, place) == Some(Ty::Str) && !matches!(place, Place::Var(_))
            }
            Reference::Array(array) => self.p.array_ty(array) == Ty::Str,
        };
        if string {
            self.problem(format!("{at}: the bytes of an element, member or array of strings"));
        }
    }

    /// What `_MEMPUT` writes or `_MEMFILL` fills with.
    fn mem_value(&mut self, value: &MemValue, owner: Option<ProcId>, at: &At) {
        match value {
            MemValue::Typed { value, ty } => {
                if !(ty.is_numeric() && !matches!(ty, Ty::Bit { .. }) && value.ty == ty.held_value()) {
                    self.problem(format!("{at}: a value of type {:?} as a {ty:?}", value.ty));
                }
                self.value(value, owner, at);
            }
            MemValue::Bytes(of) => self.mem_bytes(of, owner, at),
            MemValue::Str(s) => {
                if s.ty != Ty::Str {
                    self.problem(format!("{at}: a string value of type {:?}", s.ty));
                }
                self.value(s, owner, at);
            }
        }
    }

    /// A block passed to a built-in ([`Slot::Mem`]): a place of the predefined type `_MEM`.
    fn block_place(&mut self, place: &Place, owner: Option<ProcId>, at: &At) {
        self.place(place, owner, at);
        if let Some(t) = place_ty_checked(self.p, place)
            && !self.p.is_mem(t)
        {
            self.problem(format!("{at}: a block that is a place of type {t:?}"));
        }
    }

    /// The array of `_UCHARPOS`, whole or from an element with an index per dimension: of 32-bit integers, typed as
    /// its elements.
    fn bytes(&mut self, v: &Expr, array: VarId, index: &[Expr], owner: Option<ProcId>, at: &At) {
        self.array_ref(&ArrayRef::Var(array), owner, at, "bytes");
        let a = self.p.var(array);
        if !matches!(a.ty, Ty::I32 | Ty::U32) || v.ty != a.ty {
            self.problem(format!(
                "{at}: the bytes of {}, an array of {:?}, as {:?}",
                a.name, a.ty, v.ty
            ));
        }
        if !index.is_empty() && a.dim_count().is_some_and(|n| n != index.len()) {
            self.problem(format!("{at}: {} indexes for {}", index.len(), a.name));
        }
        for i in index {
            self.value(i, owner, at);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::{
        AllocMode, Arg, ArrayRef, Body, Expr, ExprKind, Label, LabelId, MemberId, Op, Place, Proc, ProcId, ProcKind,
        Program, Resume, Settings, Shape, Stmt, Storage, Ty, Var, VarId,
    };
    use qb64rust_base::{FileId, Span};

    fn stmt(ops: Vec<Op>) -> Stmt {
        Stmt {
            span: Span {
                file: FileId(0),
                start: 0,
                end: 0,
            },
            line: 1,
            ops,
            may_raise: false,
        }
    }

    fn label(at: usize) -> Label {
        Label {
            name: None,
            line: 1,
            file: None,
            at,
        }
    }

    fn var(name: &str, storage: Storage) -> Var {
        Var {
            name: name.into(),
            ty: Ty::I32,
            storage,
            shape: crate::Shape::Scalar,
            conventional: false,
        }
    }

    fn expr(ty: Ty, kind: ExprKind) -> Expr {
        Expr {
            span: Span::new(FileId(0), 0, 0),
            ty,
            qb: ty,
            kind,
        }
    }

    fn load(id: u32) -> Expr {
        expr(Ty::I32, ExprKind::Load(Place::Var(VarId(id))))
    }

    /// A program with a main module of one labelled statement and a SUB `S (P)` with a local `L`.
    fn program() -> Program {
        Program {
            vars: vec![
                var("G", Storage::Global),
                var("P", Storage::Param(ProcId(0))),
                var("L", Storage::Local(ProcId(0))),
            ],
            procs: vec![Proc {
                name: "S".into(),
                kind: ProcKind::Sub,
                params: vec![VarId(1)],
                result: None,
                body: Body {
                    labels: vec![label(0)],
                    stmts: vec![stmt(vec![Op::Assign {
                        place: Place::Var(VarId(2)),
                        value: load(1),
                    }])],
                },
                line: 3,
                end_line: 5,
            }],
            main: Body {
                labels: vec![label(0)],
                stmts: vec![stmt(vec![
                    Op::Jump(LabelId(0)),
                    Op::Call {
                        proc: ProcId(0),
                        args: vec![Arg::Ref(Place::Var(VarId(0)))],
                    },
                ])],
            },
            types: Vec::new(),
            data: Vec::new(),
            settings: Settings {
                console: true,
                ..Settings::default()
            },
        }
    }

    fn problems(p: &Program) -> Vec<String> {
        validate(p).err().unwrap_or_default()
    }

    #[test]
    fn a_sound_program_passes() {
        assert_eq!(problems(&program()), Vec::<String>::new());
    }

    #[test]
    fn settings() {
        let mut p = program();
        p.main.stmts[0].ops.push(Op::SelectConsole);
        assert_eq!(problems(&p), Vec::<String>::new());
        // A window and no console: sound, until the console is selected.
        p.settings.console = false;
        p.settings.window = true;
        assert_eq!(
            problems(&p),
            ["main: statement 0 (line 1): the console is selected in a program without one"]
        );
        p.main.stmts[0].ops.pop();
        assert_eq!(problems(&p), Vec::<String>::new());
        p.settings.window = false;
        assert_eq!(problems(&p), ["settings: neither a console nor a window"]);
    }

    #[test]
    fn label_never_placed() {
        let mut p = program();
        p.main.labels[0].at = usize::MAX;
        assert_eq!(problems(&p), ["main: label 0 (made) (line 1) is never placed"]);
    }

    #[test]
    fn jump_to_a_label_of_another_body() {
        let mut p = program();
        p.procs[0].body.stmts[0].ops.push(Op::Jump(LabelId(1)));
        assert_eq!(
            problems(&p),
            ["S: statement 0 (line 1): jump to label 1, which this body does not have"]
        );
    }

    #[test]
    fn main_module_targets() {
        let mut p = program();
        p.procs[0].body.stmts[0].ops.extend([
            Op::SetHandler(Some(LabelId(2))),
            Op::Resume(Resume::To(LabelId(0))),
            Op::Return(Some(LabelId(3))),
        ]);
        assert_eq!(
            problems(&p),
            [
                "S: statement 0 (line 1): handler to label 2, which the main module does not have",
                "S: statement 0 (line 1): return to label 3, which the main module does not have",
            ]
        );
    }

    #[test]
    fn missing_variable_and_procedure() {
        let mut p = program();
        p.main.stmts[0].ops.extend([
            Op::Raise(load(9)),
            Op::Call {
                proc: ProcId(4),
                args: vec![],
            },
        ]);
        assert_eq!(
            problems(&p),
            [
                "main: statement 0 (line 1): variable 9 does not exist",
                "main: statement 0 (line 1): call of procedure 4, which does not exist",
            ]
        );
    }

    #[test]
    fn wrong_argument_count() {
        let mut p = program();
        p.main.stmts[0].ops.push(Op::Call {
            proc: ProcId(0),
            args: vec![],
        });
        assert_eq!(
            problems(&p),
            ["main: statement 0 (line 1): call of S with 0 arguments for 1 parameters"]
        );
    }

    #[test]
    fn variable_of_another_body() {
        let mut p = program();
        p.main.stmts[0].ops.push(Op::Raise(load(2)));
        p.vars.push(var("T", Storage::Temp(None)));
        p.procs[0].body.stmts[0].ops.push(Op::Raise(load(3)));
        assert_eq!(
            problems(&p),
            [
                "main: statement 0 (line 1): uses L of S",
                "S: statement 0 (line 1): uses T of main",
            ]
        );
    }

    #[test]
    fn parameter_and_result_storage() {
        let mut p = program();
        p.vars[1].storage = Storage::Local(ProcId(0));
        p.procs[0].result = Some(VarId(0));
        assert_eq!(
            problems(&p),
            [
                "S: parameter 1 is not a parameter of this procedure",
                "S: the result is not a result of this procedure",
            ]
        );
    }

    /// A choice (`_IIF`): its condition is an integer, and its parts are both strings or both numbers, as the
    /// node is; each part is checked as a value of the statement's body.
    #[test]
    fn choice() {
        let choice = |ty: Ty, condition: Expr, then: Expr, else_: Expr| {
            let kind = ExprKind::Choice {
                condition: Box::new(condition),
                then: Box::new(then),
                else_: Box::new(else_),
            };
            Op::Raise(expr(ty, kind))
        };
        let text = || expr(Ty::Str, ExprKind::Str(b"a".to_vec()));
        let single = || expr(Ty::F32, ExprKind::Float("1.5E+0".into()));
        let mut p = program();
        p.main.stmts[0].ops.extend([
            choice(Ty::F32, load(0), load(0), single()),
            choice(Ty::Str, load(0), text(), text()),
        ]);
        assert_eq!(problems(&p), Vec::<String>::new());
        p.main.stmts[0].ops.extend([
            // A float as the condition, a string beside a number, a number in a string choice, a part that
            // names a variable of another body.
            choice(Ty::I32, single(), load(0), load(0)),
            choice(Ty::I32, load(0), text(), load(0)),
            choice(Ty::Str, load(0), text(), load(0)),
            choice(Ty::I32, load(0), load(0), load(2)),
        ]);
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: the condition of a choice is a F32"),
                format!("{at}: a part of a choice typed I32 is a Str"),
                format!("{at}: a part of a choice typed Str is a I32"),
                format!("{at}: uses L of S"),
            ]
        );
    }

    #[test]
    fn builtin_statement_slots() {
        use crate::StmtArg;
        let name = qb64rust_sema::builtins::stmt_lookup("NAME", false)[0].id;
        let text = |s: &[u8]| StmtArg::Value(expr(Ty::Str, ExprKind::Str(s.to_vec())));
        let mut p = program();
        p.main.stmts[0].ops.push(Op::Builtin {
            id: name,
            args: vec![text(b"a"), StmtArg::Word(0), text(b"b")],
        });
        assert_eq!(problems(&p), Vec::<String>::new());
        // A number in a string slot, a word the choice does not have, a required slot left out, a place.
        p.main.stmts[0].ops.push(Op::Builtin {
            id: name,
            args: vec![StmtArg::Value(load(0)), StmtArg::Word(1), StmtArg::Absent],
        });
        p.main.stmts[0].ops.push(Op::Builtin {
            id: name,
            args: vec![StmtArg::Place(Place::Var(VarId(0))), StmtArg::Absent, text(b"b")],
        });
        p.main.stmts[0].ops.push(Op::Builtin {
            id: name,
            args: vec![text(b"a")],
        });
        // A function's entry is no statement.
        p.main.stmts[0].ops.push(Op::Builtin {
            id: qb64rust_builtins::find_function(b"INSTR").unwrap(),
            args: vec![],
        });
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: slot 1 of Name holds a value of type I32"),
                format!("{at}: slot 2 of Name names word 1 of 1"),
                format!("{at}: slot 3 of Name is required"),
                format!("{at}: slot 1 of Name holds a place"),
                format!("{at}: slot 2 of Name is required"),
                format!("{at}: Name with 1 slots for 3"),
                format!("{at}: InStr is no built-in statement that is compiled"),
            ]
        );
    }

    #[test]
    fn read_and_restore() {
        use crate::DataItem;
        let mut p = program();
        p.data.push(DataItem {
            text: b"1".to_vec(),
            quoted: false,
        });
        // The end of the data is a position; the start needs no label.
        p.main.stmts[0].ops.extend([
            Op::Read(vec![Place::Var(VarId(0))]),
            Op::Restore { at: 0, label: None },
            Op::Restore {
                at: 1,
                label: Some("L".into()),
            },
        ]);
        assert_eq!(problems(&p), Vec::<String>::new());
        p.main.stmts[0].ops.extend([
            Op::Read(Vec::new()),
            Op::Read(vec![Place::Var(VarId(9))]),
            Op::Restore {
                at: 2,
                label: Some("L".into()),
            },
            Op::Restore { at: 1, label: None },
        ]);
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: a Read without a target"),
                format!("{at}: variable 9 does not exist"),
                format!("{at}: a Restore to item 2 of 1"),
                format!("{at}: a Restore to item 1 without a label"),
            ]
        );
    }

    /// [`program`] with a static array `S(0 TO 5)` (variable 3) and a dynamic one `D` of one dimension (4).
    fn arrays() -> Program {
        let mut p = program();
        let array = |name: &str, shape| Var {
            shape,
            ..var(name, Storage::Global)
        };
        p.vars.extend([
            array("S", Shape::Static { bounds: vec![(0, 5)] }),
            array(
                "D",
                Shape::Dynamic {
                    dims: Some(1),
                    at_start: None,
                },
            ),
        ]);
        p
    }

    fn bounds(upper: i64) -> Vec<(Expr, Expr)> {
        vec![(expr(Ty::I64, ExprKind::Int(0)), expr(Ty::I64, ExprKind::Int(upper)))]
    }

    fn allocate(array: u32, bounds: Vec<(Expr, Expr)>) -> Op {
        Op::Allocate {
            array: VarId(array),
            bounds,
            mode: AllocMode::Redim,
        }
    }

    #[test]
    fn sound_array_operations() {
        let mut p = arrays();
        // An allocation may name a static array: it raises an error when it runs.
        p.main.stmts[0].ops.extend([
            allocate(4, bounds(3)),
            allocate(3, bounds(3)),
            Op::Clear {
                array: ArrayRef::Var(VarId(3)),
            },
            Op::Release { array: VarId(4) },
        ]);
        assert_eq!(problems(&p), Vec::<String>::new());
    }

    /// [`arrays`] with the SUB's parameter `P` an array parameter of one dimension, which its body erases, and
    /// the call passing it `array`.
    fn array_param(array: u32) -> Program {
        let mut p = arrays();
        p.vars[1].shape = Shape::Dynamic {
            dims: Some(1),
            at_start: None,
        };
        p.procs[0].body.stmts = vec![stmt(vec![Op::EraseParam { array: VarId(1) }])];
        p.main.stmts[0].ops[1] = Op::Call {
            proc: ProcId(0),
            args: vec![Arg::Array(ArrayRef::Var(VarId(array)))],
        };
        p
    }

    #[test]
    fn array_parameters() {
        // A static and a dynamic array may be passed.
        assert_eq!(problems(&array_param(3)), Vec::<String>::new());
        assert_eq!(problems(&array_param(4)), Vec::<String>::new());
        let at = "main: statement 0 (line 1)";
        // A scalar as a whole array, and a place for an array parameter.
        assert_eq!(
            problems(&array_param(0)),
            [format!("{at}: an array argument of G, which is no array")]
        );
        let mut p = array_param(4);
        p.main.stmts[0].ops[1] = Op::Call {
            proc: ProcId(0),
            args: vec![Arg::Ref(Place::Var(VarId(0)))],
        };
        assert_eq!(
            problems(&p),
            [format!(
                "{at}: argument 1 is no whole array for a parameter that is an array"
            )]
        );
        // A whole array for a scalar parameter.
        let mut p = arrays();
        p.main.stmts[0].ops[1] = Op::Call {
            proc: ProcId(0),
            args: vec![Arg::Array(ArrayRef::Var(VarId(4)))],
        };
        assert_eq!(
            problems(&p),
            [format!(
                "{at}: argument 1 is a whole array for a parameter that is no array"
            )]
        );
        // Another number of dimensions than the parameter's.
        let mut p = array_param(4);
        p.vars[4].shape = Shape::Dynamic {
            dims: Some(2),
            at_start: None,
        };
        assert_eq!(problems(&p), [format!("{at}: D of 2 dimensions passed to P of 1")]);
    }

    #[test]
    fn erase_of_an_array_parameter() {
        // The array passed may be a static one: a parameter is never released, and only a parameter is erased
        // by what is passed.
        let mut p = array_param(4);
        p.procs[0].body.stmts[0].ops = vec![Op::Release { array: VarId(1) }];
        assert_eq!(
            problems(&p),
            ["S: statement 0 (line 1): Release of the array parameter P"]
        );
        let mut p = array_param(4);
        p.main.stmts[0].ops.push(Op::EraseParam { array: VarId(4) });
        assert_eq!(
            problems(&p),
            ["main: statement 0 (line 1): EraseParam of D, which is no parameter"]
        );
    }

    #[test]
    fn array_allocated_at_the_start() {
        let at_start = |dims, bounds: &[(i64, i64)]| Shape::Dynamic {
            dims,
            at_start: Some(bounds.to_vec()),
        };
        let mut p = arrays();
        p.vars[4].shape = at_start(Some(1), &[(0, 3)]);
        assert_eq!(problems(&p), Vec::<String>::new());
        // One pair of bounds for two dimensions, and an array that is made on each call.
        p.vars[4].shape = at_start(Some(2), &[(0, 3)]);
        p.vars[4].storage = Storage::Local(ProcId(0));
        assert_eq!(
            problems(&p),
            [
                "D: allocated at the start, but it does not live for the program",
                "D: 1 pairs of bounds at the start for Some(2) dimensions"
            ]
        );
    }

    #[test]
    fn array_without_its_number_of_dimensions() {
        let listed = Shape::Dynamic {
            dims: None,
            at_start: None,
        };
        let mut p = arrays();
        p.vars[4].shape = listed;
        p.vars[4].storage = Storage::Static(ProcId(0));
        assert_eq!(problems(&p), Vec::<String>::new());
        p.vars[4].storage = Storage::Local(ProcId(0));
        assert_eq!(
            problems(&p),
            ["D: an array made on each call without its number of dimensions"]
        );
    }

    #[test]
    fn release_of_a_static_array() {
        let mut p = arrays();
        p.main.stmts[0].ops.push(Op::Release { array: VarId(3) });
        assert_eq!(
            problems(&p),
            ["main: statement 0 (line 1): Release of S, which is no dynamic array"]
        );
    }

    #[test]
    fn clear_of_a_dynamic_array() {
        let mut p = arrays();
        p.main.stmts[0].ops.push(Op::Clear {
            array: ArrayRef::Var(VarId(4)),
        });
        assert_eq!(
            problems(&p),
            ["main: statement 0 (line 1): Clear of D, which is no static array"]
        );
    }

    #[test]
    fn allocation_with_another_number_of_bounds() {
        let mut p = arrays();
        let two = [bounds(3), bounds(4)].concat();
        p.main.stmts[0].ops.push(allocate(4, two));
        assert_eq!(
            problems(&p),
            ["main: statement 0 (line 1): 2 pairs of bounds for D of 1 dimensions"]
        );
    }

    #[test]
    fn allocation_bound_of_another_type() {
        let mut p = arrays();
        let narrow = vec![(expr(Ty::I64, ExprKind::Int(0)), load(0))];
        p.main.stmts[0].ops.push(allocate(4, narrow));
        assert_eq!(problems(&p), ["main: statement 0 (line 1): a bound of type I32"]);
    }

    #[test]
    fn array_operations_of_a_scalar() {
        let mut p = arrays();
        p.main.stmts[0].ops.extend([
            allocate(0, bounds(3)),
            Op::Clear {
                array: ArrayRef::Var(VarId(0)),
            },
            Op::Release { array: VarId(0) },
        ]);
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: Allocate of G, which is no array"),
                format!("{at}: Clear of G, which is no array"),
                format!("{at}: Release of G, which is no array"),
            ]
        );
    }

    #[test]
    fn element_of_an_array_of_another_shape() {
        let mut p = arrays();
        let element = |array: u32, n: usize| {
            let index = vec![expr(Ty::I64, ExprKind::Int(1)); n];
            Op::Raise(expr(
                Ty::I32,
                ExprKind::Load(Place::Element {
                    array: ArrayRef::Var(VarId(array)),
                    index,
                }),
            ))
        };
        p.main.stmts[0]
            .ops
            .extend([element(4, 1), element(4, 2), element(0, 1)]);
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: 2 indexes for D of 1 dimensions"),
                format!("{at}: an element of G, which is no array"),
            ]
        );
    }

    /// [`program`] with two user types of the same members, `G` of the first, and the SUB's parameter `P` of the
    /// first too; the main module's call passes `G`.
    fn user_types() -> Program {
        let mut p = program();
        let ut = |name: &str| crate::UserType {
            name: name.into(),
            members: vec![crate::Member {
                name: "X".into(),
                ty: Ty::I32,
                bounds: Vec::new(),
            }],
        };
        p.types = vec![ut("T1"), ut("T2")];
        p.vars[0].ty = Ty::User(crate::TypeId(0));
        p.vars[1].ty = Ty::User(crate::TypeId(0));
        p.vars.push(var("H", Storage::Global));
        p.vars[3].ty = Ty::User(crate::TypeId(1));
        p.procs[0].body.stmts.clear();
        p
    }

    /// [`user_types`] with a second member of the first type, the member array `V(0 TO 3)`.
    fn member_arrays() -> Program {
        let mut p = user_types();
        p.types[0].members.push(crate::Member {
            name: "V".into(),
            ty: Ty::I32,
            bounds: vec![(0, 3)],
        });
        p
    }

    #[test]
    fn member_arrays_are_named_as_arrays() {
        let mut p = member_arrays();
        let member = |m: u32| ArrayRef::Member {
            base: Box::new(Place::Var(VarId(0))),
            member: MemberId(m),
        };
        let element = |m: u32, n: usize| {
            Op::Raise(expr(
                Ty::I32,
                ExprKind::Load(Place::Element {
                    array: member(m),
                    index: vec![expr(Ty::I64, ExprKind::Int(1)); n],
                }),
            ))
        };
        let whole = Op::Raise(expr(
            Ty::I32,
            ExprKind::Load(Place::Member {
                base: Box::new(Place::Var(VarId(0))),
                member: MemberId(1),
            }),
        ));
        p.main.stmts[0]
            .ops
            .extend([element(1, 1), Op::Clear { array: member(1) }]);
        assert_eq!(problems(&p), Vec::<String>::new());
        p.main.stmts[0]
            .ops
            .extend([element(1, 2), element(0, 1), whole, Op::Clear { array: member(0) }]);
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: 2 indexes for V of 1 dimensions"),
                format!("{at}: an element of the member X, which is no array"),
                format!("{at}: the member array V used as a single value"),
                format!("{at}: Clear of the member X, which is no array"),
            ]
        );
    }

    #[test]
    fn whole_value_copies() {
        let mut p = user_types();
        let copy = |from: u32, to: u32| Op::CopyWhole {
            from: Place::Var(VarId(from)),
            to: Place::Var(VarId(to)),
        };
        p.main.stmts[0].ops.push(copy(0, 0));
        assert_eq!(problems(&p), Vec::<String>::new());
        p.main.stmts[0].ops.push(copy(3, 0));
        p.vars.push(var("N", Storage::Global));
        p.main.stmts[0].ops.push(copy(4, 4));
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: a CopyWhole from a User(TypeId(1)) to a User(TypeId(0))"),
                format!("{at}: a CopyWhole from a I32 to a I32"),
            ]
        );
    }

    #[test]
    fn user_type_parameters() {
        let mut p = user_types();
        assert_eq!(problems(&p), Vec::<String>::new());
        let call = |arg: Arg| Op::Call {
            proc: ProcId(0),
            args: vec![arg],
        };
        p.main.stmts[0].ops.extend([
            call(Arg::Ref(Place::Var(VarId(3)))),
            call(Arg::Temp(expr(Ty::I32, ExprKind::Int(1)))),
        ]);
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: argument 1 is no place of the user type of P"),
                format!("{at}: argument 1 is no place of the user type of P"),
            ]
        );
    }

    #[test]
    fn builtin_slot_count() {
        let mut p = program();
        let instr = qb64rust_builtins::find_function(b"INSTR").unwrap();
        let call = ExprKind::Call {
            builtin: instr,
            args: vec![Some(expr(Ty::Str, ExprKind::Str(b"a".to_vec())))],
        };
        p.main.stmts[0].ops.push(Op::Raise(expr(Ty::I32, call)));
        let got = problems(&p);
        assert_eq!(got.len(), 1, "{got:?}");
        assert!(got[0].ends_with("with 1 argument slots for 3"), "{got:?}");
    }

    /// The array of `_UCHARPOS`: of 32-bit integers, typed as its elements, one index per dimension.
    #[test]
    fn bytes_of_an_array_as_a_function_argument() {
        let mut p = arrays();
        p.vars[3].ty = Ty::I64;
        let block = |array: u32, ty: Ty, index: Vec<Expr>| {
            let array = VarId(array);
            Some(expr(ty, ExprKind::Block { array, index }))
        };
        let index = || vec![expr(Ty::I64, ExprKind::Int(2))];
        let uchar = qb64rust_builtins::find_function(b"_UCHARPOS").unwrap();
        for arg in [
            block(4, Ty::I32, Vec::new()),
            block(4, Ty::I32, index()),
            block(3, Ty::I64, Vec::new()),
            block(4, Ty::U32, Vec::new()),
            block(4, Ty::I32, vec![index().remove(0), index().remove(0)]),
        ] {
            let call = ExprKind::Call {
                builtin: uchar,
                args: vec![Some(expr(Ty::Str, ExprKind::Str(b"a".to_vec()))), arg, None, None],
            };
            p.main.stmts[0].ops.push(Op::Raise(expr(Ty::I32, call)));
        }
        // A value in the array's slot, and the bytes where a value goes: as an operand and in a value slot.
        let call = ExprKind::Call {
            builtin: uchar,
            args: vec![
                Some(expr(Ty::Str, ExprKind::Str(b"a".to_vec()))),
                Some(expr(Ty::I32, ExprKind::Int(5))),
                None,
                None,
            ],
        };
        p.main.stmts[0].ops.push(Op::Raise(expr(Ty::I32, call)));
        p.main.stmts[0]
            .ops
            .push(Op::Raise(block(4, Ty::I32, Vec::new()).unwrap()));
        let call = ExprKind::Call {
            builtin: uchar,
            args: vec![
                Some(expr(Ty::Str, ExprKind::Str(b"a".to_vec()))),
                None,
                block(4, Ty::I32, Vec::new()),
                None,
            ],
        };
        p.main.stmts[0].ops.push(Op::Raise(expr(Ty::I32, call)));
        let at = "main: statement 0 (line 1)";
        assert_eq!(
            problems(&p),
            [
                format!("{at}: the bytes of S, an array of I64, as I64"),
                format!("{at}: the bytes of D, an array of I32, as U32"),
                format!("{at}: 2 indexes for D"),
                format!("{at}: argument 2 of _UCharPos takes an array's bytes, not a value"),
                format!("{at}: the bytes of D as a value"),
                format!("{at}: the bytes of D as a value"),
            ]
        );
    }
}
