# Tasks

Standing rules: CLAUDE.md. Differently here: the measurements of task 1.1 were taken before the proposal; task 1.2
runs programs that listen on a port.

## 1. Measurements and records

- [x] 1.1 Record what was measured (`verification\v44_*`, 71 programs, the files they embed in `v44_files\`, an
  included file in `v44_inc\`; `.gitattributes` keeps `v44_files` byte for byte): the facts of the proposal into
  `study\00` §5 (a `v44` paragraph), the family into `verification\README.md`, a row in `DECISIONS.md` for the
  step. Verify: `python tools\repo_check\check_repo.py --untracked` is clean.
- [x] 1.2 Tell the user that the next run listens on a port (Windows may ask once whether it may), then write and
  record `v44_e_loopback` (a host on a fixed port, a client to `localhost`, `_OPENCONNECTION` until it gives the
  connection, a string each way with `PUT` and `GET`, `_CONNECTED`, the client's and the connection's addresses,
  `CLOSE`; never a host's address, which asks the internet). Verify: it prints the same twice when recorded
  twice; its facts added to the `v44` paragraph.

## 2. The network functions and _SOCKETS_

- [x] 2.1 The six rows of `builtins.rs`, `RuntimePart::Sockets` (`of_dependency`, the IR, `make_flags` in the old
  order, design D5). Verify: front-end tests `network_errors` (`check-fail`, `x40`–`x46`, `x56`, `x57`) and
  `network_typed`; a `make_flags` unit test; the call-site programs of D9 equal (`tools\callsite\callsite.py
  --glob "*open*"` and the others by name).
  *Done:* the table gets `mayhave` (`_CONNECTIONADDRESS` is found with and without `$`); twelve call-site programs
  (`openhost_*`, `openclient_*`, `openconnection_*`, `connected_*`, `statuscode_*`, `connectionaddress_*`), all
  equal. `x46` (`_OPENCLIENT&("x")`, "Invalid array name") stays "not supported yet", as every `_name(...)` that
  names no array does (`v33_x25`–`x29` of `m3-asserts-debug-zlib` likewise), so it is not in `network_errors`;
  `x56` and `x57` (the URL functions) go into it with group 3.
- [x] 2.2 `Feature::Sockets`, met at the two names in compiled code; `_SOCKETS_` from it or `$DEBUG` (design D6).
  Verify: `pp.rs` unit tests (a SUB never called, a branch not taken, the other four functions); the
  `language/network` scenarios of the flag as front-end tests.
  *Done:* `pp.rs` `sockets_from_the_network_functions`; front-end tests `network_sockets_on_ir` and
  `network_sockets_off_ir`; `MAX_ROUNDS` 8 for six features.
- [x] 2.3 Slice programs `s139_net_errors` and `s141_tcp_loopback`, recorded with `qb64pe.exe` (`--record`) and
  run with this compiler; upstream `precomp-flags/socket`, `precomp-flags/all`, `http/invalid_url`,
  `http/statuscode_invalid_handle`, `sockets/test` and `console_only/network` added to
  `tests\upstream\pass.list` once the file of group 3 links (until then a program with a network function fails
  to link, which this task's runs show). Verify: each passes (`--glob`).
  *Done:* before group 3 both failed to link (`FUNC__WHATISMYIP()`, `FUNC__DECODEURL(qbs*)` undefined); after it
  both pass, and the six upstream programs pass and are listed; `http/read_example` compiles and stays a known
  failure, listed in `tests\known_clean_not_passing.list`.

## 3. afterlastline.bm

- [x] 3.1 The copy of the file by `tools\builtins\extract_builtins.py`, exposed by `crates\builtins` (design D7);
  its source named in `tools\builtins\README.md`. Verify: the extractor run against the clone leaves the copy
  unchanged; a `builtins` unit test that the copy starts with QB64pe's header line.
- [x] 3.2 The tree parsed after the main file when needed; `$CHECKING` in it; `_IKW_` names, underscore names
  allowed there and its helper hidden from the program (D7); `ir::Proc::checked` and the unchecked body in
  `procs.rs` (D8). Verify: front-end tests `url_ir` and `url_cpp` (no `evnt` in the three procedures), `x47`–`x50`
  as `check-fail` forms; a program that names none of its functions has unchanged `ir` and `cpp` snapshots
  (tier 1 passes without updating any).
  *Done:* `x47`–`x50` in `url_errors`, `x56`, `x57` in `network_errors`. The parser also notes the two names at
  the start of a statement (`x57`, `_ENCODEURL$ "a b"`). The file's procedures are declared before the program's,
  so a program's `FUNCTION _EncodeURL$` is the one in error. Snapshots that changed: `network_typed` and
  `network_sockets_on_ir` (they use `_OPENHOST`/`_OPENCLIENT`, so the file is now compiled in), and
  `auto_const_errors`, whose `s = _WhatIsMyIP` line, now an error and so dropped by the follow-on rule, was taken
  out (`url_errors` holds that form). No other snapshot changed.
- [x] 3.3 `DIVERGENCES.md` D-017 narrowed (the sockets part is linked); the `$DEBUG` CLI test that the build links
  it. Verify: the CLI test passes (`--ignored`); `precomp-flags/debug` and `auto_include/debug` still pass.
  *Done:* CLI test `debug_links_the_sockets_part` (the licence file names libcurl); both upstream programs and
  slice program `s108_asserts_off_debug` pass.
- [x] 3.4 Slice program `s140_url`, recorded and run; the network programs of 2.3 run again. Verify: each passes
  (`--glob`).
  *Done:* `s140_url` also takes `_ENCODEURL$` in a SUB and a FUNCTION, without `$`, and a name with `@` and `:` in
  its query; `s139`, `s140` and `s141` pass and are in `slice.list`.

## 4. $EMBED and _EMBEDDED$

- [x] 4.1 `check\embed.rs`: the line, the handle, the class of the name, the duplicates and the order (design
  D1); `Rule::Embedded` (D2); the settings' embedded files. Verify: front-end test `embed_errors`
  (`check-fail`, one form of `x01`–`x39` each that `sema` decides) and `embed_typed`.
  *Done:* every `v44_x01`–`x39` program checked with the new front end: each accepted or rejected as the old
  compiler; `x19` (`_EMBEDDED` without `$`) and `x26` (a name relative to the compiler's folder) "not supported
  yet". `_EMBEDDED$` with its argument in parentheses is "not supported yet" (not measured).
- [x] 4.2 The driver's file check (D3; `DIVERGENCES.md` D-015 widened), `embedded.cpp` and `DEP_EMBED=y` in the
  build (D3, D4). Verify: CLI test of a missing file and of a folder; a `make_flags` unit test; a `build.rs` unit
  test that `embedded.cpp` holds an empty file, every byte value and a file of 33 bytes, and that an unused
  file is not written.
  *Done:* CLI tests `embedded_file_is_missing_or_a_folder` and `only_used_embedded_files_are_built_in` (the
  unused file is left out of a kept build's `embedded.cpp`; `#[ignore]`d); unit tests
  `make_flags_of_the_embed_part` and `embedded_files_in_the_build`. The demand report makes the file check too.
- [x] 4.3 Slice program `s138_embed` (its files in `s138_embed.files\`), recorded and run; call-site program
  `func_embedded`; upstream `embed_relative/test` and `midi/midisoundbank_embed_test` added to the pass list.
  Verify: each passes (`--glob`); the call-site program equal.
  *Done:* `s138_embed` also takes the accepted forms of `x16`, `x22`, `x23`, `x28`, `x30`, `x35`, `x37`–`x39`
  and an unused file; it passes and is in `slice.list`.

## 5. Close

- [x] 5.1 The closing runs (`python tools\close\close.py --plan`, then `--check`), the shrink-only lists, the
  demand reports and `tools\diagnostics\old_wording.py` regenerated, the counts by
  `tools\legacy_tests\counts.py`. Verify: `close.py --check` passes; every run passes or fails only by known
  failures.
  *Done:* every run passes (numbers in `STATUS.md`); `close.py --check` passes with the steps before the last
  snapshot edits accepted as not repeated (only snapshot files changed). The regenerated wording table added
  notes to five error snapshots.
- [x] 5.2 Documents: `STATUS.md`, `ROADMAP.md`, `SOMEDAY.md` (the rows of `afterlastline.bm` and the network
  functions out of "Left marked"; `embed/test` beside `icon/test`; `x59` among "QB64pe behaviours to review").
  Verify: `openspec validate m3-network-embed --strict` passes.
- [x] 5.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## 6. Review fixes

- [x] 6.1 `$EMBED`'s duplicate file: `./f` and `.\f` are one file, as the old compiler compares them (`sema`
  `check\embed.rs` `same_file`; measured `verification\v44_x69`, `x70` accepted). Absolute names stay compared as
  written (`SOMEDAY.md`). Verify: front-end test `embed_errors` (its last three lines).
- [x] 6.2 A wrong kind of argument to a FUNCTION has its own message, "… for this parameter of a FUNCTION": the
  generic one paired with both "… required for sub" and "… for function" (`x56`), so the wording table had dropped
  the note of every SUB call, and `x = f&("a")` had the note "Number required for sub" where the old compiler says
  "Number required for function" (measured `v44_x71`). Verify: `old_wording.py` and `--check`; snapshots
  `proc_errors`, `network_errors`; tier 1.
  *Done:* tier 1 passes; the slice and upstream lists run again with the pass cache; the other closing runs were not
  repeated: the two edits change only a duplicate check and the text of two errors, which no accepted program
  reaches, so the call-site check, the CLI tests and the demand reports see nothing new.

## Workflow follow-up

- Archive the change when the user has looked at it.
