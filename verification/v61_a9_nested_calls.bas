$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a main-module variable in conventional memory, a SUB whose local is
' there calling another such SUB, and the first SUB's local read after the inner one returns.
DIM g AS LONG
g = 1
PRINT VARPTR(g)
a
a
b
SYSTEM

SUB a
    DIM x AS INTEGER
    x = 11
    PRINT "a"; VARPTR(x)
    b
    DEF SEG = VARSEG(x)
    PRINT "a"; VARPTR(x); PEEK(VARPTR(x)); x
    DEF SEG
END SUB

SUB b
    DIM y AS DOUBLE, t AS STRING * 3
    y = 2
    PRINT "b"; VARPTR(y); VARPTR(t)
END SUB
