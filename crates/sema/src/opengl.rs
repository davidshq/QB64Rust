//! The OpenGL names of a program with `SUB _GL` (design D2 and D3 of `m4-sub-gl`): lookups in the table that
//! `tools\opengl\gl_table.py` writes from `libqb\`'s `gl.h` ([`crate::gl_table`]), and the tier 1 tests that hold
//! the table to `libqb\` and to the call-site programs.

use crate::gl_table::{CONSTANTS, GlConst, GlRoutine, ROUTINES};
use std::cmp::Ordering;

/// The table's order: by upper-case name.
fn by_upper(entry: &str, name: &str) -> Ordering {
    entry.bytes().map(|b| b.to_ascii_uppercase()).cmp(name.bytes())
}

/// The constant named `name` (upper case, without suffix: `_GL_COLOR_BUFFER_BIT`).
pub fn gl_constant(name: &str) -> Option<&'static GlConst> {
    CONSTANTS
        .binary_search_by(|c| by_upper(c.name, name))
        .ok()
        .map(|i| &CONSTANTS[i])
}

/// The routine named `name` (upper case, without suffix: `_GLVIEWPORT`).
pub fn gl_routine(name: &str) -> Option<&'static GlRoutine> {
    ROUTINES
        .binary_search_by(|r| by_upper(r.name, name))
        .ok()
        .map(|i| &ROUTINES[i])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ty;
    use std::collections::HashMap;

    const GL_DIR: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../libqb/internal/c/parts/core/gl_header_for_parsing"
    );

    #[expect(clippy::disallowed_methods, reason = "C headers of libqb, ASCII, not BASIC source")]
    fn read(path: &str) -> String {
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"))
    }

    #[test]
    fn lookups_ignore_letter_case() {
        assert_eq!(gl_constant("_GL_COLOR_BUFFER_BIT").map(|c| c.value), Some(16384));
        assert_eq!(gl_routine("_GLVIEWPORT").map(|r| r.c_name), Some("call_glViewport"));
        assert_eq!(gl_routine("_GLUPERSPECTIVE").map(|r| r.c_name), Some("gluPerspective"));
        assert!(gl_routine("_GLULOOKAT").is_none() && gl_constant("_GL_EXT_BGRA").is_none());
    }

    /// The checker names a routine's parameters `A` to `Z`, as the helpers name theirs (`check/opengl.rs`).
    #[test]
    fn no_routine_takes_more_than_26_arguments() {
        for r in &ROUTINES {
            assert!(r.args.len() <= 26, "{} takes {} arguments", r.name, r.args.len());
        }
    }

    /// The constants are the `#define` lines of `gl.h` the old compiler takes (`gl_scan_header`): a name of
    /// upper-case letters, digits and `_`, a hex or decimal value or the name of an earlier one; with `_` before it.
    #[test]
    fn the_constants_are_gl_h_s() {
        let mut want: Vec<(String, i64)> = Vec::new();
        for line in read(&format!("{GL_DIR}/gl.h")).lines() {
            let mut words = line.split_whitespace();
            let (Some("#define"), Some(name), Some(value), None) =
                (words.next(), words.next(), words.next(), words.next())
            else {
                continue;
            };
            if !name
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
            {
                continue;
            }
            let value = if let Some(hex) = value.strip_prefix("0x") {
                i64::from_str_radix(hex, 16).unwrap_or_else(|_| panic!("{line}"))
            } else if value.starts_with(|c: char| c.is_ascii_digit()) {
                value.parse().unwrap_or_else(|_| panic!("{line}"))
            } else {
                let earlier = want.iter().find(|(n, _)| n[1..] == *value);
                earlier.unwrap_or_else(|| panic!("{line}: names no earlier constant")).1
            };
            want.push((format!("_{name}"), value));
        }
        for (name, value) in &want {
            let got = gl_constant(&name.to_ascii_uppercase()).map(|c| (c.name, c.value));
            assert_eq!(got, Some((name.as_str(), *value)), "the constant {name} of gl.h");
        }
        let names: Vec<&str> = want.iter().map(|(n, _)| n.as_str()).collect();
        for c in &CONSTANTS {
            assert!(names.contains(&c.name), "{} is in the table, not in gl.h", c.name);
        }
        assert_eq!(CONSTANTS.len(), 585);
    }

    /// The C type of a helper's parameter or result (`gl2qb_type_convert`'s `ctyp$`).
    fn c_type(t: Ty) -> &'static str {
        match t {
            Ty::U32 => "uint32",
            Ty::I32 => "int32",
            Ty::U8 => "uint8",
            Ty::I8 => "int8",
            Ty::U16 => "uint16",
            Ty::I16 => "int16",
            Ty::F32 => "float",
            Ty::F64 => "double",
            Ty::Off | Ty::Str => "ptrszint",
            Ty::I64 | Ty::U64 | Ty::UOff | Ty::Bit { .. } | Ty::F80 | Ty::FixedStr(_) | Ty::User(_) => {
                panic!("no OpenGL type: {t:?}")
            }
        }
    }

    /// Each routine with a helper is called as `gl_helper_code.h` (the old compiler's output, copied into
    /// `libqb\`) declares it: the name, the result's C type, each parameter's; every helper is in the table; and
    /// each routine without one is a `gl.h` function with no parameter and no pointer result.
    #[test]
    fn the_routines_match_the_helpers() {
        let mut helpers: HashMap<String, (String, Vec<String>)> = HashMap::new();
        for line in read(&format!("{GL_DIR}/temp/gl_helper_code.h")).lines() {
            let Some(head) = line.strip_suffix("){") else {
                continue;
            };
            let (ret_name, params) = head.split_once('(').expect("a helper's head");
            let (ret, name) = ret_name.split_once(' ').expect("a result type and a name");
            let params = params
                .split(',')
                .filter(|p| !p.is_empty())
                .map(|p| p.split_once(' ').expect("a parameter").0.to_string())
                .collect();
            helpers.insert(name.to_string(), (ret.to_string(), params));
        }
        let gl_h = read(&format!("{GL_DIR}/gl.h"));
        for r in &ROUTINES {
            if let Some((ret, params)) = helpers.get(r.c_name) {
                let want_ret = r.ret.map_or("void", c_type);
                let want: Vec<&str> = r.args.iter().map(|&t| c_type(t)).collect();
                assert_eq!(
                    (ret.as_str(), params.iter().map(String::as_str).collect::<Vec<_>>()),
                    (want_ret, want),
                    "{}",
                    r.name
                );
                assert_eq!(r.c_name, format!("call_{}", &r.name[1..]), "{}", r.name);
            } else if r.name == "_gluPerspective" {
                assert_eq!((r.c_name, r.ret, r.args), ("gluPerspective", None, &[Ty::F64; 4][..]));
            } else {
                assert!(r.args.is_empty(), "{} has arguments but no helper", r.name);
                let decl = format!(" APIENTRY {}(void);", r.c_name);
                assert!(
                    gl_h.lines().any(|l| l.starts_with("WINGDIAPI ") && l.ends_with(&decl)),
                    "{} is no function of gl.h",
                    r.name
                );
            }
        }
        for name in helpers.keys() {
            assert!(
                ROUTINES.iter().any(|r| r.c_name == name),
                "the helper {name} has no routine in the table"
            );
        }
        let declared = gl_h
            .lines()
            .filter(|l| l.starts_with("WINGDIAPI ") && l.contains("APIENTRY ") && l.ends_with(';'))
            .count();
        assert_eq!(ROUTINES.len(), declared + 1, "the routines of gl.h and _gluPerspective");
        assert_eq!(ROUTINES.len(), 337);
    }

    /// Every routine has its call-site program (delta spec `testing/compiler-tests`).
    #[test]
    fn every_routine_has_a_call_site_program() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/callsite");
        for r in &ROUTINES {
            let path = format!("{dir}/gl{}.bas", r.name);
            assert!(
                std::path::Path::new(&path).is_file(),
                "{} has no call-site program (gl{}.bas)",
                r.name,
                r.name
            );
        }
    }
}
