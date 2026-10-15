$CONSOLE:ONLY

' Verification (m3-typefields, before its design): does the old compiler accept this? (45_flag_after_type)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
$UNSTABLE:TYPEFIELDS
SYSTEM
