$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB, REDIM _PRESERVE of a shared array whose bound first names an
' array used without DIM, called twice: what the elements hold at each call (kept between calls or not).
ON ERROR GOTO h
REDIM SHARED sd(1)
sd(1) = 11
PRINT "1.";: q
PRINT " main:"; UBOUND(sd); sd(1)
PRINT "2.";: q
PRINT " main:"; UBOUND(sd); sd(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q
    REDIM _PRESERVE sd(lc(1) + 4)
    PRINT " q:"; UBOUND(sd); sd(1);
    sd(1) = 22
END SUB
