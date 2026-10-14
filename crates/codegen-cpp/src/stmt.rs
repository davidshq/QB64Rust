//! Operations: one IR operation as C++ lines of its statement, stores by the rule of their place.

use crate::Emitter;
use crate::decl::{Part, dim_count, dim_slot, is_qbs};
use crate::names::{c_type, int_const, proc_name};
use crate::place::fixed_at;
use qb64rust_ir::{ArrayRef, Expr, ExprKind, Facts, LabelId, MemberId, OnError, Op, Place, PrintTo, Resume, Ty, When};
use std::fmt::Write as _;

impl Emitter<'_> {
    /// The handler number of a label, assigned on first use.
    fn handler(&mut self, l: LabelId) -> usize {
        match self.handlers.iter().position(|&h| h == l) {
            Some(i) => i + 1,
            None => {
                self.handlers.push(l);
                self.handlers.len()
            }
        }
    }

    /// One operation. `raised`: an earlier operation of the statement may have raised, so a jump checks for a
    /// pending error (design D9).
    pub(crate) fn op(&mut self, op: &Op, raised: bool, out: &mut Vec<String>) {
        let unless_error = if raised { "if (!is_error_pending()) " } else { "" };
        match op {
            Op::SelectConsole => {
                out.push("sub__dest(func__console());".into());
                out.push("sub__source(func__console());".into());
            }
            Op::End => out.push("sub_end();".into()),
            Op::System => {
                out.push("if (sub_gl_called) error(271);".into());
                out.push("close_program=1;".into());
                out.push("end();".into());
            }
            Op::Exit => out.push("goto exit_subfunc;".into()),
            Op::SetHandler(Some(l)) => {
                let n = self.handler(*l);
                out.push(format!("error_goto_line={n};"));
            }
            Op::SetHandler(None) => {
                out.push("error_goto_line=0;".into());
                out.push("qbs_set(error_handler_history, qbs_new_txt_len(\"\", 0));".into());
            }
            Op::Raise(v) => {
                out.push(format!("error({});", self.value(v)));
                if v.uses_strings(self.p) {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::Resume(r) => {
                let then = match r {
                    Resume::Retry => "error_retry=1; qbevent=1; error_handling=0; error_err=0; return;".to_string(),
                    Resume::Next => "error_handling=0; error_err=0; return;".to_string(),
                    Resume::To(l) => format!("error_handling=0; error_err=0; goto {};", self.label_name(*l)),
                };
                out.push(format!("if (!error_handling){{error(20);}}else{{{then}}}"));
            }
            Op::Assign { place, value } => self.assign(place, value, out),
            Op::CopyWhole { from, to } => self.copy_whole(from, to, out),
            Op::Allocate { array, bounds, mode } => self.allocate(*array, bounds, *mode, out),
            Op::Clear {
                array: ArrayRef::Var(array),
            } => self.clear_array(*array, out),
            Op::Clear {
                array: ArrayRef::Member { base, member },
            } => self.clear_member_array(base, *member, out),
            Op::Release { array } => self.release_array(*array, out),
            Op::EraseParam { array } => self.erase_param(*array, out),
            Op::AssignAll(stores) => {
                for (place, value) in stores {
                    self.assign(&Place::Var(*place), value, out);
                }
            }
            Op::Jump(l) => out.push(format!("{unless_error}goto {};", self.local_label(*l))),
            Op::Branch {
                cond,
                when,
                to,
                on_error,
            } => {
                let mut c = self.value(cond);
                if cond.uses_strings(self.p) {
                    c = format!("qbs_cleanup(qbs_tmp_base,{c})");
                }
                let test = match when {
                    When::Zero => format!("!({c})"),
                    When::NonZero => format!("({c})"),
                };
                // The condition is evaluated first, so a raising condition never jumps.
                let test = match on_error {
                    OnError::Skip => format!("({test})&&(!is_error_pending())"),
                    OnError::UseValue => test,
                };
                out.push(format!("if ({test}) goto {};", self.local_label(*to)));
            }
            Op::Gosub(l) => {
                self.gosubs += 1;
                let g = self.gosubs;
                // With an error pending the jump is not taken and nothing is pushed.
                if raised {
                    out.push("if (!is_error_pending()){".into());
                }
                out.push(format!("return_point[next_return_point++]={g};"));
                out.push("if (next_return_point>=return_points) more_return_points();".into());
                out.push(format!("goto {};", self.local_label(*l)));
                if raised {
                    out.push("}".into());
                }
                out.push(format!("RETURN_{g}:;"));
                writeln!(self.ret_cases, "case {g}:\ngoto RETURN_{g};\nbreak;").unwrap();
            }
            Op::Return(None) => {
                self.returns = true;
                out.push(format!("#include \"ret{}.txt\"", self.k));
            }
            // The decrement is guarded, unlike the old compiler's, whose counter wrapped below zero (DIVERGENCES.md
            // D-003).
            Op::Return(Some(l)) => {
                out.push("if (!next_return_point) error(3); else next_return_point--;".into());
                out.push(format!("goto {};", self.label_name(*l)));
            }
            Op::Call { proc, args } => {
                let q = self.p.proc(*proc);
                let a = self.args(*proc, args);
                out.push(format!("{}({a});", proc_name(q)));
                let string_param = |v: &qb64rust_ir::Var| !v.is_array() && is_qbs(v.ty);
                let strings = args.iter().any(|a| a.uses_strings(self.p))
                    || q.params.iter().any(|&v| string_param(self.p.var(v)));
                if strings {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::Builtin { id, args } => self.stmt_call(*id, args, out),
            Op::MemGet { block, offset, target } => {
                self.mem_get_stmt(block, offset, target, out);
                let p = self.p;
                if block.uses_strings(p) || offset.uses_strings(p) || reference_strings(p, target) {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::MemPut { block, offset, value } => {
                self.mem_put_stmt(block, offset, value, out);
                let p = self.p;
                if block.uses_strings(p) || offset.uses_strings(p) || value_strings(p, value) {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::MemFill {
                block,
                offset,
                bytes,
                value,
            } => {
                self.mem_fill_stmt((block, offset, bytes), value, out);
                let p = self.p;
                if block.uses_strings(p) || offset.uses_strings(p) || bytes.uses_strings(p) || value_strings(p, value) {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::Print {
                to: PrintTo::File(n),
                items,
                newline,
            } => self.file_print(n, items, *newline, out),
            Op::Print {
                to: PrintTo::Screen,
                items,
                newline,
            } => self.print(false, items, *newline, out),
            Op::Print {
                to: PrintTo::Printer,
                items,
                newline,
            } => self.print(true, items, *newline, out),
            Op::PrintUsing {
                to,
                before,
                format,
                values,
                newline,
            } => self.print_using(to, before, format, values, *newline, out),
            Op::Write { to, items, newline } => self.write(to.as_ref(), items, *newline, out),
            Op::Field { file, parts } => self.field(file, parts, out),
            Op::Lset { right, target, value } => self.lset(*right, target, value, out),
            Op::Input { from, line, targets } => self.input(from, *line, targets, out),
            Op::Read(targets) => self.read(targets, out),
            Op::Restore { label, .. } => self.restore(label.as_deref(), out),
        }
    }

    /// A store of a value, by the rule of its place.
    fn assign(&mut self, place: &Place, value: &Expr, out: &mut Vec<String>) {
        if let Ty::User(_) = value.ty {
            return self.store_block(place, value, out);
        }
        let cleanup = value.uses_strings(self.p) || place.uses_strings(self.p);
        self.store(place, is_qbs(value.ty), &mut |e| e.value(value), out);
        if cleanup {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// A block a built-in gives, stored into a place of type `_MEM` (design D2 of `m3-mem`), as the old compiler
    /// writes it: the call's result into a `mem_block` temporary of its own, declared in `global.txt`, and its 56
    /// bytes copied (`memcpy((((char*)__UDT_M)+((0+0))+(0)),(((char*)&(func1=func__memnew( 64 )))+(0)),56);`,
    /// measured, `verification\v32_x01_accepted`). The place is found as a whole value's is, not guarded.
    fn store_block(&mut self, place: &Place, value: &Expr, out: &mut Vec<String>) {
        let to = match place {
            Place::Var(v) => format!("(((char*){})+((0+0))+(0))", self.name(*v)),
            Place::Element { .. } | Place::Member { .. } => format!("({}+(0))", self.bytes_of(place)),
        };
        let v = self.value(value);
        // A value of a user type read by `_MEMGET` is the address of its bytes in the block (`((void*)+…)`),
        // copied from there (measured, `verification\v32_x01_accepted`); a block a built-in gives is held first.
        let from = if let ExprKind::MemRead { .. } = value.kind {
            v
        } else {
            self.funcs += 1;
            let t = format!("func{}", self.funcs);
            self.func_decls.push(format!("mem_block {t};\n"));
            format!("&({t}={v})")
        };
        out.push(format!(
            "memcpy({to},(((char*){from})+(0)),{});",
            self.ty_size(value.ty)
        ));
        if value.uses_strings(self.p) || place.uses_strings(self.p) {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// `CopyWhole`: the bytes of a type that holds no string in one move; of a type that holds strings each part
    /// by itself, a string assigned and any other part's bytes copied (the old compiler's `copy_full_udt`). Both
    /// addresses are written anew for every copy, target first, as the IR says: an index in them is evaluated
    /// each time. Nothing checks for a pending error: after an index outside its bounds `array_check` gives the
    /// first position.
    fn copy_whole(&mut self, from: &Place, to: &Place, out: &mut Vec<String>) {
        let ty = self.p.place_ty(to);
        let (to_at, from_at) = (self.bytes_of(to), self.bytes_of(from));
        let parts = if self.holds_strings(ty) {
            self.parts(ty)
        } else {
            vec![(0, Part::Bytes(self.ty_size(ty)))]
        };
        for (offset, part) in parts {
            out.push("{".into());
            out.push(format!("char *copy_to={to_at};"));
            out.push(format!("char *copy_from={from_at};"));
            out.push(match part {
                Part::Str => format!("qbs_set(*(qbs**)(copy_to+{offset}),*(qbs**)(copy_from+{offset}));"),
                // A place copied to itself: the two may be the same bytes.
                Part::Bytes(n) => format!("memmove(copy_to+{offset},copy_from+{offset},{n});"),
            });
            out.push("}".into());
        }
        if from.uses_strings(self.p) || to.uses_strings(self.p) {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// `Clear` of a member array: the place it is part of is found once, then its elements become zero, empty or
    /// NUL bytes. Nothing checks for a pending error: after an index outside its bounds `array_check` gives
    /// position 0 (measured, `verification\v24_x312`).
    fn clear_member_array(&mut self, base: &Place, member: MemberId, out: &mut Vec<String>) {
        let m = self.p.member_of(base, member);
        let (ty, count) = (m.ty, m.count());
        let first = self.member_array_at(base, member);
        out.push("{".into());
        out.push(format!("char *clear_at={first};"));
        if ty == Ty::Str {
            out.push(format!("tmp_long={count};"));
            out.push("while(tmp_long--) (*(qbs**)(clear_at+tmp_long*8))->len=0;".into());
        } else if self.holds_strings(ty) {
            let text = self.each_element(ty, "clear_at", &count.to_string(), |at| self.zero_value(ty, at));
            out.extend(text.lines().map(str::to_string));
        } else {
            out.push(format!("memset((void*)clear_at,0,{});", self.ty_size(ty) * count));
        }
        out.push("}".into());
        if base.uses_strings(self.p) {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// A store, by the rule of its place (the IR's error rule; `study\02` §2.3 for the old compiler's forms).
    /// `value` writes the C++ of the value to store; it is called once, where the rule evaluates the value (after
    /// an element's indexes; through a member with an index on the way, a number before the indexes and a string
    /// after them). `string`: the value is a string.
    pub(crate) fn store(
        &mut self,
        place: &Place,
        string: bool,
        value: &mut dyn FnMut(&mut Self) -> String,
        out: &mut Vec<String>,
    ) {
        match place {
            Place::Var(id) => {
                let v = value(self);
                if string {
                    out.push(format!("qbs_set({},{v});", self.name(*id)));
                } else if let Ty::Bit { width, signed } = self.p.var(*id).ty {
                    out.push(bit_store(&self.scalar(*id), &v, width, signed));
                } else {
                    out.push(format!("{}={v};", self.scalar(*id)));
                }
            }
            // The index first; with an error pending after it, neither the value nor the store.
            Place::Element {
                array: ArrayRef::Var(array),
                index,
            } => {
                let flat = self.flat_index(*array, index);
                out.push(format!("tmp_long={flat};"));
                let element = self.element_at(*array, "tmp_long");
                let v = value(self);
                if let Ty::Bit { width, .. } = self.p.var(*array).ty {
                    // A packed element: `setbits` keeps the low bits of the value, as a `_BIT` variable does.
                    let name = self.name(*array);
                    out.push(format!(
                        "if (!is_error_pending()) setbits({width},(uint8*)({name}[0]),tmp_long,{v});"
                    ));
                } else if string {
                    out.push(format!("if (!is_error_pending()) qbs_set({element},{v});"));
                } else {
                    out.push(format!("if (!is_error_pending()) {element}={v};"));
                }
            }
            Place::Element {
                array: ArrayRef::Member { .. },
                ..
            } => self.store_through_member(place, value, out),
            Place::Member { .. } if place.has_element() => self.store_through_member(place, value, out),
            Place::Member { .. } => {
                let v = value(self);
                let member = self.load_place(place);
                if string {
                    out.push(format!("qbs_set({member},{v});"));
                } else {
                    out.push(format!("{member}={v};"));
                }
            }
        }
    }

    /// A store into a place reached through a member with an index on the way (`a(i).m = v`, `t.v(i) = v`,
    /// `a(i).v(j) = v`): a number is evaluated before the indexes and a string after them, as in the old
    /// compiler's program (measured, `verification\v18_h_member_index_order`, `v24_x313`, `x316`; its string
    /// store is one call whose first argument, the place, is evaluated first). Each index is checked against
    /// its dimension, outermost first. The store is skipped when an index is out of range or the indexes
    /// raised an error while none was pending before them; the old compiler writes position 0 of that dimension
    /// then (`DIVERGENCES.md` D-004). The block's own variables are in braces, so no `goto` crosses their
    /// initialisation.
    fn store_through_member(
        &mut self,
        place: &Place,
        value: &mut dyn FnMut(&mut Self) -> String,
        out: &mut Vec<String>,
    ) {
        let ty = self.p.place_ty(place);
        let string = is_qbs(ty);
        out.push("{".into());
        if !string {
            let v = value(self);
            out.push(format!("{} mbr_value={v};", c_type(ty)));
        }
        out.push("int32 mbr_pending=is_error_pending();".into());
        out.push("int32 mbr_bad=0;".into());
        out.push("int64 mbr_k;".into());
        out.push("ptrszint mbr_off=0;".into());
        let root = self.member_path(place, out);
        out.push("int32 mbr_ok=!mbr_bad&&(mbr_pending||!is_error_pending());".into());
        // The address last: an index may have given the array another block.
        let at = format!("((char*){root})+mbr_off");
        let store = if let Ty::FixedStr(k) = ty {
            format!("qbs_set({},mbr_value);", fixed_at(&format!("(uint8*)({at})"), k))
        } else if string {
            format!("qbs_set(*(qbs**)({at}),mbr_value);")
        } else {
            format!("*({}*)({at})=mbr_value;", c_type(ty))
        };
        if string {
            // The value is the string itself, not a copy: what an index stored into a variable it reads is
            // what is stored (measured, `v24_x316`).
            let v = value(self);
            out.push(format!("qbs *mbr_value={v};"));
        }
        out.push(format!("if (mbr_ok) {store}"));
        out.push("}".into());
    }

    /// The way to a place for [`Self::store_through_member`]: writes the check of every index on it, outermost
    /// first, adding the place's distance in bytes from its root to `mbr_off` and setting `mbr_bad` for an index
    /// outside its bounds. Gives the root: the pointer of a variable of a user type, or the block of an array.
    fn member_path(&mut self, place: &Place, out: &mut Vec<String>) -> String {
        let check = |count: &str| format!("if ((uptrszint)mbr_k>=(uptrszint){count}){{error(9);mbr_bad=1;}}");
        match place {
            Place::Var(v) => self.name(*v),
            Place::Element {
                array: ArrayRef::Var(array),
                index,
            } => {
                let n = dim_count(self.p.var(*array));
                let name = self.name(*array);
                let size = self.ty_size(self.p.var(*array).ty);
                for (k, i) in index.iter().enumerate() {
                    let s = dim_slot(n, k);
                    out.push(format!("mbr_k=({})-{name}[{s}];", self.value(i)));
                    out.push(check(&format!("{name}[{}]", s + 1)));
                    if k == 0 {
                        out.push("mbr_off=mbr_k;".into());
                    } else {
                        out.push(format!("mbr_off+=mbr_k*{name}[{}];", s + 2));
                    }
                }
                out.push(format!("mbr_off*={size};"));
                format!("{name}[0]")
            }
            Place::Element {
                array: ArrayRef::Member { base, member },
                index,
            } => {
                let root = self.member_path(base, out);
                let offset = self.member_offset(self.p.place_ty(base), *member);
                let size = i64::from(self.ty_size(self.p.member_of(base, *member).ty));
                if offset != 0 {
                    out.push(format!("mbr_off+={offset};"));
                }
                for (i, (lower, count, multiplier)) in index.iter().zip(self.member_dims(base, *member)) {
                    out.push(format!("mbr_k=({})-{};", self.value(i), int_const(lower, Ty::I64)));
                    out.push(check(&count.to_string()));
                    out.push(format!("mbr_off+=mbr_k*{};", multiplier * size));
                }
                root
            }
            Place::Member { base, member } => {
                let root = self.member_path(base, out);
                let offset = self.member_offset(self.p.place_ty(base), *member);
                if offset != 0 {
                    out.push(format!("mbr_off+={offset};"));
                }
                root
            }
        }
    }
}

/// Whether the variable of a statement of memory blocks makes string temporaries: an index that involves strings,
/// or a fixed-length string element (a temporary fixed `qbs` over its bytes).
fn reference_strings(p: &qb64rust_ir::Program, of: &qb64rust_ir::Reference) -> bool {
    of.indexes().iter().any(|i| i.uses_strings(p))
        || matches!(of, qb64rust_ir::Reference::Place(Place::Element { array, .. })
            if matches!(p.array_ty(array), Ty::FixedStr(_)))
}

/// Whether the value of `_MEMPUT` or `_MEMFILL` makes string temporaries.
fn value_strings(p: &qb64rust_ir::Program, v: &qb64rust_ir::MemValue) -> bool {
    match v {
        qb64rust_ir::MemValue::Typed { value, .. } => value.uses_strings(p),
        qb64rust_ir::MemValue::Bytes(of) => reference_strings(p, of),
        qb64rust_ir::MemValue::Str(_) => true,
    }
}

/// A store into a `_BIT * width` scalar `r` (design D5; `qb64pe.bas` 27054–27077): an unsigned one keeps the low
/// `width` bits of the value, a signed one is assigned and then sign-extended from bit `width - 1`.
fn bit_store(r: &str, v: &str, width: u8, signed: bool) -> String {
    let mask = if width == 64 { u64::MAX } else { (1u64 << width) - 1 };
    if signed {
        let sign = 1u64 << (width - 1);
        format!("if (({r}={v})&0x{sign:X}ull){{{r}|=~0x{mask:X}ull;}}else{{{r}&=0x{mask:X}ull;}}")
    } else {
        format!("{r}=({v})&0x{mask:X}ull;")
    }
}
