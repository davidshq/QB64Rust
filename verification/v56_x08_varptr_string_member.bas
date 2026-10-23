$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR of a STRING member of a TYPE variable.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a(3) AS ts, x AS ts
PRINT VARPTR(x.n)
SYSTEM
