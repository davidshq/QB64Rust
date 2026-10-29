//! Built-in functions: the call of a built-in in an expression, written as the old compiler writes it for the
//! function's rule (design D5 of `m2-core-builtins`; the entries and their choice by argument type are read from
//! `qb64pe.bas` `evaluatefunc` and the C++ it writes, `study\00` §5).

use crate::Emitter;
use crate::decl::dim_count;
use crate::names::c_type;
use qb64rust_base::fixes::Fix;
use qb64rust_builtins::BuiltinId;
use qb64rust_builtins::passing::plan;
use qb64rust_ir::builtins::{
    Rule, Slot, StmtRule, StmtSlot, cpp_arithmetic, find, int_entry, radix_width, slots, stmt_find, stmt_slots,
};
use qb64rust_ir::{ArrayRef, Expr, ExprKind, Facts, Place, StmtArg, Ty, VarId, numeric_value_types};

impl Emitter<'_> {
    /// A built-in call. A built-in no rule covers (`ERR`, `ERL` are `Fixed`; every call `sema` makes has a rule) is
    /// written by the table.
    pub(crate) fn call(&mut self, id: BuiltinId, args: &[Option<Expr>], ty: Ty, qb: Ty) -> String {
        let Some(s) = find(id) else {
            return self.table_call(id, args, &[]);
        };
        let arg = |k: usize| args[k].as_ref().expect("a required argument");
        match s.rule {
            // `ENVIRON$`: C++ overloading picks the entry for a string or for a number. `_EMBEDDED$`: the handle's
            // literal, `func__embedded(qbs_new_txt_len("h",1))` (design D2 of `m3-network-embed`).
            Rule::Plain | Rule::ResultOfArg | Rule::FloatByArg | Rule::Fixed(_) | Rule::StrOrIndex | Rule::Embedded => {
                let slots: Vec<Slot> = slots(s).into_iter().map(|(slot, _)| slot).collect();
                self.table_call(id, args, &slots)
            }
            // The arguments given, each converted by its slot; libqb has an entry for each number of them.
            Rule::GivenArgs(_) => {
                let slots: Vec<Slot> = slots(s).into_iter().map(|(slot, _)| slot).collect();
                let given: Vec<Option<Expr>> = args.iter().filter(|a| a.is_some()).cloned().collect();
                self.table_call(id, &given, &slots[..given.len()])
            }
            Rule::IntFix => {
                let a = arg(0);
                let x = self.value(a);
                // `INT`: `std::floor` of a float, the value of an integer; `FIX`: `func_fix_float` above 64 bits.
                let f = match (s.name, a.qb) {
                    (_, t) if t.is_int() => "",
                    ("INT", _) => "std::floor",
                    (_, Ty::F80) => "func_fix_float",
                    (_, Ty::F32 | Ty::F64) => "func_fix_double",
                    (_, t) => unreachable!("`{}` of a {t:?}", s.name),
                };
                format!("{f}({x})")
            }
            Rule::Exp => {
                let x = self.value(arg(0));
                let f = if qb == Ty::F32 {
                    "func_exp_single"
                } else {
                    "func_exp_float"
                };
                format!("{f}({x})")
            }
            Rule::Convert(to) => self.convert_fn(to, arg(0)),
            Rule::Len => format!("((int32)({})->len)", self.value(arg(0))),
            // The result type says which parse (`check\builtins.rs`).
            Rule::Val => {
                let x = self.value(arg(0));
                match ty {
                    Ty::F32 => format!("((float)qbs_val<long double>({x}))"),
                    Ty::F64 => format!("((double)qbs_val<long double>({x}))"),
                    Ty::F80 => format!("qbs_val<long double>({x})"),
                    Ty::I64 => format!("qbs_val<int64_t>({x})"),
                    Ty::U64 => format!("qbs_val<uint64_t>({x})"),
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
                    | Ty::User(_) => unreachable!("VAL typed {ty:?}"),
                }
            }
            Rule::Radix(bits) => {
                let a = arg(0);
                let x = self.value(a);
                let f = match bits {
                    4 => "func_hex",
                    3 => "func_oct",
                    1 => "func__bin",
                    _ => unreachable!("{bits} bits per digit"),
                };
                match radix_width(bits, a) {
                    Some(w) => format!("{f}({x},{w})"),
                    None => format!("{f}_float({x})"),
                }
            }
            Rule::Asc => {
                let s = self.value(arg(0));
                match &args[1] {
                    Some(p) => format!("qbs_asc({s},{})", self.value(p)),
                    None => format!("qbs_asc({s})"),
                }
            }
            // `STRING$(n, s$)` takes the string's first byte.
            Rule::StringFill => {
                let n = self.value(arg(0));
                let c = arg(1);
                let code = if c.ty == Ty::Str {
                    format!("({})->chr[0]", self.value(c))
                } else {
                    self.value(c)
                };
                format!("func_string({n},{code})")
            }
            // `std::min<T>(a,b)`, `std::max<T>(a,b)`, `func_clamp<T>(value,low,high)`: the arguments as they are,
            // T the C++ type of what the result is believed (`qb64pe.bas` 20771–20850).
            Rule::Widest => {
                let f = match s.name {
                    "_MIN" => "std::min",
                    "_MAX" => "std::max",
                    "_CLAMP" => "func_clamp",
                    other => unreachable!("`{other}` is no least or greatest"),
                };
                let parts: Vec<String> = args.iter().flatten().map(|a| self.inner(a)).collect();
                format!("{f}<{}>({})", cpp_arithmetic(qb), parts.join(","))
            }
            // `func__rol<uintN_t>(value,count)`: N the width the value is turned in, the result's.
            Rule::Rotate => {
                let f = match s.name {
                    "_ROL" => "func__rol",
                    "_ROR" => "func__ror",
                    other => unreachable!("`{other}` is no rotation"),
                };
                let (value, count) = (self.inner(arg(0)), self.inner(arg(1)));
                format!("{f}<{}>({value},{count})", cpp_arithmetic(ty))
            }
            Rule::Choice => unreachable!("`_IIF` is an `ExprKind::Choice`"),
            Rule::Own | Rule::Cast => unreachable!("`{}` is an expression of its own", s.name),
            // `l2string(x)`, `string2l(s)`; a `_BIT` with its width (`bit2string(1,x)`, `string2bit(s,1)`). The type
            // is the rule's, or for `_MK$` and `_CV` the one their first slot holds (`qb64pe.bas` 21217–21296).
            Rule::Mk(fixed) | Rule::Cv(fixed) => {
                let (t, x) = match fixed {
                    Some(t) => (t, arg(0)),
                    None => (arg(0).ty, arg(1)),
                };
                let v = self.value(x);
                let make = matches!(s.rule, Rule::Mk(_));
                match (t, make) {
                    (Ty::Bit { width, signed }, true) => {
                        format!("{}bit2string({width},{v})", if signed { "" } else { "u" })
                    }
                    (Ty::Bit { width, signed }, false) => {
                        format!("string2{}bit({v},{width})", if signed { "" } else { "u" })
                    }
                    (t, true) => format!("{}2string({v})", bytes_letters(t)),
                    (t, false) => format!("string2{}({v})", bytes_letters(t)),
                }
            }
        }
    }

    /// `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND` of an argument: the libqb entry the old compiler picks by the
    /// argument's believed type, or the value itself (`qb64pe.bas` 21116–21215).
    fn convert_fn(&mut self, to: Ty, a: &Expr) -> String {
        let x = self.value(a);
        let int = a.qb.is_int();
        let f = match (to, a.qb) {
            (Ty::I16 | Ty::I32, _) if int => int_entry(to, a.qb).unwrap_or(""),
            (Ty::I16, Ty::F80) => "func_cint_float",
            (Ty::I16, Ty::F32 | Ty::F64) => "func_cint_double",
            (Ty::I32, Ty::F80) => "func_clng_float",
            (Ty::I32, Ty::F32 | Ty::F64) => "func_clng_double",
            (Ty::F32, Ty::F64) => "func_csng_double",
            (Ty::F32, Ty::F80) => "func_csng_float",
            (Ty::F64, Ty::F80) => "func_cdbl_float",
            (Ty::F32 | Ty::F64, _) if int => return format!("((double)({x}))"),
            (Ty::I64, Ty::F80) => "func_round_float",
            (Ty::I64, Ty::F32 | Ty::F64) => "func_round_double",
            (Ty::I64, _) if int => "",
            (Ty::F32, Ty::F32) | (Ty::F64, Ty::F32 | Ty::F64) => "",
            (_, from) => unreachable!("a conversion to {to:?} of {from:?}"),
        };
        format!("{f}({x})")
    }

    /// A built-in statement, written by its rule (design D5 of `m2-builtin-statements`).
    pub(crate) fn stmt_call(&mut self, id: BuiltinId, args: &[StmtArg], out: &mut Vec<String>) {
        let form = stmt_find(id).unwrap_or_else(|| unreachable!("{} is no compiled statement", id.get().name));
        match form.rule {
            StmtRule::Plain | StmtRule::Graphics => out.push(self.plain_stmt(id, args)),
            StmtRule::Log(_) => unreachable!("a log statement is an `Op::Log` (`validate`)"),
            StmtRule::File => out.push(self.file_get_put(id, args)),
            // One call per number, in order; every file when there is none.
            StmtRule::Close => {
                for arg in args {
                    let StmtArg::Value(n) = arg else {
                        unreachable!("`CLOSE` takes values (`validate`)");
                    };
                    out.push(format!("sub_close({},1);", self.value(n)));
                }
                if args.is_empty() {
                    out.push("sub_close(NULL,0);".into());
                }
            }
            // The runtime's entry by the places' type (`qb64pe.bas` 11412–11502): strings of any kind, a whole
            // `TYPE` by its size (the second place first, as the old compiler writes it), a number by its width.
            // An element's index is checked inside the call's arguments, with no test after it, so a bad index
            // exchanges with the array's first element, as the old compiler's program does (measured).
            //
            // When an index of either place calls a FUNCTION, the two places are found in the order of the call's
            // arguments, each into a variable first: as arguments of one call their order is the C++ compiler's,
            // left to right on Windows (the old compiler's `g++`, llvm-mingw's `clang`), right to left with `g++` on
            // Linux (measured, `tests\corpus\slice\s61_member_arrays` line 26; `QB64PE-DIVERGENCES.md` D-054).
            // Otherwise the call is the old compiler's.
            StmtRule::Swap => {
                let [StmtArg::Place(a), StmtArg::Place(b)] = args else {
                    unreachable!("`SWAP` takes two places (`validate`)");
                };
                let ty = self.p.place_ty(a);
                // The entry, its two place arguments in order, and what follows them.
                let (name, x, y, tail) = match ty {
                    Ty::Str | Ty::FixedStr(_) => (
                        "swap_string".to_string(),
                        self.place_ref(a),
                        self.place_ref(b),
                        String::new(),
                    ),
                    Ty::User(_) => {
                        let (x, y) = (self.bytes_of(b), self.bytes_of(a));
                        match self.ty_size(ty) {
                            n @ (1 | 2 | 4 | 8) => (format!("swap_{}", n * 8), x, y, String::new()),
                            n => ("swap_block".to_string(), x, y, format!(",{n}")),
                        }
                    }
                    Ty::F80 => (
                        "swap_longdouble".to_string(),
                        format!("&{}", self.load_place(a)),
                        format!("&{}", self.load_place(b)),
                        String::new(),
                    ),
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
                    | Ty::F64 => (
                        format!("swap_{}", self.ty_size(ty) * 8),
                        format!("&{}", self.load_place(a)),
                        format!("&{}", self.load_place(b)),
                        String::new(),
                    ),
                    Ty::Bit { .. } => unreachable!("`SWAP` of a `_BIT` (`validate`)"),
                };
                out.push(if a.calls_procedure() || b.calls_procedure() {
                    format!("{{auto swap_x={x};auto swap_y={y};{name}(swap_x,swap_y{tail});}}")
                } else {
                    format!("{name}({x},{y}{tail});")
                });
                // A whole array with no storage on either side: error 9 and no exchange (`study\39` row 7, D-042),
                // where the old compiler exchanges with libqb's `nothingvalue` (`verification\v62_c3`).
                let missing: Vec<String> = [a, b]
                    .into_iter()
                    .filter_map(|p| match p {
                        Place::Element {
                            array: ArrayRef::Var(v),
                            index,
                        } if index.is_empty() && self.checking => Some(Self::whole_missing(&self.name(*v))),
                        Place::Var(_) | Place::Element { .. } | Place::Member { .. } => None,
                    })
                    .collect();
                if !missing.is_empty()
                    && let Some(call) = out.pop()
                {
                    out.push(format!(
                        "if ({}){{qbr_diverged(42);error(9);}}else{{{call}}}",
                        missing.join("||")
                    ));
                }
            }
            // `sub_mid(target, start, length, value, passed)`: 0 for a length left out.
            StmtRule::MidAssign => {
                let [StmtArg::Place(target), StmtArg::Value(start), length, StmtArg::Value(v)] = args else {
                    unreachable!("the slots of the `MID$` statement (`validate`)");
                };
                let target = self.place_ref(target);
                let start = self.value(start);
                let (length, passed) = match length {
                    StmtArg::Value(l) => (self.value(l), 1),
                    StmtArg::Absent
                    | StmtArg::Place(_)
                    | StmtArg::Block { .. }
                    | StmtArg::Bytes(_)
                    | StmtArg::Rest(_)
                    | StmtArg::Word(_) => ("0".to_string(), 0),
                };
                let v = self.value(v);
                out.push(format!("sub_mid({target},{start},{length},{v},{passed});"));
            }
            // Inline, as the old compiler writes it (`qb64pe.bas` 8410–8436): the place, the position and the
            // value in that order, each skipped once an error is pending; error 5 for an empty string or a
            // position outside it.
            StmtRule::AscAssign => {
                let [StmtArg::Place(target), position, StmtArg::Value(v)] = args else {
                    unreachable!("the slots of the `ASC` statement (`validate`)");
                };
                out.push(format!("tqbs={}; if (!is_error_pending()){{", self.place_ref(target)));
                match position {
                    StmtArg::Value(p) => {
                        out.push(format!("tmp_fileno={}; if (!is_error_pending()){{", self.value(p)));
                        out.push(format!("tmp_long={}; if (!is_error_pending()){{", self.value(v)));
                        out.push(
                            "if ((tmp_fileno>0)&&(tmp_fileno<=tqbs->len)){tqbs->chr[tmp_fileno-1]=tmp_long;}else{error(5);}"
                                .into(),
                        );
                        out.push("}}}".into());
                    }
                    StmtArg::Absent
                    | StmtArg::Place(_)
                    | StmtArg::Block { .. }
                    | StmtArg::Bytes(_)
                    | StmtArg::Rest(_)
                    | StmtArg::Word(_) => {
                        out.push(format!("tmp_long={}; if (!is_error_pending()){{", self.value(v)));
                        out.push("if (tqbs->len){tqbs->chr[0]=tmp_long;}else{error(5);}".into());
                        out.push("}}".into());
                    }
                }
            }
        }
        if args.iter().any(|a| a.uses_strings(self.p)) {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// A statement call by the table's `callname` and the old compiler's template rule
    /// (`qb64rust_builtins::passing`): each argument and each several-alternative choice is a C argument (the
    /// value, the alternative's number from 1, `NULL` when left out), and the `passed` mask comes last where the
    /// template has optional parts that need a flag.
    fn plain_stmt(&mut self, id: BuiltinId, args: &[StmtArg]) -> String {
        let b = id.get();
        let plan = plan(&b.template());
        let slots = stmt_slots(id);
        let mut c_args = Vec::new();
        let mut mask = 0u32;
        for ((part, slot), arg) in plan.parts.iter().zip(slots).zip(args) {
            let text = match arg {
                StmtArg::Value(v) => self.value(v),
                StmtArg::Word(w) => (u32::from(*w) + 1).to_string(),
                StmtArg::Absent => "NULL".to_string(),
                // A block (`_MEMFREE`, `_MEMCOPY`): the address of its place.
                StmtArg::Place(place) => self.mem_ref(place),
                StmtArg::Bytes(_) => unreachable!("`{}` takes no place's bytes (`validate`)", b.name),
                StmtArg::Block { array, index } => self.block(*array, index),
                StmtArg::Rest(place) => self.rest_bytes(place),
            };
            // The fill of `PAINT`: the old compiler's cast picks libqb's entry for a colour or for a tile string,
            // `(uint32)NULL` for a fill left out (measured, design D2 of `m3-window-output`).
            let text = match (slot, arg) {
                (
                    StmtSlot::Arg {
                        slot: Slot::ColorOrTile,
                        ..
                    },
                    StmtArg::Value(v),
                ) if v.ty == Ty::Str => format!("(qbs*){text}"),
                (
                    StmtSlot::Arg {
                        slot: Slot::ColorOrTile,
                        ..
                    },
                    _,
                ) => format!("(uint32){text}"),
                (StmtSlot::Arg { .. } | StmtSlot::Choice { .. }, _) => text,
            };
            if !matches!(arg, StmtArg::Absent) {
                mask |= part.flag;
            }
            if part.passed {
                c_args.push(text);
            }
        }
        if plan.mask {
            c_args.push(mask.to_string());
        }
        format!("{}({});", b.callname, c_args.join(","))
    }

    /// The bytes of an array from an element to its end (the whole array: from its first element), for graphics
    /// `GET` and `PUT`, as the old compiler writes them (measured, `study\00` §5): libqb's `byte_element` of the
    /// element's address, the bytes left (none while the array is not allocated) and a temporary of its own,
    /// declared with the variables of the body. The index is written twice; `sema` allows none that calls a
    /// procedure.
    pub(crate) fn block(&mut self, array: VarId, index: &[Expr]) -> String {
        let (v, n) = (self.p.var(array), self.name(array));
        let size = self.ty_size(v.ty);
        let flat = if index.is_empty() {
            "0".to_string()
        } else {
            self.flat_index(array, index)
        };
        // The number of elements, the last dimension's count first, as the old compiler writes the product.
        let counts: Vec<String> = (1..=dim_count(v)).map(|i| format!("{n}[{}]", 4 * i + 1)).collect();
        let counts = counts.join("*");
        let t = self.byte_element_temp();
        // The address and the bytes left by the element type, each as the old compiler spells it. Only a file's
        // `GET` and `PUT` take an array of fixed-length strings or of a user type, and only whole.
        match v.ty {
            Ty::FixedStr(_) => format!(
                "byte_element((uint64)(&((uint8*)({n}[0]))[({flat})*{size}]),\
                 (({size}*({n}[2]&1)*{counts})-({size}*({flat}))),{t})"
            ),
            Ty::User(_) => format!(
                "byte_element((uint64)(((char*){n}[0])+((({flat})*{size}+0))),\
                 ({size}*({n}[2]&1)*{counts})-((({flat})*{size}+0)),{t})"
            ),
            numeric_value_types!() => format!(
                "byte_element((uint64)(&({})),(({size}*({n}[2]&1)*{counts})-({size}*({flat}))),{t})",
                self.element_at(array, &flat)
            ),
            Ty::Str | Ty::Bit { .. } => unreachable!("the bytes of an array of {:?} (`validate`)", v.ty),
        }
    }

    /// A new temporary for libqb's `byte_element`, declared with the variables of the body.
    pub(crate) fn byte_element_temp(&mut self) -> String {
        self.byte_element += 1;
        let t = format!("byte_element_{}", self.byte_element);
        self.pass_decls.push(format!(
            "byte_element_struct *{t}=NULL;\nif (!{t}){{\n\
             if ((mem_static_pointer+=12)<mem_static_limit) {t}=(byte_element_struct*)(mem_static_pointer-12); \
             else {t}=(byte_element_struct*)mem_static_malloc(12);\n}}\n"
        ));
        t
    }

    /// `GET` and `PUT` on a file, as the old compiler writes them (`qb64pe.bas` 11580–11955; read with `qb64pe
    /// -z`, `study\00` §5): the file number, the position or `NULL`, what the variable is, and the mask, 1 when
    /// the position is given. With no variable the call is `field_get` or `field_put` (the file's record
    /// buffer); a variable-length string goes to `sub_get2` as the string itself and to `sub_put2` as its
    /// characters and their number; anything else to `sub_get` or `sub_put` as its address and its size.
    fn file_get_put(&mut self, id: BuiltinId, args: &[StmtArg]) -> String {
        let get = id.get().callname == "sub_get";
        let [StmtArg::Value(file), position, variable] = args else {
            unreachable!("the slots of `GET` and `PUT` on a file (`validate`)");
        };
        let file = self.value(file);
        let (position, mask) = match position {
            StmtArg::Value(v) => (self.value(v), 1),
            StmtArg::Absent
            | StmtArg::Place(_)
            | StmtArg::Block { .. }
            | StmtArg::Bytes(_)
            | StmtArg::Rest(_)
            | StmtArg::Word(_) => ("NULL".to_string(), 0),
        };
        let word = if get { "get" } else { "put" };
        match variable {
            StmtArg::Bytes(place) if self.p.place_ty(place) == Ty::Str => {
                let s = self.place_ref(place);
                if get {
                    return format!("sub_get2({file},{position},{s},{mask});");
                }
                // A member's string, or an element's of a member array, is `*(qbs**)(…)`, which needs parentheses
                // before `->`; a variable's name and an element's text keep the old compiler's text.
                let s = if place.in_member() { format!("({s})") } else { s };
                // Written twice, as the old compiler writes it: an element's index is evaluated twice.
                let t = self.byte_element_temp();
                format!("sub_put2({file},{position},byte_element((uint64){s}->chr,{s}->len,{t}),{mask});")
            }
            StmtArg::Bytes(place) => {
                let (address, size) = self.place_bytes(place);
                let t = self.byte_element_temp();
                format!("sub_{word}({file},{position},byte_element((uint64){address},{size},{t}),{mask});")
            }
            // A whole array with no storage: error 9 and nothing read or written (`study\39` row 7, D-042), where the
            // old compiler reads or writes no byte and raises nothing (`verification\v62_c4`). Below `$CHECKING:OFF`
            // the call alone, as the old compiler writes it (as `SWAP` and `whole_index`).
            StmtArg::Block { array, index } if index.is_empty() => {
                let call = if self.owns_desc(self.p.var(*array).ty) {
                    self.owner_file(*array, get, (&file, &position, mask))
                } else {
                    let bytes = self.block(*array, index);
                    format!("sub_{word}({file},{position},{bytes},{mask});")
                };
                if !self.checking {
                    return call;
                }
                let missing = Self::whole_missing(&self.name(*array));
                format!("if {missing}{{qbr_diverged(42);error(9);}}else{{{call}}}")
            }
            StmtArg::Block { array, index } => {
                let bytes = self.block(*array, index);
                format!("sub_{word}({file},{position},{bytes},{mask});")
            }
            StmtArg::Absent | StmtArg::Value(_) | StmtArg::Place(_) | StmtArg::Rest(_) | StmtArg::Word(_) => {
                format!("field_{word}({file},{position},{mask});")
            }
        }
    }

    /// The address and the size in bytes of a place with a fixed size: a variable through its own pointer (a
    /// fixed-length string through its characters), a value of a user type as the old compiler spells its
    /// address, an element or a member through its address.
    fn place_bytes(&mut self, place: &Place) -> (String, u32) {
        let ty = self.p.place_ty(place);
        let size = self.ty_size(ty);
        let address = match (ty, place) {
            (Ty::FixedStr(_), Place::Var(v)) => format!("{}->chr", self.name(*v)),
            (Ty::FixedStr(_), _) => format!("{}->chr", self.load_place(place)),
            (Ty::User(_), Place::Var(v)) => format!("(((char*){})+((0+0)))", self.name(*v)),
            (Ty::User(_), _) => self.bytes_of(place),
            (_, Place::Var(v)) => self.name(*v),
            (_, _) => self.place_ref(place),
        };
        (address, size)
    }

    /// A call by the table's `callname`. A built-in with optional slots gets `NULL` for each absent argument, as the
    /// old compiler writes it, and, last, the `passed` mask: bit n set when the n-th optional slot is present (`study\02` §4.3). An argument
    /// of an any-numeric slot is cast to its C type, so libqb's overload for it is chosen (`func_abs((int16)(…))`:
    /// an INTEGER literal is otherwise a C `int`).
    fn table_call(&mut self, id: BuiltinId, args: &[Option<Expr>], slots: &[Slot]) -> String {
        let b = id.get();
        let optional = b.optional.unwrap_or(&[]);
        let mut parts = Vec::new();
        let mut mask = 0u32;
        let mut bit = 0;
        for (i, a) in args.iter().enumerate() {
            let is_optional = optional.get(i).copied().unwrap_or(false);
            match a {
                Some(a) => {
                    if is_optional {
                        mask |= 1 << bit;
                    }
                    parts.push(match (slots.get(i), &a.kind) {
                        (Some(Slot::AnyNumeric), _) => format!("(({})({}))", c_type(a.ty), self.value(a)),
                        // A block: the address of its place.
                        (Some(Slot::Mem), ExprKind::Load(place)) => self.mem_ref(place),
                        (Some(Slot::Mem), _) => unreachable!("a block is a place (`validate`)"),
                        (
                            Some(
                                Slot::Long
                                | Slot::Integer
                                | Slot::Int64
                                | Slot::Double
                                | Slot::Float
                                | Slot::Str
                                | Slot::UInt64
                                | Slot::ULong
                                | Slot::ColorOrTile
                                | Slot::Block
                                | Slot::Bytes
                                | Slot::Offset,
                            )
                            | None,
                            _,
                        ) => self.value(a),
                    });
                }
                None => parts.push("NULL".into()),
            }
            if is_optional {
                bit += 1;
            }
        }
        if optional.iter().any(|&o| o) {
            parts.push(mask.to_string());
        }
        // The fix `bit-number-check` (design D7 of `libqb-fixes`): the bit number of `_READBIT` and its kin goes through
        // a check that raises error 5 outside 0 to 63 and gives the number back, so libqb's mask gives the value the
        // program gives with the fix off. With the fix off nothing is written.
        if self.p.settings.fixes.is_on(Fix::BitNumberCheck)
            && matches!(
                b.callname,
                "func__readbit" | "func__setbit" | "func__resetbit" | "func__togglebit"
            )
        {
            if !self.func_decls.iter().any(|d| d == BIT_NUMBER_CHECK) {
                self.func_decls.push(BIT_NUMBER_CHECK.to_string());
            }
            parts[1] = format!("qbr_bit_number({})", parts[1]);
        }
        // Every result of `_COTH` takes the path of `QB64PE-DIVERGENCES.md` D-031 (the copy of libqb's `func_coth` is the
        // hyperbolic cotangent): a wrapper in `global.txt`, written for a program that calls it, traces it
        // (`tools\callsite` reads it as `func_coth`).
        let callname = if b.callname == "func_coth" {
            if !self.func_decls.iter().any(|d| d == COTH_WRAPPER) {
                self.func_decls.push(COTH_WRAPPER.to_string());
            }
            "qbr_coth"
        } else {
            b.callname
        };
        format!("{callname}({})", parts.join(","))
    }
}

/// `_COTH` with the run-time trace of `QB64PE-DIVERGENCES.md` D-031 (`qbr_diverged`, design D1 of `m3-divergence-trace`):
/// every call takes the row's path.
const COTH_WRAPPER: &str = "static inline double qbr_coth(double qbr_x){qbr_diverged(31);return func_coth(qbr_x);}\n";

/// The bit number of `_READBIT`, `_SETBIT`, `_RESETBIT` and `_TOGGLEBIT` with the fix `bit-number-check` on.
const BIT_NUMBER_CHECK: &str =
    "static inline int64 qbr_bit_number(int64 qbr_n){if (qbr_n<0||qbr_n>63) error(5);return qbr_n;}\n";

/// The letters of libqb's `<t>2string` and `string2<t>` for a numeric type (`qb64pe.bas` 21230–21244).
fn bytes_letters(t: Ty) -> &'static str {
    match t {
        Ty::I8 => "b",
        Ty::U8 => "ub",
        Ty::I16 => "i",
        Ty::U16 => "ui",
        Ty::I32 => "l",
        Ty::U32 => "ul",
        Ty::I64 => "i64",
        Ty::U64 => "ui64",
        Ty::Off => "o",
        Ty::UOff => "uo",
        Ty::F32 => "s",
        Ty::F64 => "d",
        Ty::F80 => "f",
        Ty::Bit { .. } | Ty::Str | Ty::FixedStr(_) | Ty::User(_) => {
            unreachable!("the bytes of a {t:?} (`check::mem`)")
        }
    }
}
