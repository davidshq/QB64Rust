$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): LBOUND and UBOUND of a TYPE array, and LBOUND() with empty parentheses.
TYPE t
    m AS LONG
END TYPE
DIM a(1 TO 4) AS t
PRINT LBOUND(a); UBOUND(a)
PRINT LBOUND(a()); UBOUND(a())
SYSTEM
