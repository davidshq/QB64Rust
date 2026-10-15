# Proposal

## Why

This is step 8 of M3's order of work (`STATUS.md`, "Next"; user: "do step 8 of m3", then "All five" and "Flag
only, no vWatch" to Claude's two questions, 2026-10-14). After `m3-mem` the demand report (`qb64rust --report
unsupported` over `tests\upstream`, 2026-10-14) names, outside `deferred.list`, few features that free programs
by themselves; grouped by the programs they free, the next cluster is small and of one kind: metacommands that
only set a flag or a global, and a libqb part that already exists. Per program (a script over the report, each
upstream program's set of "not supported yet" messages):

- **`$ASSERTS`, `$ASSERTS:CONSOLE`** hold back `precomp-flags/asserts` and `precomp-flags/assertscon` alone.
- **`$DEBUG`** holds back `precomp-flags/debug` and `auto_include/debug` alone.
- **`_DEFLATE$`, `_INFLATE$`, `_BASE64DECODE$`** hold back `console_only/libz`, `libz/test`, and the image
  tests `image/pcx_test` and `image/webp_test` (their shared helper `imageassert.bm` decodes its reference
  images with them).
- **`SYSTEM` with an exit code** is used in 15 programs (228 uses) and alone holds back
  `arrays/t_qb64pe_refactor_preserve_inline_varstrings`; the others need `REDIM _RETAIN`, `_ARRAYCOPY` or
  `$Unstable` too.
- **`-f:GenerateLicenseFile=true`** is asked for by every upstream program with a `.win.license` file beside it:
  `libz/test`, and `audio_out/test` and `image/test`, which compile cleanly today and are on
  `tests\known_clean_not_passing.list` for this reason alone.

Read before this proposal (`qb64pe.bas`, `subs_functions.bas`, libqb, `vwatch.bm` and the `Makefile` at the
reference clone's commit):

- **`$ASSERTS` sets a global**: the old compiler writes `int32 asserts=1;` (else `0`) and sets `_ASSERTS_`;
  `$ASSERTS:CONSOLE` also sets `_CONSOLE_` and gives the program a console (`qb64pe.bas` 1978–1988, 13088).
  **`_ASSERT condition[, message$]`** is a row of the table (`sub__assert`), which returns at once when
  `asserts` is 0, and otherwise raises error 314, or, in a program with a console and with a message, echoes the
  message and raises error 315 (`libqb.cpp` 5892).
- **`$DEBUG` makes the old compiler write a call of `SUB_VWATCH` on every line** and include
  `internal\support\vwatch\vwatch.bm`, and from the command line it warns "$DEBUG features only work from the IDE"
  (`qb64pe.bas` 2004, 3553). At run time `vwatch` reads `QB64DEBUGPORT`; when it is empty it sets `vw_bypass`
  and every later call returns at once (`vwatch.bm` 684–687). A program built with `$DEBUG` and run outside the
  IDE does what the same program without `$DEBUG` does.
- **The six data-processing functions are plain rows** (`subs_functions.bas` 4019–4100): `_ADLER32`, `_CRC32`,
  `_DEFLATE$` (`?[,?]`), `_INFLATE$` (`?[,?]`), `_BASE64DECODE$`, `_BASE64ENCODE$`, each with
  `DEPENDENCY_ZLIB`, which links libqb's data-processing library (`DEP_ZLIB=y`; `Makefile` 437, the same library
  the audio part already links).
- **`SYSTEM n` and `END n`** evaluate `n` as an `_INTEGER64`, check `qbevent`, and store it into `exit_code`
  before the usual end (`qb64pe.bas` 9850–9895).
- **`-f:GenerateLicenseFile=true`** adds `GENERATE_LICENSE=y` to the make line (`qb64pe.bas` 13772); the
  `Makefile` then writes `<exe>.license.txt` from the licences of the parts it links (`Makefile` 508–529).

## What Changes

- **`$ASSERTS` and `$ASSERTS:CONSOLE`** accepted: the flag, the `asserts` global, and with `:CONSOLE` the
  console, as the old compiler does; no longer "not supported yet".
- **`_ASSERT condition[, message$]`**, a built-in statement, with errors 314 and 315 at run time.
- **`$DEBUG`** accepted: `_DEBUG_` set and QB64pe's warning "$DEBUG features only work from the IDE" given with
  `-w`; **no vWatch code is written** (user's decision). A program behaves as the old compiler's does outside its
  IDE; it cannot be debugged from the QB64pe IDE. Recorded in `DIVERGENCES.md`.
- **`_ADLER32`, `_CRC32`, `_DEFLATE$`, `_INFLATE$`, `_BASE64ENCODE$`, `_BASE64DECODE$`**, rows of the table, with
  a new runtime part that adds `DEP_ZLIB=y` in the old compiler's place among the make flags.
- **`SYSTEM n` and `END n`**: the exit code, as the old compiler writes it.
- **`-f:GenerateLicenseFile=true|false`**: `GENERATE_LICENSE=y` and the licence file beside the executable.
- **Tests and lists**: `verification\v33_*` (the measured forms, accepted and rejected, the runtime errors, the
  exit codes); a call-site program per new form; front-end tests; console slice programs on
  `tests\corpus\slice.list`; the upstream programs that pass on `tests\upstream\pass.list` (about 185 of 279,
  from 174); `tests\known_clean_not_passing.list` shrinks by the licence tests.

## Capabilities

### New Capabilities

- `language/asserts-and-debug`: `$ASSERTS`, `$ASSERTS:CONSOLE`, `_ASSERT` and its runtime errors, `$DEBUG` and
  what it does and does not do here.
- `language/data-processing`: the checksums, compression and base64 functions and the runtime part they link.

### Modified Capabilities

- `language/builtin-functions`: the six functions join the list.
- `language/builtin-statements`: `_ASSERT` joins the list.
- `language/control-flow`: `END` and `SYSTEM` with an exit code (added requirement).
- `compiler/cli`: `-f:GenerateLicenseFile` is a supported setting; the scenario that used it as the unknown
  setting names another.

## Impact

- Changed: `crates\syntax` (`pp.rs`: the metacommands as settings), `crates\sema` (rows, the statement, the exit
  code, the runtime part, the warning), `crates\ir` (the exit code, the settings), `crates\codegen-cpp` (`asserts`
  global, the exit code), `crates\driver` (the make flags, `GenerateLicenseFile`, moving the licence file with the
  executable, the build cache key); new `tests\callsite\*.bas`, `tests\frontend\*.bas`, `tests\corpus\slice\*`,
  `verification\v33_*`; `tests\corpus\slice.list`, `tests\upstream\pass.list`, the shrink-only lists.
- No change to libqb or the reference clone.
- Not in this change: vWatch (the IDE's debugger); `$CHECKING`; `_MD5$` (a row of the font part, no upstream
  program waits on it alone); `_EMBED`/`_EMBEDDED$`; `$VERSIONINFO`; the other "free alone" items of the report
  (`_ERRORLINE`, `_CONTINUE`, `_OS$`, `CLEAR`, …).
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\upstream\README.md`, `study\00` §5, `SOMEDAY.md`,
  `DIVERGENCES.md`, `DECISIONS.md`, and `CLAUDE.md`'s layout table updated when done.
