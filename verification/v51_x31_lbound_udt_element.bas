$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x31_lbound_udt_element)
TYPE e
    x AS LONG
END TYPE
DIM f(15) AS e
PRINT LBOUND(f(1))
SYSTEM
