$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR$ of a member of an array element: its length, type byte and offset, beside VARPTR.
TYPE box
    a AS LONG
    b AS INTEGER
END TYPE
DIM e(2) AS box
v$ = VARPTR$(e(1).b)
PRINT LEN(v$); ASC(v$); CVI(MID$(v$, 2, 2)); VARPTR(e(1).b)
SYSTEM
