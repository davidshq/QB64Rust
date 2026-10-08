# Differential programs

Generated BASIC programs that compare the new compiler's numeric results with the old compiler's (spec
`testing/differential-tests`, design D2 of the OpenSpec change `m2-numeric-types`). Each `<group>\<name>.bas` has its
expected output in `<name>.output` beside it, recorded once with `qb64pe.exe`; running the programs needs no old
compiler. **The `.bas` files are generated: never edit them by hand** (tier 1 fails when one differs from the
generator).

## Groups

107 programs: 59 for all numeric types, and the same 48 again in `old6` for the six old types only. The numeric types (`crates\difftest\src\lib.rs`, `TYPES`): `_BYTE`, `INTEGER`, `LONG`, `_INTEGER64`,
`_OFFSET`, each also `_UNSIGNED`; `_BIT`, `_UNSIGNED _BIT * 7`, `_BIT * 24`, `_UNSIGNED _BIT * 40` (four widths stand
for `_BIT * n`: 1 bit, narrow, `int32` storage, `int64` storage); `SINGLE`, `DOUBLE`, `_FLOAT`. Each type has seven
value slots: its extremes, -1 (an unsigned type's largest value, the bits of -1), 0, 1 (0 for the 1-bit `_BIT`,
whose range is -1 to 0) and two seeded random values. A float's extremes are its largest finite literal and its
negative: `3.402823E+38`, `1.797693134862315D+308`, and for `_FLOAT` `1.797693134862315F+308`, because the old
compiler writes a `_FLOAT` literal as a C++ double (`1.18973149535723176F+4932` is infinite; it is `_FLOAT`'s
"beyond" value in `fold` and `print`).

| Group | Programs | What each line prints |
|---|---|---|
| `ops` | one per binary operator: `+ - * / \ MOD ^`, the six comparisons, `AND OR XOR EQV IMP _ANDALSO _ORELSE` (20) | `a op b` for every ordered pair of the 17 types and eight value pairs: min/min, max/max, min/max, max/min, -1/1, 0/1 and two random pairs; operands in variables |
| `fold` | one per binary operator (20) | the same with the operands written as literals (the 15 types with a literal suffix), plus two pairs one step beyond the range (above/below, below/above) of the integer types narrower than 64 bits and of `_FLOAT`: constant folding and literal typing against the old compiler, which does not fold |
| `unary` | `unary.bas` | `-`, `NOT`, `_NEGATE` on every type's slots |
| `store` | one per target type (17) | every type's slots stored into a variable of the target type, then printed |
| `print` | `print.bas` | `PRINT` and `STR$` of every type's slots from variables, and of every literal of the `fold` group |
| `old6` | the programs above for `INTEGER`, `LONG`, `_INTEGER64`, `SINGLE`, `DOUBLE`, `_FLOAT` only, named `<group>_<name>.bas` (48: one `store` per kept type) | as above, with the same values; a header line names the subset |

`old6` exists because every full program declares all 17 types and so cannot compile before the new types do; the
subset is what today's compiler can build, so tier 2 guards it while the new types come in (`DECISIONS.md`
2026-10-08). It goes when the full programs pass.

Each line is labelled with the operator, the types and the value pair (`^ LONG SINGLE 7:`), so a differing line
names its case. Values are stored into their variables from literals of the variable's own type (the `_OFFSET`
types, which have no literal suffix, from `&&` and `~&&` literals; the smallest `_INTEGER64` as
`(-9223372036854775807&& - 1)`), so the store under test is not the one that creates the value.

Rules (spec "Safe programs"; each program's header names what it leaves out):

- `ON ERROR GOTO h` at the start; the handler prints ` error` and `ERR` and resumes next, so an error inside a line
  shows in that line's place (`^ BYTE SINGLE 8: error 5`).
- `\` and `MOD`: no divisor that rounds to 0; no smallest LONG or `_INTEGER64`, or float beyond LONG, divided by a
  divisor that rounds to -1 (the old program crashes, `DIVERGENCES.md` D-006).
- `^`: no type pair with an `_OFFSET` operand (a compile error in the old compiler).
- A `_BIT * 32` pad is DIMmed just before every `_BIT * 40`, so the old compiler's overlap hits the pad
  (`DIVERGENCES.md` D-009).
- No signed radix literal wider than its type (no radix literals at all), no `PRINT` comma, `SYSTEM` at the end (no
  key press needed).

## Regenerate, record, run

```text
cargo run -p qb64rust-difftest -- gen            # writes the programs that changed
cargo run -p qb64rust-difftest -- gen --check    # writes nothing; fails if a program is missing, differs or is extra
python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root tests\differential --record
python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root tests\differential ^
    --qb64 target\release\qb64rust.exe --list tests\differential\pass.list
```

`--record` uses `qb64pe.exe` (default build), runs each program twice and writes the `.output` only when both runs
agree. A program whose text changed must be recorded again before it is committed. Recorded 2026-10-08 against the
reference clone (`qb64pe.exe` 4.7.0): all 59 compile and run to their last line; 211 s for the whole set; a second
`--record` changed no file. `--category <group>` and `--glob <name>.bas` record or run a part. `old6` recorded the same day (`--category
old6`): all 48 run to their last line; 132 s.

The recordings hold the old compiler's text for an infinite result (`DIVERGENCES.md` D-010): after `INF` come NUL
bytes and the remains of a number printed earlier (`+ SINGLE SINGLE 1:-INF<NUL>2979E+11`), the same on every run.
They are kept byte for byte (`.gitattributes` marks the outputs `-text`). When D-010 is implemented (M3) those lines
change, and the programs that print them (`ops` and `fold`: `+ - * / ^`) get a `.normalize` rule or a new recording.

`gen --out <dir> --types <KEY,...>` writes the same programs for some of the types only (keys as in the labels,
e.g. `BYTE,UBYTE,LONG`), to record and run elsewhere. Such a subset is never written into this folder (`gen`
refuses); the one subset kept here is `old6`.

## Tiers

- Tier 1 (`cargo test`): `crates\difftest\tests\fresh.rs` regenerates in memory and compares with these files, and
  requires a recording for each; `crates\driver\tests\inputs.rs` runs the front end over them (no panic, exact
  round trip, no false error) and requires every program that compiles cleanly to be in `pass.list`.
- Tier 2 and CI (`rust.yml`, job `tier2`): the programs of `pass.list` built and run with `qb64rust`.

`pass.list` names the programs the new compiler passes, one `<group>/<name>` per line. It only grows; by the end of
`m2-numeric-types` it names every program. On 2026-10-08 it names 27 of `old6` (all `ops`, the six `store`,
`unary`); `old6`'s 20 `fold` programs and `print` stop at "overflow" for literals beyond INTEGER and LONG
(`32768%`), which the old compiler accepts, so they are in `tests\known_false_errors.list` until design D7
(group 4) removes that error.
