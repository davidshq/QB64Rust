$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (34_redim_static_member_new_bounds)
TYPE t
    v(1) _STATIC AS LONG
END TYPE
DIM x AS t
REDIM x.v(0 TO 5) AS LONG
PRINT UBOUND(x.v)
SYSTEM
