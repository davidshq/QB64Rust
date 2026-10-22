$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x05_bound_len_var)
DIM x AS STRING
TYPE t1
    v(LEN(x)) AS LONG
END TYPE
PRINT "ok"
SYSTEM
