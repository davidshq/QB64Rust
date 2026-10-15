$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (39_common_shared)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
COMMON SHARED x AS t
REDIM x.v(0 TO 2) AS LONG
PRINT UBOUND(x.v)
SYSTEM
