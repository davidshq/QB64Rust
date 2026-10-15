$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (49_type_dyn_in_static_type_member_array)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE o
    a(1 TO 2) AS t
END TYPE
DIM x AS o, y AS o
REDIM x.a(2).v(7 TO 8) AS LONG
x.a(2).v(8) = 8
y = x
x.a(2).v(8) = -8
PRINT LBOUND(y.a(2).v); UBOUND(y.a(2).v); y.a(2).v(8); x.a(2).v(8)
SYSTEM
