$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (02_indexed_source)
DIM a(2) AS LONG, b(2) AS LONG
a() = b(1)
PRINT "ok"
SYSTEM
