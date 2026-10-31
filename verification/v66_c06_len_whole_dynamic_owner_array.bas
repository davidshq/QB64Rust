$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m4-names-arrays row): LEN of a whole array of a TYPE with a _DYNAMIC member array.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM a(2) AS dy
PRINT LEN(a())
SYSTEM
