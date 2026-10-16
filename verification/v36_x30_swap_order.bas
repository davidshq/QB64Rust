$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (30_swap_order)
DIM a(2) AS LONG, b(2) AS LONG
b() = a(): a() = b()
PRINT "ok"
SYSTEM
