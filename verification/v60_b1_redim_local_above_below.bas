$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB, REDIM of a shared array whose bound first names an array used
' without DIM: which sd the statements above and below the REDIM name, on a first and a second call.
ON ERROR GOTO h
REDIM SHARED sd(1)
sd(1) = 11
PRINT "1. first call:";: q
PRINT " main:"; UBOUND(sd); sd(1)
PRINT "2. second call:";: q
PRINT " main:"; UBOUND(sd); sd(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q
    PRINT " above:"; UBOUND(sd); sd(1);
    sd(1) = 33
    REDIM sd(lc(1) + 4)
    PRINT " below:"; UBOUND(sd); sd(1);
    sd(1) = 22
END SUB
