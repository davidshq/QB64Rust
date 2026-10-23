$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR of a member of an element of an array of a TYPE with a STRING member.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a(3) AS ts, x AS ts
PRINT VARPTR(a(1).v); VARPTR(a(2))
SYSTEM
