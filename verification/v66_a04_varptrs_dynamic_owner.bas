$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR$ of a TYPE variable that owns a _DYNAMIC member array: its length, type byte and offset.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM x AS dy
v$ = VARPTR$(x)
PRINT LEN(v$); ASC(v$); CVI(MID$(v$, 2, 2))
SYSTEM
