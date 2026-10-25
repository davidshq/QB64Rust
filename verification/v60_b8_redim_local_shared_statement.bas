$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB with SHARED sd(), REDIM of the main module's sd (not SHARED
' there) whose bound first names an array used without DIM.
ON ERROR GOTO h
REDIM sd(1)
sd(1) = 11
PRINT "1.";: q
PRINT " main:"; UBOUND(sd); sd(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q
    SHARED sd()
    PRINT " above:"; UBOUND(sd);
    REDIM sd(lc(1) + 4)
    PRINT " q:"; UBOUND(sd); sd(1);
END SUB
