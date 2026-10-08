//! Names: C types and identifiers, labels, string literals, `#line` directives and constants.

use crate::Emitter;
use qb64rust_base::FileId;
use qb64rust_ir::{Body, LabelId, Proc, ProcKind, Program, Storage, Ty, Var, VarId};
use std::fmt::Write as _;

pub fn c_type(t: Ty) -> &'static str {
    match t {
        Ty::I16 => "int16",
        Ty::I32 => "int32",
        Ty::I64 => "int64",
        Ty::F32 => "float",
        Ty::F64 => "double",
        Ty::F80 => "long double",
        Ty::Str => "qbs*",
        Ty::User(_) => unreachable!("no value has a user type"),
    }
}

/// The type part of a C variable name.
fn type_word(t: Ty) -> &'static str {
    match t {
        Ty::I16 => "INTEGER",
        Ty::I32 => "LONG",
        Ty::I64 => "INTEGER64",
        Ty::F32 => "SINGLE",
        Ty::F64 => "DOUBLE",
        Ty::F80 => "FLOAT",
        Ty::Str => "STRING",
        Ty::User(_) => "UDT",
    }
}

/// A BASIC name as part of a C identifier: a `.` becomes `__046__`.
fn c_ident(name: &str) -> String {
    name.replace('.', "__046__")
}

/// `SUB_<NAME>` or `FUNC_<NAME>`.
pub fn proc_name(proc: &Proc) -> String {
    let prefix = match proc.kind {
        ProcKind::Sub => "SUB",
        ProcKind::Function(_) => "FUNC",
    };
    format!("{prefix}_{}", c_ident(&proc.name))
}

/// The C++ label of a body's label: `LABEL_<NAME>` for a user label, `L_<n>` (its index) for one the lowering made.
fn label_name(b: &Body, l: LabelId) -> String {
    match &b.labels[l.0 as usize].name {
        Some(name) => format!("LABEL_{}", c_ident(name)),
        None => format!("L_{}", l.0),
    }
}

/// `__<TYPE>_<NAME>` for a global variable; `_<SUB_P>_<TYPE>_<NAME>` for any variable of procedure P;
/// `temp_<name>` for a temporary of the lowering (`for1.value` is `temp_for1_value`).
pub fn var_name(p: &Program, v: &Var) -> String {
    let scope = match v.storage {
        Storage::Global => String::new(),
        Storage::Static(q) | Storage::Local(q) | Storage::Param(q) | Storage::Result(q) => proc_name(p.proc(q)),
        Storage::Temp(_) => return format!("temp_{}", v.name.replace('.', "_")),
    };
    let array = if v.is_array() { "ARRAY_" } else { "" };
    format!("_{scope}_{array}{}_{}", type_word(v.ty), c_ident(&v.name))
}

/// A C string literal for arbitrary bytes: printable ASCII as is, everything else (and `"`, `\`, `?`) as a
/// three-digit octal escape, so no byte value can end an escape early.
pub(crate) fn c_string(bytes: &[u8]) -> String {
    let mut s = String::from("\"");
    for &b in bytes {
        if (0x20..0x7f).contains(&b) && !matches!(b, b'"' | b'\\' | b'?') {
            s.push(b as char);
        } else {
            write!(s, "\\{b:03o}").unwrap();
        }
    }
    s.push('"');
    s
}

/// The file name of a `#line` directive. Its string is unevaluated, where clang rejects numeric escapes (`\134`),
/// so only `\\` and `\"` are escaped; other bytes stay as they are (the name is UTF-8), control bytes become `_`.
pub(crate) fn line_name(name: &str) -> String {
    let mut s = String::from("\"");
    for c in name.chars() {
        match c {
            '\\' | '"' => {
                s.push('\\');
                s.push(c);
            }
            c if c.is_control() => s.push('_'),
            c => s.push(c),
        }
    }
    s.push('"');
    s
}

/// An integer constant of a type; negative ones in parentheses, the minimum values spelled without overflow.
pub(crate) fn int_const(i: i64, ty: Ty) -> String {
    let text = match (ty, i) {
        (Ty::I64, i64::MIN) => "(-9223372036854775807ll-1)".to_string(),
        (Ty::I64, _) => format!("{i}ll"),
        (_, -2147483648) => "(-2147483647-1)".to_string(),
        _ => i.to_string(),
    };
    if i < 0 && !text.starts_with('(') {
        format!("({text})")
    } else {
        text
    }
}

impl Emitter<'_> {
    /// `LABEL_<NAME>` of a main-module label.
    pub(crate) fn label_name(&self, l: LabelId) -> String {
        label_name(&self.p.main, l)
    }

    /// The C++ label of a label of the body being emitted.
    pub(crate) fn local_label(&self, l: LabelId) -> String {
        label_name(self.body, l)
    }

    pub(crate) fn name(&self, id: VarId) -> String {
        var_name(self.p, self.p.var(id))
    }

    /// Each line preceded by a `#line` directive for source line `line` of the main file.
    pub(crate) fn lines(&self, line: u32, lines: &[String], out: &mut String) {
        self.lines_in(None, line, lines, out);
    }

    /// [`Self::lines`] for a line of `file` (`None`, or a file not included: the main file).
    pub(crate) fn lines_in(&self, file: Option<FileId>, line: u32, lines: &[String], out: &mut String) {
        let name = file
            .and_then(|f| self.included.get(&f))
            .map_or(&self.line_file, |(n, _)| n);
        for l in lines {
            writeln!(out, "#line {line} {name}\n{l}").unwrap();
        }
    }

    /// The arguments of `evnt` for source line `line` of `file`: the line, and for an included file the line
    /// again and the file's name. (The old compiler's first argument is the main file's line there; it only feeds
    /// `_ERRORLINE`, not supported yet.)
    pub(crate) fn evnt_args(&self, file: Option<FileId>, line: u32) -> String {
        match file.and_then(|f| self.included.get(&f)) {
            Some((_, base)) => format!("{line},{line},{base}"),
            None => line.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qb64rust_ir::ProcId;

    #[test]
    fn strings_escape_every_non_printable_byte() {
        assert_eq!(c_string(b"a\"\\?\x00\xC9"), "\"a\\042\\134\\077\\000\\311\"");
    }

    /// clang rejects numeric escapes in a `#line` file name; an absolute Windows path must still work.
    #[test]
    fn line_names_use_simple_escapes_only() {
        assert_eq!(
            line_name("C:\\does\\not\\exist\\p q.bas"),
            r#""C:\\does\\not\\exist\\p q.bas""#
        );
        assert_eq!(line_name("a\"b\u{1}\u{e9}.bas"), "\"a\\\"b_\u{e9}.bas\"");
    }

    #[test]
    fn int_constants() {
        assert_eq!(int_const(-5, Ty::I16), "(-5)");
        assert_eq!(int_const(i32::MIN as i64, Ty::I32), "(-2147483647-1)");
        assert_eq!(int_const(i64::MIN, Ty::I64), "(-9223372036854775807ll-1)");
        assert_eq!(int_const(7, Ty::I64), "7ll");
    }

    #[test]
    fn var_names() {
        let var = |name: &str, ty, storage| Var {
            name: name.into(),
            ty,
            storage,
            dims: Vec::new(),
        };
        let mut p = Program::default();
        for (name, kind) in [("BUMP", ProcKind::Sub), ("TWICE", ProcKind::Function(Ty::I32))] {
            p.procs.push(Proc {
                name: name.into(),
                kind,
                params: Vec::new(),
                result: None,
                body: Body::default(),
                line: 1,
                end_line: 1,
            });
        }
        assert_eq!(var_name(&p, &var("A", Ty::I32, Storage::Global)), "__LONG_A");
        assert_eq!(var_name(&p, &var("S", Ty::Str, Storage::Global)), "__STRING_S");
        assert_eq!(
            var_name(&p, &var("X", Ty::I32, Storage::Param(ProcId(0)))),
            "_SUB_BUMP_LONG_X"
        );
        assert_eq!(
            var_name(&p, &var("C", Ty::F32, Storage::Static(ProcId(0)))),
            "_SUB_BUMP_SINGLE_C"
        );
        assert_eq!(
            var_name(&p, &var("TWICE", Ty::I32, Storage::Result(ProcId(1)))),
            "_FUNC_TWICE_LONG_TWICE"
        );
        assert_eq!(
            var_name(&p, &var("A.B", Ty::I16, Storage::Local(ProcId(1)))),
            "_FUNC_TWICE_INTEGER_A__046__B"
        );
    }
}
