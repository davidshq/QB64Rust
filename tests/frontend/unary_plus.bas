' TEST: check-fail
$CONSOLE:ONLY
' There is no unary `+` (verification\v15_plus_*: the old compiler rejects each of these lines)
n = 3: s$ = "abc"
PRINT +5
x = +n
PRINT 2 * +n
u$ = +s$
PRINT -n
t +s$
SUB t (q AS STRING)
END SUB
