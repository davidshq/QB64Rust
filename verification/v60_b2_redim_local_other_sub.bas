$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB, REDIM of a shared array whose bound first names an array used
' without DIM, then a call of another SUB that names sd, and a SUB called after it; called twice.
ON ERROR GOTO h
REDIM SHARED sd(1)
sd(1) = 11
PRINT "1.";: q: r
PRINT
PRINT "2.";: q: r
PRINT
PRINT "main:"; UBOUND(sd); sd(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q
    REDIM sd(lc(1) + 4)
    PRINT " q:"; UBOUND(sd); sd(1);
    sd(1) = 22
    r
    PRINT " q:"; UBOUND(sd); sd(1);
END SUB

SUB r
    PRINT " r:"; UBOUND(sd); sd(1);
END SUB
