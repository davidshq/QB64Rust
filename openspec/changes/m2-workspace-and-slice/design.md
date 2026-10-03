# Design

## Context

See `proposal.md` for motivation and the four spec deltas for requirements. Background: pipeline (R5) and back
end (R6, R7) in `study\00` §2; numeric facts in `study\00` §4 and `study\02` §1.4–1.5; PRINT emission in
`study\10` §2; shape of the generated program in `study\02` §8; the Makefile in `study\05` §2; FreeBASIC lessons
L1, L2, L4, L6, L8, L12 in `study\16` §3 and the panel decisions in `study\16` §8; test tiers in `study\19`.

### What the old compiler emits for the slice (measured 2026-10-03, `qb64pe -z`, `16f629784e`)

A program using everything in the slice produces, in `internal\temp\`:

| Fragment | Content for the slice | Notes |
|---|---|---|
| `global.txt` | `qb_safe_idiv`/`qb_safe_mod` templates; `int32 *__LONG_A=NULL;` per main-module variable; `console=1`, `screen_hide_startup=0`, `asserts=0`, `vwatch=0`, `data_size=0`, `data=(uint8*)calloc(1,1)` | the six globals are referenced by `qbx.cpp`/libqb |
| `maindata.txt` | `if(__LONG_A==NULL){ __LONG_A=(int32*)mem_static_malloc(4); *__LONG_A=0; }` | strings: `qbs_new(0,0)`-based, to be recorded in task 4.1 |
| `clear.txt` | `*__LONG_A=0;` | |
| `mainerr.txt` | `if (!error_handler_history) …; if (error_occurred){ error_occurred=0; exit(99); }` | no `ON ERROR` in the slice |
| `main.txt` | `#include "main0.txt"` … + `func__compdate`, `func__comptime`, `func__compvers` | the three functions are required by libqb |
| `main0.txt` | `S_0:;`, one `do{ … if(!qbevent)break;evnt(N);}while(r);` per statement, then `sub_end(); return; }` | `$CONSOLE:ONLY` emits `sub__dest(func__console()); sub__source(func__console());` as statement 1 |
| `regsf.txt`, `main1..3.txt` | `SUB_VWATCH`, `FUNC__ENCODEURL`, `FUNC__DECODEURL` from the auto-included BASIC files | nothing in `internal\c` references them; the slice omits them (task 4.2 confirms by linking) |
| the other ~15 `.txt` files `qbx.cpp` includes | empty | must exist |

Per PRINT statement: `tqbs=qbs_new(0,0);`, then per item `qbs_set(tqbs, <string expr>); if (is_error_pending()) goto skipK; makefit(tqbs); qbs_print(tqbs,0);`; a numeric item is
`qbs_add(qbs_str((<C type>)(e)), qbs_new_txt(" "))`; `,` is `tab();`; the newline `qbs_print(nothingstring,1);`;
then `skipK: qbs_free(tqbs); qbs_cleanup(qbs_tmp_base,0);`. Casts seen: `(int16)( -3 )`, `(float)( 1.5E+0 )`,
`(int64)(*__INTEGER_I+ 1 )` (computed in C `int`, printed as 64-bit: 32768), `(double)(*__DOUBLE_D/ 3 )`,
`(long double)( 7 / ((long double)( 2 )))`. `INSTR("hello","ll")` is `func_instr(NULL, s1, s2, 0)`;
`INSTR(3, …)` is `func_instr( 3 , s1, s2, 0|1)`: the optional argument becomes a placeholder plus a bit in the
`passed` mask. Every line is preceded by `#line N "file"`.

## Goals / Non-Goals

**Goals:**
- Every stage exists, with the shape it will keep: bytes in, lossless tree, typed tree, ABI-neutral IR, C++ out.
- The seven slice programs of the corpus and the new `slice` group compile with `qb64rust`, link with libqb, and
  pass the existing corpus runner against output recorded from `qb64pe.exe`.
- The IR can express optional arguments, explicit conversions and statement-granular raise points without
  mentioning `passed` masks, `qbs*` or `evnt`.
- Numeric rules are written down before code depends on them.

**Non-Goals:**
- Language coverage beyond the subset. Procedures, control flow, arrays, `CONST`, `DEFxxx` are later changes.
- Performance, incremental compilation, the language server, the formatter.
- Matching the old compiler's C++ text. Only the ABI must match (names libqb calls back, calling conventions,
  fragment set); spelling, temporaries and layout are free.
- Matching the old compiler's error texts (`CLAUDE.md`: new error messages).
- Compiling the auto-included BASIC files (`beforefirstline.bi`, `aftermain.bas`, `afterlastline.bm`).

## Decisions

### D1. Workspace at the repo root, one crate per stage
```
Cargo.toml                workspace; members = crates/*; shared lints and dependency versions
rust-toolchain.toml       channel = "1.88.0" (the installed stable; R13: pinned toolchains)
crates/
  base/                   FileId, Span (byte range), SourceMap, line index, Diagnostic, Severity
  syntax/                 lexer (bytes), lossless tree, parser; parser/ split by statement family (L1)
  builtins/               build.rs turns tools/builtins/builtins.json into a static table
  sema/                   symbols, suffix and implicit-type rules, literal typing, type checking, constant folder
  ir/                     typed IR types and lowering from the typed tree
  codegen-cpp/            IR -> qbx.cpp fragments
  driver/                 bin `qb64rust`: CLI, pipeline, build (make), diagnostics output
tests/frontend/           our own front-end tests with mode lines (D10)
```
Package names are `qb64rust-<crate>`, matching the project id. Dependencies point only downward
(`driver → codegen-cpp → ir → sema → syntax → base`; `sema` and `ir` also use `builtins`); `ir` does not depend
on `codegen-cpp`, so nothing in the IR can name a libqb symbol. The root already ignores `/target/` (shared with
the legacy runner's results). Edition 2024. External crates: `insta` (dev), `serde_json` (build-dependency of
`builtins`), nothing else. *Alternative:* a single crate with modules. Rejected: the layering is the point of the
slice, and crate boundaries keep it honest when the code grows.

### D2. Bytes and positions
Source files are read as `Vec<u8>` and never converted to `str` (`study\15` §1). A position is
`(FileId, u32 byte offset)`; a `Span` is a byte range in one file. `SourceMap` holds the files and a line index
(offsets of line starts; CR LF, LF and a lone CR all end a line, as in `lineformat$`). Columns in diagnostics are
1-based **byte** columns; under CP437 that equals the character column. Conversion to UTF-16 positions belongs to
the language server, later. Identifiers are compared case-insensitively on ASCII only; bytes ≥ 0x80 are
accepted only inside string literals and comments for now (the old compiler's rule for names is to be checked
when the full lexer is written).

### D3. Lossless tree: our own, over bytes
A green tree (immutable nodes: kind, byte length, children; tokens: kind, byte length) and a red cursor layer
that adds absolute offsets. Token text is a slice of the file's bytes. Trivia (spaces, tabs, comments, line
continuations) are tokens in the tree, so printing the tree reproduces the input **byte for byte**; a tier-1 test
checks this for every corpus file, including files the parser cannot fully handle (unknown statements become an
`ERROR` node holding the skipped tokens). *Alternative:* `rowan` (rust-analyzer's tree). Rejected: its token text
is `str`, which contradicts D2; CP437 bytes 0x80–0xFF are not valid UTF-8, and mapping them to chars breaks byte
offsets. Our tree is a few hundred lines and has exactly the API we need.

### D4. Parser and recovery
Hand-written recursive descent over the token stream; Pratt parsing for expressions with the 16-level table of
`study\02` §1.3 (only `+ - * /` and unary minus are accepted in the slice, but the table is complete so precedence
tests can be written now). A statement ends at a newline or `:` outside parentheses and strings. Recovery
(panel decision 4): at most **one error per statement**; after an error the parser skips to the statement end
into an `ERROR` node. Total cap: 100 errors per run, then "too many errors". Block errors do not arise in the
slice. Statement families get their own modules (`parser/print.rs`, `parser/decl.rs`, `parser/assign.rs`,
`parser/meta.rs`, `parser/expr.rs`), dispatched by the first token (L1). Keywords the slice does not support are
recognised and reported "`<KEYWORD>` is not supported yet"; this keeps the never-panic corpus test meaningful.

### D5. Resolution and typing (`sema`)
Produces a **typed tree**: a separate arena of typed expression and statement nodes, each pointing back to the
syntax node by `Span`. Rules implemented (all with tests, values from `language/numeric-semantics`):
- Variables: `x`, `x%`, `x&`, `x&&`, `x!`, `x#`, `x$` are distinct; a `DIM x AS T` declares the suffix-less name
  with type T and makes `x<suffix of T>` the same variable; what the old compiler does with a *different* suffix
  on a DIMmed name (error or separate variable) is measured in task 3.3 and implemented as measured. Undeclared
  names are implicitly SINGLE
  (no `DEFxxx` in the slice). `OPTION _EXPLICIT` is not in the slice.
- Literal typing (spec): integer literals take the smallest of INTEGER, LONG, `_INTEGER64` that holds the value;
  float literals by significant digits.
- Operators produce the spec's **computation type** (the type the value is computed in, e.g. 32-bit for
  `INTEGER + INTEGER`) and every implicit conversion becomes an explicit `Convert` node.
- Constant folding of literal-only subexpressions uses `wrapping_add/sub/mul` on `i32`/`i64` (panel decision 1) and
  `f64`/extended rules per the spec; folding never changes a printed result (snapshot tests compare folded and
  unfolded runs of the slice programs).

A debug dump of the typed tree (`--dump typed`) prints one node per line with its type, for assertions (L8).

### D6. The IR
Per procedure (only the main module in the slice) a list of statements. Each IR statement:
```
Stmt { span, line, ops: Vec<Op>, may_raise: bool }
Op   = Assign { place: VarId, value: Value }
     | Print { dest: Console, items: Vec<PrintItem>, newline: bool }   // PrintItem = Str(Value) | Num(Value) | Zone
     | End
Value = Const(..) | Var(VarId) | Convert { to: Ty, how: Round|Truncate|Widen, from: Box<Value> }
      | Binary { op, ty, lhs, rhs, overflow: Wrap } | Neg | Concat
      | CallBuiltin { id: BuiltinId, args: Vec<Option<Value>> }          // None = optional argument absent
```
Raise semantics are explicit and statement-granular, matching QB64pe (`study\00` §4): "an op that may raise is
followed by an implicit check; on error the rest of the statement is skipped; errors and events are serviced at
the statement boundary; `RESUME` re-runs, `RESUME NEXT` continues after, the statement". The IR marks *which*
ops may raise (`CallBuiltin` and string ops: always, until the "cannot raise" flag of M3) but contains no jumps,
labels, `qbevent` or `evnt`; those are the emitter's encoding of the rule. Optional arguments are `Option` slots
in table order; the `passed` mask is computed by the emitter. Types are the IR's own (`I16, I32, I64, F32, F64,
F80, Str`), mapped to C types in the emitter only.

Lowering pairs (L12) are kept as `insta` snapshots: source line → IR text → C++ text, e.g.
```
PRINT "x="; a; INSTR(3, s$, "l")
Stmt line 4 may_raise
  Print console newline
    Str "x="
    Num Var a:I32
    Num CallBuiltin INSTR [Some(Const 3:I16 -> Convert I32 Widen), Some(Var s$:Str), Some(Const "l")]
```

### D7. The emitter (`codegen-cpp`)
Writes the full fragment set `qbx.cpp` includes (the table in Context), empty files where the slice has nothing.
It must match the old compiler where libqb or `qbx.cpp` depend on it: the six globals, `func__compdate/
comptime/compvers` (version string `QB64-PE v4.7.0-GLFW-UNKNOWN`, as measured), the `QBMAIN` body ending with
`sub_end(); return; }`, `S_0:;`, statement wrappers `do{ … if(!qbevent)break;evnt(N);}while(r);`, the PRINT item
sequence with `skipK`, string temporaries and `qbs_cleanup`, variable storage via `mem_static_malloc`, `passed`
masks. Variable names follow the old scheme (`__LONG_A`, `__STRING_S`) because the debugger (M5) and readers of
generated code know it, but nothing depends on them. Numeric items are printed via `qbs_str` with the C type of
the **computation type** (so `i% + 1` is computed in `int32` and printed with an `int64` overload, as measured).
Every statement carries `#line N "file"` (relative path). The emitter has no knowledge of syntax.

### D8. Building against the reference clone
The driver creates `<exe folder>\<exe name>.qb64rust\` with `c\qbx.cpp` (copied from `..\QB64pe\internal\c\qbx.cpp`
on every build; never committed) and `temp\*.txt`, deletes any old `qbx.o` there, and runs
```
<qb64pe>\internal\c\c_compiler\bin\mingw32-make.exe -C <qb64pe> -j3 OS=win BITS=64 DEP_CONSOLE_ONLY=y exe
    EXE=<exe> QB_QBX_SRC=<build>/c/qbx.cpp PATH_INTERNAL_TEMP=<build>/temp
    "CXXFLAGS_EXTRA=-fwrapv -I<qb64pe>/internal/c"
```
Make command-line variables override the Makefile's own assignments, so `qbx.o`, the fragments and the
`.sym` file land in the build folder; the libqb objects (`libqb_make_*.o`, part libraries) are built into the
clone's git-ignored folders if missing and reused otherwise, exactly as `qb64pe.exe` does. `-fwrapv` defines
LONG and `_INTEGER64` overflow (`study\16` L4; divergence D-001/D-002). The clone is located by `--qb64pe-root`,
else the environment variable `QB64RUST_QB64PE_ROOT`, else `..\QB64pe` relative to the repository the binary was
built from (development default). Success = make exits 0 **and** the exe exists. The build folder is kept on
failure and deleted on success unless `--keep-build`. Building libqb objects into the clone's git-ignored folders
was approved by the user on 2026-10-03 (`CLAUDE.md`; rule 3 still forbids changing tracked files). Task 4.3 checks `git -C ..\QB64pe status --porcelain`
is unchanged by a build. *Alternatives:* (a) copy `internal\c` into this repo now: that is M3's plain copy
(`CLAUDE.md`), 1,100 files, too early; (b) write fragments into the clone's `internal\temp`: shared with
`qb64pe.exe` (`study\05`), races with the corpus runner and the extension, and edits the clone's working tree;
(c) compile `qbx.cpp` ourselves without make: duplicates the Makefile's flag and library logic.

### D9. Command line
`qb64rust [-x] [-q] [-m] [-w] [-z] <file.bas> [-o <exe>] [--dump tokens|tree|typed|ir|cpp]
[--qb64pe-root <dir>] [--keep-build]`. `-x -q -m -w` are accepted for compatibility with the runner and the M1
extension (`-x` and `-m` change nothing; `-q` suppresses progress; `-w` shows warnings, none exist yet). `-z`
stops after writing the fragments. `--dump` prints the stage and exits 0 (or 1 with diagnostics). Diagnostics
go to stdout as `<file>:<line>:<col>: error: <message>` (byte column, D2), then a summary line; exit status 1 if
any error, 0 otherwise. The default output name is the source name with `.exe`, next to the source.

### D10. Tests (panel decision 3, `study\19` tiers 1–2)
- **Mode line** for files under `tests\frontend\`: the first line is `' TEST: <mode>` with mode one of
  `parse-ok` (no syntax errors, tree round-trips), `check-ok` (no errors after `sema`), `check-fail` (at least one
  error; diagnostics snapshot), `typed` (typed-tree snapshot), `ir` (IR snapshot), `cpp` (fragment snapshot).
  A plain comment, so `qb64pe.exe` ignores it; the corpus runner never sees these files. Corpus programs keep their
  sidecar convention (`.output`, `.err`), which already says what to expect.
- **Snapshots** with `insta` (`.snap` files next to the test crate, reviewed with `cargo insta review`).
- **Tier 1** (`cargo test`): every corpus `.bas` (263 + the slice group) goes through lexer, parser and `sema` with
  no panic, and its tree prints back to the original bytes; the programs in `tests\corpus\slice.list` (D11) give
  no diagnostics; `tests\frontend\*.bas` per mode line; unit tests per crate. No C++.
- **Tier 2**: `run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list tests\corpus\slice.list`.
  The runner already accepts any compiler with qb64pe's flags; `--list` (new) restricts it to the names in a file.

### D11. Corpus additions
`tests\corpus\slice\` holds programs written for this change, with a `SOURCE.md` ("written for QB64Rust; expected
output recorded with `qb64pe.exe` `16f629784e`"): `s01_literals` (each literal form printed; typing visible in the
format), `s02_integer_wrap` (`i% + 1`, `x% = 70000`, LONG and `_INTEGER64` wrap, `2147483647 * 2`),
`s03_float_to_int` (half-to-even, single narrowing for INTEGER targets), `s04_division` (`1 / 3`, `7 / 2`,
`d# / 3`), `s05_instr` (optional argument; a start of 0 raising an untrapped error), `s06_print_items` (`;`,
trailing `;`, auto-semicolon, empty `PRINT`, mixed types), `s07_cp437_bytes` (bytes 0x80–0xFF in a string
literal and a comment; checks D2 end to end). They are recorded with `--record` like the rest.
`tests\corpus\slice.list` names the programs the slice must pass: the seven above plus `105_let`, `109_implicit_dim`,
`111_concat_multiple`, `131_expression_priority`, `134_float_literal`, `135_negate`, `14_instr`. No comma zones in
any of them (`study\10` §2.2: the console hang).

### D12. Divergence register and numeric spec
`DIVERGENCES.md` at the repo root, next to `SOMEDAY.md`. One row per intentional difference from the old
compiler's observed behaviour: id (`D-001`…), behaviour, old compiler (default build / `-O2` where they differ),
QB64Rust, reason, decided (date, by whom), test that pins it. Rows are added only for decided differences; the
open choices of `study\00` §6 stay there until decided. First rows: D-001 LONG overflow wraps (equals the default
build, differs from `-O2`); D-002 `_INTEGER64` likewise. The numeric rules themselves are the normative spec
`openspec\specs\language\numeric-semantics` (it becomes a main spec when this change is archived); its scenarios
are measured examples and each one is a test in the slice group or `tests\frontend\`.

## Risks / Trade-offs

- **Make overrides may not be enough** (an include path or a rule that assumes `internal\c\qbx.cpp`). → Task 4.2
  is a spike that builds a hand-written fragment set first; fallback is alternative (c) of D8, recorded here if
  taken.
- **A slice that is too small** to show IR problems. → It includes the two hardest ABI features cheaply available:
  an optional argument (`passed` mask) and a raise point inside a multi-item statement. Procedures, by-reference
  arguments and `ON ERROR` are the next things to add to it, in the next change.
- **The lossless tree is hand-written** and could be subtly lossy. → Round-trip test over every corpus file plus
  a test with every byte value 0x01–0xFF in strings and comments.
- **libqb objects built in the clone** are shared with `qb64pe.exe` runs; a different compiler flag set would
  force rebuilds. → Only `CXXFLAGS_EXTRA` differs, and it applies to `qbx.o` too, which is ours. Check in task 4.3
  that a slice build does not rebuild `libqb_make_*.o` after a `qb64pe.exe` build of the same feature set.
- **Pinning Rust 1.88** may lag crates. → Two dependencies only; bump deliberately.

## Open Questions

- Whether `func_instr` with start 0 raises error 5 or returns 0: recorded by `s05_instr` (task 3.1).
