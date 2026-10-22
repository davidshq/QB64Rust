$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x35_lbound_member_element_nodim)
TYPE e
    a(1 TO 50, 2 TO 20) AS _BYTE
END TYPE
DIM f(15) AS e
PRINT LBOUND(f(10).a(5))
SYSTEM
