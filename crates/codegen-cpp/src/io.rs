//! File output and input (design D5 of `m2-builtin-statements`): `PRINT #`, `WRITE` and `INPUT`/`LINE INPUT` as the
//! old compiler writes them (read with `qb64pe -z`, `study\00` §5): the file number stored in `tmp_fileno`, one
//! libqb call per item or target chosen by its type, and the IR's pending-error check after the number and after
//! each item or target (`goto skipN`). Also `READ`, `RESTORE` and the program's data (design D6).

use crate::Emitter;
use crate::names::{c_ident, c_string, c_type};
use qb64rust_ir::{Conv, DataItem, Expr, ExprKind, Op, Place, PrintItem, PrintTo, Source, Ty, UsingValue};
use std::fmt::Write as _;

/// libqb's entries for a print to the screen or to the printer: the one that prints a string and the one that
/// fits it to the line.
fn screen_entries(printer: bool) -> (&'static str, &'static str) {
    if printer {
        ("qbs_lprint", "lprint_makefit")
    } else {
        ("qbs_print", "makefit")
    }
}

/// The bits of the old compiler's type value that say what kind of place a target is (`qb64pe.bas`: `ISPOINTER`
/// and `ISREFERENCE`, plus `ISARRAY` or `ISUDT`). libqb reads none of them; they are written so that the call is
/// the old compiler's.
const VARIABLE: u32 = 0x0840_0000;
const ELEMENT: u32 = 0x08C0_0000;
const MEMBER: u32 = 0x0860_0000;
/// `ISFLOAT`, `ISUNSIGNED`, and the two bits a `_BIT` type carries (`ISINCONVENTIONALMEMORY`, `ISOFFSETINBITS`).
const FLOAT: u32 = 0x2000_0000;
const UNSIGNED: u32 = 0x1000_0000;
const BIT: u32 = 0x0300_0000;
/// Console `INPUT` writes the type without `ISPOINTER`, `ISREFERENCE`, `ISUDT` and `ISINCONVENTIONALMEMORY`, and
/// with `ISOFFSET` for an `_OFFSET` target (which the file and `DATA` readers take no type value for).
const CONSOLE_DROPS: u32 = 0x0A60_0000;
const OFFSET: u32 = 0x0010_0000;

/// The type code libqb's `func_file_input_float` takes for a numeric target of type `ty` (the old compiler's type
/// value: the width in bits, the float and unsigned bits, the kind of place). libqb reads the width, the float bit
/// and the unsigned bit: they give the range a value is checked against (error 6 outside it).
pub(crate) fn input_type_code(ty: Ty, place: &Place) -> u32 {
    let kind = match place {
        Place::Var(_) => VARIABLE,
        Place::Element { .. } => ELEMENT,
        Place::Member { .. } => MEMBER,
    };
    let (bits, flags) = match ty {
        Ty::Bit { width, .. } => (u32::from(width), BIT),
        Ty::F32 => (32, FLOAT),
        Ty::F64 => (64, FLOAT),
        Ty::F80 => (256, FLOAT),
        Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 | Ty::I32 | Ty::U32 | Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => {
            (ty.int_bits().expect("an integer type"), 0)
        }
        Ty::Str | Ty::FixedStr(_) | Ty::User(_) => unreachable!("a numeric target, not {ty:?}"),
    };
    let unsigned = if ty.is_unsigned() { UNSIGNED } else { 0 };
    kind | flags | unsigned | bits
}

impl Emitter<'_> {
    /// The allocations of the op of items being written that run just before its part `n` (design D1 of
    /// `m4-names-implicit-arrays`), where the old compiler writes them: between the code of the parts.
    fn allocs_before(&mut self, n: usize, out: &mut Vec<String>) {
        let here: Vec<_> = self
            .inside
            .iter()
            .filter(|(k, _)| *k == n)
            .map(|(_, a)| a.clone())
            .collect();
        for a in here {
            self.allocate(&a.array, &a.bounds, a.mode, out);
        }
    }

    fn skip_label(&mut self) -> String {
        self.skip += 1;
        format!("skip{}", self.skip)
    }

    /// A file number for `tmp_fileno` (an `int32`): the value without its last integer conversion, which the C++
    /// assignment makes (the old compiler writes `tmp_fileno=qbr(*__DOUBLE_D);`).
    fn file_number(&mut self, n: &Expr) -> String {
        if let ExprKind::Convert {
            how: Conv::Truncate | Conv::Widen,
            from,
        } = &n.kind
            && from.ty.is_int()
        {
            return self.value(from);
        }
        self.value(n)
    }

    /// `PRINT items` to the screen, or `LPRINT items` (`printer`): each item set into the statement's string,
    /// fitted to the line and printed; the IR's raise rule for a print (a raising item skips the rest). `LPRINT`
    /// is the same code with the printer's two entries, between `tab_LPRINT=1;` and `tab_LPRINT=0;` (measured,
    /// `verification\v30_x13`; `study\00` §5).
    pub(crate) fn print(&mut self, printer: bool, items: &[PrintItem], newline: bool, out: &mut Vec<String>) {
        let skip = self.skip_label();
        let (print, _) = screen_entries(printer);
        if printer {
            out.push("tab_LPRINT=1;".into());
        }
        out.push("tqbs=qbs_new(0,0);".into());
        self.screen_items(printer, items, &skip, out);
        if newline {
            out.push(format!("{print}(nothingstring,1);"));
        }
        out.push(format!("{skip}:"));
        out.push("qbs_free(tqbs);".into());
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        if printer {
            out.push("tab_LPRINT=0;".into());
        }
    }

    /// The items of a print to the screen or the printer, each followed by the jump to `skip` when an error is
    /// pending.
    fn screen_items(&mut self, printer: bool, items: &[PrintItem], skip: &str, out: &mut Vec<String>) {
        let (print, makefit) = screen_entries(printer);
        for (k, item) in items.iter().enumerate() {
            self.allocs_before(k, out);
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
            out.push(format!("{makefit}(tqbs);"));
            out.push(format!("{print}(tqbs,0);"));
        }
    }

    /// `PRINT [#n,] [before] USING format; values` and `LPRINT … USING …`, as the old compiler writes it
    /// (`qb64pe.bas` 27285–27400 and 27713–27851; read with `qb64pe -z`, `study\00` §5): the statement's string
    /// and its place in the format, the format copied once into a string of the statement's own, one call per
    /// value chosen by its type, each followed by the jump to the statement's second label when an error is
    /// pending; at that label what was built is printed, with no line end after an error, which stays. The
    /// string of the format, the two labels and the first label of a plain print share one number.
    pub(crate) fn print_using(
        &mut self,
        to: &PrintTo,
        before: &[PrintItem],
        format: &Expr,
        values: &[UsingValue],
        newline: bool,
        out: &mut Vec<String>,
    ) {
        self.skip += 1;
        let n = self.skip;
        let (skip, skip_pu, fmt) = (
            format!("skip{n}"),
            format!("skip_pu{n}"),
            format!("print_using_format{n}"),
        );
        self.pass_decls.push(format!("static qbs *{fmt};\n"));
        let test = |label: &str| format!("if (is_error_pending()) goto {label};");
        let printer = matches!(to, PrintTo::Printer);
        let file = matches!(to, PrintTo::File(_));
        let (print, makefit) = screen_entries(printer);
        // The statement's string sent to where the statement prints, with or without a line end.
        let send = |line_end: bool| {
            let end = u8::from(line_end);
            if file {
                format!("sub_file_print(tmp_fileno,tqbs,0,0,{end});")
            } else {
                format!("{print}(tqbs,{end});")
            }
        };
        match to {
            PrintTo::File(number) => {
                out.push("tab_spc_cr_size=2;".into());
                out.push(format!("tab_fileno=tmp_fileno={};", self.file_number(number)));
                out.push(test(&skip));
                self.file_items(before, false, &skip, out);
                out.push("tqbs=qbs_new(0,0);".into());
            }
            PrintTo::Screen | PrintTo::Printer => {
                if printer {
                    out.push("tab_LPRINT=1;".into());
                }
                out.push("tqbs=qbs_new(0,0);".into());
                self.screen_items(printer, before, &skip, out);
                if !before.is_empty() {
                    out.push("qbs_set(tqbs,qbs_new_txt_len(\"\",0));".into());
                }
            }
        }
        out.push("tmp_long=0;".into());
        self.allocs_before(before.len(), out);
        out.push(format!("{fmt}=qbs_new(0,0); qbs_set({fmt},{});", self.value(format)));
        // After the format a statement that prints to a file leaves by its first label.
        out.push(test(if file { &skip } else { &skip_pu }));
        for (k, v) in values.iter().enumerate() {
            self.allocs_before(before.len() + 1 + k, out);
            match v {
                UsingValue::Str(e) => {
                    out.push(format!("tmp_long=print_using({fmt},tmp_long,tqbs,{});", self.value(e)));
                }
                UsingValue::Num { value, as_ } => {
                    let entry = match as_ {
                        Ty::F32 => "single",
                        Ty::F64 => "double",
                        Ty::F80 => "float",
                        Ty::U64 => "uinteger64",
                        Ty::I64 => "integer64",
                        Ty::I8
                        | Ty::U8
                        | Ty::I16
                        | Ty::U16
                        | Ty::I32
                        | Ty::U32
                        | Ty::Off
                        | Ty::UOff
                        | Ty::Bit { .. }
                        | Ty::Str
                        | Ty::FixedStr(_)
                        | Ty::User(_) => unreachable!("a `PRINT USING` number formatted as {as_:?} (`validate`)"),
                    };
                    let x = self.value(value);
                    out.push(format!("tmp_long=print_using_{entry}({fmt},{x},tmp_long,tqbs);"));
                }
                // `TAB(n)` or `SPC(n)`: what is built is printed, then the blanks; the string starts again.
                UsingValue::Blanks(e) => {
                    out.push(send(false));
                    out.push(format!("qbs_set(tqbs,{});", self.value(e)));
                    out.push(test(&skip_pu));
                    if !file {
                        out.push(format!("{makefit}(tqbs);"));
                    }
                    out.push(send(false));
                    out.push("tqbs->len=0;".into());
                }
            }
            out.push(test(&skip_pu));
        }
        out.push(format!("{skip_pu}:"));
        out.push("if (is_error_pending()){".into());
        out.push(format!(
            "g_tmp_long=new_error; new_error=0; {} new_error=g_tmp_long;",
            send(false)
        ));
        out.push("}else{".into());
        out.push(send(newline));
        out.push("}".into());
        out.push("qbs_free(tqbs);".into());
        out.push(format!("qbs_free({fmt});"));
        out.push(format!("{skip}:"));
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        if file {
            out.push("tab_spc_cr_size=1;".into());
        }
        if printer {
            out.push("tab_LPRINT=0;".into());
        }
    }

    /// `FIELD`, as the old compiler writes it (`qb64pe.bas` 7340–7432; read with `qb64pe -z`, `study\00` §5):
    /// one call for the file, then one per part with the string itself and its width, nothing between them.
    /// libqb keeps the record buffer, the file's field strings and each string's link to its field.
    pub(crate) fn field(&mut self, file: &Expr, parts: &[(Place, Expr)], out: &mut Vec<String>) {
        out.push(format!("field_new({});", self.value(file)));
        for (place, width) in parts {
            let string = self.place_ref(place);
            out.push(format!("field_add({string},{});", self.value(width)));
        }
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
    }

    /// `LSET` and `RSET` (`qb64pe.bas` 11327–11374): the target as a string (a fixed-length element or member
    /// as a temporary fixed `qbs` over its bytes), then the value.
    pub(crate) fn lset(&mut self, right: bool, target: &Place, value: &Expr, out: &mut Vec<String>) {
        let f = if right { "sub_rset" } else { "sub_lset" };
        let string = self.place_ref(target);
        out.push(format!("{f}({string},{});", self.value(value)));
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
    }

    /// `PRINT #n, items`. A comma is the `tab` flag of the item before it, or a call of its own when there is none.
    pub(crate) fn file_print(&mut self, to: &Expr, items: &[PrintItem], newline: bool, out: &mut Vec<String>) {
        let skip = self.skip_label();
        let test = format!("if (is_error_pending()) goto {skip};");
        out.push("tab_spc_cr_size=2;".into());
        out.push(format!("tab_fileno=tmp_fileno={};", self.file_number(to)));
        out.push(test);
        self.file_items(items, newline, &skip, out);
        if items.is_empty() && newline {
            out.push("sub_file_print(tmp_fileno,nothingstring,0,0,1);".into());
        }
        out.push(format!("{skip}:"));
        out.push("tab_spc_cr_size=1;".into());
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
    }

    /// The items of a print to the file in `tmp_fileno`, the last one with the line end if `newline`, each
    /// followed by the jump to `skip` when an error is pending.
    fn file_items(&mut self, items: &[PrintItem], newline: bool, skip: &str, out: &mut Vec<String>) {
        let test = format!("if (is_error_pending()) goto {skip};");
        let mut i = 0;
        while i < items.len() {
            self.allocs_before(i, out);
            let (text, extra_space, zone) = match &items[i] {
                PrintItem::Zone => ("nothingstring".to_string(), 0, true),
                PrintItem::Str(v) => (self.value(v), 0, false),
                PrintItem::Num(v) => (format!("qbs_str(({})({}))", c_type(v.ty), self.value(v)), 1, false),
            };
            let tab = zone || matches!(items.get(i + 1), Some(PrintItem::Zone));
            i += if tab && !zone { 2 } else { 1 };
            let line_end = newline && i == items.len();
            out.push(format!(
                "sub_file_print(tmp_fileno,{text},{extra_space},{},{});",
                u8::from(tab),
                u8::from(line_end)
            ));
            out.push(test.clone());
        }
    }

    /// `WRITE [#n,] items`: each item's text is built here (a number without blanks, a string in quotes, a comma
    /// after every item but the last, or after the last too when the statement ends with one) and sent as one
    /// string.
    pub(crate) fn write(&mut self, to: Option<&Expr>, items: &[PrintItem], newline: bool, out: &mut Vec<String>) {
        let skip = self.skip_label();
        let test = format!("if (is_error_pending()) goto {skip};");
        if let Some(n) = to {
            out.push("tab_spc_cr_size=2;".into());
            out.push(format!("tab_fileno=tmp_fileno={};", self.file_number(n)));
            out.push(test.clone());
        }
        let send = |text: &str, line_end: bool| match to {
            Some(_) => format!("sub_file_print(tmp_fileno,{text},0,0,{});", u8::from(line_end)),
            None => format!("qbs_print({text},{});", u8::from(line_end)),
        };
        let quote = format!("qbs_new_txt_len({},1)", c_string(b"\""));
        for (k, item) in items.iter().enumerate() {
            self.allocs_before(k, out);
            let last = k + 1 == items.len();
            let mut text = match item {
                PrintItem::Num(v) => format!("qbs_ltrim(qbs_str(({})({})))", c_type(v.ty), self.value(v)),
                PrintItem::Str(v) => format!("qbs_add(qbs_add({quote},{}),{quote})", self.value(v)),
                PrintItem::Zone => unreachable!("a `WRITE` has no zones (`validate`)"),
            };
            if !last || !newline {
                text = format!("qbs_add({text},qbs_new_txt_len(\",\",1))");
            }
            out.push(send(&text, last && newline));
            out.push(test.clone());
        }
        if items.is_empty() {
            out.push(send("nothingstring", true));
        }
        out.push(format!("{skip}:"));
        if to.is_some() {
            out.push("tab_spc_cr_size=1;".into());
        }
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
    }

    /// `INPUT` and `LINE INPUT`: one read per target, stored by the rule of its place.
    pub(crate) fn input(&mut self, from: &Source, line: bool, targets: &[Place], out: &mut Vec<String>) {
        if let Source::Console { prompt, question, stay } = from {
            return self.console_input(prompt.as_deref(), *question, *stay, line, targets, out);
        }
        let skip = self.skip_label();
        let test = format!("if (is_error_pending()) goto {skip};");
        match from {
            Source::File(n) => {
                out.push(format!("tmp_fileno={};", self.file_number(n)));
                out.push(test.clone());
            }
            Source::Console { .. } => unreachable!("handled above"),
        }
        for (k, place) in targets.iter().enumerate() {
            self.allocs_before(k, out);
            let ty = self.p.place_ty(place);
            if ty.is_string() {
                let f = if line {
                    "sub_file_line_input_string"
                } else {
                    "sub_file_input_string"
                };
                let target = self.place_ref(place);
                out.push(format!("{f}(tmp_fileno,{target});"));
            } else {
                let read = self.file_read(ty, place);
                self.store(place, false, &mut |_| read.clone(), out);
            }
            out.push(test.clone());
        }
        out.push(format!("{skip}:"));
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
    }

    /// Console `INPUT` and `LINE INPUT`, as the old compiler writes them (`qb64pe.bas` 11123–11236): the prompt and
    /// the `? ` printed, then the type and the address of each target handed to libqb's `qbs_input`, which reads
    /// the line, asks again where it does, and stores through the addresses itself; `stay` is its second argument
    /// as 0 (no line end after the input). `stop_program` is set when the input ended.
    fn console_input(
        &mut self,
        prompt: Option<&[u8]>,
        question: bool,
        stay: bool,
        line: bool,
        targets: &[Place],
        out: &mut Vec<String>,
    ) {
        if let Some(text) = prompt {
            out.push(format!(
                "qbs_print(qbs_new_txt_len({},{}),0);",
                c_string(text),
                text.len()
            ));
        }
        if question {
            out.push("qbs_print(qbs_new_txt(\"? \"),0);".into());
        }
        for (place, k) in targets.iter().zip(1..) {
            self.allocs_before(k - 1, out);
            let ty = self.p.place_ty(place);
            let (code, address) = if ty.is_string() {
                let code = if line { "ISSTRING+512" } else { "ISSTRING" };
                (code.to_string(), self.place_ref(place))
            } else {
                // Of the place bits only `ISARRAY` is written here (an element), and `_OFFSET` carries its own bit.
                let offset = if matches!(ty, Ty::Off | Ty::UOff) { OFFSET } else { 0 };
                let code = input_type_code(ty, place) & !CONSOLE_DROPS | offset;
                (code.to_string(), format!("&({})", self.load_place(place)))
            };
            out.push(format!("qbs_input_variabletypes[{k}]={code};"));
            out.push(format!("qbs_input_variableoffsets[{k}]={address};"));
        }
        out.push(format!("qbs_input({},{});", targets.len(), u8::from(!stay)));
        out.push("if (stop_program) end();".into());
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
    }

    /// `READ`: one read per target, stored by the rule of its place, with no check for a pending error between them
    /// (libqb's readers return at once while one is pending).
    pub(crate) fn read(&mut self, targets: &[Place], out: &mut Vec<String>) {
        const ARGS: &str = "data,&data_offset,data_size";
        for (k, place) in targets.iter().enumerate() {
            self.allocs_before(k, out);
            let ty = self.p.place_ty(place);
            let read = match ty {
                Ty::Str | Ty::FixedStr(_) => {
                    let target = self.place_ref(place);
                    out.push(format!("sub_read_string({ARGS},{target});"));
                    continue;
                }
                Ty::I64 | Ty::Off => format!("func_read_int64({ARGS})"),
                Ty::U64 | Ty::UOff => format!("func_read_uint64({ARGS})"),
                Ty::Bit { .. } => format!("((int64)func_read_float({ARGS},{}))", input_type_code(ty, place)),
                Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 | Ty::I32 | Ty::U32 | Ty::F32 | Ty::F64 | Ty::F80 => {
                    format!("func_read_float({ARGS},{})", input_type_code(ty, place))
                }
                Ty::User(_) => unreachable!("a `READ` target of a user type (`validate`)"),
            };
            self.store(place, false, &mut |_| read.clone(), out);
        }
        out.push("qbs_cleanup(qbs_tmp_base,0);".into());
    }

    /// `RESTORE`: the read position as a byte offset into the stored data, by the old compiler's name for a
    /// label's.
    pub(crate) fn restore(&mut self, label: Option<&str>, out: &mut Vec<String>) {
        match label {
            Some(l) => out.push(format!("data_offset=data_at_LABEL_{};", c_ident(l))),
            None => out.push("data_offset=0;".into()),
        }
    }

    /// The program's data in `global.txt`, as the old compiler stores it (`qb64pe.bas`, "DATA_finalize"): each
    /// item's text followed by a comma, a quoted item in its quotes, as one byte array that libqb's readers parse;
    /// its size; and the offset of each label a `RESTORE` names.
    pub(crate) fn data_fragment(&self) -> String {
        let stored = |item: &DataItem| item.text.len() + if item.quoted { 2 } else { 0 } + 1;
        let offset = |at: usize| self.p.data[..at].iter().map(stored).sum::<usize>();
        let mut out = String::new();
        let mut named: Vec<&str> = Vec::new();
        let bodies = std::iter::once(&self.p.main).chain(self.p.procs.iter().map(|q| &q.body));
        for op in bodies.flat_map(|b| &b.stmts).flat_map(|s| &s.ops) {
            if let Op::Restore { at, label: Some(l) } = op
                && !named.contains(&l.as_str())
            {
                named.push(l);
                writeln!(out, "ptrszint data_at_LABEL_{}={};", c_ident(l), offset(*at)).unwrap();
            }
        }
        writeln!(out, "ptrszint data_size={};", offset(self.p.data.len())).unwrap();
        if self.p.data.is_empty() {
            out.push_str("uint8 *data=(uint8*)calloc(1,1);\n");
            return out;
        }
        out.push_str("uint8 inline_data[]={\n");
        for item in &self.p.data {
            let quote: &[u8] = if item.quoted { b"\"" } else { b"" };
            for b in quote.iter().chain(&item.text).chain(quote).chain(b",") {
                write!(out, "{b},").unwrap();
            }
        }
        out.push_str("\n0};\nuint8 *data=&inline_data[0];\n");
        out
    }

    /// The libqb call that reads a number for a target of type `ty` from the file in `tmp_fileno`: the 64-bit
    /// integer readers for `_INTEGER64` and `_OFFSET`, the float reader with the target's type code for every other
    /// type (it checks the range and rounds).
    fn file_read(&self, ty: Ty, place: &Place) -> String {
        match ty {
            Ty::I64 | Ty::Off => "func_file_input_int64(tmp_fileno)".to_string(),
            Ty::U64 | Ty::UOff => "func_file_input_uint64(tmp_fileno)".to_string(),
            Ty::Bit { .. } => format!(
                "((int64)func_file_input_float(tmp_fileno,{}))",
                input_type_code(ty, place)
            ),
            Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 | Ty::I32 | Ty::U32 | Ty::F32 | Ty::F64 | Ty::F80 => {
                format!("func_file_input_float(tmp_fileno,{})", input_type_code(ty, place))
            }
            Ty::Str | Ty::FixedStr(_) | Ty::User(_) => unreachable!("a numeric target, not {ty:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::input_type_code;
    use qb64rust_ir::{ArrayRef, MemberId, Place, Ty, VarId};

    /// The codes the old compiler wrote for each target type (`qb64pe -z`, 2026-10-10; `study\00` §5).
    #[test]
    fn type_codes_as_the_old_compiler_writes_them() {
        let var = Place::Var(VarId(0));
        let bit = |width, signed| Ty::Bit { width, signed };
        for (ty, code) in [
            (Ty::I8, 138_412_040),
            (Ty::U8, 406_847_496),
            (Ty::I16, 138_412_048),
            (Ty::U16, 406_847_504),
            (Ty::I32, 138_412_064),
            (Ty::U32, 406_847_520),
            (Ty::F32, 675_282_976),
            (Ty::F64, 675_283_008),
            (Ty::F80, 675_283_200),
            (bit(5, true), 188_743_685),
            (bit(40, false), 457_179_176),
        ] {
            assert_eq!(input_type_code(ty, &var), code, "{ty:?}");
        }
        let element = Place::Element {
            array: ArrayRef::Var(VarId(0)),
            index: Vec::new(),
        };
        assert_eq!(input_type_code(Ty::I32, &element), 146_800_672);
        let member = Place::Member {
            base: Box::new(var),
            member: MemberId(0),
        };
        assert_eq!(input_type_code(Ty::I32, &member), 140_509_216);
    }
}
