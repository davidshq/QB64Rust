' TEST: check-fail
$CONSOLE:ONLY
' The pipeline delta of m2-numeric-types ("No follow-on errors after an unsupported declaration"), whose scenarios
' use `DIM m AS _MEM` now that `STRING * n` and the other new types are declared: a real error before the
' declaration is reported; after it only "not supported yet" errors are (`PRINT LEN(m)` gets none, the string
' stored in a number variable none); a constant of QB64pe's auto-included file whose value is not computed yet
' (`_CHR_CR` needs `CHR$`; `_TRUE` and the others that are computed: `auto_const_typed`) is "not supported yet".
n = "before"
DIM m AS _MEM
PRINT LEN(m)
n = "after"
PRINT _CHR_CR
SYSTEM
