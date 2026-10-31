$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARSEG(x) of a TYPE variable that owns a _DYNAMIC member array.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM x AS dy
x.k = 7
PRINT VARSEG(x)
SYSTEM
