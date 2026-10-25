$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB, REDIM ... AS LONG of a shared LONG array whose bound first
' names an array used without DIM, and REDIM with a suffix of a shared array with that suffix.
ON ERROR GOTO h
REDIM SHARED sd(1) AS LONG
REDIM SHARED se&(1)
sd(1) = 11: se&(1) = 12
PRINT "1.";: q
PRINT " main:"; UBOUND(sd); sd(1); UBOUND(se&); se&(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q
    REDIM sd(lc(1) + 4) AS LONG
    REDIM se&(ld(1) + 5)
    PRINT " q:"; UBOUND(sd); sd(1); UBOUND(se&); se&(1);
END SUB
