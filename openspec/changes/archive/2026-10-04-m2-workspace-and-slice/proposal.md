# Proposal

## Why

The project's riskiest technical bet is that one IR can stay ABI-neutral (optional arguments as present/absent,
explicit conversions, explicit raise/resume points) while a C++ emitter still produces exactly what `qbx.cpp` and
libqb expect (`study\15` §2). The roadmap would test that only in M3, after the whole front end exists. The
pre-coding review moved it forward: the first Rust code is an **end-to-end vertical slice** that takes a tiny
subset of the language from bytes through the lossless tree, resolution, typing, IR and the C++ emitter, links it
with the old runtime, runs it, and compares the output with `qb64pe.exe`. The slice fixes the shape of every stage
while it is still small; the rest of M2 then grows around a validated core.

Two documents the slice depends on do not exist yet: a **numeric-semantics spec** (the slice computes and folds
numbers, and LONG/`_INTEGER64` wrap is already decided) and a **divergence register** (where intentional
differences from the old compiler are recorded). Both are queued for the start of M2 (`STATUS.md`).

## What Changes

- **Rust workspace** at the repo root (`Cargo.toml`, `crates/`), toolchain pinned in `rust-toolchain.toml`. Six
  crates, one per pipeline stage, so the layering is enforced by the dependency graph: `base` (files, byte spans,
  diagnostics), `syntax` (byte lexer, lossless tree, parser), `sema` (resolution, typing, constant folding),
  `ir` (typed IR and lowering), `codegen-cpp` (emitter for `qbx.cpp`), `driver` (the `qb64rust` binary). A
  seventh, `builtins`, generates the built-in table from `tools\builtins\builtins.json` at build time (R9).
- **The slice language subset**: `$CONSOLE:ONLY`; comments; `:` separators; `DIM` of scalars (`AS INTEGER`,
  `LONG`, `_INTEGER64`, `SINGLE`, `DOUBLE`, `STRING`); `[LET] v = e`; implicit variables with type suffixes;
  `PRINT` with `;`, `,` and auto-semicolon next to string literals; `END`; numeric and string literals; unary `-`,
  `+ - * /`, parentheses, string `+`; and one built-in with an optional argument, `INSTR([start,] a$, b$)`. Any
  other construct gets a "not supported yet" diagnostic, never wrong code.
- **Build**: the driver writes the C++ fragments that `qbx.cpp` includes into a build folder of its own, then runs
  the reference clone's `Makefile` with command-line overrides so that `qbx.cpp`, its fragments and the
  executable live outside the clone; libqb is used as built there. Generated code is compiled with `-fwrapv`.
- **CLI** compatible with the corpus runner (`-x -q -m -z -o`), plus `--dump` stages for tests.
- **Tests**: the test conventions decided by the panel (`study\16` §8): mode lines in our own front-end tests,
  a typed-tree dump for literal-typing assertions, `insta` snapshots for diagnostics and lowering pairs, one error
  per statement for recovery. Tier 1 (`cargo test`) runs the front end over all 263 corpus programs (must never
  panic) and checks the slice programs; tier 2 runs the slice programs end to end through the existing runner.
- **New corpus group `tests\corpus\slice\`**: about six small programs written for this change (literal typing,
  integer wrap, float-to-integer rounding, division, `INSTR`, PRINT item forms), with output recorded by
  `qb64pe.exe` like the rest of the corpus.
- **`language/numeric-semantics` spec**: the numeric rules the slice implements, each with a measured example.
- **`DIVERGENCES.md`**: the divergence register, with its first entries (LONG and `_INTEGER64` wrap; differs from
  QB64pe `-O2`).
- The runner gains `--list <file>` (run the programs named in a file) so the slice subset can be selected.

## Capabilities

### New Capabilities

- `compiler/pipeline`: the stages, what each guarantees (bytes, lossless tree, typed tree, ABI-neutral IR,
  statement-granular raise points), the emitted fragments and the build.
- `compiler/cli`: the `qb64rust` command line, exit status and diagnostics output.
- `language/numeric-semantics`: literal typing, integer arithmetic width and wrap, float-to-integer conversion,
  division, constant folding.
- `testing/compiler-tests`: mode lines, typed-tree and IR snapshots, tier-1 corpus checks, the slice corpus group.

### Modified Capabilities

None. (`testing/golden-corpus` is unchanged: `slice` is a new group under the existing layout requirement; the
`--list` option is a runner convenience, not a corpus rule.)

## Impact

- New: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `crates\`, `tests\corpus\slice\`, `tests\frontend\`,
  `DIVERGENCES.md`. Changed: `tools\legacy_tests\run_legacy_tests.py` (`--list`), `.gitignore` if needed.
- Building a slice program needs the reference clone `..\QB64pe` with its toolchain. The Makefile builds the
  libqb objects into the clone's git-ignored `internal\c\*.o` if they are missing, exactly as running
  `qb64pe.exe` does; no tracked file of the clone changes (design D6).
- Not in this change: the full parser, the formatter, the language server, the built-in table beyond `INSTR`,
  compiling the auto-included BASIC files, procedures, control flow, arrays, programs without `$CONSOLE:ONLY`,
  CI for Rust, the runner's parallel option (`study\19` §5: measured here, added when the new compiler covers
  more of the corpus), the "cannot raise" flag (M3, `study\16` §8).
- `STATUS.md`, `study\00` §2/§10, `CLAUDE.md` (layout table, decisions) updated when done.
