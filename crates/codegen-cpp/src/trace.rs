//! A value-trace build (OpenSpec change `value-trace`, spec `compiler/value-trace`): a record call at each store
//! into a variable, and at exit one record of the values appended to the file `QB64RUST_TRACE` names.
//!
//! An entry is a scalar variable, an array (all its elements together) or a member path of a user type (`P.POS.X`,
//! `SHIPS().HP`); [`Entries::entry`] numbers them while the bodies are emitted (design D2). The record call stands
//! where the store is made and reads the value through the address the store used, so nothing is evaluated twice
//! (design D1). A store that writes many elements at once and brings new values (`b() = a()`, `GET #` into a
//! whole array) is recorded element by element after it ([`Emitter::trace_whole_array`]). The stores that are not
//! recorded by value are counted by statement: those that write zeros (`REDIM`, `ERASE`), the stores at an address
//! (`untraced` lines, design D5), and the arrays passed by reference (`alias` lines).
//!
//! The run-time part is `trace_rt.h`, C++ that goes into the program after the tables [`Entries::table`] writes
//! (design D3). The record's format is in `tools\trace\README.md` (design D6).

use crate::Emitter;
use crate::names::{c_string, c_type};
use qb64rust_ir::{
    AllocMode, Arg, ArrayRef, Expr, ExprKind, MemberId, Op, Place, ProcId, Program, Storage, Store, Target, Ty, VarId,
    numeric_types,
};
use std::collections::HashMap;
use std::fmt::Write as _;

/// What a value-trace build needs to know about the program's files.
pub struct Trace {
    /// Each file of the program by its index (`FileId`): its path relative to the main file's folder and the
    /// FNV-1a 64 hash of its bytes, as 16 hex digits. The same as a coverage build's.
    pub files: Vec<(String, String)>,
}

/// The run-time part, written after the tables.
const RUNTIME: &str = include_str!("trace_rt.h");

/// How the run-time part keeps and writes the values of an entry (`QBR_TRC_INT` and the others of `trace_rt.h`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Int,
    UInt,
    F32,
    F64,
    F80,
    Str,
}

impl Kind {
    fn of(ty: Ty) -> Kind {
        match ty {
            Ty::F32 => Kind::F32,
            Ty::F64 => Kind::F64,
            Ty::F80 => Kind::F80,
            Ty::Str | Ty::FixedStr(_) => Kind::Str,
            Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::Off => Kind::Int,
            Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::UOff => Kind::UInt,
            Ty::Bit { signed: true, .. } => Kind::Int,
            Ty::Bit { signed: false, .. } => Kind::UInt,
            Ty::User(_) => unreachable!("an entry is a number or a string, not a user type"),
        }
    }

    /// The number of the kind in `qbr_trc_kinds`.
    fn number(self) -> u8 {
        match self {
            Kind::Int => 0,
            Kind::UInt => 1,
            Kind::F32 => 2,
            Kind::F64 => 3,
            Kind::F80 => 4,
            Kind::Str => 5,
        }
    }

    /// What follows `qbr_trc_` (a store) or `qbr_trc_a` (after a call) in the record call of a value.
    fn call(self) -> &'static str {
        match self {
            Kind::Int => "i",
            Kind::UInt => "u",
            Kind::F32 => "f32",
            Kind::F64 => "f64",
            Kind::F80 => "f80",
            Kind::Str => "s",
        }
    }
}

/// How a record call reads the value stored.
#[derive(Clone, Copy)]
pub(crate) enum Read<'s> {
    /// A C expression of the value: a number, or the `qbs*` of a string.
    Value(&'s str),
    /// A C expression of the address of the value's bytes: a number's, the slot of a `STRING`'s `qbs*`, the
    /// characters of a `STRING * n`.
    Addr(&'s str),
}

/// What a built-in is given for a place, held in a temporary so that the place is read through it after the call.
#[derive(Clone, Copy)]
pub(crate) enum Held {
    /// The `qbs*` of a string.
    Qbs,
    /// A pointer to a number.
    Ptr,
    /// The address of the place's bytes.
    Bytes,
}

/// What a member array inside a whole value gets ([`Emitter::trace_members`]).
#[derive(Clone, Copy)]
pub(crate) enum Inside<'s> {
    /// The value was stored into: each element of a member array is recorded. A `_DYNAMIC` member array and a
    /// member of type `_MEM` get an `untraced` line that says `what` stored.
    Stored(&'s str),
    /// The value was passed by reference to a parameter: an `alias` line to the parameter's member array.
    Passed { callee: ProcId, param: VarId },
}

struct Entry {
    name: String,
    ty: String,
    procedure: Option<String>,
    kind: Kind,
}

/// The entries, sites, aliases and untraced stores numbered while the bodies are emitted.
pub(crate) struct Entries<'a> {
    trace: &'a Trace,
    entries: Vec<Entry>,
    index: HashMap<(VarId, Vec<MemberId>), usize>,
    /// A site is a file and a line.
    sites: Vec<(u32, u32)>,
    site_index: HashMap<(u32, u32), usize>,
    /// Each with its site; the text is the rest of its line of the record.
    aliases: Vec<(usize, String)>,
    untraced: Vec<(usize, String)>,
    /// The lines to write after the operation being emitted: the reads and counters of the calls in its values.
    pub(crate) after: Vec<String>,
    /// The procedure whose body is being emitted; `None` in the main module.
    pub(crate) proc: Option<ProcId>,
    /// The strings the program's `FIELD` statements name ([`Emitter::trace_fields`]).
    fields: Vec<Place>,
}

/// The variable a place is, or is part of, and the members on the way to the place. An index is left out: all the
/// elements of an array are one entry.
pub(crate) fn path(place: &Place) -> (VarId, Vec<MemberId>) {
    match place {
        Place::Var(v) => (*v, Vec::new()),
        Place::Element { array, .. } => array_path(array),
        Place::Member { base, member } => {
            let (root, mut members) = path(base);
            members.push(*member);
            (root, members)
        }
    }
}

/// [`path`] of an array.
pub(crate) fn array_path(array: &ArrayRef) -> (VarId, Vec<MemberId>) {
    match array {
        ArrayRef::Var(v) => (*v, Vec::new()),
        ArrayRef::Member { base, member } => {
            let (root, mut members) = path(base);
            members.push(*member);
            (root, members)
        }
    }
}

/// The name of a path as the record gives it: the variable's name, `()` after an array's, then each member's
/// after a dot (`P.POS.X`, `SHIPS().HP`, `T.V()`). Upper case, without a type suffix.
pub(crate) fn path_name(p: &Program, root: VarId, members: &[MemberId]) -> String {
    let v = p.var(root);
    let mut name = v.name.clone();
    if v.is_array() {
        name.push_str("()");
    }
    let mut ty = v.ty;
    for &m in members {
        let member = p.member(ty, m);
        name.push('.');
        name.push_str(&member.name);
        if member.is_array() || member.dynamic {
            name.push_str("()");
        }
        ty = member.ty;
    }
    name
}

/// The type at the end of a path: the variable's (an array's element type), or the last member's.
fn path_ty(p: &Program, root: VarId, members: &[MemberId]) -> Ty {
    members.iter().fold(p.var(root).ty, |ty, &m| p.member(ty, m).ty)
}

/// The procedure a variable belongs to; `None` for one of the main module.
fn procedure_of(p: &Program, v: VarId) -> Option<String> {
    match p.var(v).storage {
        Storage::Global | Storage::Temp(None) => None,
        Storage::Static(q) | Storage::Local(q) | Storage::Param(q) | Storage::Result(q) | Storage::Temp(Some(q)) => {
            Some(p.proc(q).name.clone())
        }
    }
}

/// A variable the lowering made (the limits of a `FOR`): it has no entry.
fn is_temp(p: &Program, v: VarId) -> bool {
    matches!(p.var(v).storage, Storage::Temp(_))
}

impl<'a> Entries<'a> {
    pub(crate) fn new(trace: &'a Trace, p: &Program) -> Entries<'a> {
        // Each place a `FIELD` of the program names, once.
        let mut fields: Vec<Place> = Vec::new();
        let bodies = std::iter::once(&p.main).chain(p.procs.iter().map(|q| &q.body));
        for op in bodies.flat_map(|b| &b.stmts).flat_map(|s| &s.ops) {
            if let Op::Field { parts, .. } = op {
                for (place, _) in parts {
                    if !fields.contains(place) {
                        fields.push(place.clone());
                    }
                }
            }
        }
        Entries {
            trace,
            entries: Vec::new(),
            index: HashMap::new(),
            sites: Vec::new(),
            site_index: HashMap::new(),
            aliases: Vec::new(),
            untraced: Vec::new(),
            after: Vec::new(),
            proc: None,
            fields,
        }
    }

    /// The entry of a place: its variable's, its array's or its member path's. `None` for a place of a variable the
    /// lowering made.
    pub(crate) fn entry(&mut self, p: &Program, place: &Place) -> Option<usize> {
        let (root, members) = path(place);
        self.entry_of(p, root, members)
    }

    /// The entry of a path ([`path`]).
    pub(crate) fn entry_of(&mut self, p: &Program, root: VarId, members: Vec<MemberId>) -> Option<usize> {
        if is_temp(p, root) {
            return None;
        }
        if let Some(&k) = self.index.get(&(root, members.clone())) {
            return Some(k);
        }
        let ty = path_ty(p, root, &members);
        self.entries.push(Entry {
            name: path_name(p, root, &members),
            ty: crate::debug::type_name(p, ty),
            procedure: procedure_of(p, root),
            kind: Kind::of(ty),
        });
        self.index.insert((root, members), self.entries.len() - 1);
        Some(self.entries.len() - 1)
    }

    /// The number of a file and line.
    pub(crate) fn site(&mut self, file: u32, line: u32) -> usize {
        let next = self.sites.len();
        let k = *self.site_index.entry((file, line)).or_insert(next);
        if k == next {
            self.sites.push((file, line));
        }
        k
    }

    /// The counter of the line `text` at `site`, of `list`: one per site and text.
    fn counter(list: &mut Vec<(usize, String)>, site: usize, text: String) -> usize {
        match list.iter().position(|(s, t)| *s == site && *t == text) {
            Some(k) => k,
            None => {
                list.push((site, text));
                list.len() - 1
            }
        }
    }

    /// The line that counts a statement that passes the array `name` of `procedure`, or an element of it, to
    /// `parameter` of `callee`.
    pub(crate) fn alias(
        &mut self,
        site: usize,
        (name, procedure): (&str, Option<&str>),
        (callee, parameter): (&str, &str),
    ) -> String {
        let text = format!("{name}\t{}\t{callee}\t{parameter}", procedure.unwrap_or("-"));
        format!("qbr_trc_al[{}]++;", Self::counter(&mut self.aliases, site, text))
    }

    /// The line that counts a statement that stores without a record of the values: `what` it was, and the array
    /// or variable it stores into when it has one, by its name and its procedure.
    pub(crate) fn untraced(&mut self, site: usize, what: &str, into: Option<(&str, Option<&str>)>) -> String {
        let (name, procedure) = into.unwrap_or(("-", None));
        let text = format!("{what}\t{name}\t{}", procedure.unwrap_or("-"));
        format!("qbr_trc_un[{}]++;", Self::counter(&mut self.untraced, site, text))
    }

    /// The tables of the entries, sites, files, aliases and untraced stores, then the run-time part.
    pub(crate) fn table(&self) -> String {
        // An empty C array is not allowed: a table with nothing in it gets one unused element.
        let strings = |items: Vec<String>| {
            let mut items: Vec<String> = items.iter().map(|s| c_string(s.as_bytes())).collect();
            if items.is_empty() {
                items.push(c_string(b""));
            }
            items.join(",")
        };
        let numbers = |items: Vec<String>| {
            if items.is_empty() {
                "0".to_string()
            } else {
                items.join(",")
            }
        };
        let at = |list: &[(usize, String)]| {
            numbers(
                list.iter()
                    .flat_map(|(site, _)| {
                        let (file, line) = self.sites[*site];
                        [file.to_string(), line.to_string()]
                    })
                    .collect(),
            )
        };
        let mut out = String::new();
        writeln!(
            out,
            "#define QBR_TRC_ENTRIES {}\n#define QBR_TRC_FILES {}\n#define QBR_TRC_ALIASES {}\n\
             #define QBR_TRC_UNTRACED {}",
            self.entries.len(),
            self.trace.files.len(),
            self.aliases.len(),
            self.untraced.len()
        )
        .unwrap();
        // An entry's name, type and procedure, as the record's `entry` line gives them.
        let names = self
            .entries
            .iter()
            .map(|e| format!("{}\t{}\t{}", e.name, e.ty, e.procedure.as_deref().unwrap_or("-")))
            .collect();
        writeln!(out, "static const char *const qbr_trc_names[]={{{}}};", strings(names)).unwrap();
        let kinds = self.entries.iter().map(|e| e.kind.number().to_string()).collect();
        writeln!(out, "static const uint8 qbr_trc_kinds[]={{{}}};", numbers(kinds)).unwrap();
        let sites = self
            .sites
            .iter()
            .flat_map(|&(file, line)| [file.to_string(), line.to_string()])
            .collect();
        writeln!(out, "static const uint32 qbr_trc_sites[]={{{}}};", numbers(sites)).unwrap();
        let files = self
            .trace
            .files
            .iter()
            .map(|(path, hash)| format!("{hash}\t{path}"))
            .collect();
        writeln!(out, "static const char *const qbr_trc_files[]={{{}}};", strings(files)).unwrap();
        for (name, list) in [("alias", &self.aliases), ("untraced", &self.untraced)] {
            let texts = list.iter().map(|(_, text)| text.clone()).collect();
            writeln!(out, "static const char *const qbr_trc_{name}[]={{{}}};", strings(texts)).unwrap();
            writeln!(out, "static const uint32 qbr_trc_{name}_at[]={{{}}};", at(list)).unwrap();
        }
        writeln!(
            out,
            "static uint64 qbr_trc_al[{}];\nstatic uint64 qbr_trc_un[{}];",
            self.aliases.len().max(1),
            self.untraced.len().max(1)
        )
        .unwrap();
        out.push_str(RUNTIME);
        out
    }
}

/// The record call of entry `entry`, of type `ty`, at `site`: a store, or (`after`) a read after a call.
fn record(entry: usize, ty: Ty, read: Read, site: usize, after: bool) -> String {
    let a = if after { "a" } else { "" };
    let kind = Kind::of(ty).call();
    match (read, ty) {
        (Read::Value(v), _) => format!("qbr_trc_{a}{kind}({entry},{v},{site});"),
        (Read::Addr(at), Ty::FixedStr(k)) => format!("qbr_trc_{a}b({entry},{at},{k},{site});"),
        (Read::Addr(at), Ty::Str) => format!("qbr_trc_{a}s({entry},*(qbs**)({at}),{site});"),
        (Read::Addr(at), numeric_types!()) => format!("qbr_trc_{a}{kind}({entry},*({}*)({at}),{site});", c_type(ty)),
        (Read::Addr(_), Ty::User(_)) => unreachable!("an entry is a number or a string, not a user type"),
    }
}

impl Emitter<'_> {
    /// The site of the statement being emitted.
    fn trace_site(&mut self) -> usize {
        let (file, line, _) = self.stmt_at;
        let t = self.trace.as_mut().expect("a value-trace build");
        t.site(file.map_or(0, |f| f.0), line)
    }

    /// The record call of a store into `place`, read as `read` says; `None` in a build without the option, and for
    /// a place of a variable the lowering made.
    pub(crate) fn trace_store(&mut self, place: &Place, read: Read) -> Option<String> {
        let p = self.p;
        let entry = self.trace.as_mut()?.entry(p, place)?;
        let site = self.trace_site();
        Some(record(entry, p.place_ty(place), read, site, false))
    }

    /// [`Self::trace_store`] of an element of an array variable at the flat position `flat`, read where the store
    /// wrote it.
    pub(crate) fn trace_element(&mut self, array: VarId, flat: &str) -> Option<String> {
        self.trace.as_ref()?;
        let place = Place::Element {
            array: ArrayRef::Var(array),
            index: Vec::new(),
        };
        let name = self.name(array);
        match self.p.var(array).ty {
            Ty::FixedStr(k) => {
                let at = format!("&((uint8*)({name}[0]))[({flat})*{k}]");
                self.trace_store(&place, Read::Addr(&at))
            }
            // A `_BIT` element is read with `getbits`, as `element_at` writes it.
            numeric_types!() | Ty::Str => {
                let element = self.element_at(array, flat);
                self.trace_store(&place, Read::Value(&element))
            }
            Ty::User(_) => None,
        }
    }

    /// A new temporary of C type `c_ty`, declared with the variables of the body: it holds what a built-in is
    /// given for a place.
    pub(crate) fn trace_temp(&mut self, c_ty: &str) -> String {
        self.pass += 1;
        let name = format!("qbr_trc_p{}", self.pass);
        self.pass_decls.push(format!("{c_ty}{name};\n"));
        name
    }

    /// What a built-in is given for `place`, the C expression `given`, and the lines that record the place after
    /// the call (design D1). In a value-trace build the expression is held in a temporary where it is passed, so
    /// that the place is read through it and nothing is evaluated twice. The caller writes the lines under `if
    /// (!is_error_pending())`. In any other build, and for a place with no entry, `given` as it is and no line.
    pub(crate) fn trace_held(&mut self, place: &Place, held: Held, given: String, what: &str) -> (String, Vec<String>) {
        let p = self.p;
        let ty = p.place_ty(place);
        if self.trace.is_none() || is_temp(p, place.root()) {
            return (given, Vec::new());
        }
        match (held, ty) {
            (Held::Qbs, _) => {
                let t = self.trace_temp("qbs *");
                let lines = self.trace_store(place, Read::Value(&t));
                (format!("({t}={given})"), lines.into_iter().collect())
            }
            (Held::Ptr, _) => {
                let t = self.trace_temp(&format!("{} *", c_type(ty)));
                let lines = self.trace_store(place, Read::Value(&format!("*{t}")));
                (format!("({t}={given})"), lines.into_iter().collect())
            }
            // The characters of a variable-length string are not its place: the string is read again by its name
            // when no index is on the way.
            (Held::Bytes, Ty::Str) => {
                let lines = if place.has_element() {
                    None
                } else {
                    let s = self.place_ref(place);
                    self.trace_store(place, Read::Value(&s))
                };
                (given, lines.into_iter().collect())
            }
            (Held::Bytes, Ty::User(_)) => {
                let t = self.trace_temp("char *");
                let (root, members) = path(place);
                let lines = self.trace_members((root, members), ty, &t, false, Inside::Stored(what));
                (format!("({t}=(char*)({given}))"), lines)
            }
            (Held::Bytes, numeric_types!() | Ty::FixedStr(_)) => {
                let t = self.trace_temp("char *");
                let lines = self.trace_store(place, Read::Addr(&t));
                (format!("({t}=(char*)({given}))"), lines.into_iter().collect())
            }
        }
    }

    /// The lines that record a whole value of user type `ty` at the address `at` (a `char*` expression that
    /// evaluates nothing), the value of the path `to`: each number and string of it, as a store or (`after`) as a
    /// read after a call. A member array inside gets what `inside` says. The counters come first, once each.
    pub(crate) fn trace_members(
        &mut self,
        to: (VarId, Vec<MemberId>),
        ty: Ty,
        at: &str,
        after: bool,
        inside: Inside,
    ) -> Vec<String> {
        let (root, on_the_way) = to;
        let (mut once, mut lines) = (Vec::new(), Vec::new());
        self.trace_value(
            (root, &on_the_way, on_the_way.len()),
            ty,
            (at, 0),
            (after, inside),
            0,
            (&mut lines, &mut once),
        );
        once.extend(lines);
        once
    }

    /// [`Self::trace_members`] of the value of type `ty` at `at` plus `base` bytes, on the path `on_the_way` from
    /// `root`, of which the first `top` members lead to the whole value. The record calls go to `lines`, inside
    /// `depth` loops over member arrays; the counters, which count the statement once, go to `once`.
    fn trace_value(
        &mut self,
        (root, on_the_way, top): (VarId, &[MemberId], usize),
        ty: Ty,
        (at, base): (&str, u32),
        (after, inside): (bool, Inside),
        depth: usize,
        (lines, once): (&mut Vec<String>, &mut Vec<String>),
    ) {
        let p = self.p;
        let Ty::User(id) = ty else {
            unreachable!("the members of {ty:?}");
        };
        let site = self.trace_site();
        let (offsets, _) = self.layout(id);
        for (i, (m, offset)) in p.user_type(id).members.iter().zip(offsets).enumerate() {
            let mut members = on_the_way.to_vec();
            members.push(MemberId(qb64rust_base::to_u32(i)));
            let offset = base + offset;
            let stored = match inside {
                Inside::Stored(what) => Some(what),
                Inside::Passed { .. } => None,
            };
            if m.dynamic || p.is_mem(m.ty) || (m.is_array() && stored.is_none()) {
                // Not read: a counter of the statement.
                let name = path_name(p, root, &members);
                let procedure = procedure_of(p, root);
                let t = self.trace.as_mut().expect("a value-trace build");
                let line = match inside {
                    Inside::Stored(what) => t.untraced(site, what, Some((&name, procedure.as_deref()))),
                    Inside::Passed { callee, param } => {
                        // The same members of the parameter.
                        let parameter = path_name(p, param, &members[top..]);
                        t.alias(site, (&name, procedure.as_deref()), (&p.proc(callee).name, &parameter))
                    }
                };
                if !once.contains(&line) {
                    once.push(line);
                }
            } else if m.is_array() {
                // Each element of a member array, which lies in the value.
                let index = format!("qbr_trc_i{depth}");
                let element = format!("{at}+{offset}+{index}*{}", self.ty_size(m.ty));
                lines.push(format!("for(ptrszint {index}=0;{index}<{};{index}++){{", m.count()));
                if let Ty::User(_) = m.ty {
                    self.trace_value(
                        (root, &members, top),
                        m.ty,
                        (&element, 0),
                        (after, inside),
                        depth + 1,
                        (lines, once),
                    );
                } else if let Some(entry) = self.trace.as_mut().and_then(|t| t.entry_of(p, root, members)) {
                    lines.push(record(entry, m.ty, Read::Addr(&element), site, after));
                }
                lines.push("}".into());
            } else if let Ty::User(_) = m.ty {
                self.trace_value(
                    (root, &members, top),
                    m.ty,
                    (at, offset),
                    (after, inside),
                    depth,
                    (lines, once),
                );
            } else if let Some(entry) = self.trace.as_mut().and_then(|t| t.entry_of(p, root, members)) {
                let addr = format!("{at}+{offset}");
                lines.push(record(entry, m.ty, Read::Addr(&addr), site, after));
            }
        }
    }

    /// The record calls of the element at position `index` (a C expression) of the block `block` of an array of
    /// `ty`, the array of the path `to`. `depth` loops stand around them.
    fn trace_block_element(
        &mut self,
        (root, members): (VarId, &[MemberId]),
        ty: Ty,
        (block, index): (&str, &str),
        what: &str,
        depth: usize,
        (lines, once): (&mut Vec<String>, &mut Vec<String>),
    ) {
        let p = self.p;
        let site = self.trace_site();
        // How far apart the elements are: the type's size, but for `_FLOAT` elements of an array variable. Those are
        // `long double`s side by side (`element_at`), 16 bytes apart, in a block of 32 bytes each. A `_FLOAT` member
        // array inside a user type has its elements 32 bytes apart, as the type's layout places them.
        let apart = if ty == Ty::F80 && members.is_empty() {
            "sizeof(long double)".to_string()
        } else {
            self.ty_size(ty).to_string()
        };
        let at = format!("((char*)({block}))+({index})*{apart}");
        if let Ty::User(_) = ty {
            let inside = (false, Inside::Stored(what));
            self.trace_value(
                (root, members, members.len()),
                ty,
                (&at, 0),
                inside,
                depth,
                (lines, once),
            );
            return;
        }
        let Some(entry) = self.trace.as_mut().and_then(|t| t.entry_of(p, root, members.to_vec())) else {
            return;
        };
        // A `_BIT` element is packed: read with `getbits`, as `element_at` reads it.
        if let Ty::Bit { width, signed } = ty {
            let get = if signed { "getbits" } else { "getubits" };
            let value = format!("(({})({get}({width},(uint8*)({block}),{index})))", c_type(ty));
            lines.push(record(entry, ty, Read::Value(&value), site, false));
        } else {
            lines.push(record(entry, ty, Read::Addr(&at), site, false));
        }
    }

    /// The text that records `count` elements (a C expression) of the block `block` of the array of the path `to`,
    /// from the flat position `first`, under the test that no error is pending. Each element is a store. `None`
    /// in a build without the option, and for an array that has no entries: one of `_MEM` values, or of a variable
    /// the lowering made.
    pub(crate) fn trace_elements(
        &mut self,
        to: (VarId, Vec<MemberId>),
        ty: Ty,
        (block, first, count): (&str, &str, &str),
        what: &str,
    ) -> Option<String> {
        let records = self.trace_element_loop(to, ty, (block, first, count), what)?;
        Some(format!("if (!is_error_pending()){{{records}}}"))
    }

    /// [`Self::trace_elements`] without the test for a pending error, for a store that is made with one pending.
    pub(crate) fn trace_element_loop(
        &mut self,
        to: (VarId, Vec<MemberId>),
        ty: Ty,
        (block, first, count): (&str, &str, &str),
        what: &str,
    ) -> Option<String> {
        let (root, members) = to;
        if self.trace.is_none() || is_temp(self.p, root) || self.p.is_mem(ty) {
            return None;
        }
        let (mut once, mut lines) = (Vec::new(), Vec::new());
        let index = format!("({first})+qbr_trc_e");
        self.trace_block_element((root, &members), ty, (block, &index), what, 0, (&mut lines, &mut once));
        Some(format!(
            "{}for(ptrszint qbr_trc_e=0;qbr_trc_e<(ptrszint)({count});qbr_trc_e++){{{}}}",
            once.concat(),
            lines.concat()
        ))
    }

    /// The text that follows a statement that stores into every element of `array` (`GET #` into a whole array,
    /// `_MEMGET` into one): each element recorded as a store, read by the array's own name, which evaluates
    /// nothing. An array that cannot be read so is counted as a bulk store of `what`: a member array reached
    /// through an index, a `_DYNAMIC` member array, an array of `_MEM` values. Empty in a build without the option.
    pub(crate) fn trace_whole_array(&mut self, array: &ArrayRef, what: &str) -> String {
        if self.trace.is_none() {
            return String::new();
        }
        let ty = self.p.array_ty(array);
        let to = array_path(array);
        let read = match array {
            ArrayRef::Var(v) => {
                let name = self.name(*v);
                // No element while the array is not allocated.
                let count = self.element_count(self.p.var(*v), &name);
                Some((format!("{name}[0]"), count))
            }
            ArrayRef::Member { base, member }
                if !base.has_element() && self.dynamic_member(base, *member).is_none() =>
            {
                let count = self.p.member_of(base, *member).count().to_string();
                Some((self.member_array_at(base, *member), count))
            }
            ArrayRef::Member { .. } => None,
        };
        let text = read.and_then(|(block, count)| self.trace_elements(to.clone(), ty, (&block, "0", &count), what));
        text.unwrap_or_else(|| self.trace_untraced(what, Some((to.0, &to.1))).unwrap_or_default())
    }

    /// The call `call` of a built-in function with the arguments `args`, of result type `ty`. `_UCHARPOS` fills the
    /// array it is given and returns how many positions the text has. A value-trace build holds that number and
    /// records as many elements after the operation, as stores. An array given from an element on is counted as
    /// a bulk store. Any other call, and any call in another build, is `call` as it is.
    pub(crate) fn trace_filled(&mut self, args: &[Option<Expr>], ty: Ty, call: String) -> String {
        let filled = args.iter().flatten().find_map(|a| {
            if let ExprKind::Block { array, index } = &a.kind {
                Some((*array, index.is_empty()))
            } else {
                None
            }
        });
        let (Some((array, whole)), true) = (filled, self.trace.is_some()) else {
            return call;
        };
        let v = self.p.var(array);
        let name = self.name(array);
        let count = self.element_count(v, &name);
        let records = if whole {
            // The number starts at 0 and is put back to 0 once it is used: a call in a part of a value that is not
            // evaluated (`_IIF`, `_ANDALSO`) then records nothing.
            self.pass += 1;
            let held = format!("qbr_trc_p{}", self.pass);
            self.pass_decls.push(format!("{} {held}=0;\n", c_type(ty)));
            let count = format!("(({held})<({count})?({held}):({count}))");
            let block = format!("{name}[0]");
            self.trace_elements((array, Vec::new()), v.ty, (&block, "0", &count), "_UCHARPOS")
                .map(|records| (format!("({held}={call})"), format!("{records}{held}=0;")))
        } else {
            None
        };
        match records {
            Some((call, records)) => {
                self.trace.as_mut().expect("a value-trace build").after.push(records);
                call
            }
            None => {
                self.trace_untraced_after("_UCHARPOS", Some((array, &[])));
                call
            }
        }
    }

    /// Whether the C++ of the body being emitted can name the variable: one that lives as long as the program, or
    /// one of the body's own procedure.
    fn trace_in_reach(&self, v: VarId) -> bool {
        let here = self.trace.as_ref().and_then(|t| t.proc);
        match self.p.var(v).storage {
            Storage::Global | Storage::Static(_) => true,
            Storage::Local(q) | Storage::Param(q) | Storage::Result(q) => Some(q) == here,
            Storage::Temp(_) => false,
        }
    }

    /// The text that follows a statement that changes the strings of a file's record buffer (`FIELD`, `GET #`
    /// with no variable): each string a `FIELD` of the program names is read again, and recorded when it changed,
    /// as a variable is after a call. Only a string that the body can name and that no index leads to is read. When
    /// the program has another, the statement is also counted as a store at an address, `what`. Empty in a build
    /// without the option.
    pub(crate) fn trace_fields(&mut self, what: &str) -> String {
        let p = self.p;
        let Some(t) = &self.trace else {
            return String::new();
        };
        let fields = t.fields.clone();
        let site = self.trace_site();
        let (mut records, mut unread) = (Vec::new(), false);
        for place in &fields {
            if place.has_element() || !self.trace_in_reach(place.root()) {
                unread = true;
                continue;
            }
            let read = match place {
                Place::Var(v) => self.scalar(*v),
                Place::Element { .. } | Place::Member { .. } => self.bytes_of(place),
            };
            let read = match place {
                Place::Var(_) => Read::Value(&read),
                Place::Element { .. } | Place::Member { .. } => Read::Addr(&read),
            };
            let t = self.trace.as_mut().expect("a value-trace build");
            if let Some(entry) = t.entry(p, place) {
                records.push(record(entry, p.place_ty(place), read, site, true));
            }
        }
        let counted = if unread {
            self.trace_untraced(what, None).unwrap_or_default()
        } else {
            String::new()
        };
        format!("{counted}{}", guarded(&records))
    }

    /// After a call of `proc` with `args` (an [`Op::Call`] or a call in a value), the lines for its arguments
    /// passed by reference (design D1), added to the lines written after the operation. A variable, or a member
    /// with no index on the way, is read again and recorded when its value changed. An element or a whole array
    /// gets an `alias` line to the parameter, whose entry holds the values stored through it.
    pub(crate) fn trace_call(&mut self, proc: ProcId, args: &[Arg]) {
        if self.trace.is_none() {
            return;
        }
        let p = self.p;
        let callee = p.proc(proc);
        let site = self.trace_site();
        let mut lines = Vec::new();
        for (arg, &param) in args.iter().zip(&callee.params) {
            let (root, members) = match arg {
                Arg::Ref(place) => path(place),
                Arg::Array(array) => array_path(array),
                Arg::Temp(..) | Arg::Value(_) | Arg::Null => continue,
            };
            if is_temp(p, root) {
                continue;
            }
            match arg {
                Arg::Ref(place) if !place.has_element() => {
                    let ty = p.place_ty(place);
                    if p.is_mem(ty) {
                        continue;
                    }
                    if let Ty::User(_) = ty {
                        let at = self.bytes_of(place);
                        let inside = Inside::Passed { callee: proc, param };
                        lines.extend(self.trace_members((root, members), ty, &at, true, inside));
                        continue;
                    }
                    let read = match place {
                        Place::Var(v) => self.scalar(*v),
                        Place::Element { .. } | Place::Member { .. } => self.bytes_of(place),
                    };
                    let read = match place {
                        Place::Var(_) => Read::Value(&read),
                        Place::Element { .. } | Place::Member { .. } => Read::Addr(&read),
                    };
                    let t = self.trace.as_mut().expect("a value-trace build");
                    if let Some(entry) = t.entry_of(p, root, members) {
                        lines.push(record(entry, ty, read, site, true));
                    }
                }
                Arg::Ref(_) | Arg::Array(_) => {
                    let name = path_name(p, root, &members);
                    let procedure = procedure_of(p, root);
                    let parameter = path_name(p, param, &[]);
                    let t = self.trace.as_mut().expect("a value-trace build");
                    lines.push(t.alias(site, (&name, procedure.as_deref()), (&callee.name, &parameter)));
                }
                Arg::Temp(..) | Arg::Value(_) | Arg::Null => {}
            }
        }
        self.trace.as_mut().expect("a value-trace build").after.extend(lines);
    }

    /// The line that counts a store without a record of the values (design D5): `what` it was, and the path
    /// ([`path`]) of the array or variable it stores into when it has one. `None` in a build without the option.
    pub(crate) fn trace_untraced(&mut self, what: &str, into: Option<(VarId, &[MemberId])>) -> Option<String> {
        self.trace.as_ref()?;
        let p = self.p;
        let site = self.trace_site();
        let into = into.map(|(root, members)| (path_name(p, root, members), procedure_of(p, root)));
        let into = into
            .as_ref()
            .map(|(name, procedure)| (name.as_str(), procedure.as_deref()));
        Some(self.trace.as_mut()?.untraced(site, what, into))
    }

    /// [`Self::trace_untraced`] for a store made inside a value (a call of a `DECLARE LIBRARY` function,
    /// `_UCHARPOS`): counted after the operation the value is part of.
    pub(crate) fn trace_untraced_after(&mut self, what: &str, into: Option<(VarId, &[MemberId])>) {
        if let Some(line) = self.trace_untraced(what, into) {
            self.trace.as_mut().expect("a value-trace build").after.push(line);
        }
    }

    /// The lines that count the bulk stores and the stores at an address of `op`, by what [`Op::stores`] says it
    /// stores. They are written before the operation.
    pub(crate) fn trace_untraced_op(&mut self, op: &Op, out: &mut Vec<String>) {
        if self.trace.is_none() {
            return;
        }
        let p = self.p;
        // These write their own lines where they store: the elements recorded, or a counter when they cannot be read.
        let own = matches!(
            op,
            Op::CopyArray { .. } | Op::ArrayCopy { .. } | Op::MemGet { .. } | Op::Field { .. }
        ) || matches!(op, Op::Builtin { id, .. } if id.get().callname == "sub_get");
        if own {
            return;
        }
        for store in op.stores(p) {
            let into = match &store {
                Store::Bulk(Target::Var(v)) => Some((*v, Vec::new())),
                Store::Bulk(Target::Array(array)) => Some(array_path(array)),
                Store::Bulk(Target::Place(place)) => Some(path(place)),
                Store::Address => None,
                Store::ByName(_) | Store::Whole(_) | Store::Call { .. } => continue,
            };
            let what = untraced_what(p, op);
            let into = into.as_ref().map(|(root, members)| (*root, members.as_slice()));
            out.extend(self.trace_untraced(&what, into));
        }
    }

    /// Whether `op` stores into a place that has an entry, by name or as a whole value: its C++ then holds a record
    /// call or a counter (checked where the operation is emitted).
    pub(crate) fn trace_expected(&self, op: &Op) -> bool {
        let p = self.p;
        self.trace.is_some()
            && op.stores(p).iter().any(|store| match store {
                Store::ByName(Target::Var(v)) => !is_temp(p, *v),
                Store::ByName(Target::Place(place)) => {
                    // `_MEMGET` into the characters of a string reached through an index is not read back
                    // ([`Self::trace_held`]).
                    let unread = matches!(op, Op::MemGet { .. }) && p.place_ty(place) == Ty::Str && place.has_element();
                    !is_temp(p, place.root()) && !unread
                }
                Store::ByName(Target::Array(_)) => false,
                // A variable the lowering made has no entry, as for a store by name.
                Store::Whole(place) => !is_temp(p, place.root()),
                Store::Call { .. } | Store::Bulk(_) | Store::Address => false,
            })
    }
}

/// What a statement that stores without a record of the values was, for its `untraced` line.
fn untraced_what(p: &Program, op: &Op) -> String {
    let array_name = |array: &ArrayRef| {
        let (root, members) = array_path(array);
        path_name(p, root, &members)
    };
    match op {
        Op::Allocate {
            mode: AllocMode::Redim | AllocMode::Preserve | AllocMode::Retain,
            ..
        } => "REDIM".into(),
        // Also an array a statement of items makes before one of its parts.
        Op::Allocate {
            mode: AllocMode::Dim, ..
        }
        | Op::Print { .. }
        | Op::PrintUsing { .. }
        | Op::Write { .. }
        | Op::Input { .. }
        | Op::Read { .. } => "DIM".into(),
        Op::Clear { .. } | Op::EraseParam { .. } => "ERASE".into(),
        Op::CopyArray { from, to } => format!("{} = {}", array_name(to), array_name(from)),
        Op::ArrayCopy { .. } => "_ARRAYCOPY".into(),
        Op::MemGet { .. } => "_MEMGET".into(),
        Op::MemPut { .. } => "_MEMPUT".into(),
        Op::MemFill { .. } => "_MEMFILL".into(),
        Op::CallAbsolute { .. } => "CALL ABSOLUTE".into(),
        Op::Run(_) => "RUN".into(),
        Op::Field { .. } => "FIELD".into(),
        Op::Assign { .. } => "whole value".into(),
        Op::Call { proc, .. } => library_what(p, *proc),
        Op::Builtin { id, .. } => {
            let b = id.get();
            let name = b.name.to_ascii_uppercase();
            if b.callname == "sub_get" {
                format!("{name} #")
            } else {
                name
            }
        }
        Op::SelectConsole
        | Op::CopyWhole { .. }
        | Op::Release { .. }
        | Op::Lset { .. }
        | Op::End(_)
        | Op::System(_)
        | Op::Stop
        | Op::Exit
        | Op::SetHandler(_)
        | Op::NewHandler(_)
        | Op::LastHandler
        | Op::PaletteUsing { .. }
        | Op::Raise(_)
        | Op::Log { .. }
        | Op::Resume(_)
        | Op::Jump(_)
        | Op::Branch { .. }
        | Op::AssignAll(_)
        | Op::Gosub(_)
        | Op::Return(_)
        | Op::OnEvent { .. }
        | Op::Key(_)
        | Op::Restore { .. } => unreachable!("an operation with no bulk store and no store at an address"),
    }
}

/// The record calls of the places a built-in was given, under the test that its call left no error pending
/// (design D1); empty for none.
pub(crate) fn guarded(records: &[String]) -> String {
    if records.is_empty() {
        String::new()
    } else {
        format!("if (!is_error_pending()){{{}}}", records.concat())
    }
}

/// What `to() = from()` is in the `untraced` line of a copy whose target cannot be read.
pub(crate) fn copy_what(p: &Program, to: &ArrayRef, from: &ArrayRef) -> String {
    let name = |array: &ArrayRef| {
        let (root, members) = array_path(array);
        path_name(p, root, &members)
    };
    format!("{} = {}", name(to), name(from))
}

/// What a call of a `DECLARE LIBRARY` procedure is in its `untraced` line.
pub(crate) fn library_what(p: &Program, proc: ProcId) -> String {
    format!("LIBRARY {}", p.proc(proc).name)
}

#[cfg(test)]
mod tests {
    use super::{Entries, Kind, Trace, path_name};
    use qb64rust_base::FileId;
    use qb64rust_ir::{
        ArrayRef, Body, Expr, ExprKind, Member, MemberId, Place, Proc, ProcId, ProcKind, Program, Shape, Storage, Ty,
        TypeId, UserType, Var, VarId,
    };

    fn var(name: &str, ty: Ty, storage: Storage, shape: Shape) -> Var {
        Var {
            name: name.into(),
            ty,
            storage,
            shape,
            conventional: false,
        }
    }

    fn sub(name: &str) -> Proc {
        Proc {
            name: name.into(),
            kind: ProcKind::Sub,
            params: Vec::new(),
            result: None,
            body: Body::default(),
            file: FileId(0),
            line: 1,
            end_file: FileId(0),
            end_line: 2,
            external: None,
        }
    }

    fn member(name: &str, ty: Ty) -> Member {
        Member {
            name: name.into(),
            ty,
            bounds: Vec::new(),
            dynamic: false,
        }
    }

    /// `TYPE POS: X, Y: END TYPE`, `TYPE PT: POS AS POS: V(0 TO 2): END TYPE`; `I` local to `A` and to `B`, an
    /// array `X`, `P AS PT`, `A%` and `A$`, an array `SHIPS() AS PT`, and a temporary of the lowering.
    fn program() -> Program {
        let (a, b) = (ProcId(0), ProcId(1));
        let pt = Ty::User(TypeId(1));
        Program {
            vars: vec![
                var("I", Ty::I16, Storage::Local(a), Shape::Scalar),
                var("I", Ty::I16, Storage::Local(b), Shape::Scalar),
                var("X", Ty::F32, Storage::Global, Shape::Static { bounds: vec![(0, 9)] }),
                var("P", pt, Storage::Global, Shape::Scalar),
                var("A", Ty::I16, Storage::Global, Shape::Scalar),
                var("A", Ty::Str, Storage::Global, Shape::Scalar),
                var("SHIPS", pt, Storage::Global, Shape::Static { bounds: vec![(0, 4)] }),
                var("for1.limit", Ty::I32, Storage::Temp(None), Shape::Scalar),
            ],
            procs: vec![sub("A"), sub("B")],
            types: vec![
                UserType {
                    name: "POS".into(),
                    members: vec![member("X", Ty::I16), member("Y", Ty::I16)],
                },
                UserType {
                    name: "PT".into(),
                    members: vec![
                        member("POS", Ty::User(TypeId(0))),
                        Member {
                            bounds: vec![(0, 2)],
                            ..member("V", Ty::I32)
                        },
                    ],
                },
            ],
            ..Program::default()
        }
    }

    fn element(array: u32, index: i64) -> Place {
        Place::Element {
            array: ArrayRef::Var(VarId(array)),
            index: vec![Expr {
                span: qb64rust_base::Span::new(FileId(0), 0, 0),
                ty: Ty::I32,
                qb: Ty::I32,
                kind: ExprKind::Int(index),
            }],
        }
    }

    fn of(base: Place, member: u32) -> Place {
        Place::Member {
            base: Box::new(base),
            member: MemberId(member),
        }
    }

    /// The scenarios of "The traced variables" and "What an entry says" (spec `compiler/value-trace`).
    #[test]
    fn entries_are_numbered_by_variable_array_and_member_path() {
        let p = program();
        let trace = Trace { files: Vec::new() };
        let mut t = Entries::new(&trace, &p);
        let mut entry = |place: &Place| t.entry(&p, place);
        // Two `i`s in two procedures.
        let (i_a, i_b) = (entry(&Place::Var(VarId(0))), entry(&Place::Var(VarId(1))));
        assert_eq!((i_a, i_b), (Some(0), Some(1)));
        assert_eq!(entry(&Place::Var(VarId(0))), Some(0));
        // `x(1)` and `x(2)` are one entry.
        assert_eq!(entry(&element(2, 1)), Some(2));
        assert_eq!(entry(&element(2, 2)), Some(2));
        // `p.pos.x` is an entry of its own, and so is `p.pos.y`.
        let pos = of(Place::Var(VarId(3)), 0);
        assert_eq!(entry(&of(pos.clone(), 0)), Some(3));
        assert_eq!(entry(&of(pos, 1)), Some(4));
        // `a%` and `a$`.
        assert_eq!(entry(&Place::Var(VarId(4))), Some(5));
        assert_eq!(entry(&Place::Var(VarId(5))), Some(6));
        // `ships(1).pos.x` and `ships(2).pos.x` are one entry; an element of the member array `p.v` is the member's.
        assert_eq!(entry(&of(of(element(6, 1), 0), 0)), Some(7));
        assert_eq!(entry(&of(of(element(6, 2), 0), 0)), Some(7));
        let v = Place::Element {
            array: ArrayRef::Member {
                base: Box::new(Place::Var(VarId(3))),
                member: MemberId(1),
            },
            index: Vec::new(),
        };
        assert_eq!(entry(&v), Some(8));
        // A variable the lowering made has none.
        assert_eq!(entry(&Place::Var(VarId(7))), None);

        let said: Vec<(&str, &str, Option<&str>, Kind)> = t
            .entries
            .iter()
            .map(|e| (e.name.as_str(), e.ty.as_str(), e.procedure.as_deref(), e.kind))
            .collect();
        assert_eq!(
            said,
            [
                ("I", "INTEGER", Some("A"), Kind::Int),
                ("I", "INTEGER", Some("B"), Kind::Int),
                ("X()", "SINGLE", None, Kind::F32),
                ("P.POS.X", "INTEGER", None, Kind::Int),
                ("P.POS.Y", "INTEGER", None, Kind::Int),
                ("A", "INTEGER", None, Kind::Int),
                ("A", "STRING", None, Kind::Str),
                ("SHIPS().POS.X", "INTEGER", None, Kind::Int),
                ("P.V()", "LONG", None, Kind::Int),
            ]
        );
        assert_eq!(path_name(&p, VarId(6), &[]), "SHIPS()");
    }

    /// A site, an alias and an untraced store are numbered once each.
    #[test]
    fn sites_and_counters_are_shared() {
        let trace = Trace { files: Vec::new() };
        let mut t = Entries::new(&trace, &Program::default());
        assert_eq!((t.site(0, 12), t.site(1, 12), t.site(0, 12)), (0, 1, 0));
        assert_eq!(t.untraced(0, "POKE", None), "qbr_trc_un[0]++;");
        assert_eq!(t.untraced(0, "POKE", None), "qbr_trc_un[0]++;");
        assert_eq!(t.untraced(1, "POKE", None), "qbr_trc_un[1]++;");
        assert_eq!(t.untraced(0, "ERASE", Some(("B()", None))), "qbr_trc_un[2]++;");
        assert_eq!(t.untraced(0, "ERASE", Some(("B()", Some("S")))), "qbr_trc_un[3]++;");
        assert_eq!(t.alias(0, ("GRID()", None), ("FILL", "A()")), "qbr_trc_al[0]++;");
        assert_eq!(t.alias(0, ("GRID()", None), ("FILL", "A()")), "qbr_trc_al[0]++;");
        assert_eq!(t.alias(0, ("GRID()", Some("S")), ("FILL", "A()")), "qbr_trc_al[1]++;");
        let table = t.table();
        assert!(table.contains("#define QBR_TRC_UNTRACED 4\n"), "{table}");
        assert!(
            table.contains("static const uint32 qbr_trc_untraced_at[]={0,12,1,12,0,12,0,12};"),
            "{table}"
        );
    }
}
