$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): MOD by zero with a handler (fatal like integer division?).
ON ERROR GOTO h
z = 0
PRINT 5 MOD z
PRINT "after"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT
