$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (07_single_double)
DIM a(2) AS SINGLE, b(2) AS DOUBLE
a() = b()
PRINT "ok"
SYSTEM
