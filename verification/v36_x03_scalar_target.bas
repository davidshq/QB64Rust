$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (03_scalar_target)
DIM a AS LONG, b(2) AS LONG
a = b()
PRINT "ok"
SYSTEM
