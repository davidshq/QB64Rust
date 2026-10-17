' TEST: check-fail
$CONSOLE:ONLY
' The pipeline delta of m2-numeric-types ("No follow-on errors after an unsupported declaration"), whose scenarios
' use `CONST c = EXP(1)` (a constant the evaluator does not compute yet; `CONST c$ = CHR$(65)` until
' m3-errorline-chr-const computed it, `DIM m AS _MEM` until m3-mem compiled it): a real error before the
' declaration is reported; after it only "not supported yet" errors are (`PRINT c` gets none, the string stored in
' a number variable none); a constant of QB64pe's auto-included file whose value is not computed yet (`_E` needs
' `EXP`; `_TRUE` and the others that are computed: `auto_const_typed`) is "not supported yet".
n = "before"
CONST c = EXP(1)
PRINT c
n = "after"
PRINT _E
SYSTEM
