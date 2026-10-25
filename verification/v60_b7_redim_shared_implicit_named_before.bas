$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB, REDIM of a shared array whose bound names an array used
' without DIM that a statement above already named (not first named in the bound).
ON ERROR GOTO h
REDIM SHARED sd(1)
sd(1) = 11
PRINT "1.";: q
PRINT " main:"; UBOUND(sd); sd(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q
    PRINT " lc:"; lc(1);
    REDIM sd(lc(1) + 4)
    PRINT " q:"; UBOUND(sd); sd(1);
END SUB
