$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of a member of a TYPE parameter (as jpegmake of
' qbasic_testcases, declared AS ANY), the argument passed through a chain of three SUBs from the main module, and
' from a SUB's own TYPE variable; a POKE through it reaches the caller's variable.
TYPE st
    a AS INTEGER
    b AS LONG
END TYPE
DECLARE SUB inner (z AS ANY)
DIM s AS st
s.b = 1000
outer s
PRINT s.b
own
SYSTEM

SUB outer (x AS st)
    mid x
END SUB

SUB mid (y AS st)
    inner y
END SUB

SUB inner (z AS st)
    PRINT VARPTR(z.b); VARSEG(z.b); VARPTR(z.a)
    DEF SEG = VARSEG(z.b)
    PRINT PEEK(VARPTR(z.b)); PEEK(VARPTR(z.b) + 1)
    POKE VARPTR(z.b), 77
    DEF SEG
END SUB

SUB own
    DIM w AS st
    w.b = 5
    inner w
    PRINT w.b
END SUB
