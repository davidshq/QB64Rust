//! The debug symbol file of a debug build (design D2 of `m5-debug-build-symbols`, spec `compiler/debug-info`): the
//! program's files, procedures, variables and user types with the C names and layouts this emitter gives them, so
//! that a debugger can show the program in BASIC terms from the executable's debug information and memory alone.
//! Written here, beside the names and layouts it describes, so that a change of either changes the file too.
//!
//! The format (version 1) is a JSON object, one entry of each list per line:
//!
//! ```text
//! version     1
//! files       index, path (as the `#line` directives name it), hash (FNV-1a 64 of its bytes, as a coverage record)
//! main        the main module: its C function `QBMAIN` and file
//! procedures  index, name, kind (`sub`, `function`), c_name, file, line, end_file, end_line (the `END` may be in
//!             another file), params and result (indexes of `variables`)
//! variables   index, name, type, storage (`main`, `static`, `local`, `param`, `result`), procedure (an index of
//!             `procedures`, null for `main`), shape (`scalar`, `static`, `dynamic`), bounds (of a static array),
//!             dims (of an array, null where nothing fixed it), conventional, c_name, reach ([`Reach`])
//! types       name, size, members (name, type, offset, size, bounds, dynamic)
//! layouts     the fixed layouts a reader needs ([`Emitter::layouts`])
//! ```

use crate::Emitter;
use crate::decl::{dim_slot, lock_slot};
use crate::names::{proc_name, var_name};
use qb64rust_ir::{ProcKind, Program, Shape, Storage, Ty, Var, numeric_value_types};
use std::fmt::Write as _;

/// The version of the format; a change of what a reader may rely on is a new one.
pub const VERSION: u32 = 1;

/// The size of a pointer and of a descriptor slot (`ptrszint`): the build is 64-bit (`BITS=64`).
const POINTER_SIZE: u32 = 8;
/// The byte offsets of `chr` (`uint8_t *`) and `len` (`int32_t`) in libqb's `struct qbs`
/// (`libqb\internal\c\libqb\include\qbs.h`, its first two members; checked by a test).
const QBS_CHR: u32 = 0;
const QBS_LEN: u32 = 8;

/// How a variable's value is reached from its C name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    /// A C pointer variable to the value (`int32 *__LONG_TOTAL`); a `_BIT * n` in its 32- or 64-bit storage.
    Pointer,
    /// A C pointer variable to an array's descriptor (`ptrszint *`), whose data slot points at the elements.
    Descriptor,
    /// A C pointer to a `qbs`, a `STRING` or a `STRING * n`.
    String,
    /// A C pointer to the bytes of a value of a user type.
    Bytes,
    /// A parameter that is no array: a C pointer to the caller's value, or to a copy made for the call (a `qbs*` for
    /// a string, the bytes for a user type).
    ParamPointer,
}

impl Reach {
    fn word(self) -> &'static str {
        match self {
            Reach::Pointer => "pointer",
            Reach::Descriptor => "descriptor",
            Reach::String => "string",
            Reach::Bytes => "bytes",
            Reach::ParamPointer => "param-pointer",
        }
    }
}

/// How the value of `v` is reached from its C name, by its storage, shape and type.
pub fn reach(v: &Var) -> Reach {
    if v.is_array() {
        return Reach::Descriptor;
    }
    if let Storage::Param(_) = v.storage {
        return Reach::ParamPointer;
    }
    match v.ty {
        Ty::Str | Ty::FixedStr(_) => Reach::String,
        Ty::User(_) => Reach::Bytes,
        numeric_value_types!() | Ty::Bit { .. } => Reach::Pointer,
    }
}

/// A source file of the program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugFile {
    pub path: String,
    pub hash: String,
}

/// A SUB or FUNCTION of the program (one of a `DECLARE LIBRARY` block, or an OpenGL routine, has no body and is
/// left out).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugProc {
    pub name: String,
    pub function: bool,
    pub c_name: String,
    pub file: u32,
    pub line: u32,
    pub end_file: u32,
    pub end_line: u32,
    /// Indexes of [`DebugInfo::variables`].
    pub params: Vec<usize>,
    pub result: Option<usize>,
}

/// A variable the program allocates (no temporary of the lowering).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugVar {
    pub name: String,
    pub ty: String,
    /// `main`, `static`, `local`, `param` or `result`.
    pub storage: &'static str,
    /// An index of [`DebugInfo::procedures`]; `None` for a main-module variable.
    pub procedure: Option<usize>,
    /// `scalar`, `static` or `dynamic`.
    pub shape: &'static str,
    pub bounds: Vec<(i64, i64)>,
    pub dims: Option<usize>,
    pub conventional: bool,
    pub c_name: String,
    pub reach: Reach,
}

/// A member of a user type, at its offset in the emitter's layout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugMember {
    pub name: String,
    pub ty: String,
    pub offset: u32,
    pub size: u32,
    pub bounds: Vec<(i64, i64)>,
    pub dynamic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugType {
    pub name: String,
    pub size: u32,
    pub members: Vec<DebugMember>,
}

/// The debug symbol file's contents; [`DebugInfo::to_json`] writes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugInfo {
    pub files: Vec<DebugFile>,
    pub procedures: Vec<DebugProc>,
    pub variables: Vec<DebugVar>,
    pub types: Vec<DebugType>,
    /// The `layouts` object, written whole ([`Emitter::layouts`]).
    pub layouts: String,
}

/// The QB name of a type as a program spells it (`sema`'s `Program::type_name`).
fn type_name(p: &Program, t: Ty) -> String {
    match t {
        Ty::User(id) => p.user_type(id).name.clone(),
        Ty::Bit { width: 1, .. } => t.qb_name().to_string(),
        Ty::Bit { width, .. } => format!("{} * {width}", t.qb_name()),
        Ty::FixedStr(n) => format!("STRING * {n}"),
        numeric_value_types!() | Ty::Str => t.qb_name().to_string(),
    }
}

/// A JSON string.
fn json_string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if u32::from(c) < 0x20 => write!(out, "\\u{:04x}", u32::from(c)).unwrap(),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_bounds(bounds: &[(i64, i64)]) -> String {
    let b: Vec<String> = bounds.iter().map(|(l, u)| format!("[{l}, {u}]")).collect();
    format!("[{}]", b.join(", "))
}

fn json_opt(n: Option<usize>) -> String {
    n.map_or_else(|| "null".to_string(), |n| n.to_string())
}

/// The lines of a JSON list, one entry per line, under `name`.
fn json_list(out: &mut String, name: &str, entries: &[String], last: bool) {
    if entries.is_empty() {
        writeln!(out, "  \"{name}\": []{}", if last { "" } else { "," }).unwrap();
        return;
    }
    writeln!(out, "  \"{name}\": [").unwrap();
    for (i, e) in entries.iter().enumerate() {
        writeln!(out, "    {e}{}", if i + 1 < entries.len() { "," } else { "" }).unwrap();
    }
    writeln!(out, "  ]{}", if last { "" } else { "," }).unwrap();
}

impl DebugInfo {
    /// The file's text: JSON, one entry of each list per line.
    pub fn to_json(&self) -> String {
        let mut out = format!("{{\n  \"version\": {VERSION},\n");
        let files: Vec<String> = (self.files.iter().enumerate())
            .map(|(i, f)| {
                format!(
                    "{{\"index\": {i}, \"path\": {}, \"hash\": {}}}",
                    json_string(&f.path),
                    json_string(&f.hash)
                )
            })
            .collect();
        json_list(&mut out, "files", &files, false);
        writeln!(out, "  \"main\": {{\"c_name\": \"QBMAIN\", \"file\": 0}},").unwrap();
        let procs: Vec<String> = (self.procedures.iter().enumerate())
            .map(|(i, q)| {
                let params: Vec<String> = q.params.iter().map(usize::to_string).collect();
                format!(
                    "{{\"index\": {i}, \"name\": {}, \"kind\": \"{}\", \"c_name\": {}, \"file\": {}, \"line\": {}, \
                     \"end_file\": {}, \"end_line\": {}, \"params\": [{}], \"result\": {}}}",
                    json_string(&q.name),
                    if q.function { "function" } else { "sub" },
                    json_string(&q.c_name),
                    q.file,
                    q.line,
                    q.end_file,
                    q.end_line,
                    params.join(", "),
                    json_opt(q.result)
                )
            })
            .collect();
        json_list(&mut out, "procedures", &procs, false);
        let vars: Vec<String> = (self.variables.iter().enumerate())
            .map(|(i, v)| {
                format!(
                    "{{\"index\": {i}, \"name\": {}, \"type\": {}, \"storage\": \"{}\", \"procedure\": {}, \
                     \"shape\": \"{}\", \"bounds\": {}, \"dims\": {}, \"conventional\": {}, \"c_name\": {}, \
                     \"reach\": \"{}\"}}",
                    json_string(&v.name),
                    json_string(&v.ty),
                    v.storage,
                    json_opt(v.procedure),
                    v.shape,
                    json_bounds(&v.bounds),
                    json_opt(v.dims),
                    v.conventional,
                    json_string(&v.c_name),
                    v.reach.word()
                )
            })
            .collect();
        json_list(&mut out, "variables", &vars, false);
        let types: Vec<String> = self
            .types
            .iter()
            .map(|t| {
                let members: Vec<String> = t
                    .members
                    .iter()
                    .map(|m| {
                        format!(
                            "{{\"name\": {}, \"type\": {}, \"offset\": {}, \"size\": {}, \"bounds\": {}, \
                             \"dynamic\": {}}}",
                            json_string(&m.name),
                            json_string(&m.ty),
                            m.offset,
                            m.size,
                            json_bounds(&m.bounds),
                            m.dynamic
                        )
                    })
                    .collect();
                format!(
                    "{{\"name\": {}, \"size\": {}, \"members\": [{}]}}",
                    json_string(&t.name),
                    t.size,
                    members.join(", ")
                )
            })
            .collect();
        json_list(&mut out, "types", &types, false);
        writeln!(out, "  \"layouts\": {}", self.layouts).unwrap();
        out.push_str("}\n");
        out
    }
}

impl Emitter<'_> {
    /// The debug symbol file of the program; `files` is each file's path and hash by its index.
    pub(crate) fn debug_info(&self, files: &[DebugFile]) -> DebugInfo {
        let p = self.p;
        // A program procedure's index in `procedures`, by its index in the program.
        let mut proc_index = vec![None; p.procs.len()];
        let mut procedures = Vec::new();
        for (i, q) in p.procs.iter().enumerate() {
            if q.external.is_none() {
                proc_index[i] = Some(procedures.len());
                procedures.push(DebugProc {
                    name: q.name.clone(),
                    function: matches!(q.kind, ProcKind::Function(_)),
                    c_name: proc_name(q),
                    file: q.file.0,
                    line: q.line,
                    end_file: q.end_file.0,
                    end_line: q.end_line,
                    params: Vec::new(),
                    result: None,
                });
            }
        }
        let mut var_index = vec![None; p.vars.len()];
        let mut variables = Vec::new();
        for (i, v) in p.vars.iter().enumerate() {
            let (storage, procedure) = match v.storage {
                Storage::Global => ("main", None),
                Storage::Static(q) => ("static", Some(q)),
                Storage::Local(q) => ("local", Some(q)),
                Storage::Param(q) => ("param", Some(q)),
                Storage::Result(q) => ("result", Some(q)),
                Storage::Temp(_) => continue,
            };
            // The parameters and result of a library procedure are variables for the calls' sake only.
            let procedure = match procedure {
                Some(q) => match proc_index[q.0 as usize] {
                    Some(k) => Some(k),
                    None => continue,
                },
                None => None,
            };
            let shape = match v.shape {
                Shape::Scalar => "scalar",
                Shape::Static { .. } => "static",
                Shape::Dynamic { .. } => "dynamic",
            };
            var_index[i] = Some(variables.len());
            variables.push(DebugVar {
                name: v.name.clone(),
                ty: type_name(p, v.ty),
                storage,
                procedure,
                shape,
                bounds: v.bounds().to_vec(),
                dims: v.dim_count(),
                conventional: v.conventional,
                c_name: var_name(p, v),
                reach: reach(v),
            });
        }
        for (q, k) in p.procs.iter().zip(&proc_index) {
            if let Some(k) = *k {
                let entry = &mut procedures[k];
                entry.params = q.params.iter().filter_map(|v| var_index[v.0 as usize]).collect();
                entry.result = q.result.and_then(|v| var_index[v.0 as usize]);
            }
        }
        let types = (p.types.iter().enumerate())
            .map(|(i, t)| {
                let (offsets, size) = self.layout(qb64rust_ir::TypeId(qb64rust_base::to_u32(i)));
                let members = (t.members.iter().zip(offsets))
                    .map(|(m, offset)| DebugMember {
                        name: m.name.clone(),
                        ty: type_name(p, m.ty),
                        offset,
                        size: self.member_size(m),
                        bounds: m.bounds.clone(),
                        dynamic: m.dynamic,
                    })
                    .collect();
                DebugType {
                    name: t.name.clone(),
                    size,
                    members,
                }
            })
            .collect();
        DebugInfo {
            files: files.to_vec(),
            procedures,
            variables,
            types,
            layouts: self.layouts(),
        }
    }

    /// The `layouts` object: the pointer size; an array descriptor's slots (`ptrszint` each), the data pointer and
    /// the flags (`decl.rs`: 1 allocated, 2 static, 4 in conventional memory), and per dimension four slots (lower
    /// bound, count, multiplier, unused) from slot 4, the last dimension first ([`dim_slot`]), then the `_MEM` lock
    /// ([`lock_slot`]); a `qbs`'s `chr` and `len`; the size of a `_FLOAT` element in an array; how a string member of
    /// a user type is held.
    fn layouts(&self) -> String {
        let first = dim_slot(1, 0);
        let each = dim_slot(2, 0) - dim_slot(2, 1);
        debug_assert!((1..4).all(|n| lock_slot(n) == each * n + first));
        format!(
            "{{\"pointer_size\": {POINTER_SIZE}, \
             \"array\": {{\"slot_size\": {POINTER_SIZE}, \"data\": 0, \"flags\": 2, \
             \"flag_bits\": {{\"allocated\": 1, \"static\": 2, \"conventional\": 4}}, \
             \"dimensions\": {{\"first_slot\": {first}, \"slots_each\": {each}, \"order\": \"last first\", \
             \"slot_of\": \"{each} * (n - 1 - k) + {first}\", \"lower_bound\": 0, \"count\": 1, \"multiplier\": 2}}, \
             \"lock\": \"slot {each} * n + {first}, after the dimensions\"}}, \
             \"qbs\": {{\"chr\": {QBS_CHR}, \"len\": {QBS_LEN}, \"len_size\": 4}}, \
             \"float_element_size\": {}, \"bit_array\": \"packed, the width in bits per element\", \
             \"user_type_members\": {{\"STRING\": \"pointer to a qbs\", \"STRING * n\": \"n bytes in place\"}}}}",
            self.ty_size(Ty::F80)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qb64rust_ir::ProcId;

    fn var(ty: Ty, storage: Storage, shape: Shape) -> Var {
        Var {
            name: "V".into(),
            ty,
            storage,
            shape,
            conventional: false,
        }
    }

    #[test]
    fn reach_by_storage_shape_and_type() {
        let scalar = Shape::Scalar;
        let q = ProcId(0);
        assert_eq!(reach(&var(Ty::I32, Storage::Global, scalar.clone())), Reach::Pointer);
        assert_eq!(reach(&var(Ty::I16, Storage::Static(q), scalar.clone())), Reach::Pointer);
        assert_eq!(reach(&var(Ty::F64, Storage::Local(q), scalar.clone())), Reach::Pointer);
        assert_eq!(reach(&var(Ty::I32, Storage::Result(q), scalar.clone())), Reach::Pointer);
        let bit = Ty::Bit { width: 3, signed: true };
        assert_eq!(reach(&var(bit, Storage::Global, scalar.clone())), Reach::Pointer);
        assert_eq!(reach(&var(Ty::Str, Storage::Global, scalar.clone())), Reach::String);
        assert_eq!(
            reach(&var(Ty::FixedStr(4), Storage::Local(q), scalar.clone())),
            Reach::String
        );
        assert_eq!(reach(&var(Ty::Str, Storage::Result(q), scalar.clone())), Reach::String);
        assert_eq!(
            reach(&var(Ty::User(qb64rust_ir::TypeId(0)), Storage::Global, scalar.clone())),
            Reach::Bytes
        );
        assert_eq!(
            reach(&var(Ty::I32, Storage::Param(q), scalar.clone())),
            Reach::ParamPointer
        );
        assert_eq!(reach(&var(Ty::Str, Storage::Param(q), scalar)), Reach::ParamPointer);
        let fixed = Shape::Static { bounds: vec![(1, 3)] };
        let dynamic = Shape::Dynamic {
            dims: Some(1),
            at_start: None,
        };
        assert_eq!(reach(&var(Ty::F32, Storage::Global, fixed)), Reach::Descriptor);
        assert_eq!(
            reach(&var(Ty::Str, Storage::Local(q), dynamic.clone())),
            Reach::Descriptor
        );
        assert_eq!(reach(&var(Ty::I32, Storage::Param(q), dynamic)), Reach::Descriptor);
    }

    /// A program of two variables, a main-module `LONG` and a SUB's local `STRING`: the file parses as JSON, is
    /// version 1, and names both with their C names and the SUB.
    #[test]
    fn a_two_variable_program_is_json_of_version_1() {
        let mut p = Program::default();
        p.procs.push(qb64rust_ir::Proc {
            name: "SHOW".into(),
            kind: ProcKind::Sub,
            params: Vec::new(),
            result: None,
            body: qb64rust_ir::Body::default(),
            file: qb64rust_base::FileId(0),
            line: 3,
            end_file: qb64rust_base::FileId(0),
            end_line: 5,
            external: None,
        });
        let mut total = var(Ty::I32, Storage::Global, Shape::Scalar);
        total.name = "TOTAL".into();
        let mut s = var(Ty::Str, Storage::Local(ProcId(0)), Shape::Scalar);
        s.name = "S".into();
        p.vars.extend([total, s]);
        let files = [DebugFile {
            path: "sub dir\\\"q\".bas".into(),
            hash: "0123456789abcdef".into(),
        }];
        let (_, info) = crate::emit_debug(
            &p,
            "p.bas",
            &std::collections::HashMap::new(),
            &crate::Resolved::default(),
            None,
            None,
            &files,
        );
        let json: serde_json::Value = serde_json::from_str(&info.to_json()).expect("valid JSON");
        assert_eq!(json["version"], 1);
        assert_eq!(json["files"][0]["path"], "sub dir\\\"q\".bas");
        assert_eq!(json["main"]["c_name"], "QBMAIN");
        assert_eq!(json["procedures"][0]["c_name"], "SUB_SHOW");
        assert_eq!(json["variables"][0]["c_name"], "__LONG_TOTAL");
        assert_eq!(json["variables"][0]["storage"], "main");
        assert_eq!(json["variables"][1]["c_name"], "_SUB_SHOW_STRING_S");
        assert_eq!(json["variables"][1]["procedure"], 0);
        assert_eq!(json["variables"][1]["reach"], "string");
        assert_eq!(json["layouts"]["array"]["dimensions"]["first_slot"], 4);
    }

    #[test]
    fn json_strings_escape_quotes_backslashes_and_control_characters() {
        assert_eq!(json_string("a\"b\\c\u{1}\u{e9}"), "\"a\\\"b\\\\c\\u0001\u{e9}\"");
    }

    /// The constants for `qbs` follow libqb's `struct qbs` in this repo's copy: a pointer `chr` first, then the
    /// `int32_t len`, so `len` is at the pointer size (the risk of design D2).
    #[test]
    #[expect(clippy::disallowed_methods, reason = "a C header of libqb, ASCII, not BASIC source")]
    fn qbs_offsets_follow_the_copys_header() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../libqb/internal/c/libqb/include/qbs.h"
        );
        let text = std::fs::read(path).expect("the copy of libqb");
        let text = String::from_utf8_lossy(&text);
        let body = text.split("struct qbs {").nth(1).expect("struct qbs");
        let members: Vec<&str> = body
            .lines()
            .map(|l| l.split("//").next().unwrap().trim())
            .filter(|l| !l.is_empty())
            .take(2)
            .collect();
        assert_eq!(members, ["uint8_t *chr;", "int32_t len;"]);
        assert_eq!((QBS_CHR, QBS_LEN), (0, POINTER_SIZE));
    }
}
