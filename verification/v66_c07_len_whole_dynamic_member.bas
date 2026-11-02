$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m4-names-arrays row): LEN of a whole _DYNAMIC member array.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM x AS dy
PRINT LEN(x.d())
SYSTEM
