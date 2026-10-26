$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a scalar parameter whose address is taken, called with an array
' element and a member of a TYPE variable: the array and the variable move to conventional memory, and a POKE
' through the parameter reaches them.
TYPE st
    a AS INTEGER
    b AS INTEGER
END TYPE
DIM a(3) AS INTEGER
DIM s AS st
a(2) = 1: s.b = 2
t a(2)
t s.b
PRINT a(2); s.b
SYSTEM

SUB t (p AS INTEGER)
    PRINT VARPTR(p); VARSEG(p); p
    DEF SEG = VARSEG(p)
    POKE VARPTR(p), 42
    DEF SEG
END SUB
