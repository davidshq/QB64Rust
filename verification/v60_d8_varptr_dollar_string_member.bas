$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): VARPTR$ of the STRING member of a TYPE variable.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM r AS ts
x$ = VARPTR$(r.n)
PRINT LEN(x$)
SYSTEM
