//! Memory blocks of references (design D3 and D6 of `m3-mem`): the five arguments of libqb's `func__mem` for what
//! `_MEM` and `_MEMELEMENT` name, and the address `_OFFSET` gives, as the old compiler writes them
//! (`qb64pe.bas` 22406–23038, its "-6", "-7" and "-8" helpers; measured, `verification\v32_x01_accepted`, the
//! values in `v32_a_blocks`; one line per kind in the change's design D3).
//!
//! An index on the way is written twice, once in the address and once in the size, as the old compiler writes
//! it. The lock is `sf_mem_lock`, the lock of the body the statement stands in, except for an array variable of
//! numbers or of fixed-length strings and its elements, which take the array's own lock from the descriptor (for
//! an array parameter always its own slot: `DIVERGENCES.md` D-016).

use crate::Emitter;
use crate::decl::{dim_count, lock_slot};
use crate::names::c_type;
use crate::place::fixed_at;
use qb64rust_ir::{ArrayRef, Conv, Expr, ExprKind, MemValue, Place, Program, Reference, Ty, VarId};

/// The numeric types a block or an address may be of: every one but `_BIT`, as a pattern.
macro_rules! numeric {
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
            | Ty::F32
            | Ty::F64
            | Ty::F80
    };
}

/// The type value of a block (`qb64pe.bas` 14731, `Type2MemTypeValue`): a number's bytes plus 128 for an integer
/// (and 1024 when unsigned, 8192 for an `_OFFSET`) or 256 for a float; 512 for a fixed-length string, its length
/// not added; 4096 for a `_MEM`, 32768 for any other user type. [`ARRAY`] is added for an array.
fn type_value(p: &Program, t: Ty) -> u32 {
    let int = |bytes: u32, unsigned: bool| bytes + 128 + if unsigned { 1024 } else { 0 };
    match t {
        Ty::I8 => int(1, false),
        Ty::U8 => int(1, true),
        Ty::I16 => int(2, false),
        Ty::U16 => int(2, true),
        Ty::I32 => int(4, false),
        Ty::U32 => int(4, true),
        Ty::I64 => int(8, false),
        Ty::U64 => int(8, true),
        Ty::Off => int(8, false) + 8192,
        Ty::UOff => int(8, true) + 8192,
        Ty::F32 => 4 + 256,
        Ty::F64 => 8 + 256,
        Ty::F80 => 32 + 256,
        Ty::FixedStr(_) | Ty::Str => 512,
        Ty::User(_) if p.is_mem(t) => 4096,
        Ty::User(_) => USER,
        Ty::Bit { .. } => unreachable!("a block of a `_BIT` (`validate`)"),
    }
}

/// The C type a copy of `size` bytes goes through when the size is 1, 2, 4 or 8 written as a number (`int8` …
/// `int64`; a DOUBLE as `int64`, a `STRING * 4` as `int32`); `None` for any other size, copied by `memmove`
/// (`qb64pe.bas` 10222–10226).
fn copy_width(size: &str) -> Option<&'static str> {
    match size {
        "1" => Some("int8"),
        "2" => Some("int16"),
        "4" => Some("int32"),
        "8" => Some("int64"),
        _ => None,
    }
}

/// The checks of `_MEMGET`, `_MEMPUT` and `_MEMFILL` around the copy, with the offset in `tmp_long`, as the old
/// compiler writes them: a block never set raises error 309, a freed one (its lock's id changed) 308, a region
/// not within the block 300.
fn mem_checked(blk: &str, size: &str, copy: &str, out: &mut Vec<String>) {
    let b = format!("((mem_block*)({blk}))");
    out.push(format!("if ( {b}->lock_offset ){{"));
    out.push("if (".into());
    out.push(format!("tmp_long < {b}->offset  ||"));
    out.push(format!("(tmp_long+({size})) > ( {b}->offset + {b}->size)  ||"));
    out.push(format!("((mem_lock*){b}->lock_offset)->id != {b}->lock_id  ){{"));
    out.push(format!(
        "if (((mem_lock*){b}->lock_offset)->id != {b}->lock_id) error(308); else error(300);"
    ));
    out.push("}else{".into());
    out.push(copy.into());
    out.push("}".into());
    out.push("}else error(309);".into());
}

/// The part of a type value for a user type other than `_MEM`, and the part added for an array.
const USER: u32 = 32768;
const ARRAY: u32 = 65536;

/// The five arguments of `func__mem`.
struct Block {
    address: String,
    size: String,
    type_value: u32,
    element_size: u32,
    lock: String,
}

impl Emitter<'_> {
    /// `_MEM(reference)` (`element` false) or `_MEMELEMENT(reference)`: libqb's `func__mem` of the block.
    pub(crate) fn mem_of(&mut self, of: &Reference, element: bool) -> String {
        if let Some(block) = self.dyn_block(of) {
            return block;
        }
        let b = self.block_of(of, element);
        format!(
            "func__mem((ptrszint){},{},{},{},{})",
            b.address, b.size, b.type_value, b.element_size, b.lock
        )
    }

    /// `_OFFSET(reference)`: the address as the old compiler's "-6" helper writes it.
    pub(crate) fn address_of(&mut self, of: &Reference) -> String {
        format!("((uptrszint)({}))", self.address(of))
    }

    /// The address of what a reference names, as the old compiler's "-6" helper writes it: a variable's pointer
    /// (a string's characters), an element's, a member's, an array's first element.
    fn address(&mut self, of: &Reference) -> String {
        match of {
            Reference::Array(ArrayRef::Var(a)) => {
                let (v, n) = (self.p.var(*a), self.name(*a));
                match v.ty {
                    Ty::User(_) => format!("(((char*){n}[0])+(((0)*{}+0)))", self.ty_size(v.ty)),
                    Ty::FixedStr(k) => format!("(&((uint8*)({n}[0]))[(0)*{k}])"),
                    numeric!() => format!("(&({}))", self.element_at(*a, "0")),
                    Ty::Str | Ty::Bit { .. } => unreachable!("the address of an array of {:?} (`validate`)", v.ty),
                }
            }
            Reference::Array(ArrayRef::Member { base, member }) => {
                let (start, offset) = self.in_root(base);
                let mo = self.member_offset(self.p.place_ty(base), *member);
                format!("(({start})+(({offset}+{mo})))")
            }
            Reference::Place(Place::Var(v)) => {
                let (ty, n) = (self.p.var(*v).ty, self.name(*v));
                match ty {
                    Ty::FixedStr(_) | Ty::Str => format!("{n}->chr"),
                    Ty::User(_) => format!("(((char*){n})+((0+0)))"),
                    numeric!() => n,
                    Ty::Bit { .. } => unreachable!("the address of a `_BIT` (`validate`)"),
                }
            }
            Reference::Place(Place::Element {
                array: ArrayRef::Var(a),
                index,
            }) => {
                let flat = self.flat_index(*a, index);
                let (v, n) = (self.p.var(*a), self.name(*a));
                match v.ty {
                    Ty::User(_) => format!("(((char*){n}[0])+((({flat})*{}+0)))", self.ty_size(v.ty)),
                    Ty::FixedStr(_) => format!("{}->chr", self.element_at(*a, &flat)),
                    numeric!() => format!("(&({}))", self.element_at(*a, &flat)),
                    Ty::Str | Ty::Bit { .. } => unreachable!("the address of an element of {:?} (`validate`)", v.ty),
                }
            }
            Reference::Place(place @ (Place::Member { .. } | Place::Element { .. })) => {
                let (start, offset) = self.in_root(place);
                format!("(({start})+({offset}))")
            }
        }
    }

    /// The number of bytes of what a reference names, as the old compiler's "-5" helper writes it: a number's,
    /// a fixed-length string's or a user type's size, a variable-length string's present length, an array's
    /// bytes from its first element, a member array's bytes.
    fn bytes_count(&mut self, of: &Reference) -> String {
        match of {
            Reference::Array(ArrayRef::Var(a)) => {
                let size = self.ty_size(self.p.var(*a).ty);
                format!("({}-({size}*(0)))", self.all_bytes(*a))
            }
            Reference::Array(ArrayRef::Member { base, member }) => {
                let m = self.p.member_of(base, *member);
                (self.ty_size(m.ty) * m.count()).to_string()
            }
            Reference::Place(Place::Var(v)) if self.p.var(*v).ty == Ty::Str => format!("{}->len", self.name(*v)),
            Reference::Place(place) => self.ty_size(self.p.place_ty(place)).to_string(),
        }
    }

    /// `LEN(a())` (design D5 of `m4-names-arrays`): the old compiler's "-5" size of the whole array (`qb64pe.bas`
    /// 20971), a LONG.
    pub(crate) fn len_whole(&mut self, a: VarId) -> String {
        let bytes = self.bytes_count(&Reference::Array(ArrayRef::Var(a)));
        format!("((int32)({bytes}))")
    }

    /// The block of `_MEMGET`, `_MEMPUT`, `_MEMFILL` and the `_MEMGET` function: the address of its place, a
    /// variable spelled as the old compiler spells it (`(((char*)__UDT_M)+((0+0)))`).
    fn block_at(&mut self, block: &Place) -> String {
        match block {
            Place::Var(v) => format!("(((char*){})+((0+0)))", self.name(*v)),
            Place::Element { .. } | Place::Member { .. } => self.bytes_of(block),
        }
    }

    /// `_MEMGET(block, offset, type)`: the runtime's checked address of the bytes (a buffer of zero bytes after
    /// an error), read as the type (`qb64pe.bas` 20573–20630; measured, `verification\v32_x01_accepted`).
    pub(crate) fn mem_read(&mut self, block: &Place, offset: &Expr, of: Ty) -> String {
        let blk = self.block_at(block);
        let off = self.value(offset);
        let size = self.ty_size(of);
        let at = format!("func__memget((mem_block*){blk},{off},{size})");
        match of {
            Ty::FixedStr(n) => format!("qbs_new_txt_len((char*){at},{n})"),
            Ty::User(_) => format!("((void*)+{at})"),
            numeric!() => format!("*({}*)({at})", c_type(of)),
            Ty::Str | Ty::Bit { .. } => unreachable!("`_MEMGET` of a {of:?} (`validate`)"),
        }
    }

    /// `_MEMGET block, offset, variable`, written inline as the old compiler writes it (`qb64pe.bas` 10168–10260;
    /// measured, `verification\v32_x01_accepted`): the offset into `tmp_long`, the checks, and the copy.
    pub(crate) fn mem_get_stmt(&mut self, block: &Place, offset: &Expr, target: &Reference, out: &mut Vec<String>) {
        let blk = self.block_at(block);
        let off = self.value(offset);
        let (address, size) = (self.address(target), self.bytes_count(target));
        let copy = match copy_width(&size) {
            Some(t) => format!("*({t}*){address}=*({t}*)tmp_long;"),
            None => format!("memmove({address},(void*)tmp_long,{size});"),
        };
        out.push(format!("tmp_long={off};"));
        mem_checked(&blk, &size, &copy, out);
    }

    /// `_MEMPUT block, offset, value`, written inline (`qb64pe.bas` 10266–10398): a string that is no variable held
    /// in `g_tmp_str` first, then the offset into `tmp_long`, the checks, and the copy of the value's bytes; a
    /// value `AS type` stored through a pointer of that type.
    pub(crate) fn mem_put_stmt(&mut self, block: &Place, offset: &Expr, value: &MemValue, out: &mut Vec<String>) {
        let blk = self.block_at(block);
        let (size, copy) = match value {
            MemValue::Typed { value, ty } => {
                let v = self.stored(value);
                (
                    self.ty_size(*ty).to_string(),
                    format!("*({}*)tmp_long={v};", c_type(*ty)),
                )
            }
            MemValue::Bytes(of) => {
                let (address, size) = (self.address(of), self.bytes_count(of));
                let copy = match copy_width(&size) {
                    Some(t) => format!("*({t}*)tmp_long=*({t}*){address};"),
                    None => format!("memmove((void*)tmp_long,{address},{size});"),
                };
                (size, copy)
            }
            MemValue::Str(s) => {
                let s = self.value(s);
                out.push(format!("g_tmp_str={s};"));
                (
                    "g_tmp_str->len".to_string(),
                    "memmove((void*)tmp_long,g_tmp_str->chr,g_tmp_str->len);".to_string(),
                )
            }
        };
        let off = self.value(offset);
        out.push(format!("tmp_long={off};"));
        mem_checked(&blk, &size, &copy, out);
    }

    /// `_MEMFILL block, offset, bytes, value` (`qb64pe.bas` 10405–10500): a call of the runtime, by the value's
    /// type for a value `AS type` (`sub__memfill_2`), with the value's bytes otherwise (`sub__memfill`), a string
    /// that is no variable held in `tmp_long` first.
    pub(crate) fn mem_fill_stmt(
        &mut self,
        (block, offset, bytes): (&Place, &Expr, &Expr),
        value: &MemValue,
        out: &mut Vec<String>,
    ) {
        let blk = self.block_at(block);
        let (off, count) = (self.value(offset), self.value(bytes));
        match value {
            MemValue::Typed { value, ty } => {
                let v = self.stored(value);
                let entry = match ty {
                    Ty::Off | Ty::UOff => "OFFSET".to_string(),
                    Ty::F32 => "SINGLE".to_string(),
                    Ty::F64 => "DOUBLE".to_string(),
                    Ty::F80 => "FLOAT".to_string(),
                    Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 | Ty::I32 | Ty::U32 | Ty::I64 | Ty::U64 => {
                        self.ty_size(*ty).to_string()
                    }
                    Ty::Bit { .. } | Ty::Str | Ty::FixedStr(_) | Ty::User(_) => {
                        unreachable!("`_MEMFILL … AS` a {ty:?} (`validate`)")
                    }
                };
                out.push(format!("sub__memfill_{entry}((mem_block*){blk},{off},{count},{v});"));
            }
            MemValue::Bytes(of) => {
                let (address, size) = (self.address(of), self.bytes_count(of));
                out.push(format!(
                    "sub__memfill((mem_block*){blk},{off},{count},(ptrszint){address},{size});"
                ));
            }
            MemValue::Str(s) => {
                let s = self.value(s);
                out.push(format!("tmp_long=(ptrszint){s};"));
                out.push(format!(
                    "sub__memfill((mem_block*){blk},{off},{count},(ptrszint)((qbs*)tmp_long)->chr,((qbs*)tmp_long)->len);"
                ));
            }
        }
    }

    /// A value `AS type`, stored as the old compiler writes it: the conversions C++ makes by itself on the store
    /// (to a wider or narrower integer, from an integer to a float, between floats) are left to it; a float
    /// rounded to an integer keeps its `qbr`.
    fn stored(&mut self, v: &Expr) -> String {
        match &v.kind {
            ExprKind::Convert {
                how: Conv::Widen | Conv::Truncate | Conv::Nearest,
                from,
            } => self.stored(from),
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
            | ExprKind::Call { .. }
            | ExprKind::CallProc { .. }
            | ExprKind::MemOf { .. }
            | ExprKind::MemAt { .. }
            | ExprKind::AddressOf(_)
            | ExprKind::MemRead { .. }
            | ExprKind::ArrayBytes(_) => self.value(v),
        }
    }

    /// `_MEM` or `_MEMELEMENT` of a whole dynamic member array or of an element of one (measured,
    /// `verification\v35_x23`, `x24`, `x66`): the elements of its descriptor, typed as an array of them, or the
    /// element alone in its type, with the descriptor's lock; the same for both functions. The descriptor is found
    /// through its guard, the declared number of dimensions for the whole array, so one that is not allocated
    /// raises error 9 and gives a block that does not exist (the old program crashes there, `DIVERGENCES.md`
    /// D-021).
    fn dyn_block(&mut self, of: &Reference) -> Option<String> {
        let (base, member) = match of {
            Reference::Array(ArrayRef::Member { base, member })
            | Reference::Place(Place::Element {
                array: ArrayRef::Member { base, member },
                ..
            }) => (base, *member),
            Reference::Array(ArrayRef::Var(_)) | Reference::Place(_) => return None,
        };
        let (id, m) = self.dynamic_member(base, member)?;
        let decl = self.p.member_of(base, member);
        let (ety, dims) = (decl.ty, decl.bounds.len());
        let size = self.ty_size(ety);
        let tv = type_value(self.p, ety);
        Some(match of {
            Reference::Array(_) => {
                let slot = self.dyn_slot(base, member);
                // `qbr_dyn_unset` traces a member that is not allocated (D-021: the old program crashes).
                format!(
                    "qbr_dyn_mem_all({}(qbr_dyn_unset(*(ptrszint**){slot}),{dims}),{size},{})",
                    crate::owner::dyn_fn(id, m, "guard"),
                    tv + ARRAY
                )
            }
            Reference::Place(Place::Element { index, .. }) => {
                // The element's address leaves its descriptor in a `passN` (`Emitter::dyn_element_desc`); the
                // function reads it after the address is found.
                let (at, d) = self.dyn_element_desc(base, member, index);
                format!("qbr_dyn_mem_one({at},&{d},{size},{tv})")
            }
            Reference::Place(_) => unreachable!("matched above"),
        })
    }

    /// The block of a reference, by kind (design D3).
    fn block_of(&mut self, of: &Reference, element: bool) -> Block {
        let sf = "sf_mem_lock".to_string();
        match of {
            // A whole array variable (`_MEMELEMENT` of one is the same).
            Reference::Array(ArrayRef::Var(a)) => {
                let v = self.p.var(*a);
                let (ty, n, size) = (v.ty, self.name(*a), self.ty_size(v.ty));
                let all = self.all_bytes(*a);
                match ty {
                    // `_MEM` writes the offset of the first element as 0, `_MEMELEMENT` as an element's
                    // (`qb64pe.bas` 22885 against 22553; measured, `tests\callsite\memelement_array_user`).
                    Ty::User(_) => {
                        let first = if element {
                            format!("((0)*{size}+0)")
                        } else {
                            "0".to_string()
                        };
                        Block {
                            address: format!("(((char*){n}[0])+({first}))"),
                            size: format!("{all}-({first})"),
                            type_value: type_value(self.p, ty) + ARRAY,
                            element_size: size,
                            lock: sf,
                        }
                    }
                    Ty::FixedStr(k) => Block {
                        address: format!("(({})->chr)", fixed_at(&format!("&((uint8*)({n}[0]))[(0)*{k}]"), k)),
                        size: format!("{all}-({k}*(0))"),
                        type_value: 512 + ARRAY,
                        element_size: k,
                        lock: self.array_lock(*a),
                    },
                    numeric!() | Ty::Str | Ty::Bit { .. } => Block {
                        address: format!("(&({}))", self.element_at(*a, "0")),
                        size: format!("{all}-({size}*(0))"),
                        type_value: type_value(self.p, ty) + ARRAY,
                        element_size: size,
                        lock: self.array_lock(*a),
                    },
                }
            }
            // A whole member array: its own bytes, typed as an array of its elements.
            Reference::Array(ArrayRef::Member { base, member }) => {
                let (start, offset) = self.in_root(base);
                let m = self.p.member_of(base, *member);
                let (ty, count) = (m.ty, m.count());
                let mo = self.member_offset(self.p.place_ty(base), *member);
                let size = self.ty_size(ty);
                Block {
                    address: format!("(({start})+(({offset}+{mo})))"),
                    size: (size * count).to_string(),
                    type_value: type_value(self.p, ty) + ARRAY,
                    element_size: size,
                    lock: sf,
                }
            }
            Reference::Place(Place::Var(v)) => {
                let (ty, n) = (self.p.var(*v).ty, self.name(*v));
                match ty {
                    Ty::FixedStr(k) => Block {
                        address: format!("{n}->chr"),
                        size: k.to_string(),
                        type_value: 512,
                        element_size: k,
                        lock: sf,
                    },
                    Ty::User(_) => {
                        let size = self.ty_size(ty);
                        Block {
                            address: format!("(((char*){n})+((0+0)))"),
                            size: if element {
                                size.to_string()
                            } else {
                                format!("{size}-((0+0))")
                            },
                            type_value: if element { USER } else { type_value(self.p, ty) },
                            element_size: size,
                            lock: sf,
                        }
                    }
                    numeric!() | Ty::Str | Ty::Bit { .. } => {
                        let size = self.ty_size(ty);
                        Block {
                            address: n,
                            size: size.to_string(),
                            type_value: type_value(self.p, ty),
                            element_size: size,
                            lock: sf,
                        }
                    }
                }
            }
            // An element of an array variable: from it to the array's end, or (`_MEMELEMENT`) itself.
            Reference::Place(Place::Element {
                array: ArrayRef::Var(a),
                index,
            }) => {
                let flat = self.flat_index(*a, index);
                let v = self.p.var(*a);
                let (ty, n, size) = (v.ty, self.name(*a), self.ty_size(v.ty));
                let all = self.all_bytes(*a);
                match ty {
                    Ty::User(_) => Block {
                        address: format!("(((char*){n}[0])+((({flat})*{size}+0)))"),
                        size: if element {
                            size.to_string()
                        } else {
                            format!("{all}-((({flat})*{size}+0))")
                        },
                        type_value: if element { USER } else { type_value(self.p, ty) + ARRAY },
                        element_size: size,
                        lock: sf,
                    },
                    Ty::FixedStr(k) => {
                        let fixed = fixed_at(&format!("&((uint8*)({n}[0]))[({flat})*{k}]"), k);
                        Block {
                            address: format!("(({fixed})->chr)"),
                            size: if element {
                                k.to_string()
                            } else {
                                format!("{all}-({k}*({flat}))")
                            },
                            type_value: 512 + ARRAY,
                            element_size: k,
                            lock: self.array_lock(*a),
                        }
                    }
                    numeric!() | Ty::Str | Ty::Bit { .. } => Block {
                        address: format!("(&({}))", self.element_at(*a, &flat)),
                        size: if element {
                            size.to_string()
                        } else {
                            format!("{all}-({size}*({flat}))")
                        },
                        type_value: type_value(self.p, ty) + ARRAY,
                        element_size: size,
                        lock: self.array_lock(*a),
                    },
                }
            }
            Reference::Place(place @ (Place::Member { .. } | Place::Element { .. })) => {
                self.member_block(place, element)
            }
        }
    }

    /// The block of a place reached through a member: a member (of a variable or of an element) or an element of
    /// a member array. `_MEM` of it is, by the old compiler's user-type rule, the bytes from the place to the end
    /// of the variable (of the whole array for an element), typed 32768 (4096 for a `_MEM`), plus [`ARRAY`] when
    /// the variable is an array, with an element size of 1, or the size of the place's type when it is a user
    /// type; an element of a member array of an array element is the whole member array's bytes, typed 98304,
    /// with its elements' size. `_MEMELEMENT` of it is the member alone in its own type (a user type 32768), also
    /// of a member of an element; of an element of a member array of a variable the whole member array's bytes
    /// typed as one element, of an array's element the same block as `_MEM` (measured, `verification\v32_x119`).
    fn member_block(&mut self, place: &Place, element: bool) -> Block {
        let (start, offset) = self.in_root(place);
        let root = place.root();
        let root_array = self.p.var(root).is_array();
        let ty = self.p.place_ty(place);
        let address = format!("(({start})+({offset}))");
        let sf = "sf_mem_lock".to_string();
        if let Place::Element {
            array: ArrayRef::Member { base, member },
            ..
        } = place
        {
            let m = self.p.member_of(base, *member);
            let (ety, count) = (m.ty, m.count());
            let esize = self.ty_size(ety);
            let bytes = esize * count;
            if root_array {
                return Block {
                    address,
                    size: bytes.to_string(),
                    type_value: ARRAY + USER,
                    element_size: esize,
                    lock: sf,
                };
            }
            if element {
                return Block {
                    address,
                    size: bytes.to_string(),
                    type_value: type_value(self.p, ety),
                    element_size: bytes,
                    lock: sf,
                };
            }
        }
        let size = self.ty_size(ty);
        if element {
            return Block {
                address,
                size: size.to_string(),
                type_value: match ty {
                    Ty::User(_) => USER,
                    numeric!() | Ty::FixedStr(_) | Ty::Str | Ty::Bit { .. } => type_value(self.p, ty),
                },
                element_size: size,
                lock: sf,
            };
        }
        let whole = if root_array {
            self.all_bytes(root)
        } else {
            self.ty_size(self.p.var(root).ty).to_string()
        };
        let user = matches!(ty, Ty::User(_));
        Block {
            address,
            size: format!("{whole}-({offset})"),
            type_value: if user { type_value(self.p, ty) } else { USER } + if root_array { ARRAY } else { 0 },
            element_size: if user { size } else { 1 },
            lock: sf,
        }
    }

    /// Where the variable a place is part of starts, as a `char*` expression, and the place's byte offset from
    /// there (the variable's value; for an element of an array of a user type, the array's first element).
    fn in_root(&mut self, place: &Place) -> (String, String) {
        match place {
            Place::Var(v) => (format!("(char*){}", self.name(*v)), "0".to_string()),
            Place::Element {
                array: ArrayRef::Var(a),
                index,
            } => {
                let flat = self.flat_index(*a, index);
                let size = self.ty_size(self.p.var(*a).ty);
                (format!("(char*){}[0]", self.name(*a)), format!("(({flat})*{size})"))
            }
            Place::Member { base, member } => {
                let (start, offset) = self.in_root(base);
                let mo = self.member_offset(self.p.place_ty(base), *member);
                (start, format!("({offset}+{mo})"))
            }
            // An element of a dynamic member array lies in the block of its descriptor (design D9 of
            // `m3-typefields`): its address is the start.
            Place::Element {
                array: ArrayRef::Member { base, member },
                index,
            } if self.dynamic_member(base, *member).is_some() => {
                (self.dyn_element(base, *member, index), "0".to_string())
            }
            Place::Element {
                array: ArrayRef::Member { base, member },
                index,
            } => {
                let (start, offset) = self.in_root(base);
                let mo = self.member_offset(self.p.place_ty(base), *member);
                let flat = self.member_flat_index(base, *member, index);
                let size = self.ty_size(self.p.member_of(base, *member).ty);
                (start, format!("({offset}+{mo}+(({flat})*{size}))"))
            }
        }
    }

    /// The bytes of all elements of an array variable, as the old compiler's `variablesize$` writes them: none
    /// while it is not allocated.
    fn all_bytes(&self, a: VarId) -> String {
        let (v, n) = (self.p.var(a), self.name(a));
        let counts: Vec<String> = (1..=dim_count(v)).map(|i| format!("{n}[{}]", 4 * i + 1)).collect();
        format!("({}*({n}[2]&1)*{})", self.ty_size(v.ty), counts.join("*"))
    }

    /// The `_MEM` lock of an array variable: the slot after its dimensions.
    fn array_lock(&self, a: VarId) -> String {
        let slot = lock_slot(dim_count(self.p.var(a)));
        format!("(mem_lock*)((ptrszint*){})[{slot}]", self.name(a))
    }
}
