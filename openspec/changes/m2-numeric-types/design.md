# Design

## Context

See `proposal.md` for why. The state this design starts from:

- `sema::Ty` (`crates\sema\src\lib.rs`) is a `Copy` enum with `I16, I32, I64, F32, F64, F80, Str, User(TypeId)`
  and a **derived** `PartialOrd`/`Ord`. The order is used in a few places: the operator typing in
  `check\ops.rs` (`promote(lt).max(promote(rt))`, the believed float type, the comparison's narrower float), the
  hidden type of a `FOR` (`check\blocks.rs` 411), and the IR dump (`ir\src\dump.rs` 198). About 520 matches on
  `Ty::` in 24 files list the variants, made exhaustive by `clippy::wildcard_enum_match_arm`.
- Each expression has two types (`sema` crate doc): `ty`, what the generated C++ computes in, and `qb`, what the old
  compiler believes and so how `PRINT` formats it. They already differ in three measured cases.
- The emitter (`crates\codegen-cpp`) maps `Ty` to C types in `names.rs` (`c_type`, `type_word`), emits its own
  `qb_safe_idiv`/`qb_safe_mod` templates (`decl.rs` 74), and builds against the reference clone's libqb, which is
  read-only. libqb already has `qbs_str` for every integer width, signed and unsigned, and `qbs_new_fixed`.
- What the old compiler does with these types is known only by reading `qb64pe.bas` (`study\02` §1.4 "markup",
  §1.5 rounding, §1.7 `_BIT`/`_UNSIGNED`/`_OFFSET`, §3.2 storage, §6.5 `FOR`), except fixed-string initial bytes and
  `_BIT * 33` overlap (`verification\v03_storage`, `v04_accidental`). No promotion rule for the new types has been
  run.
- The decisions this change implements are taken (`study\00` §6, `DIVERGENCES.md` D-005 to D-009,
  `DIVERGENCES-QB45.md` Q-003 to Q-008).
- `sema` already marks `label: CONST` (`check\constants.rs` 35), an `IMP` whose left operand is an `IMP`, `CONST …
  1 / 0` and an integer power beyond `_INTEGER64` "not supported yet"; `Severity::Warning` exists in `base` but
  nothing produces one, and the driver accepts `-w` and ignores it.

## Goals / Non-Goals

**Goals:**
- The tester exists and its recordings are in the repository **before** any typing rule for the new types is
  written, so each rule is written against measured output, not against `study\02`'s reading.
- One place states each promotion rule; nothing depends on the order of `Ty`'s variants.
- Every new type flows through the existing model (places, store rules, the pending-error rule, `Arg::Ref`/`Temp`)
  without a new IR concept, except the `_BIT` store mask.

**Non-Goals:**
- A type table or interned types (`study\27` §5, decided 2026-10-08).
- A random-expression fuzzer. The tester enumerates operators × type pairs × chosen values; nested random
  expressions can come later if the enumerated programs stop finding differences.
- The libqb-side divergences (D-010 to D-012) and every built-in beyond the 43 (step 9).
- Making the tester independent of `qb64pe.exe`: recording needs it once; running does not.

## Decisions

### D1. Measure first

`verification\v21_*`, run with `qb64pe.exe` (`verification\run.sh`) before the code that depends on them, for what
the generated programs do not cover:

- declarations: every `AS` spelling (`_UNSIGNED _BYTE`, `_BIT * 7`, `_UNSIGNED _BIT`, …), every suffix on variables,
  FUNCTION names and literals (`300~%%`, `&HFF~%%`, `-1~&`), the `_BIT * n` limit, implicit variables with the new
  suffixes, `LEN` of each type;
- `_BIT` stores (mask, sign extension, rounding of floats into 17 to 31 bits), `_BIT` members and `_BIT` parameters
  (the old compiler refuses some: "Cannot resolve bit-length variables inside user defined types");
- fixed-length strings: `STRING * 0`, the length limit, a suffix form if any, assignment, comparison, `LEN`, members,
  arrays, `ASC`/`MID$`/`INSTR` on them, a fixed string passed to a `STRING` parameter (does the change reach it, cut
  and padded?), a parameter declared `STRING * n`, `SWAP`-free forms only;
- passing by reference across signedness (`study\02` §1.7 says the same storage is passed with a pointer cast), and
  across `_BIT` widths;
- `FOR` with each new type as variable, `SELECT CASE` with each new type as selector (the hidden copy's type), `CONST`
  with each new suffix;
- the special-cased built-ins with the new types: `LEN`, `STR$`, `HEX$`, `OCT$`, `_BIN$`, `ABS`, `SGN`, `VAL(…, type)`,
  `CINT`, `CLNG`, `_ROUND`, `INT`, `FIX`.

Findings go to `study\00` §5; a spec scenario or a decision below that disagrees is corrected and the correction
listed in task 1.5. A measured behaviour that looks like wrong code is implemented as the old compiler does it and
listed in `SOMEDAY.md` "QB64pe behaviours to review" (`DECISIONS.md`, 2026-10-08); only a crash, memory corruption,
hang or failed build is fixed, as `study\00` §6 decides.

*Done 2026-10-08* (`verification\v21_*`, `study\00` §5; corrections listed in task 1.5; the user's answers in
`DECISIONS.md`). What changed below: D5's rounding for `_BIT` targets, D6's arguments, D7's literals, and the
`_BIT` read type in D3 and D4. Task 1.6 (same day) measured where out-of-range and `_BIT`-suffixed literals and
`STRING * n` parameters are converted, so that D6 and D7 implement them as QB64pe does instead of "not supported
yet" (`v21_f_*`, `v21_x37`–`x41`).

### D2. The differential tester: a Rust generator, recorded once, run by the corpus runner

**Generator**: a workspace crate `crates\difftest` with a binary `qb64rust-difftest` (`gen [--check]`), not part
of the compiler. It writes `tests\differential\<group>\<name>.bas`. A Rust test in the same crate regenerates in
memory and compares with the files, so tier 1 catches a stale or hand-edited program without running anything
external. *Alternative:* a Python script under `tools\` like the other generators; rejected because tier 1 could
not check freshness without a Python step, and the generator needs the type list that `sema` defines (it uses
`sema::Ty`, so a new variant fails its build until the generator covers it). Randomness is a small seeded
xorshift (as `mutate.rs` does), no new dependency.

**Programs** (about 45, each a few thousand lines at most; one C++ build each):
- `ops\<op>.bas`, one per binary operator (`+ - * / \ MOD ^`, the six comparisons, `AND OR XOR EQV IMP`,
  `_ANDALSO _ORELSE`): one `DIM` per numeric type and value slot, then for every ordered pair of types eight value
  pairs: the extremes of both types (min/min, max/max, min/max, max/min where they differ), -1/1, 0/1, and two seeded
  random pairs; each line `PRINT "<op> <ltype> <rtype> <i>:"; a <op> b`.
- `unary\unary.bas`: `-`, `NOT`, `_NEGATE` on every type's values.
- `store\<target>.bas`, one per target type: every source type's values stored into the target and printed.
- `fold\<op>.bas`: the same pairs for the types that have a literal suffix, written as literals, so constant folding
  is checked against the old compiler's (which does not fold, `study\02` §1.4; the result must still match).
- `print\print.bas`: `PRINT` and `STR$` of every type's values.

Values are stored into typed variables by assignment from literals of the variable's own type (so the store under
test is not the one creating the value). Errors: `^` and the float conversions can raise; each program starts with
an `ON ERROR` handler that prints `ERR` and resumes next. The generator skips integer `\ 0` and `MOD 0` (fatal) and
the smallest-integer `\ -1`, `MOD -1` (crashes the old program, D-006), naming D-006 in the header comment. No
`PRINT` comma.

**From the measurements (task 1)**: the generator DIMs a `_BIT * 32` pad just before each `_BIT * n` with n > 32,
since such a variable overwrites the `_BIT` scalar allocated before it in the old compiler (D-009; otherwise the
recording would hold the corruption); it leaves out `^` with an `_OFFSET` operand (a compile error), and the
`fold` group uses literals of every integer suffix, `_BIT` suffixes included, at the extremes of the type and one
step beyond them (the held/believed split of D7), never a signed radix literal wider than its type (a compile
error).

**Recording and running**: the corpus runner already takes `--corpus-root`, `--record` (two runs, writes only equal
output) and `--list`. Record with `--suite corpus --corpus-root tests\differential --record` against `qb64pe.exe`;
run with `--qb64 target\release\qb64rust.exe --list tests\differential\pass.list`. `pass.list` is shrink-proof
(only grows) and names every program at the end of the change. CI's `tier2` job gains one step. *Alternative:*
running both compilers live in CI; rejected because it doubles CI time and needs `qb64pe` on every run, while the
recordings change only when the generator does.

**Size check**: if a program's C++ build with `qb64rust` takes longer than about 20 s, the generator splits it by
left type. The number of value pairs (eight) is the knob for output size.

*As built (group 2, 2026-10-08):* 59 programs (`fold` one per operator, `store` one per target); 17 types, `_BIT * n`
represented by `_BIT`, `_UNSIGNED _BIT * 7`, `_BIT * 24` and `_UNSIGNED _BIT * 40`; seven value slots per type,
DIMmed once, so the eight value pairs are pairs of slots. `_FLOAT`'s extremes are ±`1.797693134862315F+308`: the old
compiler writes a `_FLOAT` literal as a C++ double, so `1.18973149535723176F+4932` is infinite; it is kept as
`_FLOAT`'s step beyond its range in `fold` and `print`. Every program declares all 17 types, so none compiles with
`qb64rust` before the new types do (task 2.6); `gen --types` writes a subset elsewhere for a look at part of the
set. Per program: about 2.1 s with `qb64rust` (`study\19` §6), far from the 20 s split. *Changed after review
(`DECISIONS.md` 2026-10-08, task 2.7):* the six-type subset is also kept, recorded, as the group `old6` (48
programs), so tier 2 covers today's types until the full programs compile.

### D3. `Ty` gains variants and loses its derived order

```text
I8, U8, I16, U16, I32, U32, I64, U64, Off, UOff, Bit { width: u8, signed: bool },
F32, F64, F80, Str, FixedStr(u32), User(TypeId)
```

`Ty` stays `Copy`, `Eq`, `Hash`; `PartialOrd`/`Ord` go. In their place, methods with one meaning each:
`int_bits()` (8 to 64; `_OFFSET` 64; `_BIT * n` its storage, 32 or 64), `is_signed()`, `is_unsigned()`,
`float_rank()` (SINGLE < DOUBLE < `_FLOAT`), `storage()` (the C type the old compiler stores it in: `_BIT * n` →
`int32`/`uint32` up to 32, `int64`/`uint64` above; `_OFFSET` → `ptrszint`/`uptrszint`). A `_BIT` value is read
through `(int64)`: believed `_INTEGER64` whatever its width and signedness (measured: `_UNSIGNED _BIT * 64` holding
2^64-1 prints -1), held in its `storage()` type in arithmetic. `Off`/`UOff` are their own
variants, not `I64`/`U64`, because their operator rules differ (D4). `FixedStr(n)` is the type of a place only:
loading it gives a `Str` value (as the old compiler reads a fixed string through a `qbs` descriptor), so expression
typing never sees it. *Alternative:* `Int { bits, signed }` as one variant; rejected because `match` arms would then
test fields instead of naming the type, and the lint could no longer force each use to decide what it does with
`_UNSIGNED LONG`.

The step is done first with **no behaviour change**: every new variant is produced nowhere yet, the old order's
uses are rewritten with the new methods, snapshots and tier 2 stay identical.

### D4. Two promotion functions, checked by the tester

In `check\ops.rs`, beside `op_typing`, two functions replace `promote(..).max(..)`:

- `held(a, b)`: the C++ type of `a op b` as the old compiler emits it (no cast for `+ - *`): each operand's
  `storage()` C type, integer promotion (below 32 bits → `int32`), then the usual arithmetic conversions (if either is
  64-bit unsigned → `uint64`; else if either is 64-bit → `int64`; else if either is `uint32` → `uint32`; else
  `int32`), floats by rank. This is also the type comparisons compute in, so `-1 < u~&` is false, as in C++.
- `believed(a, b)`: the old compiler's markup (`study\02` §1.4): the wider float of the float operands; otherwise
  `_INTEGER64`, or `_UNSIGNED _INTEGER64` only when both are; with an `_OFFSET` operand, `_OFFSET` (unsigned unless an
  operand is a signed offset).

The `_OFFSET` operand rules (forced integral, `qbr` around `*` with a float and `/`, `^` an error) are a third arm of
`op_typing`. The existing three measured `ty`/`qb` differences fall out of these functions or stay as named special
cases; every rule is first written from `study\02` and then corrected until the `ops`, `unary` and `fold`
programs pass. *Alternative:* a table indexed by type pairs; rejected (`study\27` §5): the rules are short and
uniform, and a table of 15 × 15 entries per operator would hide them.

### D5. Conversions and stores

`ConvKind` keeps its four kinds; their choice moves to one function `conversion(from, to)`: `Widen` and `Truncate`
now cover signedness (truncation keeps the low bits; the value is read with the target's signedness), `RoundEven`
picks the old compiler's rounding by target (measured, `v21_b_float_stores`, `v21_b_bit_stores`): INTEGER, `_BYTE`
and their unsigned forms from SINGLE (`qbr_float_to_long`); every wider target **and every `_BIT * n`, whatever its
width** by `qbr` from `_FLOAT` to 64 bits, with its path above `INT64_MAX` for the unsigned 64-bit targets (not
`qbr_double_to_long` for 17 to 31 bits, as `study\02` §1.5 read it; that helper appears only for `CASE` items),
`Nearest` as now. The emitter writes the same C++ helpers the old compiler calls (`qbr_float_to_long`, `qbr`), so
rounding is libqb's, not ours.

A store into a `_BIT` place gets the mask or the sign extension in the emitter (`study\02` §1.7: unsigned
`v = e & mask`; signed: assign, then extend from bit n-1), decided by the place's type; the IR's store is unchanged.
`_BIT * n` scalars get storage of their `storage()` size, so n > 32 takes 8 bytes (D-009).

*As built (group 5, 2026-10-09):* `held` is `check\ops.rs` `held` (C's conversions on the held types, `_OFFSET` as
`int64`, `_UNSIGNED _OFFSET` as `uint64`), the markup `int_believed`, the `_OFFSET` arm `offset_typing` (with
`Typing::round_result` for the `qbr` around `*` with a float and `/`); `conversion(from, to)` is in `sema`'s
`lib.rs`, used by the checker and the IR's lowering. Corrected by the differential programs (task 5.3): a `_BIT`
value is held in its storage type and **believed its own `_BIT` type** (not `_INTEGER64`: the markup sees its width
and signedness; `Ty::printed` gives `_INTEGER64` for `PRINT`/`STR$` only); the markup is unsigned when both
operands are unsigned and one is 64 bits wide; `EQV`/`IMP` convert their left operand only to its own promoted type
(`left_operand`), because the old compiler complements it before C's conversions; literals whose suffix's type is
wider than 32 bits are `ll`/`ull` (D7 extended to `` `n `` and `` ~`n ``). The emitter writes the `_BIT` store mask
already in group 5 (`bit_store`), because every differential program stores into its `_BIT` slots; `_BIT` members,
parameters, arrays and `s32_bit` stay with group 6. What group 8 brings stays "not supported yet" behind
`Checker::later` and `later_types!()`: `FOR` variables, `SELECT CASE` selectors, FUNCTION results, constants used with
another suffix, arguments of the special-cased built-ins (all but `STR$` and LONG/DOUBLE slots), and a variable passed
to a parameter of the other signedness.

### D6. Fixed-length strings through libqb's fixed descriptors

A `FixedStr(n)` variable is `n` bytes plus a `qbs` made by `qbs_new_fixed(bytes, n, 0)`, NUL-filled at allocation, as
`study\02` §3.2 shows the old compiler does; assignment is `qbs_set` (libqb pads and cuts for a fixed `qbs`). A member
or element is read through a temporary `qbs_new_fixed(address, n, 1)` over the record's or array's bytes, as the old
compiler does for members (`udtreference`) and arrays (`__ARRAY_STRINGn_X`). `size_of(FixedStr(n))` is `n`, so
member offsets and `LEN` follow. Arguments, as measured (`v21_c_fixed_args`): a fixed string passed to a `STRING`
parameter is passed by reference even in parentheses, as a member or an element; the old compiler hands over a
fixed `qbs` and the procedure's stores go through `qbs_set`, so they are cut and padded. The IR's `Arg::Ref` of a
`FixedStr` place carries that descriptor. A FUNCTION named `f$n` has a `FixedStr(n)` result variable and returns
its n bytes as a `Str`, as the old compiler does (`fs$5("ab")` is `ab` and three blanks). A parameter declared
`STRING * n` or `t$n` is an ordinary `Str` parameter whose symbol records n; `LEN` of that parameter (the name
alone as `LEN`'s argument) folds to the constant n, and nothing else sees n (`v21_f_fixed_param`: stores are not
cut, `LEN(t + "!")` is the real length, a fixed variable passed to it is still cut by its own descriptor). n is read as a 32-bit integer, as the old compiler does (0 or a
negative result a compile error). Variable-length `STRING` members stay out (they need `initialise_udt_varstrings`
and per-type free lists).

*As built (group 7, 2026-10-09):* as above, with `Ty::held_value` and `Ty::believed_value` giving `Str` for a
`FixedStr` place's value, and `place_only_types!()` the arm of a `match` on a value's type. `name$n` is read by
`split_var_name` only where a scalar variable is named (`DIM`, `STATIC`, `SHARED`, assignment, an expression); every
other name keeps `name$n` "not supported yet" (FUNCTIONs `f$n` until task 8.2, arrays named so, constants, `FOR`).
A `TYPE` member's n must be a number (blocks are read before constants). A store into a member of an element copies
the string first (`qbs_set(qbs_new(0,1),…)`), so an index cannot change it. The `MID$` statement stays "not
supported yet" with the other built-in statements (`STATUS.md` step 9). Group 6 needed no new code (storage, mask
and errors were in from groups 4 and 5): only its tests, `s32_bit` and D-009's CLI test.

### D7. Declarations, suffixes and literals

`sema`'s suffix table (`check\decl.rs`) and `type_of` gain every QB64pe spelling; the lexer already tokenizes the
suffixes (every accepted program parses).

**Literals, as QB64pe** (`DECISIONS.md`, 2026-10-08; `v21_a_literals`, `v21_d_const`, `v21_f_literal_uses`,
`v21_f_radix`). The old compiler emits a suffixed decimal literal as its own digits (`ll`/`ull` added for the
64-bit suffixes) and only *types* it by the suffix. That is D4's two types: a suffixed literal's `ty` (held) is
the C++ type of that text (`uint64` for `~&&`; `int64` for `&&`, whose `ll` makes `2147483647&& + 1` 2147483648;
otherwise `int32` if the value fits, else `int64`; `uint64` beyond the `int64` range; a unary minus directly before
it is part of the text), its value the digits; its `qb` (believed) type is the suffix's. In
range or not, nothing else is needed: an operator computes in the held type (`4294967295~& + 1` is 4294967296,
which a `U32`-held literal would get wrong), a store or argument converts the held value to its own type
(`l& = 300~%%` is 300), and the places that convert to the believed type are the ones that already do:
`PRINT`/`STR$` format through a cast to the `qb` type (`PRINT 300~%%`, `PRINT (300~%%)`: 44), and a built-in whose
result keeps its argument's type converts the argument to it (`ABS(-1~&)` is 4294967295). `HEX$`/`OCT$`/`_BIN$`
take the held value and the `qb` width (`HEX$(300~%%)` is `12C`, `HEX$(-1~%%)` `FF`; a 64-bit `qb` type goes
through `m2-core-builtins`' non-place path instead: `HEX$(-1~&&)` is `""`, `HEX$(-2~&&)` `FFFE`). A `_BIT` `qb` type is read
through `(int64)` (D3), so a `_BIT`-suffixed literal is never narrowed (`` 9`3 `` prints 9). The emitter writes a
suffixed literal as the old compiler does (digits and the 64-bit suffix), so C++ does the arithmetic. Today's
"overflow" error for `40000%` goes (it prints -25536). A radix literal is read when lexed: its bits with the type's
signedness when they fit the width (`&HFF%%` is -1 everywhere), its whole value when an unsigned one is wider
(`&H1FF~%%`: held 511, prints 255); a signed one wider than its type is a compile error ("Overflow", `v21_x37`–`x41`).
A literal beyond 64 bits stays a compile error.

**Suffixed `CONST`s** (`check\constants.rs`, `consteval`): the evaluator computes the value as today (a suffixed
literal inside is its held value: `CONST r = 300~%%` is 300); an integer or `_BIT` suffix on the name then makes the
constant a suffixed literal whose digits are that value, rounded half to even if a float, a negative value under an
unsigned suffix written as its unsigned 64-bit value, as the old compiler writes it (`CONST b~%% = -1`: held
`uint64` 2^64-1, so `b~%% < 0` is false and `b~%% + 0` prints -1, while `PRINT b~%%` is 255). A float suffix
converts as an assignment, as today. `_BIT` parameters and members are compile errors (QB64pe
fails its C++ build, or refuses the member). Reserved-name rules (`reserved`, `reserved_proc`) were measured
only for no suffix and `&`; other suffixes on a keyword stay "not supported yet", unchanged.

*As built (group 4, 2026-10-09):* declarations come before the typing rules (D10), so the checker has a **gate**:
every declaration of a new numeric type is accepted, sized and emitted, but a *value* of one (a load, a store, a
literal or constant held or believed in one, a FUNCTION result, a `FOR` variable) is "not supported yet" until
groups 5 and 6 (`Ty::is_gated`; `gated_types!()` is the arm of each `match` on a value's type, `unproduced_types!()`
keeps `FixedStr`). Literals and constants of the old types out of range need no new rule and work end to end
(`32768%`, `CONST c% = 40000`). One simplification of the held type: an INTEGER literal in INTEGER's range stays
held as INTEGER, which C++ widens in every operation anyway. `_UNSIGNED STRING` turned out to be an error on a
parameter (`verification\v21_x42`, `x43`).

### D8. The decided fixes

- **D-005**: drop the `IMP` chain mark; the emitter already parenthesizes each operand, so `(a IMP b) IMP c` comes out
  as written. Pinned by a `cpp` snapshot and a CLI build-and-run test (the old compiler disagrees, so no slice
  program).
- **D-006**: the emitted `qb_safe_idiv`/`qb_safe_mod` templates test a divisor of -1 for signed integral operand
  types: `\` raises error 6 and returns the dividend when it is the type's minimum; `MOD` returns 0. The test is
  `if constexpr` on `std::is_signed`, so unsigned and float operands compile as before. Pinned by a CLI test.
- **D-007**: in `consteval`, `/` by 0 becomes an error and an integer power beyond `_INTEGER64` a DOUBLE. Neither can
  be recorded with the old compiler (it disagrees by decision), so `1 / 0` is pinned by a `check-fail` front-end
  test and `2 ^ 70` by a `typed` snapshot and a CLI build-and-run test.
- **D-008**: `check\constants.rs` no longer refuses a `CONST` after a label on its line. `check-ok` test and a CLI
  build-and-run test.
- **D-009**: by D5's storage rule; CLI build-and-run test.
- **Warning**: `consteval` reports, for a `CONST` value whose tree has a `^` whose right operand is an unparenthesised
  `^`, a `Diagnostic` with `Severity::Warning`. The driver prints warnings only with `-w` and leaves them out of the
  summary and the exit status (spec `compiler/cli`). The language server is parser-only and shows nothing new.

### D9. Tests

- Tier 1: the difftest crate's freshness test; `inputs.rs` walks `tests\differential` too (front end, round trip,
  and the "clean programs are listed" check, satisfied by `tests\differential\pass.list`); front-end tests per
  new declaration form; `typed`, `ir` and `cpp` snapshots for each type family.
- Tier 2: slice programs `s30_new_types` (declarations, suffixes, literals, `LEN`), `s31_unsigned_ops` (the cases
  that surprised during D4, as readable examples), `s32_bit` (`_BIT` stores), `s33_fixed_strings`, `s34_types_procs`
  (arrays, members, parameters, FUNCTIONs, `FOR`, `SELECT CASE`, `CONST` with the new types), recorded with
  `qb64pe.exe`; the differential programs; CLI tests for D-005, D-006, D-008, D-009 (build and run, `#[ignore]`d
  without the clone, as D-003's).
- Upstream: the programs that now pass are added to `pass.list`; the shrink-only lists shrink.

### D10. Order of work

1. Measurements (D1). 2. The tester, recorded, run once against today's `qb64rust` (most programs fail to compile:
expected). 3. `Ty` refactor without behaviour change (D3). 4. Declarations and literals (D7). 5. Typing,
conversions and emission (D4, D5) until the `ops`, `unary`, `store`, `fold`, `print` programs pass. 6. `_BIT`. 7.
Fixed-length strings (D6). 8. Arrays, members, procedures, `FOR`, `SELECT CASE`, `CONST`, built-in arguments. 9. The
decided fixes (D8). 10. Slice programs, upstream, docs. Each group keeps tier 1 and tier 2 green.

## Risks / Trade-offs

- [The old compiler's C++ for some type pair does not compile, or behaves differently between its two builds] →
  the recording shows it (a compile error is recorded as `.err`, and the runner's `--cpp-opt` can compare); such a
  pair follows `study\00` §6: a failed build or crash is fixed (and put in `DIVERGENCES.md`), anything else is
  implemented as the default build behaves and listed in `SOMEDAY.md` for review.
- [`_FLOAT` printing or `long double` arithmetic differs between the old compiler's build and ours] → both use the
  reference clone's llvm-mingw toolchain and libqb; `--record` already rejects output that differs between two runs.
- [CI time grows by the differential programs] → about 45 builds of a few seconds each; D2's size check splits a slow
  program; measured in task 2.5 and reported in `study\19`.
- [The tester only covers what it enumerates: nested expressions, mixed literal/variable operands] → the slice
  programs add readable mixed cases; a random-expression mode is a later option (non-goal).
- [Removing `Ty`'s order breaks code that sorted or compared types silently] → the compiler finds every use once
  the derive is gone; D3 is done as its own step with unchanged snapshots.
- [A measured case contradicts a spec scenario written before measurement] → task 1.5 corrects the delta and lists
  the correction, as earlier changes did.
- [`_OFFSET` is pointer-sized; only 64-bit Windows is built today] → `Off`/`UOff` are 64-bit by D3; a 32-bit target is
  not planned.

## Migration Plan

None: no user-visible behaviour of a program that compiles today changes, except D-005 to D-008, which change only
programs that were "not supported yet" or crashed. Rollback is reverting the change.

## Open Questions

- Whether the eight value pairs per type pair find everything the old compiler does with mixed signedness, or a
  denser set is needed: answered by the first run (task 2.6); changing the number touches only the generator.
  *Answered 2026-10-08:* eight are enough for now (task 2.6: every mixed-signedness regime shows in at least two
  pairs; the differences found came from the extremes).
