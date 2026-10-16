' TEST: check-fail
$CONSOLE:ONLY
' The pipeline delta of m2-numeric-types ("No follow-on errors after an unsupported declaration"), whose scenarios
' use `CONST c$ = CHR$(65)` (a constant the evaluator does not compute yet) since m3-mem compiles `DIM m AS _MEM`: a
' real error before the declaration is reported; after it only "not supported yet" errors are (`PRINT LEN(c$)`
' gets none, the string stored in a number variable none); a constant of QB64pe's auto-included file whose value is
' not computed yet
' (`_CHR_CR` needs `CHR$`; `_TRUE` and the others that are computed: `auto_const_typed`) is "not supported yet".
n = "before"
CONST c$ = CHR$(65)
PRINT LEN(c$)
n = "after"
PRINT _CHR_CR
SYSTEM
