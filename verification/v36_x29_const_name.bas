$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (29_const_name)
CONST c = 1
DIM a(2) AS LONG
a() = c()
PRINT "ok"
SYSTEM
