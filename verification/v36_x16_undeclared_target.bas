$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (16_undeclared_target)
DIM b(2) AS SINGLE
b(1) = 3
a() = b()
PRINT a(1)
PRINT "ok"
SYSTEM
