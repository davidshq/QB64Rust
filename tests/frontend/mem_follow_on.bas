' TEST: check-fail
$CONSOLE:ONLY
' The pipeline delta of m2-numeric-types ("No follow-on errors after an unsupported declaration"), whose scenarios
' use `DIM m AS _MEM` now that `STRING * n` and the other new types are declared: a real error before the
' declaration is reported; after it only "not supported yet" errors are (`PRINT LEN(m)` gets none, the string
' stored in a number variable none); a name of QB64pe's auto-included files is "not supported yet".
n = "before"
DIM m AS _MEM
PRINT LEN(m)
n = "after"
PRINT _TRUE
SYSTEM
