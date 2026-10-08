# Proposal

## Why

The new compiler knows six numeric types (INTEGER, LONG, `_INTEGER64`, SINGLE, DOUBLE, `_FLOAT`) and variable-length
strings. Every other type QB64pe has is "not supported yet": `_BYTE`, the `_UNSIGNED` types, `_BIT`, `_OFFSET`, and
fixed-length strings (`STRING * n`). They are the wall in front of the upstream suite. Counted 2026-10-08 with the
release build, **127 of the 348 upstream programs the old compiler accepts carry a mark for one of these types**
(the first such mark: `_BYTE` 50, fixed-length strings 29, `_OFFSET` 12, unsigned members and suffixes the rest),
and 12 carry no other mark. In the corpus only 3 programs do. Step 8 of `STATUS.md` puts this work next, together
with the differential tester: typing so far has been derived by hand from `study\02` and checked one measured case
at a time, and with unsigned and 8-bit types mixed with signed ones the C++ promotion rules the old compiler relies
on are no longer a total order (`study\20` §3.3, `study\27` §5). The bug-compatibility choices this work depends on
were decided on 2026-10-08 (`study\00` §6), so the change can also implement the fixes that need no new runtime.

## What Changes

- **Measurements before design**: `verification\v21_*` programs run with `qb64pe.exe` for what the tester cannot
  generate: declarations and suffixes of the new types, their literals, `_BIT` stores, fixed-length strings
  (assignment, comparison, `LEN`, members, arrays, arguments), passing by reference across signedness, `FOR` and
  `SELECT CASE` with the new types, `CONST` with their suffixes, and the built-ins that special-case their
  argument's type.
- **Differential tester**: a seeded, deterministic generator writes BASIC programs that apply every operator to
  every pair of numeric types, convert every type to every other, and print each result, at boundary values and
  seeded random ones. Their expected output is recorded once with `qb64pe.exe` by the corpus runner, so the check
  needs no old compiler; tier 1 parses and checks them, tier 2 and CI build and run them with `qb64rust`.
- **The full numeric type set**: `_BYTE`, `_UNSIGNED _BYTE`, `_UNSIGNED INTEGER`, `_UNSIGNED LONG`,
  `_UNSIGNED _INTEGER64`, `_OFFSET`, `_UNSIGNED _OFFSET`, `_BIT` and `_BIT * n` (signed and unsigned), with their
  `AS` names, type suffixes and literal suffixes, in variables, static arrays (except `_BIT` arrays), `TYPE`
  members, parameters, FUNCTION results, `FOR`, `SELECT CASE` and `CONST`. `Ty` gains the variants and loses its
  derived order; an explicit rank and the two promotion rules (what C++ computes in, and what the old compiler
  believes) take its place, checked by the tester (`study\27` §5: no type table).
- **Fixed-length strings**: `STRING * n` variables, static arrays and `TYPE` members, NUL-filled at the start
  (`DIVERGENCES-QB45.md` Q-005), padded with spaces or cut on assignment.
- **Decided fixes** (`DIVERGENCES.md`): `a IMP b IMP c` computed as `(a IMP b) IMP c` (D-005); the smallest LONG
  or `_INTEGER64` `\ -1` raises error 6 and `MOD -1` gives 0 (D-006); `CONST … 1 / 0` a compile error and an
  integer power beyond `_INTEGER64` a DOUBLE (D-007); `label: CONST` on one line compiles (D-008); a `_BIT * n`
  scalar with n > 32 gets its own 8 bytes (D-009).
- **The first compiler warning**: a `CONST` expression that chains `^` without parentheses (right-associative in a
  `CONST`, left-associative at run time) gets a warning, shown with `-w` as `qb64pe` shows its warnings.
- **Not in this change**: `_MEM` and the built-ins that take or return the new types beyond the 43 already supported
  (step 9, by demand); `_BIT` arrays (bit-packed storage); variable-length `STRING` members of a `TYPE`; `DEFxxx` and
  `_DEFINE` (step 11); D-010 to D-013, which are libqb's (the `INF` text, the console comma, the exit code after a
  runtime error, two registrations) and wait for the runtime copy of M3, except D-013, which waits for its two
  built-ins. `_BIT` parameters and members are compile errors, as in QB64pe.
- **As QB64pe, also where it looks odd** (`DECISIONS.md`, 2026-10-08; listed in `SOMEDAY.md` for review): suffixed
  literals and `CONST`s outside their type's range and `_BIT`-suffixed ones, held as written and converted only
  where QB64pe converts them (`PRINT 300~%%` is 44, `300~%% + 0` is 300); parameters declared `STRING * n` (a `STRING`
  parameter whose `LEN` is n); FUNCTIONs named `f$n`.

## Capabilities

### New Capabilities
- `testing/differential-tests`: the generated programs that compare the new compiler's results with the old
  compiler's for every operator, type pair and conversion: how they are generated, recorded and run in each tier.
- `language/fixed-length-strings`: `STRING * n` variables, arrays and members: initial contents, assignment,
  reading, comparison, `LEN`, argument passing.

### Modified Capabilities
- `language/numeric-semantics`: the numeric type set; literal suffixes of the new types; arithmetic width and
  result types with unsigned, 8-bit, `_BIT` and `_OFFSET` operands; stores into every integer type, including
  `_BIT`'s mask and sign extension; `IMP` chains (D-005); the smallest integer divided by -1 (D-006).
- `language/constants`: `CONST` with the new suffixes; `1 / 0` a compile error and an integer power beyond
  `_INTEGER64` a DOUBLE (D-007); `label: CONST` (D-008); the warning for a chained `^`.
- `language/arrays-and-types`: arrays of the new types and of fixed-length strings; `TYPE` members of the new
  numeric types and of fixed-length strings.
- `language/procedures`: parameters and FUNCTION results of the new types (not `_BIT` parameters); by reference
  across signedness and between the 64-bit integer and `_OFFSET` types.
- `language/control-flow`: `FOR` variables of the new types (the wider type of the hidden values; a `_BIT`
  variable a compile error). `SELECT CASE`: how an item is compared with the selector, stated as measured (an
  integer item is not converted to the selector's type, which shows with unsigned selectors).
- `language/builtin-functions`: arguments of the new types to the supported built-ins.
- `compiler/cli`: warnings printed with `-w`.
- `compiler/pipeline`: the "no follow-on errors" scenarios, whose example declaration (`STRING * 5`) becomes
  supported.

## Impact

- `crates\sema`: `Ty` (new variants, no derived order, `size_of`), the typing rules in `check\ops.rs`, literals,
  declarations and suffixes, conversions, the `CONST` evaluator, `FOR` and `SELECT`; `crates\ir` (lowering and
  validation of the new types); `crates\codegen-cpp` (C types and names, stores with masks, fixed strings through
  libqb's `qbs_new_fixed`, `qb_safe_idiv`/`qb_safe_mod`); `crates\driver` (warnings with `-w`). About 520 matches on
  `Ty::` in 24 files are touched once; `wildcard_enum_match_arm` makes each say what it does with the new variants.
- New: `crates\difftest` (generator, design D2), `tests\differential\` (generated programs and their recorded output),
  slice programs `s30` onwards, `verification\v21_*`; the runner and CI gain a step for the differential programs.
- No libqb change: the reference clone stays read-only; the runtime functions the new types need (`qbs_str` for every
  integer width, signed and unsigned, and `qbs_new_fixed`) already exist there.
- Docs: `crates\README.md`, `study\00` §5 (measured facts), `DIVERGENCES.md` (pinned-by for D-005 to D-009),
  `DIVERGENCES-QB45.md` (Q-005), `STATUS.md`.
