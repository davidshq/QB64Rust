$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (11_types_differ)
TYPE t1
    x AS LONG
END TYPE
TYPE t2
    x AS LONG
END TYPE
DIM a(2) AS t1, b(2) AS t2
a() = b()
PRINT "ok"
SYSTEM
