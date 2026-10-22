$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x37_ubound_member_element_full)
TYPE e
    a(1 TO 50, 2 TO 20) AS _BYTE
END TYPE
DIM f(15) AS e
PRINT UBOUND(f(10).a(5, 3), 2)
SYSTEM
