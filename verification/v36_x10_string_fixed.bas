$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (10_string_fixed)
DIM a(2) AS STRING, b(2) AS STRING * 4
a() = b()
PRINT "ok"
SYSTEM
