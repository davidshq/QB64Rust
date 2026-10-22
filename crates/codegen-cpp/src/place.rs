//! Places: variables, elements and members as C lvalues, addresses and references.
//!
//! A member array lies in the value it is a member of (design D8 of `m2-defxxx-common-arrays`): its elements
//! follow each other from the member's offset, first dimension fastest, and its bounds are constants of its
//! user type, so an element is the member's address plus its checked flat position times the element size.

use crate::Emitter;
use crate::decl::{dim_slot, is_plain, is_qbs};
use crate::names::{c_type, int_const};
use qb64rust_ir::{ArrayRef, Expr, MemberId, Place, Ty, VarId};

impl Emitter<'_> {
    /// A variable as a C expression: a `qbs*` or a temporary as it is, any other through its pointer.
    pub(crate) fn scalar(&self, id: VarId) -> String {
        let (v, name) = (self.p.var(id), self.name(id));
        if is_qbs(v.ty) || is_plain(v) {
            name
        } else {
            format!("*{name}")
        }
    }

    /// The flat position of an element, with libqb's `array_check` per dimension (error 9 outside the bounds; it
    /// then gives 0, the first position, as the old compiler reads), first dimension fastest. Below
    /// `$CHECKING:OFF` without it (`qb64pe.bas` 15627–15640, 16136–16149, 16335–16348). With no index, 0: the
    /// first element of a whole array taken as a value, unchecked as the old compiler writes it (design D4 of
    /// `m4-line-numbers-whole-arrays`; an array not allocated points at libqb's `nothingvalue`, whose 8 bytes are
    /// zeros).
    pub(crate) fn flat_index(&mut self, array: VarId, index: &[Expr]) -> String {
        let name = self.name(array);
        self.flat_index_in(&name, index)
    }

    /// [`Self::flat_index`] against the descriptor named `desc`: an array variable's, or the `passN` that holds
    /// a dynamic member array's.
    fn flat_index_in(&mut self, desc: &str, index: &[Expr]) -> String {
        let n = index.len();
        if n == 0 {
            return "0".to_string();
        }
        let parts: Vec<String> = index
            .iter()
            .enumerate()
            .map(|(k, i)| {
                let s = dim_slot(n, k);
                let i = self.value(i);
                match (self.checking, k) {
                    (true, 0) => format!("array_check(({i})-{desc}[{s}],{desc}[{}])", s + 1),
                    (true, _) => format!("array_check(({i})-{desc}[{s}],{desc}[{}])*{desc}[{}]", s + 1, s + 2),
                    (false, 0) => format!("({i})-{desc}[{s}]"),
                    (false, _) => format!("(({i})-{desc}[{s}])*{desc}[{}]", s + 2),
                }
            })
            .collect();
        parts.join("+")
    }

    /// The lower bound, the number of elements and the multiplier of each dimension of a member array, first
    /// dimension first: the multiplier of a dimension is the number of elements of the dimensions before it.
    pub(crate) fn member_dims(&self, base: &Place, member: MemberId) -> Vec<(i64, i64, i64)> {
        let mut multiplier = 1;
        let bounds = &self.p.member_of(base, member).bounds;
        bounds
            .iter()
            .map(|&(lower, upper)| {
                let count = upper - lower + 1;
                let dim = (lower, count, multiplier);
                multiplier *= count;
                dim
            })
            .collect()
    }

    /// [`Self::flat_index`] for an element of a member array, whose bounds are constants: an index outside
    /// raises error 9 and counts as 0, position 0 of its dimension (measured, `verification\v24_h_member_store`).
    pub(crate) fn member_flat_index(&mut self, base: &Place, member: MemberId, index: &[Expr]) -> String {
        let dims = self.member_dims(base, member);
        let parts: Vec<String> = index
            .iter()
            .zip(dims)
            .map(|(i, (lower, count, multiplier))| {
                let lower = int_const(lower, Ty::I64);
                let i = self.value(i);
                let check = if self.checking {
                    format!("array_check(({i})-{lower},{count})")
                } else {
                    format!("(({i})-{lower})")
                };
                if multiplier == 1 {
                    check
                } else {
                    format!("{check}*{multiplier}")
                }
            })
            .collect();
        parts.join("+")
    }

    /// The address of the first element of a member array, as a `char*` expression.
    pub(crate) fn member_array_at(&mut self, base: &Place, member: MemberId) -> String {
        let offset = self.member_offset(self.p.place_ty(base), member);
        format!("({}+{offset})", self.bytes_of(base))
    }

    /// The element at flat position `flat` of a numeric or string array, as an lvalue; of a fixed-length string
    /// array, a temporary fixed `qbs` over its bytes; of a `_BIT * n` array, the value of its packed element (it
    /// is stored by [`Self::store`]).
    pub(crate) fn element_at(&self, array: VarId, flat: &str) -> String {
        let (v, name) = (self.p.var(array), self.name(array));
        if let Ty::Bit { width, signed } = v.ty {
            let get = if signed { "getbits" } else { "getubits" };
            format!("(({})({get}({width},(uint8*)({name}[0]),{flat})))", c_type(v.ty))
        } else if let Ty::FixedStr(k) = v.ty {
            fixed_at(&format!("&((uint8*)({name}[0]))[({flat})*{k}]"), k)
        } else if is_qbs(v.ty) {
            format!("(((qbs**)({name}[0]))[{flat}])")
        } else {
            format!("(({}*)({name}[0]))[{flat}]", c_type(v.ty))
        }
    }

    /// The address of a place of a user type, of a member, or of an element of a member array, as a `char*`
    /// expression.
    pub(crate) fn bytes_of(&mut self, place: &Place) -> String {
        match place {
            Place::Var(v) => format!("((char*){})", self.name(*v)),
            Place::Element {
                array: ArrayRef::Var(array),
                index,
            } => {
                let flat = self.flat_index(*array, index);
                let size = self.ty_size(self.p.var(*array).ty);
                format!("(((char*){}[0])+(({flat})*{size}))", self.name(*array))
            }
            Place::Element {
                array: ArrayRef::Member { base, member },
                index,
            } if self.dynamic_member(base, *member).is_some() => self.dyn_element(base, *member, index),
            Place::Element {
                array: ArrayRef::Member { base, member },
                index,
            } => {
                let first = self.member_array_at(base, *member);
                let flat = self.member_flat_index(base, *member, index);
                let size = self.ty_size(self.p.member_of(base, *member).ty);
                format!("({first}+(({flat})*{size}))")
            }
            Place::Member { base, member } => {
                let offset = self.member_offset(self.p.place_ty(base), *member);
                format!("({}+{offset})", self.bytes_of(base))
            }
        }
    }

    /// The address of an element of a dynamic member array (design D9 of `m3-typefields`), as a `char*`
    /// expression: the place it is part of found once (an index outside its bounds raises error 9 and position 0
    /// stands in), its descriptor through the guard (error 9 when it is not allocated or has another number of
    /// dimensions, and then a dummy element), then the element's flat position from the descriptor, each index
    /// checked as a dynamic array's. The descriptor is held in a `passN` of the body.
    pub(crate) fn dyn_element(&mut self, base: &Place, member: MemberId, index: &[Expr]) -> String {
        self.dyn_element_desc(base, member, index).0
    }

    /// [`Self::dyn_element`], with the `passN` that holds the element's descriptor once the address is found. The
    /// indexes of `base` are found before it and may take `passN`s of their own.
    pub(crate) fn dyn_element_desc(&mut self, base: &Place, member: MemberId, index: &[Expr]) -> (String, String) {
        let (id, m) = self.dynamic_member(base, member).expect("a dynamic member array");
        let size = self.ty_size(self.p.member_of(base, member).ty);
        let slot = self.dyn_slot(base, member);
        self.pass += 1;
        let d = format!("pass{}", self.pass);
        self.pass_decls.push(format!("ptrszint *{d};\n"));
        let flat = self.flat_index_in(&d, index);
        let at = format!(
            "(({d}={}(*(ptrszint**){slot},{n})),(((char*){d}[0])+(({flat})*{size})))",
            crate::owner::dyn_fn(id, m, "guard"),
            n = index.len()
        );
        (at, d)
    }

    /// A block passed to the runtime (a `_MEM` argument of `_MEMEXISTS`, `_MEMFREE`, `_MEMCOPY`, `_MEMGET`;
    /// design D4 of `m3-mem`): the address of its place, a variable spelled as the old compiler spells it
    /// (`(void*)( ((char*)(__UDT_M)) + ((0+0)) )`, measured, `verification\v32_x01_accepted`).
    pub(crate) fn mem_ref(&mut self, place: &Place) -> String {
        match place {
            Place::Var(v) => format!("(void*)( ((char*)({})) + ((0+0)) )", self.name(*v)),
            Place::Element { .. } | Place::Member { .. } => format!("(void*)({})", self.bytes_of(place)),
        }
    }

    /// A place as a C lvalue: a scalar as [`Self::scalar`], an element, a member or an element of a member
    /// array through a typed pointer (a `STRING` one is the `qbs*` in its slot; a fixed-length string one is
    /// read through a temporary fixed `qbs` over its bytes, as the old compiler's `udtreference`).
    pub(crate) fn load_place(&mut self, place: &Place) -> String {
        match place {
            Place::Var(id) => self.scalar(*id),
            Place::Element {
                array: ArrayRef::Var(array),
                index,
            } => {
                let flat = self.flat_index(*array, index);
                self.element_at(*array, &flat)
            }
            Place::Member { .. }
            | Place::Element {
                array: ArrayRef::Member { .. },
                ..
            } => {
                let ty = self.p.place_ty(place);
                if let Ty::FixedStr(k) = ty {
                    return fixed_at(&format!("(uint8*){}", self.bytes_of(place)), k);
                }
                format!("*({}*)({})", c_type(ty), self.bytes_of(place))
            }
        }
    }

    /// A place passed by reference: a pointer to it, or a string's own `qbs*`; found in argument order, as the old
    /// compiler passes them (`study\02` §4.1).
    pub(crate) fn place_ref(&mut self, place: &Place) -> String {
        // A place of a user type: its bytes (a variable's pointer is their address).
        if let Ty::User(_) = self.p.place_ty(place) {
            return match place {
                Place::Var(id) => self.name(*id),
                Place::Element { .. } | Place::Member { .. } => format!("((void*){})", self.bytes_of(place)),
            };
        }
        match place {
            Place::Var(id) => self.name(*id),
            Place::Element {
                array: ArrayRef::Var(array),
                index,
            } => {
                let flat = self.flat_index(*array, index);
                let element = self.element_at(*array, &flat);
                if is_qbs(self.p.var(*array).ty) {
                    element
                } else {
                    format!("(&{element})")
                }
            }
            Place::Member { .. }
            | Place::Element {
                array: ArrayRef::Member { .. },
                ..
            } => {
                // A string is its own `qbs*`: the one in its slot, or a fixed one over its bytes.
                let ty = self.p.place_ty(place);
                if is_qbs(ty) {
                    return self.load_place(place);
                }
                format!("({}*)(void*)({})", c_type(ty), self.bytes_of(place))
            }
        }
    }
}

/// A temporary fixed `qbs` over the `k` bytes at `addr` (a `uint8*` expression): the old compiler's
/// `qbs_new_fixed(addr,k,1)`, freed by the statement's `qbs_cleanup`; `qbs_set` into it cuts and pads.
pub(crate) fn fixed_at(addr: &str, k: u32) -> String {
    format!("qbs_new_fixed({addr},{k},1)")
}
