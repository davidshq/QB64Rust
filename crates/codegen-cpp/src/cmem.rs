//! Conventional memory (design D1–D3 of `m4-memory-records`): the variables a `VARPTR`, `VARSEG`, `SADD` or
//! `VARPTR$` names, allocated from the top of libqb's `dblock` as the old compiler allocates them, and the texts of
//! those four functions.
//!
//! `dblock` is `&cmem[1280]`, the segment 80 (`libqb.cpp` 25968). libqb takes its first 16 bytes from the top for
//! `nothingvalue` and two strings; each variable then takes its bytes below the last, in the order the old
//! compiler's data fragment declares them (measured, `verification\v56_a1`–`a3`: from 65520 down, no alignment).

use crate::Emitter;
use crate::names::c_type;
use qb64rust_ir::{AddressWord, ArrayRef, Place, Storage, Ty, Var, numeric_types};

impl Emitter<'_> {
    /// Whether the program has a variable in `dblock`: a scalar in conventional memory, or an array of strings
    /// there, whose elements' descriptors lie in it. A `_BIT` scalar of the main module then lies there too.
    pub(crate) fn uses_dblock(&self) -> bool {
        self.p
            .vars
            .iter()
            .any(|v| v.conventional && (!v.is_array() || v.ty == Ty::Str))
    }

    /// Whether the variable is allocated from `dblock`: a scalar in conventional memory, and every `_BIT` scalar of
    /// the main module when the program has another variable there (the old compiler allocates each `_BIT` scalar
    /// there, `qb64pe.bas` 18215; measured, `verification\v56_a3`, `v56_x06`).
    pub(crate) fn in_dblock(&self, v: &Var) -> bool {
        if v.is_array() {
            return false;
        }
        v.conventional || (matches!(v.ty, Ty::Bit { .. }) && v.storage == Storage::Global && self.uses_dblock())
    }

    /// The allocation of a scalar from `dblock`, the old compiler's text for each kind (`qb64pe.bas` 17790, 18008,
    /// 18109, 18215–18733; read from its C++ of `verification\v56_a1`, `a3`, `a4`).
    pub(crate) fn alloc_dblock(&self, v: &Var, n: &str) -> String {
        let check = "if (cmem_sp<qbs_cmem_sp) error(257);";
        match v.ty {
            Ty::Str => format!("if (!{n}){n}=qbs_new_cmem(0,0);\n"),
            Ty::FixedStr(k) => format!(
                "if({n}==NULL){{\ncmem_sp-={k};\n{check}\n{n}=qbs_new_fixed((uint8*)(dblock+cmem_sp),{k},0);\n\
                 memset({n}->chr,0,{k});\n}}\n"
            ),
            // Its strings are made as in ordinary memory, where the old compiler leaves them out and its program
            // crashes at the first use of one (`qb64pe.bas` 17790–17852; `QB64PE-DIVERGENCES.md` D-037).
            Ty::User(_) => {
                let size = self.ty_size(v.ty);
                let mut strings = self.new_value(v.ty, &format!("(char*){n}"));
                if !strings.is_empty() {
                    strings.insert_str(0, "qbr_diverged(37);\n");
                }
                format!(
                    "if({n}==NULL){{\ncmem_sp-={size};\n{check}\n{n}=(void*)(dblock+cmem_sp);\nmemset({n},0,{size});\n\
                     {strings}}}\n"
                )
            }
            Ty::Bit { .. } => format!(
                "if({n}==NULL){{\ncmem_sp-=4;\n{check}\n{n}=({}*)(dblock+cmem_sp);\n*{n}=0;\n}}\n",
                c_type(v.ty)
            ),
            t @ numeric_types!() => format!(
                "if({n}==NULL){{\ncmem_sp-={};\n{n}=({}*)(dblock+cmem_sp);\n{check}\n*{n}=0;\n}}\n",
                self.ty_size(t),
                c_type(t)
            ),
        }
    }

    /// A new slot `passN` in conventional memory for a value of numeric type `t`, allocated once per run of the body
    /// that holds it, among its variables after the first `made` (design D2 of `qbasic-testcases`; the old
    /// compiler's text, its C++ of `verification\v61_p4`, `v61_c4`): its name. In the main module the pointer is
    /// `static`, as the old compiler's is global.
    pub(crate) fn cmem_slot(&mut self, t: Ty, made: u32) -> String {
        self.pass += 1;
        let n = format!("pass{}", self.pass);
        let (c, size) = (c_type(t), self.ty_size(t));
        let static_ = if self.k == 0 { "static " } else { "" };
        self.slots.push((
            made,
            format!(
                "{static_}{c} *{n}=NULL;\nif({n}==NULL){{\ncmem_sp-={size};\n{n}=({c}*)(dblock+cmem_sp);\n\
                 if (cmem_sp<qbs_cmem_sp) error(257);\n}}\n"
            ),
        ));
        n
    }

    /// The allocations of a body's variables (each with its index) and its slots ([`Self::cmem_slot`]), the slots
    /// placed among the variables as they were made.
    pub(crate) fn with_slots(&mut self, allocs: Vec<(usize, String)>) -> String {
        let mut slots = std::mem::take(&mut self.slots);
        slots.sort_by_key(|&(made, _)| made);
        let mut slots = slots.into_iter().peekable();
        let mut out = String::new();
        for (i, text) in allocs {
            while let Some((_, s)) = slots.next_if(|&(made, _)| made as usize <= i) {
                out.push_str(&s);
            }
            out.push_str(&text);
        }
        out.extend(slots.map(|(_, s)| s));
        out
    }

    /// `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` of a place whose variable is in conventional memory, as the old
    /// compiler writes them (`qb64pe.bas` 21306–21560; its C++ of `verification\v56_*`, design D2 and D3).
    ///
    /// `VARPTR$` of a parameter, as the old compiler's, assumes its variable is in `dblock` even when the
    /// argument lies in an array outside it (`verification\v61_p10`; a QB64pe behaviour to review in `SOMEDAY.md`).
    pub(crate) fn cmem_address(&mut self, word: AddressWord, of: &Place) -> String {
        let root = of.root();
        let v = self.p.var(root);
        let n = self.name(root);
        // A parameter that is no array may point into an array, outside `dblock`: libqb's checks give the offset
        // and segment of where it lies (`qb64pe.bas` 21468–21552, `qbx.cpp`'s `varptr_dblock_check`; measured,
        // `verification\v61_p5`, `p6`). A string's descriptor, and the segment of a string, are as for a variable.
        // `VARPTR` of a place in a `TYPE` parameter is the offset from `dblock`, as for a variable: the old compiler
        // looks the parameter up by the place's offset, not its id (`qb64pe.bas` 21464; its C++ of `v61_p1`;
        // measured, `v61_p9`).
        if matches!(v.storage, Storage::Param(_)) && !v.is_array() {
            let at = match (of, v.ty) {
                (_, Ty::Str) | (_, Ty::FixedStr(_)) if word == AddressWord::VarSeg => None,
                (Place::Var(_), Ty::FixedStr(_)) => Some(format!("{n}->chr")),
                (Place::Var(_), numeric_types!()) => Some(format!("(uint8*){n}")),
                (_, Ty::User(_)) if word == AddressWord::VarSeg => {
                    Some(format!("((uint8*){n})+({})", self.udt_offset(of)))
                }
                _ => None,
            };
            match (word, at) {
                (AddressWord::VarPtr, Some(at)) => return format!("varptr_dblock_check({at})"),
                (AddressWord::VarSeg, Some(at)) => return format!("varseg_dblock_check({at})"),
                (AddressWord::VarSeg, None) => return "80".to_string(),
                _ => {}
            }
        }
        match word {
            // 80 for a variable and an array of strings; the segment of any other array's data.
            AddressWord::VarSeg => {
                if v.is_array() && v.ty != Ty::Str {
                    format!("( ( ((ptrszint)({n}[0])) - ((ptrszint)(&cmem[0])) ) /16)")
                } else {
                    "80".to_string()
                }
            }
            AddressWord::Sadd => format!("((unsigned short)({}->chr-&cmem[1280]))", self.string_of(of)),
            AddressWord::VarPtrStr => {
                let t = varptr_type(self.p.place_ty(of));
                let offset = match (of, v.ty) {
                    (Place::Var(_), Ty::Str | Ty::FixedStr(_)) | (Place::Element { .. }, _) => {
                        format!("{}->cmem_descriptor_offset", self.string_of(of))
                    }
                    // A member gives its variable's offset, without the member's (`verification\v56_t05`).
                    (Place::Var(_) | Place::Member { .. }, _) => {
                        format!("((unsigned short)(((uint8*){n})-&cmem[1280]))")
                    }
                };
                format!("func_varptr_helper({t},{offset})")
            }
            AddressWord::VarPtr => match (of, v.ty) {
                (Place::Var(_), Ty::Str) => format!("{n}->cmem_descriptor_offset"),
                (Place::Var(_), Ty::FixedStr(_)) => format!("((unsigned short)({n}->chr-&cmem[1280]))"),
                (Place::Var(_), _) => format!("((unsigned short)(((uint8*){n})-&cmem[1280]))"),
                (_, Ty::Str) => format!("{}->cmem_descriptor_offset", self.string_of(of)),
                // An element of an array of numbers or fixed-length strings: its offset from the array's start,
                // which a segment names.
                (
                    Place::Element {
                        array: ArrayRef::Var(a),
                        index,
                    },
                    _,
                ) if !matches!(v.ty, Ty::User(_)) => {
                    let flat = self.flat_index(*a, index);
                    format!("(({flat})*{})", self.ty_size(v.ty))
                }
                // A place in a value of a user type: in an array, its offset from the array's start; in a
                // variable, the variable's offset plus its own.
                _ if v.is_array() => format!("({})", self.udt_offset(of)),
                _ => format!(
                    "((unsigned short)(((uint8*){n}) - &cmem[1280] + ({}) ))",
                    self.udt_offset(of)
                ),
            },
        }
    }

    /// The array's place of a graphics `GET` or `PUT` that is no array of numbers ([`StmtArg::Rest`]), as the old
    /// compiler writes it (its C++ of `verification\v56_d` and of each form, task 1.2 of `m4-memory-records`): a
    /// string's characters, a number's bytes, a fixed-length string's characters, and from a place in a user type
    /// or in an array of fixed-length strings or of a user type the bytes to the end of its variable or array.
    ///
    /// [`StmtArg::Rest`]: qb64rust_ir::StmtArg::Rest
    pub(crate) fn rest_bytes(&mut self, place: &Place) -> String {
        let root = place.root();
        let v = self.p.var(root);
        let (ty, n) = (v.ty, self.name(root));
        let (at, size) = match (place, ty) {
            (Place::Var(_), Ty::Str) => (format!("{n}->chr"), format!("{n}->len")),
            (Place::Var(_), Ty::FixedStr(k)) => (format!("{n}->chr"), k.to_string()),
            (_, Ty::Str) => {
                let s = self.string_of(place);
                (format!("{s}->chr"), format!("{s}->len"))
            }
            (
                Place::Element {
                    array: ArrayRef::Var(a),
                    index,
                },
                _,
            ) => return self.block(*a, index),
            (_, Ty::User(_)) if v.is_array() => {
                let o = self.udt_offset(place);
                let size = self.ty_size(ty);
                let counts: Vec<String> = (1..=v.dim_count().unwrap_or(1))
                    .map(|i| format!("{n}[{}]", 4 * i + 1))
                    .collect();
                (
                    format!("(((char*){n}[0])+({o}))"),
                    format!("({size}*({n}[2]&1)*{})-({o})", counts.join("*")),
                )
            }
            (_, Ty::User(_)) => {
                let o = self.udt_offset(place);
                (format!("(((char*){n})+({o}))"), format!("{}-({o})", self.ty_size(ty)))
            }
            (_, t) => (n, self.ty_size(t).to_string()),
        };
        let t = self.byte_element_temp();
        format!("byte_element((uint64){at},{size},{t})")
    }

    /// The offset of a place in a value of a user type, as the old compiler builds it (`o$` of `qb64pe.bas`
    /// 21440–21470): `(e+c)`, where `e` is the offset found so far (the element's position times its size in an
    /// array, else 0, then each element of a member array on the way) and `c` the members' offsets since.
    fn udt_offset(&mut self, place: &Place) -> String {
        let (e, c) = self.udt_parts(place);
        format!("({e}+{c})")
    }

    fn udt_parts(&mut self, place: &Place) -> (String, u32) {
        match place {
            Place::Var(_) => ("0".to_string(), 0),
            Place::Element {
                array: ArrayRef::Var(a),
                index,
            } => {
                let flat = self.flat_index(*a, index);
                (format!("({flat})*{}", self.ty_size(self.p.var(*a).ty)), 0)
            }
            Place::Element {
                array: ArrayRef::Member { base, member },
                index,
            } => {
                let (e, c) = self.udt_parts(base);
                let c = c + self.member_offset(self.p.place_ty(base), *member);
                let flat = self.member_flat_index(base, *member, index);
                let size = self.ty_size(self.p.member_of(base, *member).ty);
                (format!("(({e}+{c})+(({flat})*{size}))"), 0)
            }
            Place::Member { base, member } => {
                let (e, c) = self.udt_parts(base);
                (e, c + self.member_offset(self.p.place_ty(base), *member))
            }
        }
    }

    /// The `qbs*` of a variable-length or fixed-length string variable, or of an element of an array of strings
    /// (the first one for a whole array, `s$()`).
    fn string_of(&mut self, of: &Place) -> String {
        match of {
            Place::Var(v) => self.name(*v),
            Place::Element {
                array: ArrayRef::Var(a),
                index,
            } => {
                let flat = self.flat_index(*a, index);
                self.element_at(*a, &flat)
            }
            Place::Element { .. } | Place::Member { .. } => unreachable!("a string of a member (`validate`)"),
        }
    }
}

/// The type byte of `VARPTR$` (`qb64pe.bas` 21359–21385): 3 for a string of either kind, 0 for a user type; for a
/// number 1, 2, 20 and 5 by its 8, 16, 32 and 64 bits, 4, 8 and 6 for `SINGLE`, `DOUBLE` and `_FLOAT`; `_BIT * n`
/// 64 + n; plus 128 when unsigned (measured, `verification\v56_b1`, `b5`, `t04`).
fn varptr_type(t: Ty) -> u8 {
    let unsigned = if t.is_unsigned() { 128 } else { 0 };
    let kind = match t {
        Ty::Str | Ty::FixedStr(_) => return 3,
        Ty::User(_) => return 0,
        // The old compiler adds the width's low 6 bits (`sourcetyp AND 63`).
        Ty::Bit { width, .. } => 64 + (width & 63),
        Ty::F32 => 4,
        Ty::F64 => 8,
        Ty::F80 => 6,
        Ty::I8 | Ty::U8 => 1,
        Ty::I16 | Ty::U16 => 2,
        Ty::I32 | Ty::U32 => 20,
        Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => 5,
    };
    unsigned + kind
}
