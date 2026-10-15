$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (15_undeclared_source)
DIM a(2) AS LONG
a() = b()
PRINT "ok"
SYSTEM
