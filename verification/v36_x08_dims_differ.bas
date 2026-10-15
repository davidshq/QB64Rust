$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (08_dims_differ)
DIM a(2) AS LONG, b(2, 2) AS LONG
a() = b()
PRINT "ok"
SYSTEM
