//! `DECLARE LIBRARY` (design D3 of `m3-declare-library`; measured, `verification\v37_*`, `study\00` §5): the headers
//! and prototypes in `regsf.txt`, the pointers of `DYNAMIC` and `CUSTOMTYPE` functions set in `maindata.txt`, and the
//! calls. Which files a block names is the driver's to find ([`Resolved`]); the emitter reads no file.

use crate::Emitter;
use crate::decl::is_qbs;
use crate::names::{c_string, c_type, proc_name};
use qb64rust_ir::{Arg, Conv, Expr, ExprKind, LibraryId, LibraryKind, Proc, ProcId, ProcKind, Track, Ty};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::PathBuf;

/// What the driver found for the program's libraries (design D2): per [`qb64rust_ir::Program::libraries`] entry, in
/// its order, and the linkage of each declaration of a library without a header. A library with no entry, and a
/// declaration with no linkage, are taken as found with nothing: no header, no file, C linkage.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Resolved {
    pub libraries: Vec<ResolvedLibrary>,
    pub linkage: HashMap<ProcId, Linkage>,
}

/// The files of one block.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResolvedLibrary {
    /// The headers its names found, in the order of the names, each `#include`d at the block as the old compiler
    /// does: once per name that finds it, so a header two blocks name is included twice (measured,
    /// `verification\v37_x16`; the driver rejects that for a header without a guard, whose C++ would fail). A block
    /// with one writes no prototype (measured, `verification\v37_x23`).
    pub headers: Vec<PathBuf>,
    /// The library files, for the link command (the driver's).
    pub files: Vec<PathBuf>,
    /// A `DYNAMIC` library: the name `LoadLibrary` is given, as the old compiler writes it (the full path when `./`
    /// found it, else the written name with `.dll`).
    pub dll: Option<String>,
}

/// How the C++ names a function of a library without a header: as C (`extern "C"`) or as C++. Decided by the
/// symbols of the library's files (the driver's `nm` step).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Linkage {
    #[default]
    C,
    /// C++, with the symbol's name and parameters as `nm --demangle` shows them, from the name to the first `)`
    /// (`cpp_add(int, int)`): a `CUSTOMTYPE` block without a header declares the function by it, as the old
    /// compiler does (`qb64pe.bas` 13819–13827).
    Cpp(String),
}

/// The linkage of a declaration the driver gave none.
static NO_LINKAGE: Linkage = Linkage::C;

impl Resolved {
    fn library(&self, id: LibraryId) -> Option<&ResolvedLibrary> {
        self.libraries.get(id.0 as usize)
    }

    fn headers(&self, id: LibraryId) -> &[PathBuf] {
        self.library(id).map_or(&[], |l| &l.headers)
    }

    fn linkage_of(&self, id: ProcId) -> &Linkage {
        self.linkage.get(&id).unwrap_or(&NO_LINKAGE)
    }
}

/// The handle of a `DYNAMIC` library and the name `LoadLibrary` is given.
struct Dll {
    handle: String,
    dll: String,
}

/// A type of a prototype, as the old compiler's `typ2ctyp` writes it: both `_OFFSET`s are `ptrszint` (measured,
/// `verification\v37_f`).
fn proto_ty(t: Ty) -> &'static str {
    match t {
        Ty::Str => "char*",
        Ty::User(_) => "void*",
        Ty::UOff => "ptrszint",
        Ty::I8
        | Ty::U8
        | Ty::I16
        | Ty::U16
        | Ty::I32
        | Ty::U32
        | Ty::I64
        | Ty::U64
        | Ty::Off
        | Ty::Bit { .. }
        | Ty::F32
        | Ty::F64
        | Ty::F80
        | Ty::FixedStr(_) => c_type(t),
    }
}

/// The name of a `DYNAMIC` library's handle as the old compiler makes it (`qb64pe.bas` 4526–4567, 5243–5249): `DLL_`
/// and the written name without its version and folder, each byte that is no ASCII letter or digit an `_` (so
/// `"my.lib"` gives `DLL_my_lib`). Each `DYNAMIC` block declares its handle, so two blocks whose handles have one
/// name fail in C++ (read in `qb64pe.bas` 5206, 5271): the driver rejects them.
pub fn dll_handle(written: &str) -> String {
    let name = strip_version(written);
    let file = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let ident: String = file
        .bytes()
        .map(|b| if b.is_ascii_alphanumeric() { char::from(b) } else { '_' })
        .collect();
    format!("DLL_{ident}")
}

/// A library's name without its version, as the old compiler's `striplibver` (`qb64pe.bas` 4526–4543): from the
/// end, a part after a `.` or a `:` that is a number without a leading zero is dropped, until a part that is no
/// number, a folder or a `:` (`lib.1.0` and `lib:1` give `lib`, `lib:1.0` gives `lib`).
pub fn strip_version(mut name: &str) -> &str {
    let is_uinteger =
        |s: &str| !s.is_empty() && !(s.len() > 1 && s.starts_with('0')) && s.bytes().all(|b| b.is_ascii_digit());
    while let Some(at) = name.rfind(['.', ':', '/', '\\']) {
        let sep = name.as_bytes()[at];
        if matches!(sep, b'/' | b'\\') || !is_uinteger(&name[at + 1..]) {
            break;
        }
        name = &name[..at];
        if sep == b':' {
            break;
        }
    }
    name
}

/// A header's path in an `#include`, with `/` between its parts: no backslash in the directive (GCC takes it as it
/// is, the old compiler doubles it).
fn include_line(path: &std::path::Path) -> String {
    format!("#include \"{}\"\n", path.to_string_lossy().replace('\\', "/"))
}

impl Emitter<'_> {
    /// The library procedures of library `id`, in declaration order.
    fn declarations(&self, id: LibraryId) -> impl Iterator<Item = (ProcId, &Proc)> {
        self.p
            .procs
            .iter()
            .enumerate()
            .filter(move |(_, q)| q.external.as_ref().is_some_and(|e| e.library == id))
            .map(|(i, q)| (ProcId(qb64rust_base::to_u32(i)), q))
    }

    /// `int32` and `(int32,int32*)` of a library procedure: its result's C type and its parameters', a parameter
    /// without `BYVAL` a pointer (a `STRING` the characters, a `TYPE` the bytes).
    fn proto_parts(&self, q: &Proc) -> (&'static str, String) {
        let ret = match q.kind {
            ProcKind::Sub => "void",
            ProcKind::Function(t) => proto_ty(t),
        };
        let byval = &q.external.as_ref().expect("a library procedure").byval;
        let params: Vec<String> = q
            .params
            .iter()
            .zip(byval)
            .map(|(&v, &byval)| {
                let t = self.p.var(v).ty;
                let pointer = !byval && !matches!(t, Ty::Str | Ty::User(_));
                if pointer {
                    format!("{}*", proto_ty(t))
                } else {
                    proto_ty(t).to_string()
                }
            })
            .collect();
        (ret, format!("({})", params.join(",")))
    }

    /// The handle of a `DYNAMIC` library `id` (`None` for the other forms): its name ([`dll_handle`]) and the name
    /// `LoadLibrary` is given.
    fn dll(&self, id: LibraryId) -> Option<Dll> {
        let lib = self.p.library(id);
        let name = lib.names.first().filter(|_| lib.kind == LibraryKind::Dynamic)?;
        let dll = self
            .resolved
            .library(id)
            .and_then(|l| l.dll.clone())
            .unwrap_or_else(|| format!("{}.dll", name.text));
        Some(Dll {
            handle: dll_handle(&name.text),
            dll,
        })
    }

    /// The libraries' part of `regsf.txt`, block by block: its headers; with names but no header a prototype per
    /// declaration, `extern "C"` for a C name; a `DYNAMIC` block's handle and function pointers; a `CUSTOMTYPE`
    /// block's function pointers.
    pub(crate) fn library_prototypes(&self) -> String {
        let mut out = String::new();
        for (i, lib) in self.p.libraries.iter().enumerate() {
            let id = LibraryId(qb64rust_base::to_u32(i));
            let headers = self.resolved.headers(id);
            for h in headers {
                out.push_str(&include_line(h));
            }
            match lib.kind {
                // libqb's `gl_helper_code.h` declares the helpers, `gl.h` the rest (design D4 of `m4-sub-gl`).
                LibraryKind::OpenGl => {}
                LibraryKind::Plain | LibraryKind::Static => {
                    // A block with no name reaches what the program's C++ already includes, the C runtime: no
                    // prototype either (measured, `verification\v37_a`).
                    if !headers.is_empty() || lib.names.is_empty() {
                        continue;
                    }
                    for (pid, q) in self.declarations(id) {
                        let (ret, params) = self.proto_parts(q);
                        let c = match self.resolved.linkage_of(pid) {
                            Linkage::C => "extern \"C\" ",
                            Linkage::Cpp(_) => "",
                        };
                        let name = &q.external.as_ref().expect("a library procedure").c_name;
                        writeln!(out, "{c}{ret} {name}{params};").unwrap();
                    }
                }
                LibraryKind::Dynamic => {
                    if let Some(Dll { handle, .. }) = self.dll(id) {
                        writeln!(out, "HINSTANCE {handle}=NULL;").unwrap();
                    }
                    for (_, q) in self.declarations(id) {
                        let (ret, params) = self.proto_parts(q);
                        let x = proc_name(q);
                        writeln!(
                            out,
                            "typedef {ret} (CALLBACK* DLLCALL_{x}){params};\nDLLCALL_{x} {x}=NULL;"
                        )
                        .unwrap();
                    }
                }
                LibraryKind::Customtype => {
                    for (_, q) in self.declarations(id) {
                        let (ret, params) = self.proto_parts(q);
                        let x = proc_name(q);
                        writeln!(out, "typedef {ret} CUSTOMCALL_{x}{params};\nCUSTOMCALL_{x} *{x}=NULL;").unwrap();
                    }
                }
            }
        }
        out
    }

    /// The libraries' part of `global.txt`: a function a `CUSTOMTYPE` block without a header takes the address of is
    /// declared with the result `void` (the pointer's type is the declaration's), a C name with no parameters, a C++
    /// one with those `nm` showed, which pick the symbol (`qb64pe.bas` 13819–13856).
    pub(crate) fn library_globals(&self) -> String {
        let mut out = String::new();
        for (i, lib) in self.p.libraries.iter().enumerate() {
            let id = LibraryId(qb64rust_base::to_u32(i));
            if lib.kind != LibraryKind::Customtype || !self.resolved.headers(id).is_empty() {
                continue;
            }
            for (pid, q) in self.declarations(id) {
                match self.resolved.linkage_of(pid) {
                    Linkage::C => {
                        let name = &q.external.as_ref().expect("a library procedure").c_name;
                        writeln!(out, "extern \"C\"{{\nextern void {name}(void);\n}}").unwrap();
                    }
                    Linkage::Cpp(demangled) => writeln!(out, "extern void {demangled};").unwrap(),
                }
            }
        }
        out
    }

    /// The libraries' part of `maindata.txt`, before the variables: a `DYNAMIC` library is loaded (error 259 if it
    /// cannot be) and each of its functions found (error 260 if it is not there); a `CUSTOMTYPE` function's pointer
    /// is set to the C function's address. Under `$ERRORLOCATION:ON` a failure first records the line of the
    /// `DECLARE` (error 259) or of the `SUB`/`FUNCTION` (error 260), since no statement's code has run yet
    /// (`qb64pe.bas` 5253–5274, 5874–5895).
    pub(crate) fn library_start(&self) -> String {
        let track = |at: &Track| {
            if self.p.settings.error_location {
                format!(
                    "error_track_line({});",
                    self.track_args(Some(at.file), at.line, at.include_line)
                )
            } else {
                String::new()
            }
        };
        let mut out = String::new();
        for (i, lib) in self.p.libraries.iter().enumerate() {
            let id = LibraryId(qb64rust_base::to_u32(i));
            match lib.kind {
                LibraryKind::Plain | LibraryKind::Static | LibraryKind::OpenGl => {}
                LibraryKind::Dynamic => {
                    let Some(Dll { handle, dll }) = self.dll(id) else {
                        continue;
                    };
                    writeln!(
                        out,
                        "if (!{handle}){{\n{handle}=LoadLibrary({});\nif (!{handle}){{{}error(259);}}\n}}",
                        c_string(dll.as_bytes()),
                        track(&lib.at)
                    )
                    .unwrap();
                    for (_, q) in self.declarations(id) {
                        let x = proc_name(q);
                        let e = q.external.as_ref().expect("a library procedure");
                        writeln!(
                            out,
                            "if (!{x}){{\n{x}=(DLLCALL_{x})GetProcAddress({handle},{});\nif (!{x}){{{}error(260);}}\n}}",
                            c_string(e.c_name.as_bytes()),
                            track(&e.at)
                        )
                        .unwrap();
                    }
                }
                LibraryKind::Customtype => {
                    for (_, q) in self.declarations(id) {
                        let x = proc_name(q);
                        let name = &q.external.as_ref().expect("a library procedure").c_name;
                        writeln!(out, "{x}=(CUSTOMCALL_{x}*)&{name};").unwrap();
                    }
                }
            }
        }
        out
    }

    /// A call of library procedure `proc`: the C function by its name, or through the pointer of a `DYNAMIC` or
    /// `CUSTOMTYPE` block; a FUNCTION's result cast to its declared type (so a C `double` is truncated for a
    /// `LONG` FUNCTION, as the old compiler does: measured, `verification\v37_x63`), a `STRING` one copied into a
    /// new string (a `NULL` gives "", `x39`). An OpenGL routine's result has no cast, and `_glGetString`'s pointer
    /// is read as the old compiler's call name has it (`(  char*  )call_glGetString`; design D4 of `m4-sub-gl`).
    pub(crate) fn library_call(&mut self, proc: ProcId, args: &[Arg]) -> String {
        let q = self.p.proc(proc);
        let e = q.external.as_ref().expect("a library procedure");
        let kind = self.p.library(e.library).kind;
        let callee = match kind {
            LibraryKind::Plain | LibraryKind::Static | LibraryKind::OpenGl => e.c_name.clone(),
            LibraryKind::Dynamic | LibraryKind::Customtype => proc_name(q),
        };
        let proc_kind = q.kind;
        let call = format!("{callee}({})", self.library_args(proc, args));
        if kind == LibraryKind::OpenGl {
            return match proc_kind {
                ProcKind::Function(Ty::Str) => format!("qbs_new_txt((  char*  ){call})"),
                ProcKind::Sub | ProcKind::Function(_) => call,
            };
        }
        match proc_kind {
            ProcKind::Sub => call,
            ProcKind::Function(Ty::Str) => format!("qbs_new_txt((char*){call})"),
            ProcKind::Function(t) => format!("(({}){call})", c_type(t)),
        }
    }

    /// The arguments of a library call (measured, `verification\v37_b`–`v37_e`): a number by reference as for a
    /// program's procedure (the place's pointer, or `&(passN=…)`), a `STRING` as its characters
    /// (`(char*)(s)->chr`, which C may change in place), a `TYPE` as its bytes, a `BYVAL` value as BASIC's own
    /// expression with no cast ([`byval_value`]), `NULL` for [`Arg::Null`].
    pub(crate) fn library_args(&mut self, proc: ProcId, args: &[Arg]) -> String {
        let params = self.p.proc(proc).params.clone();
        let parts: Vec<String> = args
            .iter()
            .zip(params)
            .map(|(a, param)| {
                let ty = self.p.var(param).ty;
                match a {
                    Arg::Ref(place) if is_qbs(ty) => format!("(char*)({})->chr", self.place_ref(place)),
                    Arg::Temp(v, _) if is_qbs(ty) => format!("(char*)({})->chr", self.value(v)),
                    Arg::Ref(_) | Arg::Temp(..) | Arg::Value(_) | Arg::Null | Arg::Array(_) => self.arg(a, param),
                }
            })
            .collect();
        parts.join(",")
    }
}

/// A `BYVAL` argument as the old compiler writes it: the value converted to the parameter's type, but with no cast
/// where C converts it the same way (measured, `verification\v37_b`, `v37_e`): an integer to another integer or to
/// a float, a float to another float. So with a header the C++ overload follows the argument's BASIC type
/// (`h_over(1)` for a `DOUBLE` parameter takes the `int` one). A float to an integer keeps its rounding (`qbr`), and
/// a `_CAST` its cast, which the overload then follows (corpus `slice\s152_library_cast`).
pub(crate) fn byval_value(v: &Expr) -> &Expr {
    if let ExprKind::Convert { how, from } = &v.kind
        && !matches!(how, Conv::RoundEven | Conv::Cast)
    {
        return byval_value(from);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The handle's name as the old compiler makes it (`qb64pe.bas` 4526–4567, 5243–5249).
    #[test]
    fn dll_handle_names() {
        let cases = [
            ("kernel32", "DLL_kernel32"),
            ("./v37dll", "DLL_v37dll"),
            ("sub\\dir/lib-x", "DLL_lib_x"),
            ("my.lib", "DLL_my_lib"),
            ("lib.1.0", "DLL_lib"),
            ("lib:1", "DLL_lib"),
            ("lib:1.0", "DLL_lib"),
            ("lib:1:2", "DLL_lib_1"),
            ("lib.01", "DLL_lib_01"),
            ("lib.2a", "DLL_lib_2a"),
            ("v.1/lib", "DLL_lib"),
            ("é", "DLL___"),
        ];
        for (written, handle) in cases {
            assert_eq!(dll_handle(written), handle, "{written}");
        }
    }
}
