$CONSOLE:ONLY
$IF 1 THEN
$UNSTABLE:TYPEFIELDS
$END IF
' Verification (m3-typefields, before its design): does the old compiler accept this? (15_flag_in_true_if)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
