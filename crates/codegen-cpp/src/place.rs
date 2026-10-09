//! Places: variables, elements and members as C lvalues, addresses and references.

use crate::Emitter;
use crate::decl::{dim_slot, is_plain, is_qbs};
use crate::names::c_type;
use qb64rust_ir::{Expr, Place, Ty, VarId};

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

    /// The element a member place is found in, its indexes, and the member's byte offset in the element.
    pub(crate) fn element_root<'v>(&self, place: &'v Place) -> (VarId, &'v [Expr], u32) {
        match place {
            Place::Element { array, index } => (*array, index, 0),
            Place::Member { base, member } => {
                let (array, index, offset) = self.element_root(base);
                (
                    array,
                    index,
                    offset + self.member_offset(self.p.place_ty(base), *member),
                )
            }
            Place::Var(_) => unreachable!("a member place without an element"),
        }
    }

    /// The flat position of an element, with libqb's `array_check` per dimension (error 9 outside the bounds; it
    /// then gives 0, the first position, as the old compiler reads), first dimension fastest.
    pub(crate) fn flat_index(&mut self, array: VarId, index: &[Expr]) -> String {
        let n = index.len();
        let name = self.name(array);
        let parts: Vec<String> = index
            .iter()
            .enumerate()
            .map(|(k, i)| {
                let s = dim_slot(n, k);
                let check = format!("array_check(({})-{name}[{s}],{name}[{}])", self.value(i), s + 1);
                if k == 0 {
                    check
                } else {
                    format!("{check}*{name}[{}]", s + 2)
                }
            })
            .collect();
        parts.join("+")
    }

    /// The element at flat position `flat` of a numeric or string array, as an lvalue; of a fixed-length string
    /// array, a temporary fixed `qbs` over its bytes.
    pub(crate) fn element_at(&self, array: VarId, flat: &str) -> String {
        let (v, name) = (self.p.var(array), self.name(array));
        if let Ty::FixedStr(k) = v.ty {
            fixed_at(&format!("&((uint8*)({name}[0]))[({flat})*{k}]"), k)
        } else if is_qbs(v.ty) {
            format!("(((qbs**)({name}[0]))[{flat}])")
        } else {
            format!("(({}*)({name}[0]))[{flat}]", c_type(v.ty))
        }
    }

    /// The address of a place of a user type, or of a member, as a `char*` expression.
    fn bytes_of(&mut self, place: &Place) -> String {
        match place {
            Place::Var(v) => format!("((char*){})", self.name(*v)),
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                let size = self.ty_size(self.p.var(*array).ty);
                format!("(((char*){}[0])+(({flat})*{size}))", self.name(*array))
            }
            Place::Member { base, member } => {
                let offset = self.member_offset(self.p.place_ty(base), *member);
                format!("({}+{offset})", self.bytes_of(base))
            }
        }
    }

    /// A place as a C lvalue: a scalar as [`Self::scalar`], an element, a member through a typed pointer (a
    /// fixed-length string member through a temporary fixed `qbs` over its bytes, as the old compiler's
    /// `udtreference`).
    pub(crate) fn load_place(&mut self, place: &Place) -> String {
        match place {
            Place::Var(id) => self.scalar(*id),
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                self.element_at(*array, &flat)
            }
            Place::Member { .. } => {
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
        match place {
            Place::Var(id) => self.name(*id),
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                let element = self.element_at(*array, &flat);
                if is_qbs(self.p.var(*array).ty) {
                    element
                } else {
                    format!("(&{element})")
                }
            }
            Place::Member { .. } => {
                let ty = self.p.place_ty(place);
                if let Ty::FixedStr(_) = ty {
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
