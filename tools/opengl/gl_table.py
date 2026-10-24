"""The OpenGL table of the new compiler (OpenSpec change m4-sub-gl, design D2 and D5).

Reads libqb's copy of gl.h with the old compiler's rules (source/subs_functions/extensions/opengl/
opengl_methods.bas: gl_scan_header, gl2qb_type_convert, gl_include_content) and writes

- crates/sema/src/gl_table.rs: the constants (name, value) and the routines (name, C name, result
  type, argument types), each list in order of upper-case name;
- tests/callsite/gl_<name>.bas: one call-site program per routine.

  python tools/opengl/gl_table.py           # write both
  python tools/opengl/gl_table.py --check   # say whether the files are what it writes (exit 1 if not)

--check also rebuilds the helper header the old compiler writes from gl.h and compares it with libqb's
copy of it (gl_helper_code.h), so a reading of gl.h that is no longer the old compiler's is found.
"""

from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
GL_DIR = REPO / "libqb" / "internal" / "c" / "parts" / "core" / "gl_header_for_parsing"
GL_H = GL_DIR / "gl.h"
HELPER_H = GL_DIR / "temp" / "gl_helper_code.h"
TABLE_RS = REPO / "crates" / "sema" / "src" / "gl_table.rs"
CALLSITE = REPO / "tests" / "callsite"

# The marker of a generated call-site program: only files that carry it are rewritten or removed.
GENERATED = "' Written by tools/opengl/gl_table.py from libqb's gl.h; not edited by hand."

# gl2qb_type_convert: the C type, the type in the table (a variant of sema's Ty), the BASIC type, the C type of
# the helper's parameter.
GL_TYPES = {
    "GLenum": ("U32", "_UNSIGNED LONG", "uint32"),
    "GLbitfield": ("U32", "_UNSIGNED LONG", "uint32"),
    "GLuint": ("U32", "_UNSIGNED LONG", "uint32"),
    "GLint": ("I32", "LONG", "int32"),
    "GLsizei": ("I32", "LONG", "int32"),
    "GLboolean": ("U8", "_UNSIGNED _BYTE", "uint8"),
    "GLubyte": ("U8", "_UNSIGNED _BYTE", "uint8"),
    "GLbyte": ("I8", "_BYTE", "int8"),
    "GLushort": ("U16", "_UNSIGNED INTEGER", "uint16"),
    "GLshort": ("I16", "INTEGER", "int16"),
    "GLfloat": ("F32", "SINGLE", "float"),
    "GLclampf": ("F32", "SINGLE", "float"),
    "GLdouble": ("F64", "DOUBLE", "double"),
    "GLclampd": ("F64", "DOUBLE", "double"),
    "GLvoid": ("Off", "_OFFSET", "ptrszint"),
}
OFFSET = ("Off", "_OFFSET", "ptrszint")
BASIC_TYPE = {ty: basic for ty, basic, _ in [*GL_TYPES.values(), OFFSET]}
BASIC_TYPE["Str"] = "STRING"


class GlError(Exception):
    """gl.h holds something the old compiler's reading would stop at or this tool does not read."""


@dataclass
class Routine:
    name: str  # as the old compiler registers it: "_" + the C name
    c_name: str  # what a call names: the helper call_<name>, or the function itself
    ret: str | None  # the result's type (a Ty variant), None for a SUB
    args: list[str]  # the arguments' types (Ty variants)
    helper: str | None  # the helper's text as the old compiler writes it, None without one


def convert(c_type: str) -> tuple[str, str, str]:
    if c_type not in GL_TYPES:
        # The old compiler prints "Unknown type:" and ends.
        raise GlError(f"unknown type {c_type!r}")
    return GL_TYPES[c_type]


def readchunk(a: str) -> tuple[str, str, str]:
    """readchunk$: (chunk, last character, rest). The text is trimmed of spaces first; a chunk ends at a
    space, a comma or a parenthesis, and the rest is trimmed of leading spaces."""
    a = a.strip(" ")
    for x, c in enumerate(a):
        if c in " ,()":
            return a[:x], c, a[x + 1 :].lstrip(" ")
    return a, "", ""


def define_name_ok(name: str) -> bool:
    """A #define's name the old compiler takes: upper-case letters, '_', digits but not first. (An empty
    name passes, as the old loop over its characters does nothing.)"""
    return all(c.isupper() and c.isascii() or c == "_" or c.isdigit() and i > 0 for i, c in enumerate(name))


def read_define(line: str, defines: list[tuple[str, int]]) -> None:
    """The #define part of gl_scan_header for one trimmed line that starts with "#define "."""
    a = line + " "
    a2 = ""
    for x, c in enumerate(a):
        if c != " ":
            a2 += c
            continue
        if not define_name_ok(a2):
            a2 = ""
            continue
        value = a[x + 1 :].strip(" ")
        if not value:
            a2 = ""
            continue
        if value.startswith("0x"):
            digits = value[2:]
            if not digits or any(d not in "0123456789abcdefABCDEF" for d in digits):
                raise GlError(f"a hex value this tool does not read: {line!r}")
            v = int(digits, 16)
            defines.append((a2, v - (1 << 64) if v >= 1 << 63 else v))
        elif value[0].isdigit():
            if not value.isdigit():
                raise GlError(f"a value this tool does not read: {line!r}")
            defines.append((a2, int(value)))
        else:
            for name, v in defines:
                if name == value:
                    defines.append((a2, v))
                    break
        return


def read_routine(line: str) -> Routine | None:
    """The routine part of gl_scan_header for one trimmed line that ends with ";"."""
    a2, _, a = readchunk(line)
    if a2 != "WINGDIAPI":
        return None
    ret_type, _, a = readchunk(a)
    if ret_type == "const":
        ret_type, _, a = readchunk(a)
    is_func = ret_type != "void"
    a2, _, a = readchunk(a)
    if a2 == "*APIENTRY":
        ret_type += "*"
        a2 = "APIENTRY"
    if a2 != "APIENTRY":
        return None
    proc_name, _, a = readchunk(a)
    hc = ""
    hd = ""
    helper = False
    ret = None
    if is_func:
        if ret_type.endswith("*"):
            ret, ctyp = "Off", "ptrszint"
            hd += "ptrszint "
            helper = True
        else:
            ret, _, ctyp = convert(ret_type)
            hd += ctyp + " "
        hc += f"return ({ctyp})({ret_type})"
    else:
        hd += "void "
    hc += proc_name + "("
    hd += "call_" + proc_name + "("
    args: list[str] = []
    while True:
        var_type, last, a = readchunk(a)
        if var_type == "" and last == "(":
            var_type, last, a = readchunk(a)
        if var_type == "const":
            var_type, last, a = readchunk(a)
        if var_type in ("void", ""):
            break
        if last not in (",", ")"):
            var_name, last, a = readchunk(a)
            for _ in range(2):
                if var_name.startswith("*"):
                    var_type += "*"
                    var_name = var_name[1:]
        typed = var_type
        pointer = 0
        for _ in range(2):
            if var_type.endswith("*"):
                var_type = var_type[:-1]
                pointer += 1
        if pointer == 2:
            ty, ctyp = "Off", "ptrszint"
        else:
            ty, _, ctyp = convert(var_type)
            if pointer == 1:
                ty, ctyp = "Off", "ptrszint"
        helper = True
        args.append(ty)
        letter = chr(96 + len(args))
        hc += f"({typed}){letter}"
        hd += f"{ctyp} {letter}"
        if last != ")":
            hc += ","
            hd += ","
        if last == ")":
            break
    hd += "){"
    hc += ");"
    text = f"{hd}\nif (!sub_gl_called) error(270);\n{hc}\n}}\n" if helper else None
    c_name = "call_" + proc_name if helper else proc_name
    if proc_name == "glGetString":
        ret = "Str"
    return Routine("_" + proc_name, c_name, ret, args, text)


def scan(gl_h: str) -> tuple[list[tuple[str, int]], list[Routine]]:
    """gl_scan_header and gl_include_content: the constants with their leading "_", the routines, and
    _gluPerspective, in the order the old compiler registers them."""
    defines: list[tuple[str, int]] = []
    routines: list[Routine] = []
    for raw in gl_h.splitlines():
        if not raw:
            continue
        line = raw.strip(" ")
        if line.startswith("#define "):
            read_define(line, defines)
        if line.endswith(";"):
            r = read_routine(line)
            if r is not None:
                routines.append(r)
    constants = [(n if n.startswith("_") else "_" + n, v) for n, v in defines]
    routines.append(Routine("_gluPerspective", "gluPerspective", None, ["F64"] * 4, None))
    return constants, routines


def unique(names: list[str], what: str) -> None:
    seen: set[str] = set()
    for n in names:
        if n.upper() in seen:
            # The old compiler would register both; which one a program reaches is not measured.
            raise GlError(f"two {what} named {n!r}")
        seen.add(n.upper())


def rust_table(constants: list[tuple[str, int]], routines: list[Routine]) -> str:
    out = [
        "//! The OpenGL constants and routines of a program with `SUB _GL` (design D2 of `m4-sub-gl`), as the old",
        "//! compiler reads them from its copy of `gl.h` (`opengl_methods.bas`: `gl_scan_header`, `gl_include_content`).",
        "//! Written by `tools\\opengl\\gl_table.py` from",
        "//! `libqb\\internal\\c\\parts\\core\\gl_header_for_parsing\\gl.h`; not edited by hand. Each list is in order of",
        "//! upper-case name, for [`slice::binary_search_by`].",
        "",
        "use crate::Ty;",
        "",
        "/// An OpenGL constant: `GL_COLOR_BUFFER_BIT` as `_GL_COLOR_BUFFER_BIT`, an `_INTEGER64`.",
        "#[derive(Debug)]",
        "pub struct GlConst {",
        "    pub name: &'static str,",
        "    pub value: i64,",
        "}",
        "",
        "/// An OpenGL routine: `glViewport` as `_glViewport`, every argument `BYVAL`.",
        "#[derive(Debug)]",
        "pub struct GlRoutine {",
        "    /// The name as the old compiler registers it.",
        "    pub name: &'static str,",
        "    /// The C function a call names: the helper of `gl_helper_code.h` (`call_glViewport`), which raises error 270",
        "    /// outside `SUB _GL`, or for a routine without one the function itself (`glFlush`, `gluPerspective`).",
        "    pub c_name: &'static str,",
        "    /// The result's type; `None` for a SUB. `_glGetString`'s is `Ty::Str`.",
        "    pub ret: Option<Ty>,",
        "    /// The arguments' types; a pointer is an `_OFFSET`.",
        "    pub args: &'static [Ty],",
        "}",
        "",
        f"pub static CONSTANTS: [GlConst; {len(constants)}] = [",
    ]
    for name, value in sorted(constants, key=lambda c: c[0].upper()):
        out.append(f'    GlConst {{ name: "{name}", value: {value} }},')
    out += ["];", "", f"pub static ROUTINES: [GlRoutine; {len(routines)}] = ["]
    for r in sorted(routines, key=lambda r: r.name.upper()):
        ret = "None" if r.ret is None else f"Some(Ty::{r.ret})"
        args = ", ".join(f"Ty::{a}" for a in r.args)
        out += [
            "    GlRoutine {",
            f'        name: "{r.name}",',
            f'        c_name: "{r.c_name}",',
            f"        ret: {ret},",
            f"        args: &[{args}],",
            "    },",
        ]
    out.append("];")
    return "\n".join(out) + "\n"


def callsite_program(r: Routine) -> str:
    names = [chr(97 + i) for i in range(len(r.args))]
    decls = [f"{n} AS {BASIC_TYPE[t]}" for n, t in zip(names, r.args)]
    call_args = ", ".join(names)
    if r.ret is None:
        call = f"{r.name} {call_args}".rstrip()
    else:
        decls.append(f"res AS {BASIC_TYPE[r.ret]}")
        call = f"res = {r.name}({call_args})" if r.args else f"res = {r.name}"
    lines = [
        "$CONSOLE:ONLY",
        "' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.",
        GENERATED,
    ]
    if decls:
        lines.append("DIM " + ", ".join(decls))
    lines += [call, "END", "", "SUB _GL", "END SUB"]
    return "\n".join(lines) + "\n"


def outputs(gl_h: str) -> dict[Path, str]:
    constants, routines = scan(gl_h)
    unique([c[0] for c in constants], "constants")
    unique([r.name for r in routines], "routines")
    files = {TABLE_RS: rust_table(constants, routines)}
    for r in routines:
        files[CALLSITE / f"gl{r.name}.bas"] = callsite_program(r)
    return files


def helper_code(gl_h: str) -> str:
    """gl_helper_code.h as the old compiler writes it: the helpers in gl.h's order, then PRINT's line end."""
    _, routines = scan(gl_h)
    return "".join(r.helper.replace("\n", "\r\n") for r in routines if r.helper) + "\r\n"


def generated_programs() -> set[Path]:
    return {
        p for p in CALLSITE.glob("gl_*.bas") if GENERATED in p.read_text(encoding="latin-1").splitlines()
    }


def read(path: Path) -> str | None:
    return path.read_text(encoding="utf-8").replace("\r\n", "\n") if path.is_file() else None


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true", help="compare the files with what would be written")
    args = ap.parse_args(argv)
    gl_h = GL_H.read_text(encoding="latin-1")
    try:
        files = outputs(gl_h)
        helpers = helper_code(gl_h)
    except GlError as e:
        print(f"gl.h: {e}")
        return 1
    stale = generated_programs() - set(files)
    if args.check:
        problems = [str(p.relative_to(REPO)) for p, text in files.items() if read(p) != text]
        problems += [f"{p.relative_to(REPO)} (no routine of gl.h)" for p in sorted(stale)]
        if HELPER_H.read_bytes().decode("latin-1") != helpers:
            problems.append(f"{HELPER_H.relative_to(REPO)} is not what gl.h's reading gives")
        for p in problems:
            print("differs:", p)
        print(f"{len(files) - 1} call-site programs and the table: " + ("ok" if not problems else "differ"))
        return 1 if problems else 0
    if HELPER_H.read_bytes().decode("latin-1") != helpers:
        print(f"warning: {HELPER_H.relative_to(REPO)} is not what gl.h's reading gives")
    for p, text in files.items():
        if read(p) != text:
            p.write_text(text, encoding="utf-8", newline="\n")
            print("written:", p.relative_to(REPO))
    for p in sorted(stale):
        p.unlink()
        print("removed:", p.relative_to(REPO))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
