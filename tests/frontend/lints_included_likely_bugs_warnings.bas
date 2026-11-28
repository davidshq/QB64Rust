' TEST: warnings
' `never-set` and `single-fraction` are given in an included file too (spec `compiler/lints`, "Only the file
' being compiled"), as a game whose code is all in `.bm` files needs: `cash` in `Fortify` and `rate` in `Pay`,
' both in `inc/lints_likely_bugs.bm`.
$CONSOLE:ONLY
DIM SHARED cash(1) AS INTEGER, rate AS SINGLE
rate = .3
Fortify 0
Pay
'$INCLUDE:'inc/lints_likely_bugs.bm'
