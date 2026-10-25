$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): a TYPE variable with a STRING member among plain variables named by
' VARPTR, and an array of that TYPE: offsets and segments (the strings are not used).
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a AS INTEGER, r AS ts, b AS LONG, s AS STRING, e(2) AS ts
PRINT VARPTR(a); VARPTR(r.n); VARPTR(r.v); VARPTR(b); VARPTR(s)
PRINT VARPTR(e(0).v); VARPTR(e(2).v); VARSEG(r); VARSEG(e(1).v) = VARSEG(r)
SYSTEM
