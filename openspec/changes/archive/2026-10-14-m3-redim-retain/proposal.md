# Proposal

## Why

The user took `REDIM _RETAIN` and the `$UNSTABLE:TYPEFIELDS` member markers out of `SOMEDAY.md` ("Do 1, 2, 3",
`DECISIONS.md` 2026-10-14), `REDIM _RETAIN` first: it frees upstream programs by itself, and 21 of the
`TYPEFIELDS` programs use it too, so the second change (`$UNSTABLE:TYPEFIELDS`) builds on it. Every upstream
program with `_RETAIN` is on `tests\upstream\deferred.list` today (28; 7 of them for `_RETAIN` alone, one of
those an `.err` program), and 14 upstream programs with `SYSTEM n` wait on it, `_ARRAYCOPY` or `$Unstable`.

Read and measured before this proposal (`qb64pe.bas` at the reference clone's commit; `verification\v34_*`, 28
programs recorded with `qb64pe.exe`, 2026-10-14):

- **`_RETAIN` is a third mode of `REDIM`** (`redimoption = 3`, `qb64pe.bas` 8779), allowed only as the word right
  after `REDIM`, before an optional `SHARED`. `REDIM SHARED _RETAIN`, `REDIM _PRESERVE _RETAIN`, `REDIM _RETAIN
  _PRESERVE`, `REDIM _RETAIN _RETAIN` and `DIM _RETAIN` are "Invalid variable name"; `REDIM _RETAIN` alone is
  "Expected REDIM _RETAIN ..." (`v34_x01`–`x07`, `x22`, `x23`). In everything else it is `_PRESERVE`'s
  statement: the same rules for a list of names, the type before the names, `REDIM _RETAIN SHARED` inside a
  procedure, another type or number of dimensions, `name()` and `name AS type`, `$DYNAMIC`, `COMMON`, `SUB …
  STATIC`, an array parameter, a static array (error 10) (`x02`, `x06`, `x08`, `x11`–`x13`, `x15`–`x21`, `x24`,
  `v34_c`).
- **It keeps elements by coordinates**, where `_PRESERVE` keeps them by flat position: for each dimension the
  overlap of the old and new ranges, and every element whose indexes lie in all of them is copied to the same
  indexes; the rest are zero or empty (`qb64pe.bas` 14782–14812, 15228–15296; `v34_a`, `v34_b`). The copy of
  one element is `_PRESERVE`'s: `qbs_set` for a string, `setbits`/`getubits` for a `_BIT` element, the parts of
  a user type with strings, `memcpy` for the rest (also fixed-length strings, member arrays and conventional
  memory, `v34_b`, `v34_c`).
- **On a member array (`REDIM _RETAIN t.v(…)`) it does what `_PRESERVE` does: nothing** (`qb64pe.bas` 8988–8993;
  `v34_x09`, `x10`).
- **A FUNCTION in a bound that gives the same array new bounds ends the old compiler's program** (exit 127,
  `v34_d`): the old block is captured before the bounds are evaluated, as with `_PRESERVE`.

## What Changes

- **`REDIM _RETAIN [SHARED] name(bounds) [AS type], …`** compiles, with `_PRESERVE`'s rules and the old compiler's
  coordinate copy for numbers, strings, fixed-length strings, `_BIT` and `_UNSIGNED _BIT * n`, user types with and
  without strings or member arrays, arrays in conventional memory, local, shared, `STATIC` and parameter arrays.
  No longer "not supported yet".
- **The word order is checked as the old compiler checks it**: the forms above that it rejects are compile
  errors here, each with its own message.
- **`REDIM _RETAIN` of a member array** is accepted and does nothing, as `REDIM _PRESERVE` of one.
- **The IR's allocation gets a third mode**, keeping the elements by coordinates.
- **Tests and lists**: `verification\v34_*` (the measured forms and behaviour); front-end tests (the word
  orders, the IR mode, the C++ of each kind of element); console slice programs on `tests\corpus\slice.list`;
  the seven `_RETAIN`-only programs leave `tests\upstream\deferred.list`, and those that pass go on
  `tests\upstream\pass.list` (six, about 192; `array_elements_change`, an `.err` program, is then rejected by a
  real error).

Out of scope: the `$UNSTABLE:TYPEFIELDS` programs that also use `_RETAIN` (the next change), whole-array
assignment and `_ARRAYCOPY` (still deferred).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/arrays-and-types`: requirement "REDIM" (the words after `REDIM` now include `_RETAIN`), and a new
  requirement "REDIM _RETAIN" (with its member-array case, so "Member arrays" is left as it is).
- `compiler/pipeline`: requirement "Array storage in the IR" (the allocation's third mode).

## Impact

- `crates\sema` (`check\decl.rs`: the words after `REDIM`; `check\places.rs`: the mode of the allocation and of a
  member array; `lib.rs`: `AllocMode`), `crates\ir` (the mode, its dump and validation), `crates\codegen-cpp`
  (`arrays.rs`: the old descriptor kept and the coordinate walk).
- Tests: `tests\frontend\` (new programs and snapshots; `redim_unsupported` loses its two `_RETAIN` lines),
  `tests\corpus\` (new slice programs and their recorded output), `tests\upstream\deferred.list`, `pass.list`,
  `tests\known_unsupported_rejections.list` if `array_elements_change` is on it.
- Documents: `SOMEDAY.md` (the `_RETAIN` row goes), `DIVERGENCES.md` if the bound-that-redims case differs,
  `study\00` §5, `crates\README.md`, `tests\upstream\README.md`, `STATUS.md`, `ROADMAP.md`.
