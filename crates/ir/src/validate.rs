//! Structural checks of a lowered program (design D8 of `m2-arrays-and-types`): what the emitter relies on and the
//! types cannot say. Tier 1 runs it on every accepted input; the lowering runs it under `debug_assert!`.

use crate::{Arg, Body, Expr, ExprKind, LabelId, Op, Place, PrintItem, ProcId, Program, Resume, Storage, Ty, VarId};

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
            Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str => None,
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
            Op::Raise(v) => self.value(v, owner, at),
            Op::Print { items, .. } => {
                for i in items {
                    match i {
                        PrintItem::Str(v) | PrintItem::Num(v) => self.value(v, owner, at),
                        PrintItem::Zone => {}
                    }
                }
            }
            Op::Call { proc, args } => self.call(*proc, args, owner, at),
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
        for a in args {
            match a {
                Arg::Ref(place) => self.place(place, owner, at),
                Arg::Temp(v) => self.value(v, owner, at),
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
                    if v.dims.len() != index.len() {
                        let msg = format!(
                            "{at}: {} indexes for {} of {} dimensions",
                            index.len(),
                            v.name,
                            v.dims.len()
                        );
                        self.problem(msg);
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
        Arg, Body, Expr, ExprKind, Label, LabelId, Op, Place, Proc, ProcId, ProcKind, Program, Resume, Stmt, Storage,
        Ty, Var, VarId,
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
            dims: Vec::new(),
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
