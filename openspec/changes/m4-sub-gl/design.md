# Design

## Context

For the motivation and the old compiler's handling, see `proposal.md` ("Why"); for the behaviour, the delta specs.
How the parts stand in this compiler (2026-10-24, on `m4-sub-gl` at the tree of `main`'s merge of pull request 14):

- **`SUB _GL` is marked.** `sema\src\check\decl.rs` `reserved_proc` returns "not supported yet: `SUB _GL`
  (OpenGL)" for a SUB of that name; `check\proc.rs` marks a statement with a leading underscore that the built-in
  table does not hold (the `_gl…` statements among them, `SOMEDAY.md`'s row on it). A FUNCTION named `_GL` takes
  the ordinary reserved-name rules.
- **What is already there.** `SYSTEM` writes `if (sub_gl_called) error(271);` (`codegen-cpp\src\stmt.rs`); `END`
  calls libqb's `sub_end`, which tests it itself; `STOP` does not (`m4-qb64pe-sources` D5). `_GLRENDER` is a plain
  statement (`m4-builtins-remaining`).
- **Procedures of a `DECLARE LIBRARY` block** (`m3-declare-library` D1, D3) are `Proc`s with an `External`
  (`sema\src\lib.rs`): no statements, a C name, a `BYVAL` per parameter; `codegen-cpp\src\library.rs` writes their
  calls. The old compiler registers the OpenGL routines the same way (`id.ccall = 1`, `opengl_methods.bas` 380).
- **The runtime parts** a program needs are `Settings::runtime_parts` (`RuntimePart`, `sema\src\lib.rs`), turned
  into make flags by `driver\src\build.rs` `make_flags` in the old compiler's order (`qb64pe.bas` 13706–13721);
  `DEP_GL=y` is first in that order. The `Makefile` names libqb's object set by its flags (`QBLIB_NAME`, line 332),
  so a GL build's objects stand beside the others.
- **libqb already has the helpers.** `libqb\internal\c\parts\core\gl_header_for_parsing\temp\gl_helper_code.h` is
  tracked in the clone and copied byte for byte; `qbx.cpp` includes it when `QB64_GUI` and `DEPENDENCY_GL` are
  defined. The old compiler rewrites that file on each compile from `gl.h` (`gl_scan_header`); its copy in the
  clone is that output, so the new compiler need not write it (checked by D2's tier 1 test).
- **Window programs of the verification suite** run on a desktop that is never shown (`press_any_key.py
  --hidden-desktop`); both compilers' programs run there.

## Goals / Non-Goals

**Goals:**

- `opengl/test` passes, and no program that the old compiler accepts with `SUB _GL` is marked.
- Every OpenGL routine's call is compared with the old compiler's; some of them are run.

**Non-Goals:**

- Comparing what OpenGL draws beyond values exact on every driver (a cleared colour, the viewport, an enabled
  flag, an error code). A rasterised shape depends on the driver; working rule 12 applies: no row of
  `DIVERGENCES.md` is needed, since the new compiler's program makes the same calls.
- `gl.h` names the old compiler's parser misses (the readme of its folder invites additions): the table is what
  the old compiler reads, no more.
- `$NOPREFIX` forms (`glViewport` without the underscore): `$NOPREFIX` is the old compiler's error since
  `m4-statements-metacommands`.

## Decisions

### D1. Measure first: `verification\v59_*`

Before the code, a family recorded with `qb64pe.exe`: `a` the forms of `SUB _GL` (plain, `STATIC`, with a
parameter, as a FUNCTION, twice, after a use, in an included file, in a `$CONSOLE:ONLY` program, which may fail
only in the C++ build), `b` the names without `SUB _GL` (a statement, a function, a constant), `c` the constants
(value, type by `LEN` and by a suffix, `_GL_…&&`), `d` the conversions of the argument types (a `SINGLE` into a
`GLenum`, a negative into a `GLuint`, a string), `e` `_glGetString` and a FUNCTION's result, `f` errors 270 and
271 (handled and not), `g` reading back inside `SUB _GL` (cleared colour, viewport, `_glIsEnabled`,
`_glGetError`), `h` the frame count with `_GLRENDER _BEHIND`, `_ONLY` and `_ONTOP`, `x…` each rejected form with
its message. The C++ of the accepted forms is read from `qb64pe -z` before D3 and D4 are written. A program that
needs the window prints with `_DEST _CONSOLE` under `$CONSOLE`, as `v53` does.

*Alternative:* write from the reading of `qb64pe.bas` alone. Rejected: every change so far found forms the reading
did not predict (the project's rule since M2).

**What the recordings showed** (task 1.1; `verification\README.md`, `v59*`), where the reading did not predict it:
errors 270 and 271 are libqb's critical errors (`error_handle.cpp`: a message, `libqb_exit`), so no `ON ERROR`
handler runs, and a line printed just after the call races the program's end (the `f` programs wait 2 s after it);
a routine without a helper (`_glFlush`, `_glGetError`) raises nothing outside `SUB _GL`; `STOP` inside it hangs the
program; under `$CONSOLE:ONLY` and `$SCREENHIDE` `SUB _GL` compiles and is never called; `SUB _GL` with a parameter,
`FUNCTION _GL` and a second `SUB _GL` are "Invalid name"; a `CONST` naming an OpenGL constant is "Unexpected
element" (the constants are registered after the pre-pass, which reads `CONST` lines); `CALL _glViewport(…)`, a
`LONG` and a whole array where an `_OFFSET` goes, and `_GL_TRUE&` are accepted.

### D2. The OpenGL table: generated from `libqb\`'s `gl.h`, checked in tier 1

`tools\opengl\gl_table.py` reads `libqb\internal\c\parts\core\gl_header_for_parsing\gl.h` with the old compiler's
rules (`gl_scan_header`: a `#define` of an upper-case name with a number or an earlier name; a `WINGDIAPI … APIENTRY`
line; `gl2qb_type_convert` for the types, every pointer an `_OFFSET`; a helper `call_…` when the routine has an
argument or returns a pointer; `glGetString`'s result a `STRING`) and writes `crates\sema\src\gl_table.rs`: the
constants (name, value) and the routines (name, SUB or FUNCTION with its type, C name, argument types), plus
`_gluPerspective` (`gl_include_content`). `--check` says whether the file is what it writes. A tier 1 test in
`sema` holds the table to `libqb\`: the counts (337 routines; the constants as the old compiler's rules count the 592 `#define` lines), and each routine with a helper against its line in
`gl_helper_code.h` (the name `call_…`, the C types of its arguments, its return type). Its own tests are
`tools\opengl\test_gl_table.py`.

*Alternatives:* (a) read `gl.h` at compile time, as the old compiler does: `sema` reads no file (its rule since
M2), and the front end without a `libqb\` (the extension's language server) would lose the names; (b) a
build script of `sema` reading `libqb\`: ties the crate's build to a folder outside it, for a file that changes
only with a new reference commit, when `copy_libqb.py --check` and the tier 1 test already catch that change.

### D3. `SUB _GL` and the names: known only when the program has it

`reserved_proc` (`check\decl.rs`) accepts `SUB _GL` without parameters and records it in the program's settings
(`RuntimePart::Gl`, which the checker's `has_gl()` asks); the other forms follow `v59_a` and `x…`. The pre-pass that collects
procedure headers already runs before the bodies, so a use of an OpenGL name before the SUB's line sees it, as in
the old compiler, where `UseGL` is set while the headers are registered. A call of the SUB is the old compiler's
error ("Cannot call SUB _GL directly") in `check\proc.rs`, before the lookup of SUBs.

The names: the lookup of a constant (`check\names.rs`) and of a SUB or FUNCTION (`check\proc.rs`) asks
`gl_table` when the program has the SUB (`has_gl()`) and the name starts with `_GL_` or `_gl`/`_glu` (case as the table's
lookup decides, measured by `v59_b`). A routine becomes a `Proc` with an `External` of a new `LibraryKind::OpenGl`
the first time it is named, so that a program pays only for the routines it uses; its parameters are `BYVAL` of
the table's types. Without `SUB _GL` the names are what `v59_b` records (probably the old compiler's errors for
a name it does not know, which the checker already gives).

**As built** (task 3.1, `sema\src\check\opengl.rs`): the checker's `has_gl()` asks `RuntimePart::Gl`; the routines share one
`Library` entry of kind `OpenGl`, made with the first; a constant is entered in the main module's constants, kept
in a set so that a `CONST` naming it is "Unexpected element" and a 0-valued one (`_GL_FALSE`) is no `NULL`. Where the
recordings asked for more: a name starting `_GL` that is neither in the table nor a built-in is a real error, with
`SUB _GL` or without (the table holds every such name the old compiler knows, so "not supported yet" would be
wrong there); an integer argument is converted without folding, so that the C++ shows it as written (`3042ll`,
`256`, `-1`); a whole array is its first element; a wrong number of arguments and a string as a later argument
are worded apart from a program procedure's, since the old compiler words them differently ("Syntax error", "2nd
sub argument requires a number") and one wording each keeps the notes after the errors right.

By the review of the change (`v59_x48`–`x54`): a later argument's wording carries the old compiler's ordinal
("for the 2nd argument"), and `old_wording.py` keeps a template with a word holding a digit when the pattern holds
the same word, so each recorded ordinal gets its note (rows for `_ADLER32`, `_RGBA32`, `$COLOR:0` came with that
rule); a routine with a type suffix is worded four ways, as the old compiler's four messages (a name alone, with
arguments, as a statement, after `CALL`); an OpenGL constant is a variable's `STRING * n` length (`x53`), while a
member's bound or length rejects it as any name that is no `CONST` above the block (`x52`, `x54`).

*Alternative:* a row per routine in `builtins.rs`'s `STATEMENTS` and `SUPPORTED`. Rejected: those rows name a
libqb call with slots and an optional mask; the OpenGL routines are C calls with plain `BYVAL` arguments, which is
what `External` already compiles, and they must vanish without `SUB _GL`.

### D4. The C++: `SUB__GL`, the helper calls, `DEP_GL=y`

`codegen-cpp` names the SUB `SUB__GL` with no arguments, as libqb's `extern void SUB__GL();` (`libqb.cpp` 24220)
needs; its body is a SUB's body. A routine's call goes through `library.rs` with the table's C name
(`call_glViewport`, or `glFlush` for a routine without a helper), no prototype (the helper header declares them)
and no header of a library. `make_flags` puts `DEP_GL=y` first for `RuntimePart::Gl`. The exact text (casts,
the `qbs` wrap of `glGetString`, the order of the SUB in `regsf.txt`) follows the old compiler's `-z` output of
`v59`, and the call-site check holds it.

Read in the old compiler's `-z` output (task 1.2: `opengl/test`, `v59_g_readback`, and a program of every call
form, 2026-10-24):

- `regsf.txt`: `void SUB__GL();` among the program's SUBs in file order (`void SUB_HELPER();` before it when
  `SUB helper` stands first); the body in its `main<n>.txt` is an ordinary SUB's (`void SUB__GL(){`, the `mem_lock`,
  `libqb_check_stack`, `exit_subfunc:`), with `SUB _GL STATIC` as any `STATIC` SUB.
- Inside it `SYSTEM` is `if (sub_gl_called) error(271);` then `close_program=1; end();`, `END` is `sub_end();`,
  `STOP` is `close_program=1; end();` (what this compiler writes already). That `STOP` hangs the program
  (`v59_f9`); the user decided (2026-10-24, `DIVERGENCES.md` D-036) that in a program with `SUB _GL` it first tests
  as `SYSTEM` does, `if (sub_gl_called){qbr_diverged(36);error(271);}`.
- A call: `call_glViewport(qbr( 1.5E+0 ), 2 , 3 ,qbr( 4E+0 ));`, `call_glClear( 16384ll );` (a constant is an
  `_INTEGER64` literal), `glFlush();` (no helper), `gluPerspective( 45 , 1.5E+0 , 1 , 10 );`; a literal 0 is `NULL`
  as for any library call, a float parameter too (`call_glClearColor( 1 ,NULL,NULL, 1 );`,
  `CALL _glViewport(0, 0, 1, 1)` gives `call_glViewport(NULL,NULL, 1 , 1 );`).
- A FUNCTION's result has no cast (`*__SINGLE_X=glGetError()+ 1 ;`, `call_glIsEnabled( 3042ll )* 2`), unlike a
  `DECLARE LIBRARY` FUNCTION's; `_glGetString` is `qbs_new_txt((  char*  )call_glGetString( 7938ll ))`.
- An `_OFFSET` argument: `((uptrszint)(…))` of `_OFFSET(x)`, a `LONG` variable as `*__LONG_V`, a whole array as its
  first element `((int32*)(__ARRAY_LONG_V[0]))[0]`.
- `$CONSOLE:ONLY` with `SUB _GL` builds (`v59_a5` runs).

### D5. A call-site program per routine, generated

`gl_table.py` also writes `tests\callsite\gl_<name>.bas` for each of the 337 routines: `$CONSOLE:ONLY`, a
`SUB _GL` with nothing in it (so the names exist), the arguments as variables of the table's types, the call on
the last line before `END`. The call stands in the main module, where it compiles (and would raise error 270 if
run; the programs are never run). `SUB _GL` compiles and builds under `$CONSOLE:ONLY` (`v59_a5`). `--check` covers these files too; a tier 1 test names a routine with
no program.

*Alternative:* one program per signature (about 124 distinct ones). Rejected: the C name differs per routine, and
a wrong name in the table would pass a check by signature; the cost that stays is about 340 more programs for the
call-site check, about 3 minutes of `qb64pe -z` in a run without the cache (CI's weekly `tier2` job), nothing with
it.

### D6. OpenGL is run: values read back inside `SUB _GL`

`v59_g` and `v59_h` are the run checks: `SUB _GL` clears, sets state and reads it back into shared arrays with
`_glReadPixels` and `_glGetIntegerv`; the main module waits for a count of frames with `_LIMIT`, prints the
values and ends with `SYSTEM`. They run in the verification suite, on the hidden desktop, with both compilers'
programs, and join `tests\verification\pass.list`. A slice program in `tests\corpus\slice\` is not added: the
values are compared already, and the corpus's screen check reads libqb's own surfaces, not OpenGL's.

## Risks / Trade-offs

- [OpenGL on the hidden desktop may have no context, or a software driver (Windows' GDI one) with other limits] →
  `v59_g` records what the old compiler's program gets there; both programs run on the same desktop. If no
  context is made there, the run programs are recorded once with `QB64RUST_SHOW_WINDOWS=1` and listed in
  `known.list` as needing a shown desktop, and the design says so. *Measured (task 1.1):* the old compiler's
  program gets a hardware context there (this machine's AMD driver); `v59_g` reads the cleared pixel, and the
  viewport of the window as the hidden desktop sizes it (512 by 320). The fallback is not needed.
- [The program's end after a critical error races its main thread] → the `f` programs wait 2 s after the failing
  call, so nothing after it is printed; two recordings were equal.
- [The render thread calls `SUB _GL` beside the main thread; a program that reads its shared variables races]
  → the run programs wait for a frame count before reading, and print only values set before it; the race is the
  old compiler's program's too.
- [A frame count is timing] → only "at least n" is printed, never a count.
- [Error 271 raised on the render thread may end the program differently from a main-thread error] → `v59_f`
  records the old compiler's exit; the new program is the same libqb, so it should be equal; if it varies, the
  program goes on `known.list` as `varies`.
- [The call-site check grows by about 340 programs] → see D5; with the cache the run is unchanged.
- [A new reference commit changes `gl.h`] → `gl_table.py --check` and the tier 1 test fail and name the routine;
  the table is regenerated.
