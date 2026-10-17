# Proposal

## Why

The next step of M3 is the user's choice (`STATUS.md`; user, 2026-10-17: "do _OPENCLIENT, _EMBEDDED$, and
$EMBED"): the first three lines of the upstream demand report, three programs each. The upstream programs that
use `_OPENCLIENT` also use `_OPENHOST`, `_STATUSCODE`, `_CONNECTIONADDRESS$` and `GET`, `EOF` and `LOF` on a
network handle, and libqb's network part calls two functions written in BASIC in QB64pe's auto-included file
`afterlastline.bm`. So the change takes the whole network family, and that file. `$EMBED` and `_EMBEDDED$` are
one feature, so they come in one change and one close.

| Upstream program | Held back by | After this change |
|---|---|---|
| `precomp-flags/socket`, `precomp-flags/all` | `_OPENCLIENT` | can pass |
| `http/invalid_url` | `_OPENCLIENT` | can pass |
| `http/statuscode_invalid_handle` | `_OPENHOST`, `_STATUSCODE` | can pass |
| `sockets/test`, `console_only/network` (compiled only) | `_OPENHOST` | can pass |
| `http/read_example` | `_OPENCLIENT`, `_CONNECTIONADDRESS$`, `_STATUSCODE` | compiles; stays a known failure (the site it downloads changed) |
| `embed_relative/test`, `midi/midisoundbank_embed_test` | `$EMBED`, `_EMBEDDED$` | can pass |
| `embed/test` | `$EMBED`, `_EMBEDDED$` | held back by its file name, relative to the compiler's folder (as `icon/test`) |
| `auto_include/general` | `_E`, `_ENCODEURL$` | held back by `_E` alone |

Measured before this proposal (`verification\v44_*`, 71 programs, compiled and run with `qb64pe.exe`; the files
they embed in `verification\v44_files\`, an included file in `v44_inc\`; the facts go into `study\00` §5 in task
1.1):

- **`$EMBED:'file','handle'`** (`v44_a`, `x01`–`x39`): the file and the handle are the text between the first two
  and the next two `'`, without blanks at their ends; anything after is dropped (`x35`, a comment; `v44_a`,
  words). The line is any whose upper-case text starts with `$EMBED` (`$embed`, `$EMBEDX`, `$EMBED : '…'` are
  the same, `x22`, `x23`, `x28`); one in a comment is a comment (`x24`). A file name that starts with `./` or `.\`
  lies in the main source file's folder, also from an included file (`x16`); any other relative name is looked
  for in the compiler's own folder (`x26`). The handle is a letter, then letters and digits; case counts (`x13`,
  `x30`). Errors: no quotes, an empty file or handle (`x01`–`x04`), the handle's first character or another one
  (`x05`, `x06`, `x29`), a missing file or a folder (`x07`, `x27`), a file or a handle used twice, naming the
  first line (`x08`, `x09`, `x31`; files are compared by their names as written after the folder is put in front,
  so `./a/../a/f` is another file, `v44_a`).
- **`_EMBEDDED$("handle")`** returns the file's bytes, every byte value, an empty file and 300,000 bytes alike
  (`v44_a`), a new string at each call. Its argument must be one string literal (`x11`, `x18`, `x33`), not empty
  (`x12`), a valid handle (`x25`), and named by a `$EMBED` line **above it in file order** (`x10`, `x14`, `x15`,
  `x34`, `x36`); a `$EMBED` in a `$IF` branch not taken names nothing (`x17`), and a call in one checks nothing
  (`x37`). Without the `$` it is an array (`x19`); no argument or two is "Incorrect number of arguments" (`x20`,
  `x21`); in a `CONST` an error (`x32`). The old compiler writes `func__embedded(qbs_new_txt_len("h",1))` and an
  `embedded.cpp` holding each used file as an array, compressed when that saves a fifth.
- **The network functions** (`v44_b`, `x40`–`x46`, `x53`, `x59`–`x68`): `_OPENHOST(s$)`, `_OPENCLIENT(s$)`,
  `_OPENCONNECTION(h)`, `_CONNECTED(h)`, `_STATUSCODE(h)` (LONG) and `_CONNECTIONADDRESS[$](h)` (string, with or
  without the `$`) compile as plain calls of their rows (`func__openclient(qbs_new_txt_len(…))`). Where nothing
  listens: error 5 for a name that is not a network address, 0 for one that cannot connect, 52 for a handle that
  is not open (258 for `_OPENCONNECTION`); `EOF`, `LOF`, `GET` and `PUT` of such a handle raise 52 and `CLOSE` of
  it nothing. `_STATUSCODE` of a negative handle that was never opened ends the program with an access violation
  (`x59`): libqb's, the same here.
- **`_SOCKETS_`** (`v44_c`, `x51`–`x53`): 1 for the whole program when `_OPENHOST` or `_OPENCLIENT` stands in its
  compiled code, anywhere (a SUB never called, below the `$IF`); not for one in a `$IF` branch not taken, nor for
  the other four functions. Such a program links libqb's sockets and HTTP part.
- **`afterlastline.bm`** (`v44_c`, `v44_d`, `x47`–`x50`, `x54`–`x58`): its `_ENCODEURL$` and `_DECODEURL$` are
  functions of the language in every program; with `_SOCKETS_` it also holds `_WhatIsMyIP$`, which libqb calls
  for a host's address (and `_DECODEURL$` for an HTTP handle's). The file starts with `$CHECKING:OFF`: its
  functions have no event checks. A program cannot call `_WhatIsMyIP$` (`x47`, `x48`) nor name a procedure like
  them (`x49`, `x50`).

## What Changes

- **`$EMBED` and `_EMBEDDED$`** with the old compiler's errors; a used file is built into the executable, and a
  program that uses one links the embed object. A relative file name without `./` is "not supported yet", as for
  `$EXEICON` (design D4 of `m3-sound`).
- **The network functions** compile as plain built-ins; `_OPENHOST` and `_OPENCLIENT` set `_SOCKETS_` and link
  libqb's sockets and HTTP part.
- **`afterlastline.bm`** is compiled into a program that names one of its functions or has `_SOCKETS_`: a copy of
  the file at the reference clone's commit, its `_IKW_` names registered as the old compiler registers them, its
  `$CHECKING:OFF` writing no event checks for its statements. Since `$DEBUG` sets `_SOCKETS_`, a program with
  `$DEBUG` now links the sockets part too, as the old compiler's does (`DIVERGENCES.md` D-017 narrowed).
- **Tests**: slice programs for `$EMBED`, the network errors, the URL functions and a TCP/IP exchange over the
  loopback address (it opens a host: Windows may ask once whether the program may listen; loopback works either
  way); front-end tests of the errors, the types and the C++; call-site programs; eight upstream programs added
  to `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

- `language/network`: the network functions, `_SOCKETS_` from their use, the network part they link, and the URL
  functions of `afterlastline.bm`.
- `language/embedded-files`: `$EMBED` and `_EMBEDDED$`.

### Modified Capabilities

- `language/asserts-and-debug`: `$DEBUG`'s requirement says the sockets part is linked. (`compiler/pipeline`'s
  "Precompiler flags" stays true as written: `_SOCKETS_` is one of the flags it settles.)

## Impact

- Changed: `crates\syntax` (`pp.rs`: a feature for the network functions, met at their names; the auto-included
  file parsed after the main file; `$EMBED` and `$CHECKING` in `meta.rs`), `crates\sema` (`$EMBED` and
  `_EMBEDDED$`'s checks; six rows in `builtins.rs`; `RuntimePart::Sockets`, `Embed`; the `_IKW_` names),
  `crates\ir` (the embedded files in the settings; procedures without event checks), `crates\codegen-cpp` (no
  event check in such a procedure), `crates\driver` (the embedded files' check and `embedded.cpp`; the make
  flags), `crates\builtins` (the copy of `afterlastline.bm`, made by `tools\builtins\extract_builtins.py`).
- No change to libqb, the Makefile or the reference clone. `DIVERGENCES.md`: D-017 narrowed (the sockets part is
  linked).
- Not in this change: `$CHECKING` in a program (`source_ordering/checking_off`, `mem/mem_checking_off`); a relative
  `$EMBED` name without `./` (`embed/test`); compressing an embedded file (stored as it is, design D4); the HTTP
  download of `http/read_example`, which no longer matches its recorded page.
- `STATUS.md`, `ROADMAP.md`, `study\00` §5, `verification\README.md` (the `v44` family), `DECISIONS.md`,
  `SOMEDAY.md` updated.
