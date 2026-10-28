$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a member of a TYPE parameter passed to a scalar parameter whose address
' is taken, and an element of an array parameter likewise: the caller's TYPE variable and array move to
' conventional memory through the SUB's parameters.
TYPE st
    a AS INTEGER
    b AS INTEGER
END TYPE
DIM s AS st
DIM w(3) AS INTEGER
s.b = 1: w(1) = 2
a s, w()
PRINT s.b; w(1)
SYSTEM

SUB a (x AS st, z() AS INTEGER)
    t x.b
    t z(1)
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p); p
    DEF SEG = VARSEG(p)
    POKE VARPTR(p), 60
    DEF SEG
END SUB
