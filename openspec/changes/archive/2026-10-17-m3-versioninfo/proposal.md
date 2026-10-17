# Proposal

## Why

The next step of M3 is the user's choice (`STATUS.md`): **`$VERSIONINFO` and `_ARRAYCOPY`** (user, 2026-10-16:
"lets do arraycopy and versioninfo"). They are two unrelated changes; this one is `$VERSIONINFO`, the smaller,
and `m3-arraycopy` follows it. `$VERSIONINFO` holds back four upstream programs (`qb64rust --report unsupported`,
its line among the first of the demand report):

| Upstream program | What it has | After this change |
|---|---|---|
| `versioninfo/test` | `FileDescription`, `ProductName` in double quotes | can pass |
| `versioninfo2/test` | `ProductVersion#` only | can pass |
| `versioninfo3/test` | `ProductVersion#`, then `ProductVersion` without quotes (a warning with `-w`) | can pass |
| `winresource/test` | the same with `$EXEICON:'tests/compile_tests/winresource/qb64pe.ico'` | stays held back by that `$EXEICON` name, which neither starts with `./` nor is absolute (`icon_relative/test` is held back the same way) |

Measured before this proposal (`verification\v40_*`, 47 programs, compiled and run with `qb64pe.exe`; the files
`qb64pe -z -w -q` writes for the resource compiler recorded by `verification\v40_rc.py` into `verification\v40_rc\`;
the clone's `git status --porcelain` empty before and after; the facts go into `study\00` §5 in task 1.1):

- **The metacommand** (`qb64pe.bas` 3647–3741): any line whose upper case starts with `$VERSIONINFO` (also
  `$VERSIONINFOX:…`, `v40_j`); the key stands between the line's first `:` and the first `=` after it, blanks
  trimmed, in any letter case; the value is the rest of the line, trimmed, with every `"` made `'`, then one pair
  of enclosing `'` removed. A trailing comment is part of the value (`v40_d`). Thirteen keys: `FILEVERSION#`,
  `PRODUCTVERSION#` (four comma-separated parts of digits only), and eleven strings (`CompanyName`,
  `FileDescription`, `FileVersion`, `InternalName`, `LegalCopyright`, `LegalTrademarks`, `OriginalFilename`,
  `ProductName`, `ProductVersion`, `Comments`, `Web`). The last line of a key counts (`v40_e`); a numeric key gives
  its string version its digits unless that string was set before it (`v40_b`, `v40_c`). A line in a `$IF` branch not
  taken does not count; one in a SUB or an included file does (`v40_i`). Three messages for the 21 rejected forms
  (`v40_x01`–`x22`; `x20`, `FILEVERSION=1,2,3,4,5`, is a string key and accepted). With `-w`, a string value
  without enclosing `'` is the warning "Missing string bracket delimiters ('')" (`v40_d`).
- **The build** (`qb64pe.bas` 13561–13640, 13719): `icon.rc` gets the version block (CRLF lines) after the icon line
  of `$EXEICON` when there is one, `manifest.h` and `<exe>.manifest` are written beside it, and the make flag is
  `DEP_ICON_RC=y`, the flag `$EXEICON` already passes. The manifest asks for the common controls version 6 and
  holds `CompanyName.ProductName.ProductName` and `FileDescription` unescaped.
- **What the old compiler accepts and its build or its executable does not**: a numeric part that is empty
  (`1,,2,3`, `,1,2,3`), over 65535, or with a leading `0` and an 8 or 9 (`08`, read as octal) fails in its
  resource compiler (`llvm-rc`; `v40_h1`, `h2`, `h4`, `h5`, `h7`); a `<` or `&` in the company name, product name or
  description, or a byte there that is not UTF-8, gives an executable Windows will not start ("side-by-side
  configuration is incorrect", `v40_g2`, `g3`, `g4`, `g6`). `>`, UTF-8, a backslash and an `&` in another value work
  (`v40_g1`, `g5`, `g7`, `g8`).

## What Changes

- **`$VERSIONINFO:key=value`** compiles: every key and form the old compiler accepts, with its three errors and its
  warning, and the version resource and manifest it writes, byte for byte, linked with `DEP_ICON_RC=y`, beside the
  icon of `$EXEICON` when both stand in a program.
- **A compile error, not the old compiler's failed build**, for the numeric parts its resource compiler refuses
  (the rule of 2026-10-16 (4): where the old compiler's build fails, a clear compile error), and **a compile error,
  not an executable that cannot start**, for a `<`, an `&` or a byte that is not UTF-8 in the three values of the
  manifest (user, 2026-10-16, asked before this proposal).
- **Tests**: a tier-1 test that the resource files written for every accepted `v40` program equal the recorded
  ones; a slice program that prints its own version resource, equal to the old compiler's; front-end tests of the
  errors and the warning; three upstream programs added to `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/console-and-window`: a requirement for `$VERSIONINFO` beside `$EXEICON`'s: the keys, the forms, the
  errors and the warning.
- `compiler/pipeline`: the build gives the executable the version resource and manifest the old compiler writes,
  and a build with another version resource does not share a cached executable.

## Impact

- Changed: `crates\sema` (`check\mod.rs`: the metacommand; `lib.rs`: the program's version settings), `crates\ir`
  (the settings carried to the driver), `crates\driver` (`build.rs`: the resource files, the make flag, the cache
  key); new `tests\frontend\versioninfo_*.bas`, a slice program `tests\corpus\slice\s130_*`, a CLI or driver test
  over `verification\v40_rc\`; `tests\upstream\pass.list`; the demand report and the shrink-only lists regenerated.
- No change to libqb, the Makefile or the reference clone.
- `DIVERGENCES.md`: none; its paragraph on rejections that are not divergences names an executable that cannot
  start beside a build that fails.
- Not in this change: `_ARRAYCOPY` (`m3-arraycopy`, next); `$EXEICON` names that neither start with `./` nor are
  absolute; `$VERSIONINFO` on other systems than Windows.
- `STATUS.md`, `ROADMAP.md`, `study\00` §5, `SOMEDAY.md` (the row of `$VERSIONINFO` goes; the questionable
  behaviours above), `verification\README.md` (the `v40` family) and `DECISIONS.md` updated.
