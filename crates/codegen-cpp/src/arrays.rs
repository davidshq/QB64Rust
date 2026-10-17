//! Arrays that a statement allocates, clears or releases when it runs (design D7 of `m2-defxxx-common-arrays`).
//!
//! The descriptor is libqb's (`study\02` §3.4; made in [`crate::decl`]). The allocation follows the old compiler's
//! sequence (`allocarray`, `qb64pe.bas` 15193–15531), whose order of checks and errors is measured (`study\00` §5):
//! the new bounds are built in a descriptor of their own, the new block is made, the old one is released after
//! it, and the array's descriptor is written last. One difference: a `REDIM` takes the old block after the bounds,
//! not before them (`DIVERGENCES.md` D-018). The working variables are locals of the statement's block,
//! where the old compiler has `static` ones: a FUNCTION called in a bound may run the same statement again.

use crate::Emitter;
use crate::decl::{Part, count_of, dim_slot, lock_slot};
use crate::names::{bare, int_const};
use crate::owner::dyn_fn;
use qb64rust_ir::{AllocMode, ArrayRef, Conv, Expr, ExprKind, Facts, Ty, Var, VarId};

/// The name of the descriptor of a dynamic member array that [`Emitter::allocate`] allocates.
const DYN_DESC: &str = "dyn_d";

/// The lines of a text, each a line of a statement.
fn push_lines(out: &mut Vec<String>, text: &str) {
    out.extend(text.lines().map(str::to_string));
}

/// The copy of `REDIM _RETAIN` (design D3 of `m3-redim-retain`), the old compiler's `AppendRetainCoordinateWalk`
/// (`qb64pe.bas` 14782–14812): an odometer over the indexes of the overlap, the first dimension fastest, with the
/// offsets of the element in the old block (`preserve_old_off`) and the new one (`preserve_new_off`); `payload`
/// copies one element.
fn retain_walk(out: &mut Vec<String>, dims: usize, payload: &[String]) {
    out.push("if (preserve_any){".into());
    for w in 0..dims {
        out.push(format!("preserve_idx[{w}]=preserve_lo[{w}];"));
    }
    out.push("for(;;){".into());
    let s = dim_slot(dims, 0);
    out.push(format!("preserve_old_off=preserve_idx[0]-preserve_old_desc[{s}];"));
    out.push(format!("preserve_new_off=preserve_idx[0]-alloc_new_desc[{s}];"));
    for w in 1..dims {
        let s = dim_slot(dims, w);
        out.push(format!(
            "preserve_old_off+=(preserve_idx[{w}]-preserve_old_desc[{s}])*preserve_old_desc[{}];",
            s + 2
        ));
        out.push(format!(
            "preserve_new_off+=(preserve_idx[{w}]-alloc_new_desc[{s}])*alloc_new_desc[{}];",
            s + 2
        ));
    }
    out.extend(payload.iter().cloned());
    out.push("preserve_dim=0;".into());
    out.push(format!("while (preserve_dim<{dims}){{"));
    out.push("preserve_idx[preserve_dim]++;".into());
    out.push("if (preserve_idx[preserve_dim]<=preserve_hi[preserve_dim]) break;".into());
    out.push("preserve_idx[preserve_dim]=preserve_lo[preserve_dim];".into());
    out.push("preserve_dim++;".into());
    out.push("}".into());
    out.push(format!("if (preserve_dim=={dims}) break;"));
    out.push("}".into());
    out.push("}".into());
}

impl Emitter<'_> {
    /// `Allocate`: see the IR for what it does and in which order.
    pub(crate) fn allocate(
        &mut self,
        array: &ArrayRef,
        bounds: &[(Expr, Expr)],
        mode: AllocMode,
        out: &mut Vec<String>,
    ) {
        let mut cleanup = bounds
            .iter()
            .any(|(lower, upper)| lower.uses_strings(self.p) || upper.uses_strings(self.p));
        match array {
            ArrayRef::Var(array) => {
                let v = self.p.var(*array);
                let n = self.name(*array);
                let texts = self.bound_texts(bounds);
                self.allocate_with(v.ty, &n, &texts, mode, out);
            }
            ArrayRef::Member { base, member } => {
                // The place first, once; a descriptor with another number of dimensions goes, and a new one is
                // made when there is none (design D4 of `m3-typefields`).
                let (id, m) = self
                    .dynamic_member(base, *member)
                    .expect("an Allocate of a dynamic member array");
                let ty = self.p.member_of(base, *member).ty;
                let dims = bounds.len();
                out.push("{".into());
                let slot = self.dyn_slot(base, *member);
                out.push(format!("char *dyn_slot={slot};"));
                out.push("ptrszint *dyn_d=*(ptrszint**)dyn_slot;".into());
                out.push(format!(
                    "if (dyn_d&&dyn_d[3]!={dims}){{{}(dyn_d);dyn_d=NULL;}}",
                    dyn_fn(id, m, "free")
                ));
                out.push(format!(
                    "if (!dyn_d){{dyn_d={}({dims});*(ptrszint**)dyn_slot=dyn_d;}}",
                    dyn_fn(id, m, "blank")
                ));
                let texts = self.bound_texts(bounds);
                self.allocate_with(ty, DYN_DESC, &texts, mode, out);
                out.push("}".into());
                cleanup |= base.uses_strings(self.p);
            }
        }
        if cleanup {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// The bounds of an `Allocate` as C expressions, lower before upper, first dimension first.
    fn bound_texts(&mut self, bounds: &[(Expr, Expr)]) -> Vec<(String, String)> {
        bounds
            .iter()
            .map(|(lower, upper)| (self.value(lower), self.value(upper)))
            .collect()
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
        self.allocate_with(v.ty, n, &texts, AllocMode::Dim, &mut out);
        out.iter().map(|line| format!("{line}\n")).collect()
    }

    /// [`Self::allocate`] of the array whose descriptor is `n`, of elements of type `t`, with the bounds as C
    /// expressions, lower before upper, first dimension first.
    pub(crate) fn allocate_with(
        &self,
        t: Ty,
        n: &str,
        bounds: &[(String, String)],
        mode: AllocMode,
        out: &mut Vec<String>,
    ) {
        let dims = bounds.len();
        let string = t == Ty::Str;
        let holds = self.has_life(t);
        // The bytes of one element; a `_BIT * n` element gets whole bytes of its own here, more than the packed
        // elements need, as in the old compiler.
        let bits = if let Ty::Bit { width, .. } = t {
            Some(width)
        } else {
            None
        };
        let size = match bits {
            Some(width) => format!("{width}/8+1"),
            None => self.ty_size(t).to_string(),
        };
        let redim = mode != AllocMode::Dim;
        let retain = mode == AllocMode::Retain;
        // A static array: error 10, and nothing else.
        out.push(format!("if ({n}[2]&2){{"));
        out.push("error(10);".into());
        out.push("}else{".into());
        if redim {
            out.push("ptrszint preserve_old_ptr=0;".into());
            out.push("uint64 preserve_old_total=0;".into());
            out.push("uint64 preserve_copy_count=0;".into());
            out.push("ptrszint preserve_new_ptr;".into());
            if retain {
                out.push("int32 preserve_any=0;".into());
                out.push(format!("int64 preserve_old_desc[{}];", lock_slot(dims)));
                out.push(format!("int64 preserve_lo[{dims}];"));
                out.push(format!("int64 preserve_hi[{dims}];"));
                out.push(format!("int64 preserve_idx[{dims}];"));
                out.push("int64 preserve_old_off;".into());
                out.push("int64 preserve_new_off;".into());
                out.push("ptrszint preserve_dim;".into());
            }
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
        if redim {
            // A FUNCTION in a bound that gives the array another block or erases it (D-018).
            out.push(format!("ptrszint alloc_old_block={n}[0];"));
            out.push(format!("ptrszint alloc_old_set={n}[2]&1;"));
        }
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
        if redim {
            // The old block (and with `_RETAIN` its bounds) is taken after the bounds, where the old compiler takes
            // it before them: a FUNCTION in a bound may give the array new bounds or erase it, and the old
            // compiler then frees a block twice (`DIVERGENCES.md` D-018). It is kept until the new one exists;
            // `_MEM` blocks over it become invalid.
            out.push(format!(
                "if ({n}[0]!=alloc_old_block||({n}[2]&1)!=alloc_old_set) qbr_diverged(18);"
            ));
            out.push(format!(
                "((mem_lock*)((ptrszint*){n})[{}])->id=(++mem_lock_id);",
                lock_slot(dims)
            ));
            out.push(format!("if ({n}[2]&1){{"));
            out.push(format!("preserve_old_ptr={n}[0];"));
            out.push(format!("preserve_old_total={};", count_of(dims, n)));
            if retain {
                for i in 4..lock_slot(dims) {
                    out.push(format!("preserve_old_desc[{i}]={n}[{i}];"));
                }
            }
            out.push("}".into());
        }
        // What the machine cannot hold is error 257, which ends the program: a negative number of elements, a
        // bound or a number that does not fit a pointer, a product or a size in bytes that overflows.
        let checks = out.len();
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
        if n == DYN_DESC {
            // A dynamic member array: the old compiler checks none of these (D-020).
            for line in &mut out[checks..] {
                *line = line.replace("error(257);", "{qbr_diverged(20);error(257);}");
            }
        }
        if redim {
            out.push("preserve_copy_count=preserve_old_total;".into());
            out.push("if (alloc_req_elems<preserve_copy_count) preserve_copy_count=alloc_req_elems;".into());
            if retain {
                // Per dimension, the overlap of the old and the new range (walk index 0: the first dimension).
                out.push("preserve_any=(preserve_old_total!=0);".into());
                for w in 0..dims {
                    let s = dim_slot(dims, w);
                    out.push(format!("preserve_lo[{w}]=alloc_new_desc[{s}];"));
                    out.push(format!(
                        "if (preserve_old_desc[{s}]>preserve_lo[{w}]) preserve_lo[{w}]=preserve_old_desc[{s}];"
                    ));
                    out.push(format!(
                        "preserve_hi[{w}]=alloc_new_desc[{s}]+(alloc_new_desc[{}]-1);",
                        s + 1
                    ));
                    out.push(format!(
                        "if ((preserve_old_desc[{s}]+(preserve_old_desc[{}]-1))<preserve_hi[{w}]) preserve_hi[{w}]=preserve_old_desc[{s}]+(preserve_old_desc[{}]-1);",
                        s + 1,
                        s + 1
                    ));
                    out.push(format!("if (preserve_hi[{w}]<preserve_lo[{w}]) preserve_any=0;"));
                }
            }
            // An allocated array: the new block, the kept elements (by flat position, or by their indexes with
            // `_RETAIN`), then the old block goes.
            out.push("if (preserve_old_total){".into());
            if string {
                out.push("preserve_new_ptr=(ptrszint)malloc((size_t)alloc_req_bytes);".into());
                out.push("if (!preserve_new_ptr) error(257);".into());
            } else {
                out.extend(self.new_block(t, n, "preserve_new_ptr"));
            }
            let keep = mode == AllocMode::Preserve;
            if string {
                out.push("tmp_long=(ptrszint)alloc_req_elems;".into());
                out.push("while(tmp_long--) ((qbs**)(preserve_new_ptr))[tmp_long]=qbs_new(0,0);".into());
                if retain {
                    retain_walk(
                        out,
                        dims,
                        &["qbs_set(((qbs**)(preserve_new_ptr))[preserve_new_off],((qbs**)(preserve_old_ptr))[preserve_old_off]);".into()],
                    );
                } else if keep {
                    out.push("tmp_long=preserve_copy_count;".into());
                    out.push(
                        "while(tmp_long--) qbs_set(((qbs**)(preserve_new_ptr))[tmp_long],((qbs**)(preserve_old_ptr))[tmp_long]);"
                            .into(),
                    );
                }
                out.push("tmp_long=preserve_old_total;".into());
                out.push("while(tmp_long--) qbs_free(((qbs**)(preserve_old_ptr))[tmp_long]);".into());
            } else if let (true, Some(width)) = (retain, bits) {
                // The offsets of a `_BIT` element count elements, as in the old compiler.
                retain_walk(
                    out,
                    dims,
                    &[format!(
                        "setbits({width},(uint8*)(preserve_new_ptr),preserve_new_off,(int64)getubits({width},(uint8*)(preserve_old_ptr),preserve_old_off));"
                    )],
                );
            } else if let (true, Some(width)) = (keep, bits) {
                // Packed elements share bytes: the kept ones are copied one by one.
                out.push("tmp_long=(ptrszint)preserve_copy_count;".into());
                out.push(format!(
                    "while(tmp_long--) setbits({width},(uint8*)(preserve_new_ptr),tmp_long,(int64)getubits({width},(uint8*)(preserve_old_ptr),tmp_long));"
                ));
            } else if holds {
                // Elements of a user type with strings or dynamic member arrays: every new element gets its own,
                // the kept ones are copied part by part (an owner value deep, as the old compiler's
                // `AppendDynUDTDescCopy` copies it), and what the old elements hold goes with the old block.
                let (new, old) = ("preserve_new_ptr", "preserve_old_ptr");
                push_lines(
                    out,
                    &self.each_element(t, new, "(ptrszint)alloc_req_elems", |at| self.new_value(t, at)),
                );
                let copy = |new_at: &str, old_at: &str| -> Vec<String> {
                    let at = |block: &str, i: &str, offset| format!("((char*)({block})+{i}*{size}+{offset})");
                    if self.owns_desc(t) {
                        let text = self.clone_value(t, &at(new, new_at, 0), &at(old, old_at, 0));
                        return text.lines().map(str::to_string).collect();
                    }
                    self.parts(t)
                        .into_iter()
                        .map(|(offset, part)| {
                            let (to, from) = (at(new, new_at, offset), at(old, old_at, offset));
                            match part {
                                Part::Str => format!("qbs_set(*(qbs**){to},*(qbs**){from});"),
                                Part::Bytes(n) => format!("memcpy({to},{from},{n});"),
                                Part::Dyn { .. } => unreachable!("a type that owns no dynamic member array"),
                            }
                        })
                        .collect()
                };
                if retain {
                    retain_walk(out, dims, &copy("preserve_new_off", "preserve_old_off"));
                } else if keep {
                    out.push("tmp_long=(ptrszint)preserve_copy_count;".into());
                    out.push("while(tmp_long--){".into());
                    out.extend(copy("tmp_long", "tmp_long"));
                    out.push("}".into());
                }
                push_lines(
                    out,
                    &self.each_element(t, old, "(ptrszint)preserve_old_total", |at| self.free_value(t, at)),
                );
            } else if retain {
                retain_walk(
                    out,
                    dims,
                    &[format!(
                        "memcpy(((uint8*)(preserve_new_ptr))+preserve_new_off*{size},((uint8*)(preserve_old_ptr))+preserve_old_off*{size},{size});"
                    )],
                );
            } else if keep {
                out.push(format!(
                    "if (preserve_copy_count) memcpy((void*)(preserve_new_ptr),(void*)(preserve_old_ptr),preserve_copy_count*{size});"
                ));
            }
            out.push(self.free_block(t, n, "preserve_old_ptr"));
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
            out.extend(self.new_block(t, n, &format!("{n}[0]")));
            let block = format!("{n}[0]");
            push_lines(
                out,
                &self.each_element(t, &block, "(ptrszint)alloc_req_elems", |at| self.new_value(t, at)),
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

    /// `CopyArray` (design D4 of `m3-whole-array-assign`): the old compiler's `AppendWholeArrayAssign`
    /// (`qb64pe.bas` 17272) with its names. The descriptors are taken target first, each finding its place once
    /// (a member array's as for an array argument, [`Self::member_array_arg`], or a dynamic member's own); when a
    /// place may raise, nothing is copied after an error (`DIVERGENCES.md` D-004). Both must be allocated and have
    /// the same bounds, else error 9; then the elements by kind: one move of the bytes where they hold nothing
    /// else, a string or a user type that holds strings element by element, an owner value deep in place
    /// ([`Self::copy_owner`]; the old compiler gives a `REDIM` target a new block, with the same values). A
    /// dynamic member's descriptor must also have the number of dimensions of the copy, which it can lose to a
    /// `REDIM` (the old compiler's guard then raises error 9 and gives a stand-in descriptor, so nothing is copied:
    /// `verification\v38_c`). Two array variables of an owner type with other bounds: [`Self::copy_owner_array`].
    pub(crate) fn copy_array(&mut self, from: &ArrayRef, to: &ArrayRef, out: &mut Vec<String>) {
        let t = self.p.array_ty(to);
        let dims = self.copy_dims(to).or_else(|| self.copy_dims(from)).unwrap_or(1);
        let (dst, src) = (self.whole_desc(to), self.whole_desc(from));
        if self.owns_desc(t) && matches!((to, from), (ArrayRef::Var(_), ArrayRef::Var(_))) {
            self.copy_owner_array(t, dims, (&dst, &src), out);
            return;
        }
        let guarded = to.may_raise() || from.may_raise();
        out.push("{".into());
        out.push(format!("ptrszint *whole_arr_dst={dst};"));
        out.push(format!("ptrszint *whole_arr_src={src};"));
        out.push(if guarded {
            "int32 whole_arr_ok=!is_error_pending();".into()
        } else {
            "int32 whole_arr_ok=1;".into()
        });
        // An index on the way raised: the old compiler copies strings and user types element by element through the
        // element at position 0 (D-004); its move of bytes checks for the error, as here.
        if guarded && matches!(t, Ty::Str | Ty::User(_)) {
            out.push("if (!whole_arr_ok) qbr_diverged_on(4);".into());
        }
        out.push("uint64 whole_arr_total=1;".into());
        let mut missing = "!(whole_arr_dst[2]&1)||!(whole_arr_src[2]&1)||(!whole_arr_dst[0])||(!whole_arr_src[0])||(whole_arr_dst[0]==(ptrszint)nothingvalue)||(whole_arr_src[0]==(ptrszint)nothingvalue)".to_string();
        for (side, array) in [("dst", to), ("src", from)] {
            if self.is_dynamic_member(array) {
                missing.push_str(&format!("||(whole_arr_{side}[3]!={dims})"));
            }
        }
        let test = if guarded {
            format!("whole_arr_ok&&({missing})")
        } else {
            missing
        };
        out.push(format!("if ({test}){{error(9); whole_arr_ok=0;}}"));
        out.push("if (whole_arr_ok){".into());
        out.push(format!(
            "for(ptrszint whole_arr_dim=1; whole_arr_dim<={dims}; whole_arr_dim++){{"
        ));
        out.push(format!("ptrszint whole_arr_arg=({dims}-whole_arr_dim)*4+4;"));
        out.push("if ((whole_arr_dst[whole_arr_arg]!=whole_arr_src[whole_arr_arg])||(whole_arr_dst[whole_arr_arg+1]!=whole_arr_src[whole_arr_arg+1])){error(9); whole_arr_ok=0; break;}".into());
        out.push("if (whole_arr_dst[whole_arr_arg+1]<0){error(9); whole_arr_ok=0; break;}".into());
        out.push("if (whole_arr_dst[whole_arr_arg+1]&&whole_arr_total>(18446744073709551615ull/(uint64)whole_arr_dst[whole_arr_arg+1])){error(257); whole_arr_ok=0; break;}".into());
        out.push("whole_arr_total*=(uint64)whole_arr_dst[whole_arr_arg+1];".into());
        out.push("}".into());
        out.push("}".into());
        out.push("if (whole_arr_ok&&(whole_arr_dst[0]!=whole_arr_src[0])){".into());
        let size = self.ty_size(t);
        let at = |side: &str| format!("((char*)whole_arr_{side}[0]+whole_arr_i*{size})");
        let each = "for(ptrszint whole_arr_i=0; whole_arr_i<(ptrszint)whole_arr_total; whole_arr_i++){";
        if t == Ty::Str {
            out.push(each.into());
            out.push("qbs_set(((qbs**)whole_arr_dst[0])[whole_arr_i],((qbs**)whole_arr_src[0])[whole_arr_i]);".into());
            out.push("}".into());
        } else if self.owns_desc(t) {
            out.push(each.into());
            self.copy_owner(t, &at("dst"), &at("src"), out);
            out.push("}".into());
        } else if self.holds_strings(t) {
            out.push(each.into());
            push_lines(out, &self.clone_value(t, &at("dst"), &at("src")));
            out.push("}".into());
        } else {
            // Bytes: numbers, `_OFFSET`, `STRING * n`, `_BIT` (an element has `width/8+1` bytes of its own, as the
            // allocation gives it), `_MEM` and the user types that hold no string.
            let element = match t {
                Ty::Bit { width, .. } => format!("({width}/8+1)"),
                Ty::FixedStr(_) => size.to_string(),
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
                | Ty::Str
                | Ty::User(_) => format!("({size})"),
            };
            out.push(format!(
                "if (whole_arr_total>(18446744073709551615ull/(uint64){element})) error(257);"
            ));
            out.push(format!("if (!is_error_pending()) memmove((void*)whole_arr_dst[0],(void*)whole_arr_src[0],(size_t)(whole_arr_total*(uint64){element}));"));
        }
        out.push("}".into());
        out.push("}".into());
        let strings = |a: &ArrayRef| matches!(a, ArrayRef::Member { base, .. } if base.uses_strings(self.p));
        if strings(to) || strings(from) {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// `CopyArray` of two array variables of an owner type, as the old compiler's `AppendDynUDTArrayAssign`
    /// (`qb64pe.bas` 16855) checks them: both must be allocated, else error 9 and nothing is copied; other bounds
    /// raise error 9 and the copy goes on, element `i` of the target from element `i` of the source for every
    /// element of the target, deep, the target keeping its bounds. Only when the source has fewer elements, which
    /// the old program reads past, is nothing copied (`DIVERGENCES.md` D-022; design D3 of `m3-measured-copies`).
    fn copy_owner_array(&mut self, t: Ty, dims: usize, (dst, src): (&str, &str), out: &mut Vec<String>) {
        out.push("{".into());
        out.push(format!("ptrszint *whole_arr_dst={dst};"));
        out.push(format!("ptrszint *whole_arr_src={src};"));
        out.push("int32 whole_arr_ok=1, whole_arr_same=1;".into());
        out.push("uint64 whole_arr_total=1, whole_arr_have=1;".into());
        // A target with elements from a source that is not allocated: the old compiler reads through its missing
        // block (D-022).
        out.push("if (!(whole_arr_dst[2]&1)||!(whole_arr_src[2]&1)||(!whole_arr_dst[0])||(!whole_arr_src[0])||(whole_arr_dst[0]==(ptrszint)nothingvalue)||(whole_arr_src[0]==(ptrszint)nothingvalue)){error(9); whole_arr_ok=0; if ((whole_arr_dst[2]&1)&&!(whole_arr_src[2]&1)) qbr_diverged(22);}".into());
        out.push("if (whole_arr_ok){".into());
        out.push(format!(
            "for(ptrszint whole_arr_dim=1; whole_arr_dim<={dims}; whole_arr_dim++){{"
        ));
        out.push(format!("ptrszint whole_arr_arg=({dims}-whole_arr_dim)*4+4;"));
        out.push("if ((whole_arr_dst[whole_arr_arg]!=whole_arr_src[whole_arr_arg])||(whole_arr_dst[whole_arr_arg+1]!=whole_arr_src[whole_arr_arg+1])) whole_arr_same=0;".into());
        out.push("if ((whole_arr_dst[whole_arr_arg+1]<0)||(whole_arr_src[whole_arr_arg+1]<0)){error(9); whole_arr_ok=0; break;}".into());
        out.push("if ((whole_arr_dst[whole_arr_arg+1]&&whole_arr_total>(18446744073709551615ull/(uint64)whole_arr_dst[whole_arr_arg+1]))||(whole_arr_src[whole_arr_arg+1]&&whole_arr_have>(18446744073709551615ull/(uint64)whole_arr_src[whole_arr_arg+1]))){error(257); whole_arr_ok=0; break;}".into());
        out.push("whole_arr_total*=(uint64)whole_arr_dst[whole_arr_arg+1];".into());
        out.push("whole_arr_have*=(uint64)whole_arr_src[whole_arr_arg+1];".into());
        out.push("}".into());
        out.push("if (whole_arr_ok&&!whole_arr_same) error(9);".into());
        out.push("if (whole_arr_ok&&whole_arr_total>whole_arr_have){whole_arr_ok=0; qbr_diverged(22);}".into());
        out.push("}".into());
        out.push("if (whole_arr_ok&&(whole_arr_dst[0]!=whole_arr_src[0])){".into());
        let size = self.ty_size(t);
        let at = |side: &str| format!("((char*)whole_arr_{side}[0]+whole_arr_i*{size})");
        out.push("for(ptrszint whole_arr_i=0; whole_arr_i<(ptrszint)whole_arr_total; whole_arr_i++){".into());
        self.copy_owner(t, &at("dst"), &at("src"), out);
        out.push("}".into());
        out.push("}".into());
        out.push("}".into());
    }

    /// `ArrayCopy` (design D5 of `m3-arraycopy`): the old compiler's block (`source\utilities\arrcpy.bm`
    /// `EmitArrayCopy1D` … `EmitArrayCopyRangeWholeND`) with its names, calling libqb's `qb64_array_copy_*`, which
    /// checks the bounds and copies. The entry point goes by the ranges and the source's number of dimensions: a
    /// range of one dimension `1d_range_at` (to the whole destination with its lower bound as the start), several
    /// `nd_range_at` (likewise), a whole source `1d_whole`, `1d_whole_at`, `nd_whole` or `nd_whole_at`; `_string`
    /// for `STRING`, else `_fixed` with the size of an element. A member side's descriptor is taken first, where
    /// its place is found (source, then destination); when a place may raise, nothing is copied after an error
    /// there (`DIVERGENCES.md` D-004); the guard is taken before the bounds, so an error a bound raises does not
    /// stop the copy, as in the old compiler. A dynamic member side whose number of dimensions a `REDIM` changed
    /// raises error 9 there and is not copied. A `_FLOAT` element is `sizeof(long double)` bytes from the next
    /// (`DIVERGENCES.md` D-025: the old compiler passes 32, its `LEN`).
    pub(crate) fn array_copy(
        &mut self,
        (from, to): (&ArrayRef, &ArrayRef),
        ranges: &[(Expr, Expr)],
        starts: &[Expr],
        out: &mut Vec<String>,
    ) {
        let t = self.p.array_ty(from);
        let dims = self.copy_dims(from).unwrap_or(1);
        out.push("{".into());
        let mut sides = Vec::new();
        for (side, array) in [("src", from), ("dst", to)] {
            let desc = self.whole_desc(array);
            let line = format!("ptrszint *array_copy_{side}={desc};");
            if matches!(array, ArrayRef::Member { .. }) {
                out.push(line);
            } else {
                sides.push(line);
            }
        }
        let guarded = from.may_raise() || to.may_raise();
        if guarded {
            out.push("int32 array_copy_ok=!is_error_pending();".into());
        }
        // A dynamic member that a `REDIM` gave another number of dimensions: error 9 where its place is found, and
        // nothing is copied, as the old compiler's stand-in descriptor does (`verification\v41_l`; libqb reads the
        // bounds by the number it is given and never checks the descriptor's).
        let reshaped: Vec<String> = [("src", from), ("dst", to)]
            .into_iter()
            .filter(|(_, array)| self.is_dynamic_member(array))
            .map(|(side, array)| format!("array_copy_{side}[3]=={}", self.copy_dims(array).unwrap_or(1)))
            .collect();
        let dims_checked = !reshaped.is_empty();
        if dims_checked {
            out.push(format!("int32 array_copy_dims={};", reshaped.join("&&")));
            out.push("if (!array_copy_dims) error(9);".into());
        }
        // A narrower integer, and a constant that a `LONG` holds, go into the statement's own `(int64)(…)` as they
        // are, as the old compiler writes them.
        let int64 = |c: &mut Self, v: &Expr| {
            if let ExprKind::Convert { how: Conv::Widen, from } = &v.kind {
                format!("(int64)({})", c.inner(from))
            } else if let ExprKind::Int(i) = v.kind
                && i32::try_from(i).is_ok()
            {
                format!("(int64)({})", bare(&int_const(i, Ty::I32)))
            } else {
                format!("(int64)({})", c.inner(v))
            }
        };
        let list = |c: &mut Self, vs: &mut dyn Iterator<Item = &Expr>| -> String {
            vs.map(|v| int64(c, v)).collect::<Vec<_>>().join(",")
        };
        match (ranges, starts) {
            ([(lower, upper)], _) => {
                out.push(format!("int64 array_copy_src_low={};", int64(self, lower)));
                out.push(format!("int64 array_copy_src_high={};", int64(self, upper)));
            }
            ([], _) => {}
            _ => {
                let n = ranges.len();
                let lows = list(self, &mut ranges.iter().map(|(lower, _)| lower));
                let highs = list(self, &mut ranges.iter().map(|(_, upper)| upper));
                out.push(format!("int64 array_copy_src_low[{n}]={{{lows}}};"));
                out.push(format!("int64 array_copy_src_high[{n}]={{{highs}}};"));
            }
        }
        match (ranges.len(), starts) {
            (_, []) => {}
            (0 | 1, [start]) if dims == 1 || ranges.len() == 1 => {
                out.push(format!("int64 array_copy_dst_start={};", int64(self, start)));
            }
            _ => {
                let starts_text = list(self, &mut starts.iter());
                out.push(format!(
                    "int64 array_copy_dst_start[{}]={{{starts_text}}};",
                    starts.len()
                ));
            }
        }
        out.extend(sides);
        let (kind, size) = if t == Ty::Str {
            ("string", None)
        } else if t == Ty::F80 {
            ("fixed", Some("(uint64)sizeof(long double)".to_string()))
        } else {
            ("fixed", Some(format!("(uint64){}", self.ty_size(t))))
        };
        let (entry, args) = match (ranges.len(), starts.is_empty()) {
            (1, false) => (
                "1d_range_at",
                "array_copy_src_low,array_copy_src_high,array_copy_dst_start".to_string(),
            ),
            (1, true) => (
                "1d_range_at",
                "array_copy_src_low,array_copy_src_high,array_copy_src_low".to_string(),
            ),
            (0, true) if dims == 1 => ("1d_whole", String::new()),
            (0, false) if dims == 1 => ("1d_whole_at", "array_copy_dst_start".to_string()),
            (0, true) => ("nd_whole", dims.to_string()),
            (0, false) => ("nd_whole_at", format!("{dims},array_copy_dst_start")),
            (n, false) => (
                "nd_range_at",
                format!("{n},array_copy_src_low,array_copy_src_high,array_copy_dst_start"),
            ),
            (n, true) => (
                "nd_range_at",
                format!("{n},array_copy_src_low,array_copy_src_high,array_copy_src_low"),
            ),
        };
        let args: Vec<String> = ["array_copy_src".to_string(), "array_copy_dst".to_string()]
            .into_iter()
            .chain((!args.is_empty()).then_some(args))
            .chain(size)
            .collect();
        let call = format!("qb64_array_copy_{entry}_{kind}({});", args.join(","));
        out.push(match (guarded, dims_checked) {
            // With a reshaped dynamic side the old compiler copies nothing either (read from its C++: the stand-in
            // descriptor is not allocated, so libqb raises error 9): no D-004 trace then.
            (true, true) => format!("if (array_copy_dims){{if (array_copy_ok) {call} else qbr_diverged_on(4);}}"),
            (true, false) => format!("if (array_copy_ok) {call} else qbr_diverged_on(4);"),
            (false, true) => format!("if (array_copy_dims) {call}"),
            (false, false) => call,
        });
        out.push("}".into());
        let strings = |a: &ArrayRef| matches!(a, ArrayRef::Member { base, .. } if base.uses_strings(self.p));
        let values = ranges.iter().flat_map(|(lower, upper)| [lower, upper]).chain(starts);
        if strings(from) || strings(to) || values.into_iter().any(|v| v.uses_strings(self.p)) {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// The number of dimensions of a side of a `CopyArray`, where it is known.
    fn copy_dims(&self, array: &ArrayRef) -> Option<usize> {
        match array {
            ArrayRef::Var(v) => self.p.var(*v).dim_count(),
            ArrayRef::Member { base, member } => Some(self.p.member_of(base, *member).bounds.len()),
        }
    }

    /// Whether an array is a dynamic member array.
    fn is_dynamic_member(&self, array: &ArrayRef) -> bool {
        matches!(array, ArrayRef::Member { base, member } if self.dynamic_member(base, *member).is_some())
    }

    /// The descriptor of a side of a `CopyArray`, as an array argument gets it: an array variable's, a dynamic
    /// member array's own (made, not allocated, where there is none), or one made for a member array.
    fn whole_desc(&mut self, array: &ArrayRef) -> String {
        match array {
            ArrayRef::Var(v) => self.name(*v),
            ArrayRef::Member { base, member } => match self.dynamic_member(base, *member) {
                Some((id, m)) => {
                    let slot = self.dyn_slot(base, *member);
                    format!("{}({slot})", dyn_fn(id, m, "desc"))
                }
                None => self.member_array_arg(base, *member),
            },
        }
    }

    /// `Release`: a dynamic array not allocated again; of a dynamic member array the place is found once.
    pub(crate) fn release_array(&mut self, array: &ArrayRef, out: &mut Vec<String>) {
        match array {
            ArrayRef::Var(array) => {
                let text = self.release(self.p.var(*array), &self.name(*array));
                out.extend(text.lines().map(str::to_string));
            }
            ArrayRef::Member { base, member } => {
                let (id, m) = self
                    .dynamic_member(base, *member)
                    .expect("a Release of a dynamic member array");
                let slot = self.dyn_slot(base, *member);
                out.push(format!("{}(*(ptrszint**){slot});", dyn_fn(id, m, "release")));
                if base.uses_strings(self.p) {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
        }
    }
}
