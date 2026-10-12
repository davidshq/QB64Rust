//! Arrays that a statement allocates, clears or releases when it runs (design D7 of `m2-defxxx-common-arrays`).
//!
//! The descriptor is libqb's (`study\02` §3.4; made in [`crate::decl`]). The allocation follows the old compiler's
//! sequence (`allocarray`, `qb64pe.bas` 15193–15531), whose order of checks and errors is measured (`study\00` §5):
//! the new bounds are built in a descriptor of their own, the new block is made, the old one is released after
//! it, and the array's descriptor is written last. The working variables are locals of the statement's block,
//! where the old compiler has `static` ones: a FUNCTION called in a bound may run the same statement again.

use crate::Emitter;
use crate::decl::{Part, dim_slot, lock_slot};
use crate::names::int_const;
use qb64rust_ir::{AllocMode, Expr, Facts, Ty, Var, VarId};

/// The lines of a text, each a line of a statement.
fn push_lines(out: &mut Vec<String>, text: &str) {
    out.extend(text.lines().map(str::to_string));
}

impl Emitter<'_> {
    /// `Allocate`: see the IR for what it does and in which order.
    pub(crate) fn allocate(&mut self, array: VarId, bounds: &[(Expr, Expr)], mode: AllocMode, out: &mut Vec<String>) {
        let v = self.p.var(array);
        let n = self.name(array);
        let texts: Vec<(String, String)> = bounds
            .iter()
            .map(|(lower, upper)| (self.value(lower), self.value(upper)))
            .collect();
        self.allocate_with(v, &n, &texts, mode, out);
        if bounds
            .iter()
            .any(|(lower, upper)| lower.uses_strings(self.p) || upper.uses_strings(self.p))
        {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// The allocation of an array that `STATIC a(bounds)` declared (`Shape::Dynamic`, `at_start`), for
    /// `maindata.txt`: mode `Dim` with its constant bounds, after the array's descriptor. That fragment runs when
    /// the program starts and again whenever an error handler is entered, where an array that is allocated is
    /// left alone (an error is pending, so no error 10) and one that an `ERASE` released is allocated again, as
    /// in the old compiler.
    pub(crate) fn allocate_at_start(&self, v: &Var, n: &str, bounds: &[(i64, i64)]) -> String {
        let texts: Vec<(String, String)> = bounds
            .iter()
            .map(|&(lower, upper)| (int_const(lower, Ty::I64), int_const(upper, Ty::I64)))
            .collect();
        let mut out = Vec::new();
        self.allocate_with(v, n, &texts, AllocMode::Dim, &mut out);
        out.iter().map(|line| format!("{line}\n")).collect()
    }

    /// [`Self::allocate`] with the bounds as C expressions, lower before upper, first dimension first.
    fn allocate_with(&self, v: &Var, n: &str, bounds: &[(String, String)], mode: AllocMode, out: &mut Vec<String>) {
        let dims = bounds.len();
        let string = v.ty == Ty::Str;
        let holds = self.holds_strings(v.ty);
        // The bytes of one element; a `_BIT * n` element gets whole bytes of its own here, more than the packed
        // elements need, as in the old compiler.
        let bits = if let Ty::Bit { width, .. } = v.ty {
            Some(width)
        } else {
            None
        };
        let size = match bits {
            Some(width) => format!("{width}/8+1"),
            None => self.ty_size(v.ty).to_string(),
        };
        let redim = match mode {
            AllocMode::Dim => false,
            AllocMode::Redim | AllocMode::Preserve => true,
        };
        // A static array: error 10, and nothing else.
        out.push(format!("if ({n}[2]&2){{"));
        out.push("error(10);".into());
        out.push("}else{".into());
        if redim {
            // The old block is kept until the new one exists; `_MEM` blocks over it become invalid.
            out.push(format!(
                "((mem_lock*)((ptrszint*){n})[{}])->id=(++mem_lock_id);",
                lock_slot(dims)
            ));
            out.push("ptrszint preserve_old_ptr=0;".into());
            out.push("uint64 preserve_old_total=0;".into());
            out.push("uint64 preserve_copy_count=0;".into());
            out.push("ptrszint preserve_new_ptr;".into());
            out.push(format!("if ({n}[2]&1){{"));
            out.push(format!("preserve_old_ptr={n}[0];"));
            out.push(format!("preserve_old_total={};", self.element_count(v, n)));
            out.push("}".into());
        } else {
            // `DIM` of an array that is allocated: error 10, and nothing else.
            out.push(format!("if ({n}[2]&1){{"));
            out.push("if (!error_occurred) error(10);".into());
            out.push("}else{".into());
        }
        out.push(format!("int64 alloc_new_desc[{}];", lock_slot(dims)));
        out.push("uint64 alloc_req_elems=1;".into());
        out.push("uint64 alloc_req_bytes;".into());
        out.push("uint64 alloc_dim_value;".into());
        out.push(format!("uint64 alloc_bytes_per_element=(uint64)({size});"));
        // The bounds, lower before upper, first dimension first. A dimension without an element raises error 5 and
        // the statement goes on, as it does after a bound that raises.
        for (k, (lower, upper)) in bounds.iter().enumerate() {
            let s = dim_slot(dims, k);
            out.push(format!("alloc_new_desc[{s}]={lower};"));
            out.push(format!("alloc_new_desc[{}]=({upper})-alloc_new_desc[{s}]+1;", s + 1));
            out.push(format!("if (alloc_new_desc[{}]<=0) error(5);", s + 1));
            if k == 0 {
                out.push(format!("alloc_new_desc[{}]=1;", s + 2));
            } else {
                let p = dim_slot(dims, k - 1);
                out.push(format!(
                    "alloc_new_desc[{}]=alloc_new_desc[{}]*alloc_new_desc[{}];",
                    s + 2,
                    p + 2,
                    p + 1
                ));
            }
        }
        // What the machine cannot hold is error 257, which ends the program: a negative number of elements, a
        // bound or a number that does not fit a pointer, a product or a size in bytes that overflows.
        for a in (1..=dims).map(|i| 4 * i) {
            let fits = |slot: usize| {
                format!("if (((int64)((ptrszint)alloc_new_desc[{slot}]))!=alloc_new_desc[{slot}]) error(257);")
            };
            out.push(fits(a));
            out.push(format!("if (alloc_new_desc[{}]<0) error(257);", a + 1));
            out.push(fits(a + 1));
            out.push(format!("if (alloc_new_desc[{}]<0) error(257);", a + 2));
            out.push(fits(a + 2));
            out.push(format!("alloc_dim_value=(uint64)alloc_new_desc[{}];", a + 1));
            out.push("if (alloc_dim_value){".into());
            out.push("if (alloc_req_elems>(18446744073709551615ull/alloc_dim_value)) error(257);".into());
            out.push("alloc_req_elems*=alloc_dim_value;".into());
            out.push("}".into());
        }
        out.push("if (alloc_req_elems>(18446744073709551615ull/alloc_bytes_per_element)) error(257);".into());
        out.push("alloc_req_bytes=alloc_req_elems*alloc_bytes_per_element;".into());
        out.push("if (((uint64)((ptrszint)alloc_req_elems))!=alloc_req_elems) error(257);".into());
        out.push("if (((uint64)((size_t)alloc_req_bytes))!=alloc_req_bytes) error(257);".into());
        if redim {
            out.push("preserve_copy_count=preserve_old_total;".into());
            out.push("if (alloc_req_elems<preserve_copy_count) preserve_copy_count=alloc_req_elems;".into());
            // An allocated array: the new block, the kept elements by flat position, then the old block goes.
            out.push("if (preserve_old_total){".into());
            let make = if string {
                "preserve_new_ptr=(ptrszint)malloc((size_t)alloc_req_bytes);"
            } else {
                "preserve_new_ptr=(ptrszint)calloc((size_t)alloc_req_bytes,1);"
            };
            out.push(make.into());
            out.push("if (!preserve_new_ptr) error(257);".into());
            let keep = mode == AllocMode::Preserve;
            if string {
                out.push("tmp_long=(ptrszint)alloc_req_elems;".into());
                out.push("while(tmp_long--) ((qbs**)(preserve_new_ptr))[tmp_long]=qbs_new(0,0);".into());
                if keep {
                    out.push("tmp_long=preserve_copy_count;".into());
                    out.push(
                        "while(tmp_long--) qbs_set(((qbs**)(preserve_new_ptr))[tmp_long],((qbs**)(preserve_old_ptr))[tmp_long]);"
                            .into(),
                    );
                }
                out.push("tmp_long=preserve_old_total;".into());
                out.push("while(tmp_long--) qbs_free(((qbs**)(preserve_old_ptr))[tmp_long]);".into());
            } else if let (true, Some(width)) = (keep, bits) {
                // Packed elements share bytes: the kept ones are copied one by one.
                out.push("tmp_long=(ptrszint)preserve_copy_count;".into());
                out.push(format!(
                    "while(tmp_long--) setbits({width},(uint8*)(preserve_new_ptr),tmp_long,(int64)getubits({width},(uint8*)(preserve_old_ptr),tmp_long));"
                ));
            } else if holds {
                // Elements of a user type with strings: every new element gets its own, the kept ones are
                // copied part by part, and the old elements' strings go with the old block.
                let (new, old) = ("preserve_new_ptr", "preserve_old_ptr");
                push_lines(
                    out,
                    &self.each_element(v.ty, new, "(ptrszint)alloc_req_elems", |at| self.new_strings(v.ty, at)),
                );
                if keep {
                    out.push("tmp_long=(ptrszint)preserve_copy_count;".into());
                    out.push("while(tmp_long--){".into());
                    for (offset, part) in self.parts(v.ty) {
                        let at = |block: &str| format!("((char*)({block})+tmp_long*{size}+{offset})");
                        out.push(match part {
                            Part::Str => format!("qbs_set(*(qbs**){},*(qbs**){});", at(new), at(old)),
                            Part::Bytes(n) => format!("memcpy({},{},{n});", at(new), at(old)),
                        });
                    }
                    out.push("}".into());
                }
                push_lines(
                    out,
                    &self.each_element(v.ty, old, "(ptrszint)preserve_old_total", |at| {
                        self.free_strings(v.ty, at)
                    }),
                );
            } else if keep {
                out.push(format!(
                    "if (preserve_copy_count) memcpy((void*)(preserve_new_ptr),(void*)(preserve_old_ptr),preserve_copy_count*{size});"
                ));
            }
            out.push("free((void*)(preserve_old_ptr));".into());
            out.push(format!("{n}[0]=preserve_new_ptr;"));
            out.push("}else{".into());
        }
        // An array that is not allocated: a new block, zero or of empty strings.
        if string {
            out.push(format!("{n}[0]=(ptrszint)malloc((size_t)alloc_req_bytes);"));
            out.push(format!("if (!{n}[0]) error(257);"));
            out.push("tmp_long=(ptrszint)alloc_req_elems;".into());
            out.push(format!("while(tmp_long--) ((qbs**)({n}[0]))[tmp_long]=qbs_new(0,0);"));
        } else {
            out.push(format!("{n}[0]=(ptrszint)calloc((size_t)alloc_req_bytes,1);"));
            out.push(format!("if (!{n}[0]) error(257);"));
            let block = format!("{n}[0]");
            push_lines(
                out,
                &self.each_element(v.ty, &block, "(ptrszint)alloc_req_elems", |at| {
                    self.new_strings(v.ty, at)
                }),
            );
        }
        if redim {
            out.push("}".into());
        }
        out.push(format!("{n}[2]|=1;"));
        for i in 4..lock_slot(dims) {
            out.push(format!("{n}[{i}]=alloc_new_desc[{i}];"));
        }
        if !redim {
            out.push("}".into());
        }
        out.push("}".into());
    }

    /// `Clear`: the elements of a static array zero, empty or NUL bytes.
    pub(crate) fn clear_array(&self, array: VarId, out: &mut Vec<String>) {
        let text = self.clear(self.p.var(array), &self.name(array));
        out.extend(text.lines().map(str::to_string));
    }

    /// `EraseParam`: the array passed is cleared when it is a static one and released when it is dynamic, by the
    /// flag of its descriptor, as the old compiler writes every `ERASE` (`qb64pe.bas` 8600-8704).
    pub(crate) fn erase_param(&self, array: VarId, out: &mut Vec<String>) {
        let (v, n) = (self.p.var(array), self.name(array));
        out.push(format!("if ({n}[2]&2){{"));
        out.extend(self.clear_elements(v, &n).lines().map(str::to_string));
        out.push("}else{".into());
        out.extend(self.release(v, &n).lines().map(str::to_string));
        out.push("}".into());
    }

    /// `Release`: a dynamic array not allocated again.
    pub(crate) fn release_array(&self, array: VarId, out: &mut Vec<String>) {
        let text = self.release(self.p.var(array), &self.name(array));
        out.extend(text.lines().map(str::to_string));
    }
}
