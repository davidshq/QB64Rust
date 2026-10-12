$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM SHARED in the main module, the array used and given new
' bounds in a SUB (with AS and without), and a DIM SHARED static array given REDIM in a SUB.
ON ERROR GOTO h
REDIM SHARED g(5) AS LONG
DIM SHARED st(5) AS LONG
g(2) = 8: st(2) = 9
grow
PRINT "main after the SUB: UBOUND(g), g(2), UBOUND(st), st(2)"; UBOUND(g); g(2); UBOUND(st); st(2)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB grow
    PRINT "SUB: g(2), UBOUND(g)"; g(2); UBOUND(g)
    REDIM g(8) AS LONG
    g(8) = 1
    PRINT "SUB after REDIM g(8) AS LONG: UBOUND"; UBOUND(g)
    REDIM g(12)
    g(12) = 1
    PRINT "SUB after REDIM g(12): UBOUND, LEN of an element"; UBOUND(g); LEN(g(0))
    PRINT "SUB, REDIM st(9) of a DIM SHARED static array:";
    REDIM st(9) AS LONG
    PRINT UBOUND(st)
END SUB
