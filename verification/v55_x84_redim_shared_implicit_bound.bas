$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 4.4): REDIM of a shared array whose bound raises an error, in the
' main module and in a SUB, and in a SUB whose bound first names an array used without DIM (x84_redim_shared_implicit_bound)
ON ERROR GOTO h
REDIM SHARED sd(1)
PRINT "1. main, REDIM sd(ASC() + 5):";
REDIM sd(ASC("") + 5)
PRINT UBOUND(sd)
PRINT "2. SUB, REDIM sd(ASC() + 6):";
p
PRINT UBOUND(sd)
PRINT "3. SUB, REDIM sd(lc(1) + 4), lc local:";
q
PRINT UBOUND(sd)
PRINT "4. SUB, REDIM sd(lc(1) + 4) twice:";
q: q
PRINT UBOUND(sd)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB p
    REDIM sd(ASC("") + 6)
END SUB
SUB q
    REDIM sd(lc(1) + 4)
    PRINT " in q:"; UBOUND(sd);
END SUB
