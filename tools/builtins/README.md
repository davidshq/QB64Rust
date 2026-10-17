# The built-in table

`extract_builtins.py` reads the reference clone (`..\QB64pe`, at the commit `tests\upstream\SOURCE.md` names) and
writes what the compiler knows of QB64pe's built-ins:

- `builtins.json`, beside it: every SUB and FUNCTION the old compiler registers (`source\subs_functions\
  subs_functions.bas`, `SUB reginternal`), the names `qb64pe.bas` special-cases, and the names QB64pe's
  auto-included files declare, with each constant's suffix and expression. `crates\builtins\build.rs` turns it into
  the table.
- `crates\builtins\src\afterlastline.bm`: a byte-for-byte copy of QB64pe's `internal\support\include\
  afterlastline.bm` (QB64pe's MIT licence), the file the old compiler includes after a program's last line. The
  compiler compiles it from there (`_ENCODEURL$`, `_DECODEURL$` and the helper `_WhatIsMyIP$` that libqb's network
  part calls; design D7 of `m3-network-embed`), so the front end needs no clone. The copy is written only when
  it differs.

```
python tools\builtins\extract_builtins.py
```

Neither output is edited by hand. Run again after the clone moves to another commit; a run on the same commit
leaves both unchanged.
