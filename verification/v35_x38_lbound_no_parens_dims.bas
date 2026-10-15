$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (38_lbound_no_parens_dims)
TYPE t
    v(1 TO 2, 3 TO 4) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT LBOUND(x.v, 2); UBOUND(x.v, 2); LBOUND(x.v); UBOUND(x.v)
SYSTEM
