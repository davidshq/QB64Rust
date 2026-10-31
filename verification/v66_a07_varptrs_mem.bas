$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR$ of a _MEM variable: its length, type byte and offset.
DIM m AS _MEM
m = _MEMNEW(8)
v$ = VARPTR$(m)
PRINT LEN(v$); ASC(v$); CVI(MID$(v$, 2, 2))
SYSTEM
