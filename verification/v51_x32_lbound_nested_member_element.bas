$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x32_lbound_nested_member_element)
TYPE g1
    a(1 TO 5) AS LONG
END TYPE
TYPE e
    g AS g1
END TYPE
DIM f(15) AS e
PRINT LBOUND(f(1).g.a(2))
SYSTEM
