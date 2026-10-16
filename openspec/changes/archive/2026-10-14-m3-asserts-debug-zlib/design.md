# Design

## Context

Motivation and scope: `proposal.md`. Where each piece stands in this compiler today (2026-10-14):

- **The precompiler already knows the flags.** `syntax\pp.rs` reads `$ASSERTS` (`Feature::Asserts`),
  `$ASSERTS:CONSOLE` (`Asserts` and `Console`) and `$DEBUG` (`Feature::Debug`) as settings of the whole program
  and settles `_ASSERTS_` and `_DEBUG_` by recompiling as the old compiler does (`m2-console-screenhide`). Only
  `sema` rejects the lines: `check\mod.rs` `meta` ends in "metacommand `…`" "not supported yet" for any
  metacommand it does not name.
- **The globals are written already.** `codegen-cpp\decl.rs` 131 writes `int32 asserts=0;` and `int32 vwatch=0;`
  for every program, as the old compiler does for a program without the two metacommands.
- **`END` and `SYSTEM`** are `StmtKind::End` / `System` (`check\mod.rs` 916); with an expression they are "not
  supported yet".
- **Runtime parts** (`ir::RuntimePart`: `Icon`, `Audio`, `Printer`, `ImageCodec`, `Font`, `ScreenImage`) are set by
  the rows that need them and turned into make flags in the old compiler's order (`driver\build.rs`
  `make_flags`).
- **`-f:` settings** are checked in `driver\main.rs` `build_settings`; `GenerateLicenseFile` is "not supported
  yet" (a `cli.rs` test asserts so). The build cache (`build.rs` `build_key`, `QB64RUST_BUILD_CACHE`) keys a build
  by its make flags, the fragments and the extra link file, and copies the cached executable.
- What the old compiler writes: `proposal.md` "Read before this proposal". Its make line puts `DEP_ZLIB=y` after
  `DEP_DEVICEINPUT` and before `DEP_EMBED`, `DEP_CONSOLE` and `DEP_ICON_RC` (`qb64pe.bas` 13706–13721), and
  `GENERATE_LICENSE=y` after the dependencies (13772).

## Goals / Non-Goals

**Goals:**

- Every form compiles to the old compiler's C++ (the call-site check compares `_ASSERT` and the six functions)
  and is rejected where it rejects it; each is run by a console slice program recorded with `qb64pe.exe`.
- The upstream programs the proposal names pass, the licence tests among them.

**Non-Goals:**

- vWatch, and anything only the QB64pe IDE observes.
- Other `-f:` settings; `_MD5$`; `$CHECKING`.

## Decisions

### D1. The metacommands are program settings, as `$CONSOLE` is

`sema`'s `meta` accepts `$ASSERTS`, `$ASSERTS:CONSOLE` and `$DEBUG` in the forms the old compiler accepts (task
1.1 measures them: case, blanks, a tab, a word after, `$ASSERTS:console`, inside a `$IF` branch not taken, in an
included file, in a SUB), and records `asserts` and `debug` on the checker. `$ASSERTS:CONSOLE` also sets
`console`, as `$CONSOLE` does (the flag value is the parser's already). The IR's `Settings` gains `asserts`;
`decl.rs` writes `int32 asserts=1;` when it is set. `vwatch` stays 0 always (D2).

*Alternative:* read the settings from `pp.rs`'s settled flags instead of the statements. Rejected: the flags are
for `$IF`; `sema` already takes `$CONSOLE` from the statement, and a metacommand in a branch that is not taken
must not count (the parser drops those lines, so reading the statements gives that for free).

### D2. `$DEBUG` writes no vWatch code (user's decision)

The old compiler's `$DEBUG` adds a call of `SUB_VWATCH` to every line, includes `vwatch.bm` (a SUB of about 700
lines with `DECLARE LIBRARY`, `_OPENCLIENT`, `GOSUB`) and writes `int32 vwatch=-1;`. Outside the IDE that SUB
sets `vw_bypass` on its first call (no `QB64DEBUGPORT`) and returns at once ever after. This compiler writes
none of it: `_DEBUG_` is set (D1), the program is the same as without `$DEBUG`, and with `-w` the warning "$DEBUG
features only work from the IDE" is given at the line, the old compiler's text. Entry in `DIVERGENCES.md` (next
free number): a program built here cannot be debugged from the QB64pe IDE. Task 1.1 measures what the old
compiler does with `$DEBUG` that a program can observe outside the IDE (a pause at start, `_DEBUG_` with
`_SOCKETS_`, whether `vwatch=-1` changes an error message or `ERL`); anything found is put to the user before
group 2 is built.

*Alternatives:* write vWatch (much code, nothing tests it, needs `DECLARE LIBRARY`); keep it "not supported yet"
(two upstream programs held for no observable difference). Both put to the user, who chose this one.

*Found by task 1.1, put to the user by task 1.3* (`DECISIONS.md` 2026-10-14; `verification\v33_d01`–`d10`,
`x49`): outside the IDE the old compiler's program prints what it prints without `$DEBUG`, but for two things.
**`_SOCKETS_` is 1** (vWatch uses `_OPENCLIENT`): the user chose to copy it, so `$DEBUG` sets `_SOCKETS_` with
`_DEBUG_` in the precompiler (`syntax\pp.rs`), and nothing is linked for it. **vWatch's names are taken** (a SUB
named `vwatch_stoptimers` is "Name already in use"): not copied, listed in the `DIVERGENCES.md` entry. The
warning is printed once per round of compiling by the old compiler (twice when a flag changed); here once.

### D3. `_ASSERT` is a plain row

`_ASSERT` is a statement row of the table (`sub__assert`, `?[,?]`); the third argument (`passed`, whether a
message was given) is what the table's optional mask writes for any optional slot, so the row needs no rule of
its own. Task 1.1 confirms it with `-z` (with and without the message, after `CALL`, with a number as the
message), and a call-site program per form compares it. Errors 314 and 315 are libqb's; the slice program shows
what `ON ERROR` does with them.

*Found by task 1.1 and 1.2* (`study\00` §5, `v33_*`): the row's condition slot is an INTEGER, so the table's
conversion is the old compiler's (`sub__assert(qbr_float_to_long(*__DOUBLE_D),__STRING_S,1)`); without `$ASSERTS`
the arguments are still evaluated (the call is made and returns at once). **Neither 314 nor 315 is trapped by `ON
ERROR`**, as 300–313 are not (`error_handle.cpp` 368): the program stops with the runtime error, exit code 0. So
the slice program cannot show a handler; a failing assertion ends it, one per program.

### D4. The six functions are plain rows with a new runtime part

`_ADLER32`, `_CRC32`, `_DEFLATE$`, `_INFLATE$`, `_BASE64ENCODE$`, `_BASE64DECODE$` are rows (`Rule::Plain`), each
naming a new `RuntimePart::DataProcessing`; `make_flags` writes `DEP_ZLIB=y` between `DEP_FONT=y` and
`DEP_CONSOLE=y`, the old compiler's place. A row in code that is never run still links the part, as with the
old compiler (it sets the dependency when it registers the name, not when the line runs). Task 1.1 measures each
with `-z` (the `_INFLATE$` size argument converted to `_INTEGER64`, `_DEFLATE$`'s level to LONG) and with
`qb64pe.exe` (values, an empty string, a bad base64 or deflate string, a level out of 0–10).

### D5. `END n` and `SYSTEM n` store `exit_code`

A new field on `StmtKind::End` and `System`: the code, an expression converted to `_INTEGER64` as an assignment
to an `_INTEGER64` would be (checked by the old compiler's `evaluatetotyp(…, ISINTEGER64)`: a string is "Expected
numeric"; task 1.1 measures the exact errors). The emitter writes, before the usual end, `if(qbevent){evnt(N);}`
then `exit_code=<expr>;`, the old compiler's two lines (with the included file's line and name when the line is
in one, as `evnt` is written elsewhere). Task 1.2 measures the process's exit code for 3, 0, -1, 256, 2.7,
`2^40`, and for `END n` in a program with a window, and how the runner records an exit code.

*Found by task 1.1 and 1.2* (`study\00` §5): `evaluatetotyp(…, ISINTEGER64)` reads the target's width as 0, so
an integer goes in as it is and **a float through `qbr_float_to_long`**, the conversion `ON n GOTO` already has
(`check\flow.rs`, `on_jump`: narrowed to SINGLE, rounded half to even; `2 ^ 40` gives 0). The process's exit code
is the low 32 bits of `exit_code` (`END -1` 0xFFFFFFFF). An `_OFFSET` is accepted; a string is "Illegal
string-number conversion". `END n` in a program with a window waits for a key in the window, which no runner
gives a console program's way; `SYSTEM 5` there gives 5.

### D6. `GenerateLicenseFile` passes `GENERATE_LICENSE=y` and moves the file with the executable

`build_settings` accepts `GenerateLicenseFile=true|false` (anything else "needs `true` or `false`", as
`OptimizeCppProgram`). When true, `build_with_make` adds `GENERATE_LICENSE=y` and `LICENSE=<build exe>.license.txt`
to the make line; the file is moved beside the final executable with it (`<exe>.license.txt`). The build cache
keys the setting (one byte) and keeps the licence file beside its cached executable, copying both; a cached entry
without the file is a miss when the setting is on. A build that fails leaves no licence file at the output path.

*Alternative:* write the licence file in Rust from the clone's `licenses\` folder. Rejected: the `Makefile`
already knows which parts are linked; a second list would drift.

### D7. Tests

- `verification\v33_*`: `v33_a` the asserts (on, off, trapped, the console form), `v33_b` the six functions'
  values, `v33_c` exit codes, `v33_x*` one accepted or rejected form each.
- Call-site programs: `_ASSERT` (one and two arguments, after `CALL`), each of the six functions by argument
  type.
- Front-end tests: `check-fail` for the rejected forms, `typed` for the result types, `ir` for an `END n`.
- Slice programs (console): asserts on and trapped, the six functions, an exit code. The runner records a
  program's exit code already (`.exitcode` is compared when present; task 1.2 checks this, and adds it if not,
  with a runner test).

  *Found by task 1.2*: the corpus runner records and compares no exit code (the upstream suite fails any code
  but 0; the verification suite writes `exit=`). Group 4 adds it: a sidecar `<name>.exitcode` holding the code
  the old compiler's program gave, compared when present and written by `--record` when the code is not 0, with
  a runner test.
- Upstream: the programs the proposal names join `pass.list`; `audio_out/test` and `image/test` leave
  `known_clean_not_passing.list` when they pass; `basic/test` is checked (its `PRINT` goes to a window).

## Risks / Trade-offs

- [The licence text depends on the clone's `licenses\` files and the parts linked] → it is compared with the
  recorded `.win.license` files by the runner; a difference names the file.
- [`$DEBUG` might change something a program can see outside the IDE] → task 1.1 measures it before any code,
  and a finding goes to the user (D2).
- [The data-processing library is a new libqb variant to build in CI's `tier2`] → it is built once and cached
  like the others; the first `tier2` run is slower.
- [An exit code may not be comparable by the runner today] → task 1.2 settles it before group 4.

## Migration Plan

None: new forms only. The `cli.rs` test that used `GenerateLicenseFile` as the unknown setting takes
`MaxCompilerProcesses`.
