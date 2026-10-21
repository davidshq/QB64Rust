# Proposal

## Why

The fourth change of M4 (user, 2026-10-21: `STATUS.md` "Next" items 2 and 3) takes the six built-ins that are
the only "not supported yet" mark of one upstream program each, and the one known bug. The upstream demand report
of `m4-names-arrays`'s close names them as "alone" lines: `_CAST` (`cast/cast_test`, 105 uses), `_CLIPBOARD$`
(`clipboard/clipboard_test`), `_FILES$` (`filesystem/filesys_test`), `_MD5$` (`hash/test`), `_OS$`
(`audio_sound_test/sound_test`) and `CLEAR` (`arrays/95_type_codegen_clear_inline_owner`). The bug is in
`STATUS.md`'s "Known bugs". A SUB or FUNCTION parameter can be typed by a `TYPE` whose block stands below the
procedure's header in the file. We accept that program, but the old compiler rejects it with "Illegal SUB/FUNCTION
parameter" (`verification\v48_x12`, `x25`, on `tests\verification\known.list`).

Found before this proposal, from the table (`tools\builtins\builtins.json`), `qb64pe.bas` and libqb:

- **`_OS$`, `_MD5$(s$)`, `_CLIPBOARD$` and `_CLIPBOARD$ = s$`** are plain rows of the table: `func__os`,
  `func__md5` (with `DEPENDENCY_LOADFONT`, which is our `RuntimePart::Font`), `func__clipboard` and
  `sub__clipboard` (special format `=?`, the form `_CLIPBOARDIMAGE = h` already takes).
- **`_FILES$[(spec$)]`** is a row with the special format `[?]`. libqb has `func__files(qbs*, int32 passed)`: the
  first call names the folder or pattern, and each bare call gives the next entry, then "".
- **`_CAST(type, value)`** is special-cased in `evaluatefunc` (`qb64pe.bas` 20394, 20718). The first argument is a
  numeric type name, and any other type is the error "_CAST TYPE unsupported". The second is any number, and a
  string or a whole user type is the error "Expected numeric argument". The call is written as a C++ cast, `((<ctype>)(value))`.
  The parser already reads the type as a `TypeArg`, as for `_MK$`.
- **`CLEAR [a][, [b][, c]]`** is a row with a special format, `sub_clear(int32, int32, int32, passed)`, whose body
  includes the program's `clear.txt`. We already write that file for every program (`codegen-cpp`, `decl.rs`
  `clear`). `qb64pe.bas` 11318 gives two errors: "CLEAR cannot be used inside a SUB/FUNCTION", and "Incorrect
  number of arguments" for a parenthesis among the arguments (upstream `arrays/45_err_solved_bug_Incorrect_nr_of_args`,
  now on `tests\known_unsupported_rejections.list`).
- **The parameter's type**: the old compiler's prepass reads a SUB's header when it meets it. So a parameter's
  `TYPE` must be defined by a block above the header, at the top level or in an earlier procedure. A `TYPE` block
  anywhere still serves `DIM … AS` (`m4-names-arrays`). `DECLARE LIBRARY` already holds its parameters to the
  types above the block (`check\library.rs`, `types_before`, `v37_x15`).

## What Changes

- **`_OS$`, `_MD5$`, `_CLIPBOARD$` (function and statement), `_FILES$` with and without its argument**: compiled
  as the plain calls of their rows. `_MD5$` brings in libqb's font part, as the old compiler's dependency does.
  `clipboard_test` sets the machine's clipboard when it runs, as it already did in the old compiler's recorded
  runs (design D3).
- **`_CAST(type, value)`**: the value converted to the numeric type named, written as the old compiler writes it.
  The result has that type. Its errors are those of the old compiler.
- **`CLEAR`**: resets the program's variables, closes files, resets `DATA`, `ON ERROR` and `RETURN`, through
  libqb's `sub_clear` and the `clear.txt` already written. Its two errors are those of the old compiler.
- **A parameter typed by a `TYPE` defined below the procedure's header**: a compile error, as in the old compiler.
  `v48_x12` and `x25` move from `known.list` to `pass.list`, and the bug leaves `STATUS.md`.
- **Tests**: a measurement family `v49` in `verification\`, slice programs, front-end tests and call-site programs
  for each new built-in. The six upstream programs go onto `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/builtin-functions`: "Supported built-in functions" adds `_CAST`, `_OS$`, `_MD5$`, `_CLIPBOARD$` and
  `_FILES$`. New requirements: "_CAST" and "The system's name and the clipboard".
- `language/builtin-statements`: "Supported built-in statements" adds `CLEAR` and `_CLIPBOARD$ =`. New
  requirement: "CLEAR".
- `language/file-io`: new requirement "Listing a folder with _FILES$".
- `language/data-processing`: new requirement "_MD5$".
- `language/procedures`: "User-type parameters" adds that the `TYPE` must be defined above the procedure's header.

## Impact

- `sema`: `builtins.rs` (the new rows and a `Rule::Cast`), `check\builtins.rs` (`_CAST`'s two arguments),
  a check of `CLEAR`'s place and form, and `check\proc.rs` `declare_params` (the parameter's type against the types
  above the header, collected in `check\mod.rs` `collect_block_settings`).
- `ir` and `codegen-cpp`: the cast expression; `CLEAR` as a plain call (no change to `clear.txt` expected; the
  `v49` programs check what it resets).
- Tests: `tests\known_unsupported_rejections.list` loses `upstream/arrays/45_err_solved_bug_Incorrect_nr_of_args`.
- No new dependency, no libqb change.
