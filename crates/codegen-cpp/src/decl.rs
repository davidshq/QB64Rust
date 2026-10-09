//! Declarations and allocation: sizes and layouts, the declaration and allocation of variables and arrays,
//! temporaries, `global.txt` and `clear.txt`.

use crate::Emitter;
use crate::names::{c_type, int_const, var_name};
use qb64rust_ir::{MemberId, NEW_TYPE_UNREACHABLE, ProcId, Storage, Ty, Var, size_of, unproduced_types};
use std::fmt::Write as _;

/// The numeric types as a pattern: each is stored as one C scalar ([`c_type`]); a declaration of any of them is
/// emitted alike.
macro_rules! numbers {
    () => {
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
    };
}

/// Whether a variable of this type is a `qbs *` (freed, assigned with `qbs_set`, used without `*`); otherwise it
/// is a pointer to a C scalar or (a user type) to its bytes.
pub(crate) fn is_qbs(t: Ty) -> bool {
    match t {
        Ty::Str => true,
        numbers!() | Ty::User(_) => false,
        unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

/// The declaration of a variable's pointer: an array's descriptor, a user type's bytes, a scalar.
pub(crate) fn declare(v: &Var, n: &str) -> String {
    if v.is_array() {
        return format!("ptrszint *{n}=NULL;\n");
    }
    match v.ty {
        Ty::Str => format!("qbs *{n}=NULL;\n"),
        Ty::User(_) => format!("void *{n}=NULL;\n"),
        t @ numbers!() => format!("{} *{n}=NULL;\n", c_type(t)),
        unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

/// The descriptor slot of dimension `k` (0 for the first) of an array of `n` dimensions: libqb keeps the
/// dimensions in reverse order, four slots each from slot 4 (lower bound, count, multiplier, unused;
/// `study\02` §3.4).
pub(crate) fn dim_slot(n: usize, k: usize) -> usize {
    4 * (n - 1 - k) + 4
}

/// Whether a variable is a plain C variable (a temporary), not a pointer.
pub(crate) fn is_plain(v: &Var) -> bool {
    match v.storage {
        Storage::Temp(_) => true,
        Storage::Global | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => false,
    }
}

/// The declaration of a temporary, zero; `static` in the main module.
fn declare_temp(v: &Var, n: &str) -> String {
    let static_ = match v.storage {
        Storage::Temp(None) => "static ",
        Storage::Temp(Some(_)) => "",
        Storage::Global | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => {
            unreachable!("not a temporary")
        }
    };
    assert!(!is_qbs(v.ty), "the lowering makes numeric temporaries only");
    format!("{static_}{} {n}=0;\n", c_type(v.ty))
}

impl Emitter<'_> {
    /// `f(var, name)` for every variable that lives as long as the program: global and `STATIC` ones.
    pub(crate) fn per_program_var(&self, f: impl Fn(&Var, &str) -> String) -> String {
        self.p
            .vars
            .iter()
            .filter(|v| match v.storage {
                Storage::Global | Storage::Static(_) => true,
                Storage::Local(_) | Storage::Param(_) | Storage::Result(_) | Storage::Temp(_) => false,
            })
            .map(|v| f(v, &var_name(self.p, v)))
            .collect()
    }

    pub(crate) fn global(&self) -> String {
        let mut out = String::from(
            "template <typename QBL, typename QBR> static inline auto qb_safe_idiv(QBL qb_l,QBR qb_r)->decltype(qb_l/qb_r){if (!qb_r){error(11);return (decltype(qb_l/qb_r))0;}return qb_l/qb_r;}\n\
             template <typename QBL, typename QBR> static inline auto qb_safe_mod(QBL qb_l,QBR qb_r)->decltype(qb_l%qb_r){if (!qb_r){error(11);return (decltype(qb_l%qb_r))0;}return qb_l%qb_r;}\n",
        );
        out.push_str(&self.per_program_var(declare));
        // Globals that qbx.cpp and libqb refer to.
        out.push_str(
            "int32 console=1;\nint32 screen_hide_startup=0;\nint32 asserts=0;\nint32 vwatch=0;\n\
             ptrszint data_size=0;\nuint8 *data=(uint8*)calloc(1,1);\n",
        );
        out
    }

    /// The declarations of the temporaries of a body (`None`: the main module), in order of creation.
    pub(crate) fn temps(&self, owner: Option<ProcId>) -> String {
        self.p
            .vars
            .iter()
            .filter(|v| v.storage == Storage::Temp(owner))
            .map(|v| declare_temp(v, &var_name(self.p, v)))
            .collect()
    }

    /// The size of a value of a type in memory: a number's or a user type's (`sema`'s `size_of`, the sizes `LEN`
    /// gives), a string's descriptor pointer.
    pub(crate) fn ty_size(&self, t: Ty) -> u32 {
        match t {
            Ty::Str => 8,
            numbers!() | Ty::User(_) => size_of(&self.p.types, t),
            unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
        }
    }

    /// The byte offset of a member in a value of user type `base`: the sizes of the members before it.
    pub(crate) fn member_offset(&self, base: Ty, m: MemberId) -> u32 {
        let Ty::User(t) = base else {
            unreachable!("a member of {base:?}");
        };
        self.p.user_type(t).members[..m.0 as usize]
            .iter()
            .map(|x| self.ty_size(x.ty))
            .sum()
    }

    /// The allocation of a variable, zero or empty; it runs once per lifetime of the pointer. A static array gets
    /// its descriptor and its zeroed elements (empty strings) as the old compiler builds them (`study\02` §3.4).
    pub(crate) fn alloc(&self, v: &Var, n: &str) -> String {
        if v.is_array() {
            return self.alloc_array(v, n);
        }
        match v.ty {
            Ty::Str => format!("if (!{n}){n}=qbs_new(0,0);\n"),
            Ty::User(_) => {
                let size = self.ty_size(v.ty);
                format!("if({n}==NULL){{\n{n}=(void*)mem_static_malloc({size});\nmemset({n},0,{size});\n}}\n")
            }
            t @ numbers!() => format!(
                "if({n}==NULL){{\n{n}=({c}*)mem_static_malloc({size});\n*{n}=0;\n}}\n",
                c = c_type(t),
                size = self.ty_size(t)
            ),
            unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
        }
    }

    fn alloc_array(&self, v: &Var, n: &str) -> String {
        let dims = v.dims.len();
        let lock = 4 * dims + 4;
        let mut out = format!(
            "if (!{n}){{\n{n}=(ptrszint*)mem_static_malloc({}*ptrsz);\nnew_mem_lock();\nmem_lock_tmp->type=4;\n\
             ((ptrszint*){n})[{lock}]=(ptrszint)mem_lock_tmp;\n",
            lock + 1
        );
        for (k, &(lower, upper)) in v.dims.iter().enumerate() {
            let s = dim_slot(dims, k);
            writeln!(out, "{n}[{s}]={};", int_const(lower, Ty::I64)).unwrap();
            writeln!(out, "{n}[{}]=({})-{n}[{s}]+1;", s + 1, int_const(upper, Ty::I64)).unwrap();
            writeln!(out, "if ({n}[{}]<=0) error(5);", s + 1).unwrap();
            if k == 0 {
                writeln!(out, "{n}[{}]=1;", s + 2).unwrap();
            } else {
                let p = dim_slot(dims, k - 1);
                writeln!(out, "{n}[{}]={n}[{}]*{n}[{}];", s + 2, p + 2, p + 1).unwrap();
            }
        }
        let count = self.element_count(v, n);
        if is_qbs(v.ty) {
            write!(
                out,
                "{n}[0]=(ptrszint)mem_static_malloc({count}*ptrsz);\ntmp_long={count};\nwhile(tmp_long--){{\n\
                 ((qbs**)({n}[0]))[tmp_long]=qbs_new(0,0);\n}}\n"
            )
            .unwrap();
        } else {
            let bytes = format!("{count}*{}", self.ty_size(v.ty));
            write!(
                out,
                "{n}[0]=(ptrszint)mem_static_malloc({bytes});\nmemset((void*)({n}[0]),0,{bytes});\n"
            )
            .unwrap();
        }
        write!(out, "{n}[2]=1+2;\n}}\n").unwrap();
        out
    }

    /// The number of elements of an array as a C expression over its descriptor.
    fn element_count(&self, v: &Var, n: &str) -> String {
        let dims = v.dims.len();
        let counts: Vec<String> = (0..dims).map(|k| format!("{n}[{}]", dim_slot(dims, k) + 1)).collect();
        counts.join("*")
    }

    /// `clear.txt`: what `CLEAR` resets in a variable that lives as long as the program.
    pub(crate) fn clear(&self, v: &Var, n: &str) -> String {
        if v.is_array() {
            let count = self.element_count(v, n);
            return if is_qbs(v.ty) {
                format!("tmp_long={count};\nwhile(tmp_long--){{\n(((qbs**)({n}[0]))[tmp_long])->len=0;\n}}\n")
            } else {
                format!("memset((void*)({n}[0]),0,{count}*{});\n", self.ty_size(v.ty))
            };
        }
        match v.ty {
            Ty::Str => format!("{n}->len=0;\n"),
            Ty::User(_) => format!("memset((void*){n},0,{});\n", self.ty_size(v.ty)),
            numbers!() => format!("*{n}=0;\n"),
            unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
        }
    }
}
