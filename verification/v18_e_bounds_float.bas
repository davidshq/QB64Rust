$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): float constants as bounds.
DIM a(2.5) AS LONG
DIM b(1.5 TO 3.5) AS LONG
PRINT LBOUND(a); UBOUND(a); LBOUND(b); UBOUND(b)
SYSTEM
