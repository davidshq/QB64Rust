$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (18_redim_no_to_no_as)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
REDIM x.v(5)
PRINT UBOUND(x.v)
SYSTEM
