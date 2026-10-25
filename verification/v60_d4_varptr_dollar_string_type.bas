$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): VARPTR$ of a number member of a TYPE variable with a STRING member, byte
' by byte (the strings are not used).
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM r AS ts
x$ = VARPTR$(r.v)
FOR k = 1 TO LEN(x$): PRINT ASC(x$, k);: NEXT: PRINT
SYSTEM
