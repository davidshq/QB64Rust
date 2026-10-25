$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB, one REDIM of two shared arrays, the new array used without
' DIM first named in the bound of the first item, then of the second.
ON ERROR GOTO h
REDIM SHARED sd(1), se(1), sf(1), sg(1)
PRINT "1.";: q1
PRINT " main:"; UBOUND(sd); UBOUND(se)
PRINT "2.";: q2
PRINT " main:"; UBOUND(sf); UBOUND(sg)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q1
    REDIM sd(lc(1) + 4), se(5)
    PRINT " q1:"; UBOUND(sd); UBOUND(se);
END SUB

SUB q2
    REDIM sf(6), sg(ld(1) + 7)
    PRINT " q2:"; UBOUND(sf); UBOUND(sg);
END SUB
