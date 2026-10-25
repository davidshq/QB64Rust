$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB ... STATIC, REDIM and REDIM _PRESERVE of a shared array whose
' bound first names an array used without DIM, each called twice.
ON ERROR GOTO h
REDIM SHARED sd(1)
sd(1) = 11
PRINT "1.";: q: q
PRINT " main:"; UBOUND(sd); sd(1)
PRINT "2.";: p: p
PRINT " main:"; UBOUND(sd); sd(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q STATIC
    REDIM sd(lc(1) + 4)
    PRINT " q:"; UBOUND(sd); sd(1);
    sd(1) = 22
END SUB

SUB p STATIC
    REDIM _PRESERVE sd(lc(1) + 3)
    PRINT " p:"; UBOUND(sd); sd(1);
    sd(1) = 44
END SUB
