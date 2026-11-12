# CHAIN

`CHAIN name$` ends the program and runs another one in its place.

## Syntax

<!-- begin generated: syntax -->
```text
CHAIN moduleName$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

It compiles for any string expression, to QB64pe's call. A test does not run it, since it starts another program.
<!-- src: language/builtin-statements "CHAIN"; tests\corpus\runtime_comparison\238_chain -->

Variables listed in `COMMON` pass to the program chained to, as QB64pe passes them.
<!-- src: language/arrays-and-types "COMMON" -->

## Errors

- `CHAIN` with no name, a number or two arguments is a compile error.
  <!-- src: language/builtin-statements "CHAIN"; verification\v30_x95_chain_alone; verification\v30_x96_chain_number; verification\v30_x97_chain_two_arguments -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN CHAIN "other.exe" -->

## See also

[`RUN`](run.md), [`COMMON`](common.md), [`SHELL`](shell.md).
