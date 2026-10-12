//! Structural checks of a lowered program (design D8 of `m2-arrays-and-types`): what the emitter relies on and the
//! types cannot say. Tier 1 runs it on every accepted input; the lowering runs it under `debug_assert!`.

use crate::builtins::{Slot, StmtRule, StmtSlot, stmt_find, stmt_slots};
use crate::{
    Arg, Body, BuiltinId, Expr, ExprKind, LabelId, Op, Place, PrintItem, ProcId, Program, Resume, Shape, Source,
    StmtArg, Storage, Ty, Var, VarId,
};

/// Every problem found, each as `<body>: <where>: <what>`; `Ok` when there is none.
pub fn validate(p: &Program) -> Result<(), Vec<String>> {
    let mut v = Validator {
        p,
        problems: Vec::new(),
    };
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
        Place::Var(v) | Place::Element { array: v, .. } => p.vars.get(v.0 as usize).map(|v| v.ty),
        Place::Member { base, member } => match place_ty_checked(p, base)? {
            Ty::User(t) => p.types.get(t.0 as usize)?.members.get(member.0 as usize).map(|m| m.ty),
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
        },
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
            Op::SelectConsole | Op::End | Op::System | Op::Exit | Op::SetHandler(None) | Op::Return(None) => {}
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
            Op::Clear { array } => {
                if self.array_use(*array, owner, at, "Clear")
                    && !matches!(self.p.var(*array).shape, Shape::Static { .. })
                {
                    let name = &self.p.var(*array).name;
                    self.problem(format!("{at}: Clear of {name}, which is no static array"));
                }
            }
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
            Op::Print { to, items, .. } | Op::Write { to, items, .. } => {
                if let Some(n) = to {
                    self.file_number(n, owner, at);
                }
                for i in items {
                    match i {
                        PrintItem::Str(v) | PrintItem::Num(v) => self.value(v, owner, at),
                        PrintItem::Zone => {
                            if matches!(op, Op::Write { .. }) {
                                self.problem(format!("{at}: a zone in a Write"));
                            }
                        }
                    }
                }
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
                    if matches!(place, Place::Member { .. }) && place.has_element() {
                        self.problem(format!("{at}: an Input target that is a member of an element"));
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
                    if matches!(place, Place::Member { .. }) && place.has_element() {
                        self.problem(format!("{at}: a Read target that is a member of an element"));
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
            StmtRule::Plain => {}
            // Any number of file numbers.
            StmtRule::Close => {
                for (k, arg) in args.iter().enumerate() {
                    match arg {
                        StmtArg::Value(v) => self.file_number(v, owner, at),
                        StmtArg::Place(_) | StmtArg::Word(_) | StmtArg::Absent => {
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
                    StmtArg::Place(_) | StmtArg::Word(_) => self.problem(format!("{at}: slot 3 of {name} is no value")),
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
                        Slot::Long | Slot::Int64 | Slot::Double | Slot::Float | Slot::AnyNumeric => false,
                    };
                    if string != (v.ty == Ty::Str) {
                        self.problem(format!("{at}: slot {k} of {name} holds a value of type {:?}", v.ty));
                    }
                    self.value(v, owner, at);
                }
                (StmtSlot::Arg { .. }, StmtArg::Place(_)) => {
                    self.problem(format!("{at}: slot {k} of {name} holds a place"));
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
                (StmtSlot::Choice { .. }, StmtArg::Value(_) | StmtArg::Place(_)) => {
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
                    if self.array_use(*array, owner, at, "an array argument")
                        && let Some(param) = param.filter(|_| to_array)
                        && let (Some(n), Some(m)) = (self.p.var(*array).dim_count(), param.dim_count())
                        && n != m
                    {
                        let name = &self.p.var(*array).name;
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
    /// base's user type has.
    fn place(&mut self, place: &Place, owner: Option<ProcId>, at: &At) {
        match place {
            Place::Var(v) => {
                if self.var_use(*v, owner, at) && self.p.var(*v).is_array() {
                    let name = &self.p.var(*v).name;
                    self.problem(format!("{at}: the array {name} used as a scalar"));
                }
            }
            Place::Element { array, index } => {
                if self.var_use(*array, owner, at) {
                    let v = self.p.var(*array);
                    match v.dim_count() {
                        Some(n) if n == index.len() => {}
                        Some(n) => {
                            let msg = format!("{at}: {} indexes for {} of {n} dimensions", index.len(), v.name);
                            self.problem(msg);
                        }
                        None if v.is_array() => {
                            self.problem(format!(
                                "{at}: an element of {}, whose dimensions are not known",
                                v.name
                            ));
                        }
                        None => self.problem(format!("{at}: an element of {}, which is no array", v.name)),
                    }
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
                let ok = match place_ty_checked(self.p, base) {
                    Some(Ty::User(t)) => self
                        .p
                        .types
                        .get(t.0 as usize)
                        .is_some_and(|ut| (member.0 as usize) < ut.members.len()),
                    Some(_) | None => false,
                };
                if !ok {
                    self.problem(format!("{at}: member {} of a place without it", member.0));
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
            ExprKind::Bound { array, dim, .. } => {
                if self.var_use(*array, owner, at) && !self.p.var(*array).is_array() {
                    let name = &self.p.var(*array).name;
                    self.problem(format!("{at}: bound of {name}, which is no array"));
                }
                if let Some(d) = dim {
                    self.value(d, owner, at);
                }
            }
            ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => self.value(from, owner, at),
            ExprKind::Binary { lhs, rhs, .. } | ExprKind::Concat(lhs, rhs) | ExprKind::StrCompare { lhs, rhs, .. } => {
                self.value(lhs, owner, at);
                self.value(rhs, owner, at);
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
                for a in args.iter().flatten() {
                    self.value(a, owner, at);
                }
            }
            ExprKind::CallProc { proc, args } => self.call(*proc, args, owner, at),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::{
        AllocMode, Arg, Body, Expr, ExprKind, Label, LabelId, Op, Place, Proc, ProcId, ProcKind, Program, Resume,
        Shape, Stmt, Storage, Ty, Var, VarId,
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
            Op::Clear { array: VarId(3) },
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
            args: vec![Arg::Array(VarId(array))],
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
            args: vec![Arg::Array(VarId(4))],
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
        p.main.stmts[0].ops.push(Op::Clear { array: VarId(4) });
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
            Op::Clear { array: VarId(0) },
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
                    array: VarId(array),
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
}
