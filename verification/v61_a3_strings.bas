$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): SADD, VARPTR and VARSEG of a FUNCTION's string and a SUB's
' fixed-length string, PEEK through SADD as misc/ripples of qbasic_testcases does, and VARPTR$ of a SUB's INTEGER.
PRINT f$("ABC")
PRINT f$("xy")
s
s
SYSTEM

FUNCTION f$ (a$)
    m$ = a$ + "!"
    PRINT SADD(m$); VARPTR(m$); VARSEG(m$)
    DEF SEG = VARSEG(m$)
    FOR t& = SADD(m$) TO SADD(m$) + LEN(m$) - 1
        PRINT PEEK(t&);
    NEXT
    PRINT
    DEF SEG
    f$ = m$
END FUNCTION

SUB s
    DIM fx AS STRING * 4, k AS INTEGER
    PRINT VARPTR(fx); VARSEG(fx); VARPTR(k)
    fx = "wxyz": k = 513
    DEF SEG = VARSEG(fx)
    PRINT PEEK(VARPTR(fx)); PEEK(VARPTR(fx) + 3)
    POKE VARPTR(fx) + 1, 65
    DEF SEG
    PRINT fx
    v$ = VARPTR$(k)
    PRINT LEN(v$); ASC(v$, 1); ASC(v$, 2); ASC(v$, 3)
END SUB
