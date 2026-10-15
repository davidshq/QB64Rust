$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (04_literal_source)
DIM a(2) AS LONG
a() = 5
PRINT "ok"
SYSTEM
