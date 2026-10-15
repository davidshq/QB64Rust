$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 5.2): does the old compiler accept LEN of an array element of a type with a
' _DYNAMIC member (v35_x20 measured a variable)?
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
REDIM p(1) AS t
PRINT LEN(p(0))
SYSTEM
