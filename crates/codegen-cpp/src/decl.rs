//! Declarations and allocation: sizes and layouts, the declaration and allocation of variables and arrays,
//! temporaries, `global.txt` and `clear.txt`.

use crate::Emitter;
use crate::names::{c_type, int_const, var_name};
use qb64rust_ir::{MemberId, ProcId, Shape, Storage, Ty, Var, size_of};
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
/// is a pointer to a C scalar or (a user type) to its bytes. A `STRING * n` variable is a fixed `qbs` over its n
/// bytes (design D6); a `STRING * n` element or member is read through a temporary one ([`crate::place::fixed_at`]).
pub(crate) fn is_qbs(t: Ty) -> bool {
    match t {
        Ty::Str | Ty::FixedStr(_) => true,
        numbers!() | Ty::User(_) => false,
    }
}

/// The declaration of a variable's pointer: an array's descriptor, a user type's bytes, a scalar.
pub(crate) fn declare(v: &Var, n: &str) -> String {
    if v.is_array() {
        return format!("ptrszint *{n}=NULL;\n");
    }
    match v.ty {
        Ty::Str | Ty::FixedStr(_) => format!("qbs *{n}=NULL;\n"),
        Ty::User(_) => format!("void *{n}=NULL;\n"),
        t @ numbers!() => format!("{} *{n}=NULL;\n", c_type(t)),
    }
}

/// The descriptor slot of dimension `k` (0 for the first) of an array of `n` dimensions: libqb keeps the
/// dimensions in reverse order, four slots each from slot 4 (lower bound, count, multiplier, unused;
/// `study\02` §3.4).
pub(crate) fn dim_slot(n: usize, k: usize) -> usize {
    4 * (n - 1 - k) + 4
}

/// The descriptor slot that holds the `_MEM` lock of an array of `n` dimensions: the one after its dimensions.
pub(crate) fn lock_slot(n: usize) -> usize {
    4 * n + 4
}

/// The number of dimensions of an array. One that was only listed (`STATIC a()`, with no `DIM`, `REDIM` or use
/// of it below) has none: no statement names it, and its descriptor is made with one.
pub(crate) fn dim_count(v: &Var) -> usize {
    v.dim_count().unwrap_or(1)
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
        // The old compiler's templates, with a divisor of -1 tested for a signed result type (`DIVERGENCES.md` D-006:
        // the smallest value divided by -1 crashed the old program): `\` raises error 6 and gives the dividend, `MOD`
        // gives its exact result, 0.
        let mut out = String::from(
            "template <typename QBL, typename QBR> static inline auto qb_safe_idiv(QBL qb_l,QBR qb_r)->decltype(qb_l/qb_r){if (!qb_r){error(11);return (decltype(qb_l/qb_r))0;}\
             if constexpr (std::is_signed_v<decltype(qb_l/qb_r)>&&std::is_integral_v<decltype(qb_l/qb_r)>){if (qb_r==-1&&qb_l==std::numeric_limits<decltype(qb_l/qb_r)>::min()){error(6);return qb_l;}}\
             return qb_l/qb_r;}\n\
             template <typename QBL, typename QBR> static inline auto qb_safe_mod(QBL qb_l,QBR qb_r)->decltype(qb_l%qb_r){if (!qb_r){error(11);return (decltype(qb_l%qb_r))0;}\
             if constexpr (std::is_signed_v<decltype(qb_l%qb_r)>){if (qb_r==-1)return (decltype(qb_l%qb_r))0;}\
             return qb_l%qb_r;}\n",
        );
        out.push_str(&self.per_program_var(declare));
        // Globals that qbx.cpp and libqb refer to.
        out.push_str("int32 console=1;\nint32 screen_hide_startup=0;\nint32 asserts=0;\nint32 vwatch=0;\n");
        out.push_str(&self.data_fragment());
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

    /// The size of a value of a type in memory: a number's, a fixed-length string's or a user type's (`sema`'s
    /// `size_of`, the sizes `LEN` gives), a string's descriptor pointer.
    pub(crate) fn ty_size(&self, t: Ty) -> u32 {
        match t {
            Ty::Str => 8,
            numbers!() | Ty::FixedStr(_) | Ty::User(_) => size_of(&self.p.types, t),
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
            // NUL bytes, as the old compiler allocates them (`DIVERGENCES-QB45.md` Q-005).
            Ty::FixedStr(k) => format!(
                "if({n}==NULL){{\n{n}=qbs_new_fixed((uint8*)mem_static_malloc({k}),{k},0);\nmemset({n}->chr,0,{k});\n}}\n"
            ),
            Ty::User(_) => {
                let size = self.ty_size(v.ty);
                format!("if({n}==NULL){{\n{n}=(void*)mem_static_malloc({size});\nmemset({n},0,{size});\n}}\n")
            }
            t @ numbers!() => format!(
                "if({n}==NULL){{\n{n}=({c}*)mem_static_malloc({size});\n*{n}=0;\n}}\n",
                c = c_type(t),
                size = self.ty_size(t)
            ),
        }
    }

    /// The descriptor of an array, made once per lifetime of its pointer. A static array gets its bounds and its
    /// elements here; a dynamic one is left in the state libqb reads as "not allocated" (`study\02` §3.4: the flag
    /// for "defined" clear, lower bound 2147483647 and no element in every dimension, the data pointer at eight
    /// zero bytes or at the empty string), so every access raises error 9 until an `Allocate` runs.
    fn alloc_array(&self, v: &Var, n: &str) -> String {
        let dims = dim_count(v);
        let lock = lock_slot(dims);
        let mut out = format!(
            "if (!{n}){{\n{n}=(ptrszint*)mem_static_malloc({}*ptrsz);\nnew_mem_lock();\nmem_lock_tmp->type=4;\n\
             ((ptrszint*){n})[{lock}]=(ptrszint)mem_lock_tmp;\n",
            lock + 1
        );
        if let Shape::Dynamic { .. } = v.shape {
            out.push_str(&format!("{n}[2]=0;\n{}}}\n", self.not_allocated(v, n)));
            return out;
        }
        for (k, &(lower, upper)) in v.bounds().iter().enumerate() {
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
        if v.ty == Ty::Str {
            write!(
                out,
                "{n}[0]=(ptrszint)mem_static_malloc({count}*ptrsz);\ntmp_long={count};\nwhile(tmp_long--){{\n\
                 ((qbs**)({n}[0]))[tmp_long]=qbs_new(0,0);\n}}\n"
            )
            .unwrap();
        } else {
            let bytes = self.array_bytes(v.ty, &count);
            write!(
                out,
                "{n}[0]=(ptrszint)mem_static_malloc({bytes});\nmemset((void*)({n}[0]),0,{bytes});\n"
            )
            .unwrap();
        }
        write!(out, "{n}[2]=1+2;\n}}\n").unwrap();
        out
    }

    /// The size in bytes of `count` (a C expression) elements of a type that is no `STRING`. `_BIT * n` elements
    /// are packed, n bits each, in `count * n / 8 + 1` bytes (measured, `study\00` §5: the old compiler's
    /// expression, which `getbits` and `setbits` rely on).
    pub(crate) fn array_bytes(&self, t: Ty, count: &str) -> String {
        match t {
            Ty::Bit { width, .. } => format!("{count}*{width}/8+1"),
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
            | Ty::FixedStr(_)
            | Ty::User(_) => format!("{count}*{}", self.ty_size(t)),
            Ty::Str => unreachable!("a string array holds pointers"),
        }
    }

    /// The number of elements of an array as a C expression over its descriptor (0 while it is not allocated).
    pub(crate) fn element_count(&self, v: &Var, n: &str) -> String {
        let dims = dim_count(v);
        let counts: Vec<String> = (0..dims).map(|k| format!("{n}[{}]", dim_slot(dims, k) + 1)).collect();
        counts.join("*")
    }

    /// The lines that leave the bounds and the data pointer of a dynamic array's descriptor as "not allocated"
    /// (the flags are the caller's).
    fn not_allocated(&self, v: &Var, n: &str) -> String {
        let mut out = String::new();
        for i in 1..=dim_count(v) {
            let s = 4 * i;
            writeln!(out, "{n}[{s}]=2147483647;\n{n}[{}]=0;\n{n}[{}]=0;", s + 1, s + 2).unwrap();
        }
        let nothing = if v.ty == Ty::Str {
            "&nothingstring"
        } else {
            "nothingvalue"
        };
        writeln!(out, "{n}[0]=(ptrszint){nothing};").unwrap();
        out
    }

    /// The release of a dynamic array, as the old compiler writes it for `ERASE` (`qb64pe.bas` 8637–8704): if it is
    /// allocated, its strings and its block are freed and its descriptor is "not allocated" again.
    pub(crate) fn release(&self, v: &Var, n: &str) -> String {
        let mut out = format!("if ({n}[2]&1){{\n");
        if v.ty == Ty::Str {
            let count = self.element_count(v, n);
            write!(
                out,
                "tmp_long={count};\nwhile(tmp_long--){{\nqbs_free(((qbs**)({n}[0]))[tmp_long]);\n}}\n"
            )
            .unwrap();
        }
        write!(
            out,
            "free((void*)({n}[0]));\n{n}[2]^=1;\n{}}}\n",
            self.not_allocated(v, n)
        )
        .unwrap();
        out
    }

    /// The elements of an array that has its bounds, zero, empty or NUL bytes.
    pub(crate) fn clear_elements(&self, v: &Var, n: &str) -> String {
        let count = self.element_count(v, n);
        if v.ty == Ty::Str {
            format!("tmp_long={count};\nwhile(tmp_long--){{\n(((qbs**)({n}[0]))[tmp_long])->len=0;\n}}\n")
        } else {
            format!("memset((void*)({n}[0]),0,{});\n", self.array_bytes(v.ty, &count))
        }
    }

    /// `clear.txt`: what `CLEAR` resets in a variable that lives as long as the program. A static array is
    /// cleared, a dynamic one released, as `ERASE` does (`qb64pe.bas` 12749–12756).
    pub(crate) fn clear(&self, v: &Var, n: &str) -> String {
        if let Shape::Dynamic { .. } = v.shape {
            return self.release(v, n);
        }
        if v.is_array() {
            return self.clear_elements(v, n);
        }
        match v.ty {
            Ty::Str => format!("{n}->len=0;\n"),
            Ty::User(_) => format!("memset((void*){n},0,{});\n", self.ty_size(v.ty)),
            numbers!() => format!("*{n}=0;\n"),
            Ty::FixedStr(k) => format!("memset((void*)({n}->chr),0,{k});\n"),
        }
    }
}
