$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (31_redim_shared_member)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
REDIM SHARED x.v(0 TO 2) AS LONG
PRINT UBOUND(x.v)
SYSTEM
