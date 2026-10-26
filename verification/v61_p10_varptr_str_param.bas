$CONSOLE:ONLY
' Verification (qbasic-testcases, review of the change): VARPTR$ of a numeric parameter whose address is taken,
' called with an array element (the array is in conventional memory, outside DBLOCK), a member of a TYPE variable
' and an expression (a slot): its type byte, and its offset beside VARPTR of the parameter.
TYPE st
    a AS INTEGER
    b AS INTEGER
END TYPE
DIM a(3) AS INTEGER
DIM s AS st
t a(2)
t s.b
t 5 + 1
SYSTEM

SUB t (p AS INTEGER)
    v$ = VARPTR$(p)
    PRINT LEN(v$); ASC(v$); CVI(MID$(v$, 2, 2)) AND &HFFFF&; VARPTR(p)
END SUB
