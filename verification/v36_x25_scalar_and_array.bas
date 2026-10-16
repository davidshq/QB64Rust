$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (25_scalar_and_array)
DIM a AS LONG, a(2) AS LONG, b(2) AS LONG
b(2) = 8
a() = b()
PRINT a(2)
PRINT "ok"
SYSTEM
