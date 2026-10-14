# Proposal

## Why

This is step 7 of M3's order of work (`STATUS.md`, "Next"; user: "Do 1", 2026-10-14): `_MEM`, which holds back
more upstream tests than any feature outside `deferred.list`. The demand report (`qb64rust --report unsupported`
over `tests\upstream`, 2026-10-14) counts the type `_MEM` in 30 programs (alone in 5), `_MEMFREE` in 22,
`_MEMGET` in 17, `_MEMIMAGE` in 14, `_MEM` in 9, `_MEMPUT` in 4, `_MEMFILL` in 2, `_OFFSET` in 4, and `MKL$` in
15. The image tests share a helper (`utilities\imageassert.bm`) that reads both images through `_MEMIMAGE` and
`_MEMGET` and writes a BMP header with `MKL$` and `MKI$`, and the six `paint/tile_*` tests need `MKL$`. Every
corpus program the old compiler accepts compiles already, so this step is tested mostly by programs of its own;
about 25 of the 39 upstream programs that name `_MEM` are also held by `_DYNAMIC` members or `$Unstable`
(deferred), and stay held.

Read before this proposal (`qb64pe.bas`, libqb at the reference clone's commit, one program compiled with
`qb64pe -z`):

- **`_MEM` is a user type the old compiler defines before the program's first line** (`qb64pe.bas` 1588): 56
  bytes, members `OFFSET`, `SIZE`, two hidden lock members, `TYPE`, `ELEMENTSIZE` (all `_OFFSET`), `IMAGE` and
  `SOUND` (LONG). It is libqb's `struct mem_block` (`libqb\include\memblock.h`). Its members can be read and
  never stored to ("Cannot set read-only element of _MEM TYPE"). Whole values copy as other user types do.
- **The locks already exist in this compiler's output**: every SUB and FUNCTION opens `sf_mem_lock` and frees it
  on exit, and every array descriptor carries a lock that `REDIM` and `ERASE` renew (`codegen-cpp\procs.rs`,
  `decl.rs`, `arrays.rs`, written to match the old compiler). `_MEM` uses those locks to see that a block is
  still valid.
- **`_MEM(x)` is not a plain call**: the old compiler builds the arguments of `func__mem` from the reference
  (address, size, type value, element size, lock), with rules for a variable, an element, a whole array `a()`, a
  member, a fixed-length string, and errors for a variable-length string (`qb64pe.bas` 22730–23038, the "-7
  helper"); `_MEMELEMENT` is the same with the element alone (the "-8 helper", 22406). `_MEM(offset, size)`
  calls `func__mem_at_offset`.
- **`_MEMGET`, `_MEMPUT` and `_MEMFILL` are written inline**: a bounds and lock check (errors 300 and 308), then
  a copy of the bytes, by the type named after `AS` or the variable's type (`qb64pe.bas` 10168–10500,
  20573–20630). `_MEMCOPY`, `_MEMNEW`, `_MEMFREE`, `_MEMEXISTS`, `_MEMIMAGE` and `_MEMSOUND` are calls of libqb.
- **`MKI$`, `MKL$`, `MKS$`, `MKD$`, `_MK$`, `CVI`, `CVL`, `CVS`, `CVD` and `_CV` are written by the old compiler,
  not by the table** (`qb64pe.bas` 20419–20441, 21217–21296): `l2string(x)`, `string2l(s)` and so on, one
  function of libqb for each numeric type, `_MK$` and `_CV` taking the type as their first argument (a
  `TypeArg`, which this parser reads already for `VAL`).

Three things were the user's to decide (2026-10-14, `DECISIONS.md`): `MKI$` … `_CV` come in with `_MEM`, and
`_INFLATE$`, `_DEFLATE$` and the base64 functions do not; `$CHECKING` stays "not supported yet" (with it the
forms the old compiler writes without checks); `_MEMSOUND` is compiled and never run, since `_SNDOPEN` is not
supported.

## What Changes

- **The type `_MEM`**: `DIM`, `REDIM`, `SHARED`, `STATIC`, arrays of it, a member of a user type, a parameter
  and a FUNCTION result where the old compiler takes them; its members read; a store to a member an error, as
  in the old compiler. It no longer is "not supported yet".
- **`_MEM(reference)`, `_MEM(offset, size)`, `_MEMELEMENT(reference)`**, with the old compiler's rules for each
  kind of reference this compiler supports (a variable, an element, a whole array, a member, a member array, a
  fixed-length string) and its errors for the others; a reference this compiler does not support yet (a
  `_DYNAMIC` member) stays "not supported yet".
- **`_MEMNEW`, `_MEMFREE`, `_MEMEXISTS`, `_MEMIMAGE`, `_MEMCOPY`**, plain calls; **`_MEMSOUND`** compiled and
  never run.
- **`_MEMGET` (function and statement), `_MEMPUT`, `_MEMFILL`**, written inline with the old compiler's checks,
  by a type named after `AS`, a numeric, string or user-type variable, as measured.
- **`_OFFSET(reference)`**, the address of a variable or array.
- **`MKI$`, `MKL$`, `MKS$`, `MKD$`, `_MK$`, `CVI`, `CVL`, `CVS`, `CVD`, `_CV`**, for every numeric type the old
  compiler takes, `_BIT` and `_UNSIGNED _BIT` included (width 1: a width cannot be written there, measured by
  task 1.1).
- **Tests and lists**: `verification\v32_*` (measured forms, accepted and rejected, and the runtime errors); a
  call-site program per form and argument type; `typed` and `ir` front-end tests; console slice programs on
  `tests\corpus\slice.list` (one with an image read through `_MEMIMAGE`); the upstream programs that pass on
  `tests\upstream\pass.list`; the shrink-only lists regenerated.

## Capabilities

### New Capabilities

- `language/mem-blocks`: the `_MEM` type and the blocks it describes: what `_MEM`, `_MEMELEMENT`, `_MEMNEW`,
  `_MEMIMAGE` and `_MEMSOUND` give for each kind of reference, the members of a block, reading and writing
  through one (`_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_MEMCOPY`), freeing and checking one (`_MEMFREE`,
  `_MEMEXISTS`), when a block stops being valid (its procedure returns, its array is redimensioned or erased,
  it is freed) and the runtime errors, and `_OFFSET(reference)`.

### Modified Capabilities

- `language/builtin-functions`: the new functions, `MKI$` … `_CV` among them, join the list; an argument of type
  `_MEM` is no longer "not supported yet" where a function takes one.
- `language/builtin-statements`: `_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_MEMCOPY`, `_MEMFREE` join the list.
- `language/numeric-semantics`: "`_MEM` SHALL stay not supported yet" goes; `_MEM` is a type (`language/
  mem-blocks`).
- `compiler/pipeline`: the scenarios of "No follow-on errors after an unsupported declaration" use `DIM m AS
  _MEM` as their unsupported declaration; they name another one.
- `testing/compiler-tests`: `_MEMSOUND` joins the built-ins that no test runs.

## Impact

- Changed: `crates\sema` (the predefined type, the `_MEM` references, the inline statements, `MK$`/`CV`, rows,
  `NEVER_RUN`), `crates\ir` (new nodes where a form is not a plain call, `dump.rs`), `crates\codegen-cpp` (the
  `func__mem` arguments, the inline checks, the result of a function of a user type), `crates\syntax` only where
  a measured form does not parse (`_MEMPUT m, o, v AS type`, `_MEMCOPY … TO …`); new `tests\callsite\*.bas`,
  `tests\frontend\*.bas`, `tests\corpus\slice\*`, `verification\v32_*`; `tests\corpus\slice.list`,
  `tests\upstream\pass.list`, the shrink-only lists; `tests\frontend` tests that use `_MEM` as their example of an
  unsupported declaration get another one.
- No change to libqb or the reference clone; no new libqb part (`_MEMSOUND` names the audio part, which exists).
- Not in this change: `$CHECKING` and the forms written without checks; `_INFLATE$`, `_DEFLATE$`,
  `_BASE64ENCODE$`, `_BASE64DECODE$` (DEP_ZLIB); `_SNDOPEN` and the other sound handles; `_MEM` over a
  `_DYNAMIC` or `_STATIC` member (deferred with them); `SYSTEM` with an exit code.
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `study\00` §5, `SOMEDAY.md` (what
  stays marked; questionable behaviours), `DIVERGENCES-QB45.md` where needed, `DECISIONS.md`, and `CLAUDE.md`'s
  layout table updated when done.
