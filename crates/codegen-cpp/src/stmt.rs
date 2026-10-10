//! Operations: one IR operation as C++ lines of its statement, stores by the rule of their place.

use crate::Emitter;
use crate::decl::{dim_slot, is_qbs};
use crate::names::{c_type, proc_name};
use crate::place::fixed_at;
use qb64rust_ir::{Expr, Facts, LabelId, OnError, Op, Place, PrintItem, Resume, Ty, When};
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
                let strings =
                    args.iter().any(|a| a.uses_strings(self.p)) || q.params.iter().any(|&v| is_qbs(self.p.var(v).ty));
                if strings {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::Builtin { id, args } => self.stmt_call(*id, args, out),
            Op::Print {
                to: Some(n),
                items,
                newline,
            } => self.file_print(n, items, *newline, out),
            Op::Write { to, items, newline } => self.write(to.as_ref(), items, *newline, out),
            Op::Input { from, line, targets } => self.input(from, *line, targets, out),
            Op::Read(targets) => self.read(targets, out),
            Op::Restore { label, .. } => self.restore(label.as_deref(), out),
            Op::Print {
                to: None,
                items,
                newline,
            } => {
                // The IR's raise rule for PRINT: a raising item skips the rest of the statement.
                self.skip += 1;
                let skip = format!("skip{}", self.skip);
                out.push("tqbs=qbs_new(0,0);".into());
                for item in items {
                    let text = match item {
                        PrintItem::Zone => {
                            out.push("tab();".into());
                            continue;
                        }
                        PrintItem::Str(v) => self.value(v),
                        PrintItem::Num(v) => {
                            format!(
                                "qbs_add(qbs_str(({})({})),qbs_new_txt(\" \"))",
                                c_type(v.ty),
                                self.value(v)
                            )
                        }
                    };
                    out.push(format!("qbs_set(tqbs,{text});"));
                    out.push(format!("if (is_error_pending()) goto {skip};"));
                    out.push("makefit(tqbs);".into());
                    out.push("qbs_print(tqbs,0);".into());
                }
                if *newline {
                    out.push("qbs_print(nothingstring,1);".into());
                }
                out.push(format!("{skip}:"));
                out.push("qbs_free(tqbs);".into());
                out.push("qbs_cleanup(qbs_tmp_base,0);".into());
            }
        }
    }

    /// A store of a value, by the rule of its place.
    fn assign(&mut self, place: &Place, value: &Expr, out: &mut Vec<String>) {
        let cleanup = value.uses_strings(self.p) || place.uses_strings(self.p);
        self.store(place, is_qbs(value.ty), &mut |e| e.value(value), out);
        if cleanup {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// A store, by the rule of its place (the IR's error rule; `study\02` §2.3 for the old compiler's forms).
    /// `value` writes the C++ of the value to store; it is called once, where the rule evaluates the value (after
    /// an element's indexes, before the indexes of a member of an element). `string`: the value is a string.
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
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                out.push(format!("tmp_long={flat};"));
                let element = self.element_at(*array, "tmp_long");
                let v = value(self);
                if string {
                    out.push(format!("if (!is_error_pending()) qbs_set({element},{v});"));
                } else {
                    out.push(format!("if (!is_error_pending()) {element}={v};"));
                }
            }
            Place::Member { .. } if place.has_element() => {
                let v = value(self);
                self.store_member_of_element(place, &v, out);
            }
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

    /// A store into a member of an element (`a(i).m = v`): the value first, then the indexes, as the old compiler
    /// (measured, `verification\v18_h_member_index_order`); each index checked against its dimension. The store is
    /// skipped when an index is out of range or the indexes raised an error while none was pending before them;
    /// the old compiler writes the first element then (`DIVERGENCES.md` D-004). The block's own variables are in
    /// braces, so no `goto` crosses their initialisation.
    fn store_member_of_element(&mut self, place: &Place, value: &str, out: &mut Vec<String>) {
        let ty = self.p.place_ty(place);
        let (array, index, offset) = self.element_root(place);
        let n = self.p.var(array).dims.len();
        let name = self.name(array);
        let size = self.ty_size(self.p.var(array).ty);
        out.push("{".into());
        // A string value is copied first, so the indexes cannot change it (a FUNCTION in an index may assign the
        // variable the value reads).
        let fixed = if let Ty::FixedStr(k) = ty { Some(k) } else { None };
        match fixed {
            Some(_) => out.push(format!("qbs *mbr_value=qbs_set(qbs_new(0,1),{value});")),
            None => out.push(format!("{} mbr_value={value};", c_type(ty))),
        }
        out.push("int32 mbr_pending=is_error_pending();".into());
        out.push("int32 mbr_bad=0;".into());
        out.push("int64 mbr_k;".into());
        out.push("ptrszint mbr_flat=0;".into());
        for (k, i) in index.iter().enumerate() {
            let s = dim_slot(n, k);
            out.push(format!("mbr_k=({})-{name}[{s}];", self.value(i)));
            out.push(format!(
                "if ((uptrszint)mbr_k>=(uptrszint){name}[{}]){{error(9);mbr_bad=1;}}",
                s + 1
            ));
            if k == 0 {
                out.push("mbr_flat=mbr_k;".into());
            } else {
                out.push(format!("mbr_flat+=mbr_k*{name}[{}];", s + 2));
            }
        }
        let at = format!("((char*){name}[0])+(mbr_flat*{size}+{offset})");
        let store = match fixed {
            Some(k) => format!("qbs_set({},mbr_value);", fixed_at(&format!("(uint8*)({at})"), k)),
            None => format!("*({}*)({at})=mbr_value;", c_type(ty)),
        };
        out.push(format!("if (!mbr_bad&&(mbr_pending||!is_error_pending())) {store}"));
        out.push("}".into());
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
