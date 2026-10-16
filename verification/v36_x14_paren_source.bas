$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (14_paren_source)
DIM a(2) AS LONG, b(2) AS LONG
b(1) = 4
a() = (b())
PRINT a(1)
PRINT "ok"
SYSTEM
