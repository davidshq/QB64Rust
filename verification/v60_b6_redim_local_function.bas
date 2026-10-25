$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a FUNCTION, REDIM of a shared array whose bound first names an array
' used without DIM, called twice.
ON ERROR GOTO h
REDIM SHARED sd(1)
sd(1) = 11
PRINT "1."; g
PRINT "2."; g
PRINT "main:"; UBOUND(sd); sd(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION g
    REDIM sd(lc(1) + 4)
    g = UBOUND(sd) * 100 + sd(1)
    sd(1) = 22
END FUNCTION
