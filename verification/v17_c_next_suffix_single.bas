$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D3): FOR i (plain i, SINGLE by default) closed by NEXT i! (the same variable).
FOR i = 1 TO 2
PRINT i
NEXT i!
PRINT "after:"; i; i!
SYSTEM
