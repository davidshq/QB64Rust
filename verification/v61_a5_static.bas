$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR of the variables of a SUB ... STATIC and of a FUNCTION's STATIC
' variable, kept across calls, with calls to a SUB whose locals are in conventional memory between them.
DIM g AS INTEGER
PRINT VARPTR(g)
s
loc
PRINT f%
s
loc
PRINT f%
SYSTEM

SUB s STATIC
    DIM a AS INTEGER, b AS LONG
    a = a + 1: b = b + 10
    PRINT VARPTR(a); VARPTR(b); a; b
END SUB

SUB loc
    DIM w AS LONG, v AS INTEGER
    w = -1: v = -1
    PRINT VARPTR(w); VARPTR(v)
END SUB

FUNCTION f%
    STATIC c AS INTEGER
    DIM e AS INTEGER
    c = c + 2
    PRINT VARPTR(c); VARPTR(e);
    f% = c
END FUNCTION
