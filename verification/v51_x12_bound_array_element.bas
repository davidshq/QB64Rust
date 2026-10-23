$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x12_bound_array_element)
DIM a(3) AS LONG
TYPE t1
    v(a(1)) AS LONG
END TYPE
PRINT "ok"
SYSTEM
