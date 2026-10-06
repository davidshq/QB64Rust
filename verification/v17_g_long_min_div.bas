$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): the smallest LONG divided by -1 with \ and MOD.
DIM b AS LONG
b = -2147483647 - 1
PRINT "mod:"; b MOD -1
PRINT "idiv:"; b \ -1
PRINT "after"
SYSTEM
