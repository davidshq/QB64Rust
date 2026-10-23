$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR of a TYPE variable with a STRING member, then the member
' used (the old compiler's allocation in conventional memory leaves out the strings of the value).
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM r AS ts
PRINT VARPTR(r.v); VARSEG(r)
r.v = 5
PRINT r.v
r.n = "abc"
PRINT r.n; LEN(r.n)
SYSTEM
