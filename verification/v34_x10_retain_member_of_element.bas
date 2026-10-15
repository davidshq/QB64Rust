$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (10_retain_member_of_element)
TYPE t
    v(1 TO 3) AS LONG
END TYPE
DIM x(2) AS t
x(1).v(2) = 5
REDIM _RETAIN x(1).v(1 TO 3) AS LONG
PRINT x(1).v(2)
SYSTEM
