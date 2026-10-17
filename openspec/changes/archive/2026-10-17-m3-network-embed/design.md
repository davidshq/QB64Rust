# Design

## Context

Motivation, scope and what was measured: `proposal.md` (`verification\v44_*`, 2026-10-17). Where this compiler
stands:

- **Metacommands** of a whole line are read in `sema` `check\mod.rs` (the `$EXEICON` and `$VERSIONINFO` branches
  near `exe_icon_meta`); a file a metacommand names is checked by the driver (`driver\src\lib.rs`
  `check_exe_icon`), since `sema` reads no file. `$EMBED` and `$CHECKING` are "not supported yet".
- **Built-ins** are rows of `sema\src\builtins.rs`; a row's dependency in the built-in table becomes a
  `RuntimePart` (`sema\src\lib.rs` `of_dependency`), the IR's settings carry the set, and the driver turns it
  into make flags in the old compiler's order (`driver\src\build.rs` `make_flags`). The six network functions and
  `_EMBEDDED$` have no row: "not supported yet" by name.
- **Precompiler flags** are settled in `syntax` (`pp.rs`, `program.rs` `parse_settled`): a round parses the
  program with the flags' values, the parser reports the features it meets (`PpState::met`, today from
  metacommand lines only), and `_SOCKETS_` is `$DEBUG`'s value (`PpFlags::defines`).
- **The auto-included files** are not parsed: the constants of `beforefirstline.bi` come from the built-in table
  (`tools\builtins\builtins.json`), and the names of `afterlastline.bm` are "not supported yet"
  (`sema` `check\decl.rs`, `is_auto_include_name`).
- **Event checks**: every statement is written as `do{ … if(!qbevent)break;evnt(…);}while(r);` and a user label
  with `if(qbevent){evnt(…);r=0;}` (`codegen-cpp` `procs.rs` `body`, `labels`).
- **The build** writes the fragments and resource files into `<build>\temp` and runs the clone's `Makefile` with
  `PATH_INTERNAL_TEMP` there (`driver\src\build.rs`); the Makefile links `temp\embedded.o`, made from
  `temp\embedded.cpp`, when given `DEP_EMBED=y`.

## Goals / Non-Goals

**Goals:** `$EMBED` and `_EMBEDDED$` with every measured check and the files' bytes in the executable; the six
network functions as the old compiler writes them; `_SOCKETS_` and the sockets part as the old compiler sets
them; `afterlastline.bm`'s functions compiled from the file itself, for the program and for libqb.

**Non-Goals:** `$CHECKING` in a program's own code; the old compiler's compression of embedded files; a `$EMBED`
name relative to the compiler's folder; testing HTTP against a real server.

## Decisions

### D1. $EMBED in sema

`sema` reads the line in a new `check\embed.rs`, called from the metacommand branch of `check\mod.rs` for a line
whose upper-case text starts with `$EMBED` (`qb64pe.bas` 3743–3818): the file and the handle between the first
two and the next two `'`, trimmed; an empty one, or a missing quote, is "expected `$EMBED:'filename','handle'`";
the handle's characters as measured (`x05`, `x06`, `x29`). The name's class is `$EXEICON`'s
(`ExeIcon::beside_source`): `./` or `.\` beside the main source file, absolute as written, anything else "not
supported yet" at the line (the old compiler's own folder has no counterpart; design D4 of `m3-sound`).

A second `$EMBED` of the same handle (case counts) or the same file is an error naming the first one's line, a
line of the main file (for one in an included file, its `$INCLUDE`'s line, as `$EXEICON`'s rule 2). The old
compiler compares the file names after it puts the folder in front of the rest of a `./` or `.\` name; comparing
whether each has that prefix and the rest as written is the same comparison (`./f` and `.\f` are one file, `x69`;
`./d/f` and `./d\f` two, `x70`), so `sema` can make it without the folder. An absolute name is compared as written,
where the old compiler first resolves its folder (`SOMEDAY.md`, "Left marked by `m3-network-embed`").

**Order.** The old compiler registers a `$EMBED` as its main pass meets the line, and `_EMBEDDED$` looks only at
what is registered (`x14`, `x15`, `x34`). `sema` collects every compiled `$EMBED` line before it checks code, each
with its place in file order (the tree and offset, an included tree taking the place of its `$INCLUDE`), and a
call accepts a handle only from a line before the call's place. A line in a `$IF` branch not taken is not in the
tree, so it registers nothing (`x17`).

The checked program's settings get the embedded files: handle, name, class, span, and whether a compiled call
uses it.

*Alternative considered*: checking calls in one pass that registers lines as it goes. Rejected: `sema` does not
visit procedures in file order.

### D2. _EMBEDDED$

A row of `builtins.rs` with a rule of its own, `Rule::Embedded`: the one argument must be a string literal by
itself (the old compiler's "single literal string in quotes"; a sum, a variable or `CHR$` is that error, `x11`,
`x18`, `x33`), not empty (`x12`), a valid handle (`x25`, the same check as the line's), registered above (D1).
The call marks the file used. It lowers to the plain call `func__embedded(qbs_new_txt_len("h",1))` the old
compiler writes; `qbx.cpp` declares it. `_EMBEDDED` without the `$` stays an array name (`x19`).

### D3. The embedded files in the build

The driver checks, as it checks the icon, that each embedded file is a file (`driver\src\lib.rs`, next to
`check_exe_icon`); a missing file or a folder is an error at its line that names the file as written. The old
compiler names the folder it looked in, so `DIVERGENCES.md` D-015 widens from `$EXEICON` to `$EMBED`.

For a program with a used file, the build writes `temp\embedded.cpp` (`build.rs`, next to `write_resources`):
each used file as a `static const unsigned char` array of its bytes and `func__embedded(qbs *handle)`, which
compares the handle with `qbs_equal` and returns a new `qbs` of the file's bytes, or an empty string, as the old
compiler's function ends. The settings get `RuntimePart::Embed`; `make_flags` adds `DEP_EMBED=y` after
`DEP_ZLIB=y`, the old compiler's place. The build's key reads the files of `temp`, so a changed file makes a new
build. An unused file is neither written nor linked.

### D4. No compression

The old compiler deflates a file and calls `func__inflate` when that saves a fifth, and then links the
data-processing part (`DEP_ZLIB=y`). Here every file is stored as it is. A program sees the same bytes; only the
executable's size differs, and the build of such a program links one part fewer.

*Alternative considered*: deflating as the old compiler does. Rejected: a compression crate in the driver, more
code to keep, for the size of an executable alone.

### D5. The network functions

Six rows of `builtins.rs`, each `Rule::Plain`: `_OPENHOST`, `_OPENCLIENT`, `_OPENCONNECTION`, `_CONNECTED`,
`_STATUSCODE` and `_CONNECTIONADDRESS` (the table gives it "may have `$`", so it is found with or without it).
`of_dependency` maps `DEPENDENCY_SOCKETS` to `RuntimePart::Sockets`, which `_OPENHOST` and `_OPENCLIENT` carry in
the table; `make_flags` adds `DEP_SOCKETS=y DEP_HTTP=y` after `DEP_CONSOLE_ONLY=y`, the old compiler's place.
`GET`, `PUT`, `EOF`, `LOF` and `CLOSE` take a network handle as the file number they already take; the call-site
programs and `v44_x63`–`x68` confirm their calls.

### D6. _SOCKETS_ from the network functions

`pp.rs` gets `Feature::Sockets`. The parser calls `met(Feature::Sockets, 1)` where it parses a name token
`_OPENHOST` or `_OPENCLIENT` (any letter case) in compiled code (`parser\expr.rs`, the name of an expression):
the old compiler sets the dependency when it compiles such a call, wherever the call stands (`v44_c`, `x52`), and
not in a branch not taken (`x51`). Strings, comments and `DATA` hold no name tokens. `_SOCKETS_` is 1 when
`Feature::Debug` or `Feature::Sockets` is (`PpFlags::defines`); like today it takes the round's value and changes
at no line. A program whose name token meets no compiled call (a reserved-name error) has errors anyway.

### D7. afterlastline.bm

`tools\builtins\extract_builtins.py` copies `internal\support\include\afterlastline.bm` byte for byte from the
reference clone into `crates\builtins\src\afterlastline.bm` (QB64pe's MIT licence; a line in `tools\builtins\
README.md` says where it is from); the crate exposes its bytes, so the front end needs no clone.

`program.rs` `parse_round` parses it after the main file, as a tree of a source file named
`internal\support\include\afterlastline.bm` (the name the old compiler's `#line` directives give), with the
preprocessor state the main file ended with, when the program needs it: the round's `_SOCKETS_` is 1 (libqb's
sockets part calls `FUNC__WHATISMYIP` and `FUNC__DECODEURL`), or the main file and its includes have a name
token `_ENCODEURL` or `_DECODEURL`. A program that needs neither is parsed as today, so its dumps and C++ do not
change. Under `$DEBUG` alone the file is parsed with `_SOCKETS_` 1, so `_WhatIsMyIP$` and its `_OPENCLIENT` are
compiled and the sockets part is linked, as with the old compiler (D-017 narrowed).

In that tree only (`syntax` marks it):
- `$CHECKING:OFF` and `$CHECKING:ON` are read, and the procedures between them are unchecked. Anywhere else
  `$CHECKING` stays "not supported yet".
- `sema` allows names that start with `_` (the reserved-name rule of `check\decl.rs`), and a procedure named
  `_IKW_x` is declared as `_x` (the old compiler registers it without `_IKW`, `afterlastline.bm` lines 41–53).
  The other procedures (`_WhatIsMyIP$`) are hidden from the program's own code: a name of the auto-included file
  stays an error there (`x47`, `x48`, now an error, not "not supported yet"). `is_auto_include_name` stays the
  test for a name the program may not declare (`x49`, `x50`).

The C++ names follow from the names: `FUNC__ENCODEURL`, `FUNC__DECODEURL`, `FUNC__WHATISMYIP`, with libqb's
signatures (`qbs *FUNC__DECODEURL(qbs*)`, `qbs *FUNC__WHATISMYIP()`); a build with the sockets part fails to link
if they do not match, which the slice programs show.

### D8. No event checks in an unchecked procedure

`ir::Proc` gets `checked: bool`, lowered from `sema`'s procedure. `codegen-cpp` `procs.rs` `body` writes an
unchecked procedure's statements without the `do{ … evnt(…);}while(r);` around them and its labels without their
event check, as the old compiler writes them under `$CHECKING:OFF` (read with `-z` from its build of `v44_c`:
`FUNC__DECODEURL` has no `evnt` and no statement label; the error checks inside statements, such as
`||is_error_pending()` in a condition, stay). `afterlastline.bm` has no array, `_MEM` or `ON ERROR`, so the
old compiler's other uses of `$CHECKING:OFF` do not arise in it. The IR dump shows `unchecked` on such a
procedure.

### D9. Tests

- Slice programs, recorded with `qb64pe.exe`: `s138_embed` (`v44_a`; its files in `s138_embed.files\`, which the
  runner puts beside it before it compiles, as for `s77_exeicon`), `s139_net_errors` (`v44_b` and the handles of
  `x60`–`x68` without `x59`, which crashes), `s140_url` (`v44_d` and `v44_c`), `s141_tcp_loopback` (a host on a
  port, a client to `localhost`, the connection, a string each way, `_CONNECTED` and the client's and the
  connection's addresses; measured as `v44_e` in task 1.2).
- Front end: `embed_errors` and `network_errors` (`check-fail`, one form of `v44_x*` each), `embed_typed`,
  `network_typed`, `url_ir` (the auto-included procedures, unchecked) and `url_cpp`.
- CLI tests: an embedded file that is missing; a build of a program with `$DEBUG` that links the sockets part
  (`#[ignore]`d, needs the clone). Unit tests of `make_flags` with both new parts.
- Call-site programs: one per network function form (`_CONNECTIONADDRESS` with and without `$`) and
  `func_embedded` (its file in its `.files` folder).
- Upstream: the eight programs of the proposal added to `tests\upstream\pass.list`.

## Risks / Trade-offs

- [A program that opens a host listens on a port, and Windows may ask once per executable whether it may]
  → `s141_tcp_loopback` and `http/statuscode_invalid_handle` connect over the loopback address, which works
  whether the prompt is answered or not; the user is told before the first run (task 1.2).
- [`http/invalid_url` needs a name lookup to fail] → it fails offline too; `http/read_example` stays a known
  failure (the page it downloads changed).
- [`$DEBUG` programs now link the sockets and HTTP part] → a larger executable and one more libqb variant to build
  once; the build cache keeps it. Their output does not change (`precomp-flags/debug`, `auto_include/debug`).
- [Large embedded files make large C++] → the array is written as hexadecimal bytes, about five characters a
  byte; no test file is above 300,000 bytes.
- [libqb's crash on `_STATUSCODE` of a never-opened negative handle (`x59`)] → the same call is made here, so
  the same crash; listed in `SOMEDAY.md`'s "QB64pe behaviours to review", not a divergence.
