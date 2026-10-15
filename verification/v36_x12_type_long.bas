$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (12_type_long)
TYPE t1
    x AS LONG
END TYPE
DIM a(2) AS t1, b(2) AS LONG
a() = b()
PRINT "ok"
SYSTEM
