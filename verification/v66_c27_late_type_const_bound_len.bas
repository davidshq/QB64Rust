$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-memory-records row): LEN above its block of a TYPE whose member array bound names a CONST.
CONST n = 3
DIM e AS late
PRINT LEN(e)
SYSTEM

TYPE late
    k AS LONG
    v(1 TO n) AS LONG
END TYPE
