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

- `crates\builtins\src\color0.bi` and `color32.bi`: byte-for-byte copies of QB64pe's `internal\support\color\`
  files (MIT), the constants `$COLOR:0` and `$COLOR:32` include (design D8 of `m3-constant-evaluation`).
- `crates\builtins\src\palettes.rs`: libqb's default 256-colour and EGA palettes (`libqb.cpp` `file_qb64_pal`,
  `file_qb64ega_pal`; MIT) as Rust tables, for the colour functions of the `CONST` evaluator (design D5 of the same
  change).

```
python tools\builtins\extract_builtins.py
```

`mpmath_values.py` writes `crates\sema\src\mpmath_values.rs`: true values of the functions the `CONST` evaluator
certifies, from Python's `mpmath` at 60 digits, which a test of `certify.rs` compares with its high-precision
reference (design D3 of `m3-constant-evaluation`). `mpmath` (`pip install mpmath`) is needed only to write the file
again.

Neither output is edited by hand. Run again after the clone moves to another commit; a run on the same commit
leaves both unchanged.
