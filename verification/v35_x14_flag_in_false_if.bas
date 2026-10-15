$CONSOLE:ONLY
$IF 0 THEN
$UNSTABLE:TYPEFIELDS
$END IF
' Verification (m3-typefields, before its design): does the old compiler accept this? (14_flag_in_false_if)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
