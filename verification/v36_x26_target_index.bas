$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (26_target_index)
DIM a(2) AS LONG, b(2) AS LONG
a(1) = b()
PRINT "ok"
SYSTEM
