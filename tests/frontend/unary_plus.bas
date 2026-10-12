' TEST: check-fail
$CONSOLE:ONLY
' There is no unary `+` (verification\v15_plus_*, v25_x01 to x06 and x10: the old compiler rejects each of these
' lines). A `+` is only dropped right after a `+` or a `-` (signs_typed.bas), so `-+s$` is `-s$`
n = 3: s$ = "abc"
PRINT +5
x = +n
PRINT 2 * +n
u$ = +s$
PRINT -n
t +s$
PRINT +-n
PRINT 2 * +-n
PRINT NOT +n
PRINT 2 * (+n)
PRINT 2 < +n
PRINT 2 ^ +n
PRINT -+s$
s$ = - -s$
SUB t (q AS STRING)
END SUB
