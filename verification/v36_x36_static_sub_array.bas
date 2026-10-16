$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (36_static_sub_array)
S
S
PRINT "ok"
SYSTEM
SUB S STATIC
    DIM a(2) AS LONG, b(2) AS LONG
    a(1) = a(1) + 1
    b() = a()
    PRINT b(1)
END SUB
