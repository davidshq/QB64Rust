$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (27_nested_scalar_owner)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE o
    k AS INTEGER
    i AS t
END TYPE
DIM a AS o, b AS o
REDIM a.i.v(4 TO 5) AS LONG
a.i.v(5) = 5
b = a
a.i.v(5) = -5
PRINT LBOUND(b.i.v); UBOUND(b.i.v); b.i.v(5); a.i.v(5)
SYSTEM
