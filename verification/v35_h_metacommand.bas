$CONSOLE:ONLY
$unstable:typefields
$UNSTABLE:TYPEFIELDS
$UNSTABLE:MIDI
$UNSTABLE:HTTP
' Verification (m3-typefields, before its design): $UNSTABLE:TYPEFIELDS in lower case and twice; $UNSTABLE:MIDI
' and $UNSTABLE:HTTP, which are stable now (a warning). After the TYPE that uses a marker it is too late
' (v35_x45).
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
REDIM x.v(2 TO 3) AS LONG
x.v(3) = 3
PRINT LBOUND(x.v); UBOUND(x.v); x.v(3)
SYSTEM
