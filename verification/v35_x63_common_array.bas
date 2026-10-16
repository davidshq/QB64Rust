$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 5.2): does the old compiler accept an array of a type with a _DYNAMIC member
' in COMMON (v35_x39 measured a variable)?
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
COMMON SHARED p() AS t
REDIM p(1) AS t
PRINT UBOUND(p(1).v)
SYSTEM
