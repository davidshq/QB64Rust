# Design

## Context

Motivation, scope and what was measured: `proposal.md` (`verification\v40_*`, `verification\v40_rc\`, 2026-10-16).
Where this compiler stands:

- **`sema`** `check\mod.rs` `meta`: a metacommand line is one token; `meta_line` (`syntax\pp.rs`) gives it in upper
  case without the spaces at its ends (`LTRIM$`/`RTRIM$` take spaces only). `$VERSIONINFO` falls to the last branch,
  "metacommand `$VERSIONINFO` not supported yet". `$EXEICON` (`exe_icon_meta`) sets `Settings::exe_icon`
  (`sema\lib.rs`), which `ir\lower.rs` carries to `ir::Settings::exe_icon: bool`.
- **`driver`** `build.rs`: `write_icon` writes `<build>/temp/icon.ico` and `icon.rc` (one line, LF);
  `make_flags` adds `DEP_ICON_RC=y` when `exe_icon`; the cache key (`build_key`) hashes the files of `ICON_FILES`
  under that flag. `main.rs` calls `write_icon` after `write_fragments`, before `build`.
- **The old compiler** writes `icon.rc` with CRLF lines: the icon line `0 ICON "icon.ico"` first when `$EXEICON`
  stands, then the version part; `manifest.h` and `<file>.exe.manifest` beside it (`qb64pe.bas` 13561–13640), the
  file name being the executable's. Its `icon.rc` for `$EXEICON` alone is the one line with CRLF; this compiler's
  is LF today, which `windres` reads the same (the resource is equal; kept as it is, D4).

## Goals / Non-Goals

**Goals:** every accepted form of `$VERSIONINFO` gives the old compiler's resource files byte for byte; its three
errors and its warning; a clear compile error where its build fails or its executable cannot start.

**Non-Goals:** other systems (the old compiler writes the files on Windows only and ignores them elsewhere); a
`$EXEICON` name that neither starts with `./` nor is absolute (`winresource/test` stays held back by it).

## Decisions

### D1. The line, in `sema`

`check\mod.rs` `meta` sends a line whose `meta_line` starts with `$VERSIONINFO` to a new `version_info_meta`,
which follows `qb64pe.bas` 3647–3741 on the raw token text (the old compiler's `a3$`, spaces trimmed) and its upper
case (`a3u$`):

1. `colon` = the first `:`; `eq` = the first `=` after it. No `:`, no `=`, or `=` right after `:`: error
   "expected `$VERSIONINFO:key=value`" (`x01`–`x04`, `x14`, `x21`, `x22`).
2. The key: the upper case between them, spaces trimmed. The value: the raw text after `=`, spaces trimmed, each
   `"` made `'`.
3. `FILEVERSION#`, `PRODUCTVERSION#`: the old compiler's check (empty, a byte outside `0-9,`, a last byte that is a
   comma, or not three commas when the last byte is reached): error "expected `$VERSIONINFO:<KEY>=#,#,#,#` (four
   comma-separated numbers: major, minor, revision and build)" (`x06`–`x13`, `x17`–`x19`). Then the parts the
   resource compiler refuses (D3). The value is kept as written; the string version of the same name takes it
   when that string is still empty.
4. The eleven string keys: the value without one pair of enclosing `'` (`RemoveStringEnclosingPair`: the value is
   longer than one byte and its first and last bytes are `'`); without that pair, the warning "Missing string
   bracket delimiters ('')" with the value shown below it as the old compiler does (`v40_d`), given with `-w` as
   every warning is. The manifest check (D3) of `CompanyName`, `ProductName`, `FileDescription` comes after the
   walk, on the values kept.
5. Any other key: error "invalid `$VERSIONINFO` key; use FILEVERSION#, PRODUCTVERSION#, CompanyName, …" (`x05`,
   `x15`, `x16`).

The fields go into `Settings::version_info: Option<VersionInfo>` (`sema\lib.rs`): the thirteen values as bytes
(`Vec<u8>`, the source's bytes as written: the old compiler copies them unchanged, `v40_g7`). The line that last
set each string field, for the error of D3, stays in the checker (built: it is needed only there, so the IR's
settings carry no spans). A line in a `$IF` branch not taken never reaches `meta` (the parser drops it),
as for `$EXEICON`. The old wording of each error comes as a note from the recorded rejections
(`tools\diagnostics\old_wording.py`), regenerated at the close.

*Alternative considered*: parsing the line in the lexer or the parser. Rejected: every other metacommand of this
kind (`$EXEICON`, `$CONSOLE`, `$UNSTABLE`) is read in `meta`.

### D2. The settings, through the IR to the driver

`ir::Settings` gets `version_info: Option<VersionInfo>` (the same type, re-exported from `sema`, as the IR has
no string fields of its own in its settings today; `ir\lower.rs` copies it). The dump writes a `Version info:`
line with the two numbers after the settings line, and one line per string field that is set (`ir\dump.rs`;
built so, as the values do not fit the one settings line). `build::make_flags` adds `DEP_ICON_RC=y` when `exe_icon || version_info.is_some()`
(`qb64pe.bas` 13719), at the place it has now.

### D3. What the old compiler's build or executable would refuse

Each a compile error at the line, not "not supported yet"; the numeric parts checked in `version_info_meta`, the
manifest after the walk (`check_manifest_fields`):

- **Numeric parts** (`llvm-rc`, `v40_h1`–`h7`): a part that is empty, a part with a leading `0` and a digit 8 or 9
  (read as octal), a part whose value (octal after a leading `0`, else decimal) is over 65535: "part <n> of
  `<KEY>`, `<p>`, is not a number from 0 to 65535 the resource compiler reads", with "(after a leading 0 it is
  octal)" for such a part, and "part <n> of `<KEY>` is empty; …" for an empty one (built so: the part's number
  names an empty part). `010` is 8 and compiles, as there.
- **The manifest** (user, 2026-10-16): in `CompanyName`, `ProductName` and `FileDescription`, a `<` or `&`, or bytes
  that are not UTF-8: "`<KEY>` holds `<c>`, which the executable's manifest cannot hold; Windows would not start
  the program". Checked after all lines are read, at the line that set the value kept: a key set twice is checked
  at its last line (a first bad value replaced later compiles, as the manifest then holds the later one). Not measured and not checked: a control byte (the lexer ends a metacommand
  at a line end; a tab is valid XML).

Neither is a row of `DIVERGENCES.md`: its paragraph on rejections that are not divergences ("a rejection with a
clearer message of a program whose build fails") gets "or whose executable cannot start".

### D4. The files, in the driver

`build.rs` gets `write_resources(build, icon: Option<&Path>, version: Option<&VersionInfo>, exe_name: &str)`, in
place of `write_icon`, which writes into `<build>/temp`:

- `icon.ico` (a copy of the bytes) when there is an icon;
- `icon.rc`: with no version info, the one line `0 ICON "icon.ico"` as today (LF); with version info, the old
  compiler's text with CRLF lines: the icon line when there is an icon, then `""`, `#include "manifest.h"`, `""`,
  `CREATEPROCESS_MANIFEST_RESOURCE_ID RT_MANIFEST "<exe_name>.manifest"`, `""`, `1 VERSIONINFO`, the
  `FILEVERSION     …`/`PRODUCTVERSION  …` lines when set, and the block of the eleven `VALUE` lines and the
  translation, each value followed by `\0` inside its quotes, bytes as written;
- `manifest.h` and `<exe_name>.manifest`, the old compiler's text, `<exe_name>` being the executable's file name
  (`game.exe`).

The text is built by a function `version_files(…) -> Vec<(String, Vec<u8>)>` that the tier-1 test calls without
building (D5). `ICON_FILES` gives way to `is_resource_file` (`icon.ico`, `icon.rc`, `manifest.h`, `*.manifest`):
`write_resources` first removes such files an earlier build left in a kept folder, and the cache key (`build_key`)
hashes each that is there under `DEP_ICON_RC=y`, so a changed value changes the key (the manifest's name is in `icon.rc`, so the executable's name
is too, which is right: the name is part of the resource script). `main.rs` calls `write_resources` where it calls
`write_icon` now.

*Alternative considered*: a fixed manifest name. Rejected: the test then compares all files but one line, for
nothing gained.

### D5. Tests

- **Driver test, tier 1** (`crates\driver\tests\`): for each accepted `verification\v40_*.bas` with a recorded
  `v40_rc\<name>.icon.rc`, the front end then `version_files` with the exe name `<name>.exe` (and the icon of
  `v40_f`); each file equal to the recorded bytes. The programs whose build fails or whose executable cannot start
  in the old compiler (`h1`, `h2`, `h4`, `h5`, `h7`, `g2`, `g3`, `g4`, `g6`) are checked for D3's error instead.
- **Front-end tests**: `versioninfo_errors.bas` (`check-fail`: the rejected forms of `v40_x*` and D3's), and a
  `check-ok` program with every accepted form; the warning through the CLI test of `-w`.
- **Slice program** `s130_versioninfo`: every key, `FILEVERSION#` set (without it .NET's `FileVersionInfo` reads
  every string as empty, `v40_f`); it writes a PowerShell script beside itself, runs it with `SHELL` on
  `COMMAND$(0)` and prints the fields, then deletes the script; recorded with `qb64pe.exe`.
- **Cache**: a unit test in `build.rs` that two version infos give two keys.
- Upstream `versioninfo`, `versioninfo2`, `versioninfo3` on `pass.list`.

## Risks / Trade-offs

- **[The text of the resource differs in a byte]** → the tier-1 test compares every recorded program that builds (17, `x20` among them) byte for byte.
- **[PowerShell in a slice program]** → it is on every Windows machine and on the CI runner; the program prints
  only fields, never a path. If it is slow there, the program still ends well inside the 60 s limit (about 1 s).
- **[A source byte that is not ASCII]** → copied as written, as the old compiler does (`v40_g7`); the manifest check
  of D3 refuses what is not UTF-8 in the three manifest fields only.
