$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x36_ubound_scalar_member_element)
TYPE e
    a(1 TO 50) AS _BYTE
END TYPE
DIM v AS e
PRINT UBOUND(v.a(5))
SYSTEM
