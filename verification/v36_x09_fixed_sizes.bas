$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (09_fixed_sizes)
DIM a(2) AS STRING * 4, b(2) AS STRING * 8
a() = b()
PRINT "ok"
SYSTEM
