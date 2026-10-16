$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (09_retain_member)
TYPE t
    v(1 TO 3) AS LONG
END TYPE
DIM x AS t
x.v(2) = 5
REDIM _RETAIN x.v(1 TO 3) AS LONG
PRINT x.v(2)
SYSTEM
