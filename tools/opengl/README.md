# The OpenGL table

`gl_table.py` writes what the new compiler knows of OpenGL (OpenSpec change `m4-sub-gl`, design D2 and D5) from
libqb's copy of `gl.h` (`libqb\internal\c\parts\core\gl_header_for_parsing\gl.h`), read with the old compiler's
rules (`source\subs_functions\extensions\opengl\opengl_methods.bas`: `gl_scan_header`, `gl2qb_type_convert`,
`gl_include_content`):

- `crates\sema\src\gl_table.rs`: the constants (`GL_COLOR_BUFFER_BIT` as `_GL_COLOR_BUFFER_BIT`, its value; an
  `_INTEGER64` in a program) and the routines (`glViewport` as `_glViewport`; the C function a call names, the
  result's type, the arguments' types), each list in order of upper-case name. `_gluPerspective`, which the old
  compiler adds by hand, is the last routine read.
- `tests\callsite\gl_<name>.bas`: one call-site program per routine (`tools\callsite\README.md`), with a `SUB _GL`
  so that the names exist and the call in the main module (never run). Only files with the tool's marker line are
  rewritten or removed.

```
python tools\opengl\gl_table.py            # write the table and the programs
python tools\opengl\gl_table.py --check    # exit 1 and name each file that differs
python -m unittest discover -s tools/opengl -p "test_*.py"
```

`--check` also rebuilds, from the same reading, the helper header the old compiler writes on each compile
(`temp\gl_helper_code.h` beside `gl.h`, copied into `libqb\` byte for byte) and compares it with libqb's copy: a
reading that is no longer the old compiler's is found there, before any call differs. A new reference commit that
changes `gl.h` makes `--check` (and tier 1: `sema`'s test of the table against `libqb\`) fail; run the tool again.

The rules, in short: a `#define` whose name has only upper-case letters, digits (not first) and `_`, with a hex or
decimal value or the name of an earlier one (a later or unknown name is dropped, as are lower-case names such as
`GL_EXT_bgra`); a `WINGDIAPI <type> APIENTRY <name>(…);` line, `const` skipped, a pointer (or a pointer to one) an
`_OFFSET`, `GLvoid` an `_OFFSET`; a routine with an argument or a pointer result is called through its helper
`call_<name>`, which raises error 270 outside `SUB _GL`, any other through the C function itself; `glGetString`'s
pointer is read as a `STRING`. A type the old compiler does not know stops it ("Unknown type"); it stops this tool
too, as does a value it does not read.
