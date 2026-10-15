//! Owner values: values of a user type that owns dynamic member arrays (design D4, D6 and D9 of `m3-typefields`).
//!
//! A dynamic member array is a pointer in the value to a descriptor of the shape libqb's dynamic arrays have
//! (`study\02` §3.4), allocated with `malloc` for its number of dimensions, which slot 3 holds, with a `_MEM` lock
//! of its own in the slot after the dimensions. A null pointer (a member of an element of a static array that
//! `ERASE` released) reads as "not allocated". Everything that works on a dynamic array's descriptor works on one:
//! `REDIM` ([`Emitter::allocate_with`]), the bounds, an array argument.
//!
//! The life of an owner value is one C++ function per owner type and use, written once into `global.txt`
//! ([`Emitter::owner_functions`]): `qbr_udtN_new` (each dynamic member array at its declared bounds with zero or
//! empty elements, each `STRING` an empty string, nested owner values made too), `qbr_udtN_free` (the reverse),
//! `qbr_udtN_zero` (`ERASE` of a static array: strings emptied, bytes zero, dynamic member arrays released and
//! left null) and `qbr_udtN_clone` (a deep copy into a value that exists). Each dynamic member array M of type N
//! has its own: `qbr_udtN_mM_blank` (a descriptor that is not allocated), `_make` (one at the declared bounds),
//! `_release` (`ERASE`), `_free`, `_dup` (a deep copy of an allocated descriptor), `_copy` (the old compiler's
//! `AppendDynUDTDescCopy`: a source that is not allocated leaves the target as it is), `_set` (its
//! `AppendDynUDTOwnCopyEx`, into a target already erased: null, not allocated or a deep copy, as the source is),
//! `_guard` (the descriptor of an element, or after error 9 a dummy one of one element so the expression can
//! finish), `_bound` and `_desc` (for an array argument). A whole-value copy is [`Emitter::copy_owner`].

use crate::Emitter;
use crate::decl::{Part, dim_slot};
use crate::names::int_const;
use qb64rust_ir::{Member, MemberId, Place, Ty, TypeId};
use std::fmt::Write as _;

/// The number of dimensions the dummy descriptor of a guard has room for.
const DUMMY_DIMS: usize = 64;

/// The name of a function of owner type `id`.
pub(crate) fn udt_fn(id: TypeId, what: &str) -> String {
    format!("qbr_udt{}_{what}", id.0)
}

/// The name of a function of the dynamic member array `member` of type `id`.
pub(crate) fn dyn_fn(id: TypeId, member: u32, what: &str) -> String {
    format!("qbr_udt{}_m{member}_{what}", id.0)
}

impl Emitter<'_> {
    /// The dynamic member array `member` of a place `base`, as its type and index, when it is one.
    pub(crate) fn dynamic_member(&self, base: &Place, member: MemberId) -> Option<(TypeId, u32)> {
        let Ty::User(id) = self.p.place_ty(base) else {
            return None;
        };
        self.p.user_type(id).members[member.0 as usize]
            .dynamic
            .then_some((id, member.0))
    }

    /// The address of the slot that holds the descriptor pointer of a dynamic member array of `base`, as a `char*`
    /// expression; finding it evaluates the indexes of `base` once.
    pub(crate) fn dyn_slot(&mut self, base: &Place, member: MemberId) -> String {
        let offset = self.member_offset(self.p.place_ty(base), member);
        format!("({}+{offset})", self.bytes_of(base))
    }

    /// The owner type `t` is, when it is one.
    fn owner(&self, t: Ty) -> Option<TypeId> {
        if let Ty::User(id) = t
            && self.owns_desc(t)
        {
            return Some(id);
        }
        None
    }

    /// The statements that make a value of type `t` at `at` (a `char*` expression) whose bytes are zero: a
    /// string empty, an owner value's members at their declared bounds, the strings of a user type empty.
    pub(crate) fn new_value(&self, t: Ty, at: &str) -> String {
        if t == Ty::Str {
            format!("*(qbs**)({at})=qbs_new(0,0);\n")
        } else if let Some(id) = self.owner(t) {
            format!("{}((char*)({at}));\n", udt_fn(id, "new"))
        } else {
            self.new_strings(t, at)
        }
    }

    /// The statements that release what a value of type `t` at `at` holds, before its bytes go.
    pub(crate) fn free_value(&self, t: Ty, at: &str) -> String {
        if t == Ty::Str {
            format!("qbs_free(*(qbs**)({at}));\n")
        } else if let Some(id) = self.owner(t) {
            format!("{}((char*)({at}));\n", udt_fn(id, "free"))
        } else {
            self.free_strings(t, at)
        }
    }

    /// The statements that copy a value of type `t` at `from` into the one at `to`, which exists: deep, a string
    /// as a string of its own.
    pub(crate) fn clone_value(&self, t: Ty, to: &str, from: &str) -> String {
        if t == Ty::Str {
            return format!("qbs_set(*(qbs**)({to}),*(qbs**)({from}));\n");
        }
        if let Some(id) = self.owner(t) {
            return format!("{}((char*)({to}),(char*)({from}));\n", udt_fn(id, "clone"));
        }
        if !self.holds_strings(t) {
            return format!("memmove((void*)({to}),(void*)({from}),{});\n", self.ty_size(t));
        }
        let mut out = String::new();
        for (offset, part) in self.parts(t) {
            match part {
                Part::Str => writeln!(out, "qbs_set(*(qbs**)(({to})+{offset}),*(qbs**)(({from})+{offset}));"),
                Part::Bytes(n) => writeln!(out, "memmove(({to})+{offset},({from})+{offset},{n});"),
                Part::Dyn { .. } => unreachable!("a type that owns no dynamic member array"),
            }
            .unwrap();
        }
        out
    }

    /// The statements that set a value of type `t` at `at` to zero as `ERASE` of a static array does: a string
    /// emptied, the bytes zero, an owner value's dynamic member arrays released and left null.
    fn zero_one(&self, t: Ty, at: &str) -> String {
        if t == Ty::Str {
            format!("(*(qbs**)({at}))->len=0;\n")
        } else {
            self.zero_value(t, at)
        }
    }

    /// `body` of each of `count` values of `size` bytes from `at` (a `char*` expression), in a loop when there
    /// is more than one; nothing when `body` writes nothing.
    fn each_of(count: u32, size: u32, at: &str, body: impl Fn(&str) -> String) -> String {
        if count == 1 {
            return body(at);
        }
        let text = body(&format!("({at}+qbr_i*{size})"));
        if text.is_empty() {
            return text;
        }
        format!("for(ptrszint qbr_i=0;qbr_i<{count};qbr_i++){{\n{text}}}\n")
    }

    /// `CopyWhole` of an owner value (design D7 of `m3-typefields`): the statements of the old compiler's
    /// `copy_full_udt_dyn` (`utilities\type.bas` 2049), each of which finds the places anew, so an index in `to`
    /// or `from` (C expressions of their addresses) is evaluated as often as there (measured,
    /// `verification\v35_x56`, `x61`). Inside `if (to!=from)`: for a type that holds a string, each part in
    /// declaration order, a dynamic member array erased (`to`) and then made as the source's (`from`, `to`);
    /// for one that holds none, every part but the dynamic member arrays, then each of those copied (`to`,
    /// `from`), a source that is not allocated leaving the target as it is. Every other part finds `to`, then
    /// `from`.
    pub(crate) fn copy_owner(&self, t: Ty, to: &str, from: &str, out: &mut Vec<String>) {
        let mut steps = Vec::new();
        if self.holds_strings(t) {
            self.copy_owned(t, "0", 0, &mut steps);
        } else {
            self.copy_plain(t, "0", 0, &mut steps);
            self.copy_descs(t, "0", 0, &mut steps);
        }
        out.push(format!("if ((void*)({to})!=(void*)({from})){{"));
        for step in steps {
            out.push(match step {
                Step::Open(line) | Step::Close(line) => line,
                Step::ToFrom(s) => format!("{{char *copy_to={to};char *copy_from={from};{s}}}"),
                Step::FromTo(s) => format!("{{char *copy_from={from};char *copy_to={to};{s}}}"),
                Step::To(s) => format!("{{char *copy_to={to};{s}}}"),
            });
        }
        out.push("}".into());
    }

    /// The members of user type `t` at offset `at` (a C expression), each with its index and its offset as a C
    /// expression, in declaration order.
    fn members_at(&self, t: Ty, at: &str) -> Vec<(&Member, u32, String)> {
        let Ty::User(id) = t else {
            unreachable!("the members of {t:?}");
        };
        let (offsets, _) = self.layout(id);
        let members = &self.p.user_type(id).members;
        members
            .iter()
            .zip(offsets)
            .enumerate()
            .map(|(i, (m, off))| {
                let here = if at == "0" {
                    off.to_string()
                } else {
                    format!("{at}+{off}")
                };
                (m, qb64rust_base::to_u32(i), here)
            })
            .collect()
    }

    /// The offset of each of the `count` elements of a member array at `at`, `size` bytes each, in a loop whose
    /// lines open and close `steps` around what `body` adds.
    fn loop_over(
        count: u32,
        size: u32,
        at: &str,
        depth: u32,
        steps: &mut Vec<Step>,
        body: impl FnOnce(&str, &mut Vec<Step>),
    ) {
        let i = format!("qbr_c{depth}");
        steps.push(Step::Open(format!("for(ptrszint {i}=0;{i}<{count};{i}++){{")));
        body(&format!("{at}+{i}*{size}"), steps);
        steps.push(Step::Close("}".into()));
    }

    /// `AppendDynUDTOwnCopyEx` (`utilities\type.bas` 1258), the copy of a type that holds a string.
    fn copy_owned(&self, t: Ty, at: &str, depth: u32, steps: &mut Vec<Step>) {
        let Ty::User(id) = t else { unreachable!() };
        let deep = |t: Ty| self.holds_strings(t) || self.owns_desc(t);
        for (m, i, here) in self.members_at(t, at) {
            let size = self.ty_size(m.ty);
            if m.dynamic {
                let slot = format!("(ptrszint**)(copy_to+{here})");
                steps.push(Step::To(format!("{}(*{slot});*{slot}=NULL;", dyn_fn(id, i, "free"))));
                steps.push(Step::FromTo(format!(
                    "{}({slot},(ptrszint**)(copy_from+{here}));",
                    dyn_fn(id, i, "set")
                )));
            } else if !m.bounds.is_empty() && (m.ty == Ty::Str || matches!(m.ty, Ty::User(_))) {
                Self::loop_over(m.count(), size, &here, depth, steps, |item, steps| {
                    if m.ty == Ty::Str {
                        steps.push(Step::ToFrom(str_set(item)));
                    } else if deep(m.ty) {
                        self.copy_owned(m.ty, item, depth + 1, steps);
                    } else {
                        steps.push(Step::ToFrom(bytes_copy(item, size)));
                    }
                });
            } else if m.ty == Ty::Str {
                steps.push(Step::ToFrom(str_set(&here)));
            } else if deep(m.ty) {
                self.copy_owned(m.ty, &here, depth, steps);
            } else {
                steps.push(Step::ToFrom(bytes_copy(&here, self.member_size(m))));
            }
        }
    }

    /// `CopyDynUDTExpr` (`utilities\type.bas` 2093): the parts of a type that holds no string, but its dynamic
    /// member arrays.
    fn copy_plain(&self, t: Ty, at: &str, depth: u32, steps: &mut Vec<Step>) {
        for (m, _, here) in self.members_at(t, at) {
            if m.dynamic {
                continue;
            }
            if !self.owns_desc(m.ty) {
                steps.push(Step::ToFrom(bytes_copy(&here, self.member_size(m))));
            } else if m.bounds.is_empty() {
                self.copy_plain(m.ty, &here, depth, steps);
            } else {
                Self::loop_over(m.count(), self.ty_size(m.ty), &here, depth, steps, |item, steps| {
                    self.copy_plain(m.ty, item, depth + 1, steps);
                });
            }
        }
    }

    /// `AppendDynUDTDescCopy` (`utilities\type.bas` 10): the dynamic member arrays of a type that holds no
    /// string.
    fn copy_descs(&self, t: Ty, at: &str, depth: u32, steps: &mut Vec<Step>) {
        let Ty::User(id) = t else { unreachable!() };
        for (m, i, here) in self.members_at(t, at) {
            if m.dynamic {
                steps.push(Step::ToFrom(format!(
                    "{}((ptrszint**)(copy_to+{here}),(ptrszint**)(copy_from+{here}));",
                    dyn_fn(id, i, "copy")
                )));
            } else if self.owns_desc(m.ty) {
                if m.bounds.is_empty() {
                    self.copy_descs(m.ty, &here, depth, steps);
                } else {
                    Self::loop_over(m.count(), self.ty_size(m.ty), &here, depth, steps, |item, steps| {
                        self.copy_descs(m.ty, item, depth + 1, steps);
                    });
                }
            }
        }
    }

    /// The C++ functions of every owner type and of each of their dynamic member arrays, for `global.txt`:
    /// their prototypes first, so that a function may call one of a type declared after it.
    pub(crate) fn owner_functions(&self) -> String {
        let owners: Vec<TypeId> = (0..self.p.types.len())
            .map(|i| TypeId(qb64rust_base::to_u32(i)))
            .filter(|&id| self.owns_desc(Ty::User(id)))
            .collect();
        if owners.is_empty() {
            return String::new();
        }
        let mut protos = String::from("static ptrszint qbr_dyn_count(ptrszint *d);\n");
        let mut defs = String::from(
            "static ptrszint qbr_dyn_count(ptrszint *d){\nptrszint n=1;\n\
             for(ptrszint i=1;i<=d[3];i++) n*=d[i*4+1];\nreturn n;\n}\n",
        );
        for &id in &owners {
            for what in ["new", "free", "zero"] {
                writeln!(protos, "static void {}(char *p);", udt_fn(id, what)).unwrap();
            }
            writeln!(protos, "static void {}(char *to,char *from);", udt_fn(id, "clone")).unwrap();
            defs.push_str(&self.udt_functions(id));
            for (i, m) in self.p.user_type(id).members.iter().enumerate() {
                if m.dynamic {
                    let i = qb64rust_base::to_u32(i);
                    protos.push_str(&dyn_prototypes(id, i));
                    defs.push_str(&self.dyn_functions(id, i, m));
                }
            }
        }
        protos + &defs
    }

    /// `qbr_udtN_new`, `_free`, `_zero` and `_clone` of owner type `id`.
    fn udt_functions(&self, id: TypeId) -> String {
        let (offsets, _) = self.layout(id);
        let members = &self.p.user_type(id).members;
        let (mut new, mut free, mut zero, mut clone) = (String::new(), String::new(), String::new(), String::new());
        for (i, (m, off)) in members.iter().zip(offsets).enumerate() {
            let slot = format!("*(ptrszint**)(p+{off})");
            if m.dynamic {
                let i = qb64rust_base::to_u32(i);
                writeln!(new, "{slot}={}();", dyn_fn(id, i, "make")).unwrap();
                writeln!(free, "{}({slot});\n{slot}=NULL;", dyn_fn(id, i, "free")).unwrap();
                writeln!(zero, "{}({slot});\n{slot}=NULL;", dyn_fn(id, i, "free")).unwrap();
                writeln!(
                    clone,
                    "{}((ptrszint**)(to+{off}),(ptrszint**)(from+{off}));",
                    dyn_fn(id, i, "copy")
                )
                .unwrap();
                continue;
            }
            let size = self.ty_size(m.ty);
            let count = m.count();
            if m.ty == Ty::Str || self.has_life(m.ty) {
                let at = format!("p+{off}");
                new.push_str(&Self::each_of(count, size, &at, |a| self.new_value(m.ty, a)));
                free.push_str(&Self::each_of(count, size, &at, |a| self.free_value(m.ty, a)));
                zero.push_str(&Self::each_of(count, size, &at, |a| self.zero_one(m.ty, a)));
                let body = |k: &str| {
                    let (to, from) = if count == 1 {
                        (format!("to+{off}"), format!("from+{off}"))
                    } else {
                        (format!("to+{off}+{k}*{size}"), format!("from+{off}+{k}*{size}"))
                    };
                    self.clone_value(m.ty, &to, &from)
                };
                if count == 1 {
                    clone.push_str(&body(""));
                } else {
                    write!(
                        clone,
                        "for(ptrszint qbr_i=0;qbr_i<{count};qbr_i++){{\n{}}}\n",
                        body("qbr_i")
                    )
                    .unwrap();
                }
            } else {
                let bytes = self.member_size(m);
                writeln!(zero, "memset((void*)(p+{off}),0,{bytes});").unwrap();
                writeln!(clone, "memmove((void*)(to+{off}),(void*)(from+{off}),{bytes});").unwrap();
            }
        }
        format!(
            "static void {}(char *p){{\n{new}}}\nstatic void {}(char *p){{\n{free}}}\n\
             static void {}(char *p){{\n{zero}}}\nstatic void {}(char *to,char *from){{\n{clone}}}\n",
            udt_fn(id, "new"),
            udt_fn(id, "free"),
            udt_fn(id, "zero"),
            udt_fn(id, "clone")
        )
    }

    /// The functions of the dynamic member array `member` (`m`) of owner type `id`.
    fn dyn_functions(&self, id: TypeId, member: u32, m: &Member) -> String {
        let f = |what: &str| dyn_fn(id, member, what);
        let (et, es) = (m.ty, self.ty_size(m.ty));
        let dims = m.bounds.len();
        let nothing = if et == Ty::Str {
            "(ptrszint)&nothingstring"
        } else {
            "(ptrszint)nothingvalue"
        };
        let mut out = String::new();
        // A descriptor of `dims` dimensions that is not allocated.
        write!(
            out,
            "static ptrszint *{}(ptrszint dims){{\n\
             ptrszint *d=(ptrszint*)calloc(dims*4+5,ptrsz);\nif (!d) error(257);\n\
             new_mem_lock();\nmem_lock_tmp->type=4;\nd[dims*4+4]=(ptrszint)mem_lock_tmp;\nd[3]=dims;\n\
             for(ptrszint i=1;i<=dims;i++) d[i*4]=2147483647;\nd[0]={nothing};\nreturn d;\n}}\n",
            f("blank")
        )
        .unwrap();
        // `body` of the address of each of the `qbr_n` elements of the block of descriptor `d` (and of the same
        // element of descriptor `s`, for a copy).
        let each = |body: &dyn Fn(&str, &str) -> String| -> String {
            let text = body(
                &format!("((char*)d[0]+qbr_i*{es})"),
                &format!("((char*)s[0]+qbr_i*{es})"),
            );
            if text.is_empty() {
                return text;
            }
            format!("for(ptrszint qbr_i=0;qbr_i<qbr_n;qbr_i++){{\n{text}}}\n")
        };
        // `ERASE`: the elements and the block released, the descriptor not allocated.
        let release = each(&|at, _| self.free_value(et, at));
        let count = if release.is_empty() {
            String::new()
        } else {
            "ptrszint qbr_n=qbr_dyn_count(d);\n".to_string()
        };
        write!(
            out,
            "static void {}(ptrszint *d){{\nif (!d||!(d[2]&1)) return;\n{count}{release}free((void*)d[0]);\n\
             d[2]&=~1;\nfor(ptrszint i=1;i<=d[3];i++){{d[i*4]=2147483647;d[i*4+1]=0;d[i*4+2]=0;}}\n\
             d[0]={nothing};\n}}\n",
            f("release")
        )
        .unwrap();
        write!(
            out,
            "static void {}(ptrszint *d){{\nif (!d) return;\n{}(d);\n\
             free_mem_lock((mem_lock*)d[d[3]*4+4]);\nfree(d);\n}}\n",
            f("free"),
            f("release")
        )
        .unwrap();
        // The declared bounds, under the `OPTION BASE` of the `TYPE` block.
        let mut make = format!("ptrszint *d={}({dims});\n", f("blank"));
        let mut multiplier = 1i64;
        for (k, &(lower, upper)) in m.bounds.iter().enumerate() {
            let s = dim_slot(dims, k);
            let count = upper - lower + 1;
            writeln!(
                make,
                "d[{s}]={};\nd[{}]={count};\nd[{}]={multiplier};",
                int_const(lower, Ty::I64),
                s + 1,
                s + 2
            )
            .unwrap();
            multiplier *= count;
        }
        write!(
            out,
            "static ptrszint *{}(){{\n{make}ptrszint qbr_n={multiplier};\n\
             d[0]=(ptrszint)calloc(qbr_n*{es},1);\nif (!d[0]) error(257);\n{}d[2]|=1;\nreturn d;\n}}\n",
            f("make"),
            each(&|at, _| self.new_value(et, at))
        )
        .unwrap();
        // A deep copy of an allocated descriptor `s`.
        let copy_elements = if self.has_life(et) || et == Ty::Str {
            each(&|to, from| self.new_value(et, to) + &self.clone_value(et, to, from))
        } else {
            format!("memcpy((void*)d[0],(void*)s[0],qbr_n*{es});\n")
        };
        write!(
            out,
            "static ptrszint *{}(ptrszint *s){{\nptrszint dims=s[3];\n\
             ptrszint *d=(ptrszint*)calloc(dims*4+5,ptrsz);\nif (!d) error(257);\n\
             memcpy((void*)d,(void*)s,(dims*4+4)*ptrsz);\n\
             new_mem_lock();\nmem_lock_tmp->type=4;\nd[dims*4+4]=(ptrszint)mem_lock_tmp;\n\
             ptrszint qbr_n=qbr_dyn_count(s);\nd[0]=(ptrszint)calloc(qbr_n?qbr_n*{es}:1,1);\n\
             if (!d[0]) error(257);\n{copy_elements}return d;\n}}\n",
            f("dup")
        )
        .unwrap();
        // A whole-value copy of a type that holds no string (`AppendDynUDTDescCopy`): a source that is not
        // allocated leaves the target as it is.
        write!(
            out,
            "static void {}(ptrszint **to,ptrszint **from){{\nptrszint *s=*from;\n\
             if (!s||!(s[2]&1)) return;\nptrszint *d={}(s);\n{}(*to);\n*to=d;\n}}\n",
            f("copy"),
            f("dup"),
            f("free")
        )
        .unwrap();
        // A whole-value copy of a type that holds a string (`AppendDynUDTOwnCopyEx`): the target, already erased,
        // becomes null, not allocated or a deep copy, as the source is.
        write!(
            out,
            "static void {}(ptrszint **to,ptrszint **from){{\nptrszint *s=*from;\n\
             *to=!s?NULL:(s[2]&1)?{}(s):{}(s[3]);\n}}\n",
            f("set"),
            f("dup"),
            f("blank")
        )
        .unwrap();
        // The descriptor of an element: error 9 when it is not allocated or has another number of dimensions,
        // and then a dummy of one element in every dimension.
        write!(
            out,
            "static ptrszint *{}(ptrszint *d,ptrszint n){{\n\
             if (d&&(d[2]&1)&&d[3]==n) return d;\nerror(9);\nstatic ptrszint *z=NULL;\nif (!z){{\n\
             z=(ptrszint*)calloc({DUMMY_DIMS}*4+5,ptrsz);\nz[2]=1;\n\
             for(ptrszint i=1;i<={DUMMY_DIMS};i++){{z[i*4+1]=1;z[i*4+2]=1;}}\n\
             z[0]=(ptrszint)calloc({es},1);\n{}}}\nreturn z;\n}}\n",
            f("guard"),
            self.new_value(et, "(char*)z[0]")
        )
        .unwrap();
        write!(
            out,
            "static int64 {}(ptrszint *d,int32 upper,int32 dim){{\nif (!d){{error(9);return 0;}}\n\
             if (upper) return func_ubound(d,dim,d[3]);\nreturn func_lbound(d,dim,d[3]);\n}}\n",
            f("bound")
        )
        .unwrap();
        write!(
            out,
            "static ptrszint *{}(char *slot){{\nptrszint *d=*(ptrszint**)slot;\n\
             if (!d){{d={}({dims});*(ptrszint**)slot=d;}}\nreturn d;\n}}\n",
            f("desc"),
            f("blank")
        )
        .unwrap();
        out
    }
}

/// A statement of [`Emitter::copy_owner`], by the places it finds and in which order (`copy_to`, `copy_from`),
/// or a line of a loop around such statements.
enum Step {
    Open(String),
    Close(String),
    ToFrom(String),
    FromTo(String),
    To(String),
}

/// The copy of the `STRING` at offset `at` of both places, safe against a descriptor that is not made, as the old
/// compiler's `AppendDynUDTOwnCopyEx` writes it.
fn str_set(at: &str) -> String {
    format!(
        "{{qbs **qbr_to=(qbs**)(copy_to+{at});qbs *qbr_from=*(qbs**)(copy_from+{at});\
         if (!*qbr_to) *qbr_to=qbs_new(0,0);if (qbr_from) qbs_set(*qbr_to,qbr_from);}}"
    )
}

/// The copy of `n` bytes at offset `at` of both places.
fn bytes_copy(at: &str, n: u32) -> String {
    format!("memmove(copy_to+{at},copy_from+{at},{n});")
}

/// The prototypes of the functions of a dynamic member array.
fn dyn_prototypes(id: TypeId, member: u32) -> String {
    let f = |what: &str| dyn_fn(id, member, what);
    format!(
        "static ptrszint *{}(ptrszint dims);\nstatic void {}(ptrszint *d);\nstatic void {}(ptrszint *d);\n\
         static ptrszint *{}();\nstatic ptrszint *{}(ptrszint *s);\nstatic void {}(ptrszint **to,ptrszint **from);\n\
         static void {}(ptrszint **to,ptrszint **from);\n\
         static ptrszint *{}(ptrszint *d,ptrszint n);\nstatic int64 {}(ptrszint *d,int32 upper,int32 dim);\n\
         static ptrszint *{}(char *slot);\n",
        f("blank"),
        f("release"),
        f("free"),
        f("make"),
        f("dup"),
        f("copy"),
        f("set"),
        f("guard"),
        f("bound"),
        f("desc")
    )
}
