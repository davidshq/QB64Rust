$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR$ of an element of a member array: its length, type byte and offset, beside VARPTR.
TYPE ma
    k AS LONG
    v(1 TO 3) AS INTEGER
END TYPE
DIM x AS ma
v$ = VARPTR$(x.v(2))
PRINT LEN(v$); ASC(v$); CVI(MID$(v$, 2, 2)); VARPTR(x.v(2))
SYSTEM
