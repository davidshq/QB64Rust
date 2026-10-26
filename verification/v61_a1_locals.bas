$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of a SUB's locals and of a STATIC one, on two calls
' with a call to another SUB between them; PEEK and POKE through them; the STATIC one keeps its value.
s
other
s
SYSTEM

SUB s
    DIM k AS INTEGER, l AS LONG
    STATIC st AS INTEGER
    k = 258: l = -2
    st = st + 1
    PRINT VARPTR(k); VARSEG(k); VARPTR(l); VARPTR(st); VARSEG(st)
    DEF SEG = VARSEG(k)
    PRINT PEEK(VARPTR(k)); PEEK(VARPTR(k) + 1); PEEK(VARPTR(l) + 3); PEEK(VARPTR(st))
    POKE VARPTR(k), 3
    DEF SEG
    PRINT k; l; st
END SUB

SUB other
    DIM m AS INTEGER, n AS LONG
    m = 7: n = 9
    PRINT VARPTR(m); VARPTR(n)
    DEF SEG = VARSEG(m)
    POKE VARPTR(n), 1
    DEF SEG
    PRINT m; n
END SUB
