' TEST: check-fail
$CONSOLE:ONLY
' The pipeline spec ("No follow-on errors after an unsupported declaration"), whose scenarios use `TYPE cos`, a
' `TYPE` named like a built-in (since m4-names-arrays, which compiles a `TYPE` block inside a block, the example
' until then; `CONST c = EXP(1)` until m3-constant-evaluation, `CONST c$ = CHR$(65)` until
' m3-errorline-chr-const, `DIM m AS _MEM` until m3-mem): a real error before the
' declaration is reported; after it only "not supported yet" errors are (the string stored in a number variable
' gets none). The constants of QB64pe's auto-included file are all computed since m3-constant-evaluation
' (`auto_const_typed`).
n = "before"
TYPE cos
    m AS LONG
END TYPE
n = "after"
SYSTEM
