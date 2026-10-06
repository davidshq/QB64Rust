$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): the smallest _INTEGER64 divided by -1 with \ and MOD, with a handler.
ON ERROR GOTO h
DIM b AS _INTEGER64
b = -9223372036854775807 - 1
PRINT "mod:"; b MOD -1
PRINT "idiv:"; b \ -1
PRINT "after"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT
